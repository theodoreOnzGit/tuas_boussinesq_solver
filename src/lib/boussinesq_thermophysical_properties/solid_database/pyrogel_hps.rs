use peroxide::fuga::{CubicSpline, Spline};
use uom::si::f64::*;
use uom::si::length::{millimeter, nanometer};
use uom::si::mass_density::gram_per_cubic_centimeter;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::specific_power::kilowatt_per_kilogram;
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
/// dQ/dt * 1/sample_mass (watts/gram) = cp * beta 
///
/// beta is heating rate (kelvin or degC per minute)
///
/// Now, based on the dsc measurements, cp of around 1500 - 2200 J/(kg K) 
/// can be expected after crystallisation. This is just a ballpark estimate
#[inline]
pub fn pryogel_hps_specific_heat_capacity(
    temperature: ThermodynamicTemperature) -> SpecificHeatCapacity {

    range_check(
        &Material::Solid(SolidMaterial::Fiberglass),
        temperature, 
        ThermodynamicTemperature::new::<degree_celsius>(10.0), 
        ThermodynamicTemperature::new::<degree_celsius>(650.0)).unwrap();


    // probably want to not use splines all the time as well, can 
    // be quite computationally expensive

    todo!("change material type to pyrogel");
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
/// dQ/dt * 1/sample_mass (watts/gram) = cp * beta 
///
/// beta is heating rate (kelvin or degC per minute)
///
///
#[inline]
pub fn pryogel_hps_specific_heat_capacity_spline_low_temp(
    temperature: ThermodynamicTemperature) -> 
Result<SpecificHeatCapacity,TuasLibError> {

    range_check(
        &Material::Solid(SolidMaterial::Fiberglass),
        temperature, 
        ThermodynamicTemperature::new::<degree_celsius>(39.819), 
        ThermodynamicTemperature::new::<degree_celsius>(9.88))?;
    let pyrogel_cp_temperature_values_degc = c!(
        9.883, 24.019, 39.819);
    let pyrogel_cp_values_joule_per_kg_kelvin = c!(
        883.261, 898.311, 983.814);
    let s = CubicSpline::from_nodes(&pyrogel_cp_temperature_values_degc, 
        &pyrogel_cp_values_joule_per_kg_kelvin);

    let temperature_value_degc: f64 = temperature.get::<degree_celsius>();
    let pyrogel_generic_cp_joule_per_kg_kelvin = 
        s.unwrap().eval(temperature_value_degc);
    todo!("change material type to Pyrogel");

    return Ok(SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(
        pyrogel_generic_cp_joule_per_kg_kelvin));
}

/// Most information comes from:
///
/// Kovács, Z., Csík, A., & Lakatos, Á. (2023). 
/// Thermal stability investigations of different 
/// aerogel insulation materials at elevated temperature.
/// Thermal Science and Engineering Progress, 42, 101906.
///
/// Note that this pyrogel information is for ground pyrogel,
/// which then destroys the structure of the pyrogel and may change its 
/// thermal conductivity. Moreover, crystallisation occurs, which changes 
/// its heat capacity too.
///
#[inline]
pub fn ground_pyrogel_hps_dsc_spline_data(temperature: ThermodynamicTemperature,) 
    -> Result<SpecificPower, TuasLibError> {

    range_check(
        &Material::Solid(SolidMaterial::Fiberglass),
        temperature, 
        ThermodynamicTemperature::new::<degree_celsius>(327.0), 
        ThermodynamicTemperature::new::<degree_celsius>(35.0))?;
    let specific_power_temperature_values_degc = c!(35.426,
        42.085,
        50.609,
        59.132,
        74.581,
        88.432,
        99.619,
        123.592,
        136.91,
        151.294,
        166.21,
        178.463,
        190.183,
        200.837,
        215.221,
        225.875,
        234.399,
        239.726,
        243.455,
        248.25,
        251.446,
        253.577,
        257.839,
        259.97,
        262.633,
        265.83,
        267.428,
        271.157,
        279.148,
        304.186,
        326.294
        );
    let specific_power_values_milliwatts_per_milligram = c!(
        0.261,
        0.274,
        0.285,
        0.292,
        0.293,
        0.291,
        0.287,
        0.28,
        0.278,
        0.277,
        0.282,
        0.285,
        0.285,
        0.285,
        0.291,
        0.302,
        0.32,
        0.338,
        0.351,
        0.365,
        0.373,
        0.374,
        0.369,
        0.361,
        0.345,
        0.322,
        0.303,
        0.296,
        0.29,
        0.286,
        0.283
            );

    let s = CubicSpline::from_nodes(&specific_power_temperature_values_degc, 
        &specific_power_values_milliwatts_per_milligram);

    let temperature_value_degc: f64 = temperature.get::<degree_celsius>();
    let pyrogel_generic_dsc_milliwatt_per_milligram = 
        s.unwrap().eval(temperature_value_degc);
    todo!("change material type to Pyrogel");

    return Ok(SpecificPower::new::<kilowatt_per_kilogram>(
        pyrogel_generic_dsc_milliwatt_per_milligram));
}

/// returns thermal conductivity of pyrogel hps
/// cited from:
/// https://www.distributioninternational.com/ASSETS/DOCUMENTS/ITEMS/EN/PYBT10HA_SS.pdf
///
/// This is from aspen, tested with ASTM C177 at 2 psi compressive load
#[inline]
pub fn pyrogel_thermal_conductivity_commercial_factsheet_spline(
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

    let pyrogel_hps_thermal_conductivity_value_milliwatt_per_meter_kelvin = s.unwrap().eval(
        temperature_value_degc);
    todo!("change material type to Pyrogel");

    return Ok(ThermalConductivity::new::<milliwatt_per_meter_kelvin>(
        pyrogel_hps_thermal_conductivity_value_milliwatt_per_meter_kelvin));
}


