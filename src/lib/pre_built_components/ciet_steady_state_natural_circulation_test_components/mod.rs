/// For CIET natural circulation tests,
///
/// there are two sets of tests.
///
/// The first one isolates the DRACS 
///
/// in this test, the DRACS loop with zero parasitic heat loss is 
/// compared against the analytical solution by Scarlat for a 
/// natural circulation loop with zero heat loss
///
/// Three sets of tests are performed. 
/// One with TCHX outlet temperature of 46 C 
/// Second with TCHX outlet temperature of 40 C
/// Third with TCHX outlet temperature of 35 C
///
/// These are enforced as part of the boundary conditions of the TCHX
///
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
/// Table 3 also provides the data for these tests. These are included 
/// in the module
///
/// For this set of tests, we do not worry about real-time calculations 
/// just yet
///
/// SAM max error threshold is about 1% 
/// that is (m_SAM - m_analytical)/m_analytical
pub mod zero_parasitic_heat_loss_isolated_dracs_loop_tests;

/// tests for parasitic heat loss regression and calibration tests
/// this is meant to callibrate CIET's model and also to test if 
/// the wall correction is working properly
pub mod parasitic_heat_loss_regression_tests;


/// For validation, real tests were done for the dracs loop coupled 
/// with the DHX and Heater branches in CIET
///
/// The relevant publications where the experimental data was pulled from 
/// was:
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
/// SAM max error threshold is about 6.76%
/// that is (m_SAM - m_experimental)/m_experimental
///
///
pub mod coupled_dracs_loop_tests;

/// components for the DRACS loop
pub mod dracs_loop_components;


