use std::{sync::{Arc, Mutex}, thread, time::{Duration, SystemTime}};

use uom::si::{mass_rate::kilogram_per_second, power::kilowatt};

use super::ciet_data::CIETState;


pub fn _coupled_dracs_loop_version_7(
    global_ciet_state_ptr: Arc<Mutex<CIETState>>){
    use uom::si::length::centimeter;
    use uom::si::f64::*;

    use uom::si::{frequency::hertz, ratio::ratio, time::millisecond};

    use tuas_boussinesq_solver::prelude::beta_testing::FluidArray;
    use uom::ConstZero;

    use tuas_boussinesq_solver::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
    use tuas_boussinesq_solver::pre_built_components::ciet_isothermal_test_components::*;
    use tuas_boussinesq_solver::pre_built_components::ciet_steady_state_natural_circulation_test_components::coupled_dracs_loop_tests::dhx_constructor::new_dhx_sthe_version_1;
    use tuas_boussinesq_solver::pre_built_components::ciet_steady_state_natural_circulation_test_components::coupled_dracs_loop_tests::dracs_loop_calc_functions_no_tchx_calibration::dracs_loop_dhx_tube_temperature_diagnostics;
    use tuas_boussinesq_solver::pre_built_components::ciet_steady_state_natural_circulation_test_components::coupled_dracs_loop_tests::dracs_loop_calc_functions_sam_tchx_calibration::{coupled_dracs_fluid_mechanics_calc_abs_mass_rate_sam_tchx_calibration, coupled_dracs_loop_link_up_components_sam_tchx_calibration, dracs_loop_advance_timestep_except_dhx_sam_tchx_calibration};
    use tuas_boussinesq_solver::pre_built_components::ciet_steady_state_natural_circulation_test_components::coupled_dracs_loop_tests::pri_loop_calc_functions::{coupled_dracs_pri_loop_branches_fluid_mechanics_calc_abs_mass_rate, coupled_dracs_pri_loop_dhx_heater_link_up_components, pri_loop_advance_timestep_dhx_br_and_heater_br_except_dhx, pri_loop_dhx_shell_temperature_diagnostics, pri_loop_heater_temperature_diagnostics};
    use tuas_boussinesq_solver::pre_built_components::
        ciet_steady_state_natural_circulation_test_components::dracs_loop_components::*;
    use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::time::second;

    // calibrated settings
    //

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


    let (heater_calibrated_nusselt_factor_float,
        _expt_heater_surf_temp_avg_degc,
        _simulated_expected_heater_surf_temp_degc,
        _heater_surface_temp_tolerance_degc) = 
        (10.0,109.47,105.76,5.0);
    use chem_eng_real_time_process_control_simulator::alpha_nightly::transfer_fn_wrapper_and_enums::TransferFnTraits;
    use chem_eng_real_time_process_control_simulator::alpha_nightly::controllers::ProportionalController;
    use chem_eng_real_time_process_control_simulator::alpha_nightly::controllers::AnalogController;

    // obtain local ciet state for reading and writing

    let local_ciet_state: CIETState = global_ciet_state_ptr.lock().unwrap().clone();
    let tchx_outlet_temperature_set_point_degc = 
        local_ciet_state.bt_66_tchx_outlet_set_pt_deg_c;
    let tchx_outlet_temperature_set_point = 
        ThermodynamicTemperature::new::<degree_celsius>(
            tchx_outlet_temperature_set_point_degc);
    // I'm setting timestep at 0.2, rather than 0.5, for smoother 
    // feel and experience
    let timestep = Time::new::<second>(0.2);
    let mut tchx_heat_transfer_coeff: HeatTransfer;

    let reference_tchx_htc = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(40.0);
    let average_temperature_for_density_calcs = 
        ThermodynamicTemperature::new::<degree_celsius>(80.0);

    let mut current_simulation_time = Time::ZERO;

    // PID controller settings
    // for version 5, controller settings are 
    // altered from version 4, to introduce more stability for set b9
    //
    // setting controller gain to 1.55 and 1.0 didn't work, still unstable
    let _controller_gain_original = Ratio::new::<ratio>(1.75);
    let _integral_time_original: Time = _controller_gain_original / Frequency::new::<hertz>(1.0);
    // i'm decreasing integral time to make controller hopefully more responsive
    // and increasing controller gain for the same reason
    let controller_gain = Ratio::new::<ratio>(90.75);
    let integral_time: Time = controller_gain / Frequency::new::<hertz>(5.0);
    let derivative_time: Time = Time::new::<second>(1.0);
    // derivative time ratio
    let alpha: Ratio = Ratio::new::<ratio>(1.0);

    let mut pid_controller: AnalogController = 
        AnalogController::new_filtered_pid_controller(controller_gain,
            integral_time,
            derivative_time,
            alpha).unwrap();

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
    let mut heater_ver_1 = new_heated_section_version_1_label_1_without_inner_annular_pipe(initial_temperature);
    let mut heater_bottom_head_1b = new_heater_bottom_head_1b(initial_temperature);
    let mut pipe_18 = new_pipe_18(initial_temperature);

    // calibration steps **************
    // calibrate DHX STHE 
    // calibrated thickness settings

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



    let mut _final_tchx_outlet_temperature: ThermodynamicTemperature 
        = ThermodynamicTemperature::ZERO;

    let ambient_htc = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // calculation loop (indefinite)
    //
    // to be done once every timestep
    let loop_time = SystemTime::now();
    loop {
        // so now, let's do the necessary things
        // first, timestep and loop time 
        //
        // second, read and update the local_ciet_state

        let loop_time_start = loop_time.elapsed().unwrap();
        // obtain local ciet state for reading and writing

        let mut local_ciet_state: CIETState = global_ciet_state_ptr.lock().unwrap().clone();

        let input_power_kilowatts = local_ciet_state.get_heater_power_kilowatts();
        let input_power = Power::new::<kilowatt>(input_power_kilowatts);

        // this is a safety killswitch 

        let heater_outlet_temp_degc = 
            local_ciet_state.get_heater_outlet_temp_degc();

        let heat_rate_through_heater;

        if heater_outlet_temp_degc > 150.0 {
            heat_rate_through_heater = Power::ZERO;
            local_ciet_state.set_heater_power_kilowatts(0.0);
        } else {
            heat_rate_through_heater = input_power;
        }

        let tchx_outlet_temperature_set_point_degc = 
            local_ciet_state.bt_66_tchx_outlet_set_pt_deg_c;

        let tchx_outlet_temperature_set_point = 
            ThermodynamicTemperature::new::<degree_celsius>(
                tchx_outlet_temperature_set_point_degc);


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
        //

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
                = pid_controller.set_user_input_and_calc(
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
                dimensionless_heat_trf_input * reference_tchx_htc
                + reference_tchx_htc;

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

        let absolute_mass_flowrate_pri_loop = 
            coupled_dracs_pri_loop_branches_fluid_mechanics_calc_abs_mass_rate(
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
                &pipe_17b);

        let counter_clockwise_pri_loop_flowrate = absolute_mass_flowrate_pri_loop;

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

        coupled_dracs_pri_loop_dhx_heater_link_up_components(
            counter_clockwise_pri_loop_flowrate, 
            heat_rate_through_heater, 
            average_temperature_for_density_calcs, 
            ambient_htc, 
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
            &mut pipe_17b);

        // need to calibrate dhx sthe ambient htc
        // because the coupled_dracs_pri_loop_dhx_heater_link_up_components 
        // function sets the heat transfer to ambient
        dhx_sthe.heat_transfer_to_ambient = 
            HeatTransfer::new::<watt_per_square_meter_kelvin>(
                dhx_heat_loss_to_ambient_watts_per_m2_kelvin);

        // calibrate heater to ambient htc as zero 
        heater_ver_1.calibrate_heat_transfer_to_ambient(
            HeatTransfer::ZERO);

        // advance timestep
        dracs_loop_advance_timestep_except_dhx_sam_tchx_calibration(
            timestep, &mut pipe_34, &mut pipe_33, &mut pipe_32, 
            &mut pipe_31a, &mut static_mixer_61_label_31, 
            &mut dhx_tube_side_30b, &mut dhx_tube_side_30a, 
            &mut tchx_35a, &mut tchx_35b_1, &mut tchx_35b_2,
            &mut static_mixer_60_label_36, 
            &mut pipe_36a, &mut pipe_37, &mut flowmeter_60_37a, 
            &mut pipe_38, &mut pipe_39);

        pri_loop_advance_timestep_dhx_br_and_heater_br_except_dhx(
            timestep, &mut pipe_4, &mut pipe_3, &mut pipe_2a, 
            &mut static_mixer_10_label_2, &mut heater_top_head_1a, 
            &mut heater_ver_1, &mut heater_bottom_head_1b, 
            &mut pipe_18, &mut pipe_5a, &mut pipe_26, &mut pipe_25a, 
            &mut static_mixer_21_label_25, &mut static_mixer_20_label_23, 
            &mut pipe_23a, &mut pipe_22, &mut flowmeter_20_21a, 
            &mut pipe_21, &mut pipe_20, &mut pipe_19, &mut pipe_17b);

        // for dhx, a little more care is needed to do the 
        // lateral and misc connections and advance timestep 
        // advance timestep
        //
        // by default, dhx flowrate is downwards in this setup

        let prandtl_wall_correction_setting = true; 
        let tube_side_total_mass_flowrate = -counter_clockwise_dracs_flowrate;
        let shell_side_total_mass_flowrate = counter_clockwise_pri_loop_flowrate;

        dhx_sthe.heat_transfer_to_ambient = ambient_htc;
        dhx_sthe.lateral_and_miscellaneous_connections(
            prandtl_wall_correction_setting, 
            tube_side_total_mass_flowrate, 
            shell_side_total_mass_flowrate).unwrap();

        dhx_sthe.advance_timestep(timestep).unwrap();

        let display_temperatures = true;
        // temperatures before and after heater
        let ((bt_11,_wt_10),(bt_12,_wt_13)) = 
            pri_loop_heater_temperature_diagnostics(
                &mut heater_bottom_head_1b, 
                &mut static_mixer_10_label_2, 
                display_temperatures);
        // temperatures before and after dhx shell
        let ((bt_21,_wt_20),(bt_27,_wt_26)) = 
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

        let _simulated_heater_avg_surf_temp_degc: f64 = 
            heater_avg_surf_temp.get::<degree_celsius>();

        // update the local ciet state 
        // update to 2dp

        local_ciet_state.bt_66_tchx_outlet_deg_c =
            (tchx_outlet_temperature.get::<degree_celsius>()*100.0)
            .round()/100.0;

        local_ciet_state.bt_11_heater_inlet_deg_c = 
            (bt_11.get::<degree_celsius>()*100.0)
            .round()/100.0;

        local_ciet_state.bt_12_heater_outlet_deg_c = 
            (bt_12.get::<degree_celsius>()*100.0)
            .round()/100.0;

        // primary loop DHX plus heater branch update
        {
            let pipe_1a_temp = 
                *heater_top_head_1a
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_1a_temp_degc = 
                pipe_1a_temp.get::<degree_celsius>() as f32;

            let pipe_1b_temp = 
                *heater_bottom_head_1b
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_1b_temp_degc = 
                pipe_1b_temp.get::<degree_celsius>() as f32;

            let pipe_2a_temp = 
                *pipe_2a
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_2a_temp_degc = 
                pipe_2a_temp.get::<degree_celsius>() as f32;

            let pipe_2_temp = 
                *static_mixer_10_label_2
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_2_temp_degc = 
                pipe_2_temp.get::<degree_celsius>() as f32;

            let pipe_3_temp = 
                *pipe_3
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_3_temp_degc = 
                pipe_3_temp.get::<degree_celsius>() as f32;

            let pipe_4_temp = 
                *pipe_4
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_4_temp_degc = 
                pipe_4_temp.get::<degree_celsius>() as f32;

            let pipe_18_temp = 
                *pipe_18
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_18_temp_degc = 
                pipe_18_temp.get::<degree_celsius>() as f32;

            // dhx branch 

            // note: need f32 for colour, because floats used to calculate 
            // rgb are all in f32

            let pipe_17b_temp = 
                *pipe_17b
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_17b_temp_degc = 
                pipe_17b_temp.get::<degree_celsius>() as f32;

            let pipe_19_temp = 
                *pipe_19
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_19_temp_degc = 
                pipe_19_temp.get::<degree_celsius>() as f32;

            let pipe_20_temp = 
                *pipe_20
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_20_temp_degc = 
                pipe_20_temp.get::<degree_celsius>() as f32;

            let pipe_21_temp = 
                *pipe_21
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_21_temp_degc = 
                pipe_21_temp.get::<degree_celsius>() as f32;

            // fm20 (21a)
            // mass flowrate at 3 dp

            let pipe_21a_temp = 
                *flowmeter_20_21a
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.fm20_label_21a_temp_degc = 
                pipe_21a_temp.get::<degree_celsius>() as f32;


            local_ciet_state.fm20_dhx_branch_kg_per_s = 
                ((absolute_mass_flowrate_pri_loop
                .get::<kilogram_per_second>() *1000.0).round()/1000.0
                ) as f32 ;

            let pipe_22_temp = 
                *pipe_22
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_22_temp_degc = 
                pipe_22_temp.get::<degree_celsius>() as f32;

            let pipe_23a_temp = 
                *pipe_23a
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_23a_temp_degc = 
                pipe_23a_temp.get::<degree_celsius>() as f32;


            local_ciet_state.bt_21_dhx_shell_inlet_deg_c = 
                bt_21.get::<degree_celsius>();
            local_ciet_state.pipe_25a_temp_degc = 
                local_ciet_state.bt_21_dhx_shell_inlet_deg_c as f32;

            local_ciet_state.bt_27_dhx_shell_outlet_deg_c = 
                bt_27.get::<degree_celsius>();
            local_ciet_state.pipe_23_temp_degc = 
                local_ciet_state.bt_27_dhx_shell_outlet_deg_c as f32;

            let pipe_25_temp = 
                *static_mixer_21_label_25
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_25_temp_degc = 
                pipe_25_temp.get::<degree_celsius>() as f32;

            let pipe_26_temp = 
                *pipe_26
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_26_temp_degc = 
                pipe_26_temp.get::<degree_celsius>() as f32;

            let pipe_5a_temp = 
                *pipe_5a
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_5a_temp_degc = 
                pipe_5a_temp.get::<degree_celsius>() as f32;
        }
        // DRACS loop update
        {
            let pipe_30a_temp = 
                *dhx_tube_side_30a
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_30a_temp_degc = 
                pipe_30a_temp.get::<degree_celsius>() as f32;


            let pipe_30b_temp = 
                *dhx_tube_side_30b
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_30b_temp_degc = 
                pipe_30b_temp.get::<degree_celsius>() as f32;

            let pipe_31_temp = 
                *static_mixer_61_label_31
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_31_temp_degc = 
                pipe_31_temp.get::<degree_celsius>() as f32;

            let pipe_31a_temp = 
                *pipe_31a
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_31a_temp_degc = 
                pipe_31a_temp.get::<degree_celsius>() as f32;



            let pipe_32_temp = 
                *pipe_32
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_32_temp_degc = 
                pipe_32_temp.get::<degree_celsius>() as f32;


            let pipe_33_temp = 
                *pipe_33
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_33_temp_degc = 
                pipe_33_temp.get::<degree_celsius>() as f32;

            let pipe_34_temp = 
                *pipe_34
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_34_temp_degc = 
                pipe_34_temp.get::<degree_celsius>() as f32;

            local_ciet_state.bt_65_tchx_inlet_deg_c = 
                ((local_ciet_state.pipe_34_temp_degc * 100.0).round() as f64)/100.0_f64;


            let pipe_36a_temp = 
                *pipe_36a
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_36a_temp_degc = 
                pipe_36a_temp.get::<degree_celsius>() as f32;

            let pipe_36_temp = 
                *static_mixer_60_label_36
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_36_temp_degc = 
                pipe_36_temp.get::<degree_celsius>() as f32;

            let pipe_37_temp = 
                *pipe_37
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_37_temp_degc = 
                pipe_37_temp.get::<degree_celsius>() as f32;


            // fm60 (37a)
            // 3dp

            let pipe_21a_temp = 
                *flowmeter_60_37a
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.fm60_label_37a_temp_degc = 
                pipe_21a_temp.get::<degree_celsius>() as f32;

            local_ciet_state.fm_60_dracs_kg_per_s = 
                (absolute_mass_flowrate_dracs
                .get::<kilogram_per_second>() *1000.0).round()/1000.0 ;

            let pipe_38_temp = 
                *pipe_38
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_38_temp_degc = 
                pipe_38_temp.get::<degree_celsius>() as f32;

            let pipe_39_temp = 
                *pipe_39
                .pipe_fluid_array_temperature()
                .unwrap()
                .first()
                .unwrap();

            local_ciet_state.pipe_39_temp_degc = 
                pipe_39_temp.get::<degree_celsius>() as f32;

        }



        current_simulation_time += timestep;



        // i want the calculation thread to sleep for awhile 
        // so that the simulation is in sync with real-time
        //
        // I'll give it 1 extra millisecond to do all this calculation


        let simulation_time_seconds = current_simulation_time.get::<second>();
        local_ciet_state.simulation_time_seconds = (simulation_time_seconds * 10.0).round()/10.0;

        // conditions for thread sleeping 
        let fast_forward_button_on: bool = 
            local_ciet_state.is_fast_fwd_on();

        let elapsed_time_seconds = 
            (loop_time.elapsed().unwrap().as_secs_f64() * 100.0).round()/100.0;
        local_ciet_state.elapsed_time_seconds = elapsed_time_seconds;

        let overall_simulation_in_realtime_or_faster: bool = 
            simulation_time_seconds > elapsed_time_seconds;

        // now update the ciet state 
        let loop_time_end = loop_time.elapsed().unwrap();
        let time_taken_for_calculation_loop_milliseconds: f64 = 
            (loop_time_end - loop_time_start)
            .as_millis() as f64;
        local_ciet_state.calc_time_ms = 
            time_taken_for_calculation_loop_milliseconds;

        

        let time_to_sleep_milliseconds: u64 = 
            (timestep.get::<millisecond>() - 
            time_taken_for_calculation_loop_milliseconds)
            .round().abs() as u64;

        let time_to_sleep: Duration = 
            Duration::from_millis(time_to_sleep_milliseconds - 1);


        // last condition for sleeping
        let real_time_in_current_timestep: bool = 
            time_to_sleep_milliseconds > 1;


        global_ciet_state_ptr.lock().unwrap().overwrite_state(
            local_ciet_state);

        // only sleep if simulation time is greater or equal to elapsed time 
        // or if the fast fwd button is off
        // if the current timestep took longer to calculate than real-time 
        // then don't sleep either


        // only sleep if real_time_in_current_timestep 
        // and global simulation is in real-time and fast fwd button is off 
        //
        if real_time_in_current_timestep && overall_simulation_in_realtime_or_faster
            && !fast_forward_button_on {
            thread::sleep(time_to_sleep);
        } else if real_time_in_current_timestep && overall_simulation_in_realtime_or_faster 
            && fast_forward_button_on {
            // though with fast forward on, it can be very hard 
            // to toggle off the fast forward setting due to 
            // the shared state issue 
            // I won't sleep so much in this case
            // just maybe 30 ms max
                let time_to_sleep: Duration = 
                    Duration::from_millis(30);
                thread::sleep(time_to_sleep);

        }

        else {
            // don't sleep otherwise
            //
            //
            // 

        }
        




    }



    


}
