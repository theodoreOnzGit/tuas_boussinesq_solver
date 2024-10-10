use uom::si::f64::*;

use crate::thermal_hydraulics_error::ThermalHydraulicsLibError;

use super::LiquidMaterial;
use super::Material;
use super::SolidMaterial;

/// contains specific enthalpy data for all materials
pub mod enthalpy_data;
use enthalpy_data::*;

// contains functions to map enthalpy to a temperature 
mod temperature_from_specific_enthalpy;
use temperature_from_specific_enthalpy::*;

/// returns specific enthaply for a given material 
/// specific_enthalpy is defined as 0 for 0 degree_celsius
/// for any material, that is 273.15 K
///
/// ```rust 
/// use uom::si::f64::*;
/// use uom::si::specific_heat_capacity::{joule_per_kilogram_kelvin,
/// joule_per_gram_degree_celsius};
/// use uom::si::thermodynamic_temperature::kelvin;
/// use uom::si::temperature_interval::degree_celsius;
/// use tuas_boussinesq_solver::boussinesq_thermophysical_properties::
/// SolidMaterial::{SteelSS304L,Copper};
/// use tuas_boussinesq_solver::boussinesq_thermophysical_properties::
/// Material;
/// use tuas_boussinesq_solver::boussinesq_thermophysical_properties::
/// specific_enthalpy::try_get_h;
///
/// use uom::si::pressure::atmosphere;
///
/// let steel = Material::Solid(SteelSS304L);
/// let steel_temp = ThermodynamicTemperature::new::<kelvin>(273.15);
/// let pressure = Pressure::new::<atmosphere>(1.0);
///
/// // enthalpy should be zero at 273.15 K
///
/// let steel_enthalpy_273_15_kelvin = 
/// try_get_h(steel, steel_temp, pressure);
///
/// approx::assert_relative_eq!(
///     0.0,
///     steel_enthalpy_273_15_kelvin.unwrap().value,
///     max_relative=0.045);
/// 
/// // we can also calculate enthalpy change of copper 
/// // from 375K to 425K
/// let test_temperature_1 = ThermodynamicTemperature::new::
/// <kelvin>(375.0);
/// let test_temperature_2 = ThermodynamicTemperature::new::
/// <kelvin>(425.0);
///
/// let copper = Material::Solid(Copper);
///
/// let copper_enthalpy_change = 
/// try_get_h(copper, test_temperature_2, pressure).unwrap()
/// - try_get_h(copper, test_temperature_1, pressure).unwrap();
///
/// // http://hyperphysics.phy-astr.gsu.edu/hbase/Tables/sphtt.html
/// // https://www.engineeringtoolbox.com/specific-heat-metals-d_152.html
/// // copper at 20C has heat capacity of 
/// // 0.386 J/(g K)
/// // going to use this to estimate a ballpark figure to find enthalpy 
/// // h = cp(T2 - T1)
/// 
/// // we can't usually subtract thermodynamic temperatures from each 
/// // other, we need a termpature interval
/// // 
///
/// let cp_copper_20_c = 
/// SpecificHeatCapacity::new::<joule_per_gram_degree_celsius>(0.386);
/// 
/// let temperature_difference = 
/// TemperatureInterval::new::<degree_celsius>(
/// test_temperature_2.value - test_temperature_1.value);
///
/// let specific_enthalpy_ballpark = 
/// cp_copper_20_c * temperature_difference;
/// 
/// // the ballpark value is 19300 J/kg
/// approx::assert_relative_eq!(
///     specific_enthalpy_ballpark.value,
///     19300.0,
///     max_relative=0.0001);
///
/// // it's less than 4% different from the ballpark value
/// // This means the copper enthalpy change should be quite reasonable
///
/// approx::assert_relative_eq!(
///     specific_enthalpy_ballpark.value,
///     copper_enthalpy_change.value,
///     max_relative=0.04);
///
/// ``` 
pub fn try_get_h(material: Material, 
    temperature: ThermodynamicTemperature,
    _pressure: Pressure) -> Result<AvailableEnergy, ThermalHydraulicsLibError> {

    let specific_enthalpy: AvailableEnergy = match material {
        Material::Solid(_) => solid_specific_enthalpy(material, temperature),
        Material::Liquid(_) => liquid_specific_enthalpy(material, temperature)
    };

    return Ok(specific_enthalpy);
}

/// This function allows you to obtain ThermodynamicTemperature 
/// from AvailableEnergy (a.k.a specific enthalpy) of a material 
/// as long as we have the material in the database
///
/// example: 
/// ```rust
/// use uom::si::f64::*;
/// use uom::si::specific_heat_capacity::{joule_per_kilogram_kelvin,
/// joule_per_gram_degree_celsius};
/// use uom::si::thermodynamic_temperature::{kelvin,degree_celsius};
/// use uom::si::temperature_interval::degree_celsius as 
/// interval_degree_celsius;
/// use tuas_boussinesq_solver::boussinesq_thermophysical_properties::
/// SolidMaterial::{SteelSS304L,Copper};
/// use tuas_boussinesq_solver::boussinesq_thermophysical_properties::
/// Material;
/// use tuas_boussinesq_solver::boussinesq_thermophysical_properties::
/// specific_enthalpy::try_get_h;
///
/// use uom::si::pressure::atmosphere;
///
/// use tuas_boussinesq_solver::boussinesq_thermophysical_properties::
/// specific_enthalpy::try_get_temperature_from_h;
///
///
/// // let's get steel at 20 degree_celsius
///
/// let steel = Material::Solid(SteelSS304L);
/// let steel_temp = ThermodynamicTemperature::new::<degree_celsius>(20.0);
/// let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
///
/// let enthalpy_spline_zweibaum = try_get_h(
///     steel,steel_temp,atmospheric_pressure).unwrap();
///
/// // now this enthalpy value is about 
/// // 9050 J/kg +/- 0.5 J/kg 
/// // the epsilon here is just round off error 
/// // NOT measurement uncertainty or anything else
/// let round_off_error = 0.5;
///
/// approx::assert_abs_diff_eq!(
///     enthalpy_spline_zweibaum.value,
///     9050_f64,
///     epsilon=round_off_error);
///
/// // let's use this enthalpy value to get a ThermodynamicTemperature
///
/// let steel_temperature_test = 
/// try_get_temperature_from_h(steel,
/// enthalpy_spline_zweibaum,
/// atmospheric_pressure).unwrap();
/// 
/// // this should get back 20 degrees C with 0.001 degree_celsius of 
/// // error at most
/// approx::assert_abs_diff_eq!(
///     steel_temperature_test.get::<degree_celsius>(),
///     20_f64,
///     epsilon=0.001);
/// ```
pub fn try_get_temperature_from_h(material: Material, 
    material_enthalpy: AvailableEnergy,
    _pressure: Pressure) -> Result<ThermodynamicTemperature, ThermalHydraulicsLibError> {

    let specific_enthalpy: ThermodynamicTemperature = match material {
        Material::Solid(_) => 
            get_solid_temperature_from_specific_enthalpy(
                material, material_enthalpy),
        Material::Liquid(_) => 
            get_liquid_temperature_from_specific_enthalpy(
                material, material_enthalpy)
    };

    return Ok(specific_enthalpy);
}


