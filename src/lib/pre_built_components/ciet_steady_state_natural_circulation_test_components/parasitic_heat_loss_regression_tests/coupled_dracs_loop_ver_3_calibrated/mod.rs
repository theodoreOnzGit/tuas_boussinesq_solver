/// c1 for
/// version 3 of coupled DRACS loop 
///
/// for version 3, simple calibration is done as with version 2,
/// but the vertical TCHX is split into two equal halves as was done in SAM,
/// only the bottom half will have the calibrated heat transfer coefficient.
/// The rest of the TCHX, the horizontal TCHX and 35b1, will be insulated.

#[test] 
pub fn regression_long_test_calibrated_ver3_set_c1(){
    use regression_coupled_dracs_loop_version_3::*;

    let max_simulation_time_seconds: f64 = 3800.0;
    // expect overprediction of mass flowrates in both loops 
    // to about 8.5%
    let pri_loop_relative_tolerance = 0.085;
    let dracs_loop_relative_tolerance = 0.085;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (841.02, 40.0, 2.6860e-2, 2.0030e-2, 2.6492e-2, 2.1387e-2);


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

    regression_coupled_dracs_loop_version_3(
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
    ).unwrap();


}
/// c2 for
/// version 3 of coupled DRACS loop 
///
/// for version 3, simple calibration is done as with version 2,
/// but the vertical TCHX is split into two equal halves as was done in SAM,
/// only the bottom half will have the calibrated heat transfer coefficient.
/// The rest of the TCHX, the horizontal TCHX and 35b1, will be insulated.
#[test] 
pub fn regression_long_test_calibrated_ver3_set_c2(){
    use regression_coupled_dracs_loop_version_3::*;

    let max_simulation_time_seconds: f64 = 3800.0;
    // expect overprediction of mass flowrates in both loops 
    // to about 8.5%
    let pri_loop_relative_tolerance = 0.085;
    let dracs_loop_relative_tolerance = 0.085;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (1158.69, 40.0, 3.0550e-2, 2.3670e-2, 3.1173e-2, 2.5320e-2);


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

    regression_coupled_dracs_loop_version_3(
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
    ).unwrap();


}

/// c3 for
/// version 3 of coupled DRACS loop 
///
/// for version 3, simple calibration is done as with version 2,
/// but the vertical TCHX is split into two equal halves as was done in SAM,
/// only the bottom half will have the calibrated heat transfer coefficient.
/// The rest of the TCHX, the horizontal TCHX and 35b1, will be insulated.
#[test] 
pub fn regression_long_test_calibrated_ver3_set_c3(){
    use regression_coupled_dracs_loop_version_3::*;

    let max_simulation_time_seconds: f64 = 3800.0;
    // expect overprediction of mass flowrates in both loops 
    // to about 8.5%
    let pri_loop_relative_tolerance = 0.085;
    let dracs_loop_relative_tolerance = 0.085;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (1409.22, 40.0, 3.3450e-2, 2.6350e-2, 3.4316e-2, 2.8012e-2);


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

    regression_coupled_dracs_loop_version_3(
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
    ).unwrap();


}
/// c4 for
/// version 3 of coupled DRACS loop 
///
/// for version 3, simple calibration is done as with version 2,
/// but the vertical TCHX is split into two equal halves as was done in SAM,
/// only the bottom half will have the calibrated heat transfer coefficient.
/// The rest of the TCHX, the horizontal TCHX and 35b1, will be insulated.
#[test] 
pub fn regression_long_test_calibrated_ver3_set_c4(){
    use regression_coupled_dracs_loop_version_3::*;

    let max_simulation_time_seconds: f64 = 3800.0;
    // expect overprediction of mass flowrates in both loops 
    // to about 8.5%
    let pri_loop_relative_tolerance = 0.085;
    let dracs_loop_relative_tolerance = 0.085;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (1736.11, 40.0, 3.6490e-2, 2.9490e-2, 3.7932e-2, 3.1109e-2);


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

    regression_coupled_dracs_loop_version_3(
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
    ).unwrap();


}

/// c5 for
/// version 3 of coupled DRACS loop 
///
/// for version 3, simple calibration is done as with version 2,
/// but the vertical TCHX is split into two equal halves as was done in SAM,
/// only the bottom half will have the calibrated heat transfer coefficient.
/// The rest of the TCHX, the horizontal TCHX and 35b1, will be insulated.
#[test] 
pub fn regression_long_test_calibrated_ver3_set_c5(){
    use regression_coupled_dracs_loop_version_3::*;

    let max_simulation_time_seconds: f64 = 3800.0;
    // expect overprediction of mass flowrates in both loops 
    // to about 8.5%
    let pri_loop_relative_tolerance = 0.085;
    let dracs_loop_relative_tolerance = 0.085;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (2026.29, 40.0, 3.8690e-2, 3.1900e-2, 4.0802e-2, 3.3544e-2);


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

    regression_coupled_dracs_loop_version_3(
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
    ).unwrap();


}
/// took about 205 s on the i5-13500H 
/// c6 for
/// version 3 of coupled DRACS loop 
///
/// for version 3, simple calibration is done as with version 2,
/// but the vertical TCHX is split into two equal halves as was done in SAM,
/// only the bottom half will have the calibrated heat transfer coefficient.
/// The rest of the TCHX, the horizontal TCHX and 35b1, will be insulated.
#[test] 
pub fn regression_long_test_calibrated_ver3_set_c6_extra_extra_extra_long(){
    use regression_coupled_dracs_loop_version_3::*;

    let max_simulation_time_seconds: f64 = 7000.0;
    // expect overprediction of mass flowrates in both loops 
    // to about 8.5%
    let pri_loop_relative_tolerance = 0.085;
    let dracs_loop_relative_tolerance = 0.085;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    // for 3800s, this is the result:
    // let (heater_power_watts,
    //     tchx_outlet_temp_degc,
    //     experimental_dracs_mass_flowrate_kg_per_s,
    //     experimental_pri_mass_flowrate_kg_per_s,
    //     simulated_expected_dracs_mass_flowrate_kg_per_s,
    //     simulated_expected_pri_mass_flowrate_kg_per_s) 
    // = (2288.83, 40.0, 4.1150e-2, 3.4120e-2, 4.2519e-2, 3.5072e-2);
    // results essentially the same as 6300s, so we'll just use 6300s 
    // as time required for test to reach steady state
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (2288.83, 40.0, 4.1150e-2, 3.4120e-2, 4.2536e-2, 3.5367e-2);


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

    regression_coupled_dracs_loop_version_3(
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
    ).unwrap();


}
/// took about 155 s on the i5-13500H 
#[test] 
pub fn regression_long_test_calibrated_ver3_set_c6_extra_extra_long(){
    use regression_coupled_dracs_loop_version_3::*;

    let max_simulation_time_seconds: f64 = 6300.0;
    // expect overprediction of mass flowrates in both loops 
    // to about 8.5%
    let pri_loop_relative_tolerance = 0.085;
    let dracs_loop_relative_tolerance = 0.085;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    // for 3800s, this is the result:
    // let (heater_power_watts,
    //     tchx_outlet_temp_degc,
    //     experimental_dracs_mass_flowrate_kg_per_s,
    //     experimental_pri_mass_flowrate_kg_per_s,
    //     simulated_expected_dracs_mass_flowrate_kg_per_s,
    //     simulated_expected_pri_mass_flowrate_kg_per_s) 
    // = (2288.83, 40.0, 4.1150e-2, 3.4120e-2, 4.2519e-2, 3.5072e-2);
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (2288.83, 40.0, 4.1150e-2, 3.4120e-2, 4.2585e-2, 3.5380e-2);


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

    regression_coupled_dracs_loop_version_3(
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
    ).unwrap();


}
/// took about 151 s on the i5-13500H 
#[test] 
pub fn regression_long_test_calibrated_ver3_set_c6_extra_long(){
    use regression_coupled_dracs_loop_version_3::*;

    let max_simulation_time_seconds: f64 = 5000.0;
    // expect overprediction of mass flowrates in both loops 
    // to about 8.5%
    let pri_loop_relative_tolerance = 0.085;
    let dracs_loop_relative_tolerance = 0.085;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    // for 3800s, this is the result:
    // let (heater_power_watts,
    //     tchx_outlet_temp_degc,
    //     experimental_dracs_mass_flowrate_kg_per_s,
    //     experimental_pri_mass_flowrate_kg_per_s,
    //     simulated_expected_dracs_mass_flowrate_kg_per_s,
    //     simulated_expected_pri_mass_flowrate_kg_per_s) 
    // = (2288.83, 40.0, 4.1150e-2, 3.4120e-2, 4.2519e-2, 3.5072e-2);
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (2288.83, 40.0, 4.1150e-2, 3.4120e-2, 4.2749e-2, 3.5424e-2);


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

    regression_coupled_dracs_loop_version_3(
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
    ).unwrap();


}
/// took about 131 s on the i5-13500H 
#[test] 
pub fn regression_long_test_calibrated_ver3_set_c6(){
    use regression_coupled_dracs_loop_version_3::*;

    let max_simulation_time_seconds: f64 = 3800.0;
    // expect overprediction of mass flowrates in both loops 
    // to about 8.5%
    let pri_loop_relative_tolerance = 0.085;
    let dracs_loop_relative_tolerance = 0.085;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (2288.83, 40.0, 4.1150e-2, 3.4120e-2, 4.3184e-2, 3.5538e-2);


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

    regression_coupled_dracs_loop_version_3(
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
    ).unwrap();


}
/// c6 for
/// version 3 of coupled DRACS loop 
///
/// for version 3, simple calibration is done as with version 2,
/// but the vertical TCHX is split into two equal halves as was done in SAM,
/// only the bottom half will have the calibrated heat transfer coefficient.
/// The rest of the TCHX, the horizontal TCHX and 35b1, will be insulated.
///
/// this is a shorter version used during early stages of testing
#[test] 
pub fn regression_long_test_calibrated_ver3_set_c6_shorter(){
    use regression_coupled_dracs_loop_version_3::*;

    let max_simulation_time_seconds: f64 = 3000.0;
    // expect overprediction of mass flowrates in both loops 
    // to about 8.5%
    let pri_loop_relative_tolerance = 0.085;
    let dracs_loop_relative_tolerance = 0.085;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (2288.83, 40.0, 4.1150e-2, 3.4120e-2, 4.3930e-2, 3.5729e-2);


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

    regression_coupled_dracs_loop_version_3(
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
    ).unwrap();


}

/// c7 for
/// version 3 of coupled DRACS loop 
///
/// for version 3, simple calibration is done as with version 2,
/// but the vertical TCHX is split into two equal halves as was done in SAM,
/// only the bottom half will have the calibrated heat transfer coefficient.
/// The rest of the TCHX, the horizontal TCHX and 35b1, will be insulated.
#[test] 
pub fn regression_long_test_calibrated_ver3_set_c7(){
    use regression_coupled_dracs_loop_version_3::*;

    let max_simulation_time_seconds: f64 = 3800.0;
    // expect overprediction of mass flowrates in both loops 
    // to about 8.5%
    let pri_loop_relative_tolerance = 0.085;
    let dracs_loop_relative_tolerance = 0.085;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (2508.71, 40.0, 4.3120e-2, 3.5620e-2, 4.5049e-2, 3.7077e-2);


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

    regression_coupled_dracs_loop_version_3(
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
    ).unwrap();


}
//
//
/// c8 for
/// version 3 of coupled DRACS loop 
///
/// for version 3, simple calibration is done as with version 2,
/// but the vertical TCHX is split into two equal halves as was done in SAM,
/// only the bottom half will have the calibrated heat transfer coefficient.
/// The rest of the TCHX, the horizontal TCHX and 35b1, will be insulated.
#[test] 
pub fn regression_long_test_calibrated_ver3_set_c8(){
    use regression_coupled_dracs_loop_version_3::*;

    let max_simulation_time_seconds: f64 = 3800.0;
    // expect overprediction of mass flowrates in both loops 
    // to about 8.5%
    let pri_loop_relative_tolerance = 0.085;
    let dracs_loop_relative_tolerance = 0.085;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (2685.83, 40.0, 4.5090e-2, 3.5930e-2, 4.6476e-2, 3.8242e-2);


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

    regression_coupled_dracs_loop_version_3(
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
    ).unwrap();


}
/// on i7-10875H 1.5 GHz clock speed, (throttled down)
/// test time is ~177 s
/// c8 for
/// version 3 of coupled DRACS loop 
///
/// for version 3, simple calibration is done as with version 2,
/// but the vertical TCHX is split into two equal halves as was done in SAM,
/// only the bottom half will have the calibrated heat transfer coefficient.
/// The rest of the TCHX, the horizontal TCHX and 35b1, will be insulated.
#[test] 
pub fn regression_long_test_calibrated_ver3_set_c9(){
    use regression_coupled_dracs_loop_version_3::*;

    let max_simulation_time_seconds: f64 = 3800.0;
    // expect overprediction of mass flowrates in both loops 
    // to about 8.5%
    let pri_loop_relative_tolerance = 0.085;
    let dracs_loop_relative_tolerance = 0.085;

    // I'm writing in this format so that the data will be easier 
    // to copy over to csv
    let (heater_power_watts,
        tchx_outlet_temp_degc,
        experimental_dracs_mass_flowrate_kg_per_s,
        experimental_pri_mass_flowrate_kg_per_s,
        simulated_expected_dracs_mass_flowrate_kg_per_s,
        simulated_expected_pri_mass_flowrate_kg_per_s) 
        = (2764.53, 40.0, 4.6990e-2, 3.5470e-2, 4.7090e-2, 3.8738e-2);


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

    regression_coupled_dracs_loop_version_3(
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
    ).unwrap();


}

/// function to test version 2 calibrated
/// coupled dracs loop and compare with experimental data 
/// this is more of a regression function, so I want to check the 
/// output of the uncalibrated loop
/// 
/// 
/// based on initial calibration with set c,
/// a best effort was made 
///
/// for the pri loop 
/// cold leg insulation thickness is 0.15 cm 
/// hot leg insulation thickness is 0.24 cm 
///
/// for the dracs loop 
/// cold leg insulation thickness is 3cm 
/// hot leg insulation thickness is 0.75 cm
///
/// for the DHX STHE,
///
/// shell side to tubes nusselt correction factor is 4.7
/// insulation thickness is 0.161 cm 
/// shell side to ambient correction factor is 10.3 
/// heat loss to ambient is 33.9 W/(m^2 K)
///
/// no changes made to tchx yet, I want to calibrate slowly
pub mod regression_coupled_dracs_loop_version_3;
