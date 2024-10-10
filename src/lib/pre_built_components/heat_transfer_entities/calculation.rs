use crate::thermal_hydraulics_error::ThermalHydraulicsLibError;

use super::HeatTransferEntity;
use super::cv_types::CVType;
use std::thread::JoinHandle;
use std::thread;

use uom::si::f64::*;

impl HeatTransferEntity {

    /// for control volumes, this method allows you to 
    /// calculate the enthalpy of the next timestep and 
    /// set the 
    /// current timestep enthalpy as the enthalpy calculated  
    /// for the next timestep
    ///
    /// you are required to explicitly provide a timestep for this 
    pub fn advance_timestep(entity: &mut HeatTransferEntity,
    timestep: Time) -> Result<(), ThermalHydraulicsLibError> {

        // first match CV or BC, 
        // Boundary conditions don't need to advance timestep
        // so we can leave them be (it should return an Ok(()) value 
        // rather than an Err() value)

        match entity {
            Self::ControlVolume(CVType::SingleCV(single_cv)) => {
                single_cv.advance_timestep(timestep)?
            },
            Self::ControlVolume(CVType::FluidArrayCV(fluid_array_cv)) => {
                fluid_array_cv.advance_timestep(timestep)?
            },
            Self::ControlVolume(CVType::SolidArrayCV(solid_array_cv)) => {
                solid_array_cv.advance_timestep(timestep)?
            },
            Self::BoundaryConditions(_) => return Ok(()),
        };

        return Ok(());
    }


    /// for control volumes, this method allows you to 
    /// calculate the enthalpy of the next timestep and 
    /// set the 
    /// current timestep enthalpy as the enthalpy calculated  
    /// for the next timestep
    ///
    /// you are required to explicitly provide a timestep for this 
    pub fn advance_timestep_mut_self(&mut self,
    timestep: Time) -> Result<(), ThermalHydraulicsLibError> {

        // first match CV or BC, 
        // Boundary conditions don't need to advance timestep
        // so we can leave them be (it should return an Ok(()) value 
        // rather than an Err() value)

        match self {
            Self::ControlVolume(CVType::SingleCV(single_cv)) => {
                single_cv.advance_timestep(timestep)?
            },
            Self::ControlVolume(CVType::FluidArrayCV(fluid_array_cv)) => {
                fluid_array_cv.advance_timestep(timestep)?
            },
            Self::ControlVolume(CVType::SolidArrayCV(solid_array_cv)) => {
                solid_array_cv.advance_timestep(timestep)?
            },
            Self::BoundaryConditions(_) => return Ok(()),
        };


        return Ok(());
    }

    /// spawns a handle to advance the timestep
    /// for parallel computation
    pub fn advance_timestep_mut_self_thread_spawn(&self,
        timestep: Time,) -> JoinHandle<Self> {

        // make a clone of the HeatTransferEntity (hte)
        let mut hte_clone = self.clone();

        // move ptr into a new thread 

        let join_handle = thread::spawn(
            move || -> Self {


                // carry out the connection calculations
                hte_clone.advance_timestep_mut_self(timestep).unwrap();
                
                hte_clone

            }
        );

        return join_handle;

    }

}
