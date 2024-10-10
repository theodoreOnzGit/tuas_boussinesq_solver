/// Zweibaum's unpublished data:
/// dataset number,dracs loop mass flowrate (kg/s),TCHX outlet(DegC),DHX tube bottom (DegC),
/// C-1,0.02686,40.42208,39.84713,
#[test]
pub fn cold_leg_regression_set_c1(){

    let (experimental_dracs_mass_flowrate_kg_per_s,
        tchx_outlet_temperature_degc,
        dhx_tube_bottom_inlet_temperature_set_point_degc) =
        (0.02686,40.42208,39.84713);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 3.00;

    // regression performed to within 0.05K
    let dhx_tube_bottom_inlet_regression_temperature_degc = 
        39.86;

    dracs_cold_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_dracs_mass_flowrate_kg_per_s, 
        tchx_outlet_temperature_degc, 
        dhx_tube_bottom_inlet_temperature_set_point_degc, 
        dhx_tube_bottom_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,dracs loop mass flowrate (kg/s),TCHX outlet(DegC),DHX tube bottom (DegC),
/// C-2,0.03055,40.25559,39.73516,
#[test]
pub fn cold_leg_regression_set_c2(){

    let (experimental_dracs_mass_flowrate_kg_per_s,
        tchx_outlet_temperature_degc,
        dhx_tube_bottom_inlet_temperature_set_point_degc) =
        (0.03055,40.25559,39.73516);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 3.00;

    // regression performed to within 0.05K
    let dhx_tube_bottom_inlet_regression_temperature_degc = 
        39.75;

    dracs_cold_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_dracs_mass_flowrate_kg_per_s, 
        tchx_outlet_temperature_degc, 
        dhx_tube_bottom_inlet_temperature_set_point_degc, 
        dhx_tube_bottom_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,dracs loop mass flowrate (kg/s),TCHX outlet(DegC),DHX tube bottom (DegC),
/// C-3,0.03345,39.74061,39.2569,
#[test]
pub fn cold_leg_regression_set_c3(){

    let (experimental_dracs_mass_flowrate_kg_per_s,
        tchx_outlet_temperature_degc,
        dhx_tube_bottom_inlet_temperature_set_point_degc) =
        (0.03345,39.74061,39.2569);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 3.00;

    // regression performed to within 0.05K
    let dhx_tube_bottom_inlet_regression_temperature_degc = 
        39.29;

    dracs_cold_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_dracs_mass_flowrate_kg_per_s, 
        tchx_outlet_temperature_degc, 
        dhx_tube_bottom_inlet_temperature_set_point_degc, 
        dhx_tube_bottom_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,dracs loop mass flowrate (kg/s),TCHX outlet(DegC),DHX tube bottom (DegC),
/// C-4,0.03649,40.25482,39.86112,
#[test]
pub fn cold_leg_regression_set_c4(){

    let (experimental_dracs_mass_flowrate_kg_per_s,
        tchx_outlet_temperature_degc,
        dhx_tube_bottom_inlet_temperature_set_point_degc) =
        (0.03649,40.25482,39.86112);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 3.00;

    // regression performed to within 0.05K
    let dhx_tube_bottom_inlet_regression_temperature_degc = 
        39.83;

    dracs_cold_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_dracs_mass_flowrate_kg_per_s, 
        tchx_outlet_temperature_degc, 
        dhx_tube_bottom_inlet_temperature_set_point_degc, 
        dhx_tube_bottom_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,dracs loop mass flowrate (kg/s),TCHX outlet(DegC),DHX tube bottom (DegC),
/// C-5,0.03869,40.37106,40.01355,
#[test]
pub fn cold_leg_regression_set_c5(){

    let (experimental_dracs_mass_flowrate_kg_per_s,
        tchx_outlet_temperature_degc,
        dhx_tube_bottom_inlet_temperature_set_point_degc) =
        (0.03869,40.37106,40.01355);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 3.00;

    // regression performed to within 0.05K
    let dhx_tube_bottom_inlet_regression_temperature_degc = 
        39.96;

    dracs_cold_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_dracs_mass_flowrate_kg_per_s, 
        tchx_outlet_temperature_degc, 
        dhx_tube_bottom_inlet_temperature_set_point_degc, 
        dhx_tube_bottom_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,dracs loop mass flowrate (kg/s),TCHX outlet(DegC),DHX tube bottom (DegC),
/// C-6,0.04115,39.97878,39.53125,
#[test]
pub fn cold_leg_regression_set_c6(){

    let (experimental_dracs_mass_flowrate_kg_per_s,
        tchx_outlet_temperature_degc,
        dhx_tube_bottom_inlet_temperature_set_point_degc) =
        (0.04115,39.97878,39.53125);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 3.00;

    // regression performed to within 0.05K
    let dhx_tube_bottom_inlet_regression_temperature_degc = 
        39.60;

    dracs_cold_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_dracs_mass_flowrate_kg_per_s, 
        tchx_outlet_temperature_degc, 
        dhx_tube_bottom_inlet_temperature_set_point_degc, 
        dhx_tube_bottom_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,dracs loop mass flowrate (kg/s),TCHX outlet(DegC),DHX tube bottom (DegC),
/// C-7,0.04312,40.24987,39.8924,
#[test]
pub fn cold_leg_regression_set_c7(){

    let (experimental_dracs_mass_flowrate_kg_per_s,
        tchx_outlet_temperature_degc,
        dhx_tube_bottom_inlet_temperature_set_point_degc) =
        (0.04312,40.24987,39.8924);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 3.00;

    // regression performed to within 0.05K
    let dhx_tube_bottom_inlet_regression_temperature_degc = 
        39.88;

    dracs_cold_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_dracs_mass_flowrate_kg_per_s, 
        tchx_outlet_temperature_degc, 
        dhx_tube_bottom_inlet_temperature_set_point_degc, 
        dhx_tube_bottom_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,dracs loop mass flowrate (kg/s),TCHX outlet(DegC),DHX tube bottom (DegC),
/// C-8,0.04509,40.14256,39.91183,
#[test]
pub fn cold_leg_regression_set_c8(){

    let (experimental_dracs_mass_flowrate_kg_per_s,
        tchx_outlet_temperature_degc,
        dhx_tube_bottom_inlet_temperature_set_point_degc) =
        (0.04509,40.14256,39.91183);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 3.00;

    // regression performed to within 0.05K
    let dhx_tube_bottom_inlet_regression_temperature_degc = 
        39.79;

    dracs_cold_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_dracs_mass_flowrate_kg_per_s, 
        tchx_outlet_temperature_degc, 
        dhx_tube_bottom_inlet_temperature_set_point_degc, 
        dhx_tube_bottom_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,dracs loop mass flowrate (kg/s),TCHX outlet(DegC),DHX tube bottom (DegC),
/// C-9,0.04699,39.87633,39.64593,
#[test]
pub fn cold_leg_regression_set_c9(){

    let (experimental_dracs_mass_flowrate_kg_per_s,
        tchx_outlet_temperature_degc,
        dhx_tube_bottom_inlet_temperature_set_point_degc) =
        (0.04699,39.87633,39.64593);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 3.00;

    // regression performed to within 0.05K
    let dhx_tube_bottom_inlet_regression_temperature_degc = 
        39.54;

    dracs_cold_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_dracs_mass_flowrate_kg_per_s, 
        tchx_outlet_temperature_degc, 
        dhx_tube_bottom_inlet_temperature_set_point_degc, 
        dhx_tube_bottom_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}

/// This test attempted to tweak the insulation thickness
/// in order to obtain the correct DHX tube inlet temperature
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
/// Zweibaum's unpublished data:
/// dataset number,dracs loop mass flowrate (kg/s),TCHX outlet(DegC),DHX tube bottom (DegC),
/// C-1,0.02686,40.42208,39.84713,
/// C-2,0.03055,40.25559,39.73516,
/// C-3,0.03345,39.74061,39.2569,
/// C-4,0.03649,40.25482,39.86112,
/// C-5,0.03869,40.37106,40.01355,
/// C-6,0.04115,39.97878,39.53125,
/// C-7,0.04312,40.24987,39.8924,
/// C-8,0.04509,40.14256,39.91183,
/// C-9,0.04699,39.87633,39.64593,
///
///
/// For this function, I want to test if a certain thickness results 
/// in the correct DHX inlet temperature. At least within 0.5 K 
/// the regression test temperature should be within 0.05 K
#[cfg(test)]
pub fn dracs_cold_leg_insulation_thickness_calibration_validation_test_v1(
    experimental_dracs_mass_flowrate_kg_per_s: f64,
    tchx_outlet_temperature_degc: f64,
    dhx_tube_bottom_inlet_temperature_set_point_degc: f64,
    dhx_tube_bottom_inlet_regression_temperature_degc: f64,
    max_time_seconds:f64,
    insulation_thickness_regression_cm: f64){
    use uom::si::length::centimeter;
    use uom::si::{f64::*, mass_rate::kilogram_per_second};


    use crate::pre_built_components::ciet_steady_state_natural_circulation_test_components::dracs_loop_components::*;
    use crate::prelude::beta_testing::{FluidArray, HeatTransferEntity, HeatTransferInteractionType, LiquidMaterial};
    use uom::ConstZero;

    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::si::time::second;

    let experimental_dracs_mass_flowrate = 
        MassRate::new::<kilogram_per_second>(
            experimental_dracs_mass_flowrate_kg_per_s);

    let tchx_outlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(
            tchx_outlet_temperature_degc);

    let dhx_tube_bottom_inlet_temperature_set_point = 
        ThermodynamicTemperature::new::<degree_celsius>(
            dhx_tube_bottom_inlet_temperature_set_point_degc);


    // time setitings
    let mut current_simulation_time = Time::ZERO;
    let max_simulation_time = Time::new::<second>(max_time_seconds);
    let timestep = Time::new::<second>(0.5);

    // calibrated thickness settings

    let mut calibrated_insulation_thickness = 
        Length::new::<centimeter>(insulation_thickness_regression_cm);

    let average_temperature_for_advection_mass_flowrate_calcs = 
        ThermodynamicTemperature::new::<degree_celsius>(
            0.5*(tchx_outlet_temperature_degc+dhx_tube_bottom_inlet_temperature_set_point_degc)
            );
    let initial_temperature: ThermodynamicTemperature 
        = dhx_tube_bottom_inlet_temperature_set_point;

    // components from dhx tube top outlet to tchx inlet
    // in dracs loop

    // before this is dhx sthe tube side (component 30)
    let mut static_mixer_60_label_36 = new_static_mixer_60_label_36(initial_temperature);
    let mut pipe_36a = new_pipe_36a(initial_temperature);
    let mut pipe_37 = new_pipe_37(initial_temperature);
    let mut flowmeter_60_37a = new_flowmeter_60_37a(initial_temperature);
    let mut pipe_38 = new_pipe_38(initial_temperature);
    let mut pipe_39 = new_pipe_39(initial_temperature);
    let mut dhx_tube_side_30a = new_dhx_tube_side_30a(initial_temperature);
    // after this is the tchx horizontal side (component 35a)
    // create the heat transfer interaction 
    let average_therminol_density = 
        LiquidMaterial::TherminolVP1.try_get_density(
            average_temperature_for_advection_mass_flowrate_calcs).unwrap();

    let advection_heat_transfer_interaction: HeatTransferInteractionType
        = HeatTransferInteractionType::
        new_advection_interaction(experimental_dracs_mass_flowrate, 
            average_therminol_density, 
            average_therminol_density);

    // the heater outlet boundary condition
    let mut tchx_outlet_bc = 
        HeatTransferEntity::new_const_temperature_bc(
            tchx_outlet_temperature);

    let mut dhx_tube_bottom_inlet_bc = 
        HeatTransferEntity::new_adiabatic_bc();

    let mut dhx_tube_bottom_inlet_actual_temperature: ThermodynamicTemperature = 
        initial_temperature;

    // calculation loop
    while current_simulation_time < max_simulation_time {

        // take the actual outlet temperature 
        dhx_tube_bottom_inlet_actual_temperature = {

            let dhx_tube_30a_fluid_arr_clone: FluidArray = 
                dhx_tube_side_30a.pipe_fluid_array
                .clone()
                .try_into()
                .unwrap();
            // take the front single cv temperature 
            //
            let dhx_tube_30a_front_single_cv_temperature: ThermodynamicTemperature 
                = dhx_tube_30a_fluid_arr_clone
                .front_single_cv
                .temperature;

            dhx_tube_30a_front_single_cv_temperature

        };
        //dbg!(&dhx_inlet_actual_temperature.get::<degree_celsius>());




        calibrated_insulation_thickness = 
            Length::new::<centimeter>(insulation_thickness_regression_cm);

        // now calibrate the insulation thickness for all 

        static_mixer_60_label_36.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_36a.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_37.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_38.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_39.calibrate_insulation_thickness(
            calibrated_insulation_thickness);



        // link the HTEs up

        static_mixer_60_label_36.pipe_fluid_array.link_to_back(
            &mut tchx_outlet_bc, 
            advection_heat_transfer_interaction)
            .unwrap();

        static_mixer_60_label_36.pipe_fluid_array.link_to_front(
            &mut pipe_36a.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_36a.pipe_fluid_array.link_to_front(
            &mut pipe_37.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_37.pipe_fluid_array.link_to_front(
            &mut flowmeter_60_37a.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        flowmeter_60_37a.pipe_fluid_array.link_to_front(
            &mut pipe_38.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_38.pipe_fluid_array.link_to_front(
            &mut pipe_39.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_39.pipe_fluid_array.link_to_front(
            &mut dhx_tube_side_30a.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        dhx_tube_side_30a.pipe_fluid_array.link_to_front(
            &mut dhx_tube_bottom_inlet_bc, 
            advection_heat_transfer_interaction)
            .unwrap();

        // lateral_and_miscellaneous_connections
        let input_power = Power::ZERO;
        
        static_mixer_60_label_36.lateral_and_miscellaneous_connections_wall_correction(
            experimental_dracs_mass_flowrate, input_power)
            .unwrap();
        pipe_36a.lateral_and_miscellaneous_connections_wall_correction(
            experimental_dracs_mass_flowrate, input_power)
            .unwrap();
        pipe_37.lateral_and_miscellaneous_connections_wall_correction(
            experimental_dracs_mass_flowrate, input_power)
            .unwrap();
        flowmeter_60_37a.lateral_and_miscellaneous_connections_wall_correction(
            experimental_dracs_mass_flowrate, input_power)
            .unwrap();
        pipe_38.lateral_and_miscellaneous_connections_wall_correction(
            experimental_dracs_mass_flowrate, input_power)
            .unwrap();
        pipe_39.lateral_and_miscellaneous_connections_wall_correction(
            experimental_dracs_mass_flowrate, input_power)
            .unwrap();
        dhx_tube_side_30a.lateral_and_miscellaneous_connections_wall_correction(
            experimental_dracs_mass_flowrate, input_power)
            .unwrap();

        // advance_timestep for all
        static_mixer_60_label_36.advance_timestep(timestep).unwrap(); 
        pipe_36a.advance_timestep(timestep).unwrap(); 
        pipe_37.advance_timestep(timestep).unwrap(); 
        flowmeter_60_37a.advance_timestep(timestep).unwrap(); 
        pipe_38.advance_timestep(timestep).unwrap(); 
        pipe_39.advance_timestep(timestep).unwrap(); 
        dhx_tube_side_30a.advance_timestep(timestep).unwrap(); 

        current_simulation_time += timestep;
    }


    // after everything, let's dbg the acutal inlet temp of the dhx 
    dbg!(&(
            dhx_tube_bottom_inlet_temperature_set_point.get::<degree_celsius>(),
            dhx_tube_bottom_inlet_actual_temperature.get::<degree_celsius>(),
            calibrated_insulation_thickness.get::<centimeter>(),
            )
        );

    // check if set point and actual temperature are within 0.1 K of 
    // each other
    // in this test, it could not be achieved
    approx::assert_abs_diff_eq!(
        dhx_tube_bottom_inlet_temperature_set_point.get::<degree_celsius>(),
        dhx_tube_bottom_inlet_actual_temperature.get::<degree_celsius>(),
        epsilon=0.5
        );
    // check if actual tmeperature is equal to the regression 
    // temperature
    approx::assert_abs_diff_eq!(
        dhx_tube_bottom_inlet_regression_temperature_degc,
        dhx_tube_bottom_inlet_actual_temperature.get::<degree_celsius>(),
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
