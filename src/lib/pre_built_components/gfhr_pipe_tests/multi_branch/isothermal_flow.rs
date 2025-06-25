use crate::pre_built_components::gfhr_pipe_tests::components::{new_fhr_pipe_4_ver_2, new_reactor_vessel_pipe_1};
use crate::pre_built_components::gfhr_pipe_tests::components::new_downcomer_pipe_3;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pipe_7;
use crate::pre_built_components::gfhr_pipe_tests::components::new_downcomer_pipe_2;
use crate::pre_built_components::gfhr_pipe_tests::components::new_ihx_sthe_6_version_1;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pri_loop_pump_9;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pipe_8;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pipe_5;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pipe_17;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pipe_15;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pipe_13;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pipe_12;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pipe_11;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pipe_10;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_intermediate_loop_steam_generator_shell_side_14;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_intermediate_loop_pump_16;
use crate::pre_built_components::gfhr_pipe_tests::multi_branch::fluid_mechanics_solvers::four_branch_pri_and_intermediate_loop_fluid_mechanics_only;
use uom::si::f64::*;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::pressure::megapascal;
use uom::si::{mass_rate::kilogram_per_second, pressure::kilopascal};
/// v0.0.9 
///
/// this test checks if the internal library is functioning correctly 
/// so as to give correct mass flowrates
///
///
/// According to KP-FHR report, 
/// https://www.nrc.gov/docs/ML2208/ML22088A231.pdf
/// The primary pump pressure head is 0.2 MPa during normal operation
///
/// for this test, we get around 733 kg/s of flow through the core
/// at said temperature and 0.2 MPa of loop pressure drop. This is 
/// less than the about 1200 kg/s of flow meant to go through the gFHR,
/// but it is in the correct order of magnitude. 
///
/// I think this test can pass
#[test]
pub fn test_fhr_four_branch_solver_pri_and_intrmd_loop_isothermal_check(){

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
    let fhr_pipe_7 = new_fhr_pipe_7(initial_temperature_pri_loop);
    // note that for HITEC, the temperature range is from 
    // 440-800K 
    // this is 167-527C
    // so intial temperature of 500C is ok
    let ihx_sthe_6 = new_ihx_sthe_6_version_1(initial_temperature_pri_loop);
    let fhr_pipe_5 = new_fhr_pipe_5(initial_temperature_pri_loop);
    let fhr_pipe_4 = new_fhr_pipe_4_ver_2(initial_temperature_pri_loop);


    let initial_temperature_intrmd_loop = 
        ThermodynamicTemperature::new::<degree_celsius>(500.0);
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


    let pri_loop_pump_pressure = Pressure::new::<megapascal>(-0.2);
    let intrmd_loop_pump_pressure = Pressure::new::<kilopascal>(-150.0);

    let (reactor_flow, downcomer_branch_1_flow, 
        downcomer_branch_2_flow, intermediate_heat_exchanger_branch_flow,
        intrmd_loop_ihx_br_flow,
        intrmd_loop_steam_gen_br_flow)
        = four_branch_pri_and_intermediate_loop_fluid_mechanics_only(
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
            downcomer_branch_2_flow, intermediate_heat_exchanger_branch_flow,
            intrmd_loop_ihx_br_flow,
            intrmd_loop_steam_gen_br_flow    
    ));

    approx::assert_relative_eq!(
        reactor_flow.get::<kilogram_per_second>(),
        734.892,
        max_relative=1e-5
        );
    approx::assert_relative_eq!(
        downcomer_branch_1_flow.get::<kilogram_per_second>(),
        21.8858,
        max_relative=1e-5
        );
    approx::assert_relative_eq!(
        downcomer_branch_2_flow.get::<kilogram_per_second>(),
        69.0525,
        max_relative=1e-5
        );
    approx::assert_relative_eq!(
        intermediate_heat_exchanger_branch_flow.get::<kilogram_per_second>(),
        -825.830,
        max_relative=1e-5
        );
    approx::assert_relative_eq!(
        intrmd_loop_ihx_br_flow.get::<kilogram_per_second>(),
        781.367,
        max_relative=1e-5
        );
    approx::assert_relative_eq!(
        intrmd_loop_steam_gen_br_flow.get::<kilogram_per_second>(),
        -781.367,
        max_relative=1e-5
        );
}



/// v0.0.9 
///
/// this test checks if the internal library is functioning correctly 
/// so as to give correct mass flowrates
///
///
/// According to KP-FHR report, 
/// https://www.nrc.gov/docs/ML2208/ML22088A231.pdf
/// The primary pump pressure head is 0.2 MPa during normal operation
///
/// when both pumps are off and there is no natural circulation driving 
/// force, then the mass flowrate should be zero in all branches
#[test]
pub fn test_fhr_four_branch_solver_pri_and_intrmd_loop_zero_flow(){

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
    let fhr_pipe_7 = new_fhr_pipe_7(initial_temperature_pri_loop);
    // note that for HITEC, the temperature range is from 
    // 440-800K 
    // this is 167-527C
    // so intial temperature of 500C is ok
    let ihx_sthe_6 = new_ihx_sthe_6_version_1(initial_temperature_pri_loop);
    let fhr_pipe_5 = new_fhr_pipe_5(initial_temperature_pri_loop);
    let fhr_pipe_4 = new_fhr_pipe_4_ver_2(initial_temperature_pri_loop);


    // make sure intermediate loop at same temperature, because 
    // the STHE is also at 500 C,
    // otherwise, will have natural circulation
    let initial_temperature_intrmd_loop = 
        ThermodynamicTemperature::new::<degree_celsius>(500.0);
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


    let pri_loop_pump_pressure = Pressure::new::<megapascal>(0.0);
    let intrmd_loop_pump_pressure = Pressure::new::<kilopascal>(0.0);

    let (reactor_flow, downcomer_branch_1_flow, 
        downcomer_branch_2_flow, intermediate_heat_exchanger_branch_flow,
        intrmd_loop_ihx_br_flow,
        intrmd_loop_steam_gen_br_flow)
        = four_branch_pri_and_intermediate_loop_fluid_mechanics_only(
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
            // intermediate loop
            &fhr_pipe_17, 
            &fhr_pipe_12, 
            &fhr_int_loop_pump_16, 
            &fhr_pipe_15, 
            &fhr_steam_generator_shell_side_14, 
            &fhr_pipe_13);

    dbg!(&(reactor_flow, downcomer_branch_1_flow, 
            downcomer_branch_2_flow, intermediate_heat_exchanger_branch_flow,
            intrmd_loop_ihx_br_flow,
            intrmd_loop_steam_gen_br_flow    
    ));

    approx::assert_abs_diff_eq!(
        reactor_flow.get::<kilogram_per_second>(),
        0.0,
        epsilon=1e-5
        );
    approx::assert_abs_diff_eq!(
        downcomer_branch_1_flow.get::<kilogram_per_second>(),
        0.0,
        epsilon=1e-5
        );
    approx::assert_abs_diff_eq!(
        downcomer_branch_2_flow.get::<kilogram_per_second>(),
        0.0,
        epsilon=1e-5
        );
    approx::assert_abs_diff_eq!(
        intermediate_heat_exchanger_branch_flow.get::<kilogram_per_second>(),
        0.0,
        epsilon=1e-5
        );
    approx::assert_abs_diff_eq!(
        intrmd_loop_ihx_br_flow.get::<kilogram_per_second>(),
        0.0,
        epsilon=1e-5
        );
    approx::assert_abs_diff_eq!(
        intrmd_loop_steam_gen_br_flow.get::<kilogram_per_second>(),
        0.0,
        epsilon=1e-5
        );
}

