use peroxide::fuga::{CubicSpline, Spline};
use uom::si::f64::*;
use uom::si::length::millimeter;
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::thermal_conductivity::watt_per_meter_kelvin;
use crate::boussinesq_thermophysical_properties::*;
use crate::tuas_lib_error::TuasLibError;
use uom::si::thermodynamic_temperature::kelvin;

/// density ranges not quite given in original text 
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code validation 
/// using the compact integral effects test (CIET) experimental data. 
/// No. ANL/NSE-19/11. 
/// Argonne National Lab.(ANL), Argonne, IL (United States), 2019.
#[inline]
pub fn fiberglass_density() -> Result<MassDensity,TuasLibError> {
    return Ok(MassDensity::new::<kilogram_per_cubic_meter>(20.0));
}

/// Value from: Perry's chemical Engineering handbook 
/// 8th edition Table 6-1 
/// generic value for drawn tubing
/// Perry, R. H., & DW, G. (2007). 
/// Perry’s chemical engineers’ handbook, 
/// 8th illustrated ed. New York: McGraw-Hill.
pub fn fiberglass_surf_roughness() -> Length {
    Length::new::<millimeter>(0.00152)
}
/// returns thermal conductivity of fiberglass
/// cited from:
/// Zou, L., Hu, R., & Charpentier, A. (2019). SAM code 
/// validation using the compact integral effects test (CIET) experimental 
/// data (No. ANL/NSE-19/11). Argonne National 
/// Lab.(ANL), Argonne, IL (United States).
#[inline]
pub fn fiberglass_specific_heat_capacity(
    _temperature: ThermodynamicTemperature) -> SpecificHeatCapacity {

    return SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(
        844.0);
}
/// returns thermal conductivity of fiberglass
/// cited from:
/// Zou, L., Hu, R., & Charpentier, A. (2019). SAM code 
/// validation using the compact integral effects test (CIET) experimental 
/// data (No. ANL/NSE-19/11). Argonne National 
/// Lab.(ANL), Argonne, IL (United States).
#[inline]
pub fn fiberglass_thermal_conductivity_zou_zweibaum_spline(
    temperature: ThermodynamicTemperature) -> Result<ThermalConductivity,TuasLibError> {

    range_check(
        &Material::Solid(SolidMaterial::Fiberglass),
        temperature, 
        ThermodynamicTemperature::new::<kelvin>(600.0), 
        ThermodynamicTemperature::new::<kelvin>(250.0))?;

    let temperature_value_kelvin: f64 = temperature.get::<kelvin>();
    // here we use a cubic spline to interpolate the values
    // it's a little calculation heavy, but don't really care now
    let thermal_cond_temperature_values_kelvin = c!(250.0, 293.15, 350.0, 
        400.0, 500.0, 600.0);
    let thermal_conductivity_values_watt_per_meter_kelin = c!(0.028616,
        0.033060, 0.038916, 0.044066, 0.054366, 0.064666);

    let s = CubicSpline::from_nodes(&thermal_cond_temperature_values_kelvin, 
        &thermal_conductivity_values_watt_per_meter_kelin);

    let fiberglass_thermal_conductivity_value = s.unwrap().eval(
        temperature_value_kelvin);

    return Ok(ThermalConductivity::new::<watt_per_meter_kelvin>(
        fiberglass_thermal_conductivity_value));
}


#[inline]
/// fiberglass max temp 
pub fn max_temp_fiberglass_zou_zweibaum_spline() -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(600.0)

}
#[inline]
/// fiberglass min temp 
pub fn min_temp_fiberglass_zou_zweibaum_spline() -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(250.0)
}
