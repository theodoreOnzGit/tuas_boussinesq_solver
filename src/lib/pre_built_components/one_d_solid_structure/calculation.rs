use super::SolidStructure;
use uom::si::f64::*;
use crate::thermal_hydraulics_error::ThermalHydraulicsLibError;
use std::thread::JoinHandle;
use std::thread;

impl SolidStructure {

    /// advances timestep for each HeatTransferEntity within the 
    #[inline]
    pub fn advance_timestep(&mut self, 
    timestep: Time) -> Result<(),ThermalHydraulicsLibError> {

        self.solid_array.advance_timestep_mut_self(timestep)?;
        Ok(())
        
    }


    /// advances timestep by spawning a thread 
    /// 
    pub fn advance_timestep_thread_spawn(&self,
        timestep: Time,) -> JoinHandle<Self> {

        // make a clone
        let mut heater_clone = self.clone();

        // move ptr into a new thread 

        let join_handle = thread::spawn(
            move || -> Self {


                // carry out the connection calculations
                heater_clone.advance_timestep(timestep).unwrap();
                
                heater_clone

            }
        );

        return join_handle;

    }
}
