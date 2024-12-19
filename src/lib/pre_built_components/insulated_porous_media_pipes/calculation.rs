use std::sync::Arc;
use std::sync::Mutex;
use std::thread::{self};
use std::thread::JoinHandle;
use std::ops::DerefMut;

use uom::si::f64::*;
use super::InsulatedPorousMediaFluidComponent;


impl InsulatedPorousMediaFluidComponent {
    /// advances timestep for each HeatTransferEntity within the 
    /// HeaterVersion2Bare
    pub fn _advance_timestepp(&mut self, 
    timestep: Time) {

        self.pipe_fluid_array.advance_timestep_mut_self(timestep).unwrap();
        self.pipe_shell.advance_timestep_mut_self(timestep).unwrap();
        self.insulation_array.advance_timestep_mut_self(timestep).unwrap();
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
                component_clone.pipe_fluid_array.advance_timestep_mut_self(timestep).unwrap();
                component_clone.pipe_shell.advance_timestep_mut_self(timestep).unwrap();
                component_clone.insulation_array.advance_timestep_mut_self(timestep).unwrap();
                
                component_clone

            }
        );

        return join_handle;

    }

    /// advances timestep for each HeatTransferEntity within the 
    /// HeaterVersion2Bare
    ///
    /// parallel implementation, spawns three threads to do it,
    /// however, it relies heavily upon cloning, so it may or may not 
    /// be faster
    ///
    /// note: buggy, not debugged yet
    pub fn _advance_timestep_parallel_buggy(&mut self, 
    timestep: Time) {
        // advances timestep in parallel, 
        // but must clone first
        let therminol_array_mutex = 
        Arc::new(Mutex::new(self.pipe_fluid_array.clone()));

        let steel_shell_mutex = 
        Arc::new(Mutex::new(self.pipe_shell.clone()));

        let twisted_tape_mutex =
        Arc::new(Mutex::new(self.insulation_array.clone()));

        // now create clones of the pointers so that we can access them 
        // later 
        
        let therminol_array_mutex_clone = therminol_array_mutex.clone();
        let steel_shell_mutex_clone = steel_shell_mutex.clone();
        let twisted_tape_mutex_clone = twisted_tape_mutex.clone();

        let handle_1: JoinHandle<()> = thread::spawn( move || {
            let mut therminol_arr_mutex_guard = 
            therminol_array_mutex_clone.lock().unwrap();

            // now advance timestep 

            therminol_arr_mutex_guard.deref_mut().advance_timestep_mut_self(
            timestep).unwrap();
        });
        let handle_2: JoinHandle<()> = thread::spawn( move || {
            let mut steel_shell_arr_mutex_guard = 
            steel_shell_mutex_clone.lock().unwrap();

            // now advance timestep 

            steel_shell_arr_mutex_guard.deref_mut().advance_timestep_mut_self(
            timestep).unwrap();
        });
        let handle_3: JoinHandle<()> = thread::spawn( move || {
            let mut twisted_tape_arr_mutex_guard = 
            twisted_tape_mutex_clone.lock().unwrap();

            // now advance timestep 

            twisted_tape_arr_mutex_guard.deref_mut().advance_timestep_mut_self(
            timestep).unwrap();
        });

        let mut handle_vec: Vec<JoinHandle<()>> = vec![];
        handle_vec.push(handle_1);
        handle_vec.push(handle_2);
        handle_vec.push(handle_3);


        // join all handles, thus synchronising data
        let _ = handle_vec.into_iter().map(
            |handle|{
                handle.join().unwrap()
            }
        );


        // now the items in the mutex locks are the timestap advanced 
        // versions, 
        //
        // To advance timestep, one must then clone what is inside 
        // the mutex locks and override the data within the object

        self.pipe_fluid_array.set(
        therminol_array_mutex.lock().as_deref_mut()
        .unwrap().clone()).unwrap();

        self.pipe_shell.set(
        steel_shell_mutex.lock().as_deref_mut()
        .unwrap().clone()).unwrap();

        self.insulation_array.set(
        twisted_tape_mutex.lock().as_deref_mut()
        .unwrap().clone()).unwrap();

        // and now we are done


    }
}
