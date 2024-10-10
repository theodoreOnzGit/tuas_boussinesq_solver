//! The nusselt correlations class has calculates nusselt numbers 
//! given a certain geometry

/// These are nusselt correlations for pipes
pub mod pipe_correlations;


/// contains data types used for nusselt number correlation 
/// enums
pub mod input_structs;

/// contains nusselt number enums used for input calculation
pub mod enums;

/// tests to ensure correlations are working correctly 
pub mod tests;
