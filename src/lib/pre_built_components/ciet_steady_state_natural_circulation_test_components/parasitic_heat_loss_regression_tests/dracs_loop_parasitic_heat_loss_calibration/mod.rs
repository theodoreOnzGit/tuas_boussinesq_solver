/// calibration strategy is to adjust insulation thickness 
/// until correct parasitic heat loss is achieved
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
pub mod hot_leg_calibration;

/// calibration strategy is to adjust insulation thickness 
/// until correct parasitic heat loss is achieved
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
pub mod cold_leg_calibration;
