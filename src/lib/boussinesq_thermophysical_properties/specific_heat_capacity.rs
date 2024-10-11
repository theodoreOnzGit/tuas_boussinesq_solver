use uom::si::f64::*;
use crate::thermal_hydraulics_error::TuasLibError;

use super::liquid_database;
use super::liquid_database::flibe::get_flibe_constant_pressure_specific_heat_capacity;
use super::liquid_database::flinak::get_flinak_constant_pressure_specific_heat_capacity;
use super::liquid_database::hitec_nitrate_salt::get_hitec_constant_pressure_specific_heat_capacity;
use super::liquid_database::yd_325_heat_transfer_oil::get_yd325_constant_pressure_specific_heat_capacity;
use super::solid_database::copper::copper_specific_heat_capacity_zou_zweibaum_spline;
use super::solid_database::custom_solid_material;
use super::solid_database::fiberglass::fiberglass_specific_heat_capacity;
use super::solid_database::ss_304_l::steel_304_l_libreoffice_spline_specific_heat_capacity_ciet_zweibaum;
use super::LiquidMaterial;
use super::Material;
use super::SolidMaterial;
use super::SolidMaterial::*;
use super::LiquidMaterial::*;
use super::liquid_database::dowtherm_a::get_dowtherm_a_constant_pressure_specific_heat_capacity;


/// returns cp for a given material 
///
/// ```rust 
/// use uom::si::f64::*;
/// use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
/// use uom::si::thermodynamic_temperature::kelvin;
/// use tuas_boussinesq_solver::boussinesq_thermophysical_properties::
/// SolidMaterial::SteelSS304L;
/// use tuas_boussinesq_solver::boussinesq_thermophysical_properties::
/// Material;
/// use tuas_boussinesq_solver::boussinesq_thermophysical_properties::
/// specific_heat_capacity::try_get_cp;
///
/// use uom::si::pressure::atmosphere;
///
/// let steel = Material::Solid(SteelSS304L);
/// let steel_temp = ThermodynamicTemperature::new::<kelvin>(350.0);
/// let pressure = Pressure::new::<atmosphere>(1.0);
///
/// // at 350K, we should expect thermal conductivity, 
/// // 470 W/(m K)
///
/// let steel_thermal_cond: SpecificHeatCapacity = 
/// try_get_cp(steel, steel_temp, pressure).unwrap();
///
///
/// approx::assert_relative_eq!(
///     470.0,
///     steel_thermal_cond.value,
///     max_relative=0.035);
///
/// ``` 
#[inline]
pub fn try_get_cp(material: Material, 
    temperature: ThermodynamicTemperature,
    _pressure: Pressure) -> Result<SpecificHeatCapacity, TuasLibError> {

    let specific_heat_capacity: SpecificHeatCapacity = match material {
        Material::Solid(_) => solid_specific_heat_capacity(material, temperature)?,
        Material::Liquid(_) => liquid_specific_heat_capacity(material, temperature)?
    };

    return Ok(specific_heat_capacity);
}

// should the material happen to be a solid, use this function
fn solid_specific_heat_capacity(material: Material,
    solid_temp: ThermodynamicTemperature) -> Result<SpecificHeatCapacity, TuasLibError>{
    
    // first match the enum

    let solid_material: SolidMaterial = match material {
        Material::Solid(SteelSS304L) => SteelSS304L,
        Material::Solid(Fiberglass) => Fiberglass,
        Material::Solid(Copper) => Copper,
        Material::Solid( CustomSolid((low_bound_temp,high_bound_temp),cp,k,rho_fn,roughness))=> {
            CustomSolid((low_bound_temp,high_bound_temp), cp, k, rho_fn,roughness)
        },
        Material::Liquid(_) => panic!("solid_specific_heat_capacity, use SolidMaterial enums only")
    };

    let specific_heat_capacity: SpecificHeatCapacity = match solid_material {
        Fiberglass => fiberglass_specific_heat_capacity(solid_temp) ,
        SteelSS304L => steel_304_l_libreoffice_spline_specific_heat_capacity_ciet_zweibaum(solid_temp)?,
        Copper => copper_specific_heat_capacity_zou_zweibaum_spline(solid_temp)?,
        CustomSolid((low_bound_temp,high_bound_temp),cp_fn,_k,_rho_fn,_roughness) => {
            custom_solid_material::get_custom_solid_constant_pressure_specific_heat_capacity(
                solid_temp, 
                cp_fn, 
                high_bound_temp, 
                low_bound_temp)?
        },
    };

    return Ok(specific_heat_capacity);


}


impl LiquidMaterial {
    /// wrapper that 
    /// returns the liquid cp in a result enum 
    #[inline]
    pub fn try_get_cp(&self,
        fluid_temp: ThermodynamicTemperature,) 
        -> Result<SpecificHeatCapacity, TuasLibError>{

            liquid_specific_heat_capacity(
                self.clone().into(),
                fluid_temp)
        }


}
impl SolidMaterial {
    /// wrapper that 
    /// returns the solid cp in a result enum 
    #[inline]
    pub fn try_get_cp(&self,
        solid_temp: ThermodynamicTemperature,) 
        -> Result<SpecificHeatCapacity, TuasLibError>{

            solid_specific_heat_capacity(
                self.clone().into(),
                solid_temp)
        }


}

// should the material happen to be a liquid, use this function
fn liquid_specific_heat_capacity(material: Material, 
    fluid_temp: ThermodynamicTemperature) -> Result<SpecificHeatCapacity,
TuasLibError>{

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
        "liquid_specific_heat_capacity, use LiquidMaterial enums only")
    };

    let specific_heat_capacity: SpecificHeatCapacity = match liquid_material {
        DowthermA => get_dowtherm_a_constant_pressure_specific_heat_capacity(fluid_temp)?,
        TherminolVP1 => get_dowtherm_a_constant_pressure_specific_heat_capacity(fluid_temp)?,
        HITEC => get_hitec_constant_pressure_specific_heat_capacity(fluid_temp)?,
        YD325 => get_yd325_constant_pressure_specific_heat_capacity(fluid_temp)?,
        FLiBe => get_flibe_constant_pressure_specific_heat_capacity(fluid_temp)?,
        FLiNaK => get_flinak_constant_pressure_specific_heat_capacity(fluid_temp)?,
        CustomLiquid((low_bound_temp,high_bound_temp), cp_fn, _k, _mu_fn, _rho_fn) => {
            liquid_database::custom_liquid_material
                ::get_custom_fluid_constant_pressure_specific_heat_capacity(fluid_temp, 
                    cp_fn, 
                    high_bound_temp, 
                    low_bound_temp)?
        },
    };

    return Ok(specific_heat_capacity);
}






