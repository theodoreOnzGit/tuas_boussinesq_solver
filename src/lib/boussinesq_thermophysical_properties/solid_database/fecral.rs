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

// Wang, X., Lu, Z., Li, Z., Shi, Y., & Xu, H. (2022). 
// Effect of Zr content on microstructure and hardness of ODS-FeCrAl 
// alloys. Materials Characterization, 192, 112221.
//
// based on chromium steel, 7.8 g/cm3
//
pub fn fecral_constant_density() -> Result<MassDensity,TuasLibError> {
    return Ok(MassDensity::new::<gram_per_cubic_centimeter>(7.8));
}
