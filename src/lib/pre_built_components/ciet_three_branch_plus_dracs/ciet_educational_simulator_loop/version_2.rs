/// this is for testing steady state forced circulation with ctah
///
/// this is approximately based on Zou's publication 
/// where at 4000s, the pump caused fluid to flow at 0.18 kg/s 
/// through the CTAH branch and Heater branch 
/// so the DHX branch was closed
/// and 
///
/// on i7-10875H it ran at 1900s approximately 
/// test conducted on 10 dec 2024
///
/// for a 4000s simulation, it ran at twice the simulated speed. 
/// this is with single thread and timestep of 0.04s 
///
/// This means that even with single thread, it is fast enough for real-time 
/// in forced circulation, at least to prevent instabilities.
///
/// I'd still like to speed this up however, because it is still cutting it 
/// close. Like parallel threads would be good. 
///
/// Then I can calibrate the PID controller with more ease as well
///
#[cfg(test)]
#[test] 
//#[ignore = "debugging"]
pub fn ctah_flow_steady_state_test(){



    let max_simulation_time_seconds: f64 = 4000.0;
    let pri_loop_relative_tolerance = 0.061;
    let dracs_loop_relative_tolerance = 0.062;

    // the flowrates should all be zero
    //
    // heater input power in watts
    //
    // note that in this simulation, the dracs loop starts at 46C 
    // then it slowly cools off. That's why there is some natural circulation
    let (heater_input_power_watts,
        tchx_outlet_temp_set_pt_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_dhx_br_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_dhx_br_mass_flowrate_kg_per_s) 
        = (4220.0, 46.0, 0.0034689, 0.0, 0.0034689, 0.0);


    let (shell_side_to_tubes_nusselt_number_correction_factor,
        dhx_insulation_thickness_regression_cm,
        shell_side_to_ambient_nusselt_correction_factor,
        dhx_heat_loss_to_ambient_watts_per_m2_kelvin) 
        = (4.7,0.161,10.3,33.9);

    let ( pri_loop_cold_leg_insulation_thickness_cm,
        pri_loop_hot_leg_insulation_thickness_cm,
        dracs_loop_cold_leg_insulation_thickness_cm,
        dracs_loop_hot_leg_insulation_thickness_cm,) 
        = (0.15, 0.24, 3.00, 0.75);

    dbg!(max_simulation_time_seconds,
        pri_loop_relative_tolerance,
        dracs_loop_relative_tolerance);

    // heater calibration for appropriate surface temp
    let (heater_calibrated_nusselt_factor_float,
        expt_heater_surf_temp_avg_degc,
        simulated_expected_heater_surf_temp_degc,
        heater_surface_temp_tolerance_degc) = 
        (10.0,149.06,149.06,5.0);

    let ctah_pump_pressure_pascals = 16200.0;
    let ctah_flow_blocked = false;
    let dhx_flow_blocked = true;
    
    // now for ctah flow stuff

    let experimental_ctah_br_mass_flowrate_kg_per_s = 0.18;
    let regression_ctah_br_mass_flowrate_kg_per_s = 0.1937;
    let ctah_outlet_temperature_set_point_degc = 80.0;

    let expt_temperature_tolerance_degc = 0.5;

    let ( expt_heater_inlet_temp_degc, 
        expt_heater_outlet_temp_degc, 
        expt_ctah_inlet_temp_degc, 
        expt_ctah_outlet_temp_degc, ) = 
        (78.985,92.756,91.845,79.86);
    let ( regression_heater_inlet_temp_degc, 
        regression_heater_outlet_temp_degc, 
        regression_ctah_inlet_temp_degc, 
        regression_ctah_outlet_temp_degc, ) = 
        (126.29,137.66,137.07,236.95);

    // timestep 50 millisecond or 0.05 s
    // or slightly less
    let timestep_seconds: f64 = 0.04;


    three_branch_ciet_ver2(
        heater_input_power_watts, 
        max_simulation_time_seconds, 
        tchx_outlet_temp_set_pt_degc, 
        ctah_outlet_temperature_set_point_degc, 
        experimental_dracs_mass_flowrate_kg_per_s, 
        experimental_dhx_br_mass_flowrate_kg_per_s, 
        regression_ctah_br_mass_flowrate_kg_per_s, 
        simulated_expected_dracs_mass_flowrate_kg_per_s, 
        simulated_expected_dhx_br_mass_flowrate_kg_per_s, 
        pri_loop_relative_tolerance, 
        dracs_loop_relative_tolerance, 
        shell_side_to_tubes_nusselt_number_correction_factor, 
        dhx_insulation_thickness_regression_cm, 
        shell_side_to_ambient_nusselt_correction_factor, 
        dhx_heat_loss_to_ambient_watts_per_m2_kelvin, 
        pri_loop_cold_leg_insulation_thickness_cm, 
        pri_loop_hot_leg_insulation_thickness_cm, 
        dracs_loop_cold_leg_insulation_thickness_cm, 
        dracs_loop_hot_leg_insulation_thickness_cm, 
        heater_calibrated_nusselt_factor_float, 
        expt_heater_surf_temp_avg_degc, 
        simulated_expected_heater_surf_temp_degc, 
        heater_surface_temp_tolerance_degc, 
        regression_heater_outlet_temp_degc, 
        regression_heater_inlet_temp_degc, 
        regression_ctah_outlet_temp_degc, 
        regression_ctah_inlet_temp_degc, 
        expt_temperature_tolerance_degc, 
        ctah_pump_pressure_pascals, 
        ctah_flow_blocked, 
        dhx_flow_blocked,
        timestep_seconds).unwrap();


}

/// this is for testing steady state forced circulation with ctah
/// just to check if it is stable
/// and if the numbers make sense
///
/// this took about 30s and the simulation results seem to make sense
/// the max simulation time is 400s
/// it's about 10 times faster than real-time
///
#[cfg(test)]
#[test] 
pub fn ctah_flow_short_test_dhx_blocked(){



    let max_simulation_time_seconds: f64 = 400.0;
    let pri_loop_relative_tolerance = 0.061;
    let dracs_loop_relative_tolerance = 0.062;

    // the flowrates should all be zero
    //
    // heater input power in watts
    //
    let (heater_input_power_watts,
        tchx_outlet_temp_set_pt_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_dhx_br_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_dhx_br_mass_flowrate_kg_per_s) 
        = (2220.0, 46.0, 0.0, 0.0, 0.0, 0.0);


    let (shell_side_to_tubes_nusselt_number_correction_factor,
        dhx_insulation_thickness_regression_cm,
        shell_side_to_ambient_nusselt_correction_factor,
        dhx_heat_loss_to_ambient_watts_per_m2_kelvin) 
        = (4.7,0.161,10.3,33.9);

    let ( pri_loop_cold_leg_insulation_thickness_cm,
        pri_loop_hot_leg_insulation_thickness_cm,
        dracs_loop_cold_leg_insulation_thickness_cm,
        dracs_loop_hot_leg_insulation_thickness_cm,) 
        = (0.15, 0.24, 3.00, 0.75);

    dbg!(max_simulation_time_seconds,
        pri_loop_relative_tolerance,
        dracs_loop_relative_tolerance);

    // heater calibration for appropriate surface temp
    let (heater_calibrated_nusselt_factor_float,
        regression_heater_surf_temp_avg_degc,
        simulated_expected_heater_surf_temp_degc,
        heater_surface_temp_tolerance_degc) = 
        (10.0,122.05,122.05,5.0);

    let ctah_pump_pressure_pascals = 2400.0;
    let ctah_flow_blocked = false;
    let dhx_flow_blocked = true;
    
    // now for ctah flow stuff

    let regression_ctah_br_mass_flowrate_kg_per_s = 0.07366;
    let ctah_outlet_temperature_set_point_degc = 80.0;

    let expt_temperature_tolerance_degc = 0.5;

    let ( regresssion_heater_inlet_temp_degc, 
        regression_heater_outlet_temp_degc, 
        regression_ctah_inlet_temp_degc, 
        regression_ctah_outlet_temp_degc, ) = 
        (73.11,89.11,85.64,82.96);

    // timestep 
    let timestep_seconds: f64 = 0.2;


    three_branch_ciet_ver2(
        heater_input_power_watts, 
        max_simulation_time_seconds, 
        tchx_outlet_temp_set_pt_degc, 
        ctah_outlet_temperature_set_point_degc, 
        experimental_dracs_mass_flowrate_kg_per_s, 
        experimental_dhx_br_mass_flowrate_kg_per_s, 
        regression_ctah_br_mass_flowrate_kg_per_s, 
        simulated_expected_dracs_mass_flowrate_kg_per_s, 
        simulated_expected_dhx_br_mass_flowrate_kg_per_s, 
        pri_loop_relative_tolerance, 
        dracs_loop_relative_tolerance, 
        shell_side_to_tubes_nusselt_number_correction_factor, 
        dhx_insulation_thickness_regression_cm, 
        shell_side_to_ambient_nusselt_correction_factor, 
        dhx_heat_loss_to_ambient_watts_per_m2_kelvin, 
        pri_loop_cold_leg_insulation_thickness_cm, 
        pri_loop_hot_leg_insulation_thickness_cm, 
        dracs_loop_cold_leg_insulation_thickness_cm, 
        dracs_loop_hot_leg_insulation_thickness_cm, 
        heater_calibrated_nusselt_factor_float, 
        regression_heater_surf_temp_avg_degc, 
        simulated_expected_heater_surf_temp_degc, 
        heater_surface_temp_tolerance_degc, 
        regression_heater_outlet_temp_degc, 
        regresssion_heater_inlet_temp_degc, 
        regression_ctah_outlet_temp_degc, 
        regression_ctah_inlet_temp_degc, 
        expt_temperature_tolerance_degc, 
        ctah_pump_pressure_pascals, 
        ctah_flow_blocked, 
        dhx_flow_blocked,
        timestep_seconds
            ).unwrap();


}
/// this is for testing steady state forced circulation with ctah
/// just to check if it is stable
/// and if the numbers make sense
///
/// this took about 60s and the simulation results seem to make sense
/// the max simulation time is 400s
/// it's about 5 times faster than real-time
#[cfg(test)]
#[test] 
pub fn ctah_flow_short_test_reverse_diode_effect(){



    let max_simulation_time_seconds: f64 = 400.0;
    let pri_loop_relative_tolerance = 0.061;
    let dracs_loop_relative_tolerance = 0.062;

    // the flowrates should all be zero
    //
    // heater input power in watts
    //
    let (heater_input_power_watts,
        tchx_outlet_temp_set_pt_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_dhx_br_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_dhx_br_mass_flowrate_kg_per_s) 
        = (2220.0, 46.0, 0.0, 0.0, 0.0, 0.0);


    let (shell_side_to_tubes_nusselt_number_correction_factor,
        dhx_insulation_thickness_regression_cm,
        shell_side_to_ambient_nusselt_correction_factor,
        dhx_heat_loss_to_ambient_watts_per_m2_kelvin) 
        = (4.7,0.161,10.3,33.9);

    let ( pri_loop_cold_leg_insulation_thickness_cm,
        pri_loop_hot_leg_insulation_thickness_cm,
        dracs_loop_cold_leg_insulation_thickness_cm,
        dracs_loop_hot_leg_insulation_thickness_cm,) 
        = (0.15, 0.24, 3.00, 0.75);

    dbg!(max_simulation_time_seconds,
        pri_loop_relative_tolerance,
        dracs_loop_relative_tolerance);

    // heater calibration for appropriate surface temp
    let (heater_calibrated_nusselt_factor_float,
        regression_heater_surf_temp_avg_degc,
        simulated_expected_heater_surf_temp_degc,
        heater_surface_temp_tolerance_degc) = 
        (10.0,117.55,117.55,5.0);

    // to counteract natural circulation u need about this much 
    // 5000 Pa 
    let ctah_pump_pressure_pascals = 4400.0;
    let ctah_flow_blocked = false;
    let dhx_flow_blocked = false;
    
    // now for ctah flow stuff

    let regression_ctah_br_mass_flowrate_kg_per_s = 0.10;
    let ctah_outlet_temperature_set_point_degc = 80.0;

    let expt_temperature_tolerance_degc = 0.5;

    let ( regression_heater_inlet_temp_degc, 
        regression_heater_outlet_temp_degc, 
        regression_ctah_inlet_temp_degc, 
        regression_ctah_outlet_temp_degc, ) = 
        (74.99,86.6,83.92,81.88);

    // timestep 
    let timestep_seconds: f64 = 0.2;


    three_branch_ciet_ver2(
        heater_input_power_watts, 
        max_simulation_time_seconds, 
        tchx_outlet_temp_set_pt_degc, 
        ctah_outlet_temperature_set_point_degc, 
        experimental_dracs_mass_flowrate_kg_per_s, 
        experimental_dhx_br_mass_flowrate_kg_per_s, 
        regression_ctah_br_mass_flowrate_kg_per_s, 
        simulated_expected_dracs_mass_flowrate_kg_per_s, 
        simulated_expected_dhx_br_mass_flowrate_kg_per_s, 
        pri_loop_relative_tolerance, 
        dracs_loop_relative_tolerance, 
        shell_side_to_tubes_nusselt_number_correction_factor, 
        dhx_insulation_thickness_regression_cm, 
        shell_side_to_ambient_nusselt_correction_factor, 
        dhx_heat_loss_to_ambient_watts_per_m2_kelvin, 
        pri_loop_cold_leg_insulation_thickness_cm, 
        pri_loop_hot_leg_insulation_thickness_cm, 
        dracs_loop_cold_leg_insulation_thickness_cm, 
        dracs_loop_hot_leg_insulation_thickness_cm, 
        heater_calibrated_nusselt_factor_float, 
        regression_heater_surf_temp_avg_degc, 
        simulated_expected_heater_surf_temp_degc, 
        heater_surface_temp_tolerance_degc, 
        regression_heater_outlet_temp_degc, 
        regression_heater_inlet_temp_degc, 
        regression_ctah_outlet_temp_degc, 
        regression_ctah_inlet_temp_degc, 
        expt_temperature_tolerance_degc, 
        ctah_pump_pressure_pascals, 
        ctah_flow_blocked, 
        dhx_flow_blocked,
        timestep_seconds
            ).unwrap();


}

/// this function runs ciet ver 1 test, 
/// mass flowrates are calculated serially
/// for simplicity
///
/// version 1 also has no pid control for ctah
///
/// this is meant to test steady state flow on the ctah
#[cfg(test)]
pub fn three_branch_ciet_ver2(
    heater_input_power_watts: f64,
    max_time_seconds: f64,
    tchx_outlet_temperature_set_point_degc: f64,
    ctah_outlet_temperature_set_point_degc: f64,
    experimental_dracs_mass_flowrate_kg_per_s: f64,
    experimental_dhx_br_mass_flowrate_kg_per_s: f64,
    experimental_ctah_br_mass_flowrate_kg_per_s: f64,
    simulated_expected_dracs_mass_flowrate_kg_per_s: f64,
    simulated_expected_dhx_br_mass_flowrate_kg_per_s: f64,
    pri_loop_relative_tolerance: f64,
    dracs_loop_relative_tolerance: f64,
    shell_side_to_tubes_nusselt_number_correction_factor: f64,
    dhx_insulation_thickness_regression_cm: f64,
    shell_side_to_ambient_nusselt_correction_factor: f64,
    dhx_heat_loss_to_ambient_watts_per_m2_kelvin: f64,
    pri_loop_cold_leg_insulation_thickness_cm: f64,
    pri_loop_hot_leg_insulation_thickness_cm: f64,
    dracs_loop_cold_leg_insulation_thickness_cm: f64,
    dracs_loop_hot_leg_insulation_thickness_cm: f64,
    heater_calibrated_nusselt_factor_float: f64,
    expt_heater_surf_temp_avg_degc: f64,
    simulated_expected_heater_surf_temp_degc: f64,
    heater_surface_temp_tolerance_degc: f64,
    expt_heater_outlet_temp_degc: f64,
    expt_heater_inlet_temp_degc: f64,
    expt_ctah_outlet_temp_degc: f64,
    expt_ctah_inlet_temp_degc: f64,
    expt_temperature_tolerance_degc: f64,
    ctah_pump_pressure_pascals: f64,
    ctah_branch_blocked: bool,
    dhx_branch_blocked: bool,
    timestep_seconds: f64,
    ) -> 
    Result<(),crate::tuas_lib_error::TuasLibError>{
        use uom::si::length::centimeter;
        use uom::si::pressure::{atmosphere, pascal};
        use uom::si::{f64::*, mass_rate::kilogram_per_second, power::watt};

        use uom::si::{frequency::hertz, ratio::ratio, time::millisecond};

        use crate::boussinesq_thermophysical_properties::LiquidMaterial;
        use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
        use crate::pre_built_components::ciet_isothermal_test_components::*;
        use crate::pre_built_components::ciet_steady_state_natural_circulation_test_components::coupled_dracs_loop_tests::dhx_constructor::new_dhx_sthe_version_1;
        use crate::pre_built_components::ciet_steady_state_natural_circulation_test_components::coupled_dracs_loop_tests::dracs_loop_calc_functions_no_tchx_calibration::dracs_loop_dhx_tube_temperature_diagnostics;
        use crate::pre_built_components::ciet_steady_state_natural_circulation_test_components::coupled_dracs_loop_tests::dracs_loop_calc_functions_sam_tchx_calibration::{coupled_dracs_fluid_mechanics_calc_abs_mass_rate_sam_tchx_calibration, coupled_dracs_loop_link_up_components_sam_tchx_calibration, dracs_loop_advance_timestep_except_dhx_sam_tchx_calibration};
        use crate::pre_built_components::ciet_steady_state_natural_circulation_test_components::coupled_dracs_loop_tests::pri_loop_calc_functions::{pri_loop_dhx_shell_temperature_diagnostics, pri_loop_heater_temperature_diagnostics};
        use crate::pre_built_components::
            ciet_steady_state_natural_circulation_test_components::dracs_loop_components::*;
        use crate::pre_built_components::ciet_three_branch_plus_dracs::components::{new_active_ctah_horizontal, new_active_ctah_vertical};
        use crate::pre_built_components::ciet_three_branch_plus_dracs::solver_functions::{ciet_pri_loop_three_branch_link_up_components, pri_loop_three_branch_advance_timestep_except_dhx, three_branch_pri_loop_flowrates};
        use crate::prelude::beta_testing::{FluidArray, HeatTransferEntity};
        use crate::single_control_vol::SingleCVNode;
        use uom::ConstZero;

        use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};
        use uom::si::heat_transfer::watt_per_square_meter_kelvin;
        use uom::si::time::second;

        let input_power = Power::new::<watt>(heater_input_power_watts);
        let experimental_dracs_mass_flowrate = 
            MassRate::new::<kilogram_per_second>(
                experimental_dracs_mass_flowrate_kg_per_s);
        let experimental_dhx_br_mass_flowrate = 
            MassRate::new::<kilogram_per_second>(
                experimental_dhx_br_mass_flowrate_kg_per_s);

        let pump_pressure: Pressure = 
            Pressure::new::<pascal>(ctah_pump_pressure_pascals);

        let tchx_outlet_temperature_set_point = 
            ThermodynamicTemperature::new::<degree_celsius>(
                tchx_outlet_temperature_set_point_degc);
        use chem_eng_real_time_process_control_simulator::alpha_nightly::transfer_fn_wrapper_and_enums::TransferFnTraits;
        use chem_eng_real_time_process_control_simulator::alpha_nightly::controllers::ProportionalController;
        use chem_eng_real_time_process_control_simulator::alpha_nightly::controllers::AnalogController;

        // timestep settings
        let timestep = Time::new::<second>(timestep_seconds);
        let heat_rate_through_heater = input_power;
        let mut tchx_heat_transfer_coeff: HeatTransfer;

        let reference_tchx_and_ctah_htc = 
            HeatTransfer::new::<watt_per_square_meter_kelvin>(40.0);
        let average_temperature_for_density_calcs = 
            ThermodynamicTemperature::new::<degree_celsius>(80.0);

        let mut current_simulation_time = Time::ZERO;
        let max_simulation_time = Time::new::<second>(max_time_seconds);

        // PID controller settings
        // for version 5, controller settings are 
        // altered from version 4, to introduce more stability for set b9
        //
        // setting controller gain to 1.55 and 1.0 didn't work, still unstable
        let controller_gain = Ratio::new::<ratio>(1.75);
        let integral_time: Time = controller_gain / Frequency::new::<hertz>(1.0);
        let derivative_time: Time = Time::new::<second>(1.0);
        // derivative time ratio
        let alpha: Ratio = Ratio::new::<ratio>(1.0);

        let mut dhx_pid_controller: AnalogController = 
            AnalogController::new_filtered_pid_controller(controller_gain,
                integral_time,
                derivative_time,
                alpha).unwrap();

        // ctah pid controller is the same type as the dracs one
        // but I changed the gain
        let ctah_controller_gain = Ratio::new::<ratio>(4.75);
        let ctah_integral_time: Time = ctah_controller_gain / Frequency::new::<hertz>(1.0);
        let ctah_derivative_time: Time = Time::new::<second>(1.0);
        // derivative time ratio
        let ctah_alpha: Ratio = Ratio::new::<ratio>(1.0);
        let mut ctah_pid_controller = 
            AnalogController::new_filtered_pid_controller(ctah_controller_gain,
                ctah_integral_time,
                ctah_derivative_time,
                ctah_alpha).unwrap();



        // we also have a measurement delay of 0.0001 s 
        // or 0.1 ms
        let measurement_delay = Time::new::<millisecond>(0.1);

        let mut measurement_delay_block: AnalogController = 
            ProportionalController::new(Ratio::new::<ratio>(1.0)).unwrap().into();

        measurement_delay_block.set_dead_time(measurement_delay);



        let initial_temperature = tchx_outlet_temperature_set_point;

        // DRACS hot branch or (mostly) hot leg
        let mut pipe_34 = new_pipe_34(initial_temperature);
        let mut pipe_33 = new_pipe_33(initial_temperature);
        let mut pipe_32 = new_pipe_32(initial_temperature);
        let mut pipe_31a = new_pipe_31a(initial_temperature);
        let mut static_mixer_61_label_31 = new_static_mixer_61_label_31(initial_temperature);
        let mut dhx_tube_side_30b = new_dhx_tube_side_30b(initial_temperature);
        let mut dhx_sthe = new_dhx_sthe_version_1(initial_temperature);
        let mut dhx_tube_side_30a = new_dhx_tube_side_30a(initial_temperature);


        // DRACS cold branch or (mostly) cold leg
        let mut tchx_35a = new_ndhx_tchx_horizontal_35a(initial_temperature);
        let mut tchx_35b_1 = new_ndhx_tchx_vertical_35b_1(initial_temperature);
        let mut tchx_35b_2 = new_ndhx_tchx_vertical_35b_2(initial_temperature);
        let mut static_mixer_60_label_36 = new_static_mixer_60_label_36(initial_temperature);
        let mut pipe_36a = new_pipe_36a(initial_temperature);
        let mut pipe_37 = new_pipe_37(initial_temperature);
        let mut flowmeter_60_37a = new_flowmeter_60_37a(initial_temperature);
        let mut pipe_38 = new_pipe_38(initial_temperature);
        let mut pipe_39 = new_pipe_39(initial_temperature);

        // pri loop dhx branch top to bottom 5a to 17b 

        let mut pipe_5a = new_branch_5a(initial_temperature);
        let mut pipe_26 = new_pipe_26(initial_temperature);
        let mut pipe_25a = new_pipe_25a(initial_temperature);
        let mut static_mixer_21_label_25 = new_static_mixer_21_label_25(initial_temperature);
        // here is where the dhx shell side should be (component 24)
        let mut pipe_23a = new_pipe_23a(initial_temperature);
        let mut static_mixer_20_label_23 = new_static_mixer_20_label_23(initial_temperature);
        let mut pipe_22 = new_pipe_22_sam_model(initial_temperature);
        let mut flowmeter_20_21a = new_flowmeter_20_label_21a(initial_temperature);
        let mut pipe_21 = new_pipe_21(initial_temperature);
        let mut pipe_20 = new_pipe_20(initial_temperature);
        let mut pipe_19 = new_pipe_19(initial_temperature);
        let mut pipe_17b = new_branch_17b(initial_temperature);

        // heater branch top to bottom 4 to 18
        let mut pipe_4 = new_pipe_4(initial_temperature);
        let mut pipe_3 = new_pipe_3_sam_model(initial_temperature);
        let mut pipe_2a = new_pipe_2a(initial_temperature);
        let mut static_mixer_10_label_2 = new_static_mixer_10_label_2(initial_temperature);
        let mut heater_top_head_1a = new_heater_top_head_1a(initial_temperature);
        let mut heater_ver_1 = new_heated_section_version_1_label_1(initial_temperature);
        let mut heater_bottom_head_1b = new_heater_bottom_head_1b(initial_temperature);
        let mut pipe_18 = new_pipe_18(initial_temperature);


        // ctah branch 
        let mut pipe_5b = new_branch_5b(initial_temperature);
        let mut static_mixer_41_label_6 = new_static_mixer_41_label_6(
            initial_temperature);
        let mut pipe_6a = new_pipe_6a(initial_temperature);
        let mut ctah_vertical_label_7a = new_active_ctah_vertical(initial_temperature);
        let mut ctah_horizontal_label_7b = new_active_ctah_horizontal(initial_temperature);
        let mut pipe_8a = new_pipe_8a(initial_temperature);
        let mut static_mixer_40_label_8 = new_static_mixer_40_label_8(
            initial_temperature);
        let mut pipe_9 = new_pipe_9(initial_temperature);
        let mut pipe_10 = new_pipe_10(initial_temperature);
        let mut pipe_11 = new_pipe_11(initial_temperature);
        let mut pipe_12 = new_pipe_12(initial_temperature);
        let mut ctah_pump = new_ctah_pump(initial_temperature);
        let mut pipe_13 = new_pipe_13(initial_temperature);
        let mut pipe_14 = new_pipe_14(initial_temperature);
        let mut flowmeter_40_14a = new_flowmeter_40_14a(initial_temperature);
        let mut pipe_15 = new_pipe_15(initial_temperature);
        let mut pipe_16 = new_pipe_16(initial_temperature);
        let mut pipe_17a = new_branch_17a(initial_temperature);

        // mixing nodes between the pipes, should make for more elegant 
        // way of linking parallel pipes. 

        let mut top_mixing_node_5a_5b_4: HeatTransferEntity;
        let mut bottom_mixing_node_17a_17b_18: HeatTransferEntity;

        // mixing node is a sphere about diameter of ping pong ball
        // (1 in) 

        let mixing_node_diameter = Length::new::<centimeter>(3.84);
        let mixing_node_material = LiquidMaterial::TherminolVP1;
        let mixing_node_pressure = Pressure::new::<atmosphere>(1.0);
        let mixing_node = SingleCVNode::new_sphere(
            mixing_node_diameter, 
            mixing_node_material.into(), 
            initial_temperature, 
            mixing_node_pressure)
            .unwrap();

        top_mixing_node_5a_5b_4 = mixing_node.clone().into();
        bottom_mixing_node_17a_17b_18 = mixing_node.into();





        // calibration steps **************
        // calibrate DHX STHE 
        // calibrated thickness settings
        // did not calibrate ctah branch in this iteration

        let dhx_calibrated_insulation_thickness = 
            Length::new::<centimeter>(dhx_insulation_thickness_regression_cm);

        let pri_loop_cold_leg_insulation_thickness = 
            Length::new::<centimeter>(pri_loop_cold_leg_insulation_thickness_cm);
        let pri_loop_hot_leg_insulation_thickness = 
            Length::new::<centimeter>(pri_loop_hot_leg_insulation_thickness_cm);
        let dracs_loop_cold_leg_insulation_thickness = 
            Length::new::<centimeter>(dracs_loop_cold_leg_insulation_thickness_cm);
        let dracs_loop_hot_leg_insulation_thickness = 
            Length::new::<centimeter>(dracs_loop_hot_leg_insulation_thickness_cm);

        // calibrated nusselt correlation settings (using Gnielinksi correlation)

        let calibrated_nusselt_factor = 
            Ratio::new::<ratio>(shell_side_to_tubes_nusselt_number_correction_factor);

        let calibrated_parasitic_heat_loss_nusselt_factor = 
            Ratio::new::<ratio>(shell_side_to_ambient_nusselt_correction_factor);
        // calibrate heat trf coeff to environment 
        // (will need to be redone in the loop
        dhx_sthe.heat_transfer_to_ambient = 
            HeatTransfer::new::<watt_per_square_meter_kelvin>(
                dhx_heat_loss_to_ambient_watts_per_m2_kelvin);
        // calibrate shell side fluid array to tubes nusselt number correlation 

        fn calibrate_nusselt_correlation_of_heat_transfer_entity(
            nusselt_correlation: &mut NusseltCorrelation,
            calibration_ratio: Ratio){


            // it's a little bit troublesome, but we have to open 
            // up the enums and change the nusselt correlation like 
            // so


            let calibrated_nusselt_correlation = match nusselt_correlation {
                NusseltCorrelation::PipeGnielinskiGeneric(gnielinski_data) => {
                    NusseltCorrelation::PipeGnielinskiCalibrated(
                        gnielinski_data.clone(), calibration_ratio)
                },
                NusseltCorrelation::PipeGnielinskiCalibrated(gnielinski_data, _) => {
                    NusseltCorrelation::PipeGnielinskiCalibrated(
                        gnielinski_data.clone(), calibration_ratio)
                },
                _ => todo!(),
            };
            *nusselt_correlation = calibrated_nusselt_correlation;



        }

        calibrate_nusselt_correlation_of_heat_transfer_entity(
            &mut dhx_sthe.shell_side_nusselt_correlation_to_tubes, 
            calibrated_nusselt_factor);

        calibrate_nusselt_correlation_of_heat_transfer_entity(
            &mut dhx_sthe.shell_side_nusselt_correlation_parasitic, 
            calibrated_parasitic_heat_loss_nusselt_factor);

        // for the heater, i also calibrate the Nusselt correlation by 5 times,
        // to prevent the steel from overheating due to high power 
        //
        // nusselt number change and calibration should be easier though, 
        // may want some quality of life improvements for user interface in future
        let heater_calibrated_nusselt_factor = Ratio::new::<ratio>(
            heater_calibrated_nusselt_factor_float);
        let mut heater_fluid_array_clone: FluidArray 
            = heater_ver_1.pipe_fluid_array.clone().try_into().unwrap();

        calibrate_nusselt_correlation_of_heat_transfer_entity(
            &mut heater_fluid_array_clone.nusselt_correlation, 
            heater_calibrated_nusselt_factor);

        heater_ver_1.pipe_fluid_array = heater_fluid_array_clone.into();

        // now calibrate the insulation thickness for all 

        dhx_sthe.calibrate_insulation_thickness(dhx_calibrated_insulation_thickness);
        // pri loop cold leg 
        static_mixer_20_label_23.calibrate_insulation_thickness(
            pri_loop_cold_leg_insulation_thickness);
        pipe_23a.calibrate_insulation_thickness(
            pri_loop_cold_leg_insulation_thickness);
        pipe_22.calibrate_insulation_thickness(
            pri_loop_cold_leg_insulation_thickness);
        pipe_21.calibrate_insulation_thickness(
            pri_loop_cold_leg_insulation_thickness);
        // note that flowmeter is considered not insulated
        pipe_20.calibrate_insulation_thickness(
            pri_loop_cold_leg_insulation_thickness);
        pipe_19.calibrate_insulation_thickness(
            pri_loop_cold_leg_insulation_thickness);
        pipe_17b.calibrate_insulation_thickness(
            pri_loop_cold_leg_insulation_thickness);
        pipe_18.calibrate_insulation_thickness(
            pri_loop_cold_leg_insulation_thickness);
        heater_bottom_head_1b.calibrate_insulation_thickness(
            pri_loop_cold_leg_insulation_thickness);

        // pri loop hot leg 
        //
        heater_top_head_1a.calibrate_insulation_thickness(
            pri_loop_hot_leg_insulation_thickness);
        static_mixer_10_label_2.calibrate_insulation_thickness(
            pri_loop_hot_leg_insulation_thickness);
        pipe_2a.calibrate_insulation_thickness(
            pri_loop_hot_leg_insulation_thickness);
        pipe_3.calibrate_insulation_thickness(
            pri_loop_hot_leg_insulation_thickness);
        pipe_4.calibrate_insulation_thickness(
            pri_loop_hot_leg_insulation_thickness);
        pipe_5a.calibrate_insulation_thickness(
            pri_loop_hot_leg_insulation_thickness);
        pipe_26.calibrate_insulation_thickness(
            pri_loop_hot_leg_insulation_thickness);
        pipe_25a.calibrate_insulation_thickness(
            pri_loop_hot_leg_insulation_thickness);
        static_mixer_21_label_25.calibrate_insulation_thickness(
            pri_loop_hot_leg_insulation_thickness);

        // dracs loop cold leg

        static_mixer_60_label_36.calibrate_insulation_thickness(
            dracs_loop_cold_leg_insulation_thickness);
        pipe_36a.calibrate_insulation_thickness(
            dracs_loop_cold_leg_insulation_thickness);
        pipe_37.calibrate_insulation_thickness(
            dracs_loop_cold_leg_insulation_thickness);
        pipe_38.calibrate_insulation_thickness(
            dracs_loop_cold_leg_insulation_thickness);
        pipe_39.calibrate_insulation_thickness(
            dracs_loop_cold_leg_insulation_thickness);

        // dracs loop hot leg 

        pipe_31a.calibrate_insulation_thickness(
            dracs_loop_hot_leg_insulation_thickness);
        static_mixer_61_label_31.calibrate_insulation_thickness(
            dracs_loop_hot_leg_insulation_thickness);
        pipe_32.calibrate_insulation_thickness(
            dracs_loop_hot_leg_insulation_thickness);
        pipe_33.calibrate_insulation_thickness(
            dracs_loop_hot_leg_insulation_thickness);
        pipe_34.calibrate_insulation_thickness(
            dracs_loop_hot_leg_insulation_thickness);



        let mut final_mass_flowrate_dhx_br: MassRate 
            = MassRate::ZERO;
        let mut final_mass_flowrate_dracs_loop: MassRate 
            = MassRate::ZERO;
        let mut final_mass_flowrate_ctah_br: MassRate 
            = MassRate::ZERO;

        let ambient_htc = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
        // calculation loop
        while current_simulation_time < max_simulation_time {

            let tchx_outlet_temperature: ThermodynamicTemperature = {

                // the front of the tchx is connected to static mixer 
                // 60 label 36
                let tchx_35_b2_pipe_fluid_array_clone: FluidArray = 
                    tchx_35b_2.pipe_fluid_array
                    .clone()
                    .try_into()
                    .unwrap();

                // take the front single cv temperature 
                //
                // front single cv temperature is defunct
                // probably need to debug this

                let tchx_35_b2_front_single_cv_temperature: ThermodynamicTemperature 
                    = tchx_35_b2_pipe_fluid_array_clone
                    .front_single_cv
                    .temperature;



                let _tchx_35b_2_array_temperature: Vec<ThermodynamicTemperature>
                    = tchx_35b_2
                    .pipe_fluid_array_temperature()
                    .unwrap();

                //dbg!(&tchx_35b_array_temperature);

                tchx_35_b2_front_single_cv_temperature

            };
            // we will need to change the tchx heat transfer coefficient 
            // using the PID controller
            //
            // record tchx outlet temperature if it is last 5s of time 

            let tchx_temperature_record_time_threshold = max_simulation_time - 
                Time::new::<second>(5.0);


            tchx_heat_transfer_coeff = {
                // first, calculate the set point error 

                let reference_temperature_interval_deg_celsius = 80.0;

                // error = y_sp - y_measured
                let set_point_abs_error_deg_celsius = 
                    - tchx_outlet_temperature_set_point.get::<kelvin>()
                    + tchx_outlet_temperature.get::<kelvin>();

                let nondimensional_error: Ratio = 
                    (set_point_abs_error_deg_celsius/
                     reference_temperature_interval_deg_celsius).into();

                // let's get the output 

                let dimensionless_heat_trf_input: Ratio
                    = dhx_pid_controller.set_user_input_and_calc(
                        nondimensional_error, 
                        current_simulation_time).unwrap();

                // the dimensionless output is:
                //
                // (desired output - ref_val)/ref_val = dimensionless_input
                // 
                //
                // the reference value is decided by the user 
                // in this case 40 W/(m^2 K)

                let mut tchx_heat_trf_output = 
                    dimensionless_heat_trf_input * reference_tchx_and_ctah_htc
                    + reference_tchx_and_ctah_htc;

                // make sure it cannot be less than a certain amount 
                let tchx_minimum_heat_transfer = 
                    HeatTransfer::new::<watt_per_square_meter_kelvin>(
                        5.0);

                // this makes it physically realistic
                if tchx_heat_trf_output < tchx_minimum_heat_transfer {
                    tchx_heat_trf_output = tchx_minimum_heat_transfer;
                }

                tchx_heat_trf_output

            };

            // now let's caluculate the ctah heat trf coeff
            // first get the ctah outlet temperature at around 
            // pipe 8a

            let ctah_heat_transfer_coeff: HeatTransfer = {
                let ctah_outlet_temp_degc = pipe_8a
                    .pipe_fluid_array
                    .try_get_bulk_temperature()
                    .unwrap()
                    .get::<degree_celsius>();


                let reference_temperature_interval_deg_celsius = 80.0;

                // error = y_sp - y_measured
                let set_point_abs_error_deg_celsius = 
                    - ctah_outlet_temperature_set_point_degc
                    + ctah_outlet_temp_degc;

                let nondimensional_error: Ratio = 
                    (set_point_abs_error_deg_celsius/
                     reference_temperature_interval_deg_celsius).into();

                // let's get the output 

                let dimensionless_heat_trf_input: Ratio
                    = ctah_pid_controller.set_user_input_and_calc(
                        nondimensional_error, 
                        current_simulation_time).unwrap();
                // the dimensionless output is:
                //
                // (desired output - ref_val)/ref_val = dimensionless_input
                // 
                //
                // the reference value is decided by the user 
                // in this case 40 W/(m^2 K)

                let mut ctah_heat_trf_output = 
                    dimensionless_heat_trf_input * reference_tchx_and_ctah_htc
                    + reference_tchx_and_ctah_htc;

                // make sure it cannot be less than a certain amount 
                let ctah_minimum_heat_transfer = 
                    HeatTransfer::new::<watt_per_square_meter_kelvin>(
                        5.0);

                // this makes it physically realistic
                if ctah_heat_trf_output < ctah_minimum_heat_transfer {
                    ctah_heat_trf_output = ctah_minimum_heat_transfer;
                }

                ctah_heat_trf_output
            };

            // fluid calculation loop 
            //
            // first, absolute mass flowrate across two branches
            let dhx_tube_side_heat_exchanger_30 = 
                dhx_sthe.get_clone_of_tube_side_parallel_tube_fluid_component();
            let dhx_shell_side_pipe_24 = 
                dhx_sthe.get_clone_of_shell_side_fluid_component();



            let absolute_mass_flowrate_dracs = 
                coupled_dracs_fluid_mechanics_calc_abs_mass_rate_sam_tchx_calibration(
                    &pipe_34, 
                    &pipe_33, 
                    &pipe_32, 
                    &pipe_31a, 
                    &static_mixer_61_label_31, 
                    &dhx_tube_side_30b, 
                    &dhx_tube_side_heat_exchanger_30, 
                    &dhx_tube_side_30a, 
                    &tchx_35a, 
                    &tchx_35b_1, 
                    &tchx_35b_2, 
                    &static_mixer_60_label_36, 
                    &pipe_36a, 
                    &pipe_37, 
                    &flowmeter_60_37a, 
                    &pipe_38, 
                    &pipe_39);

            // likely the natural circulation is counter clockwise 
            let counter_clockwise_dracs_flowrate = absolute_mass_flowrate_dracs;


            // flow should go from up to down
            // this was tested ok
            let (dhx_flow, heater_flow, ctah_flow) = 
                three_branch_pri_loop_flowrates(
                    pump_pressure, 
                    ctah_branch_blocked, 
                    dhx_branch_blocked, 
                    &pipe_4, 
                    &pipe_3, 
                    &pipe_2a, 
                    &static_mixer_10_label_2, 
                    &heater_top_head_1a, 
                    &heater_ver_1, 
                    &heater_bottom_head_1b, 
                    &pipe_18, 
                    &pipe_5a, 
                    &pipe_26, 
                    &pipe_25a, 
                    &static_mixer_21_label_25, 
                    &dhx_shell_side_pipe_24, 
                    &static_mixer_20_label_23, 
                    &pipe_23a, 
                    &pipe_22, 
                    &flowmeter_20_21a, 
                    &pipe_21, 
                    &pipe_20, 
                    &pipe_19, 
                    &pipe_17b, 
                    &pipe_5b, 
                    &static_mixer_41_label_6, 
                    &pipe_6a, 
                    &ctah_vertical_label_7a, 
                    &ctah_horizontal_label_7b, 
                    &pipe_8a, 
                    &static_mixer_40_label_8, 
                    &pipe_9, 
                    &pipe_10, 
                    &pipe_11, 
                    &pipe_12, 
                    &ctah_pump, 
                    &pipe_13, 
                    &pipe_14, 
                    &flowmeter_40_14a, 
                    &pipe_15, 
                    &pipe_16, 
                    &pipe_17a);

            let counter_clockwise_dhx_flowrate = dhx_flow;
            // next, 
            // link up the heat transfer entities 
            // all lateral linking is done except for DHX
            //
            // note, the ambient heat transfer coefficient is not set for 
            // the DHX sthe
            coupled_dracs_loop_link_up_components_sam_tchx_calibration(
                counter_clockwise_dracs_flowrate, 
                tchx_heat_transfer_coeff, 
                average_temperature_for_density_calcs, 
                ambient_htc, 
                &mut pipe_34, 
                &mut pipe_33, 
                &mut pipe_32, 
                &mut pipe_31a, 
                &mut static_mixer_61_label_31, 
                &mut dhx_tube_side_30b, 
                &mut dhx_sthe, 
                &mut dhx_tube_side_30a, 
                &mut tchx_35a, 
                &mut tchx_35b_1, 
                &mut tchx_35b_2, 
                &mut static_mixer_60_label_36, 
                &mut pipe_36a, 
                &mut pipe_37, 
                &mut flowmeter_60_37a, 
                &mut pipe_38, 
                &mut pipe_39);

            //dbg!(&(dhx_flow,heater_flow,ctah_flow));



            ciet_pri_loop_three_branch_link_up_components(
                dhx_flow, 
                heater_flow, 
                ctah_flow, 
                heat_rate_through_heater, 
                average_temperature_for_density_calcs, 
                ambient_htc, 
                ctah_heat_transfer_coeff, 
                &mut pipe_4, 
                &mut pipe_3, 
                &mut pipe_2a, 
                &mut static_mixer_10_label_2, 
                &mut heater_top_head_1a, 
                &mut heater_ver_1, 
                &mut heater_bottom_head_1b, 
                &mut pipe_18, 
                &mut pipe_5a, 
                &mut pipe_26, 
                &mut pipe_25a, 
                &mut static_mixer_21_label_25, 
                &mut dhx_sthe, 
                &mut static_mixer_20_label_23, 
                &mut pipe_23a, 
                &mut pipe_22, 
                &mut flowmeter_20_21a, 
                &mut pipe_21, 
                &mut pipe_20, 
                &mut pipe_19, 
                &mut pipe_17b, 
                &mut pipe_5b, 
                &mut static_mixer_41_label_6, 
                &mut pipe_6a, 
                &mut ctah_vertical_label_7a, 
                &mut ctah_horizontal_label_7b, 
                &mut pipe_8a, 
                &mut static_mixer_40_label_8, 
                &mut pipe_9, 
                &mut pipe_10, 
                &mut pipe_11, 
                &mut pipe_12, 
                &mut ctah_pump, 
                &mut pipe_13, 
                &mut pipe_14, 
                &mut flowmeter_40_14a, 
                &mut pipe_15, 
                &mut pipe_16, 
                &mut pipe_17a,
                &mut top_mixing_node_5a_5b_4,
                &mut bottom_mixing_node_17a_17b_18);


            // need to calibrate dhx sthe ambient htc
            // because the coupled_dracs_pri_loop_dhx_heater_link_up_components 
            // function sets the heat transfer to ambient
            dhx_sthe.heat_transfer_to_ambient = 
                HeatTransfer::new::<watt_per_square_meter_kelvin>(
                    dhx_heat_loss_to_ambient_watts_per_m2_kelvin);

            // calibrate heater to ambient htc as zero 
            heater_ver_1.calibrate_heat_transfer_to_ambient(
                HeatTransfer::ZERO);


            // todo: need to advance timestep here

            // advance timestep
            dracs_loop_advance_timestep_except_dhx_sam_tchx_calibration(
                timestep, &mut pipe_34, &mut pipe_33, &mut pipe_32, 
                &mut pipe_31a, &mut static_mixer_61_label_31, 
                &mut dhx_tube_side_30b, &mut dhx_tube_side_30a, 
                &mut tchx_35a, &mut tchx_35b_1, &mut tchx_35b_2,
                &mut static_mixer_60_label_36, 
                &mut pipe_36a, &mut pipe_37, &mut flowmeter_60_37a, 
                &mut pipe_38, &mut pipe_39);


            // pri_loop_advance_timestep_dhx_br_and_heater_br_except_dhx(
            //     timestep, &mut pipe_4, &mut pipe_3, &mut pipe_2a, 
            //     &mut static_mixer_10_label_2, &mut heater_top_head_1a, 
            //     &mut heater_ver_1, &mut heater_bottom_head_1b, 
            //     &mut pipe_18, &mut pipe_5a, &mut pipe_26, &mut pipe_25a, 
            //     &mut static_mixer_21_label_25, &mut static_mixer_20_label_23, 
            //     &mut pipe_23a, &mut pipe_22, &mut flowmeter_20_21a, 
            //     &mut pipe_21, &mut pipe_20, &mut pipe_19, &mut pipe_17b);
            pri_loop_three_branch_advance_timestep_except_dhx(
                timestep, &mut pipe_4, &mut pipe_3, 
                &mut pipe_2a, &mut static_mixer_10_label_2, 
                &mut heater_top_head_1a, &mut heater_ver_1, 
                &mut heater_bottom_head_1b, &mut pipe_18, 
                &mut pipe_5a, &mut pipe_26, &mut pipe_25a, 
                &mut static_mixer_21_label_25, 
                &mut static_mixer_20_label_23, &mut pipe_23a, 
                &mut pipe_22, &mut flowmeter_20_21a, 
                &mut pipe_21, &mut pipe_20, &mut pipe_19, 
                &mut pipe_17b, &mut pipe_5b, 
                &mut static_mixer_41_label_6, &mut pipe_6a, 
                &mut ctah_vertical_label_7a, 
                &mut ctah_horizontal_label_7b, &mut pipe_8a, 
                &mut static_mixer_40_label_8, &mut pipe_9, 
                &mut pipe_10, &mut pipe_11, &mut pipe_12, 
                &mut ctah_pump, &mut pipe_13, &mut pipe_14, 
                &mut flowmeter_40_14a, &mut pipe_15, &mut pipe_16, 
                &mut pipe_17a, &mut top_mixing_node_5a_5b_4, 
                &mut bottom_mixing_node_17a_17b_18);

            // for dhx, a little more care is needed to do the 
            // lateral and misc connections and advance timestep 
            // advance timestep
            //
            // by default, dhx flowrate is downwards in this setup

            let prandtl_wall_correction_setting = true; 
            let tube_side_total_mass_flowrate = -counter_clockwise_dracs_flowrate;
            let shell_side_total_mass_flowrate = counter_clockwise_dhx_flowrate;

            dhx_sthe.lateral_and_miscellaneous_connections(
                prandtl_wall_correction_setting, 
                tube_side_total_mass_flowrate, 
                shell_side_total_mass_flowrate).unwrap();

            dhx_sthe.advance_timestep(timestep).unwrap();



            // record 
            if current_simulation_time > tchx_temperature_record_time_threshold {
                final_mass_flowrate_dracs_loop = counter_clockwise_dracs_flowrate;
                final_mass_flowrate_dhx_br = counter_clockwise_dhx_flowrate;
                final_mass_flowrate_ctah_br = ctah_flow.abs();
            }

            // debugging 
            let debug_settings = false;

            if debug_settings == true {
                dbg!(&current_simulation_time);
                // temperatures before and after heater
                let ((_bt_11,_wt_10),(_bt_12,_wt_13)) = 
                    pri_loop_heater_temperature_diagnostics(
                        &mut heater_bottom_head_1b, 
                        &mut static_mixer_10_label_2, 
                        debug_settings);
                // temperatures before and after dhx shell
                let ((_bt_21,_wt_20),(_bt_27,_wt_26)) = 
                    pri_loop_dhx_shell_temperature_diagnostics(
                        &mut pipe_25a, 
                        &mut static_mixer_20_label_23, 
                        debug_settings);
                // temperatures before and after dhx tube
                let ((_bt_21,_wt_20),(_bt_27,_wt_26)) = 
                    dracs_loop_dhx_tube_temperature_diagnostics(
                        &mut dhx_tube_side_30a, 
                        &mut dhx_tube_side_30b, 
                        debug_settings);
            }



            current_simulation_time += timestep;

        }

        let display_temperatures = true;
        // temperatures before and after heater
        let ((bt_11,_wt_10),(bt_12,_wt_13)) = 
            pri_loop_heater_temperature_diagnostics(
                &mut heater_bottom_head_1b, 
                &mut static_mixer_10_label_2, 
                display_temperatures);
        // temperatures before and after dhx shell
        let ((_bt_21,_wt_20),(_bt_27,_wt_26)) = 
            pri_loop_dhx_shell_temperature_diagnostics(
                &mut pipe_25a, 
                &mut static_mixer_20_label_23, 
                display_temperatures);
        // temperatures before and after dhx tube
        let ((_bt_21,_wt_20),(_bt_27,_wt_26)) = 
            dracs_loop_dhx_tube_temperature_diagnostics(
                &mut dhx_tube_side_30a, 
                &mut dhx_tube_side_30b, 
                display_temperatures);

        // heater average surface temp 
        let heater_avg_surf_temp: ThermodynamicTemperature = 
            heater_ver_1.pipe_shell.try_get_bulk_temperature().unwrap();

        let simulated_heater_avg_surf_temp_degc: f64 = 
            heater_avg_surf_temp.get::<degree_celsius>();

        // ctah inlet and ctah outlet
        // for inlet temperature, use pipe 6a and pipe 8a temperatures 
        // as proxies

        let simulated_ctah_inlet_temp_degc = pipe_6a
            .pipe_fluid_array
            .try_get_bulk_temperature()
            .unwrap()
            .get::<degree_celsius>();
        let simulated_ctah_outlet_temp_degc = pipe_8a
            .pipe_fluid_array
            .try_get_bulk_temperature()
            .unwrap()
            .get::<degree_celsius>();
        dbg!(&(
                input_power,
                final_mass_flowrate_ctah_br,
                final_mass_flowrate_dhx_br,
                final_mass_flowrate_dracs_loop,
                simulated_heater_avg_surf_temp_degc
        ));

        dbg!(&(
                bt_11.get::<degree_celsius>(),
                bt_12.get::<degree_celsius>(),
                simulated_ctah_inlet_temp_degc,
                simulated_ctah_outlet_temp_degc,
        ));



        // this asserts the final mass flowrate against experimental flowrate
        approx::assert_relative_eq!(
            experimental_dhx_br_mass_flowrate.get::<kilogram_per_second>(),
            final_mass_flowrate_dhx_br.get::<kilogram_per_second>(),
            max_relative=pri_loop_relative_tolerance);

        approx::assert_relative_eq!(
            experimental_dracs_mass_flowrate.get::<kilogram_per_second>(),
            final_mass_flowrate_dracs_loop.get::<kilogram_per_second>(),
            max_relative=dracs_loop_relative_tolerance);

        approx::assert_relative_eq!(
            experimental_ctah_br_mass_flowrate_kg_per_s,
            final_mass_flowrate_ctah_br.get::<kilogram_per_second>(),
            max_relative=dracs_loop_relative_tolerance);

        // check heater surface temp to within tolerance 


        // this asserts the final mass flowrate against experimental flowrate 
        // for regression to within 0.1%
        approx::assert_relative_eq!(
            simulated_expected_dhx_br_mass_flowrate_kg_per_s,
            final_mass_flowrate_dhx_br.get::<kilogram_per_second>(),
            max_relative=0.001);

        approx::assert_relative_eq!(
            simulated_expected_dracs_mass_flowrate_kg_per_s,
            final_mass_flowrate_dracs_loop.get::<kilogram_per_second>(),
            max_relative=0.001);

        // also assert heater surface temp to within 0.1%
        approx::assert_relative_eq!(
            simulated_expected_heater_surf_temp_degc,
            simulated_heater_avg_surf_temp_degc,
            max_relative=0.001);

        approx::assert_abs_diff_eq!(
            expt_heater_surf_temp_avg_degc,
            simulated_heater_avg_surf_temp_degc,
            epsilon=heater_surface_temp_tolerance_degc);

        // heater inlet and outlet check to user set temperature tolerance 

        approx::assert_abs_diff_eq!(
            expt_heater_inlet_temp_degc,
            bt_11.get::<degree_celsius>(),
            epsilon=expt_temperature_tolerance_degc);

        approx::assert_abs_diff_eq!(
            expt_heater_outlet_temp_degc,
            bt_12.get::<degree_celsius>(),
            epsilon=expt_temperature_tolerance_degc);

        // ctah inlet and outlet temperature check to user set temperature 
        // tolerance 

        approx::assert_abs_diff_eq!(
            expt_ctah_inlet_temp_degc,
            simulated_ctah_inlet_temp_degc,
            epsilon=expt_temperature_tolerance_degc);

        approx::assert_abs_diff_eq!(
            expt_ctah_outlet_temp_degc,
            simulated_ctah_outlet_temp_degc,
            epsilon=expt_temperature_tolerance_degc);

        Ok(())

    }
