use uom::si::{mass_rate::kilogram_per_second, pressure::kilopascal};
use uom::si::thermodynamic_temperature::degree_celsius;

use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pri_loop_pump_9;
use crate::pre_built_components::gfhr_pipe_tests::multi_branch::fluid_mechanics_solvers::four_branch_pri_loop_flowrates_parallel_debug;
use crate::pre_built_components::gfhr_pipe_tests::multi_branch::regression::new_reactor_vessel_pipe_1_regression;
use crate::pre_built_components::gfhr_pipe_tests::multi_branch::regression::new_fhr_pipe_7_regression;
use crate::pre_built_components::gfhr_pipe_tests::multi_branch::regression::new_downcomer_pipe_3_regression;
use crate::pre_built_components::gfhr_pipe_tests::multi_branch::regression::new_downcomer_pipe_2_regression;
use uom::si::f64::*;
/// supposing the pump applies 15 kPa of absolute pressure to this loop,
/// solve for flow within each of the branches.
///
/// at v0.0.7, this code crashes.
///
/// This is here to debug what is wrong with the parallel branch flow solver
///
/// the solver iterates constantly between -38 kg/s and 12 kg/s overall 
/// flowrate... unsure why
///
/// another one:
///
/// pressure across four branches oscillates between -5053.0145 and 
/// -5010, whereas mass flowrate oscillates between 7.105e-13 kg/s and 
/// -50.059 kg/s... I wonder why it is so sudden...
///
/// function appears to not be smooth in that regard
///
#[test]
pub fn test_fhr_four_branch_solver_debug(){

    let initial_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(500.0);
    let reactor_pipe_1 = new_reactor_vessel_pipe_1_regression(initial_temperature);
    let downcomer_pipe_2 = new_downcomer_pipe_2_regression(initial_temperature);
    let downcomer_pipe_3 = new_downcomer_pipe_3_regression(initial_temperature);
    let fhr_pipe_7 = new_fhr_pipe_7_regression(initial_temperature);
    let fhr_pri_loop_pump = new_fhr_pri_loop_pump_9(initial_temperature);


    let pump_pressure = Pressure::new::<kilopascal>(15.0);

    let (reactor_flow, downcomer_branch_1_flow, 
        downcomer_branch_2_flow, intermediate_heat_exchanger_branch_flow)
        = four_branch_pri_loop_flowrates_parallel_debug(
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

