/// Tutorial 1 shows how to make a pipe component 
/// and obtain pressure losses given a mass flowrate
pub mod tutorial_1;

/// Tutorial 2 shows how to make a pipe component 
/// and obtain mass flowrate given pressure losses
pub mod tutorial_2;

/// Tutorial 3 shows the difference between pressure  
/// change and pressure drop. This is important to 
/// distinguish for hydrostatic pressure calculations 
///
/// hydrostatic pressure is important to consider for 
/// natural circulation
pub mod tutorial_3;

/// Tutorial 4 shows how to perform basic 
/// heat transfer calculations in a pipe
pub mod tutorial_4;

/// Tutorial 5 explains how to do both 
/// heat transfer calculations and 
/// fluid mechanics calculations in one timestep.
pub mod tutorial_5;

/// Tutorial 6 deals with a flow scenario in the 
/// generic Fluoride Salt Cooled High Temperature Reactor (gFHR)
/// based on Kairos Power's non proprietary model
/// https://kairospower.com/generic-fhr-core-model/
///
/// Here, tutorial 6 deals with large volumes of flows
/// the typical flowrates are around 1200 kg/s
/// and 
/// typically, pressure drop for the loop is around 0.2 MPa
/// this is on page 169 of 195 in:
/// https://www.nrc.gov/docs/ML2208/ML22088A231.pdf
/// where it says "The primary pump pressure head is 0.2 MPa during normal operation"
///
/// Temperatures can also be high, so we may have flows in excess of 
/// 1000 degrees C especially during transients.
/// 
/// At those temperatures, steel will melt, 
/// so the "pipe" will be made of graphite.
///
/// However, as of v0.0.11, graphite is not yet available as a material 
/// inside TUAS. You will have to define your own material properties,
/// 
///
/// This tutorial will show you how.
///
pub mod tutorial_6;
