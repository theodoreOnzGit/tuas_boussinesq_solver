/// contains code for fluid mechanics solvers 
/// for the gFHR branches
pub mod fluid_mechanics_solvers;

/// contains code iterative solution across multiple branches
pub mod multi_branch_solvers;


/// contains code iterative solution for single branches 
pub mod single_branch_solvers;

use uom::si::{mass_rate::kilogram_per_second, pressure::kilopascal};
use uom::si::thermodynamic_temperature::degree_celsius;

use crate::pre_built_components::gfhr_pipe_tests::components::new_reactor_vessel_pipe_1;
use crate::pre_built_components::gfhr_pipe_tests::components::new_ihx_sthe_6_version_1;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pri_loop_pump_9;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pipe_8;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pipe_7_old;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pipe_5;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pipe_4;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pipe_17;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pipe_15;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pipe_13;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pipe_12;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pipe_11;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pipe_10;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_intermediate_loop_steam_generator_shell_side_14;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_intermediate_loop_pump_16;
use crate::pre_built_components::gfhr_pipe_tests::components::new_downcomer_pipe_3;
use crate::pre_built_components::gfhr_pipe_tests::components::new_downcomer_pipe_2;
use crate::pre_built_components::gfhr_pipe_tests::multi_branch::fluid_mechanics_solvers::four_branch_pri_and_intermediate_loop;
use uom::si::f64::*;

/// contains legacy code used for debugging the fhr solver for four branches
pub mod debug;
/// contains legacy code used for regression of the fhr solver for four 
/// branches, unlike debug, it uses the actual tuas library's pre-built 
/// pressure change and mass flowrate components completely
pub mod regression;


/// v0.0.9 
///
/// this test checks if the internal library is functioning correctly 
/// so as to give correct mass flowrates
#[test]
pub fn test_fhr_four_branch_solver_pri_and_intrmd_loop(){

    let initial_temperature_pri_loop = 
        ThermodynamicTemperature::new::<degree_celsius>(500.0);
    let reactor_pipe_1 = new_reactor_vessel_pipe_1(initial_temperature_pri_loop);
    let downcomer_pipe_2 = new_downcomer_pipe_2(initial_temperature_pri_loop);
    let downcomer_pipe_3 = new_downcomer_pipe_3(initial_temperature_pri_loop);

    // pri loop branch (positive is in this order of flow)
    let fhr_pipe_11 = new_fhr_pipe_11(initial_temperature_pri_loop);
    let fhr_pipe_10 = new_fhr_pipe_10(initial_temperature_pri_loop);
    let fhr_pri_loop_pump_9 = new_fhr_pri_loop_pump_9(initial_temperature_pri_loop);
    let fhr_pipe_8 = new_fhr_pipe_8(initial_temperature_pri_loop);
    let fhr_pipe_7 = new_fhr_pipe_7_old(initial_temperature_pri_loop);
    // note that for HITEC, the temperature range is from 
    // 440-800K 
    // this is 167-527C
    // so intial temperature of 500C is ok
    let ihx_sthe_6 = new_ihx_sthe_6_version_1(initial_temperature_pri_loop);
    let fhr_pipe_5 = new_fhr_pipe_5(initial_temperature_pri_loop);
    let fhr_pipe_4 = new_fhr_pipe_4(initial_temperature_pri_loop);


    let initial_temperature_intrmd_loop = 
        ThermodynamicTemperature::new::<degree_celsius>(350.0);
    // intermediate loop ihx side 
    // (excluding sthe)
    let fhr_pipe_17 = new_fhr_pipe_17(initial_temperature_intrmd_loop);
    let fhr_pipe_12 = new_fhr_pipe_12(initial_temperature_intrmd_loop);

    // intermediate loop steam generator side 
    let fhr_int_loop_pump_16 = new_fhr_intermediate_loop_pump_16(
        initial_temperature_intrmd_loop);
    let fhr_pipe_15 = new_fhr_pipe_15(initial_temperature_intrmd_loop);
    let fhr_steam_generator_shell_side_14 
        = new_fhr_intermediate_loop_steam_generator_shell_side_14(
            initial_temperature_intrmd_loop);
    let fhr_pipe_13 = new_fhr_pipe_13(initial_temperature_intrmd_loop);


    let pri_loop_pump_pressure = Pressure::new::<kilopascal>(-15.0);
    let intrmd_loop_pump_pressure = Pressure::new::<kilopascal>(-15.0);

    let (reactor_flow, downcomer_branch_1_flow, 
        downcomer_branch_2_flow, intermediate_heat_exchanger_branch_flow)
        = four_branch_pri_and_intermediate_loop(
            pri_loop_pump_pressure, 
            intrmd_loop_pump_pressure, 
            &reactor_pipe_1, 
            &downcomer_pipe_2, 
            &downcomer_pipe_3, 
            &fhr_pipe_11, 
            &fhr_pipe_10, 
            &fhr_pri_loop_pump_9, 
            &fhr_pipe_8, 
            &fhr_pipe_7, 
            &ihx_sthe_6, 
            &fhr_pipe_5, 
            &fhr_pipe_4, 
            &fhr_pipe_17, 
            &fhr_pipe_12, 
            &fhr_int_loop_pump_16, 
            &fhr_pipe_15, 
            &fhr_steam_generator_shell_side_14, 
            &fhr_pipe_13);

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



