# TUAS Boussinesq Solver — Codebase Guide

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
# Run all tests (release mode for speed)
cargo test --release

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
  heat_transfer_correlations             Nusselt correlations, thermal resistance, view factors
  control_volume_dimensions              Geometry newtypes (InnerDiameter, OuterDiameter, …)
  boundary_conditions                    Boundary condition structs

Layer 2 — Single control volume
  single_control_vol          SingleCVNode struct + constructors + timestep advance

Layer 3 — Array control volumes & networks
  array_control_vol_and_fluid_component_collections
    ├── standalone_fluid_nodes / standalone_solid_nodes   (raw matrix solvers)
    ├── one_dimension_cartesian_conducting_medium          (1D Cartesian, no lateral coupling)
    ├── one_d_solid_array_with_lateral_coupling            (1D solid array)
    ├── one_d_fluid_array_with_lateral_coupling            (1D fluid array)
    ├── conductance_array_functions
    └── fluid_component_collection                         (series/parallel pipe networks)

Layer 4 — Pre-built components
  pre_built_components
    ├── heat_transfer_entities            HeatTransferEntity enum (unifies CVs + BCs)
    ├── non_insulated_fluid_components
    ├── insulated_pipes_and_fluid_components
    ├── non_insulated_parallel_fluid_components
    ├── shell_and_tube_heat_exchanger
    ├── one_d_solid_structure
    ├── ciet_struct_supports
    ├── ciet_heater_top_and_bottom_head_bare
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

All thermophysical property functions (`try_get_rho`, `try_get_h`, `try_get_temperature_from_h`, …) take a `Material` + temperature (+ pressure) and return the property or a `TuasLibError`. Custom materials accept function pointers so the caller can inject arbitrary correlations.

Temperature range checking is enforced via `range_check()`; property calls outside a material's valid temperature range return `TuasLibError::ThermophysicalPropertyTemperatureRangeError`.

### `SingleCVNode`
`src/lib/single_control_vol/mod.rs`

The fundamental building block — one lumped control volume node. Contains:
- `current_timestep_control_volume_specific_enthalpy` / `next_timestep_specific_enthalpy` — energy state
- `rate_enthalpy_change_vector: Vec<Power>` — accumulates power inputs during a timestep
- `mass_control_volume`, `material_control_volume`, `pressure_control_volume`, `volume`
- `max_timestep_vector` / `mesh_stability_lengthscale_vector` — auto-timestepping helpers
- `volumetric_flowrate_vector` — tracks advective flows in/out
- `temperature` — cached current temperature

**Constructors:** `new`, `new_sphere`, `new_cylinder`, `new_cylindrical_shell`, `new_block`, `new_one_dimension_volume`, `new_odd_shaped_pipe`.

**Timestep loop:**
1. Link CVs and BCs — interactions push values into `rate_enthalpy_change_vector`.
2. Call `advance_timestep` (in `calculation.rs`) — integrates powers × Δt to get `next_timestep_specific_enthalpy`.
3. Read back temperature via `get_temperature_from_enthalpy_and_set`.

### `HeatTransferEntity`
`src/lib/pre_built_components/heat_transfer_entities/`

An enum that abstracts over `SingleCVNode`, array control volumes, and boundary conditions. Use this at the top level to link components without caring about their internal type.

### `FluidComponentCollection`
`src/lib/array_control_vol_and_fluid_component_collections/fluid_component_collection/`

Handles pipe networks: computes mass flowrate given a pressure difference for components wired in series or parallel. Implements `FluidComponent` trait.

### Array CVs
- `OneDFluidArrayWithLateralCoupling` — a 1D fluid pipe discretised into N nodes; can be connected laterally (e.g., to a solid shell) for conjugate heat transfer.
- `OneDSolidArrayWithLateralCoupling` — analogous for solid structures.
- Both use matrix solvers (ndarray-linalg / OpenBLAS) for the implicit energy equation.

---

## Simulation Pattern

```rust
use tuas_boussinesq_solver::prelude::beta_testing::*;

// 1. Construct control volumes
let fluid_cv = SingleCVNode::new_cylinder(length, diameter, Material::Liquid(LiquidMaterial::TherminolVP1), T_init, P_atm)?;
let wall_cv  = SingleCVNode::new_cylindrical_shell(length, id, od, Material::Solid(SolidMaterial::SteelSS304L), T_init, P_atm)?;

// 2. Link and interact (each call pushes a power into rate_enthalpy_change_vector)
// ... use interaction functions or HeatTransferEntity wrappers ...

// 3. Advance timestep
fluid_cv.advance_timestep(dt)?;
wall_cv.advance_timestep(dt)?;

// 4. Read temperature
let T_fluid = fluid_cv.get_temperature_from_enthalpy_and_set()?;
```

For complete working examples, see the tutorials in `src/lib/pre_built_components/` test modules and the `ciet_educational_simulator` example.

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
- Use `cargo watch -x "test --release" --ignore '*.csv'` to avoid infinite re-trigger loops.
- Regression tests are co-located with the components they validate (in `tests_and_examples/` and `parasitic_heat_loss_regression_tests/` subdirectories).
- CIET steady-state natural circulation and isothermal tests validate against published Zweibaum (2015) and Zou et al. (2019) SAM data.

---

## Examples

### CIET Educational Simulator
`examples/ciet_educational_simulator/`

A real-time egui GUI that simulates the CIET loop. Includes pages for the heater, CTAH, DHX, and TCHX components, plus full-loop coupled simulation. Run with:

```bash
cargo run --example ciet_educational_simulator --release
```
