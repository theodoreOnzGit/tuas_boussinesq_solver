use uom::si::f64::*;

use super::liquid_database::dowtherm_a::min_temp_dowtherm_a;
use super::liquid_database::flibe::max_temp_flibe;
use super::liquid_database::flibe::min_temp_flibe;
use super::liquid_database::flinak::max_temp_flinak;
use super::liquid_database::flinak::min_temp_flinak;
use super::liquid_database::hitec_nitrate_salt::max_temp_hitec;
use super::liquid_database::hitec_nitrate_salt::min_temp_hitec;
use super::liquid_database::yd_325_heat_transfer_oil::max_temp_yd325_oil;
use super::liquid_database::yd_325_heat_transfer_oil::min_temp_yd325_oil;
use super::solid_database::copper::max_temp_copper_zou_zweibaum_spline;
use super::solid_database::copper::min_temp_copper_zou_zweibaum_spline;
use super::solid_database::fiberglass::max_temp_fiberglass_zou_zweibaum_spline;
use super::solid_database::fiberglass::min_temp_fiberglass_zou_zweibaum_spline;
use super::solid_database::pyrogel_hps::max_temp_pyrogel_hps;
use super::solid_database::pyrogel_hps::min_temp_pyrogel_hps;
use super::solid_database::ss_304_l::max_temp_ss_304l_zou_zweibaum_spline;
use super::solid_database::ss_304_l::min_temp_ss_304l_zou_zweibaum_spline;
use super::SolidMaterial;
use super::Material;
use super::LiquidMaterial;
use super::liquid_database::dowtherm_a::max_temp_dowtherm_a;

impl Material {

    /// gives the maximum temperature for the correlations in the
    /// given material
    pub fn max_temperature(&self) -> ThermodynamicTemperature {
        match self {
            Material::Solid(solid) => {
                solid.max_temperature()
            },
            Material::Liquid(liquid) => {
                liquid.max_temperature()
            },
        }
    }
    /// gives the maximum temperature for the correlations in the
    /// given material
    pub fn min_temperature(&self) -> ThermodynamicTemperature {

        match self {
            Material::Solid(solid) => {
                solid.min_temperature()
            },
            Material::Liquid(liquid) => {
                liquid.min_temperature()
            },
        }
    }
}

impl LiquidMaterial {

    /// gives the maximum temperature for the correlations in the
    /// given material
    pub fn max_temperature(&self) -> ThermodynamicTemperature {
        match self {
            LiquidMaterial::TherminolVP1 => {
                max_temp_dowtherm_a()
            },
            LiquidMaterial::DowthermA => {
                max_temp_dowtherm_a()
            },
            LiquidMaterial::HITEC => {
                max_temp_hitec()
            },
            LiquidMaterial::YD325 => {
                max_temp_yd325_oil()
            },
            LiquidMaterial::FLiBe => {
                max_temp_flibe()
            },
            LiquidMaterial::FLiNaK => {
                max_temp_flinak()
            },
            LiquidMaterial::CustomLiquid((_lower_bound, upper_bound)
                , _, _, _, _) => {
                *upper_bound
            },
        }
    }
    /// gives the maximum temperature for the correlations in the
    /// given material
    pub fn min_temperature(&self) -> ThermodynamicTemperature {
        match self {
            LiquidMaterial::TherminolVP1 => {
                min_temp_dowtherm_a()
            },
            LiquidMaterial::DowthermA => {
                min_temp_dowtherm_a()
            },
            LiquidMaterial::HITEC => {
                min_temp_hitec()
            },
            LiquidMaterial::YD325 => {
                min_temp_yd325_oil()
            },
            LiquidMaterial::FLiBe => {
                min_temp_flibe()
            },
            LiquidMaterial::FLiNaK => {
                min_temp_flinak()
            },
            LiquidMaterial::CustomLiquid((lower_bound, _upper_bound)
                , _, _, _, _) => {
                *lower_bound
            },
        }

    }
}
impl SolidMaterial {

    /// gives the maximum temperature for the correlations in the
    /// given material
    pub fn max_temperature(&self) -> ThermodynamicTemperature {
        match self {
            SolidMaterial::SteelSS304L => max_temp_ss_304l_zou_zweibaum_spline(),
            SolidMaterial::Copper => max_temp_copper_zou_zweibaum_spline(),
            SolidMaterial::Fiberglass => max_temp_fiberglass_zou_zweibaum_spline(),
            SolidMaterial::PyrogelHPS => max_temp_pyrogel_hps(),
            SolidMaterial::CustomSolid((_lower_bound,upper_bound), 
                _, _, _, _) => {
                *upper_bound
            },
        }
    }
    /// gives the maximum temperature for the correlations in the
    /// given material
    pub fn min_temperature(&self) -> ThermodynamicTemperature {
        match self {
            SolidMaterial::SteelSS304L => min_temp_ss_304l_zou_zweibaum_spline(),
            SolidMaterial::Copper => min_temp_copper_zou_zweibaum_spline(),
            SolidMaterial::Fiberglass => min_temp_fiberglass_zou_zweibaum_spline(),
            SolidMaterial::PyrogelHPS => min_temp_pyrogel_hps(),
            SolidMaterial::CustomSolid((lower_bound, _upper_bound), 
                _, _, _, _) => {
                *lower_bound
            },
        }

    }
}
