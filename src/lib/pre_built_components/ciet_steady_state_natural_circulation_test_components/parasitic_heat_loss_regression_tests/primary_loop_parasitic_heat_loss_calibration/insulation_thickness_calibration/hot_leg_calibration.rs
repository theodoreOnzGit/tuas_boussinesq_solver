///
/// dataset number,pri loop mass flowrate (kg/s),Heater outlet (DegC),DHX shell top (DegC),DHX shell bottom (DegC),Heater inlet (DegC),
/// C-1,0.02003,75.22747,71.47752,53.60943,50.45784,
/// C-2,0.02367,82.41863,78.36713,57.13467,53.79036,
/// C-3,0.02635,87.78188,84.37342,59.82845,56.71891,
/// C-4,0.02949,94.71628,90.97595,63.9812,60.83029,
/// C-5,0.0319,100.37023,96.20228,67.05336,64.07406,
/// C-6,0.03412,105.25073,101.3375,69.85085,67.1654,
/// C-7,0.03562,110.34289,106.43149,73.21226,70.6215,
/// C-8,0.03593,115.52364,111.37615,76.13202,73.63344,
/// C-9,0.03547,119.96879,116.05003,79.02407,76.54479,
#[test]
pub fn hot_leg_insulation_thickness_calibration_v1(){
    // checks hot leg to ambient calibration
    use std::thread;

    // in this test, I attempted to 
    // calibrate the heat loss using an increased ambient_heat_transfer_coeff 
    // however, I was only able to get heat losses up such that the 
    // dhx inlet was down to 1 degree celsius
    //
    // this is not enough

    // dataset number,pri loop mass flowrate (kg/s),Heater outlet (DegC),DHX shell top (DegC),DHX shell bottom (DegC),Heater inlet (DegC),
    // C-1,0.02003,75.22747,71.47752,53.60943,50.45784,
    let set_c1 = thread::Builder::new()
        .name("set_c1".to_string()).spawn(||{

            let (experimental_primary_mass_flowrate_kg_per_s,
                heater_outlet_temperature_degc,
                dhx_inlet_temperature_set_point_degc) =
                (0.02003,75.22747,71.47752);
            let max_time_seconds = 500.0;
            let insulation_thickness_cm_for_regression_testing = 
                0.27;

            // regression is to within +/- 0.1 K
            let dhx_inlet_regression_temperature_degc = 
                dhx_inlet_temperature_set_point_degc;

            pri_loop_hot_leg_insulation_thickness_calibration_regression_test_v1(
                experimental_primary_mass_flowrate_kg_per_s, 
                heater_outlet_temperature_degc, 
                dhx_inlet_temperature_set_point_degc, 
                dhx_inlet_regression_temperature_degc,
                max_time_seconds, 
                insulation_thickness_cm_for_regression_testing);



        }).unwrap();
    // dataset number,pri loop mass flowrate (kg/s),Heater outlet (DegC),DHX shell top (DegC),DHX shell bottom (DegC),Heater inlet (DegC),
    // C-2,0.02367,82.41863,78.36713,57.13467,53.79036,
    let set_c2 = thread::Builder::new()
        .name("set_c2".to_string()).spawn(||{

            let (experimental_primary_mass_flowrate_kg_per_s,
                heater_outlet_temperature_degc,
                dhx_inlet_temperature_set_point_degc) =
                (0.02367,82.41863,78.22);
            let max_time_seconds = 500.0;
            // temperatures are regressed to within 0.1 K
            let insulation_thickness_cm_for_regression_testing = 0.21;

            // regression is to within +/- 0.1 K
            let dhx_inlet_regression_temperature_degc = 
                dhx_inlet_temperature_set_point_degc;

            pri_loop_hot_leg_insulation_thickness_calibration_regression_test_v1(
                experimental_primary_mass_flowrate_kg_per_s, 
                heater_outlet_temperature_degc, 
                dhx_inlet_temperature_set_point_degc, 
                dhx_inlet_regression_temperature_degc,
                max_time_seconds, 
                insulation_thickness_cm_for_regression_testing);



        }).unwrap();
    // dataset number,pri loop mass flowrate (kg/s),Heater outlet (DegC),DHX shell top (DegC),DHX shell bottom (DegC),Heater inlet (DegC),
    // C-3,0.02635,87.78188,84.37342,59.82845,56.71891,
    let set_c3 = thread::Builder::new()
        .name("set_c3".to_string()).spawn(||{

            let (experimental_primary_mass_flowrate_kg_per_s,
                heater_outlet_temperature_degc,
                dhx_inlet_temperature_set_point_degc) =
                (0.02635,87.78188,84.21);
            let max_time_seconds = 500.0;
            // temperatures are regressed to within 0.1 K
            let insulation_thickness_cm_for_regression_testing = 0.288;

            // regression is to within +/- 0.1 K
            let dhx_inlet_regression_temperature_degc = 
                dhx_inlet_temperature_set_point_degc;

            pri_loop_hot_leg_insulation_thickness_calibration_regression_test_v1(
                experimental_primary_mass_flowrate_kg_per_s, 
                heater_outlet_temperature_degc, 
                dhx_inlet_temperature_set_point_degc, 
                dhx_inlet_regression_temperature_degc,
                max_time_seconds, 
                insulation_thickness_cm_for_regression_testing);



        }).unwrap();

    // dataset number,pri loop mass flowrate (kg/s),Heater outlet (DegC),DHX shell top (DegC),DHX shell bottom (DegC),Heater inlet (DegC),
    // C-4,0.02949,94.71628,90.97595,63.9812,60.83029,
    let set_c4 = thread::Builder::new()
        .name("set_c4".to_string()).spawn(||{

            let (experimental_primary_mass_flowrate_kg_per_s,
                heater_outlet_temperature_degc,
                dhx_inlet_temperature_set_point_degc) =
                (0.02949,94.71628,90.76);
            let max_time_seconds = 500.0;
            // temperatures are regressed to within 0.1 K
            let insulation_thickness_cm_for_regression_testing = 0.23;

            // regression is to within +/- 0.1 K
            let dhx_inlet_regression_temperature_degc = 
                dhx_inlet_temperature_set_point_degc;

            pri_loop_hot_leg_insulation_thickness_calibration_regression_test_v1(
                experimental_primary_mass_flowrate_kg_per_s, 
                heater_outlet_temperature_degc, 
                dhx_inlet_temperature_set_point_degc, 
                dhx_inlet_regression_temperature_degc,
                max_time_seconds, 
                insulation_thickness_cm_for_regression_testing);



        }).unwrap();
    set_c1.join().unwrap();
    set_c2.join().unwrap();
    set_c3.join().unwrap();
    set_c4.join().unwrap();


}
// dataset number,pri loop mass flowrate (kg/s),Heater outlet (DegC),DHX shell top (DegC),DHX shell bottom (DegC),Heater inlet (DegC),
// C-1,0.02003,75.22747,71.47752,53.60943,50.45784,
#[test]
pub fn hot_leg_validation_set_c1(){

    let (experimental_primary_mass_flowrate_kg_per_s,
        heater_outlet_temperature_degc,
        dhx_inlet_temperature_set_point_degc) =
        (0.02003,75.22747,71.47752);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.24;

    let dhx_inlet_regression_temperature_degc = 
        71.17;

    pri_loop_hot_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_primary_mass_flowrate_kg_per_s, 
        heater_outlet_temperature_degc, 
        dhx_inlet_temperature_set_point_degc, 
        dhx_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
// dataset number,pri loop mass flowrate (kg/s),Heater outlet (DegC),DHX shell top (DegC),DHX shell bottom (DegC),Heater inlet (DegC),
// C-2,0.02367,82.41863,78.36713,57.13467,53.79036,
#[test]
pub fn hot_leg_validation_set_c2(){

    let (experimental_primary_mass_flowrate_kg_per_s,
        heater_outlet_temperature_degc,
        dhx_inlet_temperature_set_point_degc) =
        (0.02367,82.41863,78.36713);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.24;

    let dhx_inlet_regression_temperature_degc = 
        78.45;

    pri_loop_hot_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_primary_mass_flowrate_kg_per_s, 
        heater_outlet_temperature_degc, 
        dhx_inlet_temperature_set_point_degc, 
        dhx_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
// dataset number,pri loop mass flowrate (kg/s),Heater outlet (DegC),DHX shell top (DegC),DHX shell bottom (DegC),Heater inlet (DegC),
// C-3,0.02635,87.78188,84.37342,59.82845,56.71891,
#[test]
pub fn hot_leg_validation_set_c3(){

    let (experimental_primary_mass_flowrate_kg_per_s,
        heater_outlet_temperature_degc,
        dhx_inlet_temperature_set_point_degc) =
        (0.02635,87.78188,84.37342);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.24;

    let dhx_inlet_regression_temperature_degc = 
        83.89;

    pri_loop_hot_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_primary_mass_flowrate_kg_per_s, 
        heater_outlet_temperature_degc, 
        dhx_inlet_temperature_set_point_degc, 
        dhx_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
// dataset number,pri loop mass flowrate (kg/s),Heater outlet (DegC),DHX shell top (DegC),DHX shell bottom (DegC),Heater inlet (DegC),
// C-4,0.02949,94.71628,90.97595,63.9812,60.83029,
#[test]
pub fn hot_leg_validation_set_c4(){

    let (experimental_primary_mass_flowrate_kg_per_s,
        heater_outlet_temperature_degc,
        dhx_inlet_temperature_set_point_degc) =
        (0.02949,94.71628,90.97595);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.24;

    let dhx_inlet_regression_temperature_degc = 
        90.84;

    pri_loop_hot_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_primary_mass_flowrate_kg_per_s, 
        heater_outlet_temperature_degc, 
        dhx_inlet_temperature_set_point_degc, 
        dhx_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
// Based on Set c1 to c4 the average insulation is about 0.2495 cm
// dataset number,pri loop mass flowrate (kg/s),Heater outlet (DegC),DHX shell top (DegC),DHX shell bottom (DegC),Heater inlet (DegC),
// C-5,0.0319,100.37023,96.20228,67.05336,64.07406,
//
#[test]
pub fn hot_leg_validation_set_c5(){

    let (experimental_primary_mass_flowrate_kg_per_s,
        heater_outlet_temperature_degc,
        dhx_inlet_temperature_set_point_degc) =
        (0.0319,100.37023,96.20228);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.24;

    let dhx_inlet_regression_temperature_degc = 
        96.48;

    pri_loop_hot_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_primary_mass_flowrate_kg_per_s, 
        heater_outlet_temperature_degc, 
        dhx_inlet_temperature_set_point_degc, 
        dhx_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}


// Based on Set c1 to c4 the average insulation is about 0.2495 cm
// dataset number,pri loop mass flowrate (kg/s),Heater outlet (DegC),DHX shell top (DegC),DHX shell bottom (DegC),Heater inlet (DegC),
// C-6,0.03412,105.25073,101.3375,69.85085,67.1654,
//
//
#[test]
pub fn hot_leg_validation_set_c6(){

    let (experimental_primary_mass_flowrate_kg_per_s,
        heater_outlet_temperature_degc,
        dhx_inlet_temperature_set_point_degc) =
        (0.03412,105.25073,101.3375);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.24;

    let dhx_inlet_regression_temperature_degc = 
        101.38;

    pri_loop_hot_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_primary_mass_flowrate_kg_per_s, 
        heater_outlet_temperature_degc, 
        dhx_inlet_temperature_set_point_degc, 
        dhx_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}

// dataset number,pri loop mass flowrate (kg/s),Heater outlet (DegC),DHX shell top (DegC),DHX shell bottom (DegC),Heater inlet (DegC),
// C-7,0.03562,110.34289,106.43149,73.21226,70.6215,
//
#[test]
pub fn hot_leg_validation_set_c7(){

    let (experimental_primary_mass_flowrate_kg_per_s,
        heater_outlet_temperature_degc,
        dhx_inlet_temperature_set_point_degc) =
        (0.03562,110.34289,106.43149);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.24;

    let dhx_inlet_regression_temperature_degc = 
        106.41;

    pri_loop_hot_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_primary_mass_flowrate_kg_per_s, 
        heater_outlet_temperature_degc, 
        dhx_inlet_temperature_set_point_degc, 
        dhx_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}

// dataset number,pri loop mass flowrate (kg/s),Heater outlet (DegC),DHX shell top (DegC),DHX shell bottom (DegC),Heater inlet (DegC),
// C-8,0.03593,115.52364,111.37615,76.13202,73.63344,
//
#[test]
pub fn hot_leg_validation_set_c8(){

    let (experimental_primary_mass_flowrate_kg_per_s,
        heater_outlet_temperature_degc,
        dhx_inlet_temperature_set_point_degc) =
        (0.03593,115.52364,111.37615);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.24;

    let dhx_inlet_regression_temperature_degc = 
        111.41;

    pri_loop_hot_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_primary_mass_flowrate_kg_per_s, 
        heater_outlet_temperature_degc, 
        dhx_inlet_temperature_set_point_degc, 
        dhx_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}


// dataset number,pri loop mass flowrate (kg/s),Heater outlet (DegC),DHX shell top (DegC),DHX shell bottom (DegC),Heater inlet (DegC),
// C-9,0.03547,119.96879,116.05003,79.02407,76.54479,
#[test]
pub fn hot_leg_validation_set_c9(){

    let (experimental_primary_mass_flowrate_kg_per_s,
        heater_outlet_temperature_degc,
        dhx_inlet_temperature_set_point_degc) =
        (0.03547,119.96879,116.05003);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.24;

    let dhx_inlet_regression_temperature_degc = 
        115.63;

    pri_loop_hot_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_primary_mass_flowrate_kg_per_s, 
        heater_outlet_temperature_degc, 
        dhx_inlet_temperature_set_point_degc, 
        dhx_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// This test attempted to tweak the insulation thickness
/// in order to obtain the correct dhx inlet temperature 
///
/// I follow as the RELAP and SAM model did,
/// which is to reduce the convective thermal resistance between 
/// fluid and wall. Either by multiplying the heat transfer 
/// area density by a certain amount (SAM) or applying a multiplicative 
/// factor (page 40-41 of Zweibaum's thesis)
///
/// Zweibaum, N. (2015). Experimental validation of passive 
/// safety system models: Application to design and optimization 
/// of fluoride-salt-cooled, high-temperature reactors. 
/// University of California, Berkeley.
///
///
///
/// 
///
/// dataset number,pri loop mass flowrate (kg/s),Heater outlet (DegC),DHX shell top (DegC),DHX shell bottom (DegC),Heater inlet (DegC),
/// C-1,0.02003,75.22747,71.47752,53.60943,50.45784,
/// C-2,0.02367,82.41863,78.36713,57.13467,53.79036,
/// C-3,0.02635,87.78188,84.37342,59.82845,56.71891,
/// C-4,0.02949,94.71628,90.97595,63.9812,60.83029,
/// C-5,0.0319,100.37023,96.20228,67.05336,64.07406,
/// C-6,0.03412,105.25073,101.3375,69.85085,67.1654,
/// C-7,0.03562,110.34289,106.43149,73.21226,70.6215,
/// C-8,0.03593,115.52364,111.37615,76.13202,73.63344,
/// C-9,0.03547,119.96879,116.05003,79.02407,76.54479,
///
/// now, while I wanted to use PID controllers to auto-tune the thickness,
/// I found that manually calibrating is much faster
///
/// because I have to tune the controller which then tunes the 
/// insulation thickness
#[cfg(test)]
pub fn pri_loop_hot_leg_insulation_thickness_calibration_regression_test_v1(
    experimental_primary_mass_flowrate_kg_per_s: f64,
    heater_outlet_temperature_degc: f64,
    dhx_inlet_temperature_set_point_degc: f64,
    dhx_inlet_regression_temperature_degc: f64,
    max_time_seconds:f64,
    insulation_thickness_regression_cm: f64){
    use uom::si::length::centimeter;
    use uom::si::{f64::*, mass_rate::kilogram_per_second};

    use uom::si::{frequency::hertz, ratio::ratio, time::millisecond};

    use crate::pre_built_components::ciet_isothermal_test_components::*;
    use crate::prelude::beta_testing::{FluidArray, HeatTransferEntity, HeatTransferInteractionType, LiquidMaterial};
    use uom::ConstZero;

    use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};
    use uom::si::time::second;

    use chem_eng_real_time_process_control_simulator::alpha_nightly::transfer_fn_wrapper_and_enums::TransferFnTraits;
    use chem_eng_real_time_process_control_simulator::alpha_nightly::controllers::ProportionalController;
    use chem_eng_real_time_process_control_simulator::alpha_nightly::controllers::AnalogController;
    let experimental_primary_mass_flowrate = 
        MassRate::new::<kilogram_per_second>(
            experimental_primary_mass_flowrate_kg_per_s);

    let heater_outlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(
            heater_outlet_temperature_degc);

    let dhx_inlet_temperature_set_point = 
        ThermodynamicTemperature::new::<degree_celsius>(
            dhx_inlet_temperature_set_point_degc);


    // time setitings
    let mut current_simulation_time = Time::ZERO;
    let max_simulation_time = Time::new::<second>(max_time_seconds);
    let timestep = Time::new::<second>(0.5);

    // PID controller settings for ambient htc

    let default_insulation_thickness = Length::new::<centimeter>(5.08);
    let mut calibrated_insulation_thickness = 
        Length::new::<centimeter>(insulation_thickness_regression_cm);

    let average_temperature_for_advection_mass_flowrate_calcs = 
        ThermodynamicTemperature::new::<degree_celsius>(
            0.5*(heater_outlet_temperature_degc+dhx_inlet_temperature_set_point_degc)
            );
    let controller_gain = Ratio::new::<ratio>(1.2);
    let integral_time: Time = controller_gain / Frequency::new::<hertz>(1.0);
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

    let initial_temperature: ThermodynamicTemperature 
        = dhx_inlet_temperature_set_point;

    // components from heater outlet to dhx shell inlet
    // in pri loop

    // before this is the heater
    let mut heater_top_head_1a = new_heater_top_head_1a(initial_temperature);
    let mut static_mixer_10_label_2 = new_static_mixer_10_label_2(initial_temperature);
    let mut pipe_2a = new_pipe_2a(initial_temperature);
    let mut pipe_3 = new_pipe_3_relap_model(initial_temperature);
    let mut pipe_4 = new_pipe_4(initial_temperature);
    let mut pipe_5a = new_branch_5a(initial_temperature);
    let mut pipe_26 = new_pipe_26(initial_temperature);
    let mut pipe_25a = new_pipe_25a(initial_temperature);
    let mut static_mixer_21_label_25 = new_static_mixer_21_label_25(initial_temperature);
    // after this is component 24 (dhx sthe)
    // create the heat transfer interaction 
    let average_therminol_density = 
        LiquidMaterial::TherminolVP1.try_get_density(
            average_temperature_for_advection_mass_flowrate_calcs).unwrap();

    let advection_heat_transfer_interaction: HeatTransferInteractionType
        = HeatTransferInteractionType::
        new_advection_interaction(experimental_primary_mass_flowrate, 
            average_therminol_density, 
            average_therminol_density);

    // the heater outlet boundary condition
    let mut heater_outlet_bc = 
        HeatTransferEntity::new_const_temperature_bc(
            heater_outlet_temperature);

    let mut dhx_inlet_bc = 
        HeatTransferEntity::new_adiabatic_bc();

    let mut dhx_inlet_actual_temperature: ThermodynamicTemperature = 
        initial_temperature;

    // calculation loop
    while current_simulation_time < max_simulation_time {

        // take the actual outlet temperature 
        dhx_inlet_actual_temperature = {

            let static_mixer_21_pipe_fluid_arr_clone: FluidArray = 
                static_mixer_21_label_25.pipe_fluid_array
                .clone()
                .try_into()
                .unwrap();
            // take the front single cv temperature 
            //
            let static_mixer_21_front_single_cv_temperature: ThermodynamicTemperature 
                = static_mixer_21_pipe_fluid_arr_clone
                .front_single_cv
                .temperature;

            static_mixer_21_front_single_cv_temperature

        };
        //dbg!(&dhx_inlet_actual_temperature.get::<degree_celsius>());


        calibrated_insulation_thickness = {

            let reference_temperature_interval_deg_celsius = 10.0;
            // error = y_sp - y_measured
            let set_point_abs_error_deg_celsius = 
                dhx_inlet_temperature_set_point.get::<kelvin>()
                - dhx_inlet_actual_temperature.get::<kelvin>();

            let nondimensional_error: Ratio = 
                (set_point_abs_error_deg_celsius/
                 reference_temperature_interval_deg_celsius).into();

            // let's get the output 

            let mut dimensionless_insulation_thickness: Ratio
                = pid_controller.set_user_input_and_calc(
                    nondimensional_error, 
                    current_simulation_time).unwrap();

            // have at least some thickness because otherwise it would be 
            // unphysical
            let minimum_dimensionless_insulation_thickness = 
                Ratio::new::<ratio>(0.01);

            // nusselt number multiplicative coefficient should be at 
            // least 1
            if dimensionless_insulation_thickness < minimum_dimensionless_insulation_thickness {
                dimensionless_insulation_thickness = minimum_dimensionless_insulation_thickness;
            };

            // dimensionless insulation thickness = 
            // (calibrated_insulation_thickness)/reference_insulation_thickness

            let calibrated_insulation_thickness: Length = 
                dimensionless_insulation_thickness * default_insulation_thickness;

            calibrated_insulation_thickness

        };

        let controller_switched_on = false;

        if !controller_switched_on {
            calibrated_insulation_thickness = 
                Length::new::<centimeter>(insulation_thickness_regression_cm);
        };

        // now calibrate the insulation thickness for all 

        heater_top_head_1a.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        static_mixer_10_label_2.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_2a.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_3.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_4.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_5a.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_26.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_25a.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        static_mixer_21_label_25.calibrate_insulation_thickness(
            calibrated_insulation_thickness);



        // link the HTEs up

        heater_top_head_1a.pipe_fluid_array.link_to_back(
            &mut heater_outlet_bc, 
            advection_heat_transfer_interaction)
            .unwrap();

        heater_top_head_1a.pipe_fluid_array.link_to_front(
            &mut static_mixer_10_label_2.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        static_mixer_10_label_2.pipe_fluid_array.link_to_front(
            &mut pipe_2a.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_2a.pipe_fluid_array.link_to_front(
            &mut pipe_3.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_3.pipe_fluid_array.link_to_front(
            &mut pipe_4.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_4.pipe_fluid_array.link_to_front(
            &mut pipe_5a.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_5a.pipe_fluid_array.link_to_front(
            &mut pipe_26.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_26.pipe_fluid_array.link_to_front(
            &mut pipe_25a.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_25a.pipe_fluid_array.link_to_front(
            &mut static_mixer_21_label_25.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        static_mixer_21_label_25.pipe_fluid_array.link_to_front(
            &mut dhx_inlet_bc,
            advection_heat_transfer_interaction)
            .unwrap();

        // lateral_and_miscellaneous_connections
        let input_power = Power::ZERO;
        
        heater_top_head_1a.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        static_mixer_10_label_2.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        pipe_2a.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        pipe_3.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        pipe_4.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        pipe_5a.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        pipe_26.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        pipe_25a.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        static_mixer_21_label_25.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();

        // advance_timestep for all
        heater_top_head_1a.advance_timestep(timestep).unwrap(); 
        static_mixer_10_label_2.advance_timestep(timestep).unwrap(); 
        pipe_2a.advance_timestep(timestep).unwrap(); 
        pipe_3.advance_timestep(timestep).unwrap(); 
        pipe_4.advance_timestep(timestep).unwrap(); 
        pipe_5a.advance_timestep(timestep).unwrap(); 
        pipe_26.advance_timestep(timestep).unwrap(); 
        pipe_25a.advance_timestep(timestep).unwrap(); 
        static_mixer_21_label_25.advance_timestep(timestep).unwrap(); 

        current_simulation_time += timestep;
    }


    // after everything, let's dbg the acutal inlet temp of the dhx 
    dbg!(&(
            dhx_inlet_temperature_set_point.get::<degree_celsius>(),
            dhx_inlet_actual_temperature.get::<degree_celsius>(),
            calibrated_insulation_thickness.get::<centimeter>(),
            )
        );

    // check if set point and actual temperature are within 0.1 K of 
    // each other
    // in this test, it could not be achieved
    approx::assert_abs_diff_eq!(
        dhx_inlet_temperature_set_point.get::<degree_celsius>(),
        dhx_inlet_actual_temperature.get::<degree_celsius>(),
        epsilon=0.1
        );
    // check if actual tmeperature is equal to the regression 
    // temperature
    approx::assert_abs_diff_eq!(
        dhx_inlet_regression_temperature_degc,
        dhx_inlet_actual_temperature.get::<degree_celsius>(),
        epsilon=0.1
        );
    // check if insulation thickness is at regression value
    // temperature

    approx::assert_relative_eq!(
        insulation_thickness_regression_cm,
        calibrated_insulation_thickness.get::<centimeter>(),
        max_relative=0.01
        )


}


/// This test attempted to tweak the insulation thickness
/// in order to obtain the correct dhx inlet temperature 
///
/// I follow as the RELAP and SAM model did,
/// which is to reduce the convective thermal resistance between 
/// fluid and wall. Either by multiplying the heat transfer 
/// area density by a certain amount (SAM) or applying a multiplicative 
/// factor (page 40-41 of Zweibaum's thesis)
///
/// Zweibaum, N. (2015). Experimental validation of passive 
/// safety system models: Application to design and optimization 
/// of fluoride-salt-cooled, high-temperature reactors. 
/// University of California, Berkeley.
///
///
/// 
///
/// dataset number,pri loop mass flowrate (kg/s),Heater outlet (DegC),DHX shell top (DegC),DHX shell bottom (DegC),Heater inlet (DegC),
/// C-1,0.02003,75.22747,71.47752,53.60943,50.45784,
/// C-2,0.02367,82.41863,78.36713,57.13467,53.79036,
/// C-3,0.02635,87.78188,84.37342,59.82845,56.71891,
/// C-4,0.02949,94.71628,90.97595,63.9812,60.83029,
/// C-5,0.0319,100.37023,96.20228,67.05336,64.07406,
/// C-6,0.03412,105.25073,101.3375,69.85085,67.1654,
/// C-7,0.03562,110.34289,106.43149,73.21226,70.6215,
/// C-8,0.03593,115.52364,111.37615,76.13202,73.63344,
/// C-9,0.03547,119.96879,116.05003,79.02407,76.54479,
///
///
/// For this function, I want to test if a certain thickness results 
/// in the correct DHX inlet temperature. At least within 0.5 K 
/// the regression test temperature should be within 0.05 K
#[cfg(test)]
pub fn pri_loop_hot_leg_insulation_thickness_calibration_validation_test_v1(
    experimental_primary_mass_flowrate_kg_per_s: f64,
    heater_outlet_temperature_degc: f64,
    dhx_inlet_temperature_set_point_degc: f64,
    dhx_inlet_regression_temperature_degc: f64,
    max_time_seconds:f64,
    insulation_thickness_regression_cm: f64){
    use uom::si::length::centimeter;
    use uom::si::{f64::*, mass_rate::kilogram_per_second};

    use uom::si::{frequency::hertz, ratio::ratio, time::millisecond};

    use crate::pre_built_components::ciet_isothermal_test_components::*;
    use crate::prelude::beta_testing::{FluidArray, HeatTransferEntity, HeatTransferInteractionType, LiquidMaterial};
    use uom::ConstZero;

    use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};
    use uom::si::time::second;

    use chem_eng_real_time_process_control_simulator::alpha_nightly::transfer_fn_wrapper_and_enums::TransferFnTraits;
    use chem_eng_real_time_process_control_simulator::alpha_nightly::controllers::ProportionalController;
    use chem_eng_real_time_process_control_simulator::alpha_nightly::controllers::AnalogController;
    let experimental_primary_mass_flowrate = 
        MassRate::new::<kilogram_per_second>(
            experimental_primary_mass_flowrate_kg_per_s);

    let heater_outlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(
            heater_outlet_temperature_degc);

    let dhx_inlet_temperature_set_point = 
        ThermodynamicTemperature::new::<degree_celsius>(
            dhx_inlet_temperature_set_point_degc);


    // time setitings
    let mut current_simulation_time = Time::ZERO;
    let max_simulation_time = Time::new::<second>(max_time_seconds);
    let timestep = Time::new::<second>(0.5);

    // PID controller settings for ambient htc

    let default_insulation_thickness = Length::new::<centimeter>(5.08);
    let mut calibrated_insulation_thickness = 
        Length::new::<centimeter>(insulation_thickness_regression_cm);

    let average_temperature_for_advection_mass_flowrate_calcs = 
        ThermodynamicTemperature::new::<degree_celsius>(
            0.5*(heater_outlet_temperature_degc+dhx_inlet_temperature_set_point_degc)
            );
    let controller_gain = Ratio::new::<ratio>(1.2);
    let integral_time: Time = controller_gain / Frequency::new::<hertz>(1.0);
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

    let initial_temperature: ThermodynamicTemperature 
        = dhx_inlet_temperature_set_point;

    // components from heater outlet to dhx shell inlet
    // in pri loop

    // before this is the heater
    let mut heater_top_head_1a = new_heater_top_head_1a(initial_temperature);
    let mut static_mixer_10_label_2 = new_static_mixer_10_label_2(initial_temperature);
    let mut pipe_2a = new_pipe_2a(initial_temperature);
    let mut pipe_3 = new_pipe_3_relap_model(initial_temperature);
    let mut pipe_4 = new_pipe_4(initial_temperature);
    let mut pipe_5a = new_branch_5a(initial_temperature);
    let mut pipe_26 = new_pipe_26(initial_temperature);
    let mut pipe_25a = new_pipe_25a(initial_temperature);
    let mut static_mixer_21_label_25 = new_static_mixer_21_label_25(initial_temperature);
    // after this is component 24 (dhx sthe)
    // create the heat transfer interaction 
    let average_therminol_density = 
        LiquidMaterial::TherminolVP1.try_get_density(
            average_temperature_for_advection_mass_flowrate_calcs).unwrap();

    let advection_heat_transfer_interaction: HeatTransferInteractionType
        = HeatTransferInteractionType::
        new_advection_interaction(experimental_primary_mass_flowrate, 
            average_therminol_density, 
            average_therminol_density);

    // the heater outlet boundary condition
    let mut heater_outlet_bc = 
        HeatTransferEntity::new_const_temperature_bc(
            heater_outlet_temperature);

    let mut dhx_inlet_bc = 
        HeatTransferEntity::new_adiabatic_bc();

    let mut dhx_inlet_actual_temperature: ThermodynamicTemperature = 
        initial_temperature;

    // calculation loop
    while current_simulation_time < max_simulation_time {

        // take the actual outlet temperature 
        dhx_inlet_actual_temperature = {

            let static_mixer_21_pipe_fluid_arr_clone: FluidArray = 
                static_mixer_21_label_25.pipe_fluid_array
                .clone()
                .try_into()
                .unwrap();
            // take the front single cv temperature 
            //
            let static_mixer_21_front_single_cv_temperature: ThermodynamicTemperature 
                = static_mixer_21_pipe_fluid_arr_clone
                .front_single_cv
                .temperature;

            static_mixer_21_front_single_cv_temperature

        };
        //dbg!(&dhx_inlet_actual_temperature.get::<degree_celsius>());


        calibrated_insulation_thickness = {

            let reference_temperature_interval_deg_celsius = 10.0;
            // error = y_sp - y_measured
            let set_point_abs_error_deg_celsius = 
                dhx_inlet_temperature_set_point.get::<kelvin>()
                - dhx_inlet_actual_temperature.get::<kelvin>();

            let nondimensional_error: Ratio = 
                (set_point_abs_error_deg_celsius/
                 reference_temperature_interval_deg_celsius).into();

            // let's get the output 

            let mut dimensionless_insulation_thickness: Ratio
                = pid_controller.set_user_input_and_calc(
                    nondimensional_error, 
                    current_simulation_time).unwrap();

            // have at least some thickness because otherwise it would be 
            // unphysical
            let minimum_dimensionless_insulation_thickness = 
                Ratio::new::<ratio>(0.01);

            // nusselt number multiplicative coefficient should be at 
            // least 1
            if dimensionless_insulation_thickness < minimum_dimensionless_insulation_thickness {
                dimensionless_insulation_thickness = minimum_dimensionless_insulation_thickness;
            };

            // dimensionless insulation thickness = 
            // (calibrated_insulation_thickness)/reference_insulation_thickness

            let calibrated_insulation_thickness: Length = 
                dimensionless_insulation_thickness * default_insulation_thickness;

            calibrated_insulation_thickness

        };

        let controller_switched_on = false;

        if !controller_switched_on {
            calibrated_insulation_thickness = 
                Length::new::<centimeter>(insulation_thickness_regression_cm);
        };

        // now calibrate the insulation thickness for all 

        heater_top_head_1a.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        static_mixer_10_label_2.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_2a.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_3.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_4.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_5a.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_26.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_25a.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        static_mixer_21_label_25.calibrate_insulation_thickness(
            calibrated_insulation_thickness);



        // link the HTEs up

        heater_top_head_1a.pipe_fluid_array.link_to_back(
            &mut heater_outlet_bc, 
            advection_heat_transfer_interaction)
            .unwrap();

        heater_top_head_1a.pipe_fluid_array.link_to_front(
            &mut static_mixer_10_label_2.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        static_mixer_10_label_2.pipe_fluid_array.link_to_front(
            &mut pipe_2a.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_2a.pipe_fluid_array.link_to_front(
            &mut pipe_3.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_3.pipe_fluid_array.link_to_front(
            &mut pipe_4.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_4.pipe_fluid_array.link_to_front(
            &mut pipe_5a.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_5a.pipe_fluid_array.link_to_front(
            &mut pipe_26.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_26.pipe_fluid_array.link_to_front(
            &mut pipe_25a.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_25a.pipe_fluid_array.link_to_front(
            &mut static_mixer_21_label_25.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        static_mixer_21_label_25.pipe_fluid_array.link_to_front(
            &mut dhx_inlet_bc,
            advection_heat_transfer_interaction)
            .unwrap();

        // lateral_and_miscellaneous_connections
        let input_power = Power::ZERO;
        
        heater_top_head_1a.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        static_mixer_10_label_2.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        pipe_2a.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        pipe_3.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        pipe_4.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        pipe_5a.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        pipe_26.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        pipe_25a.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        static_mixer_21_label_25.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();

        // advance_timestep for all
        heater_top_head_1a.advance_timestep(timestep).unwrap(); 
        static_mixer_10_label_2.advance_timestep(timestep).unwrap(); 
        pipe_2a.advance_timestep(timestep).unwrap(); 
        pipe_3.advance_timestep(timestep).unwrap(); 
        pipe_4.advance_timestep(timestep).unwrap(); 
        pipe_5a.advance_timestep(timestep).unwrap(); 
        pipe_26.advance_timestep(timestep).unwrap(); 
        pipe_25a.advance_timestep(timestep).unwrap(); 
        static_mixer_21_label_25.advance_timestep(timestep).unwrap(); 

        current_simulation_time += timestep;
    }


    // after everything, let's dbg the acutal inlet temp of the dhx 
    dbg!(&(
            dhx_inlet_temperature_set_point.get::<degree_celsius>(),
            dhx_inlet_actual_temperature.get::<degree_celsius>(),
            calibrated_insulation_thickness.get::<centimeter>(),
            )
        );

    // check if set point and actual temperature are within 0.1 K of 
    // each other
    // in this test, it could not be achieved
    approx::assert_abs_diff_eq!(
        dhx_inlet_temperature_set_point.get::<degree_celsius>(),
        dhx_inlet_actual_temperature.get::<degree_celsius>(),
        epsilon=0.5
        );
    // check if actual tmeperature is equal to the regression 
    // temperature
    approx::assert_abs_diff_eq!(
        dhx_inlet_regression_temperature_degc,
        dhx_inlet_actual_temperature.get::<degree_celsius>(),
        epsilon=0.05
        );
    // check if insulation thickness is at regression value
    // temperature

    approx::assert_relative_eq!(
        insulation_thickness_regression_cm,
        calibrated_insulation_thickness.get::<centimeter>(),
        max_relative=0.01
        )


}
