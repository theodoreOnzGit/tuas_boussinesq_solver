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
pub fn dhx_calibration_validation_test_v1(
    experimental_dracs_tube_side_mass_flowrate_kg_per_s_abs: f64,
    experimental_pri_shell_side_mass_flowrate_kg_per_s_abs: f64,
    dhx_shell_side_inlet_temp_degc: f64,
    dhx_shell_side_outlet_temp_set_point_degc: f64,
    dhx_shell_side_outlet_regression_temperature_degc: f64,
    dhx_tube_side_inlet_temp_degc: f64,
    dhx_tube_side_outlet_temp_set_point_degc: f64,
    dhx_tube_side_outlet_regression_temperature_degc: f64,
    max_time_seconds:f64,
    insulation_thickness_regression_cm: f64,
    shell_side_to_tubes_nusselt_number_correction_factor: f64,
    shell_side_to_ambient_nusselt_correction_factor: f64,){
    use uom::si::length::centimeter;
    use uom::si::ratio::ratio;
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
    let nusselt_number_shell_side_to_outer_shell = dhx_sthe.nusselt_number_shell_side_parasitic();

    dbg!(&(
            shell_side_reynolds,
            shell_side_bulk_prandtl,
            outer_tube_wall_prandtl,
            nusselt_number_shell_side_to_outer_shell
    )
    );

    approx::assert_abs_diff_eq!(
        dhx_tube_side_outlet_temp_set_point_degc,
        dhx_tube_outlet_actual_temperature.get::<degree_celsius>(),
        epsilon=0.5
        );

    // settings for checking parasitic heat loss, mostly to do 
    // with shell side
    // to do after calibrating tube side nusselt numbers
    let check_parasitic_heat_loss = true;
    if check_parasitic_heat_loss {

        // for shell,
        // check if set point and actual temperature are within 0.5 K of 
        // each other
        // in this test, it could not be achieved
        approx::assert_abs_diff_eq!(
            dhx_shell_side_outlet_temperature_set_point.get::<degree_celsius>(),
            dhx_shell_outlet_actual_temperature.get::<degree_celsius>(),
            epsilon=0.5
        );
    }
    // setting for regression testing (after calibration)
    let regression = true; 
    if regression {
        // check if actual temperature is equal to the regression 
        // temperature
        approx::assert_abs_diff_eq!(
            dhx_tube_side_outlet_regression_temperature_degc,
            dhx_tube_outlet_actual_temperature.get::<degree_celsius>(),
            epsilon=0.05
        );
        // check if actual temperature is equal to the regression 
        // temperature
        approx::assert_abs_diff_eq!(
            dhx_shell_side_outlet_regression_temperature_degc,
            dhx_shell_outlet_actual_temperature.get::<degree_celsius>(),
            epsilon=0.05
        );
    }


}
