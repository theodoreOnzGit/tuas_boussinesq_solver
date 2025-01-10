/// this gives some basic tests to see what is optimal for parasitic heat loss
pub mod preliminaries;

/// For the InsulatedFluidComponents, and porous media counterparts
/// one useful verification test is the log-mean-temp-diff test.
/// 
/// Though this can be repeated for the noninsulated versions too
/// 
/// this is because these are usually meant to simulate heat loss.
/// Different lengths of pipes can be used.
/// 
/// the theory is, for a fluid flowing through a pipe with constant ambient temp
/// 
/// Q = UA * LMTD
/// 
/// m cp * (Tout - Tin) = UA * LMTD
/// m cp * (Tout - Tin) = UA * ( (Tout - Tamb) - (Tin - Tamb) ) / ( ln(Tout - Tamb) - ln(Tin - Tamb) )
/// m cp * (Tout - Tin) = UA * ( Tout - Tin ) / ( ln(Tout - Tamb) - ln(Tin - Tamb) )
/// m cp = UA / ( ln(Tout - Tamb) - ln(Tin - Tamb) )
/// 
/// ln ( (Tout - Tamb) / (Tin - Tamb) ) = UA / (m cp)
/// 
/// We thereby have an analytical expression for Tout
/// 
/// Tout - Tamb = (Tin - Tamb) exp ( (UA)/( m cp) )
/// Tout = (Tin - Tamb) exp ( (UA)/( m cp) ) + Tamb
/// 
/// This provides a simple analytical solution for which to test the 
/// components
///
/// we vary lengths from 1 to 10m here
///
/// all agree to within 0.3K.
/// But I wonder why it's slightly off. 
/// Mesh refinement doesn't help 
/// and extra time doesn't help to reach a steady state either
///
pub mod length_tests;

/// there is quite some discrepancy between 
/// the analytical solution and the calculated solution
/// this debugging process is meant to try and resolve that
pub mod axial_conduction_verification;
