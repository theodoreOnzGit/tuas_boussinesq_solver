
/// contains matrix calculations specific to fluid nodes
/// arranged in a 1D array
///
/// These are standalone, and not abstracted under an arrayCV
/// struct, they only use SingleCVNode structs and representative 
/// arrays to represent an array of control volumes
pub mod standalone_fluid_nodes;

/// contains matrix calculations specific to solid nodes 
/// these are meant to represent the "shell" of the pipe 
/// or any kind of solid material in the pipe
///
/// These are standalone, and not abstracted under an arrayCV
/// struct, they only use SingleCVNode structs and representative 
/// arrays to represent an array of control volumes
pub mod standalone_solid_nodes;

/// 
pub mod conductance_array_functions;

/// contains a full struct which abstracts away calculation details 
///
/// this is relevant for one dimension cartesian (x,y,z) coordinates
/// you can't really couple these arrays laterally though
pub mod one_dimension_cartesian_conducting_medium;


/// contains a full struct which abstracts away calculation details 
/// 1 dimensional solid arrays
///
/// this is relevant for one dimension cartesian (x,y,z) coordinates
/// except that you can couple these arrays laterally to form a 2D or 
/// 3D lattice
pub mod one_d_solid_array_with_lateral_coupling;

/// contains a full struct which abstracts away calculation details 
/// 1 dimensional fluid arrays
///
/// this is relevant for one dimension cartesian (x,y,z) coordinates
/// except that you can couple these arrays laterally to form a 2D or 
/// 3D lattice
pub mod one_d_fluid_array_with_lateral_coupling;

/// contains code for calculating pressure drop and mass flowrates over 
/// pipes in series or parallel
pub mod fluid_component_collection;
