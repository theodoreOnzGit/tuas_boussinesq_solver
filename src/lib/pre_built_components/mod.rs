/// HeatTransferEntity module 
///
/// For practical reasons, using different functions to connect 
/// control volumes of various types (whether singleCV or arrayed control 
/// volumes) can be quite cumbersome 
///
/// To help the user connect these, I classify (and abstract) all control 
/// volumes and boundary conditions as HeatTransferEntity objects.
///
/// The basic use is that HeatTransferEntity objects are connected to 
/// each other by a user specified heat transfer interaction
/// 
pub mod heat_transfer_entities;

/// for fluid flow through non insulated pipes and fluid components, these pipes will 
/// be represented by control volumes laterally coupled to one 
/// another. 
pub mod non_insulated_fluid_components;


/// for fluid flow through insulated and fluid components with one layer 
/// of insulation, these pipes will 
/// be represented by control volumes laterally coupled to one 
/// another. 
pub mod insulated_pipes_and_fluid_components;

/// for fluid through through a series of parallel pipes 
/// each with a uniform hydraulic diameter and length 
/// usually used for heat exchangers
/// these are non insulated by default to maximise heat transfer rates
///
/// They are used to model the tube side of a heat exchanger (without 
/// calculations for the shell side) 
///
/// These are used in isolated DRACS loop calculations where the parallel 
/// pipes are exposed to a boundary condition rather than a modelled tube
/// They can also be used to model coolers where parallel tubes are exposed 
/// to a stream of colder air.
///
pub mod non_insulated_parallel_fluid_components;

/// This is code for 1D modelling of shell and tube heat exchangers
///
pub mod shell_and_tube_heat_exchanger;

/// represents one dimensional solid structure
pub mod one_d_solid_structure;



/// represents the old CIET struct support codes based on
/// https://escholarship.org/uc/item/0362h3zf
///
/// Ong, T. K. C. (2024). Digital Twins as 
/// Testbeds for Iterative Simulated Neutronics Feedback 
/// Controller Development (Doctoral dissertation, UC Berkeley).
pub mod ciet_struct_supports;

/// represents the CIET heater top and bottom head codes based on
/// https://escholarship.org/uc/item/0362h3zf
///
/// Ong, T. K. C. (2024). Digital Twins as 
/// Testbeds for Iterative Simulated Neutronics Feedback 
/// Controller Development (Doctoral dissertation, UC Berkeley).
pub mod ciet_heater_top_and_bottom_head_bare;


/// insulated porous media pipes are basically insulated pipes with 
/// some things lodged inside. It could be a packed bed, or an annular inner pipe 
/// Some of the code 
/// represents the CIET static mixer codes based on
/// https://escholarship.org/uc/item/0362h3zf
///
/// Ong, T. K. C. (2024). Digital Twins as 
/// Testbeds for Iterative Simulated Neutronics Feedback 
/// Controller Development (Doctoral dissertation, UC Berkeley).
pub mod insulated_porous_media_fluid_components;


/// non insulated porous media pipes are basically non insulated pipes with 
/// some things lodged inside. It could be a packed bed, or an annular inner pipe 
/// Some of the code 
/// this can be used to represent non insulated porous media pipes such as 
/// such as the old CIET heater version 2 based on 
/// https://escholarship.org/uc/item/0362h3zf
///
/// Ong, T. K. C. (2024). Digital Twins as 
/// Testbeds for Iterative Simulated Neutronics Feedback 
/// Controller Development (Doctoral dissertation, UC Berkeley).
pub mod non_insulated_porous_media_fluid_components;

/// ciet components for pipes and valves for use in the isothermal test
///
/// Zweibaum, N. (2015). Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University 
/// of California, Berkeley.
///
/// In my master's thesis, heat structure information was not included. However, 
/// I shall include them in this round
pub mod ciet_isothermal_test_components;


/// ciet components for pipes and valves for use in the natural circulation 
/// test. I attempt to reproduce some results in the following 
/// publications:
///
/// Zweibaum, N. (2015). Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University 
/// of California, Berkeley.
///
/// Zou, L., Hu, R., & Charpentier, A. (2019). SAM code 
/// validation using the compact integral effects test (CIET) experimental 
/// data (No. ANL/NSE-19/11). Argonne National 
/// Lab.(ANL), Argonne, IL (United States).
pub mod ciet_steady_state_natural_circulation_test_components;


/// From UW Madison FLiBe loop:
///
///
/// Britsch, K., Anderson, M., Brooks, P., & 
/// Sridharan, K. (2019). Natural circulation 
/// FLiBe loop overview. International Journal of 
/// Heat and Mass Transfer, 134, 970-983.
pub mod uw_madison_flibe_loop_components;


/// Based on the natural circulation and isothermal loop, 
/// I'm constructing CIET as a full loop now, inclusive of all 3 
/// branches. 
///
/// todo: will need to validate this loop for steady state,
/// natural circulation flow, and transient response (freq response 
/// testing)
/// 
///
/// Zweibaum, N. (2015). Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University 
/// of California, Berkeley.
///
/// Zou, L., Hu, R., & Charpentier, A. (2019). SAM code 
/// validation using the compact integral effects test (CIET) experimental 
/// data (No. ANL/NSE-19/11). Argonne National 
/// Lab.(ANL), Argonne, IL (United States).
pub mod ciet_three_branch_plus_dracs;


/// based on schematics of gFHR, flowrates of around 1173 kg/s are 
/// expected. These series of tests is meant to ensure that pipes,
/// individual branches and parallel branches work correctly in this 
/// flow regime
#[cfg(test)]
pub mod gfhr_flibe_pipe_tests;
