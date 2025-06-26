use uom::si::pressure::{kilopascal, pascal};
use uom::si::f64::*;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::mass_rate::kilogram_per_second;

use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
use crate::pre_built_components::gfhr_pipe_tests::multi_branch::regression::new_reactor_vessel_pipe_1_regression;
/// for flibe pipe, check if setting internal pressure changes 
/// the pressure change given
#[test]
pub fn single_pipe_internal_pressure_change_check(){

    // set initial temp 
    let initial_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(500.0);
    let mut pipe_1 = new_reactor_vessel_pipe_1_regression(initial_temperature);

    // mass flowrate 
    let mass_flowrate_zero = MassRate::new::<kilogram_per_second>(0.0);

    // pressure chg at no internal pressure (just hydrostatic)
    let pressure_chg_hydrostatic = 
        pipe_1.get_pressure_change_immutable(mass_flowrate_zero);



    // set internal pressure to 120 kpa 
    let pump_pressure = Pressure::new::<kilopascal>(120.0);
    pipe_1.set_internal_pressure_source(pump_pressure);
    let pressure_chg_hydrostatic_plus_pump = 
        pipe_1.get_pressure_change_immutable(mass_flowrate_zero);
    
    dbg!(&(pressure_chg_hydrostatic,pressure_chg_hydrostatic_plus_pump));

    approx::assert_relative_eq!(
        pressure_chg_hydrostatic.get::<pascal>(),
        -61922.87,
        max_relative=1e-5
        );
    approx::assert_relative_eq!(
        pressure_chg_hydrostatic_plus_pump.get::<pascal>(),
        100024.879,
        max_relative=1e-5
        );
}


