use std::thread::JoinHandle;
use std::thread;

use uom::si::f64::Time;

use super::StructuralSupport;


impl StructuralSupport {
    /// advances timestep for each HeatTransferEntity within the 
    /// HeaterVersion2Bare
    pub fn _advance_timestep(&mut self, 
    timestep: Time) {

        self.support_array.advance_timestep_mut_self(timestep).unwrap();
    }

    /// advances timestep by spawning a thread 
    /// 
    pub fn advance_timestep_thread_spawn(&self,
        timestep: Time,) -> JoinHandle<Self> {

        // make clone
        let mut component_clone = self.clone();

        // move ptr into a new thread 

        let join_handle = thread::spawn(
            move || -> Self {


                // carry out the connection calculations
                component_clone.support_array.advance_timestep_mut_self(timestep).unwrap();
                
                component_clone

            }
        );

        return join_handle;

    }

}
