use uom::si::f64::*;
use roots::{find_root_brent, SimpleConvergency};
use specific_enthalpy::try_get_h;
use uom::si::available_energy::joule_per_kilogram;
use uom::si::f64::*;
use uom::si::length::nanometer;
use uom::si::mass_density::gram_per_cubic_centimeter;
use uom::si::pressure::atmosphere;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::specific_power::kilowatt_per_kilogram;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::thermal_conductivity::milliwatt_per_meter_kelvin;
use crate::boussinesq_thermophysical_properties::*;
use crate::tuas_lib_error::TuasLibError;
use uom::si::thermodynamic_temperature::kelvin;

/// Wang, X., Lu, Z., Li, Z., Shi, Y., & Xu, H. (2022). 
/// Effect of Zr content on microstructure and hardness of ODS-FeCrAl 
/// alloys. Materials Characterization, 192, 112221.
///
/// based on chromium steel, 7.8 g/cm3
///
pub fn fecral_constant_density() -> Result<MassDensity,TuasLibError> {
    return Ok(MassDensity::new::<gram_per_cubic_centimeter>(7.8));
}

pub fn fecral_surf_roughness() -> Length {
    todo!()
}


/// Field, K. G., Snead, M. A., Yamamoto, Y., & Terrani, K. A. (2017). 
/// Handbook on the material properties of FeCrAl alloys for nuclear 
/// power production applications. Nuclear Technology Research and 
/// Development.
///
/// While cp is temperature dependent, this provides a rough 
/// estimate in for simpler approximate calculations 
///
/// However, it is reasonably accurate in the range 700-1000C for  
/// fecral undergoing cooling. There is hysteresis apparently
pub fn fecral_const_cp_estimate() -> Result<SpecificHeatCapacity, TuasLibError> {
    return Ok(SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(0.8));
}

