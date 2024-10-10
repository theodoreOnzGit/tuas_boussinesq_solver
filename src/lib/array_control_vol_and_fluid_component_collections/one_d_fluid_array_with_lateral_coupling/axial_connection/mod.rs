// for axial connections, we can connect heat transfer entities to it 
// or away from it,
//
// these could be boundary conditions or other control volumes
//
// in my original code, these are quite well abstracted as all you see 
// is connection to new heat transfer entities


/// the baseline for all interactions with other array cvs 
/// is the interaction with single cvs and bcs 
/// this module takes care of the interactions with single cvs
pub mod interaction_with_single_cv;

/// the baseline for all interactions with other array cvs 
/// is the interaction with single cvs and bcs 
/// this module takes care of the interactions with single cvs
pub mod interaction_with_bc;

/// this module takes care of the interactions with other array cvs 
/// both solid arrays and fluid arrays
pub mod interaction_with_array_cv;


