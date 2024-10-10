use std::ops::DerefMut;
use std::thread::JoinHandle;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

use uom::si::f64::*;

use super::HeaterVersion2Bare;


impl HeaterVersion2Bare {
    /// advances timestep for each HeatTransferEntity within the 
    /// HeaterVersion2Bare
    #[inline]
    pub fn advance_timestep(&mut self, 
    timestep: Time) {

        self.therminol_array.advance_timestep_mut_self(timestep).unwrap();
        self.steel_shell.advance_timestep_mut_self(timestep).unwrap();
        self.twisted_tape_interior.advance_timestep_mut_self(timestep).unwrap();
        
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
                heater_clone.advance_timestep(timestep);
                
                heater_clone

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
    /// Note: BUGGY
    pub fn _advance_timestep_parallel_buggy(&mut self, 
    timestep: Time) {

        // advances timestep in parallel, 
        // but must clone first
        let therminol_array_mutex = 
        Arc::new(Mutex::new(self.therminol_array.clone()));

        let steel_shell_mutex = 
        Arc::new(Mutex::new(self.steel_shell.clone()));

        let twisted_tape_mutex =
        Arc::new(Mutex::new(self.twisted_tape_interior.clone()));

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

        self.therminol_array.set(
        therminol_array_mutex.lock().as_deref_mut()
        .unwrap().clone()).unwrap();

        self.steel_shell.set(
        steel_shell_mutex.lock().as_deref_mut()
        .unwrap().clone()).unwrap();

        self.twisted_tape_interior.set(
        twisted_tape_mutex.lock().as_deref_mut()
        .unwrap().clone()).unwrap();

        // and now we are done


    }
}
