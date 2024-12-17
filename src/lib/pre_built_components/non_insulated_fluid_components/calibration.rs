use crate::{heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation, prelude::beta_testing::FluidArray};

use super::NonInsulatedFluidComponent;

impl NonInsulatedFluidComponent {

    /// allows user to set nusselt correlation for the 
    /// fluid flowing within the pipe
    /// done using cloning, so abit slow
    pub fn calibrate_nusselt_correlation_for_fluid_within_pipe(
        &mut self,
        nusselt_correlation_user_set: NusseltCorrelation){

        let mut fluid_array_local: FluidArray = 
            self.pipe_fluid_array.clone().try_into().unwrap();

        fluid_array_local.nusselt_correlation = nusselt_correlation_user_set;

        self.pipe_fluid_array = fluid_array_local.into();

    }

}
