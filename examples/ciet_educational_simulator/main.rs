#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
pub fn main(){
    println!("Starting CIET Educational Simulator...");
        ciet_simulator_v1::ciet_simulator_v1().unwrap();
}


pub mod ciet_simulator_v1;
