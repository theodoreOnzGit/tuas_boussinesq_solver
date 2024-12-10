#[cfg(test)]
/// Contains tests for simple lumped heat capacitance
mod lumped_heat_capacitance;

#[cfg(test)]
/// Contains tests 
/// for automatic time step adjustment
/// using simple lumped heat capacitance as an example
mod automatic_timestep_using_lumped_capacitance;

#[cfg(test)]
/// Contains tests for conjugate heat transfer 
mod conjugate_heat_transfer_tests;

#[cfg(test)]
/// Contains tests for a semi infinite medium in 1D 
mod semi_infinite_one_dimension_transient_conduction;

#[cfg(test)]
/// contains test for a mixing joint, that means two control volumes 
/// carrying fluid come to a single control vol, and 
/// the outlet temperature should be at a correct temperature
pub mod mixing_joint;
