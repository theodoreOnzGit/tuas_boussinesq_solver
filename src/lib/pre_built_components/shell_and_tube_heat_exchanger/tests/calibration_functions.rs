
/// based on 
/// Zweibaum's unpublished data:
/// dataset number,pri loop mass flowrate (kg/s),DRACS loop mass flowrate (kg/s),DHX shell top inlet (DegC),DHX tube bottom inlet(DegC),DHX shell bottom outlet (DegC),DHX tube top outlet (DegC),
/// C-9,0.03547,0.04699,116.05003,39.64593,79.02407,67.40722,
///
/// but modified to test calibration functions 
///
/// firstly, one regression test without heat loss
#[test]
pub fn calibration_regression_set_c9_no_heat_loss(){

    let (experimental_pri_shell_side_mass_flowrate_kg_per_s_abs,
        experimental_dracs_tube_side_mass_flowrate_kg_per_s_abs,
        dhx_shell_side_inlet_temp_degc, 
        dhx_tube_side_inlet_temp_degc, 
        dhx_shell_side_outlet_temp_set_point_degc, 
        dhx_tube_side_outlet_temp_set_point_degc) =
        (0.03547,0.04699,116.05003,39.64593,79.02407,67.40722);

    // temperatures are validated to within 0.5 K
    // regression performed to within 0.05K
    let dhx_shell_side_outlet_regression_temperature_degc = 53.58;
    let dhx_tube_side_outlet_regression_temperature_degc = 53.02;
    // tested that at 1500s, the results are more or less the same 
    // as at 750s and even 500s
    //
    // let max_time_seconds = 1500.0;
    // let max_time_seconds = 750.0;
    // let max_time_seconds = 500.0;
    let max_time_seconds = 400.0;

    // settings for insulation and shell side nusselt correction 
    // factor
    let shell_side_to_tubes_nusselt_number_correction_factor = 1.0;
    let insulation_thickness_regression_cm = 5.08;
    let shell_side_to_ambient_nusselt_correction_factor = 0.0;
    let mesh_refined_setting = false;

    dhx_calibration_regression_unit_test_shell_to_tube_only(
        experimental_dracs_tube_side_mass_flowrate_kg_per_s_abs, 
        experimental_pri_shell_side_mass_flowrate_kg_per_s_abs, 
        dhx_shell_side_inlet_temp_degc, 
        dhx_shell_side_outlet_temp_set_point_degc, 
        dhx_shell_side_outlet_regression_temperature_degc, 
        dhx_tube_side_inlet_temp_degc, 
        dhx_tube_side_outlet_temp_set_point_degc, 
        dhx_tube_side_outlet_regression_temperature_degc, 
        max_time_seconds, 
        insulation_thickness_regression_cm, 
        shell_side_to_tubes_nusselt_number_correction_factor,
        shell_side_to_ambient_nusselt_correction_factor,
        mesh_refined_setting);


}
/// based on 
/// Zweibaum's unpublished data:
/// dataset number,pri loop mass flowrate (kg/s),DRACS loop mass flowrate (kg/s),DHX shell top inlet (DegC),DHX tube bottom inlet(DegC),DHX shell bottom outlet (DegC),DHX tube top outlet (DegC),
/// C-9,0.03547,0.04699,116.05003,39.64593,79.02407,67.40722,
///
/// but modified to test calibration functions 
///
/// firstly, one regression test without heat loss
/// with a mesh refined version with slightly stricter criteria
#[test]
pub fn calibration_regression_set_c9_no_heat_loss_mesh_refined(){

    let (experimental_pri_shell_side_mass_flowrate_kg_per_s_abs,
        experimental_dracs_tube_side_mass_flowrate_kg_per_s_abs,
        dhx_shell_side_inlet_temp_degc, 
        dhx_tube_side_inlet_temp_degc, 
        dhx_shell_side_outlet_temp_set_point_degc, 
        dhx_tube_side_outlet_temp_set_point_degc) =
        (0.03547,0.04699,116.05003,39.64593,79.02407,67.40722);

    // temperatures are validated to within 0.5 K
    // regression performed to within 0.05K
    let dhx_shell_side_outlet_regression_temperature_degc = 53.58;
    let dhx_tube_side_outlet_regression_temperature_degc = 53.02;
    // tested that at 1500s, the results are more or less the same 
    // as at 750s and even 500s
    //
    // let max_time_seconds = 1500.0;
    // let max_time_seconds = 750.0;
    // let max_time_seconds = 500.0;
    let max_time_seconds = 400.0;

    // settings for insulation and shell side nusselt correction 
    // factor
    let shell_side_to_tubes_nusselt_number_correction_factor = 1.0;
    let insulation_thickness_regression_cm = 5.08;
    let shell_side_to_ambient_nusselt_correction_factor = 0.0;
    let mesh_refined_setting = true;

    dhx_calibration_regression_unit_test_shell_to_tube_only(
        experimental_dracs_tube_side_mass_flowrate_kg_per_s_abs, 
        experimental_pri_shell_side_mass_flowrate_kg_per_s_abs, 
        dhx_shell_side_inlet_temp_degc, 
        dhx_shell_side_outlet_temp_set_point_degc, 
        dhx_shell_side_outlet_regression_temperature_degc, 
        dhx_tube_side_inlet_temp_degc, 
        dhx_tube_side_outlet_temp_set_point_degc, 
        dhx_tube_side_outlet_regression_temperature_degc, 
        max_time_seconds, 
        insulation_thickness_regression_cm, 
        shell_side_to_tubes_nusselt_number_correction_factor,
        shell_side_to_ambient_nusselt_correction_factor,
        mesh_refined_setting);


}
///
///
/// in this module, I want to calibrate dhx shell and tube heat exchanger (STHE)
/// heat transfer and calibration.
///
/// on page 13 of Zou's publication
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
/// Zou writes that the STHE for the DHX has an underestimated heat transfer 
/// coefficient rather than an overestimated one as mentioned by Zweibaum,
/// Zou attributes this to a typo error as increased heat transfer area 
/// densities were used.
///
/// Again set C is used to calibrate the DHX data
///
/// Zweibaum's unpublished data:
/// pri loop is shell side flowrate, dracs loop is tube side flowrate
/// dataset number,pri loop mass flowrate (kg/s),DRACS loop mass flowrate (kg/s),DHX shell top inlet (DegC),DHX tube bottom inlet(DegC),DHX shell bottom outlet (DegC),DHX tube top outlet (DegC),
/// C-1,0.02003,0.02686,71.47752,39.84713,53.60943,53.00304,
/// C-2,0.02367,0.03055,78.36713,39.73516,57.13467,55.30506,
/// C-3,0.02635,0.03345,84.37342,39.2569,59.82845,56.82298,
/// C-4,0.02949,0.03649,90.97595,39.86112,63.9812,59.44921,
/// C-5,0.0319,0.03869,96.20228,40.01355,67.05336,61.31769,
/// C-6,0.03412,0.04115,101.3375,39.53125,69.85085,62.69342,
/// C-7,0.03562,0.04312,106.43149,39.8924,73.21226,64.45658,
/// C-8,0.03593,0.04509,111.37615,39.91183,76.13202,66.11271,
/// C-9,0.03547,0.04699,116.05003,39.64593,79.02407,67.40722,
/// 
/// To calibrate, 
///
/// (1) first adjust the shell side to tubes nusselt number 
/// until the tube side outlet temperature is correct,
///
/// (2) secondly, adjust the insulation thickness until the shell side 
/// outlet temperature is correct
#[cfg(test)]
pub fn dhx_calibration_regression_unit_test_shell_to_tube_only(
    experimental_dracs_tube_side_mass_flowrate_kg_per_s_abs: f64,
    experimental_pri_shell_side_mass_flowrate_kg_per_s_abs: f64,
    dhx_shell_side_inlet_temp_degc: f64,
    dhx_shell_side_outlet_temp_set_point_degc: f64,
    _dhx_shell_side_outlet_regression_temperature_degc: f64,
    dhx_tube_side_inlet_temp_degc: f64,
    dhx_tube_side_outlet_temp_set_point_degc: f64,
    _dhx_tube_side_outlet_regression_temperature_degc: f64,
    max_time_seconds:f64,
    insulation_thickness_regression_cm: f64,
    shell_side_to_tubes_nusselt_number_correction_factor: f64,
    shell_side_to_ambient_nusselt_correction_factor: f64,
    mesh_refined: bool){
    use uom::si::length::centimeter;
    use uom::si::ratio::ratio;
    use uom::si::thermal_resistance::kelvin_per_watt;
    use uom::si::{f64::*, mass_rate::kilogram_per_second};


    use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::pre_built_components::ciet_steady_state_natural_circulation_test_components::coupled_dracs_loop_tests::dhx_constructor::*;
    use crate::pre_built_components::shell_and_tube_heat_exchanger::SimpleShellAndTubeHeatExchanger;
    use crate::prelude::beta_testing::{FluidArray, HeatTransferEntity, HeatTransferInteractionType, LiquidMaterial};
    use uom::ConstZero;

    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::si::time::second;

    // pri flowrate (shell side)
    // shell side flows in forward direction
    let experimental_pri_mass_flowrate = 
        MassRate::new::<kilogram_per_second>(
            experimental_pri_shell_side_mass_flowrate_kg_per_s_abs);
    // dracs flowrate (tube side)
    // tube side flows in reverse direction
    // (need to negative)
    let experimental_dracs_mass_flowrate = 
        MassRate::new::<kilogram_per_second>(
            -experimental_dracs_tube_side_mass_flowrate_kg_per_s_abs);


    let dhx_shell_side_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(
            dhx_shell_side_inlet_temp_degc);

    let dhx_tube_side_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(
            dhx_tube_side_inlet_temp_degc);

    let dhx_shell_side_outlet_temperature_set_point = 
        ThermodynamicTemperature::new::<degree_celsius>(
            dhx_shell_side_outlet_temp_set_point_degc);


    // time setitings
    let mut current_simulation_time = Time::ZERO;
    let max_simulation_time = Time::new::<second>(max_time_seconds);
    let timestep = Time::new::<second>(0.5);

    // calibrated thickness settings

    let calibrated_insulation_thickness = 
        Length::new::<centimeter>(insulation_thickness_regression_cm);

    // calibrated nusselt correlation settings (using Gnielinksi correlation)

    let calibrated_nusselt_factor = 
        Ratio::new::<ratio>(shell_side_to_tubes_nusselt_number_correction_factor);

    let calibrated_parasitic_heat_loss_nusselt_factor = 
        Ratio::new::<ratio>(shell_side_to_ambient_nusselt_correction_factor);

    // initial temperature 
    let average_temperature_for_advection_mass_flowrate_calcs = 
        ThermodynamicTemperature::new::<degree_celsius>(
            0.5*(dhx_shell_side_inlet_temp_degc+dhx_shell_side_outlet_temp_set_point_degc)
            );
    let initial_temperature: ThermodynamicTemperature 
        = dhx_shell_side_outlet_temperature_set_point;

    // components from dhx tube top outlet to tchx inlet
    // in dracs loop


    let mut dhx_sthe = new_dhx_sthe_version_1(initial_temperature);

    if mesh_refined {
        dhx_sthe = new_dhx_sthe_version_1_mesh_refined(initial_temperature);
    }
    // create the heat transfer interaction 
    let average_therminol_density = 
        LiquidMaterial::TherminolVP1.try_get_density(
            average_temperature_for_advection_mass_flowrate_calcs).unwrap();

    let shell_side_advection_heat_transfer_interaction: HeatTransferInteractionType
        = HeatTransferInteractionType::
        new_advection_interaction(experimental_pri_mass_flowrate, 
            average_therminol_density, 
            average_therminol_density);
    let tube_side_advection_heat_transfer_interaction: HeatTransferInteractionType
        = HeatTransferInteractionType::
        new_advection_interaction(experimental_dracs_mass_flowrate, 
            average_therminol_density, 
            average_therminol_density);

    // the heater outlet boundary condition
    let mut dhx_shell_inlet_bc = 
        HeatTransferEntity::new_const_temperature_bc(
            dhx_shell_side_inlet_temperature);

    let mut dhx_tube_inlet_bc = 
        HeatTransferEntity::new_const_temperature_bc(
            dhx_tube_side_inlet_temperature);

    let mut dhx_shell_outlet_bc = 
        HeatTransferEntity::new_adiabatic_bc();
    let mut dhx_tube_outlet_bc = 
        HeatTransferEntity::new_adiabatic_bc();

    let mut dhx_shell_outlet_actual_temperature: ThermodynamicTemperature = 
        initial_temperature;
    let mut dhx_tube_outlet_actual_temperature: ThermodynamicTemperature = 
        initial_temperature;
    let mut dhx_shell_entrance_cv_actual_temperature: ThermodynamicTemperature = 
        initial_temperature;
    let mut dhx_tube_entrance_cv_actual_temperature: ThermodynamicTemperature = 
        initial_temperature;

    // calculation loop
    while current_simulation_time < max_simulation_time {

        // for dhx sthe calibration
        // the shell side flow is positive, so take the front single cv 
        // and tube side flow is negative
        // so, for shell temperature take it from the front cv,
        // and for tube temperature, take it from the back cv
        dhx_shell_outlet_actual_temperature = {

            let dhx_shell_fluid_arr_clone: FluidArray = 
                dhx_sthe.shell_side_fluid_array
                .clone()
                .try_into()
                .unwrap();
            // take the front single cv temperature 
            //
            let dhx_shell_fluid_arr_front_single_cv_temperature: ThermodynamicTemperature 
                = dhx_shell_fluid_arr_clone
                .front_single_cv
                .temperature;

            dhx_shell_entrance_cv_actual_temperature 
                = dhx_shell_fluid_arr_clone
                .back_single_cv
                .temperature;

            dhx_shell_fluid_arr_front_single_cv_temperature

        };
        dhx_tube_outlet_actual_temperature = {

            let dhx_tube_fluid_arr_clone: FluidArray = 
                dhx_sthe.tube_side_fluid_array_for_single_tube
                .clone()
                .try_into()
                .unwrap();
            // take the front single cv temperature 
            //
            let dhx_tube_fluid_arr_back_single_cv_temperature: ThermodynamicTemperature 
                = dhx_tube_fluid_arr_clone
                .back_single_cv
                .temperature;

            dhx_tube_entrance_cv_actual_temperature
                = dhx_tube_fluid_arr_clone
                .front_single_cv
                .temperature;

            dhx_tube_fluid_arr_back_single_cv_temperature

        };

        let debug_on = false;
        if debug_on {
            dbg!(&(
                    dhx_shell_outlet_actual_temperature.get::<degree_celsius>(),
                    dhx_tube_outlet_actual_temperature.get::<degree_celsius>()
            ));
        }
        
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

        // now calibrate the insulation thickness for all 

        dhx_sthe.calibrate_insulation_thickness(calibrated_insulation_thickness);

        // link the HTEs up

        dhx_sthe.shell_side_fluid_array.link_to_back(
            &mut dhx_shell_inlet_bc, 
            shell_side_advection_heat_transfer_interaction)
            .unwrap();

        dhx_sthe.shell_side_fluid_array.link_to_front(
            &mut dhx_shell_outlet_bc, 
            shell_side_advection_heat_transfer_interaction)
            .unwrap();

        dhx_sthe.tube_side_fluid_array_for_single_tube.link_to_front(
            &mut dhx_tube_inlet_bc, 
            tube_side_advection_heat_transfer_interaction)
            .unwrap();

        dhx_sthe.tube_side_fluid_array_for_single_tube.link_to_back(
            &mut dhx_tube_outlet_bc, 
            tube_side_advection_heat_transfer_interaction)
            .unwrap();

        // lateral_and_miscellaneous_connections
        let correct_prandtl_for_wall_temperatures = true;

        dhx_sthe.lateral_and_miscellaneous_connections(
            correct_prandtl_for_wall_temperatures, 
            experimental_dracs_mass_flowrate, 
            experimental_pri_mass_flowrate)
            .unwrap();

        // advance_timestep for all
        dhx_sthe.advance_timestep(timestep).unwrap();

        current_simulation_time += timestep;
    }


    // after everything, let's dbg the acutal inlet temp of the dhx 
    dbg!(&(
            dhx_shell_side_inlet_temperature.get::<degree_celsius>(),
            dhx_tube_side_inlet_temperature.get::<degree_celsius>(),
            dhx_shell_side_outlet_temp_set_point_degc,
            dhx_shell_outlet_actual_temperature.get::<degree_celsius>(),
            dhx_tube_side_outlet_temp_set_point_degc,
            dhx_tube_outlet_actual_temperature.get::<degree_celsius>(),
            calibrated_insulation_thickness.get::<centimeter>(),
            shell_side_to_tubes_nusselt_number_correction_factor
            )
        );
    // for calibration, calibrate tube side first, 
    // then shell side,

    // for tube,
    // check if set point and actual temperature are within 0.5 K of 
    // each other
    // in this test, it could not be achieved
    // debug shell side dimensionless numbers

    let shell_side_reynolds = dhx_sthe.reynolds_shell_side();
    let shell_side_bulk_prandtl = dhx_sthe.bulk_prandtl_number_shell_side();
    let inner_tube_wall_prandtl = dhx_sthe.wall_prandtl_number_shell_side_fluid_for_inner_tube();
    let nusselt_number_shell_side_to_tubes = dhx_sthe.nusselt_number_shell_side_to_tubes();

    dbg!(&(
            shell_side_reynolds,
            shell_side_bulk_prandtl,
            inner_tube_wall_prandtl,
            nusselt_number_shell_side_to_tubes
    )
    );
    let outer_tube_wall_prandtl = dhx_sthe.wall_prandtl_number_shell_side_fluid_for_outer_tube();
    let nusselt_number_shell_side_parasitic = dhx_sthe.nusselt_number_shell_side_parasitic();

    dbg!(&(
            shell_side_reynolds,
            shell_side_bulk_prandtl,
            outer_tube_wall_prandtl,
            nusselt_number_shell_side_parasitic
    )
    );
    let correct_prandtl_for_wall_temperatures = true;
    // thermal resistance assertion for shell to tubes 
    let overall_htc_postprocessing: HeatTransfer = 
        dhx_sthe.overall_htc_based_on_conductance(
            correct_prandtl_for_wall_temperatures,
            experimental_dracs_mass_flowrate,
            experimental_pri_mass_flowrate);

    let shell_side_heat_transfer_area: Area = 
        dhx_sthe.circular_tube_bundle_heat_transfer_area_shell_side();

    let thermal_resistance_overall_expected: ThermalResistance
        = (overall_htc_postprocessing*shell_side_heat_transfer_area).recip();

    let thermal_resistance_overall_actual: ThermalResistance = 
        dhx_sthe.get_shell_side_convective_thermal_resistance_cylindrical()
        + dhx_sthe.get_inner_tubes_convective_thermal_resistance_based_on_wetted_perimeter()
        + dhx_sthe.get_inner_tubes_cylindrical_thermal_resistance();


    dbg!(&(thermal_resistance_overall_expected,
            thermal_resistance_overall_actual));

    // assert shell to tube side thermal resistance
    //
    // assert to within 0.01%
    // to ensure that thermal resistance functions are working correctly

    approx::assert_relative_eq!(
        thermal_resistance_overall_expected.get::<kelvin_per_watt>(),
        thermal_resistance_overall_actual.get::<kelvin_per_watt>(),
        max_relative = 0.0001
        );



    // assert nusselt number
    let tube_inlet_temperature = dhx_tube_side_inlet_temperature;
    let tube_outlet_temeprature = dhx_tube_outlet_actual_temperature;
    let tube_mass_flowrate = experimental_dracs_mass_flowrate;
    let shell_inlet_temperature = dhx_shell_side_inlet_temperature;
    let shell_outlet_temeprature = dhx_shell_outlet_actual_temperature;
    let shell_mass_flowrate = experimental_pri_mass_flowrate;
    let is_counter_current = true;
    // assert overall thermal resistance 
    // vs that gained from expt data
    let thermal_resistance_based_on_expt_data: ThermalResistance = 
        dhx_sthe.get_ua_based_on_mass_flowrates_and_temperature_differences(
            tube_inlet_temperature, 
            tube_outlet_temeprature, 
            tube_mass_flowrate, 
            shell_inlet_temperature, 
            shell_outlet_temeprature, 
            shell_mass_flowrate, 
            is_counter_current).recip();

    // should agree within 5%
    //
    // issue not mainly due to mesh refinement, but perhaps that the inlet 
    // temperature BCs used to calculate LMTD are not actually used for 
    // calculating temperature differences
    //
    // mesh refinement does play a role though, reduces the 
    // effect of the error from 5% to 3% when I roughly double the number 
    // of cvs

    if mesh_refined {
        approx::assert_relative_eq!(
            thermal_resistance_overall_expected.get::<kelvin_per_watt>(),
            thermal_resistance_based_on_expt_data.get::<kelvin_per_watt>(),
            max_relative = 0.03
        );
    }
     
    approx::assert_relative_eq!(
        thermal_resistance_overall_expected.get::<kelvin_per_watt>(),
        thermal_resistance_based_on_expt_data.get::<kelvin_per_watt>(),
        max_relative = 0.05
        );

    // now lets base it on entrance cv temperature instead
    //
    // I just need to be careful, 
    // because before,
    // I calculated UA based on LMTD and also Q
    // Q = m cp (T_out - T_in)
    // both based on the entrance cv temperature
    // 
    // the heat transfer rate needs to be calculated based on the 
    // boundary condition inlet temperature in order to be accurate
    //
    // however, the LMTD needs to be calculated based on the 
    // entrance cv temperature

    let heat_transferred_in_sthe = 
        dhx_sthe.get_tube_side_heat_rate_based_on_mass_flowrate(
            tube_inlet_temperature, 
            tube_outlet_temeprature, 
            tube_mass_flowrate);
    let thermal_resistance_based_on_entrance_cv_temp: ThermalResistance = 
        SimpleShellAndTubeHeatExchanger::get_ua_based_on_heat_transfer_and_temperature_differences(
            heat_transferred_in_sthe, 
            dhx_tube_entrance_cv_actual_temperature, 
            tube_outlet_temeprature, 
            dhx_shell_entrance_cv_actual_temperature, 
            shell_outlet_temeprature, 
            is_counter_current).recip();


    dbg!(&(thermal_resistance_overall_expected,
            thermal_resistance_based_on_expt_data,
            thermal_resistance_based_on_entrance_cv_temp));


    // the one based on entrance cv should agree to within 0.5%
    // note: mesh refinement gets it to within 0.1%
    if mesh_refined {
        approx::assert_relative_eq!(
            thermal_resistance_overall_expected.get::<kelvin_per_watt>(),
            thermal_resistance_based_on_entrance_cv_temp.get::<kelvin_per_watt>(),
            max_relative = 0.001
        );

    }
    approx::assert_relative_eq!(
        thermal_resistance_overall_expected.get::<kelvin_per_watt>(),
        thermal_resistance_based_on_entrance_cv_temp.get::<kelvin_per_watt>(),
        max_relative = 0.005
        );
    // assert shell side nusselt number 
    let nusselt_number_shell_side_to_tubes_actual =
        dhx_sthe.obtain_shell_side_nusselt_number_based_on_expt_data(
            tube_inlet_temperature, 
            tube_outlet_temeprature, 
            tube_mass_flowrate, 
            shell_inlet_temperature, 
            shell_outlet_temeprature, 
            shell_mass_flowrate, 
            is_counter_current);

    // these two should agree to within 8%
    // now due to error propagation, the calculated shell side nusselt 
    // number agrees to wihtin 8%
    approx::assert_relative_eq!(
        nusselt_number_shell_side_to_tubes.get::<ratio>(),
        nusselt_number_shell_side_to_tubes_actual.get::<ratio>(),
        max_relative = 0.08
        );

    if mesh_refined {
        approx::assert_relative_eq!(
            nusselt_number_shell_side_to_tubes.get::<ratio>(),
            nusselt_number_shell_side_to_tubes_actual.get::<ratio>(),
            max_relative = 0.05
        );

    }

    // if one wants to factor out the effect of the entrance cv and 
    // boundary condition differences 

    let nusselt_number_shell_side_to_tubes_based_on_entrance_cv_temp =
        dhx_sthe.obtain_shell_side_nusselt_number_based_on_expt_data_and_heat_rate(
            heat_transferred_in_sthe,
            dhx_tube_entrance_cv_actual_temperature, 
            tube_outlet_temeprature, 
            dhx_shell_entrance_cv_actual_temperature, 
            shell_outlet_temeprature, 
            is_counter_current);

    dbg!(&(
            nusselt_number_shell_side_to_tubes,
            nusselt_number_shell_side_to_tubes_actual,
            nusselt_number_shell_side_to_tubes_based_on_entrance_cv_temp
    ));

    // once this correction is made, nusselt numbers of shell side should 
    // agree to within 0.5%
    if mesh_refined {
        // should agree to within 0.2% if mesh refined
        approx::assert_relative_eq!(
            nusselt_number_shell_side_to_tubes.get::<ratio>(),
            nusselt_number_shell_side_to_tubes_based_on_entrance_cv_temp.get::<ratio>(),
            max_relative = 0.002
        );
    }
    approx::assert_relative_eq!(
        nusselt_number_shell_side_to_tubes.get::<ratio>(),
        nusselt_number_shell_side_to_tubes_based_on_entrance_cv_temp.get::<ratio>(),
        max_relative = 0.005
    );
}


///
///
/// in this module, I want to calibrate dhx shell and tube heat exchanger (STHE)
/// heat transfer and calibration.
///
/// on page 13 of Zou's publication
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
/// Zou writes that the STHE for the DHX has an underestimated heat transfer 
/// coefficient rather than an overestimated one as mentioned by Zweibaum,
/// Zou attributes this to a typo error as increased heat transfer area 
/// densities were used.
///
/// Again set C is used to calibrate the DHX data
///
/// Zweibaum's unpublished data:
/// pri loop is shell side flowrate, dracs loop is tube side flowrate
/// dataset number,pri loop mass flowrate (kg/s),DRACS loop mass flowrate (kg/s),DHX shell top inlet (DegC),DHX tube bottom inlet(DegC),DHX shell bottom outlet (DegC),DHX tube top outlet (DegC),
/// C-1,0.02003,0.02686,71.47752,39.84713,53.60943,53.00304,
/// C-2,0.02367,0.03055,78.36713,39.73516,57.13467,55.30506,
/// C-3,0.02635,0.03345,84.37342,39.2569,59.82845,56.82298,
/// C-4,0.02949,0.03649,90.97595,39.86112,63.9812,59.44921,
/// C-5,0.0319,0.03869,96.20228,40.01355,67.05336,61.31769,
/// C-6,0.03412,0.04115,101.3375,39.53125,69.85085,62.69342,
/// C-7,0.03562,0.04312,106.43149,39.8924,73.21226,64.45658,
/// C-8,0.03593,0.04509,111.37615,39.91183,76.13202,66.11271,
/// C-9,0.03547,0.04699,116.05003,39.64593,79.02407,67.40722,
/// 
/// To calibrate, 
///
/// (1) first adjust the shell side to tubes nusselt number 
/// until the tube side outlet temperature is correct,
///
/// (2) secondly, adjust the insulation thickness until the shell side 
/// outlet temperature is correct
#[cfg(test)]
pub fn dhx_calibration_regression_unit_test_parasitic_heat_loss_only(
    experimental_dracs_tube_side_mass_flowrate_kg_per_s_abs: f64,
    experimental_pri_shell_side_mass_flowrate_kg_per_s_abs: f64,
    dhx_shell_side_inlet_temp_degc: f64,
    dhx_shell_side_outlet_temp_set_point_degc: f64,
    _dhx_shell_side_outlet_regression_temperature_degc: f64,
    dhx_tube_side_inlet_temp_degc: f64,
    dhx_tube_side_outlet_temp_set_point_degc: f64,
    _dhx_tube_side_outlet_regression_temperature_degc: f64,
    max_time_seconds:f64,
    insulation_thickness_regression_cm: f64,
    shell_side_to_tubes_nusselt_number_correction_factor: f64,
    shell_side_to_ambient_nusselt_correction_factor: f64,){
    use uom::si::length::centimeter;
    use uom::si::ratio::ratio;
    use uom::si::thermal_resistance::kelvin_per_watt;
    use uom::si::{f64::*, mass_rate::kilogram_per_second};


    use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::pre_built_components::ciet_steady_state_natural_circulation_test_components::coupled_dracs_loop_tests::dhx_constructor::new_dhx_sthe_version_1;
    use crate::prelude::beta_testing::{FluidArray, HeatTransferEntity, HeatTransferInteractionType, LiquidMaterial};
    use uom::ConstZero;

    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::si::time::second;

    // pri flowrate (shell side)
    // shell side flows in forward direction
    let experimental_pri_mass_flowrate = 
        MassRate::new::<kilogram_per_second>(
            experimental_pri_shell_side_mass_flowrate_kg_per_s_abs);
    // dracs flowrate (tube side)
    // tube side flows in reverse direction
    // (need to negative)
    let experimental_dracs_mass_flowrate = 
        MassRate::new::<kilogram_per_second>(
            -experimental_dracs_tube_side_mass_flowrate_kg_per_s_abs);


    let dhx_shell_side_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(
            dhx_shell_side_inlet_temp_degc);

    let dhx_tube_side_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(
            dhx_tube_side_inlet_temp_degc);

    let dhx_shell_side_outlet_temperature_set_point = 
        ThermodynamicTemperature::new::<degree_celsius>(
            dhx_shell_side_outlet_temp_set_point_degc);


    // time setitings
    let mut current_simulation_time = Time::ZERO;
    let max_simulation_time = Time::new::<second>(max_time_seconds);
    let timestep = Time::new::<second>(0.5);

    // calibrated thickness settings

    let calibrated_insulation_thickness = 
        Length::new::<centimeter>(insulation_thickness_regression_cm);

    // calibrated nusselt correlation settings (using Gnielinksi correlation)

    let calibrated_nusselt_factor = 
        Ratio::new::<ratio>(shell_side_to_tubes_nusselt_number_correction_factor);

    let calibrated_parasitic_heat_loss_nusselt_factor = 
        Ratio::new::<ratio>(shell_side_to_ambient_nusselt_correction_factor);

    // initial temperature 
    let average_temperature_for_advection_mass_flowrate_calcs = 
        ThermodynamicTemperature::new::<degree_celsius>(
            0.5*(dhx_shell_side_inlet_temp_degc+dhx_shell_side_outlet_temp_set_point_degc)
            );
    let initial_temperature: ThermodynamicTemperature 
        = dhx_shell_side_outlet_temperature_set_point;

    // components from dhx tube top outlet to tchx inlet
    // in dracs loop


    let mut dhx_sthe = new_dhx_sthe_version_1(initial_temperature);
    // create the heat transfer interaction 
    let average_therminol_density = 
        LiquidMaterial::TherminolVP1.try_get_density(
            average_temperature_for_advection_mass_flowrate_calcs).unwrap();

    let shell_side_advection_heat_transfer_interaction: HeatTransferInteractionType
        = HeatTransferInteractionType::
        new_advection_interaction(experimental_pri_mass_flowrate, 
            average_therminol_density, 
            average_therminol_density);
    let tube_side_advection_heat_transfer_interaction: HeatTransferInteractionType
        = HeatTransferInteractionType::
        new_advection_interaction(experimental_dracs_mass_flowrate, 
            average_therminol_density, 
            average_therminol_density);

    // the heater outlet boundary condition
    let mut dhx_shell_inlet_bc = 
        HeatTransferEntity::new_const_temperature_bc(
            dhx_shell_side_inlet_temperature);

    let mut dhx_tube_inlet_bc = 
        HeatTransferEntity::new_const_temperature_bc(
            dhx_tube_side_inlet_temperature);

    let mut dhx_shell_outlet_bc = 
        HeatTransferEntity::new_adiabatic_bc();
    let mut dhx_tube_outlet_bc = 
        HeatTransferEntity::new_adiabatic_bc();

    let mut dhx_shell_outlet_actual_temperature: ThermodynamicTemperature = 
        initial_temperature;
    let mut dhx_tube_outlet_actual_temperature: ThermodynamicTemperature = 
        initial_temperature;

    // calculation loop
    while current_simulation_time < max_simulation_time {

        // for dhx sthe calibration
        // the shell side flow is positive, so take the front single cv 
        // and tube side flow is negative
        // so, for shell temperature take it from the front cv,
        // and for tube temperature, take it from the back cv
        dhx_shell_outlet_actual_temperature = {

            let dhx_shell_fluid_arr_clone: FluidArray = 
                dhx_sthe.shell_side_fluid_array
                .clone()
                .try_into()
                .unwrap();
            // take the front single cv temperature 
            //
            let dhx_shell_fluid_arr_front_single_cv_temperature: ThermodynamicTemperature 
                = dhx_shell_fluid_arr_clone
                .front_single_cv
                .temperature;

            dhx_shell_fluid_arr_front_single_cv_temperature

        };
        dhx_tube_outlet_actual_temperature = {

            let dhx_tube_fluid_arr_clone: FluidArray = 
                dhx_sthe.tube_side_fluid_array_for_single_tube
                .clone()
                .try_into()
                .unwrap();
            // take the front single cv temperature 
            //
            let dhx_tube_fluid_arr_back_single_cv_temperature: ThermodynamicTemperature 
                = dhx_tube_fluid_arr_clone
                .back_single_cv
                .temperature;

            dhx_tube_fluid_arr_back_single_cv_temperature

        };

        let debug_on = false;
        if debug_on {
            dbg!(&(
                    dhx_shell_outlet_actual_temperature.get::<degree_celsius>(),
                    dhx_tube_outlet_actual_temperature.get::<degree_celsius>()
            ));
        }
        
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

        // now calibrate the insulation thickness for all 

        dhx_sthe.calibrate_insulation_thickness(calibrated_insulation_thickness);

        // link the HTEs up

        dhx_sthe.shell_side_fluid_array.link_to_back(
            &mut dhx_shell_inlet_bc, 
            shell_side_advection_heat_transfer_interaction)
            .unwrap();

        dhx_sthe.shell_side_fluid_array.link_to_front(
            &mut dhx_shell_outlet_bc, 
            shell_side_advection_heat_transfer_interaction)
            .unwrap();

        dhx_sthe.tube_side_fluid_array_for_single_tube.link_to_front(
            &mut dhx_tube_inlet_bc, 
            tube_side_advection_heat_transfer_interaction)
            .unwrap();

        dhx_sthe.tube_side_fluid_array_for_single_tube.link_to_back(
            &mut dhx_tube_outlet_bc, 
            tube_side_advection_heat_transfer_interaction)
            .unwrap();

        // lateral_and_miscellaneous_connections
        let correct_prandtl_for_wall_temperatures = true;

        dhx_sthe.lateral_and_miscellaneous_connections(
            correct_prandtl_for_wall_temperatures, 
            experimental_dracs_mass_flowrate, 
            experimental_pri_mass_flowrate)
            .unwrap();

        // advance_timestep for all
        dhx_sthe.advance_timestep(timestep).unwrap();

        current_simulation_time += timestep;
    }


    // after everything, let's dbg the acutal inlet temp of the dhx 
    dbg!(&(
            dhx_shell_side_inlet_temperature.get::<degree_celsius>(),
            dhx_tube_side_inlet_temperature.get::<degree_celsius>(),
            dhx_shell_side_outlet_temp_set_point_degc,
            dhx_shell_outlet_actual_temperature.get::<degree_celsius>(),
            dhx_tube_side_outlet_temp_set_point_degc,
            dhx_tube_outlet_actual_temperature.get::<degree_celsius>(),
            calibrated_insulation_thickness.get::<centimeter>(),
            shell_side_to_tubes_nusselt_number_correction_factor
            )
        );
    // for calibration, calibrate tube side first, 
    // then shell side,

    // for tube,
    // check if set point and actual temperature are within 0.5 K of 
    // each other
    // in this test, it could not be achieved
    // debug shell side dimensionless numbers

    let shell_side_reynolds = dhx_sthe.reynolds_shell_side();
    let shell_side_bulk_prandtl = dhx_sthe.bulk_prandtl_number_shell_side();
    let inner_tube_wall_prandtl = dhx_sthe.wall_prandtl_number_shell_side_fluid_for_inner_tube();
    let nusselt_number_shell_side_to_tubes = dhx_sthe.nusselt_number_shell_side_to_tubes();

    dbg!(&(
            shell_side_reynolds,
            shell_side_bulk_prandtl,
            inner_tube_wall_prandtl,
            nusselt_number_shell_side_to_tubes
    )
    );
    let outer_tube_wall_prandtl = dhx_sthe.wall_prandtl_number_shell_side_fluid_for_outer_tube();
    let nusselt_number_shell_side_parasitic = dhx_sthe.nusselt_number_shell_side_parasitic();

    dbg!(&(
            shell_side_reynolds,
            shell_side_bulk_prandtl,
            outer_tube_wall_prandtl,
            nusselt_number_shell_side_parasitic
    )
    );
    let correct_prandtl_for_wall_temperatures = true;
    // thermal resistance assertion for shell to tubes 
    let overall_htc_postprocessing: HeatTransfer = 
        dhx_sthe.overall_htc_based_on_conductance(
            correct_prandtl_for_wall_temperatures,
            experimental_dracs_mass_flowrate,
            experimental_pri_mass_flowrate);

    let shell_side_heat_transfer_area: Area = 
        dhx_sthe.circular_tube_bundle_heat_transfer_area_shell_side();

    let thermal_resistance_overall_expected: ThermalResistance
        = (overall_htc_postprocessing*shell_side_heat_transfer_area).recip();

    let thermal_resistance_overall_actual: ThermalResistance = 
        dhx_sthe.get_shell_side_convective_thermal_resistance_cylindrical()
        + dhx_sthe.get_inner_tubes_convective_thermal_resistance_based_on_wetted_perimeter()
        + dhx_sthe.get_inner_tubes_cylindrical_thermal_resistance();


    dbg!(&(thermal_resistance_overall_expected,
            thermal_resistance_overall_actual));

    // assert shell to tube side thermal resistance

    approx::assert_relative_eq!(
        thermal_resistance_overall_expected.get::<kelvin_per_watt>(),
        thermal_resistance_overall_actual.get::<kelvin_per_watt>(),
        max_relative = 0.01
        );



    // assert nusselt number
    let tube_inlet_temperature = dhx_tube_side_inlet_temperature;
    let tube_outlet_temeprature = dhx_tube_outlet_actual_temperature;
    let tube_mass_flowrate = experimental_dracs_mass_flowrate;
    let shell_inlet_temperature = dhx_shell_side_inlet_temperature;
    let shell_outlet_temeprature = dhx_shell_outlet_actual_temperature;
    let shell_mass_flowrate = experimental_pri_mass_flowrate;
    let is_counter_current = true;
    // assert overall thermal resistance 
    // vs that gained from expt data
    let thermal_resistance_based_on_expt_data: ThermalResistance = 
        dhx_sthe.get_ua_based_on_mass_flowrates_and_temperature_differences(
            tube_inlet_temperature, 
            tube_outlet_temeprature, 
            tube_mass_flowrate, 
            shell_inlet_temperature, 
            shell_outlet_temeprature, 
            shell_mass_flowrate, 
            is_counter_current).recip();

    // should agree within 8%
    approx::assert_relative_eq!(
        thermal_resistance_overall_expected.get::<kelvin_per_watt>(),
        thermal_resistance_based_on_expt_data.get::<kelvin_per_watt>(),
        max_relative = 0.08
        );

    // assert shell side nusselt number 
    let nusselt_number_shell_side_to_tubes_actual =
        dhx_sthe.obtain_shell_side_nusselt_number_based_on_expt_data(
            tube_inlet_temperature, 
            tube_outlet_temeprature, 
            tube_mass_flowrate, 
            shell_inlet_temperature, 
            shell_outlet_temeprature, 
            shell_mass_flowrate, 
            is_counter_current);

    // these two should agree to within 10%
    approx::assert_relative_eq!(
        nusselt_number_shell_side_to_tubes.get::<ratio>(),
        nusselt_number_shell_side_to_tubes_actual.get::<ratio>(),
        max_relative = 0.10
        );

    // thermal resistance for parasitic heat loss
    // should agree to the experimental thermal resistance to within 
    // 10%
    let thermal_resistance_for_parasitic_heat_loss: ThermalResistance;

    let thermal_resistance_for_shell_side_parasitic = 
        dhx_sthe.get_shell_side_parasitic_convective_thermal_resistance_cylindrical();
    let convective_thermal_resistance_to_ambient = 
        dhx_sthe.get_convective_thermal_resistance_to_ambient();
    let outer_shell_cylindrical_thermal_resistance =
        dhx_sthe.get_outer_shell_cylindrical_thermal_resistance();
    let insulation_cylindrical_thermal_resistance = 
        dhx_sthe.try_get_insulation_cylindrical_thermal_resistance().unwrap();


    // i'm not 100% sure what is wrong here.

    thermal_resistance_for_parasitic_heat_loss = 
        convective_thermal_resistance_to_ambient
        + outer_shell_cylindrical_thermal_resistance
        + insulation_cylindrical_thermal_resistance
        + thermal_resistance_for_shell_side_parasitic;

    
    let thermal_resistance_for_parasitic_heat_loss_experimental 
        = dhx_sthe.obtain_parasitic_thermal_resistance_based_on_expt_data(
            tube_inlet_temperature, 
            tube_outlet_temeprature, 
            tube_mass_flowrate, 
            shell_inlet_temperature, 
            shell_outlet_temeprature, 
            shell_mass_flowrate);

    dbg!(&(
            thermal_resistance_for_shell_side_parasitic,
            outer_shell_cylindrical_thermal_resistance,
            insulation_cylindrical_thermal_resistance,
            convective_thermal_resistance_to_ambient,
            thermal_resistance_for_parasitic_heat_loss,
            thermal_resistance_for_parasitic_heat_loss_experimental
    ));

    let parasitic_loss_rate: Power = 
        dhx_sthe.obtain_parasitic_heat_loss_rate_based_on_expt_data(
            tube_inlet_temperature, 
            tube_outlet_temeprature, 
            tube_mass_flowrate, 
            shell_inlet_temperature, 
            shell_outlet_temeprature, 
            shell_mass_flowrate);

    let tube_side_heat_transfer_rate: Power = 
        dhx_sthe.get_tube_side_heat_rate_based_on_mass_flowrate(
            tube_inlet_temperature, 
            tube_outlet_temeprature, 
            tube_mass_flowrate);

    let shell_side_heat_transfer_rate: Power = 
        dhx_sthe.get_shell_side_heat_rate_based_on_mass_flowrate(
            shell_inlet_temperature, 
            shell_outlet_temeprature, 
            shell_mass_flowrate);

    dbg!(&(
            parasitic_loss_rate,
            tube_side_heat_transfer_rate,
            shell_side_heat_transfer_rate
    ));

    // these two agree to within 20%
    approx::assert_relative_eq!(
        thermal_resistance_for_parasitic_heat_loss.get::<kelvin_per_watt>(),
        thermal_resistance_for_parasitic_heat_loss_experimental.get::<kelvin_per_watt>(),
        max_relative = 0.20
        );

    // thermal resistance for shell side compared 


    let thermal_resistance_for_shell_side_parasitic_actual = 
        thermal_resistance_for_parasitic_heat_loss_experimental 
        - convective_thermal_resistance_to_ambient
        - outer_shell_cylindrical_thermal_resistance
        - insulation_cylindrical_thermal_resistance;

    // there is like a 90% difference though the thermal resistances 
    // to surrounding is ok

    approx::assert_relative_eq!(
        thermal_resistance_for_shell_side_parasitic.get::<kelvin_per_watt>(),
        thermal_resistance_for_shell_side_parasitic_actual.get::<kelvin_per_watt>(),
        max_relative = 0.95
        );

    // parasitic heat loss nusselt number

    let nusselt_number_shell_side_parasitic_actual = 
        dhx_sthe.obtain_parasitic_nusselt_number_based_on_expt_data(
            tube_inlet_temperature, 
            tube_outlet_temeprature, 
            tube_mass_flowrate, 
            shell_inlet_temperature, 
            shell_outlet_temeprature, 
            shell_mass_flowrate);

    approx::assert_relative_eq!(
        nusselt_number_shell_side_parasitic.get::<ratio>(),
        nusselt_number_shell_side_parasitic_actual.get::<ratio>(),
        max_relative = 0.95
        );

    todo!();
}


