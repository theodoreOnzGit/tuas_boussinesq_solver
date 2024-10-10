use crate::array_control_vol_and_fluid_component_collections::{fluid_component_collection::fluid_component::FluidComponent, one_d_fluid_array_with_lateral_coupling::FluidArray};

use super::NonInsulatedParallelFluidComponent;

impl Into<FluidComponent> for NonInsulatedParallelFluidComponent {
    fn into(self) -> FluidComponent {
        // get the fluid component 
        let fluid_array_heat_transfer_entity = self.pipe_fluid_array;
        let fluid_array: FluidArray = fluid_array_heat_transfer_entity.try_into().unwrap();

        let number_of_parallel_tubes: u32 = self.number_of_tubes;

        FluidComponent::ParallelUniformFluidArray(
            fluid_array,number_of_parallel_tubes)
    }
}


