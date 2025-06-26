/// contains code for fluid mechanics solvers 
/// for the gFHR branches
pub mod fluid_mechanics_solvers;

/// contains code iterative solution across multiple branches
pub mod multi_branch_solvers;


/// contains code iterative solution for single branches 
pub mod single_branch_solvers;

use fluid_mechanics_solvers::{four_branch_pri_and_intermediate_loop_single_time_step, FHRThermalHydraulicsState};
use uom::si::power::megawatt;
use uom::si::pressure::megapascal;
use uom::si::thermal_conductance::watt_per_kelvin;
use uom::si::time::{hour, second};
use uom::si::{mass_rate::kilogram_per_second, pressure::kilopascal};
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::ConstZero;

use crate::pre_built_components::gfhr_pipe_tests::components::new_reactor_vessel_pipe_1;
use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pipe_4_ver_2;
use crate::pre_built_components::gfhr_pipe_tests::components::gfhr_top_mixing_node_pri_loop;
use crate::pre_built_components::gfhr_pipe_tests::components::gfhr_top_mixing_node_intrmd_loop;
use crate::pre_built_components::gfhr_pipe_tests::components::gfhr_bottom_mixing_node_pri_loop;
use crate::pre_built_components::gfhr_pipe_tests::components::gfhr_bottom_mixing_node_intrmd_loop;
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

/// contains legacy code used for debugging the fhr solver for four branches
pub mod debug;
/// contains legacy code used for regression of the fhr solver for four 
/// branches, unlike debug, it uses the actual tuas library's pre-built 
/// pressure change and mass flowrate components completely
pub mod regression;

/// isothermal test checks 
/// fluid mechanics only
pub mod isothermal_flow;





/// v0.0.9 
///
/// this test is a regression test for about 30 mins of simulation time
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
///
///
#[test]
pub(crate) fn test_fhr_four_branch_full_th_long_regression(){

    let initial_temperature_pri_loop = 
        ThermodynamicTemperature::new::<degree_celsius>(500.0);
    let mut reactor_pipe_1 = new_reactor_vessel_pipe_1(initial_temperature_pri_loop);
    let mut downcomer_pipe_2 = new_downcomer_pipe_2(initial_temperature_pri_loop);
    let mut downcomer_pipe_3 = new_downcomer_pipe_3(initial_temperature_pri_loop);

    // pri loop branch (positive is in this order of flow)
    let mut fhr_pipe_11 = new_fhr_pipe_11(initial_temperature_pri_loop);
    let mut fhr_pipe_10 = new_fhr_pipe_10(initial_temperature_pri_loop);
    let mut fhr_pri_loop_pump_9 = new_fhr_pri_loop_pump_9(initial_temperature_pri_loop);
    let mut fhr_pipe_8 = new_fhr_pipe_8(initial_temperature_pri_loop);
    let mut fhr_pipe_7 = new_fhr_pipe_7(initial_temperature_pri_loop);
    // note that for HITEC, the temperature range is from 
    // 440-800K 
    // this is 167-527C
    // so intial temperature of 500C is ok
    let mut ihx_sthe_6 = new_ihx_sthe_6_version_1(initial_temperature_pri_loop);
    let mut fhr_pipe_5 = new_fhr_pipe_5(initial_temperature_pri_loop);
    let mut fhr_pipe_4 = new_fhr_pipe_4_ver_2(initial_temperature_pri_loop);


    let initial_temperature_intrmd_loop = 
        ThermodynamicTemperature::new::<degree_celsius>(500.0);
    // intermediate loop ihx side 
    // (excluding sthe)
    let mut fhr_pipe_17 = new_fhr_pipe_17(initial_temperature_intrmd_loop);
    let mut fhr_pipe_12 = new_fhr_pipe_12(initial_temperature_intrmd_loop);

    // intermediate loop steam generator side 
    let mut fhr_intrmd_loop_pump_16 = new_fhr_intermediate_loop_pump_16(
        initial_temperature_intrmd_loop);
    let mut fhr_pipe_15 = new_fhr_pipe_15(initial_temperature_intrmd_loop);
    let mut fhr_steam_generator_shell_side_14 
        = new_fhr_intermediate_loop_steam_generator_shell_side_14(
            initial_temperature_intrmd_loop);
    let mut fhr_pipe_13 = new_fhr_pipe_13(initial_temperature_intrmd_loop);


    let pri_loop_pump_pressure = Pressure::new::<megapascal>(-0.2);
    let intrmd_loop_pump_pressure = Pressure::new::<kilopascal>(-150.0);

    // mixing nodes for pri loop 
    let mut bottom_mixing_node_pri_loop = 
        gfhr_bottom_mixing_node_pri_loop(initial_temperature_pri_loop);
    let mut top_mixing_node_pri_loop = 
        gfhr_top_mixing_node_pri_loop(initial_temperature_pri_loop);
    // mixing nodes for intermediate loop 
    let mut bottom_mixing_node_intrmd_loop = 
        gfhr_bottom_mixing_node_intrmd_loop(initial_temperature_intrmd_loop);
    let mut top_mixing_node_intrmd_loop = 
        gfhr_top_mixing_node_intrmd_loop(initial_temperature_intrmd_loop);


    // timestep settings
    //
    // simulate for some time maybe 6 mins
    let timestep = Time::new::<second>(0.1);
    let mut simulation_time = Time::ZERO;
    let max_time = Time::new::<hour>(0.5);

    // steam generator settings 
    let steam_generator_tube_side_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(30.0);

    // I made this based on UA for 35 MWth heat load, and 
    // 30 degrees steam temperature, 300 degrees salt temperature
    let steam_generator_overall_ua: ThermalConductance 
        = ThermalConductance::new::<watt_per_kelvin>(1.2e5);

    // start with some initial flow rates
    let (mut reactor_branch_flow, mut downcomer_branch_1_flow, 
        mut downcomer_branch_2_flow, mut intermediate_heat_exchanger_branch_flow,
        mut intrmd_loop_ihx_br_flow,
        mut intrmd_loop_steam_gen_br_flow)
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
            &fhr_intrmd_loop_pump_16, 
            &fhr_pipe_15, 
            &fhr_steam_generator_shell_side_14, 
            &fhr_pipe_13,
            );

    let mut fhr_state = FHRThermalHydraulicsState {
        downcomer_branch_1_flow,
        downcomer_branch_2_flow,
        intermediate_heat_exchanger_branch_flow,
        intrmd_loop_ihx_br_flow,
        intrmd_loop_steam_gen_br_flow,
        reactor_branch_flow,
        simulation_time,
        reactor_temp_profile_degc: vec![],
        ihx_shell_side_temp_profile_degc: vec![],
        ihx_tube_side_temp_profile_degc: vec![],
        sg_shell_side_temp_profile_degc: vec![],
        pipe_4_temp_profile_degc: vec![],
        pipe_5_temp_profile_degc: vec![],
        pipe_7_temp_profile_degc: vec![],
        pipe_8_temp_profile_degc: vec![],
        pump_9_temp_profile_degc: vec![],
        pipe_10_temp_profile_degc: vec![],
        pipe_11_temp_profile_degc: vec![],
        pipe_12_temp_profile_degc: vec![],
        pipe_13_temp_profile_degc: vec![],
        pipe_15_temp_profile_degc: vec![],
        pump_16_temp_profile_degc: vec![],
        pipe_17_temp_profile_degc: vec![],
        downcomer_2_temp_profile_degc: vec![],
        downcomer_3_temp_profile_degc: vec![],
    };


    // main calculation loop 

    while simulation_time < max_time {

        let reactor_power = Power::new::<megawatt>(35.0);

        fhr_state = four_branch_pri_and_intermediate_loop_single_time_step(
            pri_loop_pump_pressure, 
            intrmd_loop_pump_pressure, 
            reactor_power, 
            timestep,
            simulation_time,
            &mut reactor_pipe_1, 
            &mut downcomer_pipe_2, 
            &mut downcomer_pipe_3, 
            &mut bottom_mixing_node_pri_loop,
            &mut top_mixing_node_pri_loop,
            &mut fhr_pipe_11, 
            &mut fhr_pipe_10, 
            &mut fhr_pri_loop_pump_9, 
            &mut fhr_pipe_8, 
            &mut fhr_pipe_7, 
            &mut ihx_sthe_6, 
            &mut fhr_pipe_5, 
            &mut fhr_pipe_4, 
            &mut fhr_pipe_17, 
            &mut fhr_pipe_12, 
            &mut fhr_intrmd_loop_pump_16, 
            &mut fhr_pipe_15, 
            &mut fhr_steam_generator_shell_side_14, 
            &mut fhr_pipe_13,
            &mut bottom_mixing_node_intrmd_loop,
            &mut top_mixing_node_intrmd_loop,
            steam_generator_tube_side_temperature,
            steam_generator_overall_ua,
            );


        simulation_time += timestep;
    }
    
    dbg!(&fhr_state);
    // flowrates 
    reactor_branch_flow = fhr_state.reactor_branch_flow;
    downcomer_branch_1_flow = fhr_state.downcomer_branch_1_flow;
    downcomer_branch_2_flow = fhr_state.downcomer_branch_2_flow;
    intermediate_heat_exchanger_branch_flow = 
        fhr_state.intermediate_heat_exchanger_branch_flow;
    intrmd_loop_ihx_br_flow = 
        fhr_state.intrmd_loop_ihx_br_flow;
    intrmd_loop_steam_gen_br_flow = 
        fhr_state.intrmd_loop_steam_gen_br_flow;
    // checks the final state of the fhr pri and intermediate loops
    // sim time 
    let final_simulation_time = fhr_state.simulation_time;

    // temperature profile of reactor and downcomers
    let reactor_temp_profile_degc = fhr_state.reactor_temp_profile_degc;
    let downcomer_2_temp_profile_degc 
        = fhr_state.downcomer_2_temp_profile_degc;
    let downcomer_3_temp_profile_degc 
        = fhr_state.downcomer_3_temp_profile_degc;

    // temp profile of heat exchangers and steam generators
    let ihx_shell_side_temp_profile_degc = fhr_state.ihx_shell_side_temp_profile_degc;
    let ihx_tube_side_temp_profile_degc = fhr_state.ihx_tube_side_temp_profile_degc;
    let sg_shell_side_temp_profile_degc = fhr_state.sg_shell_side_temp_profile_degc;
    
    // temp profile of pipes in pri loop
    let pipe_4_temp_profile_degc 
        = fhr_state.pipe_4_temp_profile_degc;
    let pipe_5_temp_profile_degc 
        = fhr_state.pipe_5_temp_profile_degc;
    let pipe_7_temp_profile_degc 
        = fhr_state.pipe_7_temp_profile_degc;
    let pipe_8_temp_profile_degc 
        = fhr_state.pipe_8_temp_profile_degc;
    let pump_9_temp_profile_degc 
        = fhr_state.pump_9_temp_profile_degc;
    let pipe_10_temp_profile_degc 
        = fhr_state.pipe_10_temp_profile_degc;
    let pipe_11_temp_profile_degc 
        = fhr_state.pipe_11_temp_profile_degc;


    // temp profile of pipes in intrmd loop
    let pipe_12_temp_profile_degc 
        = fhr_state.pipe_12_temp_profile_degc;
    let pipe_13_temp_profile_degc 
        = fhr_state.pipe_13_temp_profile_degc;
    let pipe_15_temp_profile_degc 
        = fhr_state.pipe_15_temp_profile_degc;
    let pump_16_temp_profile_degc 
        = fhr_state.pump_16_temp_profile_degc;
    let pipe_17_temp_profile_degc 
        = fhr_state.pipe_17_temp_profile_degc;


    // assert final simulation time
    approx::assert_relative_eq!(
        final_simulation_time.get::<second>(),
        1799.999999,
        max_relative=1e-5
        );

    // assert flowrates
    approx::assert_relative_eq!(
        reactor_branch_flow.get::<kilogram_per_second>(),
        739.4227,
        max_relative=1e-5
        );
    approx::assert_relative_eq!(
        downcomer_branch_1_flow.get::<kilogram_per_second>(),
        21.7141,
        max_relative=1e-5
        );
    approx::assert_relative_eq!(
        downcomer_branch_2_flow.get::<kilogram_per_second>(),
        68.4728,
        max_relative=1e-5
        );
    approx::assert_relative_eq!(
        intermediate_heat_exchanger_branch_flow.get::<kilogram_per_second>(),
        -829.6096,
        max_relative=1e-5
        );
    approx::assert_relative_eq!(
        intrmd_loop_ihx_br_flow.get::<kilogram_per_second>(),
        810.915,
        max_relative=1e-5
        );
    approx::assert_relative_eq!(
        intrmd_loop_steam_gen_br_flow.get::<kilogram_per_second>(),
        -810.915,
        max_relative=1e-5
        );

    // assert temperature profiles 
    assert_eq!(
        reactor_temp_profile_degc,
        vec![485.61, 485.62, 505.44, 505.45, 505.45],
    );
    assert_eq!(
        pipe_4_temp_profile_degc,
        vec![503.18, 503.18],
    );
    assert_eq!(
        pipe_5_temp_profile_degc,
        vec![503.13, 503.16],
    );
    assert_eq!(
        ihx_shell_side_temp_profile_degc,
        vec![485.64, 494.52],
    );
    assert_eq!(
        pipe_7_temp_profile_degc,
        vec![485.63, 485.63, 485.64, 485.64, 485.64],
    );
    assert_eq!(
        pipe_8_temp_profile_degc,
        vec![485.63, 485.63],
    );
    assert_eq!(
        pump_9_temp_profile_degc,
        vec![485.63, 485.63],
    );
    assert_eq!(
        pipe_10_temp_profile_degc,
        vec![485.60, 485.61, 485.61, 485.61, 
        485.62, 485.62, 485.62, 485.63],
    );
    assert_eq!(
        pipe_11_temp_profile_degc,
        vec![485.60, 485.60],
    );


    // intermediate loop
    assert_eq!(
        ihx_tube_side_temp_profile_degc,
        vec![326.05, 339.55],
    );
    assert_eq!(
        pipe_12_temp_profile_degc,
        vec![339.55, 339.56],
    );
    assert_eq!(
        pipe_13_temp_profile_degc,
        vec![339.58, 339.57],
    );
    assert_eq!(
        sg_shell_side_temp_profile_degc,
        vec![312.11, 325.84],
    );
    assert_eq!(
        pipe_15_temp_profile_degc,
        vec![312.12, 312.12],
    );
    assert_eq!(
        pump_16_temp_profile_degc,
        vec![312.13, 312.13],
    );
    assert_eq!(
        pipe_17_temp_profile_degc,
        vec![312.14, 312.15],
    );


    // downcomers
    assert_eq!(
        downcomer_2_temp_profile_degc,
        vec![485.51, 485.43, 485.35, 485.27, 485.19],
    );
    assert_eq!(
        downcomer_3_temp_profile_degc,
        vec![485.35, 485.11, 484.86, 484.62, 484.38],
    );

}



