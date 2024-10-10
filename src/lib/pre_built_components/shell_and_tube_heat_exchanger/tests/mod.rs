/// checks if basic things such as obtaining the overall 
/// heat transfer coefficient and shell side area work okay
pub mod basic_postprocessing;


/// heat transfer verification 
/// runs a series of simplified cases to check if heat 
/// exchanger works correctly
pub mod heat_transfer_verification;

/// heat exchanger verification and validation 
/// using Du's paper 
/// Du, B. C., He, Y. L., Qiu, Y., Liang, Q., & Zhou, Y. P. 
/// (2018). Investigation on heat transfer characteristics of 
/// molten salt in a shell-and-tube heat exchanger. International 
/// Communications in Heat and Mass Transfer, 96, 61-68.
pub mod hitec_molten_salt_to_yd325_du_heat_exchanger;


/// constructor tests
pub mod constructor_tests;

/// calibration function tests 
pub mod calibration_functions;
