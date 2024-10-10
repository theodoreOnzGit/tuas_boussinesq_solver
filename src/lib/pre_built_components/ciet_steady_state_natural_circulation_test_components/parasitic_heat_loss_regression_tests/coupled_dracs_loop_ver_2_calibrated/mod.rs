
#[test] 
pub fn regression_long_test_calibrated_ver2_set_c1(){
    use regression_coupled_dracs_loop_version_2::*;

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
        = (841.02, 40.0, 2.6860e-2, 2.0030e-2, 2.5814e-2, 2.1428e-2);


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

    regression_coupled_dracs_loop_version_2(
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
#[test] 
pub fn regression_long_test_calibrated_ver2_set_c2(){
    use regression_coupled_dracs_loop_version_2::*;

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
        = (1158.69, 40.0, 3.0550e-2, 2.3670e-2, 3.0965e-2, 2.5347e-2);


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

    regression_coupled_dracs_loop_version_2(
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
#[test] 
pub fn regression_long_test_calibrated_ver2_set_c3(){
    use regression_coupled_dracs_loop_version_2::*;

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
        = (1409.22, 40.0, 3.3450e-2, 2.6350e-2, 3.4296e-2, 2.7981e-2);


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

    regression_coupled_dracs_loop_version_2(
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
#[test] 
pub fn regression_long_test_calibrated_ver2_set_c4(){
    use regression_coupled_dracs_loop_version_2::*;

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
        = (1736.11, 40.0, 3.6490e-2, 2.9490e-2, 3.8046e-2, 3.0990e-2);


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

    regression_coupled_dracs_loop_version_2(
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

#[test] 
pub fn regression_long_test_calibrated_ver2_set_c5(){
    use regression_coupled_dracs_loop_version_2::*;

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
        = (2026.29, 40.0, 3.8690e-2, 3.1900e-2, 4.0970e-2, 3.3346e-2);


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

    regression_coupled_dracs_loop_version_2(
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
#[test] 
pub fn regression_long_test_calibrated_ver2_set_c6(){
    use regression_coupled_dracs_loop_version_2::*;

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
        = (2288.83, 40.0, 4.1150e-2, 3.4120e-2, 4.3369e-2, 3.5275e-2);


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

    regression_coupled_dracs_loop_version_2(
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

#[test] 
pub fn regression_long_test_calibrated_ver2_set_c7(){
    use regression_coupled_dracs_loop_version_2::*;

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
        = (2508.71, 40.0, 4.3120e-2, 3.5620e-2, 4.5232e-2, 3.6764e-2);


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

    regression_coupled_dracs_loop_version_2(
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
#[test] 
pub fn regression_long_test_calibrated_ver2_set_c8(){
    use regression_coupled_dracs_loop_version_2::*;

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
        = (2685.83, 40.0, 4.5090e-2, 3.5930e-2, 4.6650e-2, 3.7891e-2);


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

    regression_coupled_dracs_loop_version_2(
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
// on i7-10875H 1.5 GHz clock speed, (throttled down)
// test time is ~177 s
#[test] 
pub fn regression_long_test_calibrated_ver2_set_c9(){
    use regression_coupled_dracs_loop_version_2::*;

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
        = (2764.53, 40.0, 4.6990e-2, 3.5470e-2, 4.7259e-2, 3.8373e-2);


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

    regression_coupled_dracs_loop_version_2(
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
pub mod regression_coupled_dracs_loop_version_2;
