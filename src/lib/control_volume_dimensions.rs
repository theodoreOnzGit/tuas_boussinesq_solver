use uom::si::f64::*;

/// XThicknessThermalConduction is essentially a struct containing 
/// one length describing a thickness in cartesian coordinates 
/// for thermal conduction.
///
/// This type is meant for an input for various enums and functions 
/// in this crate.
///
/// It is meant to guide the user so that they know what the 
/// length inputs represents.
///
/// ```rust
///
/// use uom::si::length::meter;
/// use uom::si::f64::*;
/// use thermal_hydraulics_rs::tuas_boussinesq_solver::control_volume_dimensions
/// ::XThicknessThermalConduction;
///
/// // let's say you have a thickness of 0.5 which you want to describe
///
/// let thickness_of_wall = Length::new::<meter>(0.5);
///
/// // we first need to convert it into an XThicknessThermalConduction
/// // type first
/// let wall_thickness_input = XThicknessThermalConduction::from(
/// thickness_of_wall);
///
/// // to convert it back into a length type, we use the into() method 
///
/// let thickness_wall_for_calculation: Length = 
/// wall_thickness_input.into();
/// 
/// // both these are the same
/// assert_eq!(thickness_of_wall, thickness_wall_for_calculation);
/// ```
/// 
/// 
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct XThicknessThermalConduction {
    thickness: Length,
}

impl From<Length> for XThicknessThermalConduction {
    fn from(thickness: Length) -> Self{
        Self { thickness }
    }
}

impl Into<Length> for XThicknessThermalConduction {
    fn into(self) -> Length {
        self.thickness
    }
}

/// This represents a thickness for radial conduction for 
/// cylindrical shell layers
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct RadialCylindricalThicknessThermalConduction {
    thickness: Length,
}

impl From<Length> for RadialCylindricalThicknessThermalConduction {
    fn from(thickness: Length) -> Self{
        Self { thickness }
    }
}

impl Into<Length> for RadialCylindricalThicknessThermalConduction {
    fn into(self) -> Length {
        self.thickness
    }
}

/// This represents an inner diameter  for radial conduction 
/// for spherical and cylindrical shell 
/// layers
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct InnerDiameterThermalConduction {
    od: Length,
}

impl From<Length> for InnerDiameterThermalConduction {
    fn from(od: Length) -> Self{
        Self { od }
    }
}

impl Into<Length> for InnerDiameterThermalConduction {
    fn into(self) -> Length {
        self.od
    }
}

/// This represents an outer diameter 
/// for radial conduction for spherical and Cylindrical shell 
/// layers
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct OuterDiameterThermalConduction {
    od: Length,
}

impl From<Length> for OuterDiameterThermalConduction {
    fn from(od: Length) -> Self{
        Self { od }
    }
}

impl Into<Length> for OuterDiameterThermalConduction {
    fn into(self) -> Length {
        self.od
    }
}

/// This represents an tube length 
/// ie. axial length for a cylindrical body
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct CylinderLengthThermalConduction {
    cylinder_length: Length,
}

impl From<Length> for CylinderLengthThermalConduction {
    fn from(cylinder_length: Length) -> Self{
        Self { cylinder_length }
    }
}

impl Into<Length> for CylinderLengthThermalConduction {
    fn into(self) -> Length {
        self.cylinder_length
    }
}


/// This represents an Cross Sectional Area
/// ie. axial length for a cylindrical body
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct CrossSectionalArea {
    xs_area: Area,
}

impl From<Area> for CrossSectionalArea {
    fn from(xs_area: Area) -> Self{
        Self { xs_area }
    }
}

impl Into<Area> for CrossSectionalArea {
    fn into(self) -> Area {
        self.xs_area
    }
}

/// This represents an Surface Area
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct SurfaceArea {
    surf_area: Area,
}

impl From<Area> for SurfaceArea {
    fn from(surf_area: Area) -> Self{
        Self { surf_area }
    }
}

impl Into<Area> for SurfaceArea {
    fn into(self) -> Area {
        self.surf_area
    }
}

/// for 1D calculations, we need to calculate conductance as well,
/// but there is no area, hence, we have to use a unit area to calculate 
/// the conductance 
pub const UNIT_AREA_SQ_METER_FOR_ONE_DIMENSIONAL_CALCS: f64 = 1.0;
