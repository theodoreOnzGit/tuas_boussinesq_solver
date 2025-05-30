

/// functions used for calculating the thermal hydraulics inside the DRACS 
/// loop
///
/// mostly without tchx calibration, that is to say,
/// the vertical TCHX is not split into two parts as was done in SAM:
///
/// Zou, L., Hu, G., O'Grady, D., & Hu, R. (2021). Code validation of 
/// SAM using natural-circulation experimental data from the compact 
/// integral effects test (CIET) facility. 
/// Nuclear Engineering and Design, 377, 111144.
///
pub mod dracs_loop_calc_functions_no_tchx_calibration;

/// functions used for calculating the thermal hydraulics inside the DRACS 
/// loop
///
/// mostly with tchx calibration, that is to say,
/// the vertical TCHX is split into two parts (35b-1 and 35b-2) 
/// as was done in SAM:
///
/// Zou, L., Hu, G., O'Grady, D., & Hu, R. (2021). Code validation of 
/// SAM using natural-circulation experimental data from the compact 
/// integral effects test (CIET) facility. 
/// Nuclear Engineering and Design, 377, 111144.
///
pub mod dracs_loop_calc_functions_sam_tchx_calibration;

/// functions used for calculating the thermal hydraulics inside 
/// the Heater and DHX branch 
/// Note: heater v1.0 is used
pub mod pri_loop_calc_functions;

/// these are calibration 

/// We use:
///
/// Zou, L., Hu, G., O'Grady, D., & Hu, R. (2021). Code validation of 
/// SAM using natural-circulation experimental data from the compact 
/// integral effects test (CIET) facility. 
/// Nuclear Engineering and Design, 377, 111144.
///
/// According to table 2,
///
/// Case A has 7 tests and TCHX out temperature of 46 C
/// Case B has 9 tests and TCHX out temperature of 35 C
/// Case C has 9 tests and TCHX out temperature of 40 C
///
/// Table 4 provides the data we use here
/// 
///
pub mod dataset_a;


/// In the original SAM publication
///
/// Zou, L., Hu, R., & Charpentier, A. (2019). SAM code 
/// validation using the compact integral effects test (CIET) experimental 
/// data (No. ANL/NSE-19/11). Argonne National 
/// Lab.(ANL), Argonne, IL (United States).
///
/// I found it hard to distinguish what TCHX temperatures case A,
/// B and C were.
///
/// But there was another publication which shows which is test group 
/// corresponds to which temperature:
///
/// Zou, L., Hu, G., O'Grady, D., & Hu, R. (2021). Code validation of 
/// SAM using natural-circulation experimental data from the compact 
/// integral effects test (CIET) facility. 
/// Nuclear Engineering and Design, 377, 111144.
///
/// According to table 2,
///
/// Case A has 7 tests and TCHX out temperature of 46 C
/// Case B has 9 tests and TCHX out temperature of 35 C
/// Case C has 9 tests and TCHX out temperature of 40 C
///
/// Table 3 also provides the data 
/// 
///
pub mod dataset_b;

/// In the original SAM publication
///
/// Zou, L., Hu, R., & Charpentier, A. (2019). SAM code 
/// validation using the compact integral effects test (CIET) experimental 
/// data (No. ANL/NSE-19/11). Argonne National 
/// Lab.(ANL), Argonne, IL (United States).
///
/// I found it hard to distinguish what TCHX temperatures case A,
/// B and C were.
///
/// But there was another publication which shows which is test group 
/// corresponds to which temperature:
///
/// Zou, L., Hu, G., O'Grady, D., & Hu, R. (2021). Code validation of 
/// SAM using natural-circulation experimental data from the compact 
/// integral effects test (CIET) facility. 
/// Nuclear Engineering and Design, 377, 111144.
///
/// According to table 2,
///
/// Case A has 7 tests and TCHX out temperature of 46 C
/// Case B has 9 tests and TCHX out temperature of 35 C
/// Case C has 9 tests and TCHX out temperature of 40 C
///
/// Table 3 also provides the data 
/// 
///
pub mod dataset_c;


/// looks like the dracs loop from the original isolated loop is 
/// not well calibrated because there may be lower resistance compared 
/// to the SAM model 
/// might want to do calibration of dracs loop resistances first 
pub mod dracs_loop_resistance_calibration;



/// constructor for the dhx shell and tube heat exchanger 
/// based on Zou's specifications
pub mod dhx_constructor;

/// debugging tests for functions to make natural circulation 
/// testing easier 
pub mod debugging;
