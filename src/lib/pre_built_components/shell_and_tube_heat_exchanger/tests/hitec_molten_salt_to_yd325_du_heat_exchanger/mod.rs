/// shell and tube heat exchanger test set A,
///
/// This is where 
/// salt volumetric flowrate is 12.63 m3/s 
/// oil volumetic flowrate is 15.635 m3/s 
/// temperatures of oil and salt are varied
/// from 74.49  - 90.41 C  (YD325 oil)
/// and 214.93 - 236.91 C (HITEC salt)
/// respectively
pub mod set_a;
/// shell and tube heat exchanger test set B,
///
/// This is where 
/// salt volumetric flowrate is 14.63 m3/s 
/// oil volumetic flowrate is 15.635 m3/s 
/// temperatures of oil and salt are varied
/// from 74.49  - 90.41 C  (YD325 oil)
/// and 214.93 - 236.91 C (HITEC salt)
/// respectively
pub mod set_b;
/// shell and tube heat exchanger test set C,
///
/// This is where 
/// salt volumetric flowrate is 16.63 m3/s 
/// oil volumetic flowrate is 15.635 m3/s 
/// temperatures of oil and salt are varied
/// from 74.49  - 90.41 C  (YD325 oil)
/// and 214.93 - 236.91 C (HITEC salt)
/// respectively
pub mod set_c;


/// in debugging, I suspected my prandtl number of hitec 
/// salt was coded wrongly 
/// This ensures things are coded correctly
pub mod thermophsyical_property_checks;


/// in debugging, I suspected dimensions were
/// coded wrongly
pub mod dimension_checks;

/// in debugging, I suspected that heat transfer correlations 
/// calculated the heat transfer coefficient wrongly
/// this is to ensure they are calculated right 
pub mod heat_transfer_correlations_checks;
