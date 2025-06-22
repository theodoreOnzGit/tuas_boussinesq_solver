use super::components::new_reactor_vessel_pipe_1;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_collection::{FluidComponentCollection, FluidComponentCollectionMethods};
use uom::si::mass_rate::kilogram_per_second;
use uom::si::pressure::pascal;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::f64::*;


/// for the reactor branch, there are some pipes involved, 
/// including the pipe representing flow through the reactor core, 
/// which is pipe 1
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
