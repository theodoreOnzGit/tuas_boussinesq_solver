/// test series took about 230 s
/// c1
/// Version 4 increases the K of pipe 22 to 45.95
/// Version 5 increases nusselt number of heater 5 times (deprecated now tho)
///
///
///
/// in this module, I want to calibrate the heater nusselt 
/// number to a suitable value
/// dataset number,pri loop mass flowrate (kg/s),Heater (heat addition),Heater inlet (DegC),Average Surface T
/// C-1,0.02003,841.01916,50.5,86.80711,
/// C-2,0.02367,1158.68584,53.8,96.92176,
/// C-3,0.02635,1409.2231,56.7,105.23976,
/// C-4,0.02949,1736.10797,60.8,114.57434,
/// C-5,0.0319,2026.28588,64.1,122.82384,
/// C-6,0.03412,2288.8349,67.2,130.37845,
/// C-7,0.03562,2508.71169,70.6,138.12225,
/// C-8,0.03593,2685.83341,73.6,145.79877,
/// C-9,0.03547,2764.52664,76.5,153.29145,
///
/// This version attempts to calibrate the nusselt number of the heater 
/// such that the surface temperature of the heater matches that of the 
/// experimental data. Moreover, the heat transfer to ambient is now 
/// set to zero
///
/// in SAM, the heat was added directly to the fluid. This is because 
/// the heat addition values were back calculated from thermocouple data 
/// so that the heat addition was a net to the fluid. Therefore, heater 
/// surface temperatures were not even considered. If the surface temperatures 
/// were simulated, then they would be lower than that of the fluid.
///
/// Here, I wanted to add a little realism. While there is zero heat loss 
/// to the environment, the heat is added to the metal rather than to 
/// the fluid. Hence, the metallic heater surface temperatures are elevated 
/// to somewhat be closer to experimental data. This would be higher than 
/// that of adding heat directly to the fluid. In this case, even a heater 
/// surface temperature 10 K lower than that of experimental data would be 
/// an improvement over the SAM model. Therefore, low temperature bounds 
/// are still acceptable.
///
/// Since the increasing the Nusselt number between shell and fluid 
/// decreases heater surface temperature, I'll just use a high bound
/// Nusselt number from this set because it is already a correction to 
/// a non-existent or relatively low heater surface temperature from SAM.
///
#[test] 
#[ignore = "legacy test used for debugging"]
pub fn regression_long_test_calibrated_ver6_set_c1(){

    let max_simulation_time_seconds: f64 = 6300.0;
    let pri_loop_relative_tolerance = 0.062;
    let dracs_loop_relative_tolerance = 0.061;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (841.02, 40.0, 2.6860e-2, 2.0030e-2, 2.5263e-2, 2.0594e-2);


    let (shell_side_to_tubes_nusselt_number_correction_factor,
        insulation_thickness_regression_cm,
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
        (1.08,86.80711,86.49,0.5);


    regression_coupled_dracs_loop_version_6(
        heater_power_watts, 
        max_simulation_time_seconds,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s,
        pri_loop_relative_tolerance,
        dracs_loop_relative_tolerance,
        shell_side_to_tubes_nusselt_number_correction_factor,
        insulation_thickness_regression_cm,
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
    ).unwrap();


}
/// test series took about 230 s
/// c2
/// Version 4 increases the K of pipe 22 to 45.95
/// Version 5 increases nusselt number of heater 5 times (deprecated now tho)
///
///
///
/// in this module, I want to calibrate the heater nusselt 
/// number to a suitable value
/// dataset number,pri loop mass flowrate (kg/s),Heater (heat addition),Heater inlet (DegC),Average Surface T
/// C-1,0.02003,841.01916,50.5,86.80711,
/// C-2,0.02367,1158.68584,53.8,96.92176,
/// C-3,0.02635,1409.2231,56.7,105.23976,
/// C-4,0.02949,1736.10797,60.8,114.57434,
/// C-5,0.0319,2026.28588,64.1,122.82384,
/// C-6,0.03412,2288.8349,67.2,130.37845,
/// C-7,0.03562,2508.71169,70.6,138.12225,
/// C-8,0.03593,2685.83341,73.6,145.79877,
/// C-9,0.03547,2764.52664,76.5,153.29145,
///
/// This version attempts to calibrate the nusselt number of the heater 
/// such that the surface temperature of the heater matches that of the 
/// experimental data. Moreover, the heat transfer to ambient is now 
/// set to zero
///
/// in SAM, the heat was added directly to the fluid. This is because 
/// the heat addition values were back calculated from thermocouple data 
/// so that the heat addition was a net to the fluid. Therefore, heater 
/// surface temperatures were not even considered. If the surface temperatures 
/// were simulated, then they would be lower than that of the fluid.
///
/// Here, I wanted to add a little realism. While there is zero heat loss 
/// to the environment, the heat is added to the metal rather than to 
/// the fluid. Hence, the metallic heater surface temperatures are elevated 
/// to somewhat be closer to experimental data. This would be higher than 
/// that of adding heat directly to the fluid. In this case, even a heater 
/// surface temperature 10 K lower than that of experimental data would be 
/// an improvement over the SAM model. Therefore, low temperature bounds 
/// are still acceptable.
///
/// Since the increasing the Nusselt number between shell and fluid 
/// decreases heater surface temperature, I'll just use a high bound
/// Nusselt number from this set because it is already a correction to 
/// a non-existent or relatively low heater surface temperature from SAM.
///
#[test] 
#[ignore = "legacy test used for debugging"]
pub fn regression_long_test_calibrated_ver6_set_c2(){

    let max_simulation_time_seconds: f64 = 6300.0;
    let pri_loop_relative_tolerance = 0.062;
    let dracs_loop_relative_tolerance = 0.061;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (1158.69, 40.0, 3.0550e-2, 2.3670e-2, 3.0377e-2, 2.4344e-2);


    let (shell_side_to_tubes_nusselt_number_correction_factor,
        insulation_thickness_regression_cm,
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
        (1.27,96.92,96.65,0.5);


    regression_coupled_dracs_loop_version_6(
        heater_power_watts, 
        max_simulation_time_seconds,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s,
        pri_loop_relative_tolerance,
        dracs_loop_relative_tolerance,
        shell_side_to_tubes_nusselt_number_correction_factor,
        insulation_thickness_regression_cm,
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
    ).unwrap();


}

/// test series took about 230 s
/// c3
/// Version 4 increases the K of pipe 22 to 45.95
/// Version 5 increases nusselt number of heater 5 times (deprecated now tho)
///
///
///
/// in this module, I want to calibrate the heater nusselt 
/// number to a suitable value
/// dataset number,pri loop mass flowrate (kg/s),Heater (heat addition),Heater inlet (DegC),Average Surface T
/// C-1,0.02003,841.01916,50.5,86.80711,
/// C-2,0.02367,1158.68584,53.8,96.92176,
/// C-3,0.02635,1409.2231,56.7,105.23976,
/// C-4,0.02949,1736.10797,60.8,114.57434,
/// C-5,0.0319,2026.28588,64.1,122.82384,
/// C-6,0.03412,2288.8349,67.2,130.37845,
/// C-7,0.03562,2508.71169,70.6,138.12225,
/// C-8,0.03593,2685.83341,73.6,145.79877,
/// C-9,0.03547,2764.52664,76.5,153.29145,
///
/// This version attempts to calibrate the nusselt number of the heater 
/// such that the surface temperature of the heater matches that of the 
/// experimental data. Moreover, the heat transfer to ambient is now 
/// set to zero
///
/// in SAM, the heat was added directly to the fluid. This is because 
/// the heat addition values were back calculated from thermocouple data 
/// so that the heat addition was a net to the fluid. Therefore, heater 
/// surface temperatures were not even considered. If the surface temperatures 
/// were simulated, then they would be lower than that of the fluid.
///
/// Here, I wanted to add a little realism. While there is zero heat loss 
/// to the environment, the heat is added to the metal rather than to 
/// the fluid. Hence, the metallic heater surface temperatures are elevated 
/// to somewhat be closer to experimental data. This would be higher than 
/// that of adding heat directly to the fluid. In this case, even a heater 
/// surface temperature 10 K lower than that of experimental data would be 
/// an improvement over the SAM model. Therefore, low temperature bounds 
/// are still acceptable.
///
/// Since the increasing the Nusselt number between shell and fluid 
/// decreases heater surface temperature, I'll just use a high bound
/// Nusselt number from this set because it is already a correction to 
/// a non-existent or relatively low heater surface temperature from SAM.
///
#[test] 
#[ignore = "legacy test used for debugging"]
pub fn regression_long_test_calibrated_ver6_set_c3(){

    let max_simulation_time_seconds: f64 = 6300.0;
    let pri_loop_relative_tolerance = 0.062;
    let dracs_loop_relative_tolerance = 0.061;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (1409.22, 40.0, 3.3450e-2, 2.6350e-2, 3.3650e-2, 2.6887e-2);


    let (shell_side_to_tubes_nusselt_number_correction_factor,
        insulation_thickness_regression_cm,
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
        (1.38,105.23,104.96,0.5);


    regression_coupled_dracs_loop_version_6(
        heater_power_watts, 
        max_simulation_time_seconds,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s,
        pri_loop_relative_tolerance,
        dracs_loop_relative_tolerance,
        shell_side_to_tubes_nusselt_number_correction_factor,
        insulation_thickness_regression_cm,
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
    ).unwrap();


}

/// test series took about 230 s
/// c4
/// Version 4 increases the K of pipe 22 to 45.95
/// Version 5 increases nusselt number of heater 5 times (deprecated now tho)
///
///
///
/// in this module, I want to calibrate the heater nusselt 
/// number to a suitable value
/// dataset number,pri loop mass flowrate (kg/s),Heater (heat addition),Heater inlet (DegC),Average Surface T
/// C-1,0.02003,841.01916,50.5,86.80711,
/// C-2,0.02367,1158.68584,53.8,96.92176,
/// C-3,0.02635,1409.2231,56.7,105.23976,
/// C-4,0.02949,1736.10797,60.8,114.57434,
/// C-5,0.0319,2026.28588,64.1,122.82384,
/// C-6,0.03412,2288.8349,67.2,130.37845,
/// C-7,0.03562,2508.71169,70.6,138.12225,
/// C-8,0.03593,2685.83341,73.6,145.79877,
/// C-9,0.03547,2764.52664,76.5,153.29145,
///
/// This version attempts to calibrate the nusselt number of the heater 
/// such that the surface temperature of the heater matches that of the 
/// experimental data. Moreover, the heat transfer to ambient is now 
/// set to zero
///
/// in SAM, the heat was added directly to the fluid. This is because 
/// the heat addition values were back calculated from thermocouple data 
/// so that the heat addition was a net to the fluid. Therefore, heater 
/// surface temperatures were not even considered. If the surface temperatures 
/// were simulated, then they would be lower than that of the fluid.
///
/// Here, I wanted to add a little realism. While there is zero heat loss 
/// to the environment, the heat is added to the metal rather than to 
/// the fluid. Hence, the metallic heater surface temperatures are elevated 
/// to somewhat be closer to experimental data. This would be higher than 
/// that of adding heat directly to the fluid. In this case, even a heater 
/// surface temperature 10 K lower than that of experimental data would be 
/// an improvement over the SAM model. Therefore, low temperature bounds 
/// are still acceptable.
///
/// Since the increasing the Nusselt number between shell and fluid 
/// decreases heater surface temperature, I'll just use a high bound
/// Nusselt number from this set because it is already a correction to 
/// a non-existent or relatively low heater surface temperature from SAM.
///
#[test] 
#[ignore = "legacy test used for debugging"]
pub fn regression_long_test_calibrated_ver6_set_c4(){

    let max_simulation_time_seconds: f64 = 6300.0;
    let pri_loop_relative_tolerance = 0.062;
    let dracs_loop_relative_tolerance = 0.061;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (1736.11, 40.0, 3.6490e-2, 2.9490e-2, 3.7349e-2, 2.9804e-2);


    let (shell_side_to_tubes_nusselt_number_correction_factor,
        insulation_thickness_regression_cm,
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
        (1.53,114.57,114.76,0.5);


    regression_coupled_dracs_loop_version_6(
        heater_power_watts, 
        max_simulation_time_seconds,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s,
        pri_loop_relative_tolerance,
        dracs_loop_relative_tolerance,
        shell_side_to_tubes_nusselt_number_correction_factor,
        insulation_thickness_regression_cm,
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
    ).unwrap();


}

/// test series took about 230 s
/// c5
/// Version 4 increases the K of pipe 22 to 45.95
/// Version 5 increases nusselt number of heater 5 times (deprecated now tho)
///
///
///
/// in this module, I want to calibrate the heater nusselt 
/// number to a suitable value
/// dataset number,pri loop mass flowrate (kg/s),Heater (heat addition),Heater inlet (DegC),Average Surface T
/// C-1,0.02003,841.01916,50.5,86.80711,
/// C-2,0.02367,1158.68584,53.8,96.92176,
/// C-3,0.02635,1409.2231,56.7,105.23976,
/// C-4,0.02949,1736.10797,60.8,114.57434,
/// C-5,0.0319,2026.28588,64.1,122.82384,
/// C-6,0.03412,2288.8349,67.2,130.37845,
/// C-7,0.03562,2508.71169,70.6,138.12225,
/// C-8,0.03593,2685.83341,73.6,145.79877,
/// C-9,0.03547,2764.52664,76.5,153.29145,
///
/// This version attempts to calibrate the nusselt number of the heater 
/// such that the surface temperature of the heater matches that of the 
/// experimental data. Moreover, the heat transfer to ambient is now 
/// set to zero
///
/// in SAM, the heat was added directly to the fluid. This is because 
/// the heat addition values were back calculated from thermocouple data 
/// so that the heat addition was a net to the fluid. Therefore, heater 
/// surface temperatures were not even considered. If the surface temperatures 
/// were simulated, then they would be lower than that of the fluid.
///
/// Here, I wanted to add a little realism. While there is zero heat loss 
/// to the environment, the heat is added to the metal rather than to 
/// the fluid. Hence, the metallic heater surface temperatures are elevated 
/// to somewhat be closer to experimental data. This would be higher than 
/// that of adding heat directly to the fluid. In this case, even a heater 
/// surface temperature 10 K lower than that of experimental data would be 
/// an improvement over the SAM model. Therefore, low temperature bounds 
/// are still acceptable.
///
/// Since the increasing the Nusselt number between shell and fluid 
/// decreases heater surface temperature, I'll just use a high bound
/// Nusselt number from this set because it is already a correction to 
/// a non-existent or relatively low heater surface temperature from SAM.
///
#[test] 
#[ignore = "legacy test used for debugging"]
pub fn regression_long_test_calibrated_ver6_set_c5(){

    let max_simulation_time_seconds: f64 = 6300.0;
    let pri_loop_relative_tolerance = 0.062;
    let dracs_loop_relative_tolerance = 0.061;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (2026.29, 40.0, 3.8690e-2, 3.1900e-2, 4.0261e-2, 3.2088e-2);


    let (shell_side_to_tubes_nusselt_number_correction_factor,
        insulation_thickness_regression_cm,
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
        (1.65,122.82,123.19,0.5);


    regression_coupled_dracs_loop_version_6(
        heater_power_watts, 
        max_simulation_time_seconds,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s,
        pri_loop_relative_tolerance,
        dracs_loop_relative_tolerance,
        shell_side_to_tubes_nusselt_number_correction_factor,
        insulation_thickness_regression_cm,
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
    ).unwrap();


}

/// test series took about 230 s
/// c6
/// Version 4 increases the K of pipe 22 to 45.95
/// Version 5 increases nusselt number of heater 5 times (deprecated now tho)
///
///
///
/// in this module, I want to calibrate the heater nusselt 
/// number to a suitable value
/// dataset number,pri loop mass flowrate (kg/s),Heater (heat addition),Heater inlet (DegC),Average Surface T
/// C-1,0.02003,841.01916,50.5,86.80711,
/// C-2,0.02367,1158.68584,53.8,96.92176,
/// C-3,0.02635,1409.2231,56.7,105.23976,
/// C-4,0.02949,1736.10797,60.8,114.57434,
/// C-5,0.0319,2026.28588,64.1,122.82384,
/// C-6,0.03412,2288.8349,67.2,130.37845,
/// C-7,0.03562,2508.71169,70.6,138.12225,
/// C-8,0.03593,2685.83341,73.6,145.79877,
/// C-9,0.03547,2764.52664,76.5,153.29145,
///
/// This version attempts to calibrate the nusselt number of the heater 
/// such that the surface temperature of the heater matches that of the 
/// experimental data. Moreover, the heat transfer to ambient is now 
/// set to zero
///
/// in SAM, the heat was added directly to the fluid. This is because 
/// the heat addition values were back calculated from thermocouple data 
/// so that the heat addition was a net to the fluid. Therefore, heater 
/// surface temperatures were not even considered. If the surface temperatures 
/// were simulated, then they would be lower than that of the fluid.
///
/// Here, I wanted to add a little realism. While there is zero heat loss 
/// to the environment, the heat is added to the metal rather than to 
/// the fluid. Hence, the metallic heater surface temperatures are elevated 
/// to somewhat be closer to experimental data. This would be higher than 
/// that of adding heat directly to the fluid. In this case, even a heater 
/// surface temperature 10 K lower than that of experimental data would be 
/// an improvement over the SAM model. Therefore, low temperature bounds 
/// are still acceptable.
///
/// Since the increasing the Nusselt number between shell and fluid 
/// decreases heater surface temperature, I'll just use a high bound
/// Nusselt number from this set because it is already a correction to 
/// a non-existent or relatively low heater surface temperature from SAM.
///
#[test] 
#[ignore = "legacy test used for debugging"]
pub fn regression_long_test_calibrated_ver6_set_c6(){

    let max_simulation_time_seconds: f64 = 6300.0;
    let pri_loop_relative_tolerance = 0.062;
    let dracs_loop_relative_tolerance = 0.061;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (2288.83, 40.0, 4.1150e-2, 3.4120e-2, 4.2673e-2, 3.3956e-2);


    let (shell_side_to_tubes_nusselt_number_correction_factor,
        insulation_thickness_regression_cm,
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
        (1.77,130.37,130.07,0.5);


    regression_coupled_dracs_loop_version_6(
        heater_power_watts, 
        max_simulation_time_seconds,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s,
        pri_loop_relative_tolerance,
        dracs_loop_relative_tolerance,
        shell_side_to_tubes_nusselt_number_correction_factor,
        insulation_thickness_regression_cm,
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
    ).unwrap();


}

/// test series took about 230 s
/// c7
/// Version 4 increases the K of pipe 22 to 45.95
/// Version 5 increases nusselt number of heater 5 times (deprecated now tho)
///
///
///
/// in this module, I want to calibrate the heater nusselt 
/// number to a suitable value
/// dataset number,pri loop mass flowrate (kg/s),Heater (heat addition),Heater inlet (DegC),Average Surface T
/// C-1,0.02003,841.01916,50.5,86.80711,
/// C-2,0.02367,1158.68584,53.8,96.92176,
/// C-3,0.02635,1409.2231,56.7,105.23976,
/// C-4,0.02949,1736.10797,60.8,114.57434,
/// C-5,0.0319,2026.28588,64.1,122.82384,
/// C-6,0.03412,2288.8349,67.2,130.37845,
/// C-7,0.03562,2508.71169,70.6,138.12225,
/// C-8,0.03593,2685.83341,73.6,145.79877,
/// C-9,0.03547,2764.52664,76.5,153.29145,
///
/// This version attempts to calibrate the nusselt number of the heater 
/// such that the surface temperature of the heater matches that of the 
/// experimental data. Moreover, the heat transfer to ambient is now 
/// set to zero
///
/// in SAM, the heat was added directly to the fluid. This is because 
/// the heat addition values were back calculated from thermocouple data 
/// so that the heat addition was a net to the fluid. Therefore, heater 
/// surface temperatures were not even considered. If the surface temperatures 
/// were simulated, then they would be lower than that of the fluid.
///
/// Here, I wanted to add a little realism. While there is zero heat loss 
/// to the environment, the heat is added to the metal rather than to 
/// the fluid. Hence, the metallic heater surface temperatures are elevated 
/// to somewhat be closer to experimental data. This would be higher than 
/// that of adding heat directly to the fluid. In this case, even a heater 
/// surface temperature 10 K lower than that of experimental data would be 
/// an improvement over the SAM model. Therefore, low temperature bounds 
/// are still acceptable.
///
/// Since the increasing the Nusselt number between shell and fluid 
/// decreases heater surface temperature, I'll just use a high bound
/// Nusselt number from this set because it is already a correction to 
/// a non-existent or relatively low heater surface temperature from SAM.
///
#[test] 
#[ignore = "legacy test used for debugging"]
pub fn regression_long_test_calibrated_ver6_set_c7(){

    let max_simulation_time_seconds: f64 = 6300.0;
    let pri_loop_relative_tolerance = 0.062;
    let dracs_loop_relative_tolerance = 0.061;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (2508.71, 40.0, 4.3120e-2, 3.5620e-2, 4.4554e-2, 3.5391e-2);


    let (shell_side_to_tubes_nusselt_number_correction_factor,
        insulation_thickness_regression_cm,
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
        (1.78,138.12,138.10,0.5);


    regression_coupled_dracs_loop_version_6(
        heater_power_watts, 
        max_simulation_time_seconds,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s,
        pri_loop_relative_tolerance,
        dracs_loop_relative_tolerance,
        shell_side_to_tubes_nusselt_number_correction_factor,
        insulation_thickness_regression_cm,
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
    ).unwrap();


}

/// test series took about 230 s
/// c8
/// Version 4 increases the K of pipe 22 to 45.95
/// Version 5 increases nusselt number of heater 5 times (deprecated now tho)
///
///
///
/// in this module, I want to calibrate the heater nusselt 
/// number to a suitable value
/// dataset number,pri loop mass flowrate (kg/s),Heater (heat addition),Heater inlet (DegC),Average Surface T
/// C-1,0.02003,841.01916,50.5,86.80711,
/// C-2,0.02367,1158.68584,53.8,96.92176,
/// C-3,0.02635,1409.2231,56.7,105.23976,
/// C-4,0.02949,1736.10797,60.8,114.57434,
/// C-5,0.0319,2026.28588,64.1,122.82384,
/// C-6,0.03412,2288.8349,67.2,130.37845,
/// C-7,0.03562,2508.71169,70.6,138.12225,
/// C-8,0.03593,2685.83341,73.6,145.79877,
/// C-9,0.03547,2764.52664,76.5,153.29145,
///
/// This version attempts to calibrate the nusselt number of the heater 
/// such that the surface temperature of the heater matches that of the 
/// experimental data. Moreover, the heat transfer to ambient is now 
/// set to zero
///
/// in SAM, the heat was added directly to the fluid. This is because 
/// the heat addition values were back calculated from thermocouple data 
/// so that the heat addition was a net to the fluid. Therefore, heater 
/// surface temperatures were not even considered. If the surface temperatures 
/// were simulated, then they would be lower than that of the fluid.
///
/// Here, I wanted to add a little realism. While there is zero heat loss 
/// to the environment, the heat is added to the metal rather than to 
/// the fluid. Hence, the metallic heater surface temperatures are elevated 
/// to somewhat be closer to experimental data. This would be higher than 
/// that of adding heat directly to the fluid. In this case, even a heater 
/// surface temperature 10 K lower than that of experimental data would be 
/// an improvement over the SAM model. Therefore, low temperature bounds 
/// are still acceptable.
///
/// Since the increasing the Nusselt number between shell and fluid 
/// decreases heater surface temperature, I'll just use a high bound
/// Nusselt number from this set because it is already a correction to 
/// a non-existent or relatively low heater surface temperature from SAM.
///
#[test] 
#[ignore = "legacy test used for debugging"]
pub fn regression_long_test_calibrated_ver6_set_c8(){

    let max_simulation_time_seconds: f64 = 6300.0;
    let pri_loop_relative_tolerance = 0.062;
    let dracs_loop_relative_tolerance = 0.061;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (2685.83, 40.0, 4.5090e-2, 3.5930e-2, 4.5990e-2, 3.6472e-2);


    let (shell_side_to_tubes_nusselt_number_correction_factor,
        insulation_thickness_regression_cm,
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
        (1.75,145.79,145.69,0.5);


    regression_coupled_dracs_loop_version_6(
        heater_power_watts, 
        max_simulation_time_seconds,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s,
        pri_loop_relative_tolerance,
        dracs_loop_relative_tolerance,
        shell_side_to_tubes_nusselt_number_correction_factor,
        insulation_thickness_regression_cm,
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
    ).unwrap();


}

/// test series took about 230 s
/// c9
/// Version 4 increases the K of pipe 22 to 45.95
/// Version 5 increases nusselt number of heater 5 times (deprecated now tho)
///
///
///
/// in this module, I want to calibrate the heater nusselt 
/// number to a suitable value
/// dataset number,pri loop mass flowrate (kg/s),Heater (heat addition),Heater inlet (DegC),Average Surface T
/// C-1,0.02003,841.01916,50.5,86.80711,
/// C-2,0.02367,1158.68584,53.8,96.92176,
/// C-3,0.02635,1409.2231,56.7,105.23976,
/// C-4,0.02949,1736.10797,60.8,114.57434,
/// C-5,0.0319,2026.28588,64.1,122.82384,
/// C-6,0.03412,2288.8349,67.2,130.37845,
/// C-7,0.03562,2508.71169,70.6,138.12225,
/// C-8,0.03593,2685.83341,73.6,145.79877,
/// C-9,0.03547,2764.52664,76.5,153.29145,
///
/// This version attempts to calibrate the nusselt number of the heater 
/// such that the surface temperature of the heater matches that of the 
/// experimental data. Moreover, the heat transfer to ambient is now 
/// set to zero
///
/// in SAM, the heat was added directly to the fluid. This is because 
/// the heat addition values were back calculated from thermocouple data 
/// so that the heat addition was a net to the fluid. Therefore, heater 
/// surface temperatures were not even considered. If the surface temperatures 
/// were simulated, then they would be lower than that of the fluid.
///
/// Here, I wanted to add a little realism. While there is zero heat loss 
/// to the environment, the heat is added to the metal rather than to 
/// the fluid. Hence, the metallic heater surface temperatures are elevated 
/// to somewhat be closer to experimental data. This would be higher than 
/// that of adding heat directly to the fluid. In this case, even a heater 
/// surface temperature 10 K lower than that of experimental data would be 
/// an improvement over the SAM model. Therefore, low temperature bounds 
/// are still acceptable.
///
/// Since the increasing the Nusselt number between shell and fluid 
/// decreases heater surface temperature, I'll just use a high bound
/// Nusselt number from this set because it is already a correction to 
/// a non-existent or relatively low heater surface temperature from SAM.
///
#[test] 
#[ignore = "legacy test used for debugging"]
pub fn regression_long_test_calibrated_ver6_set_c9(){

    let max_simulation_time_seconds: f64 = 6300.0;
    let pri_loop_relative_tolerance = 0.062;
    let dracs_loop_relative_tolerance = 0.061;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (2764.53, 40.0, 4.6990e-2, 3.5470e-2, 4.6596e-2, 3.6925e-2);


    let (shell_side_to_tubes_nusselt_number_correction_factor,
        insulation_thickness_regression_cm,
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
        (1.62,153.29,153.28,0.5);


    regression_coupled_dracs_loop_version_6(
        heater_power_watts, 
        max_simulation_time_seconds,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s,
        pri_loop_relative_tolerance,
        dracs_loop_relative_tolerance,
        shell_side_to_tubes_nusselt_number_correction_factor,
        insulation_thickness_regression_cm,
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
    ).unwrap();


}

/// This version attempts to calibrate the nusselt number of the heater 
/// such that the surface temperature of the heater matches that of the 
/// experimental data. Moreover, the heat transfer to ambient is now 
/// set to zero
///
/// in SAM, the heat was added directly to the fluid. This is because 
/// the heat addition values were back calculated from thermocouple data 
/// so that the heat addition was a net to the fluid. Therefore, heater 
/// surface temperatures were not even considered. If the surface temperatures 
/// were simulated, then they would be lower than that of the fluid.
///
/// Here, I wanted to add a little realism. While there is zero heat loss 
/// to the environment, the heat is added to the metal rather than to 
/// the fluid. Hence, the metallic heater surface temperatures are elevated 
/// to somewhat be closer to experimental data. This would be higher than 
/// that of adding heat directly to the fluid. In this case, even a heater 
/// surface temperature 10 K lower than that of experimental data would be 
/// an improvement over the SAM model. Therefore, low temperature bounds 
/// are still acceptable.
///
/// Since the increasing the Nusselt number between shell and fluid 
/// decreases heater surface temperature, I'll just use a high bound
/// Nusselt number from this set because it is already a correction to 
/// a non-existent or relatively low heater surface temperature from SAM.
#[cfg(test)]
pub fn regression_coupled_dracs_loop_version_6(
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
    heater_surface_temp_tolerance_degc: f64) -> 
Result<(),crate::tuas_lib_error::TuasLibError>{
    use uom::si::length::centimeter;
    use uom::si::{f64::*, mass_rate::kilogram_per_second, power::watt};

    use uom::si::{frequency::hertz, ratio::ratio, time::millisecond};

    use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::pre_built_components::ciet_isothermal_test_components::*;
    use crate::pre_built_components::ciet_steady_state_natural_circulation_test_components::coupled_dracs_loop_tests::dhx_constructor::new_dhx_sthe_version_1;
    use crate::pre_built_components::ciet_steady_state_natural_circulation_test_components::coupled_dracs_loop_tests::dracs_loop_calc_functions_no_tchx_calibration::dracs_loop_dhx_tube_temperature_diagnostics;
    use crate::pre_built_components::ciet_steady_state_natural_circulation_test_components::coupled_dracs_loop_tests::dracs_loop_calc_functions_sam_tchx_calibration::{coupled_dracs_fluid_mechanics_calc_abs_mass_rate_sam_tchx_calibration, coupled_dracs_loop_link_up_components_sam_tchx_calibration, dracs_loop_advance_timestep_except_dhx_sam_tchx_calibration};
    use crate::pre_built_components::ciet_steady_state_natural_circulation_test_components::coupled_dracs_loop_tests::pri_loop_calc_functions::{coupled_dracs_pri_loop_branches_fluid_mechanics_calc_abs_mass_rate, coupled_dracs_pri_loop_dhx_heater_link_up_components, pri_loop_advance_timestep_dhx_br_and_heater_br_except_dhx, pri_loop_dhx_shell_temperature_diagnostics, pri_loop_heater_temperature_diagnostics};
    use crate::pre_built_components::
        ciet_steady_state_natural_circulation_test_components::dracs_loop_components::*;
    use crate::prelude::beta_testing::FluidArray;
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
    let timestep = Time::new::<second>(0.5);
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
    let mut pipe_3 = new_pipe_3_relap_model(initial_temperature);
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



    let mut final_mass_flowrate_pri_loop: MassRate 
        = MassRate::ZERO;
    let mut final_mass_flowrate_dracs_loop: MassRate 
        = MassRate::ZERO;
    let mut _final_tchx_outlet_temperature: ThermodynamicTemperature 
        = ThermodynamicTemperature::ZERO;

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

        if current_simulation_time > tchx_temperature_record_time_threshold {
            _final_tchx_outlet_temperature = tchx_outlet_temperature;
        }

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

        // next, link up the heat transfer entities 
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

        

        // record 
        if current_simulation_time > tchx_temperature_record_time_threshold {
            final_mass_flowrate_dracs_loop = counter_clockwise_dracs_flowrate;
            final_mass_flowrate_pri_loop = counter_clockwise_pri_loop_flowrate;
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

    approx::assert_abs_diff_eq!(
        expt_heater_surf_temp_avg_degc,
        simulated_heater_avg_surf_temp_degc,
        epsilon=heater_surface_temp_tolerance_degc);


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
    


    Ok(())

}
