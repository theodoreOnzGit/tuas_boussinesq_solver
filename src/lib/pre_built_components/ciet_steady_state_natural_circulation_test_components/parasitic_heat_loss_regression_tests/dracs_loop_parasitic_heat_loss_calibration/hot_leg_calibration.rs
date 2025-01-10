/// dataset number,dracs loop mass flowrate (kg/s),DHX tube top (outlet) (DegC),TCHX inlet (DegC),TCHX outlet(DegC),DHX tube bottom (DegC),
/// C-1,0.02686,53.00304,51.79332,40.42208,39.84713,
#[test]
pub fn hot_leg_regression_set_c1(){

    let (experimental_dracs_mass_flowrate_kg_per_s,
        dhx_tube_top_outlet_temperature_degc,
        tchx_inlet_temperature_set_point_degc) =
        (0.02686,53.00304,51.79332);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.75;

    // regression performed to within 0.05K
    let tchx_inlet_regression_temperature_degc = 
        51.81;

    dracs_hot_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_dracs_mass_flowrate_kg_per_s, 
        dhx_tube_top_outlet_temperature_degc, 
        tchx_inlet_temperature_set_point_degc, 
        tchx_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,dracs loop mass flowrate (kg/s),DHX tube top (outlet) (DegC),TCHX inlet (DegC),TCHX outlet(DegC),DHX tube bottom (DegC),
/// C-2,0.03055,55.30506,54.27495,40.25559,39.73516,
#[test]
pub fn hot_leg_regression_set_c2(){

    let (experimental_dracs_mass_flowrate_kg_per_s,
        dhx_tube_top_outlet_temperature_degc,
        tchx_inlet_temperature_set_point_degc) =
        (0.03055,55.30506,54.27495);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.75;

    // regression performed to within 0.05K
    let tchx_inlet_regression_temperature_degc = 
        54.17;

    dracs_hot_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_dracs_mass_flowrate_kg_per_s, 
        dhx_tube_top_outlet_temperature_degc, 
        tchx_inlet_temperature_set_point_degc, 
        tchx_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,dracs loop mass flowrate (kg/s),DHX tube top (outlet) (DegC),TCHX inlet (DegC),TCHX outlet(DegC),DHX tube bottom (DegC),
/// C-3,0.03345,56.82298,55.83001,39.74061,39.2569,
#[test]
pub fn hot_leg_regression_set_c3(){

    let (experimental_dracs_mass_flowrate_kg_per_s,
        dhx_tube_top_outlet_temperature_degc,
        tchx_inlet_temperature_set_point_degc) =
        (0.03345,56.82298,55.83001);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.75;

    // regression performed to within 0.05K
    let tchx_inlet_regression_temperature_degc = 
        55.68;

    dracs_hot_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_dracs_mass_flowrate_kg_per_s, 
        dhx_tube_top_outlet_temperature_degc, 
        tchx_inlet_temperature_set_point_degc, 
        tchx_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,dracs loop mass flowrate (kg/s),DHX tube top (outlet) (DegC),TCHX inlet (DegC),TCHX outlet(DegC),DHX tube bottom (DegC),
/// C-4,0.03649,59.44921,58.32055,40.25482,39.86112,
#[test]
pub fn hot_leg_regression_set_c4(){

    let (experimental_dracs_mass_flowrate_kg_per_s,
        dhx_tube_top_outlet_temperature_degc,
        tchx_inlet_temperature_set_point_degc) =
        (0.03649,59.44921,58.32055);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.75;

    // regression performed to within 0.05K
    let tchx_inlet_regression_temperature_degc = 
        58.31;

    dracs_hot_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_dracs_mass_flowrate_kg_per_s, 
        dhx_tube_top_outlet_temperature_degc, 
        tchx_inlet_temperature_set_point_degc, 
        tchx_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,dracs loop mass flowrate (kg/s),DHX tube top (outlet) (DegC),TCHX inlet (DegC),TCHX outlet(DegC),DHX tube bottom (DegC),
/// C-5,0.03869,61.31769,60.157,40.37106,40.01355,
#[test]
pub fn hot_leg_regression_set_c5(){

    let (experimental_dracs_mass_flowrate_kg_per_s,
        dhx_tube_top_outlet_temperature_degc,
        tchx_inlet_temperature_set_point_degc) =
        (0.03869,61.31769,60.157);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.75;

    // regression performed to within 0.05K
    let tchx_inlet_regression_temperature_degc = 
        60.19;

    dracs_hot_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_dracs_mass_flowrate_kg_per_s, 
        dhx_tube_top_outlet_temperature_degc, 
        tchx_inlet_temperature_set_point_degc, 
        tchx_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,dracs loop mass flowrate (kg/s),DHX tube top (outlet) (DegC),TCHX inlet (DegC),TCHX outlet(DegC),DHX tube bottom (DegC),
/// C-6,0.04115,62.69342,61.72605,39.97878,39.53125,
#[test]
pub fn hot_leg_regression_set_c6(){

    let (experimental_dracs_mass_flowrate_kg_per_s,
        dhx_tube_top_outlet_temperature_degc,
        tchx_inlet_temperature_set_point_degc) =
        (0.04115,62.69342,61.72605);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.75;

    // regression performed to within 0.05K
    let tchx_inlet_regression_temperature_degc = 
        61.59;

    dracs_hot_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_dracs_mass_flowrate_kg_per_s, 
        dhx_tube_top_outlet_temperature_degc, 
        tchx_inlet_temperature_set_point_degc, 
        tchx_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,dracs loop mass flowrate (kg/s),DHX tube top (outlet) (DegC),TCHX inlet (DegC),TCHX outlet(DegC),DHX tube bottom (DegC),
/// C-7,0.04312,64.45658,63.45641,40.24987,39.8924,
#[test]
pub fn hot_leg_regression_set_c7(){

    let (experimental_dracs_mass_flowrate_kg_per_s,
        dhx_tube_top_outlet_temperature_degc,
        tchx_inlet_temperature_set_point_degc) =
        (0.04312,64.45658,63.45641);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.75;

    // regression performed to within 0.05K
    let tchx_inlet_regression_temperature_degc = 
        63.36;

    dracs_hot_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_dracs_mass_flowrate_kg_per_s, 
        dhx_tube_top_outlet_temperature_degc, 
        tchx_inlet_temperature_set_point_degc, 
        tchx_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,dracs loop mass flowrate (kg/s),DHX tube top (outlet) (DegC),TCHX inlet (DegC),TCHX outlet(DegC),DHX tube bottom (DegC),
/// C-8,0.04509,66.11271,65.13191,40.14256,39.91183,
#[test]
pub fn hot_leg_regression_set_c8(){

    let (experimental_dracs_mass_flowrate_kg_per_s,
        dhx_tube_top_outlet_temperature_degc,
        tchx_inlet_temperature_set_point_degc) =
        (0.04509,66.11271,65.13191);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.75;

    // regression performed to within 0.05K
    let tchx_inlet_regression_temperature_degc = 
        65.02;

    dracs_hot_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_dracs_mass_flowrate_kg_per_s, 
        dhx_tube_top_outlet_temperature_degc, 
        tchx_inlet_temperature_set_point_degc, 
        tchx_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}
/// dataset number,dracs loop mass flowrate (kg/s),DHX tube top (outlet) (DegC),TCHX inlet (DegC),TCHX outlet(DegC),DHX tube bottom (DegC),
/// C-9,0.04699,67.40722,66.51369,39.87633,39.64593,
#[test]
pub fn hot_leg_regression_set_c9(){

    let (experimental_dracs_mass_flowrate_kg_per_s,
        dhx_tube_top_outlet_temperature_degc,
        tchx_inlet_temperature_set_point_degc) =
        (0.04699,67.40722,66.51369);
    let max_time_seconds = 500.0;
    // temperatures are validated to within 0.5 K
    let insulation_thickness_cm_for_regression_testing = 0.75;

    // regression performed to within 0.05K
    let tchx_inlet_regression_temperature_degc = 
        66.33;

    dracs_hot_leg_insulation_thickness_calibration_validation_test_v1(
        experimental_dracs_mass_flowrate_kg_per_s, 
        dhx_tube_top_outlet_temperature_degc, 
        tchx_inlet_temperature_set_point_degc, 
        tchx_inlet_regression_temperature_degc,
        max_time_seconds, 
        insulation_thickness_cm_for_regression_testing);


}

/// This test attempted to tweak the insulation thickness
/// in order to obtain the correct TCHX inlet temperature
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
/// dataset number,dracs loop mass flowrate (kg/s),DHX tube top (outlet) (DegC),TCHX inlet (DegC),TCHX outlet(DegC),DHX tube bottom (DegC),
/// C-1,0.02686,53.00304,51.79332,40.42208,39.84713,
/// C-2,0.03055,55.30506,54.27495,40.25559,39.73516,
/// C-3,0.03345,56.82298,55.83001,39.74061,39.2569,
/// C-4,0.03649,59.44921,58.32055,40.25482,39.86112,
/// C-5,0.03869,61.31769,60.157,40.37106,40.01355,
/// C-6,0.04115,62.69342,61.72605,39.97878,39.53125,
/// C-7,0.04312,64.45658,63.45641,40.24987,39.8924,
/// C-8,0.04509,66.11271,65.13191,40.14256,39.91183,
/// C-9,0.04699,67.40722,66.51369,39.87633,39.64593,
///
///
/// For this function, I want to test if a certain thickness results 
/// in the correct DHX inlet temperature. At least within 0.5 K 
/// the regression test temperature should be within 0.05 K
#[cfg(test)]
pub fn dracs_hot_leg_insulation_thickness_calibration_validation_test_v1(
    experimental_dracs_mass_flowrate_kg_per_s: f64,
    dhx_tube_top_outlet_temperature_degc: f64,
    tchx_inlet_temperature_set_point_degc: f64,
    tchx_inlet_regression_temperature_degc: f64,
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

    let dhx_tube_top_outlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(
            dhx_tube_top_outlet_temperature_degc);

    let tchx_inlet_temperature_set_point = 
        ThermodynamicTemperature::new::<degree_celsius>(
            tchx_inlet_temperature_set_point_degc);


    // time setitings
    let mut current_simulation_time = Time::ZERO;
    let max_simulation_time = Time::new::<second>(max_time_seconds);
    let timestep = Time::new::<second>(0.5);

    // calibrated thickness settings

    let mut calibrated_insulation_thickness = 
        Length::new::<centimeter>(insulation_thickness_regression_cm);

    let average_temperature_for_advection_mass_flowrate_calcs = 
        ThermodynamicTemperature::new::<degree_celsius>(
            0.5*(dhx_tube_top_outlet_temperature_degc+tchx_inlet_temperature_set_point_degc)
            );
    let initial_temperature: ThermodynamicTemperature 
        = tchx_inlet_temperature_set_point;

    // components from dhx tube top outlet to tchx inlet
    // in dracs loop

    // before this is dhx sthe tube side (component 30)
    let mut dhx_tube_side_30b = new_dhx_tube_side_30b(initial_temperature);
    let mut pipe_31a = new_pipe_31a(initial_temperature);
    let mut static_mixer_61_label_31 = new_static_mixer_61_label_31(initial_temperature);
    let mut pipe_32 = new_pipe_32(initial_temperature);
    let mut pipe_33 = new_pipe_33(initial_temperature);
    let mut pipe_34 = new_pipe_34(initial_temperature);
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
    let mut dhx_tube_top_outlet_bc = 
        HeatTransferEntity::new_const_temperature_bc(
            dhx_tube_top_outlet_temperature);

    let mut tchx_inlet_bc = 
        HeatTransferEntity::new_adiabatic_bc();

    let mut tchx_inlet_actual_temperature: ThermodynamicTemperature = 
        initial_temperature;

    // calculation loop
    while current_simulation_time < max_simulation_time {

        // take the actual outlet temperature 
        tchx_inlet_actual_temperature = {

            let pipe_34_fluid_arr_clone: FluidArray = 
                pipe_34.pipe_fluid_array
                .clone()
                .try_into()
                .unwrap();
            // take the front single cv temperature 
            //
            let pipe_34_front_single_cv_temperature: ThermodynamicTemperature 
                = pipe_34_fluid_arr_clone
                .front_single_cv
                .temperature;

            pipe_34_front_single_cv_temperature

        };
        //dbg!(&dhx_inlet_actual_temperature.get::<degree_celsius>());




        calibrated_insulation_thickness = 
            Length::new::<centimeter>(insulation_thickness_regression_cm);

        // now calibrate the insulation thickness for all 

        pipe_31a.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        static_mixer_61_label_31.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_32.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_33.calibrate_insulation_thickness(
            calibrated_insulation_thickness);
        pipe_34.calibrate_insulation_thickness(
            calibrated_insulation_thickness);



        // link the HTEs up

        dhx_tube_side_30b.pipe_fluid_array.link_to_back(
            &mut dhx_tube_top_outlet_bc, 
            advection_heat_transfer_interaction)
            .unwrap();

        dhx_tube_side_30b.pipe_fluid_array.link_to_front(
            &mut pipe_31a.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_31a.pipe_fluid_array.link_to_front(
            &mut static_mixer_61_label_31.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        static_mixer_61_label_31.pipe_fluid_array.link_to_front(
            &mut pipe_32.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_32.pipe_fluid_array.link_to_front(
            &mut pipe_33.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_33.pipe_fluid_array.link_to_front(
            &mut pipe_34.pipe_fluid_array, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_34.pipe_fluid_array.link_to_front(
            &mut tchx_inlet_bc, 
            advection_heat_transfer_interaction)
            .unwrap();


        // lateral_and_miscellaneous_connections
        let input_power = Power::ZERO;
        
        dhx_tube_side_30b.lateral_and_miscellaneous_connections_wall_correction(
            experimental_dracs_mass_flowrate, input_power)
            .unwrap();
        pipe_31a.lateral_and_miscellaneous_connections_wall_correction(
            experimental_dracs_mass_flowrate, input_power)
            .unwrap();
        static_mixer_61_label_31.lateral_and_miscellaneous_connections_wall_correction(
            experimental_dracs_mass_flowrate, input_power)
            .unwrap();
        pipe_32.lateral_and_miscellaneous_connections_wall_correction(
            experimental_dracs_mass_flowrate, input_power)
            .unwrap();
        pipe_33.lateral_and_miscellaneous_connections_wall_correction(
            experimental_dracs_mass_flowrate, input_power)
            .unwrap();
        pipe_34.lateral_and_miscellaneous_connections_wall_correction(
            experimental_dracs_mass_flowrate, input_power)
            .unwrap();

        // advance_timestep for all
        dhx_tube_side_30b.advance_timestep(timestep).unwrap(); 
        pipe_31a.advance_timestep(timestep).unwrap(); 
        static_mixer_61_label_31.advance_timestep(timestep).unwrap(); 
        pipe_32.advance_timestep(timestep).unwrap(); 
        pipe_33.advance_timestep(timestep).unwrap(); 
        pipe_34.advance_timestep(timestep).unwrap(); 

        current_simulation_time += timestep;
    }


    // after everything, let's dbg the acutal inlet temp of the dhx 
    dbg!(&(
            tchx_inlet_temperature_set_point.get::<degree_celsius>(),
            tchx_inlet_actual_temperature.get::<degree_celsius>(),
            calibrated_insulation_thickness.get::<centimeter>(),
            )
        );

    // check if set point and actual temperature are within 0.1 K of 
    // each other
    // in this test, it could not be achieved
    approx::assert_abs_diff_eq!(
        tchx_inlet_temperature_set_point.get::<degree_celsius>(),
        tchx_inlet_actual_temperature.get::<degree_celsius>(),
        epsilon=0.5
        );
    // check if actual tmeperature is equal to the regression 
    // temperature
    approx::assert_abs_diff_eq!(
        tchx_inlet_regression_temperature_degc,
        tchx_inlet_actual_temperature.get::<degree_celsius>(),
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
