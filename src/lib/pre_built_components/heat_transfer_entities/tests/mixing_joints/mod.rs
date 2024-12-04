/// this test checks if FluidArrays can form adiabatic mixing joints 
/// with single cvs 
///
/// so let's say, two pipes with 0.05 kg/s of therminol vp1 
/// flowing into a mixing joint (singleCV)
///
/// one is 50C, one is 100C
///
/// and 0.10 kg/s flows out. it should be 75 C is adiabatically mixed
///
/// flows are positive (forward)
pub mod fwd_flow;


/// this test checks if FluidArrays can form adiabatic mixing joints 
/// with single cvs 
///
/// so let's say, two pipes with 0.05 kg/s of therminol vp1 
/// flowing into a mixing joint (singleCV)
///
/// one is 50C, one is 100C
///
/// and 0.10 kg/s flows out. it should be 75 C is adiabatically mixed
///
/// flows are negative value (reverse)
pub mod reverse_flow;
