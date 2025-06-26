
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
use crate::pre_built_components::gfhr_pipe_tests::multi_branch::regression::new_reactor_vessel_pipe_1_regression;
use uom::si::f64::*;
use uom::si::mass_rate::kilogram_per_second;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::pressure::{kilopascal, pascal};

/// tests for flibe pipe, getting pressure drop given mass flowrate 
/// 1200 kg/s 
#[test]
pub fn single_pipe_flibe_mass_flow_1200_kg_per_s(){
    // set initial temp 
    let initial_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(500.0);
    let pipe_1 = new_reactor_vessel_pipe_1_regression(initial_temperature);

    // mass flowrate 
    let mass_flowrate_front = MassRate::new::<kilogram_per_second>(1200.0);

    let pressure_chg_front = pipe_1.get_pressure_change_immutable(mass_flowrate_front);


    approx::assert_relative_eq!(
        pressure_chg_front.get::<pascal>(),
        -1030352.187,
        max_relative=1e-5
        );


    let mass_flowrate_back = MassRate::new::<kilogram_per_second>(-1200.0);

    let pressure_chg_back = pipe_1.get_pressure_change_immutable(mass_flowrate_back);

    approx::assert_relative_eq!(
        pressure_chg_back.get::<pascal>(),
        906506.4112,
        max_relative=1e-5
        );

}
/// tests for flibe pipe, getting pressure drop 
///
/// -983020.7855354407 Pa 
/// and 
/// 943070.5448316006 Pa 
/// respectively
/// mass flowrate should be 1200 kg/s 
#[test]
pub fn single_pipe_flibe_pressure_drop_for_mass_flow_1200_kg_per_s(){
    // set initial temp 
    let initial_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(500.0);
    let pipe_1 = new_reactor_vessel_pipe_1_regression(initial_temperature);

    // mass flowrate 
    let mass_flowrate_front_ref = MassRate::new::<kilogram_per_second>(1200.0);
    let pressure_chg_frontal = Pressure::new::<pascal>(-1030352.187);
    let mass_flowrate_front = pipe_1.get_mass_flowrate_from_pressure_change_immutable(
        pressure_chg_frontal);



    approx::assert_relative_eq!(
        mass_flowrate_front.get::<kilogram_per_second>(),
        mass_flowrate_front_ref.get::<kilogram_per_second>(),
        max_relative=1e-5
        );


    let mass_flowrate_back_ref = MassRate::new::<kilogram_per_second>(-1200.0);

    let pressure_chg_back = Pressure::new::<pascal>(906506.4112);
    let mass_flowrate_back = pipe_1.get_mass_flowrate_from_pressure_change_immutable(
        pressure_chg_back);

    approx::assert_relative_eq!(
        mass_flowrate_back.get::<kilogram_per_second>(),
        mass_flowrate_back_ref.get::<kilogram_per_second>(),
        max_relative=1e-5
        );

}


/// for flibe pipe, check if setting internal pressure changes 
/// the pressure change given
#[test]
pub fn single_pipe_flibe_internal_pressure_change_check(){

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
        -61922.873,
        max_relative=1e-5
        );
    approx::assert_relative_eq!(
        pressure_chg_hydrostatic_plus_pump.get::<pascal>(),
        58077.1269,
        max_relative=1e-5
        );
}


