use super::components::new_reactor_vessel_pipe_1;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_collection::{FluidComponentCollection, FluidComponentCollectionMethods};
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
use crate::pre_built_components::gfhr_pipe_tests::components::{new_fhr_pipe_4, new_fhr_pri_loop_pump};
use uom::si::mass_rate::kilogram_per_second;
use uom::si::pressure::{kilopascal, pascal};
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::f64::*;


/// for the reactor branch, there are some pipes involved, 
/// including the pipe representing flow through the reactor core, 
/// which is pipe 1
///
/// this test checks if getting pressure change given a fixed mass flowrate 
/// works for large flowrates, eg 1200 kg/s
#[test]
pub fn reactor_branch_test_get_pressure_change_from_mass_flowrate(){

    let initial_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(500.0);
    let reactor_pipe_1 = new_reactor_vessel_pipe_1(initial_temperature);
    let mut reactor_branch = 
        FluidComponentCollection::new_series_component_collection();

    reactor_branch.clone_and_add_component(&reactor_pipe_1);

    // mass flowrate 
    let mass_flowrate_front = MassRate::new::<kilogram_per_second>(1200.0);
    // now let's get the pressure change 
    let pressure_chg_front = 
        reactor_branch.get_pressure_change(mass_flowrate_front);
    approx::assert_relative_eq!(
        pressure_chg_front.get::<pascal>(),
        -983020.7855354407,
        max_relative=1e-5
        );
    let mass_flowrate_back = MassRate::new::<kilogram_per_second>(-1200.0);

    let pressure_chg_back = 
        reactor_branch.get_pressure_change(mass_flowrate_back);

    approx::assert_relative_eq!(
        pressure_chg_back.get::<pascal>(),
        943070.5448316006,
        max_relative=1e-5
        );
}


/// for the reactor branch, there are some pipes involved, 
/// including the pipe representing flow through the reactor core, 
/// which is pipe 1
///
///
/// this test checks if iteratively getting mass flowrate from pressure change
/// works for large flowrates, eg 1200 kg/s
#[test]
pub fn reactor_branch_test_get_mass_flowrate_from_pressure_change(){
    // set initial temp 
    let initial_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(500.0);
    let reactor_pipe_1 = new_reactor_vessel_pipe_1(initial_temperature);

    let mut reactor_branch = 
        FluidComponentCollection::new_series_component_collection();
    reactor_branch.clone_and_add_component(&reactor_pipe_1);
    // mass flowrate 
    let mass_flowrate_front_ref = MassRate::new::<kilogram_per_second>(1200.0);
    let pressure_chg_frontal = Pressure::new::<pascal>(-983020.7855354407);
    let mass_flowrate_front = 
        reactor_branch.get_mass_flowrate_from_pressure_change(pressure_chg_frontal);


    approx::assert_relative_eq!(
        mass_flowrate_front.get::<kilogram_per_second>(),
        mass_flowrate_front_ref.get::<kilogram_per_second>(),
        max_relative=1e-5
        );


    let mass_flowrate_back_ref = MassRate::new::<kilogram_per_second>(-1200.0);

    let pressure_chg_back = Pressure::new::<pascal>(943070.5448316006);
    let mass_flowrate_back = 
        reactor_branch.
        get_mass_flowrate_from_pressure_change(pressure_chg_back);

    approx::assert_relative_eq!(
        mass_flowrate_back.get::<kilogram_per_second>(),
        mass_flowrate_back_ref.get::<kilogram_per_second>(),
        max_relative=1e-5
        );

}


/// for the intermediate heat exchanger branch, there are some pipes involved, 
/// including the pipe representing flow through the reactor core, 
/// which is pipe 4 and the fhr pump, component 5 
///
/// 
///
/// this test checks if getting pressure change given a fixed mass flowrate 
/// works for large flowrates, eg 1200 kg/s
#[test]
pub fn ihx_branch_test_get_pressure_change_from_mass_flowrate(){

    let initial_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(500.0);
    let pipe_4 = new_fhr_pipe_4(initial_temperature);
    let mut pump_5 = new_fhr_pri_loop_pump(initial_temperature);
    let pump_pressure = Pressure::new::<kilopascal>(15.0);

    pump_5.set_internal_pressure_source(pump_pressure);

    let mut ihx_branch = 
        FluidComponentCollection::new_series_component_collection();

    

    ihx_branch.clone_and_add_component(&pipe_4);
    ihx_branch.clone_and_add_component(&pump_5);

    // mass flowrate 
    let mass_flowrate_front = MassRate::new::<kilogram_per_second>(1200.0);
    // now let's get the pressure change 
    let pressure_chg_front = 
        ihx_branch.get_pressure_change(mass_flowrate_front);
    approx::assert_relative_eq!(
        pressure_chg_front.get::<pascal>(),
        -9847.246086295654,
        max_relative=1e-5
        );
    let mass_flowrate_back = MassRate::new::<kilogram_per_second>(-1200.0);

    let pressure_chg_back = 
        ihx_branch.get_pressure_change(mass_flowrate_back);

    approx::assert_relative_eq!(
        pressure_chg_back.get::<pascal>(),
        -102.99461754434378,
        max_relative=1e-5
        );
}


/// for the intermediate heat exchanger branch, there are some pipes involved, 
/// including the pipe representing flow through the reactor core, 
/// which is pipe 4 and the fhr pump, component 5 
///
/// 
///
/// this test checks if getting pressure change given a fixed mass flowrate 
/// works for large flowrates, eg 1200 kg/s
#[test]
pub fn ihx_branch_test_get_mass_flowrate_from_pressure_change(){
    // set initial temp 
    let initial_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(500.0);

    let pipe_4 = new_fhr_pipe_4(initial_temperature);
    let mut pump_5 = new_fhr_pri_loop_pump(initial_temperature);
    let pump_pressure = Pressure::new::<kilopascal>(15.0);


    let mut ihx_branch = 
        FluidComponentCollection::new_series_component_collection();
    ihx_branch.clone_and_add_component(&pipe_4);
    ihx_branch.clone_and_add_component(&pump_5);
    // mass flowrate 
    let mass_flowrate_front_ref = MassRate::new::<kilogram_per_second>(1200.0);
    let pressure_chg_frontal = Pressure::new::<pascal>(-9847.246086295654);
    let mass_flowrate_front = 
        ihx_branch.get_mass_flowrate_from_pressure_change(pressure_chg_frontal);


    approx::assert_relative_eq!(
        mass_flowrate_front.get::<kilogram_per_second>(),
        mass_flowrate_front_ref.get::<kilogram_per_second>(),
        max_relative=1e-5
        );


    let mass_flowrate_back_ref = MassRate::new::<kilogram_per_second>(-1200.0);

    let pressure_chg_back = Pressure::new::<pascal>(-102.99461754434378);
    let mass_flowrate_back = 
        ihx_branch.
        get_mass_flowrate_from_pressure_change(pressure_chg_back);

    approx::assert_relative_eq!(
        mass_flowrate_back.get::<kilogram_per_second>(),
        mass_flowrate_back_ref.get::<kilogram_per_second>(),
        max_relative=1e-5
        );

}
