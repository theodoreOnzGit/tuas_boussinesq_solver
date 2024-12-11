use std::{ops::{Deref, DerefMut}, sync::{Arc, Mutex}, thread, time::{Duration, SystemTime}};

use tuas_boussinesq_solver::{boussinesq_thermophysical_properties::LiquidMaterial, pre_built_components::ciet_three_branch_plus_dracs::{components::{new_active_ctah_horizontal, new_active_ctah_vertical}, solver_functions::{ciet_pri_loop_three_branch_link_up_components, pri_loop_three_branch_advance_timestep_except_dhx, three_branch_pri_loop_flowrates}}, prelude::beta_testing::HeatTransferEntity, single_control_vol::SingleCVNode};
use uom::si::{mass_rate::kilogram_per_second, power::kilowatt, pressure::{atmosphere, pascal}};

use crate::ciet_simulator_v1::app::panels_and_pages::ciet_data::CIETState;

pub fn _educational_ciet_loop_version_3_fluid_mechanics(
    global_ciet_state_ptr:Arc<Mutex<CIETState>>){

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
    let ctah_outlet_temperature_set_point_degc = 
        local_ciet_state.bt_41_ctah_outlet_set_pt_deg_c;
    let ctah_outlet_temperature_set_point = 
        ThermodynamicTemperature::new::<degree_celsius>(
            ctah_outlet_temperature_set_point_degc);

    // timestep settings are user set 

    let timestep = Time::new::<second>(
        local_ciet_state.get_timestep_seconds() as f64);
    let mut tchx_heat_transfer_coeff: HeatTransfer;
    let mut current_simulation_time = Time::ZERO;

    // for advection, we just use this temperature to calculate 
    // density for enthalpy flows
    let average_temperature_for_density_calcs = 
        ThermodynamicTemperature::new::<degree_celsius>(80.0);

    // controller for tchx 

    let tchx_controller_gain = Ratio::new::<ratio>(90.75);
    let tchx_integral_time: Time = tchx_controller_gain / Frequency::new::<hertz>(5.0);
    let tchx_derivative_time: Time = Time::new::<second>(1.0);
    // derivative time ratio
    let tchx_alpha: Ratio = Ratio::new::<ratio>(1.0);

    let mut tchx_pid_controller: AnalogController = 
        AnalogController::new_filtered_pid_controller(tchx_controller_gain,
            tchx_integral_time,
            tchx_derivative_time,
            tchx_alpha).unwrap();
    let tchx_measurement_delay = Time::new::<millisecond>(0.1);

    let mut tchx_measurement_delay_block: AnalogController = 
        ProportionalController::new(Ratio::new::<ratio>(1.0)).unwrap().into();

    tchx_measurement_delay_block.set_dead_time(tchx_measurement_delay);

    // controller for ctah
    let ctah_controller_gain = Ratio::new::<ratio>(90.75);
    let ctah_integral_time: Time = ctah_controller_gain / Frequency::new::<hertz>(5.0);
    let ctah_derivative_time: Time = Time::new::<second>(1.0);
    // derivative time ratio
    let ctah_alpha: Ratio = Ratio::new::<ratio>(1.0);

    let mut ctah_pid_controller: AnalogController = 
        AnalogController::new_filtered_pid_controller(ctah_controller_gain,
            ctah_integral_time,
            ctah_derivative_time,
            ctah_alpha).unwrap();
    let ctah_measurement_delay = Time::new::<millisecond>(0.1);

    let mut ctah_measurement_delay_block: AnalogController = 
        ProportionalController::new(Ratio::new::<ratio>(1.0)).unwrap().into();

    ctah_measurement_delay_block.set_dead_time(ctah_measurement_delay);

    let initial_temperature = tchx_outlet_temperature_set_point;

    // create components
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
        mixing_node_pressure).
        unwrap();

    top_mixing_node_5a_5b_4 = mixing_node.clone().into();
    bottom_mixing_node_17a_17b_18 = mixing_node.into();



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



    let mut mass_flowrate_dhx_br: MassRate 
        = MassRate::ZERO;
    let mut mass_flowrate_dracs_loop_abs: MassRate 
        = MassRate::ZERO;
    let mut mass_flowrate_ctah_br: MassRate 
        = MassRate::ZERO;
    let mut mass_flowrate_heater_br: MassRate 
        = MassRate::ZERO;

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


        // set initial mass flowrate pointers for parallelism first 

        let mass_flow_dhx_br_ptr = Arc::new(Mutex::new(mass_flowrate_dhx_br.clone()));
        let mass_flow_dracs_loop_ptr = Arc::new(Mutex::new(mass_flowrate_dracs_loop_abs.clone()));
        let mass_flow_ctah_br_ptr = Arc::new(Mutex::new(mass_flowrate_ctah_br.clone()));
        let mass_flow_heater_br_ptr = Arc::new(Mutex::new(mass_flowrate_heater_br.clone()));

        // clone the pointers to move into the mass flowrate calc

        let mass_flow_dhx_br_ptr_clone = mass_flow_dhx_br_ptr.clone();
        let mass_flow_dracs_loop_ptr_clone = mass_flow_dracs_loop_ptr.clone();
        let mass_flow_ctah_br_ptr_clone = mass_flow_ctah_br_ptr.clone();
        let mass_flow_heater_br_ptr_clone = mass_flow_heater_br_ptr.clone();

        let reference_tchx_and_ctah_htc = 
            HeatTransfer::new::<watt_per_square_meter_kelvin>(40.0);

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
                = tchx_pid_controller.set_user_input_and_calc(
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

        // now first parallel loop
        // clone all components

        let cloned_pipe_34 = pipe_34.clone();
        let cloned_pipe_33 = pipe_33.clone();
        let cloned_pipe_32 = pipe_32.clone();
        let cloned_pipe_31a = pipe_31a.clone();
        let cloned_static_mixer_61_label_31 = static_mixer_61_label_31.clone();
        let cloned_dhx_tube_side_30b = dhx_tube_side_30b.clone();
        let cloned_dhx_sthe = dhx_sthe.clone();
        let cloned_dhx_sthe_2 = dhx_sthe.clone();
        let cloned_dhx_tube_side_30a = dhx_tube_side_30a.clone();


        // DRACS cold branch or (mostly) cold leg
        let cloned_tchx_35a = tchx_35a.clone();
        let cloned_tchx_35b_1 = tchx_35b_1.clone();
        let cloned_tchx_35b_2 = tchx_35b_2.clone();
        let cloned_static_mixer_60_label_36 = static_mixer_60_label_36.clone();
        let cloned_pipe_36a = pipe_36a.clone();
        let cloned_pipe_37 = pipe_37.clone();
        let cloned_flowmeter_60_37a = flowmeter_60_37a.clone();
        let cloned_pipe_38 = pipe_38.clone();
        let cloned_pipe_39 = pipe_39.clone();

        // pri loop dhx branch top to bottom 5a to 17b 

        let cloned_pipe_5a = pipe_5a.clone();
        let cloned_pipe_26 = pipe_26.clone();
        let cloned_pipe_25a = pipe_25a.clone();
        let cloned_static_mixer_21_label_25 = static_mixer_21_label_25.clone();
        // here is where the dhx shell side should be (component 24)
        let cloned_pipe_23a = pipe_23a.clone();
        let cloned_static_mixer_20_label_23 = static_mixer_20_label_23.clone();
        let cloned_pipe_22 = pipe_22.clone();
        let cloned_flowmeter_20_21a = flowmeter_20_21a.clone();
        let cloned_pipe_21 = pipe_21.clone();
        let cloned_pipe_20 = pipe_20.clone();
        let cloned_pipe_19 = pipe_19.clone();
        let cloned_pipe_17b = pipe_17b.clone();

        // heater branch top to bottom 4 to 18
        let cloned_pipe_4 = pipe_4.clone();
        let cloned_pipe_3 = pipe_3.clone();
        let cloned_pipe_2a = pipe_2a.clone();
        let cloned_static_mixer_10_label_2 = static_mixer_10_label_2.clone();
        let cloned_heater_top_head_1a = heater_top_head_1a.clone();
        let cloned_heater_ver_1 = heater_ver_1.clone();
        let cloned_heater_bottom_head_1b = heater_bottom_head_1b.clone();
        let cloned_pipe_18 = pipe_18.clone();


        // ctah branch 
        let cloned_pipe_5b = pipe_5b.clone();
        let cloned_static_mixer_41_label_6 = static_mixer_41_label_6.clone();
        let cloned_pipe_6a = pipe_6a.clone();
        let cloned_ctah_vertical_label_7a = ctah_vertical_label_7a.clone();
        let cloned_ctah_horizontal_label_7b = ctah_horizontal_label_7b.clone();
        let cloned_pipe_8a = pipe_8a.clone();
        let cloned_static_mixer_40_label_8 = static_mixer_40_label_8.clone();
        let cloned_pipe_9 = pipe_9.clone();
        let cloned_pipe_10 = pipe_10.clone();
        let cloned_pipe_11 = pipe_11.clone();
        let cloned_pipe_12 = pipe_12.clone();
        let cloned_ctah_pump = ctah_pump.clone();
        let cloned_pipe_13 = pipe_13.clone();
        let cloned_pipe_14 = pipe_14.clone();
        let cloned_flowmeter_40_14a = flowmeter_40_14a.clone();
        let cloned_pipe_15 = pipe_15.clone();
        let cloned_pipe_16 = pipe_16.clone();
        let cloned_pipe_17a = pipe_17a.clone();

        let dracs_flowrate_join_handle = thread::spawn( move ||{

            let cloned_dhx_tube_side_heat_exchanger_30 = 
                cloned_dhx_sthe.get_clone_of_tube_side_parallel_tube_fluid_component();
            let absolute_mass_flowrate_dracs = 
                coupled_dracs_fluid_mechanics_calc_abs_mass_rate_sam_tchx_calibration(
                    &cloned_pipe_34, 
                    &cloned_pipe_33, 
                    &cloned_pipe_32, 
                    &cloned_pipe_31a, 
                    &cloned_static_mixer_61_label_31, 
                    &cloned_dhx_tube_side_30b, 
                    &cloned_dhx_tube_side_heat_exchanger_30, 
                    &cloned_dhx_tube_side_30a, 
                    &cloned_tchx_35a, 
                    &cloned_tchx_35b_1, 
                    &cloned_tchx_35b_2, 
                    &cloned_static_mixer_60_label_36, 
                    &cloned_pipe_36a, 
                    &cloned_pipe_37, 
                    &cloned_flowmeter_60_37a, 
                    &cloned_pipe_38, 
                    &cloned_pipe_39);

            // mutate the pointer 
            *mass_flow_dracs_loop_ptr_clone.lock().unwrap().deref_mut()
                = absolute_mass_flowrate_dracs;
        }
        );


        // pump pressure and valve settings, read from ciet state 
        // place holder first
        let pump_pressure = Pressure::new::<pascal>(
            local_ciet_state.get_ctah_pump_pressure_f64()
        );
        let ctah_branch_blocked = local_ciet_state.is_ctah_branch_blocked;
        let dhx_branch_blocked = local_ciet_state.is_dhx_branch_blocked;


        let pri_flowrate_join_handle = thread::spawn(move || {
            //
            // first, absolute mass flowrate across two branches
            let cloned_dhx_shell_side_pipe_24 = 
                cloned_dhx_sthe_2.get_clone_of_shell_side_fluid_component();
            // flow should go from up to down
            // this was tested ok
            let (dhx_flow, heater_flow, ctah_flow) = 
                three_branch_pri_loop_flowrates(
                    pump_pressure, 
                    ctah_branch_blocked, 
                    dhx_branch_blocked, 
                    &cloned_pipe_4, 
                    &cloned_pipe_3, 
                    &cloned_pipe_2a, 
                    &cloned_static_mixer_10_label_2, 
                    &cloned_heater_top_head_1a, 
                    &cloned_heater_ver_1, 
                    &cloned_heater_bottom_head_1b, 
                    &cloned_pipe_18, 
                    &cloned_pipe_5a, 
                    &cloned_pipe_26, 
                    &cloned_pipe_25a, 
                    &cloned_static_mixer_21_label_25, 
                    &cloned_dhx_shell_side_pipe_24, 
                    &cloned_static_mixer_20_label_23, 
                    &cloned_pipe_23a, 
                    &cloned_pipe_22, 
                    &cloned_flowmeter_20_21a, 
                    &cloned_pipe_21, 
                    &cloned_pipe_20, 
                    &cloned_pipe_19, 
                    &cloned_pipe_17b, 
                    &cloned_pipe_5b, 
                    &cloned_static_mixer_41_label_6, 
                    &cloned_pipe_6a, 
                    &cloned_ctah_vertical_label_7a, 
                    &cloned_ctah_horizontal_label_7b, 
                    &cloned_pipe_8a, 
                    &cloned_static_mixer_40_label_8, 
                    &cloned_pipe_9, 
                    &cloned_pipe_10, 
                    &cloned_pipe_11, 
                    &cloned_pipe_12, 
                    &cloned_ctah_pump, 
                    &cloned_pipe_13, 
                    &cloned_pipe_14, 
                    &cloned_flowmeter_40_14a, 
                    &cloned_pipe_15, 
                    &cloned_pipe_16, 
                    &cloned_pipe_17a);

            *mass_flow_dhx_br_ptr_clone.lock().unwrap().deref_mut() 
                = dhx_flow;
            *mass_flow_heater_br_ptr_clone.lock().unwrap().deref_mut() 
                = heater_flow;
            *mass_flow_ctah_br_ptr_clone.lock().unwrap().deref_mut() 
                = ctah_flow;


        }
        );




        // likely the natural circulation is counter clockwise 
        // now, set flowrate using the global flowrate first

        dracs_flowrate_join_handle.join().unwrap();
        pri_flowrate_join_handle.join().unwrap();


        // record and mutate global flowrates
        mass_flowrate_dracs_loop_abs = *mass_flow_dracs_loop_ptr.lock().unwrap().deref();
        mass_flowrate_dhx_br = *mass_flow_dhx_br_ptr.lock().unwrap().deref();
        mass_flowrate_ctah_br = *mass_flow_ctah_br_ptr.lock().unwrap().deref();
        mass_flowrate_heater_br = *mass_flow_heater_br_ptr.lock().unwrap().deref();



        // update the local ciet state 
        // update to 2dp
        // primary loop DHX plus heater branch update
        {


            local_ciet_state.fm20_dhx_branch_kg_per_s = 
                ((mass_flowrate_dhx_br.clone()
                  .get::<kilogram_per_second>() *1000.0).round()/1000.0
                ) as f32 ;

        }
        // DRACS loop update
        {

            local_ciet_state.bt_65_tchx_inlet_deg_c = 
                ((local_ciet_state.pipe_34_temp_degc * 100.0).round() as f64)/100.0_f64;

        }

        // still need to do ctah branch update
        {

            local_ciet_state.fm40_ctah_branch_kg_per_s = 
                (mass_flowrate_ctah_br.clone()
                 .get::<kilogram_per_second>() *1000.0).round()/1000.0;

            }


        current_simulation_time += timestep;



        // i want the calculation thread to sleep for awhile 
        // so that the simulation is in sync with real-time
        //
        // I'll give it 1 extra millisecond to do all this calculation


        let simulation_time_seconds = current_simulation_time.get::<second>();
        local_ciet_state.simulation_time_seconds = (simulation_time_seconds * 10.0).round()/10.0;

        let elapsed_time_seconds = 
            (loop_time.elapsed().unwrap().as_secs_f64() * 100.0).round()/100.0;
        local_ciet_state.elapsed_time_seconds = elapsed_time_seconds;

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

        let mut time_to_sleep: Duration = 
            Duration::from_millis(time_to_sleep_milliseconds - 1);

        // i cannot overwrite everything now, just the 
        // mass flowrates
        global_ciet_state_ptr.lock().unwrap().overwrite_state(
            local_ciet_state);

        // we will sleep only if fast forward button is off 

        if local_ciet_state.is_fast_fwd_on(){
            // sleep for just 1ms
            time_to_sleep = Duration::from_millis(1);
            thread::sleep(time_to_sleep);

        } else {

            thread::sleep(time_to_sleep);
        }


    }


}

