use crate::boundary_conditions::BCType;
use uom::num_traits::Zero;
use uom::si::f64::*;

use super::HeatTransferEntity;

impl HeatTransferEntity {

    /// constructors for BCs (convenience)
    /// creates a new constant temperature BC
    pub fn new_const_temperature_bc(temperature:ThermodynamicTemperature)
        -> Self {
        BCType::UserSpecifiedTemperature(temperature).into()
    }

    /// creates a new constant heat flux bc
    pub fn new_const_heat_flux_bc(heat_flux: HeatFluxDensity)
        -> Self {
        BCType::UserSpecifiedHeatFlux(heat_flux).into()
    }

    /// creates a new constant heat addition bc
    pub fn new_const_heat_addition(heat_addition: Power)
        -> Self {
        BCType::UserSpecifiedHeatAddition(heat_addition).into()
    }

    /// creates a new constant heat addition bc
    pub fn new_adiabatic_bc() -> Self {
        BCType::UserSpecifiedHeatAddition(Power::zero()).into()
    }
}
