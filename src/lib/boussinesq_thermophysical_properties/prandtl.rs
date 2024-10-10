use uom::si::f64::*;
use crate::thermal_hydraulics_error::ThermalHydraulicsLibError;
use super::{Material, LiquidMaterial};
use super::dynamic_viscosity::try_get_mu_viscosity;
use super::specific_heat_capacity::try_get_cp;
use super::thermal_conductivity::try_get_kappa_thermal_conductivity;


/// provides the prandtl number
#[inline]
pub fn try_get_prandtl(material: Material,
    temperature: ThermodynamicTemperature,
    pressure: Pressure) -> Result<Ratio, ThermalHydraulicsLibError> {

    // get mu 
    let mu: DynamicViscosity = try_get_mu_viscosity(
        material,
        temperature,
        pressure)?;

    // get cp 
    let cp: SpecificHeatCapacity = try_get_cp(
        material,
        temperature,
        pressure)?;

    // get k 
    let k: ThermalConductivity = try_get_kappa_thermal_conductivity(
        material,
        temperature,
        pressure)?;

    Ok(mu*cp/k)

}

impl LiquidMaterial {

    /// provides the prandtl number for a liquid material
    /// probably can use some rewriting to increase efficiency
    pub fn try_get_prandtl_liquid(&self,
    temperature: ThermodynamicTemperature,
    pressure: Pressure) -> Result<Ratio, ThermalHydraulicsLibError>{

        let material : Material = self.clone().into();

        try_get_prandtl(material, temperature, pressure)


    }
}
