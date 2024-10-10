/// dataset number,pri loop mass flowrate (kg/s),DHX shell bottom (DegC),Heater inlet (DegC),
/// C-1,0.02003,53.60943,50.45784,
#[test]
pub fn cold_leg_regression_set_c1(){

    let (experimental_primary_mass_flowrate_kg_per_s,
        dhx_outlet_temperature_degc,
        heater_inlet_temperature_set_point_degc) =
        (0.02003,53.60943,50.45784);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.15;

    // regression performed to within 0.05K
    let heater_inlet_regression_temperature_degc = 
        50.69;

    cold_leg_insulation_thickness_validation_test_v1(
        experimental_primary_mass_flowrate_kg_per_s, 
        dhx_outlet_temperature_degc, 
        heater_inlet_temperature_set_point_degc, 
        heater_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,pri loop mass flowrate (kg/s),DHX shell bottom (DegC),Heater inlet (DegC),
/// C-2,0.02367,57.13467,53.79036,
///
/// I was unable to have this work with insulation thickness = 0.15 cm,
/// 0.1 cm worked
#[test]
pub fn cold_leg_regression_set_c2(){

    let (experimental_primary_mass_flowrate_kg_per_s,
        dhx_outlet_temperature_degc,
        heater_inlet_temperature_set_point_degc) =
        (0.02367,57.13467,53.79036);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.1;

    // regression performed to within 0.05K
    let heater_inlet_regression_temperature_degc = 
        54.07;

    cold_leg_insulation_thickness_validation_test_v1(
        experimental_primary_mass_flowrate_kg_per_s, 
        dhx_outlet_temperature_degc, 
        heater_inlet_temperature_set_point_degc, 
        heater_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,pri loop mass flowrate (kg/s),DHX shell bottom (DegC),Heater inlet (DegC),
/// C-3,0.02635,59.82845,56.71891,
#[test]
pub fn cold_leg_validation_set_c3(){

    let (experimental_primary_mass_flowrate_kg_per_s,
        dhx_outlet_temperature_degc,
        heater_inlet_temperature_set_point_degc) =
        (0.02635,59.82845,56.71891);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.15;

    // regression performed to within 0.05K
    let heater_inlet_regression_temperature_degc = 
        57.14;

    cold_leg_insulation_thickness_validation_test_v1(
        experimental_primary_mass_flowrate_kg_per_s, 
        dhx_outlet_temperature_degc, 
        heater_inlet_temperature_set_point_degc, 
        heater_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,pri loop mass flowrate (kg/s),DHX shell bottom (DegC),Heater inlet (DegC),
/// C-4,0.02949,63.9812,60.83029,
#[test]
pub fn cold_leg_validation_set_c4(){

    let (experimental_primary_mass_flowrate_kg_per_s,
        dhx_outlet_temperature_degc,
        heater_inlet_temperature_set_point_degc) =
        (0.02949,63.9812,60.83029);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.15;

    // regression performed to within 0.05K
    let heater_inlet_regression_temperature_degc = 
        61.31;

    cold_leg_insulation_thickness_validation_test_v1(
        experimental_primary_mass_flowrate_kg_per_s, 
        dhx_outlet_temperature_degc, 
        heater_inlet_temperature_set_point_degc, 
        heater_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,pri loop mass flowrate (kg/s),DHX shell bottom (DegC),Heater inlet (DegC),
/// C-5,0.0319,67.05336,64.07406,
#[test]
pub fn cold_leg_validation_set_c5(){

    let (experimental_primary_mass_flowrate_kg_per_s,
        dhx_outlet_temperature_degc,
        heater_inlet_temperature_set_point_degc) =
        (0.0319,67.05336,64.07406);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.15;

    // regression performed to within 0.05K
    let heater_inlet_regression_temperature_degc = 
        64.41;

    cold_leg_insulation_thickness_validation_test_v1(
        experimental_primary_mass_flowrate_kg_per_s, 
        dhx_outlet_temperature_degc, 
        heater_inlet_temperature_set_point_degc, 
        heater_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,pri loop mass flowrate (kg/s),DHX shell bottom (DegC),Heater inlet (DegC),
/// C-6,0.03412,69.85085,67.1654,
#[test]
pub fn cold_leg_validation_set_c6(){

    let (experimental_primary_mass_flowrate_kg_per_s,
        dhx_outlet_temperature_degc,
        heater_inlet_temperature_set_point_degc) =
        (0.03412,69.85085,67.1654);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.15;

    // regression performed to within 0.05K
    let heater_inlet_regression_temperature_degc = 
        67.27;

    cold_leg_insulation_thickness_validation_test_v1(
        experimental_primary_mass_flowrate_kg_per_s, 
        dhx_outlet_temperature_degc, 
        heater_inlet_temperature_set_point_degc, 
        heater_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,pri loop mass flowrate (kg/s),DHX shell bottom (DegC),Heater inlet (DegC),
/// C-7,0.03562,73.21226,70.6215,
#[test]
pub fn cold_leg_validation_set_c7(){

    let (experimental_primary_mass_flowrate_kg_per_s,
        dhx_outlet_temperature_degc,
        heater_inlet_temperature_set_point_degc) =
        (0.03562,73.21226,70.6215);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.15;

    // regression performed to within 0.05K
    let heater_inlet_regression_temperature_degc = 
        70.57;

    cold_leg_insulation_thickness_validation_test_v1(
        experimental_primary_mass_flowrate_kg_per_s, 
        dhx_outlet_temperature_degc, 
        heater_inlet_temperature_set_point_degc, 
        heater_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,pri loop mass flowrate (kg/s),DHX shell bottom (DegC),Heater inlet (DegC),
/// C-8,0.03593,76.13202,73.63344,
#[test]
pub fn cold_leg_validation_set_c8(){

    let (experimental_primary_mass_flowrate_kg_per_s,
        dhx_outlet_temperature_degc,
        heater_inlet_temperature_set_point_degc) =
        (0.03593,76.13202,73.63344);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.15;

    // regression performed to within 0.05K
    let heater_inlet_regression_temperature_degc = 
        73.38;

    cold_leg_insulation_thickness_validation_test_v1(
        experimental_primary_mass_flowrate_kg_per_s, 
        dhx_outlet_temperature_degc, 
        heater_inlet_temperature_set_point_degc, 
        heater_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,pri loop mass flowrate (kg/s),DHX shell bottom (DegC),Heater inlet (DegC),
/// C-9,0.03547,79.02407,76.54479,
#[test]
pub fn cold_leg_validation_set_c9(){

    let (experimental_primary_mass_flowrate_kg_per_s,
        dhx_outlet_temperature_degc,
        heater_inlet_temperature_set_point_degc) =
        (0.03547,79.02407,76.54479);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.15;

    // regression performed to within 0.05K
    let heater_inlet_regression_temperature_degc = 
        76.05;

    cold_leg_insulation_thickness_validation_test_v1(
        experimental_primary_mass_flowrate_kg_per_s, 
        dhx_outlet_temperature_degc, 
        heater_inlet_temperature_set_point_degc, 
        heater_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}

/// This test attempted to tweak the inner tube nusselt number
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
/// dataset number,pri loop mass flowrate (kg/s),DHX shell bottom (DegC),Heater inlet (DegC),
/// C-1,0.02003,53.60943,50.45784,
/// C-2,0.02367,57.13467,53.79036,
/// C-3,0.02635,59.82845,56.71891,
/// C-4,0.02949,63.9812,60.83029,
/// C-5,0.0319,67.05336,64.07406,
/// C-6,0.03412,69.85085,67.1654,
/// C-7,0.03562,73.21226,70.6215,
/// C-8,0.03593,76.13202,73.63344,
/// C-9,0.03547,79.02407,76.54479,
///
///
/// For this function, I want to test if a certain thickness results 
/// in the correct DHX inlet temperature. At least within 0.5 K 
/// the regression test temperature should be within 0.05 K
#[cfg(test)]
pub fn cold_leg_insulation_thickness_validation_test_v1(
    experimental_primary_mass_flowrate_kg_per_s: f64,
    heater_outlet_temperature_degc: f64,
    dhx_inlet_temperature_set_point_degc: f64,
    heater_inlet_regression_temperature_degc: f64,
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

    let heater_inlet_temperature_set_point = 
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
        = heater_inlet_temperature_set_point;

    // components from dhx outlet to heater inlet
    // in pri loop

    // before this is the dhx outlet (dhx shell bottom)
    let mut static_mixer_20_label_23 = new_static_mixer_20_label_23(initial_temperature);
    let mut pipe_23a = new_pipe_23a(initial_temperature);
    let mut pipe_22 = new_pipe_22_relap_model(initial_temperature);
    let mut fm_20_21a = new_flowmeter_20_label_21a(initial_temperature);
    let mut pipe_21 = new_pipe_21(initial_temperature);
    let mut pipe_20 = new_pipe_20(initial_temperature);
    let mut pipe_19 = new_pipe_19(initial_temperature);
    let mut pipe_17b = new_branch_17b(initial_temperature);
    let mut pipe_18 = new_pipe_18(initial_temperature);
    let mut heater_bottom_head_1b = new_heater_bottom_head_1b(initial_temperature);
    // after this is heater
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
    let mut dhx_shell_bottom_outlet_bc = 
        HeatTransferEntity::new_const_temperature_bc(
            heater_outlet_temperature);

    let mut heater_inlet_bc = 
        HeatTransferEntity::new_adiabatic_bc();

    let mut heater_inlet_actual_temperature: ThermodynamicTemperature = 
        initial_temperature;

    // calculation loop
    while current_simulation_time < max_simulation_time {

        // take the actual outlet temperature 
        heater_inlet_actual_temperature = {

            let heater_bottom_head_1b_pipe_fluid_arr_clone: FluidArray = 
                heater_bottom_head_1b.pipe_fluid_array
                .clone()
                .try_into()
                .unwrap();
            // take the front single cv temperature 
            //
            let heater_bottom_head_1b_front_single_cv_temperature: ThermodynamicTemperature 
                = heater_bottom_head_1b_pipe_fluid_arr_clone
                .front_single_cv
                .temperature;

            heater_bottom_head_1b_front_single_cv_temperature

        };
        //dbg!(&dhx_inlet_actual_temperature.get::<degree_celsius>());


        calibrated_insulation_thickness = {

            let reference_temperature_interval_deg_celsius = 10.0;
            // error = y_sp - y_measured
            let set_point_abs_error_deg_celsius = 
                heater_inlet_temperature_set_point.get::<kelvin>()
                - heater_inlet_actual_temperature.get::<kelvin>();

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

        static_mixer_20_label_23.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_23a.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_22.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_21.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        // note that flowmeter is considered not insulated
        pipe_20.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_19.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_17b.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_18.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        heater_bottom_head_1b.calibrate_insulation_thickness(
            calibrated_insulation_thickness);



        // link the HTEs up

        static_mixer_20_label_23.pipe_fluid_array.link_to_back(
            &mut dhx_shell_bottom_outlet_bc, 
            advection_heat_transfer_interaction)
            .unwrap();

        static_mixer_20_label_23.pipe_fluid_array.link_to_front(
            &mut pipe_23a.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_23a.pipe_fluid_array.link_to_front(
            &mut pipe_22.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_22.pipe_fluid_array.link_to_front(
            &mut fm_20_21a.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        fm_20_21a.pipe_fluid_array.link_to_front(
            &mut pipe_21.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_21.pipe_fluid_array.link_to_front(
            &mut pipe_20.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_20.pipe_fluid_array.link_to_front(
            &mut pipe_19.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_19.pipe_fluid_array.link_to_front(
            &mut pipe_17b.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_17b.pipe_fluid_array.link_to_front(
            &mut pipe_18.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_18.pipe_fluid_array.link_to_front(
            &mut heater_bottom_head_1b.pipe_fluid_array,
            advection_heat_transfer_interaction)
            .unwrap();

        heater_bottom_head_1b.pipe_fluid_array.link_to_front(
            &mut heater_inlet_bc,
            advection_heat_transfer_interaction)
            .unwrap();

        // lateral_and_miscellaneous_connections
        let input_power = Power::ZERO;
        
        static_mixer_20_label_23.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        pipe_23a.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        pipe_22.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        fm_20_21a.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        pipe_21.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        pipe_20.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        pipe_19.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        pipe_17b.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        pipe_18.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();
        heater_bottom_head_1b.lateral_and_miscellaneous_connections_wall_correction(
            experimental_primary_mass_flowrate, input_power)
            .unwrap();

        // advance_timestep for all
        static_mixer_20_label_23.advance_timestep(timestep).unwrap(); 
        pipe_23a.advance_timestep(timestep).unwrap(); 
        pipe_22.advance_timestep(timestep).unwrap(); 
        fm_20_21a.advance_timestep(timestep).unwrap(); 
        pipe_21.advance_timestep(timestep).unwrap(); 
        pipe_20.advance_timestep(timestep).unwrap(); 
        pipe_19.advance_timestep(timestep).unwrap(); 
        pipe_17b.advance_timestep(timestep).unwrap(); 
        pipe_18.advance_timestep(timestep).unwrap(); 
        heater_bottom_head_1b.advance_timestep(timestep).unwrap(); 

        current_simulation_time += timestep;
    }


    // after everything, let's dbg the acutal inlet temp of the dhx 
    dbg!(&(
            heater_inlet_temperature_set_point.get::<degree_celsius>(),
            heater_inlet_actual_temperature.get::<degree_celsius>(),
            calibrated_insulation_thickness.get::<centimeter>(),
            )
        );

    // check if set point and actual temperature are within 0.1 K of 
    // each other
    // in this test, it could not be achieved
    approx::assert_abs_diff_eq!(
        heater_inlet_temperature_set_point.get::<degree_celsius>(),
        heater_inlet_actual_temperature.get::<degree_celsius>(),
        epsilon=0.5
        );
    // check if actual tmeperature is equal to the regression 
    // temperature
    approx::assert_abs_diff_eq!(
        heater_inlet_regression_temperature_degc,
        heater_inlet_actual_temperature.get::<degree_celsius>(),
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
