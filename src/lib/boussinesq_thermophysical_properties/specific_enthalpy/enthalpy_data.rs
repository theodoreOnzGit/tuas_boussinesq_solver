use uom::si::available_energy::joule_per_gram;
use uom::si::f64::*;
use uom::si::available_energy::joule_per_kilogram;
use uom::si::thermodynamic_temperature::{degree_celsius,kelvin};

use crate::boussinesq_thermophysical_properties::liquid_database::flibe::get_flibe_specific_enthalpy;
use crate::boussinesq_thermophysical_properties::liquid_database::flinak::get_flinak_specific_enthalpy;
use crate::boussinesq_thermophysical_properties::liquid_database::yd_325_heat_transfer_oil::get_yd325_specific_enthalpy;
use crate::boussinesq_thermophysical_properties::liquid_database::{self, dowtherm_a};
use crate::boussinesq_thermophysical_properties::liquid_database::hitec_nitrate_salt::get_hitec_specific_enthalpy;
use crate::boussinesq_thermophysical_properties::solid_database::custom_solid_material;

use super::LiquidMaterial;
use super::Material;
use super::SolidMaterial;
use super::SolidMaterial::*;
use super::LiquidMaterial::*;

use peroxide::prelude::*;
// should the material happen to be a solid, use this function
//
// probably should have a temperature range checker in 
// future
//
// 
// pub(in crate::boussinesq_thermophysical_properties) 
// here only makes it accessible to the 
// specific_enthalpy/mod.rs 
// nothing else
pub(in crate::boussinesq_thermophysical_properties) 
fn solid_specific_enthalpy(material: Material,
    solid_temp: ThermodynamicTemperature) -> AvailableEnergy {
    
    // first match the enum

    let solid_material: SolidMaterial = match material {
        Material::Solid(SteelSS304L) => SteelSS304L,
        Material::Solid(Fiberglass) => Fiberglass,
        Material::Solid(Copper) => Copper,
        Material::Solid(CustomSolid((low_bound_temp,high_bound_temp),cp,k,rho,roughness)) => {
            CustomSolid((low_bound_temp,high_bound_temp), cp, k, rho,roughness)
        },
        Material::Liquid(_) => panic!("solid_specific_enthalpy, use SolidMaterial enums only")
    };

    let specific_enthalpy: AvailableEnergy = match solid_material {
        Fiberglass => fiberglass_specific_enthalpy(solid_temp) ,
        SteelSS304L => steel_304_l_spline_specific_enthalpy_ciet_zweibaum(solid_temp),
        Copper => copper_specific_enthalpy(solid_temp),
        CustomSolid((low_bound_temp,high_bound_temp),cp_fn,_k,_rho_fn,_roughness) => {
            custom_solid_material::get_custom_solid_enthalpy(
                solid_temp, 
                cp_fn, 
                high_bound_temp, 
                low_bound_temp).unwrap()
        },
    };

    return specific_enthalpy;


}

// should the material happen to be a liquid, use this function
// pub(in crate::boussinesq_thermophysical_properties)
// here only makes it accessible to the 
// specific_enthalpy/mod.rs 
// nothing else
pub(in crate::boussinesq_thermophysical_properties) 
fn liquid_specific_enthalpy(material: Material, 
    fluid_temp: ThermodynamicTemperature) -> AvailableEnergy {

    let liquid_material: LiquidMaterial = match material {
        Material::Liquid(DowthermA) => DowthermA,
        Material::Liquid(TherminolVP1) => TherminolVP1,
        Material::Liquid(HITEC) => HITEC,
        Material::Liquid(YD325) => YD325,
        Material::Liquid(FLiBe) => FLiBe,
        Material::Liquid(FLiNaK) => FLiNaK,
        Material::Liquid(CustomLiquid((low_bound_temp,high_bound_temp),cp,k,mu,rho)) => {
            CustomLiquid((low_bound_temp,high_bound_temp), cp, k, mu, rho)
        },
        Material::Solid(_) => panic!(
        "liquid_specific_enthalpy, use LiquidMaterial enums only")
    };

    let specific_enthalpy: AvailableEnergy = match liquid_material {
        DowthermA => dowtherm_a_specific_enthalpy(fluid_temp),
        TherminolVP1 => dowtherm_a_specific_enthalpy(fluid_temp),
        HITEC => get_hitec_specific_enthalpy(fluid_temp).unwrap(),
        YD325 => get_yd325_specific_enthalpy(fluid_temp).unwrap(),
        FLiBe => get_flibe_specific_enthalpy(fluid_temp).unwrap(),
        FLiNaK => get_flinak_specific_enthalpy(fluid_temp).unwrap(),
        CustomLiquid((low_bound_temp,high_bound_temp), cp_fn, _k, _mu_fn, _rho_fn) => {
            liquid_database::custom_liquid_material
                ::get_custom_fluid_enthalpy(fluid_temp, 
                    cp_fn, 
                    high_bound_temp, 
                    low_bound_temp).unwrap()
        },
    };

    return specific_enthalpy;
}

/// returns specific enthalpy of fiberglass
/// cited from:
/// Zou, L., Hu, R., & Charpentier, A. (2019). SAM code 
/// validation using the compact integral effects test (CIET) experimental 
/// data (No. ANL/NSE-19/11). Argonne National 
/// Lab.(ANL), Argonne, IL (United States).
///
/// specific enthalpy at 273.15 K = 0
///
/// cp = 844 J/(kg K)
/// hence 
/// h = 844 * T - 844 * (273.15)
#[inline]
fn fiberglass_specific_enthalpy(
    temperature: ThermodynamicTemperature) -> AvailableEnergy {

    let specific_enthalpy_value_j_per_kg = 
    844.0 * temperature.get::<degree_celsius>() ;

    return AvailableEnergy::new::<joule_per_kilogram>(
        specific_enthalpy_value_j_per_kg);
}
#[test]
fn fiberglass_enthalpy_test() {

    let fiberglass_temp = ThermodynamicTemperature::new::
        <kelvin>(373.0);


    let fiberglass_reference_enthalpy_value 
    = 844.0 * (fiberglass_temp.get::<kelvin>() - 
        273.15);

    let fiberglass_enthalpy = 
    fiberglass_specific_enthalpy(fiberglass_temp);

    approx::assert_abs_diff_eq!(
        fiberglass_reference_enthalpy_value,
        fiberglass_enthalpy.value,
        epsilon=0.005);

}

/// returns specific enthalpy of copper
/// cited from:
/// Zou, L., Hu, R., & Charpentier, A. (2019). SAM code 
/// validation using the compact integral effects test (CIET) experimental 
/// data (No. ANL/NSE-19/11). Argonne National 
/// Lab.(ANL), Argonne, IL (United States).
#[inline]
pub fn copper_specific_enthalpy(
    temperature: ThermodynamicTemperature) -> AvailableEnergy {

    let temperature_value_kelvin: f64 = temperature.get::<kelvin>();
    // here we use a cubic spline to interpolate the values
    // it's a little calculation heavy, but don't really care now
    let specific_enthalpy_temperature_values_kelvin = c!(200.0, 
        250.0, 300.0, 350.0, 
        400.0, 500.0, 1000.0);
    let specific_heat_capacity_values_joule_per_kilogram_kelvin = c!(355.7047,
        373.6018, 384.7875, 392.6174,
        398.2103, 407.1588, 417.2260);

    let s = CubicSpline::from_nodes(&specific_enthalpy_temperature_values_kelvin, 
        &specific_heat_capacity_values_joule_per_kilogram_kelvin);

    let copper_specific_enthalpy_value = s.unwrap().
        integrate((273.15,temperature_value_kelvin));

    return AvailableEnergy::new::<joule_per_kilogram>(
        copper_specific_enthalpy_value);

}

/// returns specific enthalpy of stainless steel 304L
/// cited from:
/// Zou, L., Hu, R., & Charpentier, A. (2019). SAM code 
/// validation using the compact integral effects test (CIET) experimental 
/// data (No. ANL/NSE-19/11). Argonne National 
/// Lab.(ANL), Argonne, IL (United States).
#[inline]
pub fn steel_304_l_spline_specific_enthalpy_ciet_zweibaum(
    temperature: ThermodynamicTemperature) -> AvailableEnergy {


    let temperature_value_kelvin: f64 = temperature.get::<kelvin>();
    // here we use a cubic spline to interpolate the values
    // it's a little calculation heavy, but don't really care now
    let specific_enthalpy_temperature_values_kelvin = c!(250.0, 300.0, 350.0, 
        400.0, 450.0, 500.0, 700.0, 1000.0);
    let specific_heat_capacity_values_joule_per_kilogram_kelvin = c!(443.3375,
        457.0361, 469.4894, 480.6974, 490.66, 500.6227, 526.7746,
        551.6812);

    let s = CubicSpline::from_nodes(&specific_enthalpy_temperature_values_kelvin, 
        &specific_heat_capacity_values_joule_per_kilogram_kelvin);

    let steel_specific_enthalpy_value = s.unwrap().integrate(
        (273.15,temperature_value_kelvin));

    return AvailableEnergy::new::<joule_per_kilogram>(
        steel_specific_enthalpy_value);
}

#[inline]
fn dowtherm_a_specific_enthalpy(
    fluid_temp: ThermodynamicTemperature) -> AvailableEnergy{
    return dowtherm_a::get_dowtherm_a_enthalpy(fluid_temp).unwrap();
}

///
/// Graves, R. S., Kollie, T. G., 
/// McElroy, D. L., & Gilchrist, K. E. (1991). The 
/// specific enthalpy of AISI 304L stainless steel. 
/// International journal of thermophysics, 12, 409-415. 
///
/// data taken from ORNL
///
/// It's only good for range of 300K to 700K
///
/// However, I analytically integrated it with wolfram alpha
#[inline]
pub fn steel_ss_304_l_ornl_specific_enthalpy_graves(
    temperature: ThermodynamicTemperature) -> AvailableEnergy {

    // first I define a function for specific enthalpy between two 
    // temperatures in kelvin
    fn definite_integral_specific_enthalpy(
        temp_1: ThermodynamicTemperature,
        temp_2: ThermodynamicTemperature) -> AvailableEnergy {

        // the integration constant is assumed to be zero 

        let temp_1_value_kelvin = temp_1.get::<kelvin>();
        let temp_2_value_kelvin = temp_2.get::<kelvin>();

        let enthalpy_value_joule_per_gram_per_kelvin_temp_1 = 
        1.73333e-8 * f64::powf(temp_1_value_kelvin,3.0) 
        + 0.000085 * f64::powf(temp_1_value_kelvin, 2.0)
        + 0.4267 * temp_1_value_kelvin;

        let enthalpy_value_joule_per_gram_per_kelvin_temp_2 = 
        1.73333e-8 * f64::powf(temp_2_value_kelvin,3.0) 
        + 0.000085 * f64::powf(temp_2_value_kelvin, 2.0)
        + 0.4267 * temp_2_value_kelvin;

        let enthalpy_difference_joule_per_gram = 
        enthalpy_value_joule_per_gram_per_kelvin_temp_2 
        - enthalpy_value_joule_per_gram_per_kelvin_temp_1;

        AvailableEnergy::new::<joule_per_gram>(
            enthalpy_difference_joule_per_gram)
    }

    // reference temperature is zero degrees c, 
    // enthalpy is zero j/kg at that point
    let refernce_temperature = ThermodynamicTemperature::new::
    <kelvin>(273.15);

    let steel_enthalpy = definite_integral_specific_enthalpy(
        refernce_temperature, temperature);

    steel_enthalpy
}

#[test]
pub fn specific_enthalpy_test_steel_ornl(){
    // let's test specifc enthalpy at 350K 

    let test_temperature = ThermodynamicTemperature::new::
    <kelvin>(350.0);

    // wolfram gives an enthalpy (assuming enthalpy is zero at zero 
    // degrees C, 273.15 K)
    // this is done using the Graves et al. 1991 version for cp
    //  37.2524 j/g
    let wolfram_enthalpy_value_joule_per_kg = 37.2524*1000.0;

    let enthalpy_analytical_ornl = 
    steel_ss_304_l_ornl_specific_enthalpy_graves(test_temperature);

    approx::assert_relative_eq!(
        wolfram_enthalpy_value_joule_per_kg,
        enthalpy_analytical_ornl.value,
        max_relative=0.0001);

    
}

/// here is a test for comparing ornl and nico zweibaum's value 
/// at 375 to 425 kelvin
///
/// the cp correlation was from 300 to 700 Kelvin, so using 273.15 
/// as zero enthalpy is technically outside the range.
///
/// Despite this, it should still be able to calculate enthalpy 
/// change from 375 K to 425K
#[test]
pub fn specific_enthalpy_test_steel_ornl_and_zweibaum_spline(){
    // let's test specifc enthalpy at 350K 

    let test_temperature_1 = ThermodynamicTemperature::new::
    <kelvin>(375.0);
    let test_temperature_2 = ThermodynamicTemperature::new::
    <kelvin>(425.0);

    // wolfram gives an enthalpy (assuming enthalpy is zero at zero 
    // degrees C, 273.15 K)
    // this is done using the Graves et al. 1991 version for cp
    // 25.1515 j/g for 375 to 425 K
    let wolfram_enthalpy_value_joule_per_kg = 25.1515*1000.0;

    let enthalpy_analytical_ornl = 
    steel_ss_304_l_ornl_specific_enthalpy_graves(test_temperature_2)
    - steel_ss_304_l_ornl_specific_enthalpy_graves(test_temperature_1);

    approx::assert_relative_eq!(
        wolfram_enthalpy_value_joule_per_kg,
        enthalpy_analytical_ornl.value,
        max_relative=0.0001);

    // now let's test the spline version 
    //
    let enthalpy_spline_zweibaum = 
    steel_304_l_spline_specific_enthalpy_ciet_zweibaum(test_temperature_2)
    - steel_304_l_spline_specific_enthalpy_ciet_zweibaum(test_temperature_1);

    // there is about a 4.5% difference between the ornl value 
    // and the spline value
    // doesn't seem too bad honestly.
    //
    // Of course, one can do uncertainty propagation in order to 
    // find out the degree of change, but I won't do that for now.
    //
    // otherwise, spline should work quite okay
    approx::assert_relative_eq!(
        enthalpy_analytical_ornl.value,
        enthalpy_spline_zweibaum.value,
        max_relative=0.045);
}
