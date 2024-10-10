/// This contains tests for the hot leg calibration of insulation 
/// thickness, 
///
/// a suitable calibrated thickness was found to be about 0.24 cm
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
pub mod hot_leg_calibration;

/// This contains tests for the cold leg calibration of insulation 
/// thickness, 
///
/// a suitable calibrated thickness from the hot leg 
/// was found to be about 0.24 cm
///
/// data from the dhx outlet to heater inlet is presented below:
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
pub mod cold_leg_validation;
