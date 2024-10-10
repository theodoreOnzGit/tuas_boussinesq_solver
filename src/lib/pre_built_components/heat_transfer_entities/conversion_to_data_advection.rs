use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::DataAdvection;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use uom::si::f64::*;

use super::HeatTransferEntity;


impl DataAdvection {


    /// constructs an advection interaction by specifying 
    /// a fluid material 
    /// mutable reference of the heat transfer entity 1
    /// and mutable reference of heat transfer entity 2
    ///
    ///
    /// (heat tranfer entity 1) ----mass flowrate --> (heat transfer entity 2)
    ///
    #[inline] 
    pub fn new_from_heat_transfer_entity(
        user_input_mass_flowrate: MassRate,
        fluid_material: LiquidMaterial,
        hte_1: &mut HeatTransferEntity,
        hte_2: &mut HeatTransferEntity) -> Self {

        // if one of the heat transfer entities is a adiabatic 
        // bc, it will have no temperature, but 
        // i can use temperature of the other heat transfer 
        // entity
        //
        // perhaps in future I can handle the errors better 
        //
        // for now, no choice but to unwrap due to time 
        // constraints
        let temperature_1: ThermodynamicTemperature = 
        HeatTransferEntity::temperature(hte_1).unwrap();

        let temperature_2: ThermodynamicTemperature = 
        HeatTransferEntity::temperature(hte_2).unwrap();

        let density_1 = fluid_material.try_get_density(temperature_1).unwrap();
        let density_2 = fluid_material.try_get_density(temperature_2).unwrap();

        return Self {
            mass_flowrate: user_input_mass_flowrate,
            fluid_density_heat_transfer_entity_1: density_1,
            fluid_density_heat_transfer_entity_2: density_2,
        };

    }
}
