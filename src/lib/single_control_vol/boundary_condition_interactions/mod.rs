/// for advection calculations with heat flux or heat addition BC,
/// the temperature of flows flowing in and out of the BC will be 
/// determined by that of the control volume
///
/// it will be the same temperature as that of the control volume 
/// at that current timestep
///
/// this will be quite similar to how OpenFOAM treats inflows and outflows 
/// at zero gradient BCs
pub mod advection_to_bcs;



/// calculates a conductance interaction between the constant 
/// temperature bc and cv
///
/// for conductance, orientation of bc and cv does not usually matter
pub mod conductance_to_bcs;

/// calculates a conductance interaction between the constant 
/// temperature bc and cv
///
/// for conductance, orientation of bc and cv does not usually matter
pub mod constant_heat_addition_to_bcs;
