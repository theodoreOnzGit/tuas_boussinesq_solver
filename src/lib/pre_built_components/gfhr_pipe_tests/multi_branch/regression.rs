use uom::si::{mass_rate::kilogram_per_second, pressure::kilopascal};
use uom::si::thermodynamic_temperature::degree_celsius;

use crate::pre_built_components::gfhr_pipe_tests::components::{new_downcomer_pipe_2, new_downcomer_pipe_3, new_fhr_pipe_7_old, new_fhr_pri_loop_pump_9, new_reactor_vessel_pipe_1};
use crate::pre_built_components::gfhr_pipe_tests::multi_branch::fluid_mechanics_solvers::four_branch_pri_loop_flowrates_parallel_debug_library;
use uom::si::f64::*;
/// v0.0.9 
///
/// this test checks if the internal library is functioning correctly 
/// so as to give correct mass flowrates
#[test]
pub fn test_fhr_four_branch_solver_regression_library(){

    let initial_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(500.0);
    let reactor_pipe_1 = new_reactor_vessel_pipe_1(initial_temperature);
    let downcomer_pipe_2 = new_downcomer_pipe_2(initial_temperature);
    let downcomer_pipe_3 = new_downcomer_pipe_3(initial_temperature);
    let fhr_pipe_7 = new_fhr_pipe_7_old(initial_temperature);
    let fhr_pri_loop_pump = new_fhr_pri_loop_pump_9(initial_temperature);


    let pump_pressure = Pressure::new::<kilopascal>(15.0);

    let (reactor_flow, downcomer_branch_1_flow, 
        downcomer_branch_2_flow, intermediate_heat_exchanger_branch_flow)
        = four_branch_pri_loop_flowrates_parallel_debug_library(
            pump_pressure, 
            &reactor_pipe_1, 
            &downcomer_pipe_2, 
            &downcomer_pipe_3, 
            &fhr_pipe_7, 
            &fhr_pri_loop_pump);

    dbg!(&(reactor_flow, downcomer_branch_1_flow,
            downcomer_branch_2_flow, intermediate_heat_exchanger_branch_flow
    ));

    approx::assert_relative_eq!(
        reactor_flow.get::<kilogram_per_second>(),
        -147.871,
        max_relative=1e-5
        );
    approx::assert_relative_eq!(
        downcomer_branch_1_flow.get::<kilogram_per_second>(),
        -1.04956,
        max_relative=1e-5
        );
    approx::assert_relative_eq!(
        downcomer_branch_2_flow.get::<kilogram_per_second>(),
        -1.04956,
        max_relative=1e-5
        );
    approx::assert_relative_eq!(
        intermediate_heat_exchanger_branch_flow.get::<kilogram_per_second>(),
        149.9704,
        max_relative=1e-5
        );
}



