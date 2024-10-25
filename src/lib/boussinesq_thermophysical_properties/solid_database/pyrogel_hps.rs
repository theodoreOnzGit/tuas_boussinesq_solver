use peroxide::fuga::{CubicSpline, Spline};
use uom::si::f64::*;
use uom::si::length::{millimeter, nanometer};
use uom::si::mass_density::gram_per_cubic_centimeter;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::temperature_interval::degree_celsius as degc_interval;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::thermal_conductivity::{milliwatt_per_meter_kelvin, watt_per_meter_kelvin};
use crate::boussinesq_thermophysical_properties::*;
use crate::tuas_lib_error::TuasLibError;
use uom::si::thermodynamic_temperature::kelvin;

/// Based on:
/// https://www.distributioninternational.com/ASSETS/DOCUMENTS/ITEMS/EN/PYBT10HA_SS.pdf
///
///
/// 0.20 g/cc density (g/cc is grams per cubic centimeter)
///
#[inline]
pub fn pyrogel_hps_density() -> Result<MassDensity,TuasLibError> {
    return Ok(MassDensity::new::<gram_per_cubic_centimeter>(0.20));
}


/// For Pyrogel HPS specficially, I don't see any surface roughness 
/// data in literature.
///
///
/// But since Pyrogel HPS is a silica aerogel, I'll use the silica 
/// aerogel surface roughness as a ballpark estimate 
///
/// Based on:
/// Mahadik, D. B., Venkateswara Rao, A., Parale, V. G., Kavale, M. S., 
/// Wagh, P. B., Ingale, S. V., & Gupta, S. C. (2011). Effect of surface 
/// composition and roughness on the apparent surface free energy of 
/// silica aerogel materials. Applied Physics Letters, 99(10).
///
/// Paper mentioned 1150–1450 nm
///
/// I'll just use 1500 nm as an estimate
///
///
pub fn pyrogel_hps_surf_roughness() -> Length {
    return Length::new::<nanometer>(1500.0);
}

/// Most information comes from:
///
/// Kovács, Z., Csík, A., & Lakatos, Á. (2023). 
/// Thermal stability investigations of different 
/// aerogel insulation materials at elevated temperature.
/// Thermal Science and Engineering Progress, 42, 101906.
///
/// work in progress though. still need to decipher the paper
///
/// Cassel, R. B. (2001). How Tzero™ Technology Improves DSC 
/// Performance Part III: The Measurement of Specific Heat Capacity. 
/// TA Instruments: New Castle, DE, USA.
///
/// for DSC:
///
/// dQ/dt (watts) = cp * beta * sample_mass
/// dQ/dt (watts) * 1/sample_mass = cp * beta 
///
/// beta is heating rate (kelvin or degC per minute)
///
///
#[inline]
pub fn pryogel_hps_specific_heat_capacity(
    _temperature: ThermodynamicTemperature) -> SpecificHeatCapacity {

    todo!()
}

/// returns thermal conductivity of pyrogel hps
/// cited from:
/// https://www.distributioninternational.com/ASSETS/DOCUMENTS/ITEMS/EN/PYBT10HA_SS.pdf
///
/// This is from aspen, tested with ASTM C177 at 2 psi compressive load
#[inline]
pub fn fiberglass_thermal_conductivity_commercial_factsheet_spline(
    temperature: ThermodynamicTemperature) -> Result<ThermalConductivity,TuasLibError> {

    range_check(
        &Material::Solid(SolidMaterial::Fiberglass),
        temperature, 
        ThermodynamicTemperature::new::<degree_celsius>(650.0), 
        ThermodynamicTemperature::new::<degree_celsius>(0.0))?;

    let temperature_value_degc: f64 = temperature.get::<degree_celsius>();
    // here we use a cubic spline to interpolate the values
    // it's a little calculation heavy, but don't really care now
    let thermal_cond_temperature_values_degc = c!(
        0.0, 100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 650.0
        );
    let thermal_conductivity_values_milliwatt_per_meter_kelvin = c!(
        20.0, 24.0, 28.0, 33.0, 40.0, 49.0, 62.0, 69.0
        );

    let s = CubicSpline::from_nodes(&thermal_cond_temperature_values_degc, 
        &thermal_conductivity_values_milliwatt_per_meter_kelvin);

    let fiberglass_thermal_conductivity_value_milliwatt_per_meter_kelvin = s.unwrap().eval(
        temperature_value_degc);

    return Ok(ThermalConductivity::new::<milliwatt_per_meter_kelvin>(
        fiberglass_thermal_conductivity_value_milliwatt_per_meter_kelvin));
}
