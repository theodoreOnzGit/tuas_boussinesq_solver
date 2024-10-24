use peroxide::fuga::{CubicSpline, Spline};
use uom::si::f64::*;
use uom::si::length::millimeter;
use uom::si::mass_density::gram_per_cubic_centimeter;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::temperature_interval::degree_celsius as degc_interval;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::thermal_conductivity::{milliwatt_per_meter_kelvin, watt_per_meter_kelvin};
use crate::boussinesq_thermophysical_properties::*;
use crate::tuas_lib_error::TuasLibError;
use uom::si::thermodynamic_temperature::kelvin;

/// https://www.distributioninternational.com/ASSETS/DOCUMENTS/ITEMS/EN/PYBT10HA_SS.pdf
///
///
/// 0.20 g/cc density (g/cc is grams per cubic centimeter)
///
#[inline]
pub fn pyrogel_hps_density() -> Result<MassDensity,TuasLibError> {
    return Ok(MassDensity::new::<gram_per_cubic_centimeter>(0.20));
}


/// To be implemented. not sure what this is
pub fn pyrogel_hps_surf_roughness() -> Length {
    todo!()
}

/// Most information comes from:
///
/// Kovács, Z., Csík, A., & Lakatos, Á. (2023). 
/// Thermal stability investigations of different 
/// aerogel insulation materials at elevated temperature.
/// Thermal Science and Engineering Progress, 42, 101906.
///
/// work in progress though. still need to decipher the paper
#[inline]
pub fn pryogel_hps_specific_heat_capacity(
    _temperature: ThermodynamicTemperature) -> SpecificHeatCapacity {

    todo!()
}

/// returns thermal conductivity of pyrogel hps
/// cited from:
/// https://www.distributioninternational.com/ASSETS/DOCUMENTS/ITEMS/EN/PYBT10HA_SS.pdf
///
/// This is from aspen
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
