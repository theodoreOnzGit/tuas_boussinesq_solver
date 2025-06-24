/// contains the code for single pipes and other components in 
/// primary and intermediate loop of what could be used to cool the gFHR
///
/// generic (Fluoride Salt Cooled High Temperature Reactor)
///
/// Note: Radiation Heat Transfer (RHT) NOT accounted for
pub mod components;
/// first, single pipe tests 
///
/// this is where single pipes of the typical fhrs are concerned 
/// both pressure change from mass flowrate and vice verse should be tested
/// in fwd and backwd flow
pub mod single_pipe_tests;


/// second, need to have 
/// tests across individual branches
pub mod single_branch;

/// third, need to have tests across multiple branches 
pub mod multi_branch;


// for building the components,
// we should design it around the intermediate heat exchanger 
// heat exchanger is salt to salt, STHE 
// maybe 1200 kg/s on tube side of FLiBe 
// and enough flowrate of HITEC on shell side to ensure that 
// temperature settings are right
