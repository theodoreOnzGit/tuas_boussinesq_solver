use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component::FluidComponent;
use crate::tuas_lib_error::TuasLibError;

use super::FluidArray;

impl Into<FluidComponent> for FluidArray {
    fn into(self) -> FluidComponent {

        FluidComponent::FluidArray(self)
    }
}

impl TryFrom<FluidComponent> for FluidArray {
    type Error = TuasLibError;

    fn try_from(value: FluidComponent) -> Result<Self, Self::Error> {
        match value {
            FluidComponent::FluidArray(fluid_array) => {
                Ok(fluid_array)
            },
            FluidComponent::ParallelUniformFluidArray(_, _) => {
                
                // probably want to change the error type to a generic 
                // type conversion error
                Err(TuasLibError::TypeConversionErrorHeatTransferEntity)
            },
        }
    }
}
