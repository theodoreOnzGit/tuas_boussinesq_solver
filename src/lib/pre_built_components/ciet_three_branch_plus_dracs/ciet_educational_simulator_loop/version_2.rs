use uom::si::f64::*;

use crate::prelude::beta_testing::HeatTransferEntity;
use crate::pre_built_components::{non_insulated_fluid_components::NonInsulatedFluidComponent, shell_and_tube_heat_exchanger::SimpleShellAndTubeHeatExchanger};
use crate::pre_built_components::insulated_pipes_and_fluid_components::InsulatedFluidComponent;


/// this function runs ciet ver 2 test, 
/// mass flowrates are calculated in parallel 
/// along with the heat transfer entity linking
/// for speed enhancements
///
///
/// version 2 also has no pid control for ctah
#[cfg(test)]
pub fn three_branch_ciet_ver2(
    input_power_watts: f64,
    max_time_seconds: f64,
    tchx_outlet_temperature_set_point_degc: f64,
    experimental_dracs_mass_flowrate_kg_per_s: f64,
    experimental_primary_mass_flowrate_kg_per_s: f64,
    simulated_expected_dracs_mass_flowrate_kg_per_s: f64,
    simulated_expected_primary_mass_flowrate_kg_per_s: f64,
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
    ctah_pump_pressure_pascals: f64,
    ctah_branch_blocked: bool,
    dhx_branch_blocked: bool) -> 
    Result<(),crate::tuas_lib_error::TuasLibError>{
        use std::ops::DerefMut;
        use std::sync::{Arc, Mutex};

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
        use crate::pre_built_components::ciet_steady_state_natural_circulation_test_components::coupled_dracs_loop_tests::pri_loop_calc_functions::{coupled_dracs_pri_loop_branches_fluid_mechanics_calc_abs_mass_rate, coupled_dracs_pri_loop_dhx_heater_link_up_components, pri_loop_advance_timestep_dhx_br_and_heater_br_except_dhx, pri_loop_dhx_shell_temperature_diagnostics, pri_loop_heater_temperature_diagnostics};
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

        let input_power = Power::new::<watt>(input_power_watts);
        let experimental_dracs_mass_flowrate = 
            MassRate::new::<kilogram_per_second>(
                experimental_dracs_mass_flowrate_kg_per_s);
        let experimental_primary_mass_flowrate = 
            MassRate::new::<kilogram_per_second>(
                experimental_primary_mass_flowrate_kg_per_s);

        let pump_pressure: Pressure = 
            Pressure::new::<pascal>(ctah_pump_pressure_pascals);

        let tchx_outlet_temperature_set_point = 
            ThermodynamicTemperature::new::<degree_celsius>(
                tchx_outlet_temperature_set_point_degc);
        use chem_eng_real_time_process_control_simulator::alpha_nightly::transfer_fn_wrapper_and_enums::TransferFnTraits;
        use chem_eng_real_time_process_control_simulator::alpha_nightly::controllers::ProportionalController;
        use chem_eng_real_time_process_control_simulator::alpha_nightly::controllers::AnalogController;

        // max error is 0.5% according to SAM 
        // is okay, because typical flowmeter measurement error is 2% anyway
        // set timestep to lower values for set b9
        // as compared to the rest
        //
        // setting to 0.01s didn't work, so my second candidate for change is 
        // to change the controller, but set timestep at 0.5s
        //
        // This is because this dataset b9, has the highest heater power 
        // but lowest TCHX outlet temperature of all datasets. And therefore, 
        // the highest cooling loads are placed on the TCHX 
        //
        // It is understandable at this extreme then, for the controller 
        // to be unstable if we don't change settings
        //
        // let timestep = Time::new::<second>(0.1);
        // for this timestep, the simulation fails around 181s of simulated time
        //
        //
        // let timestep = Time::new::<second>(0.01);
        // for this timestep, the simulation fails around 181s of simulated time
        //
        // let timestep = Time::new::<second>(0.5);
        // for this timestep, the simulation fails around 185s of simulated time
        //
        // the conclusion is that this instability is almost independent of timestep
        let timestep = Time::new::<second>(0.2);
        let heat_rate_through_heater = input_power;
        let mut tchx_heat_transfer_coeff: HeatTransfer;

        let reference_tchx_htc = 
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

        let mut_ctah_pid_controller = dhx_pid_controller.clone();

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



        let mut final_mass_flowrate_pri_loop: MassRate 
            = MassRate::ZERO;
        let mut final_mass_flowrate_dracs_loop: MassRate 
            = MassRate::ZERO;

        let ambient_htc = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);

        // create shared state pointer for parallelism
        //

        let ciet_state_ptr_main_loop = Arc::new(Mutex::new(
                CIETComponentsAndState {
                    pipe_4,
                    pipe_3,
                    static_mixer_10_label_2,
                    pipe_2a,
                    heater_top_head_1a,
                    heater_ver_1,
                    heater_bottom_head_1b,
                    pipe_18,
                    pipe_5a,
                    pipe_26,
                    pipe_25a,
                    static_mixer_21_label_25,
                    dhx_sthe,
                    static_mixer_20_label_23,
                    pipe_23a,
                    pipe_22,
                    flowmeter_20_21a,
                    pipe_21,
                    pipe_20,
                    pipe_19,
                    pipe_17b,
                    pipe_5b,
                    pipe_6a,
                    static_mixer_41_label_6,
                    ctah_vertical_label_7a,
                    ctah_horizontal_label_7b,
                    pipe_8a,
                    static_mixer_40_label_8,
                    pipe_9,
                    pipe_10,
                    pipe_11,
                    pipe_12,
                    ctah_pump,
                    pipe_13,
                    pipe_14,
                    flowmeter_40_14a,
                    pipe_15,
                    pipe_16,
                    pipe_17a,
                    dhx_tube_side_30a,
                    dhx_tube_side_30b,
                    pipe_31a,
                    static_mixer_61_label_31,
                    pipe_32,
                    pipe_33,
                    pipe_34,
                    tchx_35a,
                    tchx_35b_1,
                    tchx_35b_2,
                    pipe_36a,
                    static_mixer_60_label_36,
                    pipe_37,
                    flowmeter_60_37a,
                    pipe_38,
                    pipe_39,
                    top_mixing_node_5a_5b_4,
                    bottom_mixing_node_17a_17b_18,
                    counter_clockwise_dracs_flowrate: MassRate::ZERO,
                    dhx_br_flowrate: MassRate::ZERO,
                    heater_br_flowrate: MassRate::ZERO,
                    ctah_br_flowrate: MassRate::ZERO,
                }));



        // calculation loop
        while current_simulation_time < max_simulation_time {

            let ciet_state_ptr_for_dracs_mass_rate = ciet_state_ptr_main_loop.clone();
            let ciet_state_ptr_for_pri_mass_rate = ciet_state_ptr_main_loop.clone();
            let ciet_state_ptr_for_dracs_calcs = ciet_state_ptr_main_loop.clone();
            let ciet_state_ptr_for_pri_calcs = ciet_state_ptr_main_loop.clone();

            // this one just reads the temperature, so make a clone
            let tchx_outlet_temperature: ThermodynamicTemperature = {

                let mut tchx_35b_2 = 
                    ciet_state_ptr_main_loop.lock().unwrap().tchx_35b_2
                    .clone();

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
                // in this case 250 W/(m^2 K)

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

            // placeholder for ctah heat trf coeff 

            let ctah_heat_transfer_coeff: HeatTransfer = {
                // need to put pid here
                reference_tchx_htc
            };

            // fluid calculation loop 
            //
            // first, absolute mass flowrate across two branches

            let dracs_massrate_join_handle = 
                std::thread::spawn(move|| {

                    let mut ciet_state_clone: CIETComponentsAndState = 
                        ciet_state_ptr_for_dracs_mass_rate
                        .lock().unwrap().clone();

                    let dhx_tube_side_heat_exchanger_30 = 
                        ciet_state_clone.
                        dhx_sthe.
                        get_clone_of_tube_side_parallel_tube_fluid_component();

                    let pipe_34 = ciet_state_clone.pipe_34.clone();
                    let pipe_33 = ciet_state_clone.pipe_33.clone();
                    let pipe_32 = ciet_state_clone.pipe_32.clone();
                    let pipe_31a = ciet_state_clone.pipe_31a.clone();
                    let static_mixer_61_label_31 = ciet_state_clone.static_mixer_61_label_31.clone();
                    let dhx_tube_side_30b = ciet_state_clone.dhx_tube_side_30b.clone();
                    let dhx_tube_side_30a = ciet_state_clone.dhx_tube_side_30a.clone();
                    let tchx_35a = ciet_state_clone.tchx_35a.clone();
                    let tchx_35b_1 = ciet_state_clone.tchx_35b_1.clone();
                    let tchx_35b_2 = ciet_state_clone.tchx_35b_2.clone();
                    let static_mixer_60_label_36 = ciet_state_clone.static_mixer_60_label_36.clone();
                    let pipe_36a = ciet_state_clone.pipe_36a.clone();
                    let pipe_37 = ciet_state_clone.pipe_37.clone();
                    let flowmeter_60_37a = ciet_state_clone.flowmeter_60_37a.clone();
                    let pipe_38 = ciet_state_clone.pipe_38.clone();
                    let pipe_39 = ciet_state_clone.pipe_39.clone();

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


                    // the only thing here I want to set is the 
                    // mass flowrate, nothing else 
                    //



                    ciet_state_ptr_for_dracs_mass_rate.lock().unwrap()
                        .counter_clockwise_dracs_flowrate = 
                        counter_clockwise_dracs_flowrate;


                });

            let pri_loop_flowrate_join_handle = 
                std::thread::spawn(move || {

                    let ciet_state_clone: CIETComponentsAndState = 
                        ciet_state_ptr_for_pri_mass_rate
                        .lock().unwrap().clone();

                    let pipe_4 = ciet_state_clone.pipe_4.clone();
                    let pipe_3 = ciet_state_clone.pipe_3.clone();
                    let pipe_2a = ciet_state_clone.pipe_2a.clone();
                    let static_mixer_10_label_2 = ciet_state_clone.static_mixer_10_label_2.clone();
                    let heater_top_head_1a = ciet_state_clone.heater_top_head_1a.clone();
                    let heater_ver_1 = ciet_state_clone.heater_ver_1.clone();
                    let heater_bottom_head_1b = ciet_state_clone.heater_bottom_head_1b.clone();
                    let pipe_18 = ciet_state_clone.pipe_18.clone();
                    let pipe_5a = ciet_state_clone.pipe_5a.clone();
                    let pipe_26 = ciet_state_clone.pipe_26.clone();
                    let pipe_25a = ciet_state_clone.pipe_25a.clone();
                    let static_mixer_21_label_25 = ciet_state_clone.static_mixer_21_label_25.clone();
                    let dhx_shell_side_pipe_24 = 
                        ciet_state_clone.
                        dhx_sthe.get_clone_of_shell_side_fluid_component();
                    let static_mixer_20_label_23 = ciet_state_clone.static_mixer_20_label_23.clone();
                    let pipe_23a = ciet_state_clone.pipe_23a.clone();
                    let pipe_22 = ciet_state_clone.pipe_22.clone();
                    let flowmeter_20_21a = ciet_state_clone.flowmeter_20_21a.clone();
                    let pipe_21 = ciet_state_clone.pipe_21.clone();
                    let pipe_20 = ciet_state_clone.pipe_20.clone();
                    let pipe_19 = ciet_state_clone.pipe_19.clone();
                    let pipe_17b = ciet_state_clone.pipe_17b.clone();
                    let pipe_5b = ciet_state_clone.pipe_5b.clone();
                    let static_mixer_41_label_6 = ciet_state_clone.static_mixer_41_label_6.clone();
                    let pipe_6a = ciet_state_clone.pipe_6a.clone();
                    let ctah_vertical_label_7a = ciet_state_clone.ctah_vertical_label_7a.clone();
                    let ctah_horizontal_label_7b = ciet_state_clone.ctah_horizontal_label_7b.clone();
                    let pipe_8a = ciet_state_clone.pipe_8a.clone();
                    let static_mixer_40_label_8 = ciet_state_clone.static_mixer_40_label_8.clone();
                    let pipe_9 = ciet_state_clone.pipe_9.clone();
                    let pipe_10 = ciet_state_clone.pipe_10.clone();
                    let pipe_11 = ciet_state_clone.pipe_11.clone();
                    let pipe_12 = ciet_state_clone.pipe_12.clone();
                    let ctah_pump = ciet_state_clone.ctah_pump.clone();
                    // todo: ciet state should contain pump pressure for UI 
                    // real-time interaction
                    let pipe_13 = ciet_state_clone.pipe_13.clone();
                    let pipe_14 = ciet_state_clone.pipe_14.clone();
                    let flowmeter_40_14a = ciet_state_clone.flowmeter_40_14a.clone();
                    let pipe_15 = ciet_state_clone.pipe_15.clone();
                    let pipe_16 = ciet_state_clone.pipe_16.clone();
                    let pipe_17a = ciet_state_clone.pipe_17a.clone();

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

                    ciet_state_ptr_for_pri_mass_rate.lock().unwrap()
                        .dhx_br_flowrate = 
                        dhx_flow;
                    ciet_state_ptr_for_pri_mass_rate.lock().unwrap()
                        .heater_br_flowrate = 
                        heater_flow;
                    ciet_state_ptr_for_pri_mass_rate.lock().unwrap()
                        .ctah_br_flowrate = 
                        ctah_flow;

                });


            let dracs_heat_trf_join_handle = 
                std::thread::spawn(move||{
                    let ciet_state_clone: CIETComponentsAndState = 
                        ciet_state_ptr_for_dracs_calcs
                        .lock().unwrap().clone();

                    let counter_clockwise_dracs_flowrate = 
                        ciet_state_clone.counter_clockwise_dracs_flowrate;
                    let mut pipe_34 = ciet_state_clone.pipe_34.clone();
                    let mut pipe_33 = ciet_state_clone.pipe_33.clone();
                    let mut pipe_32 = ciet_state_clone.pipe_32.clone();
                    let mut pipe_31a = ciet_state_clone.pipe_31a.clone();
                    let mut static_mixer_61_label_31 = ciet_state_clone.static_mixer_61_label_31.clone();
                    let mut dhx_tube_side_30b = ciet_state_clone.dhx_tube_side_30b.clone();
                    let mut dhx_tube_side_30a = ciet_state_clone.dhx_tube_side_30a.clone();
                    let mut tchx_35a = ciet_state_clone.tchx_35a.clone();
                    let mut tchx_35b_1 = ciet_state_clone.tchx_35b_1.clone();
                    let mut tchx_35b_2 = ciet_state_clone.tchx_35b_2.clone();
                    let mut static_mixer_60_label_36 = ciet_state_clone.static_mixer_60_label_36.clone();
                    let mut pipe_36a = ciet_state_clone.pipe_36a.clone();
                    let mut pipe_37 = ciet_state_clone.pipe_37.clone();
                    let mut flowmeter_60_37a = ciet_state_clone.flowmeter_60_37a.clone();
                    let mut pipe_38 = ciet_state_clone.pipe_38.clone();
                    let mut pipe_39 = ciet_state_clone.pipe_39.clone();

                    let mut dhx_sthe = ciet_state_clone.dhx_sthe.clone();
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

                    // now lock the pointer to update the state
                {
                    ciet_state_ptr_for_dracs_calcs
                        .lock().unwrap().deref_mut().pipe_34 = pipe_34;
                    ciet_state_ptr_for_dracs_calcs
                        .lock().unwrap().deref_mut().pipe_33 = pipe_33;
                    ciet_state_ptr_for_dracs_calcs
                        .lock().unwrap().deref_mut().pipe_32 = pipe_32;
                    ciet_state_ptr_for_dracs_calcs
                        .lock().unwrap().deref_mut().pipe_31a = pipe_31a;
                    ciet_state_ptr_for_dracs_calcs
                        .lock().unwrap().deref_mut().static_mixer_61_label_31 = static_mixer_61_label_31;
                    ciet_state_ptr_for_dracs_calcs
                        .lock().unwrap().deref_mut().dhx_tube_side_30b = dhx_tube_side_30b;
                    // note: both calc loops require the dhx sthe. I cannot 
                    // replace the whole dhx sthe
                    // have to change the tube side only
                    ciet_state_ptr_for_dracs_calcs
                        .lock().unwrap().deref_mut().dhx_sthe.tube_side_fluid_array_for_single_tube 
                        = dhx_sthe.tube_side_fluid_array_for_single_tube;
                    ciet_state_ptr_for_dracs_calcs
                        .lock().unwrap().deref_mut().dhx_tube_side_30a = dhx_tube_side_30a;
                    ciet_state_ptr_for_dracs_calcs
                        .lock().unwrap().deref_mut().tchx_35a = tchx_35a;
                    ciet_state_ptr_for_dracs_calcs
                        .lock().unwrap().deref_mut().tchx_35b_1 = tchx_35b_1;
                    ciet_state_ptr_for_dracs_calcs
                        .lock().unwrap().deref_mut().tchx_35b_2 = tchx_35b_2;
                    ciet_state_ptr_for_dracs_calcs
                        .lock().unwrap().deref_mut().static_mixer_60_label_36 = static_mixer_60_label_36;
                    ciet_state_ptr_for_dracs_calcs
                        .lock().unwrap().deref_mut().pipe_36a = pipe_36a;
                    ciet_state_ptr_for_dracs_calcs
                        .lock().unwrap().deref_mut().pipe_37 = pipe_37;
                    ciet_state_ptr_for_dracs_calcs
                        .lock().unwrap().deref_mut().flowmeter_60_37a = flowmeter_60_37a;
                    ciet_state_ptr_for_dracs_calcs
                        .lock().unwrap().deref_mut().pipe_38 = pipe_38;
                    ciet_state_ptr_for_dracs_calcs
                        .lock().unwrap().deref_mut().pipe_39 = pipe_39;


                }


                });


            //dbg!(&(dhx_flow,heater_flow,ctah_flow));

            let pri_heat_trf_join_handle = 
                std::thread::spawn(move||{
                    let mut ciet_state_clone: CIETComponentsAndState = 
                        ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().clone();


                    let dhx_flow = 
                        ciet_state_clone.dhx_br_flowrate;
                    let heater_flow = 
                        ciet_state_clone.heater_br_flowrate;
                    let ctah_flow = 
                        ciet_state_clone.ctah_br_flowrate;


                    let mut pipe_4 = ciet_state_clone.pipe_4.clone();
                    let mut pipe_3 = ciet_state_clone.pipe_3.clone();
                    let mut pipe_2a = ciet_state_clone.pipe_2a.clone();
                    let mut static_mixer_10_label_2 = ciet_state_clone.static_mixer_10_label_2.clone();
                    let mut heater_top_head_1a = ciet_state_clone.heater_top_head_1a.clone();
                    let mut heater_ver_1 = ciet_state_clone.heater_ver_1.clone();
                    let mut heater_bottom_head_1b = ciet_state_clone.heater_bottom_head_1b.clone();
                    let mut pipe_18 = ciet_state_clone.pipe_18.clone();
                    let mut pipe_5a = ciet_state_clone.pipe_5a.clone();
                    let mut pipe_26 = ciet_state_clone.pipe_26.clone();
                    let mut pipe_25a = ciet_state_clone.pipe_25a.clone();
                    let mut static_mixer_21_label_25 = ciet_state_clone.static_mixer_21_label_25.clone();
                    let mut static_mixer_20_label_23 = ciet_state_clone.static_mixer_20_label_23.clone();
                    let mut pipe_23a = ciet_state_clone.pipe_23a.clone();
                    let mut pipe_22 = ciet_state_clone.pipe_22.clone();
                    let mut flowmeter_20_21a = ciet_state_clone.flowmeter_20_21a.clone();
                    let mut pipe_21 = ciet_state_clone.pipe_21.clone();
                    let mut pipe_20 = ciet_state_clone.pipe_20.clone();
                    let mut pipe_19 = ciet_state_clone.pipe_19.clone();
                    let mut pipe_17b = ciet_state_clone.pipe_17b.clone();
                    let mut pipe_5b = ciet_state_clone.pipe_5b.clone();
                    let mut static_mixer_41_label_6 = ciet_state_clone.static_mixer_41_label_6.clone();
                    let mut pipe_6a = ciet_state_clone.pipe_6a.clone();
                    let mut ctah_vertical_label_7a = ciet_state_clone.ctah_vertical_label_7a.clone();
                    let mut ctah_horizontal_label_7b = ciet_state_clone.ctah_horizontal_label_7b.clone();
                    let mut pipe_8a = ciet_state_clone.pipe_8a.clone();
                    let mut static_mixer_40_label_8 = ciet_state_clone.static_mixer_40_label_8.clone();
                    let mut pipe_9 = ciet_state_clone.pipe_9.clone();
                    let mut pipe_10 = ciet_state_clone.pipe_10.clone();
                    let mut pipe_11 = ciet_state_clone.pipe_11.clone();
                    let mut pipe_12 = ciet_state_clone.pipe_12.clone();
                    let mut ctah_pump = ciet_state_clone.ctah_pump.clone();
                    // todo: ciet state should contain pump pressure for UI 
                    // real-time interaction
                    let mut pipe_13 = ciet_state_clone.pipe_13.clone();
                    let mut pipe_14 = ciet_state_clone.pipe_14.clone();
                    let mut flowmeter_40_14a = ciet_state_clone.flowmeter_40_14a.clone();
                    let mut pipe_15 = ciet_state_clone.pipe_15.clone();
                    let mut pipe_16 = ciet_state_clone.pipe_16.clone();
                    let mut pipe_17a = ciet_state_clone.pipe_17a.clone();

                    let mut dhx_sthe = ciet_state_clone.dhx_sthe.clone();
                    let mut top_mixing_node_5a_5b_4 = ciet_state_clone.top_mixing_node_5a_5b_4.clone();
                    let mut bottom_mixing_node_17a_17b_18 = ciet_state_clone.bottom_mixing_node_17a_17b_18.clone();

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

                    // need to update state of CIET
                {
                    // heater br
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_4 = pipe_4;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_3 = pipe_3;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_2a = pipe_2a;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().static_mixer_10_label_2 = static_mixer_10_label_2;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().heater_top_head_1a = heater_top_head_1a;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().heater_ver_1 = heater_ver_1;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().heater_bottom_head_1b = heater_bottom_head_1b;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_18 = pipe_18;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_5a = pipe_5a;
                    // dhx br
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_26 = pipe_26;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_25a = pipe_25a;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().static_mixer_21_label_25 = static_mixer_21_label_25;
                    // note: both calc loops require the dhx sthe. I cannot 
                    // replace the whole dhx sthe
                    // have to change the shell side only
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().dhx_sthe.shell_side_fluid_array 
                        = dhx_sthe.shell_side_fluid_array;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().static_mixer_20_label_23 = static_mixer_20_label_23;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_23a = pipe_23a;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_22 = pipe_22;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().flowmeter_20_21a = flowmeter_20_21a;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_21 = pipe_21;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_20 = pipe_20;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_19 = pipe_19;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_19 = pipe_17b;
                    // ctah br
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_5b = pipe_5b;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().static_mixer_41_label_6 = static_mixer_41_label_6;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_6a = pipe_6a;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().ctah_vertical_label_7a = ctah_vertical_label_7a;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().ctah_horizontal_label_7b = ctah_horizontal_label_7b;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_8a = pipe_8a;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().static_mixer_40_label_8 = static_mixer_40_label_8;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_9 = pipe_9;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_10 = pipe_10;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_11 = pipe_11;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_12 = pipe_12;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().ctah_pump = ctah_pump;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_13 = pipe_13;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_14 = pipe_14;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().flowmeter_40_14a = flowmeter_40_14a;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_15 = pipe_15;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_16 = pipe_16;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().pipe_17a = pipe_17a;

                    // mixing nodes 
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().top_mixing_node_5a_5b_4 = top_mixing_node_5a_5b_4;
                    ciet_state_ptr_for_pri_calcs
                        .lock().unwrap().deref_mut().bottom_mixing_node_17a_17b_18 = bottom_mixing_node_17a_17b_18;



                }

                });



            // join the handles
            dracs_massrate_join_handle.join().unwrap();
            pri_loop_flowrate_join_handle.join().unwrap();
            dracs_heat_trf_join_handle.join().unwrap();
            pri_heat_trf_join_handle.join().unwrap();

            // update the states from dracs loop into the pri loop 
            // except dhx sthe
            let ciet_state_clone :CIETComponentsAndState 
                = ciet_state_ptr_main_loop.lock().unwrap().clone();

            let mut pipe_34 = ciet_state_clone.pipe_34.clone();
            let mut pipe_33 = ciet_state_clone.pipe_33.clone();
            let mut pipe_32 = ciet_state_clone.pipe_32.clone();
            let mut pipe_31a = ciet_state_clone.pipe_31a.clone();
            let mut static_mixer_61_label_31 = ciet_state_clone.static_mixer_61_label_31.clone();
            let mut dhx_tube_side_30b = ciet_state_clone.dhx_tube_side_30b.clone();
            let mut dhx_tube_side_30a = ciet_state_clone.dhx_tube_side_30a.clone();
            let mut tchx_35a = ciet_state_clone.tchx_35a.clone();
            let mut tchx_35b_1 = ciet_state_clone.tchx_35b_1.clone();
            let mut tchx_35b_2 = ciet_state_clone.tchx_35b_2.clone();
            let mut static_mixer_60_label_36 = ciet_state_clone.static_mixer_60_label_36.clone();
            let mut pipe_36a = ciet_state_clone.pipe_36a.clone();
            let mut pipe_37 = ciet_state_clone.pipe_37.clone();
            let mut flowmeter_60_37a = ciet_state_clone.flowmeter_60_37a.clone();
            let mut pipe_38 = ciet_state_clone.pipe_38.clone();
            let mut pipe_39 = ciet_state_clone.pipe_39.clone();

            let mut dhx_sthe = ciet_state_clone.dhx_sthe.clone();
            let mut pipe_4 = ciet_state_clone.pipe_4.clone();
            let mut pipe_3 = ciet_state_clone.pipe_3.clone();
            let mut pipe_2a = ciet_state_clone.pipe_2a.clone();
            let mut static_mixer_10_label_2 = ciet_state_clone.static_mixer_10_label_2.clone();
            let mut heater_top_head_1a = ciet_state_clone.heater_top_head_1a.clone();
            let mut heater_ver_1 = ciet_state_clone.heater_ver_1.clone();
            let mut heater_bottom_head_1b = ciet_state_clone.heater_bottom_head_1b.clone();
            let mut pipe_18 = ciet_state_clone.pipe_18.clone();
            let mut pipe_5a = ciet_state_clone.pipe_5a.clone();
            let mut pipe_26 = ciet_state_clone.pipe_26.clone();
            let mut pipe_25a = ciet_state_clone.pipe_25a.clone();
            let mut static_mixer_21_label_25 = ciet_state_clone.static_mixer_21_label_25.clone();
            let mut static_mixer_20_label_23 = ciet_state_clone.static_mixer_20_label_23.clone();
            let mut pipe_23a = ciet_state_clone.pipe_23a.clone();
            let mut pipe_22 = ciet_state_clone.pipe_22.clone();
            let mut flowmeter_20_21a = ciet_state_clone.flowmeter_20_21a.clone();
            let mut pipe_21 = ciet_state_clone.pipe_21.clone();
            let mut pipe_20 = ciet_state_clone.pipe_20.clone();
            let mut pipe_19 = ciet_state_clone.pipe_19.clone();
            let mut pipe_17b = ciet_state_clone.pipe_17b.clone();
            let mut pipe_5b = ciet_state_clone.pipe_5b.clone();
            let mut static_mixer_41_label_6 = ciet_state_clone.static_mixer_41_label_6.clone();
            let mut pipe_6a = ciet_state_clone.pipe_6a.clone();
            let mut ctah_vertical_label_7a = ciet_state_clone.ctah_vertical_label_7a.clone();
            let mut ctah_horizontal_label_7b = ciet_state_clone.ctah_horizontal_label_7b.clone();
            let mut pipe_8a = ciet_state_clone.pipe_8a.clone();
            let mut static_mixer_40_label_8 = ciet_state_clone.static_mixer_40_label_8.clone();
            let mut pipe_9 = ciet_state_clone.pipe_9.clone();
            let mut pipe_10 = ciet_state_clone.pipe_10.clone();
            let mut pipe_11 = ciet_state_clone.pipe_11.clone();
            let mut pipe_12 = ciet_state_clone.pipe_12.clone();
            let mut ctah_pump = ciet_state_clone.ctah_pump.clone();
            // todo: ciet state should contain pump pressure for UI 
            // real-time interaction
            let mut pipe_13 = ciet_state_clone.pipe_13.clone();
            let mut pipe_14 = ciet_state_clone.pipe_14.clone();
            let mut flowmeter_40_14a = ciet_state_clone.flowmeter_40_14a.clone();
            let mut pipe_15 = ciet_state_clone.pipe_15.clone();
            let mut pipe_16 = ciet_state_clone.pipe_16.clone();
            let mut pipe_17a = ciet_state_clone.pipe_17a.clone();

            let mut top_mixing_node_5a_5b_4 = ciet_state_clone.top_mixing_node_5a_5b_4.clone();
            let mut bottom_mixing_node_17a_17b_18 = ciet_state_clone.bottom_mixing_node_17a_17b_18.clone();


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

            let counter_clockwise_dracs_flowrate = 
                ciet_state_ptr_main_loop.lock().unwrap()
                .counter_clockwise_dracs_flowrate;

            let counter_clockwise_dhx_flowrate = 
                ciet_state_ptr_main_loop.lock().unwrap()
                .dhx_br_flowrate;

            let prandtl_wall_correction_setting = true; 
            let tube_side_total_mass_flowrate = -counter_clockwise_dracs_flowrate;
            let shell_side_total_mass_flowrate = counter_clockwise_dhx_flowrate;

            dhx_sthe.lateral_and_miscellaneous_connections(
                prandtl_wall_correction_setting, 
                tube_side_total_mass_flowrate, 
                shell_side_total_mass_flowrate).unwrap();

            dhx_sthe.advance_timestep(timestep).unwrap();

            // after all the timestep advancing,
            // update the state
            {
                // heater br
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_4 = pipe_4;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_3 = pipe_3;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_2a = pipe_2a;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().static_mixer_10_label_2 = static_mixer_10_label_2.clone();
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().heater_top_head_1a = heater_top_head_1a;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().heater_ver_1 = heater_ver_1;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().heater_bottom_head_1b = heater_bottom_head_1b.clone();
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_18 = pipe_18;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_5a = pipe_5a;
                // dhx br
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_26 = pipe_26;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_25a = pipe_25a.clone();
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().static_mixer_21_label_25 = static_mixer_21_label_25;
                // note: both calc loops require the dhx sthe. I cannot 
                // replace the whole dhx sthe
                // have to change the shell side only
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().dhx_sthe = dhx_sthe;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().static_mixer_20_label_23 = static_mixer_20_label_23.clone();
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_23a = pipe_23a;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_22 = pipe_22;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().flowmeter_20_21a = flowmeter_20_21a;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_21 = pipe_21;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_20 = pipe_20;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_19 = pipe_19;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_19 = pipe_17b;
                // ctah br
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_5b = pipe_5b;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().static_mixer_41_label_6 = static_mixer_41_label_6;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_6a = pipe_6a;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().ctah_vertical_label_7a = ctah_vertical_label_7a;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().ctah_horizontal_label_7b = ctah_horizontal_label_7b;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_8a = pipe_8a;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().static_mixer_40_label_8 = static_mixer_40_label_8;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_9 = pipe_9;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_10 = pipe_10;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_11 = pipe_11;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_12 = pipe_12;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().ctah_pump = ctah_pump;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_13 = pipe_13;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_14 = pipe_14;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().flowmeter_40_14a = flowmeter_40_14a;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_15 = pipe_15;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_16 = pipe_16;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_17a = pipe_17a;

                // mixing nodes 
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().top_mixing_node_5a_5b_4 = top_mixing_node_5a_5b_4;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().bottom_mixing_node_17a_17b_18 = bottom_mixing_node_17a_17b_18;

                // dracs loop

                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_34 = pipe_34;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_33 = pipe_33;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_32 = pipe_32;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_31a = pipe_31a;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().static_mixer_61_label_31 = static_mixer_61_label_31;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().dhx_tube_side_30b = dhx_tube_side_30b.clone();
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().dhx_tube_side_30a = dhx_tube_side_30a.clone();
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().tchx_35a = tchx_35a;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().tchx_35b_1 = tchx_35b_1;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().tchx_35b_2 = tchx_35b_2;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().static_mixer_60_label_36 = static_mixer_60_label_36;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_36a = pipe_36a;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_37 = pipe_37;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().flowmeter_60_37a = flowmeter_60_37a;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_38 = pipe_38;
                ciet_state_ptr_main_loop
                    .lock().unwrap().deref_mut().pipe_39 = pipe_39;

                }



            // record 
            if current_simulation_time > tchx_temperature_record_time_threshold {
                final_mass_flowrate_dracs_loop = counter_clockwise_dracs_flowrate;
                final_mass_flowrate_pri_loop = counter_clockwise_dhx_flowrate;
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


        // for displaying temperatures, it is good to take the last state 
        // out of the arc mutex ptr

        let ciet_state_clone :CIETComponentsAndState 
            = ciet_state_ptr_main_loop.lock().unwrap().clone();

        let mut _pipe_34 = ciet_state_clone.pipe_34.clone();
        let mut _pipe_33 = ciet_state_clone.pipe_33.clone();
        let mut _pipe_32 = ciet_state_clone.pipe_32.clone();
        let mut _pipe_31a = ciet_state_clone.pipe_31a.clone();
        let mut _static_mixer_61_label_31 = ciet_state_clone.static_mixer_61_label_31.clone();
        let mut dhx_tube_side_30b = ciet_state_clone.dhx_tube_side_30b.clone();
        let mut dhx_tube_side_30a = ciet_state_clone.dhx_tube_side_30a.clone();
        let mut _tchx_35a = ciet_state_clone.tchx_35a.clone();
        let mut _tchx_35b_1 = ciet_state_clone.tchx_35b_1.clone();
        let mut _tchx_35b_2 = ciet_state_clone.tchx_35b_2.clone();
        let mut _static_mixer_60_label_36 = ciet_state_clone.static_mixer_60_label_36.clone();
        let mut _pipe_36a = ciet_state_clone.pipe_36a.clone();
        let mut _pipe_37 = ciet_state_clone.pipe_37.clone();
        let mut _flowmeter_60_37a = ciet_state_clone.flowmeter_60_37a.clone();
        let mut _pipe_38 = ciet_state_clone.pipe_38.clone();
        let mut _pipe_39 = ciet_state_clone.pipe_39.clone();

        let mut _dhx_sthe = ciet_state_clone.dhx_sthe.clone();
        let mut _pipe_4 = ciet_state_clone.pipe_4.clone();
        let mut _pipe_3 = ciet_state_clone.pipe_3.clone();
        let mut _pipe_2a = ciet_state_clone.pipe_2a.clone();
        let mut static_mixer_10_label_2 = ciet_state_clone.static_mixer_10_label_2.clone();
        let mut _heater_top_head_1a = ciet_state_clone.heater_top_head_1a.clone();
        let mut heater_ver_1 = ciet_state_clone.heater_ver_1.clone();
        let mut heater_bottom_head_1b = ciet_state_clone.heater_bottom_head_1b.clone();
        let mut _pipe_18 = ciet_state_clone.pipe_18.clone();
        let mut _pipe_5a = ciet_state_clone.pipe_5a.clone();
        let mut _pipe_26 = ciet_state_clone.pipe_26.clone();
        let mut pipe_25a = ciet_state_clone.pipe_25a.clone();
        let mut _static_mixer_21_label_25 = ciet_state_clone.static_mixer_21_label_25.clone();
        let mut static_mixer_20_label_23 = ciet_state_clone.static_mixer_20_label_23.clone();
        let mut _pipe_23a = ciet_state_clone.pipe_23a.clone();
        let mut _pipe_22 = ciet_state_clone.pipe_22.clone();
        let mut _flowmeter_20_21a = ciet_state_clone.flowmeter_20_21a.clone();
        let mut _pipe_21 = ciet_state_clone.pipe_21.clone();
        let mut _pipe_20 = ciet_state_clone.pipe_20.clone();
        let mut _pipe_19 = ciet_state_clone.pipe_19.clone();
        let mut _pipe_17b = ciet_state_clone.pipe_17b.clone();
        let mut _pipe_5b = ciet_state_clone.pipe_5b.clone();
        let mut _static_mixer_41_label_6 = ciet_state_clone.static_mixer_41_label_6.clone();
        let mut _pipe_6a = ciet_state_clone.pipe_6a.clone();
        let mut _ctah_vertical_label_7a = ciet_state_clone.ctah_vertical_label_7a.clone();
        let mut _ctah_horizontal_label_7b = ciet_state_clone.ctah_horizontal_label_7b.clone();
        let mut _pipe_8a = ciet_state_clone.pipe_8a.clone();
        let mut _static_mixer_40_label_8 = ciet_state_clone.static_mixer_40_label_8.clone();
        let mut _pipe_9 = ciet_state_clone.pipe_9.clone();
        let mut _pipe_10 = ciet_state_clone.pipe_10.clone();
        let mut _pipe_11 = ciet_state_clone.pipe_11.clone();
        let mut _pipe_12 = ciet_state_clone.pipe_12.clone();
        let mut _ctah_pump = ciet_state_clone.ctah_pump.clone();
        // todo: ciet state should contain pump pressure for UI 
        // real-time interaction
        let mut _pipe_13 = ciet_state_clone.pipe_13.clone();
        let mut _pipe_14 = ciet_state_clone.pipe_14.clone();
        let mut _flowmeter_40_14a = ciet_state_clone.flowmeter_40_14a.clone();
        let mut _pipe_15 = ciet_state_clone.pipe_15.clone();
        let mut _pipe_16 = ciet_state_clone.pipe_16.clone();
        let mut _pipe_17a = ciet_state_clone.pipe_17a.clone();

        let mut _top_mixing_node_5a_5b_4 = ciet_state_clone.top_mixing_node_5a_5b_4.clone();
        let mut _bottom_mixing_node_17a_17b_18 = ciet_state_clone.bottom_mixing_node_17a_17b_18.clone();

        // temperatures before and after heater
        let ((_bt_11,_wt_10),(_bt_12,_wt_13)) = 
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

        dbg!(&(
                input_power,
                final_mass_flowrate_pri_loop,
                final_mass_flowrate_dracs_loop,
                simulated_heater_avg_surf_temp_degc
        ));



        // this asserts the final mass flowrate against experimental flowrate
        approx::assert_relative_eq!(
            experimental_primary_mass_flowrate.get::<kilogram_per_second>(),
            final_mass_flowrate_pri_loop.get::<kilogram_per_second>(),
            max_relative=pri_loop_relative_tolerance);

        approx::assert_relative_eq!(
            experimental_dracs_mass_flowrate.get::<kilogram_per_second>(),
            final_mass_flowrate_dracs_loop.get::<kilogram_per_second>(),
            max_relative=dracs_loop_relative_tolerance);

        // check heater surface temp to within tolerance 


        // this asserts the final mass flowrate against experimental flowrate 
        // for regression to within 0.1%
        approx::assert_relative_eq!(
            simulated_expected_primary_mass_flowrate_kg_per_s,
            final_mass_flowrate_pri_loop.get::<kilogram_per_second>(),
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


        Ok(())

    }

/// this contains all the necessary piping and instrumentation in CIET 
/// so that we need only make one Arc Mutex Pointer for the whole of CIET

#[derive(Clone,Debug,PartialEq)]
pub struct CIETComponentsAndState {
    // for heater branch 


    /// pipe in top of heater br, connects to three way joint
    pub pipe_4: InsulatedFluidComponent,
    /// pipe in heater br
    pub pipe_3: InsulatedFluidComponent,
    /// static mixer 
    pub static_mixer_10_label_2: InsulatedFluidComponent,
    /// static mixer pipe in heater br
    pub pipe_2a: InsulatedFluidComponent,
    /// version 1 of heater top head
    pub heater_top_head_1a: InsulatedFluidComponent,
    /// version 1 of heater
    pub heater_ver_1: InsulatedFluidComponent,
    /// version 1 of heater bottom head
    pub heater_bottom_head_1b: InsulatedFluidComponent,
    /// pipe in bottom of heater br, connects to three way joint
    pub pipe_18: InsulatedFluidComponent,

    // for dhx branch (nat circ loop)
    
    /// pipe in top of dhx br, connects to three way joint
    pub pipe_5a: InsulatedFluidComponent,
    /// pipe in dhx br
    pub pipe_26: InsulatedFluidComponent,
    /// static mixer pipe dhx branch
    pub pipe_25a: InsulatedFluidComponent,
    /// static mixer dhx branch
    pub static_mixer_21_label_25: InsulatedFluidComponent,
    /// dracs heat exchanger connecting dhx branch in pri loop 
    /// to dracs branch 
    ///
    /// the shell side is on the pri loop side 
    /// the tube side is on the dracs side
    pub dhx_sthe: SimpleShellAndTubeHeatExchanger,
    /// static mixer dhx branch
    pub static_mixer_20_label_23: InsulatedFluidComponent,
    /// static mixer pipe dhx branch
    pub pipe_23a: InsulatedFluidComponent,
    /// pipe dhx branch
    pub pipe_22: InsulatedFluidComponent,
    /// flowmeter in dhx branch
    pub flowmeter_20_21a: NonInsulatedFluidComponent,
    /// pipe dhx branch
    pub pipe_21: InsulatedFluidComponent,
    /// pipe dhx branch
    pub pipe_20: InsulatedFluidComponent,
    /// pipe dhx branch
    pub pipe_19: InsulatedFluidComponent,
    /// pipe in bottom of dhx br, connects to three way joint
    pub pipe_17b: InsulatedFluidComponent,


    // for ctah branch (forced circ loop)

    /// pipe in top of ctah br, connects to three way joint
    pub pipe_5b: InsulatedFluidComponent,
    /// static mixer pipe ctah branch
    pub pipe_6a: InsulatedFluidComponent,
    /// static mixer ctah brm adjacent to ctah
    pub static_mixer_41_label_6: InsulatedFluidComponent,
    /// ctah vertical simplified representation
    pub ctah_vertical_label_7a: NonInsulatedFluidComponent,
    /// ctah horizontal simplified representation
    pub ctah_horizontal_label_7b: NonInsulatedFluidComponent,
    /// static mixer pipe ctah branch
    pub pipe_8a: InsulatedFluidComponent,
    /// static mixer ctah brm adjacent to ctah
    pub static_mixer_40_label_8: InsulatedFluidComponent,
    /// pipe ctah branch
    pub pipe_9: InsulatedFluidComponent,
    /// pipe ctah branch
    pub pipe_10: InsulatedFluidComponent,
    /// pipe ctah branch
    pub pipe_11: InsulatedFluidComponent,
    /// pipe ctah branch
    pub pipe_12: InsulatedFluidComponent,
    /// ctah pump, the only pump for forced circulation in CIET
    pub ctah_pump: NonInsulatedFluidComponent,
    /// pipe ctah branch
    pub pipe_13: InsulatedFluidComponent,
    /// pipe ctah branch
    pub pipe_14: InsulatedFluidComponent,
    /// flowmeter ctah branch
    pub flowmeter_40_14a: NonInsulatedFluidComponent,
    /// pipe ctah branch
    pub pipe_15: InsulatedFluidComponent,
    /// pipe ctah branch
    pub pipe_16: InsulatedFluidComponent,
    /// pipe in bottom of ctah br, connects to three way joint
    pub pipe_17a: InsulatedFluidComponent,

    // dracs loop

    /// dracs loop pipe at bottom of dhx tube side
    pub dhx_tube_side_30a: NonInsulatedFluidComponent,
    /// dracs loop pipe at top of dhx tube side
    pub dhx_tube_side_30b: NonInsulatedFluidComponent,
    /// static mixer pipe dracs loop
    pub pipe_31a: InsulatedFluidComponent,
    /// static mixer in dracs loop adjacent to dhx
    pub static_mixer_61_label_31: InsulatedFluidComponent,
    /// pipe in dracs loop
    pub pipe_32: InsulatedFluidComponent,
    /// pipe in dracs loop
    pub pipe_33: InsulatedFluidComponent,
    /// pipe in dracs loop adjacent to tchx
    pub pipe_34: InsulatedFluidComponent,
    /// horizontal representation of TCHX (NDHX) 
    pub tchx_35a: NonInsulatedFluidComponent,
    /// first half of vertical representation of TCHX (NDHX) 
    pub tchx_35b_1: NonInsulatedFluidComponent,
    /// second half of vertical representation of TCHX (NDHX) 
    pub tchx_35b_2: NonInsulatedFluidComponent,
    /// static mixer pipe dracs loop
    pub pipe_36a: InsulatedFluidComponent,
    /// static mixer in dracs loop adjacent to tchx
    pub static_mixer_60_label_36: InsulatedFluidComponent,
    /// pipe in dracs loop
    pub pipe_37: InsulatedFluidComponent,
    /// flowmeter in dracs loop
    pub flowmeter_60_37a: NonInsulatedFluidComponent,
    /// pipe in dracs loop
    pub pipe_38: InsulatedFluidComponent,
    /// pipe in dracs loop adjacent to dhx bottom tube side (30a)
    pub pipe_39: InsulatedFluidComponent,


    // mixing nodes 
    /// the top of the 3 way joints connecting three branches in pri loop
    pub top_mixing_node_5a_5b_4: HeatTransferEntity,
    /// the bottom of the 3 way joints connecting three branches in pri loop
    pub bottom_mixing_node_17a_17b_18: HeatTransferEntity,


    // flowrates for all branches and loops 
    /// dracs loop flowrate 
    pub counter_clockwise_dracs_flowrate: MassRate,
    /// dhx branch flowrate 
    pub dhx_br_flowrate: MassRate,
    /// heater branch flowrate 
    pub heater_br_flowrate: MassRate,
    /// ctah branch flowrate 
    pub ctah_br_flowrate: MassRate,

}

impl CIETComponentsAndState {
    /// allows you to set the value of ciet state
    pub fn overwrite(&mut self, ciet_state: Self){
        *self = ciet_state;
    }
}

