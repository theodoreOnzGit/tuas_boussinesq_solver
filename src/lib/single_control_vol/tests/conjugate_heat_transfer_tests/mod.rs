/// Emulates Frequency Response test by Chris Poresky, 
/// Poresky, C. (2017). Frequency response testing in the ciet facility. Berkeley, CA.
mod one_dimension_heater;

/// Creates a CIET heater v2.0 steady state test with 8 axial nodes and 
/// two metallic shell nodes in the radial direction using only 
/// SingleCV objects 
///
/// This acts as a baseline test to check which methods impact calculation 
/// time the most
mod ciet_heater_v2_single_cv;


/// Creates a CIET heater v2.0 steady state test with 8 axial nodes and 
/// two metallic shell nodes in the radial direction using only 
/// SingleCV objects 
///
/// This test checks the impact of multithreading on simulation speed 
/// of the heater
mod ciet_heater_v2_single_cv_speedup_multithreading;

/// Creates a CIET heater v2.0 steady state test with 8 axial nodes and 
/// two metallic shell nodes in the radial direction using only 
/// SingleCV objects 
///
/// This test checks the impact of 
/// reducing the number of axial nodes across the heater v2 from 8 to 3
mod ciet_heater_v2_single_cv_speedup_three_axial_nodes;


/// Creates a CIET heater v2.0 steady state test with 8 axial nodes and 
/// two metallic shell nodes in the radial direction using only 
/// SingleCV objects 
///
/// This test checks the impact of 
/// reducing the number of radial nodes for the steel shell from 2 to 1
mod ciet_heater_v2_single_cv_speedup_one_radial_steel_shell_node;
