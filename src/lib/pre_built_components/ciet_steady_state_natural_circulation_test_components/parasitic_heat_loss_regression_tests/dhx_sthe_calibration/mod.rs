/// Zweibaum's unpublished data:
/// dataset number,pri loop mass flowrate (kg/s),DRACS loop mass flowrate (kg/s),DHX shell top inlet (DegC),DHX tube bottom inlet(DegC),DHX shell bottom outlet (DegC),DHX tube top outlet (DegC),
/// C-1,0.02003,0.02686,71.47752,39.84713,53.60943,53.00304,
#[test]
//#[ignore="debugging"]
pub fn dhx_regression_set_c1(){

    pub use calibration_version_2::*;
    let (experimental_pri_shell_side_mass_flowrate_kg_per_s_abs,
        experimental_dracs_tube_side_mass_flowrate_kg_per_s_abs,
        dhx_shell_side_inlet_temp_degc, 
        dhx_tube_side_inlet_temp_degc, 
        dhx_shell_side_outlet_temp_set_point_degc, 
        dhx_tube_side_outlet_temp_set_point_degc) =
        (0.02003,0.02686,71.47752,39.84713,53.60943,53.00304);

    // temperatures are validated to within 0.5 K
    // regression performed to within 0.05K
    let dhx_shell_side_outlet_regression_temperature_degc = 53.20;
    let dhx_tube_side_outlet_regression_temperature_degc = 53.31;
    let max_time_seconds = 750.0;

    // settings for insulation and shell side nusselt correction 
    // factor
    let shell_side_to_tubes_nusselt_number_correction_factor = 3.3;
    let insulation_thickness_regression_cm = 0.80;
    let shell_side_to_ambient_nusselt_correction_factor = 1.0;
    let heat_loss_to_ambient_watts_per_m2_kelvin = 30.0;

    dhx_calibration_validation_test_v2(
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
        heat_loss_to_ambient_watts_per_m2_kelvin);


}
/// Zweibaum's unpublished data:
/// dataset number,pri loop mass flowrate (kg/s),DRACS loop mass flowrate (kg/s),DHX shell top inlet (DegC),DHX tube bottom inlet(DegC),DHX shell bottom outlet (DegC),DHX tube top outlet (DegC),
/// C-2,0.02367,0.03055,78.36713,39.73516,57.13467,55.30506,
#[test]
//#[ignore="debugging"]
pub fn dhx_regression_set_c2(){

    pub use calibration_version_2::*;
    let (experimental_pri_shell_side_mass_flowrate_kg_per_s_abs,
        experimental_dracs_tube_side_mass_flowrate_kg_per_s_abs,
        dhx_shell_side_inlet_temp_degc, 
        dhx_tube_side_inlet_temp_degc, 
        dhx_shell_side_outlet_temp_set_point_degc, 
        dhx_tube_side_outlet_temp_set_point_degc) =
        (0.02367,0.03055,78.36713,39.73516,57.13467,55.30506);

    // temperatures are validated to within 0.5 K
    // regression performed to within 0.05K
    let dhx_shell_side_outlet_regression_temperature_degc = 57.31;
    let dhx_tube_side_outlet_regression_temperature_degc = 55.51;
    let max_time_seconds = 750.0;

    // settings for insulation and shell side nusselt correction 
    // factor
    let shell_side_to_tubes_nusselt_number_correction_factor = 3.3;
    let insulation_thickness_regression_cm = 0.15;
    let shell_side_to_ambient_nusselt_correction_factor = 1.0;
    let heat_loss_to_ambient_watts_per_m2_kelvin = 30.0;

    dhx_calibration_validation_test_v2(
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
        heat_loss_to_ambient_watts_per_m2_kelvin,);


}
/// Zweibaum's unpublished data:
/// dataset number,pri loop mass flowrate (kg/s),DRACS loop mass flowrate (kg/s),DHX shell top inlet (DegC),DHX tube bottom inlet(DegC),DHX shell bottom outlet (DegC),DHX tube top outlet (DegC),
/// C-3,0.02635,0.03345,84.37342,39.2569,59.82845,56.82298,
#[test]
//#[ignore="debugging"]
pub fn dhx_regression_set_c3(){

    pub use calibration_version_2::*;
    let (experimental_pri_shell_side_mass_flowrate_kg_per_s_abs,
        experimental_dracs_tube_side_mass_flowrate_kg_per_s_abs,
        dhx_shell_side_inlet_temp_degc, 
        dhx_tube_side_inlet_temp_degc, 
        dhx_shell_side_outlet_temp_set_point_degc, 
        dhx_tube_side_outlet_temp_set_point_degc) =
        (0.02635,0.03345,84.37342,39.2569,59.82845,56.82298);

    // temperatures are validated to within 0.5 K
    // regression performed to within 0.05K
    let dhx_shell_side_outlet_regression_temperature_degc = 60.06;
    let dhx_tube_side_outlet_regression_temperature_degc = 56.85;
    let max_time_seconds = 750.0;

    // settings for insulation and shell side nusselt correction 
    // factor
    //
    // note: while the correction factor of 3.3 fits most dhx tube 
    // side outlet temp data points to within 0.5 K, 
    // I could not do it for this one, the correction factor had 
    // to be lowered to 2.75
    let shell_side_to_tubes_nusselt_number_correction_factor = 3.3;
    let insulation_thickness_regression_cm = 0.15;
    let shell_side_to_ambient_nusselt_correction_factor = 15.5;
    let heat_loss_to_ambient_watts_per_m2_kelvin = 30.0;

    dhx_calibration_validation_test_v2(
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
        heat_loss_to_ambient_watts_per_m2_kelvin);


}
/// Zweibaum's unpublished data:
/// dataset number,pri loop mass flowrate (kg/s),DRACS loop mass flowrate (kg/s),DHX shell top inlet (DegC),DHX tube bottom inlet(DegC),DHX shell bottom outlet (DegC),DHX tube top outlet (DegC),
/// C-4,0.02949,0.03649,90.97595,39.86112,63.9812,59.44921,
#[test]
//#[ignore="debugging"]
pub fn dhx_regression_set_c4(){

    pub use calibration_version_2::*;
    let (experimental_pri_shell_side_mass_flowrate_kg_per_s_abs,
        experimental_dracs_tube_side_mass_flowrate_kg_per_s_abs,
        dhx_shell_side_inlet_temp_degc, 
        dhx_tube_side_inlet_temp_degc, 
        dhx_shell_side_outlet_temp_set_point_degc, 
        dhx_tube_side_outlet_temp_set_point_degc) =
        (0.02949,0.03649,90.97595,39.86112,63.9812,59.44921);

    // temperatures are validated to within 0.5 K
    // regression performed to within 0.05K
    let dhx_shell_side_outlet_regression_temperature_degc = 63.70;
    let dhx_tube_side_outlet_regression_temperature_degc = 59.51;
    let max_time_seconds = 750.0;

    // settings for insulation and shell side nusselt correction 
    // factor
    let shell_side_to_tubes_nusselt_number_correction_factor = 4.0;
    let insulation_thickness_regression_cm = 0.05;
    let shell_side_to_ambient_nusselt_correction_factor = 12.5;
    let heat_loss_to_ambient_watts_per_m2_kelvin = 30.0;

    dhx_calibration_validation_test_v2(
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
        heat_loss_to_ambient_watts_per_m2_kelvin);

}
/// Zweibaum's unpublished data:
/// dataset number,pri loop mass flowrate (kg/s),DRACS loop mass flowrate (kg/s),DHX shell top inlet (DegC),DHX tube bottom inlet(DegC),DHX shell bottom outlet (DegC),DHX tube top outlet (DegC),
/// C-5,0.0319,0.03869,96.20228,40.01355,67.05336,61.31769,
#[test]
//#[ignore="debugging"]
pub fn dhx_regression_set_c5(){

    pub use calibration_version_2::*;
    let (experimental_pri_shell_side_mass_flowrate_kg_per_s_abs,
        experimental_dracs_tube_side_mass_flowrate_kg_per_s_abs,
        dhx_shell_side_inlet_temp_degc, 
        dhx_tube_side_inlet_temp_degc, 
        dhx_shell_side_outlet_temp_set_point_degc, 
        dhx_tube_side_outlet_temp_set_point_degc) =
        (0.0319,0.03869,96.20228,40.01355,67.05336,61.31769);

    // temperatures are validated to within 0.5 K
    // regression performed to within 0.05K
    let dhx_shell_side_outlet_regression_temperature_degc = 67.05;
    let dhx_tube_side_outlet_regression_temperature_degc = 61.18;
    let max_time_seconds = 750.0;

    // settings for insulation and shell side nusselt correction 
    // factor
    let shell_side_to_tubes_nusselt_number_correction_factor = 4.0;
    let insulation_thickness_regression_cm = 0.05;
    let shell_side_to_ambient_nusselt_correction_factor = 12.5;
    let heat_loss_to_ambient_watts_per_m2_kelvin = 40.0;
    // note, probably need to calibrate heat loss to environment
    

    dhx_calibration_validation_test_v2(
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
        heat_loss_to_ambient_watts_per_m2_kelvin);


}
/// Zweibaum's unpublished data:
/// dataset number,pri loop mass flowrate (kg/s),DRACS loop mass flowrate (kg/s),DHX shell top inlet (DegC),DHX tube bottom inlet(DegC),DHX shell bottom outlet (DegC),DHX tube top outlet (DegC),
/// C-6,0.03412,0.04115,101.3375,39.53125,69.85085,62.69342,
#[test]
//#[ignore="debugging"]
pub fn dhx_regression_set_c6(){

    pub use calibration_version_2::*;
    let (experimental_pri_shell_side_mass_flowrate_kg_per_s_abs,
        experimental_dracs_tube_side_mass_flowrate_kg_per_s_abs,
        dhx_shell_side_inlet_temp_degc, 
        dhx_tube_side_inlet_temp_degc, 
        dhx_shell_side_outlet_temp_set_point_degc, 
        dhx_tube_side_outlet_temp_set_point_degc) =
        (0.03412,0.04115,101.3375,39.53125,69.85085,62.69342);

    // temperatures are validated to within 0.5 K
    // regression performed to within 0.05K
    let dhx_shell_side_outlet_regression_temperature_degc = 70.28;
    let dhx_tube_side_outlet_regression_temperature_degc = 62.34;
    let max_time_seconds = 750.0;

    // settings for insulation and shell side nusselt correction 
    // factor
    let shell_side_to_tubes_nusselt_number_correction_factor = 4.0;
    let insulation_thickness_regression_cm = 0.05;
    let shell_side_to_ambient_nusselt_correction_factor = 12.5;
    let heat_loss_to_ambient_watts_per_m2_kelvin = 45.0;
    // note, probably need to calibrate heat loss to environment

    dhx_calibration_validation_test_v2(
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
        heat_loss_to_ambient_watts_per_m2_kelvin);

}
/// Zweibaum's unpublished data:
/// dataset number,pri loop mass flowrate (kg/s),DRACS loop mass flowrate (kg/s),DHX shell top inlet (DegC),DHX tube bottom inlet(DegC),DHX shell bottom outlet (DegC),DHX tube top outlet (DegC),
/// C-7,0.03562,0.04312,106.43149,39.8924,73.21226,64.45658,
#[test]
//#[ignore="debugging"]
pub fn dhx_regression_set_c7(){

    pub use calibration_version_2::*;
    let (experimental_pri_shell_side_mass_flowrate_kg_per_s_abs,
        experimental_dracs_tube_side_mass_flowrate_kg_per_s_abs,
        dhx_shell_side_inlet_temp_degc, 
        dhx_tube_side_inlet_temp_degc, 
        dhx_shell_side_outlet_temp_set_point_degc, 
        dhx_tube_side_outlet_temp_set_point_degc) =
        (0.03562,0.04312,106.43149,39.8924,73.21226,64.45658);

    // temperatures are validated to within 0.5 K
    // regression performed to within 0.05K
    let dhx_shell_side_outlet_regression_temperature_degc = 73.59;
    let dhx_tube_side_outlet_regression_temperature_degc = 63.97;
    let max_time_seconds = 750.0;

    // settings for insulation and shell side nusselt correction 
    // factor
    let shell_side_to_tubes_nusselt_number_correction_factor = 4.0;
    let insulation_thickness_regression_cm = 0.05;
    let shell_side_to_ambient_nusselt_correction_factor = 12.5;
    let heat_loss_to_ambient_watts_per_m2_kelvin = 50.0;
    // note, probably need to calibrate heat loss to environment

    dhx_calibration_validation_test_v2(
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
        heat_loss_to_ambient_watts_per_m2_kelvin);


}
/// Zweibaum's unpublished data:
/// dataset number,pri loop mass flowrate (kg/s),DRACS loop mass flowrate (kg/s),DHX shell top inlet (DegC),DHX tube bottom inlet(DegC),DHX shell bottom outlet (DegC),DHX tube top outlet (DegC),
/// C-8,0.03593,0.04509,111.37615,39.91183,76.13202,66.11271,
#[test]
//#[ignore="debugging"]
pub fn dhx_regression_set_c8(){

    pub use calibration_version_2::*;
    let (experimental_pri_shell_side_mass_flowrate_kg_per_s_abs,
        experimental_dracs_tube_side_mass_flowrate_kg_per_s_abs,
        dhx_shell_side_inlet_temp_degc, 
        dhx_tube_side_inlet_temp_degc, 
        dhx_shell_side_outlet_temp_set_point_degc, 
        dhx_tube_side_outlet_temp_set_point_degc) =
        (0.03593,0.04509,111.37615,39.91183,76.13202,66.11271);

    // temperatures are validated to within 0.5 K
    // regression performed to within 0.05K
    let dhx_shell_side_outlet_regression_temperature_degc = 76.49;
    let dhx_tube_side_outlet_regression_temperature_degc = 65.99;
    let max_time_seconds = 750.0;

    // settings for insulation and shell side nusselt correction 
    // factor
    // note: while the correction factor of 3.3 fits most dhx tube 
    // side outlet temp data points to within 0.5 K, 
    // I could not do it for this one, the correction factor had 
    // to be increased to 4.0
    let shell_side_to_tubes_nusselt_number_correction_factor = 5.0;
    let insulation_thickness_regression_cm = 0.05;
    // increasing the ambient nusselt number beyond 20 doesn't make much 
    // difference here if insulation thickness is fixed at 0.10 cm
    //let shell_side_to_ambient_nusselt_correction_factor = 226.5;
    let shell_side_to_ambient_nusselt_correction_factor = 12.5;
    let heat_loss_to_ambient_watts_per_m2_kelvin = 30.0;

    dhx_calibration_validation_test_v2(
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
        heat_loss_to_ambient_watts_per_m2_kelvin);


}
/// Zweibaum's unpublished data:
/// dataset number,pri loop mass flowrate (kg/s),DRACS loop mass flowrate (kg/s),DHX shell top inlet (DegC),DHX tube bottom inlet(DegC),DHX shell bottom outlet (DegC),DHX tube top outlet (DegC),
/// C-9,0.03547,0.04699,116.05003,39.64593,79.02407,67.40722,
#[test]
//#[ignore="debugging"]
pub fn dhx_regression_set_c9(){

    pub use calibration_version_1::*;
    let (experimental_pri_shell_side_mass_flowrate_kg_per_s_abs,
        experimental_dracs_tube_side_mass_flowrate_kg_per_s_abs,
        dhx_shell_side_inlet_temp_degc, 
        dhx_tube_side_inlet_temp_degc, 
        dhx_shell_side_outlet_temp_set_point_degc, 
        dhx_tube_side_outlet_temp_set_point_degc) =
        (0.03547,0.04699,116.05003,39.64593,79.02407,67.40722);

    // temperatures are validated to within 0.5 K
    // regression performed to within 0.05K
    let dhx_shell_side_outlet_regression_temperature_degc = 78.88;
    let dhx_tube_side_outlet_regression_temperature_degc = 67.39;
    let max_time_seconds = 750.0;

    // settings for insulation and shell side nusselt correction 
    // factor
    // note: while the correction factor of 3.3 fits most dhx tube 
    // side outlet temp data points to within 0.5 K, 
    // I could not do it for this one, the correction factor had 
    // to be increased to 4.5
    let shell_side_to_tubes_nusselt_number_correction_factor = 5.8;
    let insulation_thickness_regression_cm = 0.10;
    let shell_side_to_ambient_nusselt_correction_factor = 12.5;

    dhx_calibration_validation_test_v1(
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
        shell_side_to_ambient_nusselt_correction_factor);


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
///
/// unfortunately, calibration version 1 is not able to account for the 
/// voracious amount of parasitic heat loss from c5 to c7 
/// I probably need to tweak the heat transfer to ambient as well
pub mod calibration_version_1;


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
///
/// unfortunately, calibration version 1 is not able to account for the 
/// voracious amount of parasitic heat loss from c5 to c7 
/// I probably need to tweak the heat transfer to ambient as well
pub mod calibration_version_2;
