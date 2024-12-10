use uom::si::f64::*;

use crate::boussinesq_thermophysical_properties::liquid_database::flibe::get_flibe_specific_enthalpy;
use crate::boussinesq_thermophysical_properties::liquid_database::flinak::get_flinak_specific_enthalpy;
use crate::boussinesq_thermophysical_properties::liquid_database::yd_325_heat_transfer_oil::get_yd325_specific_enthalpy;
use crate::boussinesq_thermophysical_properties::liquid_database::{self, dowtherm_a};
use crate::boussinesq_thermophysical_properties::liquid_database::hitec_nitrate_salt::get_hitec_specific_enthalpy;
use crate::boussinesq_thermophysical_properties::solid_database::copper::copper_specific_enthalpy;
use crate::boussinesq_thermophysical_properties::solid_database::custom_solid_material;
use crate::boussinesq_thermophysical_properties::solid_database::fiberglass::fiberglass_specific_enthalpy;
use crate::boussinesq_thermophysical_properties::solid_database::pyrogel_hps::pyrogel_hps_specific_enthalpy;
use crate::boussinesq_thermophysical_properties::solid_database::ss_304_l::steel_304_l_spline_specific_enthalpy_ciet_zweibaum;

use super::LiquidMaterial;
use super::Material;
use super::SolidMaterial;
use super::SolidMaterial::*;
use super::LiquidMaterial::*;

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
        Material::Solid(PyrogelHPS) => PyrogelHPS,
        Material::Solid(Copper) => Copper,
        Material::Solid(CustomSolid((low_bound_temp,high_bound_temp),cp,k,rho,roughness)) => {
            CustomSolid((low_bound_temp,high_bound_temp), cp, k, rho,roughness)
        },
        Material::Liquid(_) => panic!("solid_specific_enthalpy, use SolidMaterial enums only")
    };

    let specific_enthalpy: AvailableEnergy = match solid_material {
        Fiberglass => fiberglass_specific_enthalpy(solid_temp) ,
        PyrogelHPS => pyrogel_hps_specific_enthalpy(solid_temp) ,
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
        DowthermA => dowtherm_a::get_dowtherm_a_enthalpy(fluid_temp).unwrap(),
        TherminolVP1 => dowtherm_a::get_dowtherm_a_enthalpy(fluid_temp).unwrap(),
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




