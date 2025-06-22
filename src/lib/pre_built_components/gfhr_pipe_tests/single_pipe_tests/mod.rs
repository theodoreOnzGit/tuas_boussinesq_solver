
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
use crate::pre_built_components::gfhr_pipe_tests::components::new_reactor_vessel_pipe_1;
use uom::si::f64::*;
use uom::si::mass_rate::kilogram_per_second;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::pressure::pascal;

/// tests for flibe pipe, getting pressure drop given mass flowrate 
/// 1200 kg/s 
#[test]
pub fn single_pipe_flibe_mass_flow_1200_kg_per_s(){
    // set initial temp 
    let initial_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(500.0);
    let pipe_1 = new_reactor_vessel_pipe_1(initial_temperature);

    // mass flowrate 
    let mass_flowrate_front = MassRate::new::<kilogram_per_second>(1200.0);

    let pressure_chg_front = pipe_1.get_pressure_change_immutable(mass_flowrate_front);


    approx::assert_relative_eq!(
        pressure_chg_front.get::<pascal>(),
        -983020.7855354407,
        max_relative=1e-5
        );


    let mass_flowrate_back = MassRate::new::<kilogram_per_second>(-1200.0);

    let pressure_chg_back = pipe_1.get_pressure_change_immutable(mass_flowrate_back);

    approx::assert_relative_eq!(
        pressure_chg_back.get::<pascal>(),
        943070.5448316006,
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
    let pipe_1 = new_reactor_vessel_pipe_1(initial_temperature);

    // mass flowrate 
    let mass_flowrate_front_ref = MassRate::new::<kilogram_per_second>(1200.0);
    let pressure_chg_frontal = Pressure::new::<pascal>(-983020.7855354407);
    let mass_flowrate_front = pipe_1.get_mass_flowrate_from_pressure_change_immutable(
        pressure_chg_frontal);



    approx::assert_relative_eq!(
        mass_flowrate_front.get::<kilogram_per_second>(),
        mass_flowrate_front_ref.get::<kilogram_per_second>(),
        max_relative=1e-5
        );


    let mass_flowrate_back_ref = MassRate::new::<kilogram_per_second>(-1200.0);

    let pressure_chg_back = Pressure::new::<pascal>(943070.5448316006);
    let mass_flowrate_back = pipe_1.get_mass_flowrate_from_pressure_change_immutable(
        pressure_chg_back);

    approx::assert_relative_eq!(
        mass_flowrate_back.get::<kilogram_per_second>(),
        mass_flowrate_back_ref.get::<kilogram_per_second>(),
        max_relative=1e-5
        );

}


