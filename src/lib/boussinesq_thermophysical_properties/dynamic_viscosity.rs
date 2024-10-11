use uom::si::f64::*;
use crate::thermal_hydraulics_error::TuasLibError;

use super::liquid_database::flibe::get_flibe_dynamic_viscosity;
use super::liquid_database::flinak::get_flinak_dynamic_viscosity;
use super::liquid_database::hitec_nitrate_salt::get_hitec_dynamic_viscosity;
use super::liquid_database::yd_325_heat_transfer_oil::get_yd325_dynamic_viscosity;
use super::LiquidMaterial;
use super::Material;
use super::LiquidMaterial::*;
use super::liquid_database::dowtherm_a::get_dowtherm_a_viscosity;
use super::liquid_database;

/// returns a dynamic_viscosity given a material, temperature and pressure
///
/// example:
///
/// ```rust
/// use uom::si::f64::*;
/// use uom::si::pressure::atmosphere;
/// use uom::si::thermodynamic_temperature::kelvin;
/// use tuas_boussinesq_solver::
/// boussinesq_thermophysical_properties::dynamic_viscosity::try_get_mu_viscosity;
///
/// use tuas_boussinesq_solver::boussinesq_thermophysical_properties
/// ::LiquidMaterial::DowthermA;
///
/// use tuas_boussinesq_solver::
/// boussinesq_thermophysical_properties::Material;
///
/// let dowtherm_a = Material::Liquid(DowthermA);
/// let temperature = ThermodynamicTemperature::new::<kelvin>(350.0);
/// let pressure = Pressure::new::<atmosphere>(1.0);
///
/// let dynamic_viscosity_result = 
/// try_get_mu_viscosity(dowtherm_a, temperature, pressure);
///
/// approx::assert_relative_eq!(
///     0.001237,
///     dynamic_viscosity_result.unwrap().value,
///     max_relative=0.01);
/// 
/// ```
#[inline]
pub fn try_get_mu_viscosity(material: Material, 
    temperature: ThermodynamicTemperature,
    _pressure: Pressure) -> Result<DynamicViscosity, TuasLibError> {

    match material {
        Material::Solid(_) => {
            println!("Error: Solids do not have dynamic viscosity");
            Err(TuasLibError::ThermophysicalPropertyError)
        },
        Material::Liquid(_) => liquid_dynamic_viscosity(material, temperature)
    }
}




// should the material happen to be a liquid, use this function
#[inline]
fn liquid_dynamic_viscosity(material: Material, 
    fluid_temp: ThermodynamicTemperature) -> Result<DynamicViscosity,TuasLibError> {

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
        Material::Solid(_) => panic!("liquid_dynamic_viscosity, use LiquidMaterial enums only")
    };

    let dynamic_viscosity: DynamicViscosity = match liquid_material {
        DowthermA => get_dowtherm_a_viscosity(fluid_temp)?,
        TherminolVP1 => get_dowtherm_a_viscosity(fluid_temp)?,
        HITEC => get_hitec_dynamic_viscosity(fluid_temp)?,
        YD325 => get_yd325_dynamic_viscosity(fluid_temp)?,
        FLiBe => get_flibe_dynamic_viscosity(fluid_temp)?,
        FLiNaK => get_flinak_dynamic_viscosity(fluid_temp)?,
        CustomLiquid((low_bound_temp,high_bound_temp), _cp, _k, mu_fn, _rho_fn) => {
            liquid_database::custom_liquid_material
                ::get_custom_fluid_viscosity(fluid_temp, 
                    mu_fn, 
                    high_bound_temp, 
                    low_bound_temp)?
        },
    };

    return Ok(dynamic_viscosity);
}

impl LiquidMaterial {
    /// obtains a result based on the dynamic viscosity of the material
    #[inline]
    pub fn try_get_dynamic_viscosity(&self,
        fluid_temp: ThermodynamicTemperature,) -> 
    Result<DynamicViscosity, TuasLibError>{

        let dynamic_viscosity: DynamicViscosity = match self {
            DowthermA => get_dowtherm_a_viscosity(fluid_temp)?,
            TherminolVP1 => get_dowtherm_a_viscosity(fluid_temp)?,
            HITEC => get_hitec_dynamic_viscosity(fluid_temp)?,
            YD325 => get_yd325_dynamic_viscosity(fluid_temp)?,
            FLiBe => get_flibe_dynamic_viscosity(fluid_temp)?,
            FLiNaK => get_flinak_dynamic_viscosity(fluid_temp)?,
            CustomLiquid((low_bound_temp,high_bound_temp), _cp, _k, mu_fn, _rho_fn) => {
                
                liquid_database::custom_liquid_material
                    ::get_custom_fluid_viscosity(fluid_temp, 
                        *mu_fn, 
                        *high_bound_temp, 
                        *low_bound_temp)?
            },
        };

        Ok(dynamic_viscosity)

    }
}





