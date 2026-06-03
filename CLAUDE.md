# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

**TUAS** (Thermo-hydraulic Uniphase Advection and Convection Solver for Salt Flows) is a Rust thermal-hydraulics library for single-phase, nearly-incompressible fluid systems using the Boussinesq approximation. It was developed as part of a PhD thesis (Theodore Ong, UC Berkeley, supervisor Prof. Per F. Peterson) to simulate the CIET integral effects test and Gen-IV FHR reactors.

License: GPL-3.0. Requires OpenBLAS on Linux/macOS, Intel MKL on Windows.

---

## Prerequisites

**Linux (Debian/Ubuntu/Mint):**
```bash
sudo apt install libopenblas-dev
```
**Arch / EndeavourOS:**
```bash
sudo pacman -S openblas
```

---

## Build & Run

```bash
# Run all tests (release mode — solvers are expensive, always use --release)
cargo test --release

# Run a single test by name (substring match)
cargo test --release fluid_mechanics_basics

# Run tests continuously, ignoring generated CSV files
cargo watch -x "test --release" --ignore '*.csv'

# Run the CIET educational GUI simulator
cargo run --example ciet_educational_simulator --release

# Profile with flamegraph
sudo sysctl kernel.perf_event_paranoid=2
cargo flamegraph --unit-test tuas_boussinesq_solver

# Update dependencies
cargo install cargo-edit
cargo upgrade -i allow && cargo update
```

Tests write CSV output files to the repo root. Use `tail -f <file>.csv` to watch them live.

---

## Module Architecture

The library is in `src/lib/` and exposes everything through `src/lib/lib.rs`. Modules are **strictly layered** — lower layers must not import from higher ones.

```
Layer 0 — Errors
  tuas_lib_error              TuasLibError enum (thiserror)

Layer 1 — Physics foundations
  boussinesq_thermophysical_properties   Material property database
  fluid_mechanics_correlations           Friction factors, pressure drop correlations
  heat_transfer_correlations             Nusselt correlations, HeatTransferInteractionType enum
  control_volume_dimensions              Geometry newtypes (InnerDiameter, OuterDiameter, …)
  boundary_conditions                    Boundary condition structs

Layer 2 — Single control volume
  single_control_vol          SingleCVNode struct + constructors + timestep advance

Layer 3 — Array control volumes & networks
  array_control_vol_and_fluid_component_collections
    ├── standalone_fluid_nodes / standalone_solid_nodes   (raw matrix solvers)
    ├── one_dimension_cartesian_conducting_medium          (1D Cartesian, no lateral coupling)
    ├── one_d_solid_array_with_lateral_coupling            SolidColumn struct
    ├── one_d_fluid_array_with_lateral_coupling            FluidArray struct
    ├── conductance_array_functions
    └── fluid_component_collection                         (series/parallel pipe networks)

Layer 4 — Pre-built components
  pre_built_components
    ├── heat_transfer_entities            HeatTransferEntity enum (unifies CVs + BCs)
    ├── non_insulated_fluid_components    NonInsulatedFluidComponent
    ├── insulated_pipes_and_fluid_components  InsulatedFluidComponent
    ├── non_insulated_parallel_fluid_components
    ├── shell_and_tube_heat_exchanger
    ├── one_d_solid_structure
    ├── ciet_struct_supports / ciet_heater_top_and_bottom_head_bare
    ├── insulated_porous_media_fluid_components
    ├── non_insulated_porous_media_fluid_components
    ├── ciet_isothermal_test_components
    ├── ciet_steady_state_natural_circulation_test_components
    ├── uw_madison_flibe_loop_components
    └── ciet_three_branch_plus_dracs

Example (dev-only, egui GUI)
  examples/ciet_educational_simulator
```

---

## Key Types

### `Material` / `SolidMaterial` / `LiquidMaterial`
`src/lib/boussinesq_thermophysical_properties/mod.rs`

```rust
pub enum Material {
    Solid(SolidMaterial),
    Liquid(LiquidMaterial),
}
pub enum SolidMaterial { SteelSS304L, Copper, Fiberglass, PyrogelHPS, CustomSolid(...) }
pub enum LiquidMaterial { TherminolVP1, DowthermA, HITEC, YD325, FLiBe, FLiNaK, CustomLiquid(...) }
```

All thermophysical property functions (`try_get_rho`, `try_get_h`, `try_get_temperature_from_h`, …) take a `Material` + temperature (+ pressure). Custom materials accept function pointers for arbitrary correlations. `range_check()` enforces valid temperature ranges; out-of-range calls return `TuasLibError::ThermophysicalPropertyTemperatureRangeError`.

### `HeatTransferInteractionType`
`src/lib/heat_transfer_correlations/heat_transfer_interactions/heat_transfer_interaction_enums.rs`

This enum is the glue between nodes. Pass one to `HeatTransferEntity::link_to_front` / `link_to_back` or the free function `link_heat_transfer_entity`. Key variants:
- `UserSpecifiedThermalConductance(ThermalConductance)` — explicit conductance
- `SingleCartesianThermalConductanceOneDimension(Material, XThickness)` — 1D slab conduction
- `DualCylindricalThermalConductance(...)` — two-layer cylindrical conduction
- `CylindricalConductionConvectionLiquidOutside / LiquidInside` — combined conduction + convective HTC
- `Advection(DataAdvection)` — fluid advection carrying enthalpy between nodes

### `HeatTransferEntity`
`src/lib/pre_built_components/heat_transfer_entities/`

An enum over `CVType` (which wraps `SingleCVNode`, `FluidArray`, or `SolidColumn`) and `BCType`. Use at the top level to link components without caring about their internal type.

```rust
pub enum HeatTransferEntity {
    ControlVolume(CVType),
    BoundaryConditions(BCType),
}
pub enum CVType { SingleCV(SingleCVNode), FluidArrayCV(FluidArray), SolidArrayCV(SolidColumn) }
```

### `SingleCVNode`
`src/lib/single_control_vol/mod.rs`

The fundamental building block — one lumped control volume node.

**Constructors:** `new_sphere`, `new_cylinder`, `new_cylindrical_shell`, `new_block`, `new_one_dimension_volume`, `new_odd_shaped_pipe`.

**Timestep loop:**
1. Link CVs and BCs — interactions push values into `rate_enthalpy_change_vector`.
2. Call `advance_timestep` — integrates powers × Δt.
3. Read back temperature via `get_temperature_from_enthalpy_and_set`.

### Array CVs: `FluidArray` and `SolidColumn`
`src/lib/array_control_vol_and_fluid_component_collections/one_d_fluid_array_with_lateral_coupling/`
`src/lib/array_control_vol_and_fluid_component_collections/one_d_solid_array_with_lateral_coupling/`

1D pipe/structure discretised into N nodes. Both have a `front_single_cv` and `back_single_cv` bounding the array. `FluidArray` also carries `fluid_component_loss_properties: DimensionlessDarcyLossCorrelations` and `nusselt_correlation: NusseltCorrelation`. Both use ndarray-linalg (OpenBLAS/MKL) matrix solvers for the implicit energy equation.

### `FluidComponentCollection`
`src/lib/array_control_vol_and_fluid_component_collections/fluid_component_collection/`

Handles pipe networks: computes mass flowrate given a pressure difference for components wired in series or parallel. The key trait is `FluidComponentTrait`; the solver implements regula falsi for convergence robustness (needed at high flowrates ~1000+ kg/s as in gFHR).

---

## Pre-built Component File Conventions

Every component in `pre_built_components/` follows the same internal file split:
- `mod.rs` — struct definition and constructors
- `preprocessing.rs` — conductance calculations and linking setup for each timestep
- `calculation.rs` — `advance_timestep` wrappers
- `postprocessing.rs` — temperature vector / outlet temperature accessors
- `fluid_component.rs` — `FluidComponentTrait` impl (pressure drop / mass flowrate)
- `type_conversion.rs` — `From`/`TryInto` impls into `HeatTransferEntity`
- `calibration.rs` — HTC calibration utilities (where present)

---

## Simulation Pattern

```rust
use tuas_boussinesq_solver::prelude::beta_testing::*;

// 1. Construct pre-built components (or raw CVs)
let mut pipe = InsulatedFluidComponent::new_insulated_pipe(...)?;

// 2. Each timestep: set flowrate, link entities, advance
pipe.set_mass_flowrate(m_dot);
// link_heat_transfer_entity or .link_to_front() / .link_to_back()
// dispatches on HeatTransferInteractionType
pipe.advance_timestep(dt)?;

// 3. Read temperatures
let temp_vec = pipe.pipe_fluid_array.get_temperature_vector()?;
```

For serial single-pipe to full coupled-loop examples, see the tutorial tests in:
`src/lib/pre_built_components/insulated_pipes_and_fluid_components/tutorials/`
- `tutorial_1` — pressure drop from mass flowrate
- `tutorial_2` — mass flowrate from pressure drop
- `tutorial_3` — mass flowrate from pressure change (includes gravity)
- `tutorial_4` — heat transfer through a pipe (steady state)
- `tutorial_5` — combined thermal-hydraulics in a time loop
- `tutorial_6` — custom material (graphite) in a gFHR-scale pipe

---

## Prelude API Stability

| Module | Stability |
|---|---|
| `prelude::beta_testing` | More stable; recommended for new code |
| `prelude::alpha_nightly` | Unstable; API may change without notice |

Import with:
```rust
use tuas_boussinesq_solver::prelude::beta_testing::*;
```

---

## Testing Notes

- Tests output CSV files to the repo root — normal behaviour, not a build artifact to commit.
- Regression tests are co-located with the components they validate (in `tests_and_examples/` and `parasitic_heat_loss_regression_tests/` subdirectories).
- CIET steady-state natural circulation and isothermal tests validate against published Zweibaum (2015) and Zou et al. (2019) SAM data; agreement is within ~6%.
- `gfhr_pipe_tests` is `#[cfg(test)]` only — it exercises FLiBe and HITEC pipes at ~1173 kg/s flowrates.
- The coupled DRACS loop tests require timestep of 0.1 s and simulation time ≥ 2000–2500 s to reach steady state; at 0.5 s timestep with an analog PID controller, oscillatory instability can prevent convergence.

---

## Key Dependencies

- `uom` — all physical quantities are unit-safe (`Length`, `ThermodynamicTemperature`, `MassRate`, etc.); import units via `uom::si::<quantity>::<unit>`.
- `ndarray` + `ndarray-linalg` — matrix solvers for array CV energy equations (OpenBLAS on Linux/macOS, Intel MKL on Windows).
- `peroxide` — numerical methods (used via `#[macro_use] extern crate peroxide` at crate root).
- `roots` — root-finding (Brent-Dekker / regula falsi for flowrate solver).
- `thiserror` — error enum derivation for `TuasLibError`.

---

## Examples

### CIET Educational Simulator
`examples/ciet_educational_simulator/`

A real-time egui GUI simulating the CIET loop. Run with:

```bash
cargo run --example ciet_educational_simulator --release
```
