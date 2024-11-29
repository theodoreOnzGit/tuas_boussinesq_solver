/// this version of ciet is optimised for real-time 
/// simulation. It will not be validated, but it will be 
/// fun to play with as a simulator. Useful for education and etc.
///
/// 
pub mod ciet_educational_simulator_loop;

/// CIET needs Thermo-hydraulic equations solved in TUAS 
///
/// Writing them out explicitly in a procedural form is quite 
/// cumbersome. It is much more concise to have these functions 
/// here
///
/// Here there are functions for connecting the 
/// heat transfer entities in:
///
/// - dracs loop 
/// - pri loop DHX branch
/// - pri loop heater branch 
/// - pri loop CTAH branch (forced circ)
///
/// Also I need to solve fluid flow in 
/// - dracs loop 
/// - pri loop, DHX, heater and CTAH branch
///
/// I would also need to be able to block flow in pri loop DHX 
/// and CTAH branch as well, so as to isolate the loops.
///
pub mod solver_functions;

/// adds extra components specific to the three branch 
/// simulation,
/// the other components were borrowed from the isothermal 
/// test and steady state natural circulation modules
pub mod components;
