use crate::array_control_vol_and_fluid_component_collections::{fluid_component_collection::fluid_component::FluidComponent, one_d_fluid_array_with_lateral_coupling::FluidArray};

use super::ClamshellRadiativeHeater;

impl Into<FluidComponent> for ClamshellRadiativeHeater {

    fn into(self) -> FluidComponent {
        // get the fluid component 
        let fluid_array_heat_transfer_entity = self.pipe_fluid_array;
        let fluid_array: FluidArray = fluid_array_heat_transfer_entity.try_into().unwrap();

        FluidComponent::FluidArray(fluid_array)
    }
}
