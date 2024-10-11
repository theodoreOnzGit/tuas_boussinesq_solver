use uom::si::f64::*;

use super::{density, dynamic_viscosity, LiquidMaterial, Material};
use crate::thermal_hydraulics_error::TuasLibError;

/// gets kinematic viscosity
#[inline]
pub fn try_get_nu_momentum_diffusivity(material: Material, 
    temperature: ThermodynamicTemperature,
    pressure: Pressure) -> Result<DiffusionCoefficient,TuasLibError> {

    let material_density: MassDensity = 
    density::try_get_rho(
        material, 
        temperature, 
        pressure)?;
    
    let material_dynamic_viscosity: DynamicViscosity = 
    dynamic_viscosity::try_get_mu_viscosity(
        material, 
        temperature, 
        pressure)?;

    let nu: DiffusionCoefficient = 
        material_dynamic_viscosity/
        material_density;

    return Ok(nu);
}


impl LiquidMaterial {
    /// wrapper that 
    /// returns the liquid cp in a result enum 
    #[inline]
    pub fn try_get_nu_momentum_diffusivity(&self,
        fluid_temp: ThermodynamicTemperature,
        pressure: Pressure) 
        -> Result<DiffusionCoefficient, TuasLibError>{

            try_get_nu_momentum_diffusivity(
                self.clone().into(),
                fluid_temp,
                pressure)
        }


}

/// for a given material, 
/// Prandtl number is nu/alpha
#[test]
pub fn check_that_prandtl_is_nu_divide_by_alpha(){
    use uom::si::pressure::bar;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::si::ratio::ratio;
    let hitec = LiquidMaterial::HITEC;

    let pressure = Pressure::new::<bar>(1.0);
    let temperature = ThermodynamicTemperature::new::<degree_celsius>(220.0);

    let prandtl = hitec
        .try_get_prandtl_liquid(temperature, pressure)
        .unwrap();

    let nu = hitec
        .try_get_nu_momentum_diffusivity(temperature, pressure)
        .unwrap();

    let alpha = hitec 
        .try_get_alpha_thermal_diffusivity(temperature, pressure)
        .unwrap();

    let test_prandtl = nu/alpha;

    approx::assert_relative_eq!(
        prandtl.get::<ratio>(),
        test_prandtl.get::<ratio>(),
        max_relative=1e-5);
}

