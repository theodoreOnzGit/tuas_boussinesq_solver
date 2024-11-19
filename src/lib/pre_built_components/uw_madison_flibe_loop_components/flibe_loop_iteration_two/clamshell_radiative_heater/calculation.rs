use super::ClamshellRadiativeHeater;
use crate::tuas_lib_error::TuasLibError;
use uom::si::f64::*;
use std::thread::JoinHandle;
use std::thread;


impl ClamshellRadiativeHeater {

    /// advances timestep for each HeatTransferEntity within the 
    /// SimpleShellAndTubeHeatExchanger
    ///
    /// gives each pipe the parallel tube treatment
    ///
    #[inline]
    pub fn advance_timestep(&mut self, 
    timestep: Time) -> Result<(),TuasLibError> {
        
        self.pipe_fluid_array.advance_timestep_mut_self(timestep)?;
        self.pipe_shell_array.advance_timestep_mut_self(timestep)?;
        self.annular_air_array.advance_timestep_mut_self(timestep)?;
        self.heating_element_shell.advance_timestep_mut_self(timestep)?;
        self.insulation_array.advance_timestep_mut_self(timestep)?;
        // done, pending test

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
