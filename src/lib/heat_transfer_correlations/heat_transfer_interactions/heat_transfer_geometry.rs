use crate::control_volume_dimensions::*;
use crate::boussinesq_thermophysical_properties::Material;


/// for a curved surface, be it cylindrical or spherical,
/// this enum indicates whether the fluid is on the inside (lower radius)
/// or on the outside (larger radius)
///
/// -----------------------------------------> r
/// fluid               ||                  solid
///
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum CylindricalAndSphericalSolidFluidArrangement {
    /// indicates that fluid in the inner side of a curved shell 
    ///
    /// -----------------------------------------> r
    /// fluid               ||                  solid
    ///
    FluidOnInnerSurfaceOfSolidShell,
    /// indicates that fluid in the inner side of a curved shell 
    ///
    /// -----------------------------------------> r
    /// solid               ||                  fluid
    ///
    FluidOnOuterSurfaceOfSolidShell
}

/// here we have a struct for dual Cartesian Thermal conduction 
/// in three dimensions
/// on
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct DataDualCartesianThermalConductanceThreeDimension{
    /// material for first cv
    pub material_1: Material,
    /// material for second cv
    pub material_2: Material,
    /// cross sectional area at interface
    pub xs_area: CrossSectionalArea,
    /// thickness of first cv
    pub thickness_1: XThicknessThermalConduction,
    /// thickness of second cv
    pub thickness_2: XThicknessThermalConduction,

}
