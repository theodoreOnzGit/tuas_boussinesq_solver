use uom::si::f64::*;

use super::LiquidMaterial;
use super::Material;
use super::SolidMaterial;
use super::SolidMaterial::*;
use super::LiquidMaterial::*;
use crate::boussinesq_thermophysical_properties::liquid_database;
use crate::boussinesq_thermophysical_properties::liquid_database::dowtherm_a;
use crate::boussinesq_thermophysical_properties::liquid_database::flibe;
use crate::boussinesq_thermophysical_properties::liquid_database::flinak;
use crate::boussinesq_thermophysical_properties::liquid_database::hitec_nitrate_salt;
use crate::boussinesq_thermophysical_properties::liquid_database::yd_325_heat_transfer_oil;
use crate::boussinesq_thermophysical_properties::solid_database::copper::copper_spline_temp_attempt_2_from_specific_enthalpy;
use crate::boussinesq_thermophysical_properties::solid_database::custom_solid_material;
use crate::boussinesq_thermophysical_properties::solid_database::fiberglass::fiberglass_spline_temp_attempt_1_from_specific_enthalpy;
use crate::boussinesq_thermophysical_properties::solid_database::pyrogel_hps::pyrogel_hps_spline_temp_attempt_1_from_specific_enthalpy;
use crate::boussinesq_thermophysical_properties::solid_database::ss_304_l::steel_304_l_spline_temp_attempt_3_from_specific_enthalpy_ciet_zweibaum;

// for spline method

// for root finding with brent
// should the material happen to be a solid, use this function
// to extract temperature from enthalpy
//
// probably should have a temperature range checker in 
// future
//
// pub(in crate::boussinesq_thermophysical_properties)
// here only makes it accessible to the 
// specific_enthalpy/mod.rs 
// nothing else
pub(in crate::boussinesq_thermophysical_properties) 
fn get_solid_temperature_from_specific_enthalpy(material: Material,
    h_material: AvailableEnergy) -> ThermodynamicTemperature {
    
    // first match the enum

    let solid_material: SolidMaterial = match material {
        Material::Solid(SteelSS304L) => SteelSS304L,
        Material::Solid(Fiberglass) => Fiberglass,
        Material::Solid(PyrogelHPS) => PyrogelHPS,
        Material::Solid(Copper) => Copper,
        Material::Solid(CustomSolid((low_bound_temp,high_bound_temp),cp,k,rho,roughness)) => {
            CustomSolid((low_bound_temp,high_bound_temp), cp, k, rho,roughness)
        },
        Material::Liquid(_) => panic!("solid_specific_enthalpy, use SolidMaterial enums only")
    };

    let material_temperature: ThermodynamicTemperature = 
        match solid_material {
            Fiberglass => 
            {
                fiberglass_spline_temp_attempt_1_from_specific_enthalpy(
                    h_material)
            },
            PyrogelHPS => 
            {
                pyrogel_hps_spline_temp_attempt_1_from_specific_enthalpy(
                    h_material)
            },
            SteelSS304L => 
            {
                steel_304_l_spline_temp_attempt_3_from_specific_enthalpy_ciet_zweibaum(
                    h_material)
            },
            Copper => 
            {
                copper_spline_temp_attempt_2_from_specific_enthalpy(
                    h_material)
            },
            CustomSolid((low_bound_temp,high_bound_temp),cp_fn,_k,_rho_fn,_roughness) => {
                custom_solid_material::get_custom_solid_temperature_from_enthalpy(
                    h_material, 
                    cp_fn, 
                    high_bound_temp, 
                    low_bound_temp).unwrap()
            },

        };

    return material_temperature;


}

// should the material happen to be a liquid, use this function
// pub(in crate::boussinesq_thermophysical_properties) 
// here only makes it accessible to the 
// specific_enthalpy/mod.rs 
// nothing else
pub(in crate::boussinesq_thermophysical_properties) 
fn get_liquid_temperature_from_specific_enthalpy(material: Material, 
    fluid_enthalpy: AvailableEnergy) -> ThermodynamicTemperature {

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

    let specific_enthalpy: ThermodynamicTemperature = match liquid_material {
        DowthermA => dowtherm_a::get_temperature_from_enthalpy(fluid_enthalpy).unwrap(),
        TherminolVP1 => dowtherm_a::get_temperature_from_enthalpy(fluid_enthalpy).unwrap(),
        HITEC => hitec_nitrate_salt::get_temperature_from_enthalpy(fluid_enthalpy).unwrap(),
        YD325 => yd_325_heat_transfer_oil::get_temperature_from_enthalpy(fluid_enthalpy).unwrap(),
        FLiBe => flibe::get_temperature_from_enthalpy(fluid_enthalpy).unwrap(),
        FLiNaK => flinak::get_temperature_from_enthalpy(fluid_enthalpy).unwrap(),
        CustomLiquid((low_bound_temp,high_bound_temp), cp_fn, _k, _mu_fn, _rho_fn) => {
            liquid_database::custom_liquid_material
                ::get_custom_fluid_temperature_from_enthalpy(fluid_enthalpy, 
                    cp_fn, 
                    high_bound_temp, 
                    low_bound_temp).unwrap()
        },
    };

    return specific_enthalpy;
}







