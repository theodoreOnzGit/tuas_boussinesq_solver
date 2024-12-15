/// this struct holds all the data required for CIET 
/// for the ui to display it
///  
///
/// This is much easier compared to having arc mutex locks for each 
/// piece of data. 
///
/// the right way to read CIETState is to obtain a lock, clone it, and 
/// drop the lock 
///
/// the right way to write to CIETState is to have a clone of CIETState 
/// ready, obtain a lock, then overwrite it completely
#[derive(Debug,Clone,Copy)]
pub struct CIETState {
    // time diagnostics
    pub simulation_time_seconds: f64,
    pub elapsed_time_seconds: f64,
    pub calc_time_ms: f64,
    // user inputs
    pub heater_power_kilowatts: f64,
    pub ctah_pump_massrate_set_point: f64,
    pub bt_41_ctah_outlet_set_pt_deg_c: f64,
    pub bt_66_tchx_outlet_set_pt_deg_c: f64,
    // heater branch
    pub pipe_18_temp_degc: f32,
    pub pipe_1a_temp_degc: f32,
    pub bt_11_heater_inlet_deg_c: f64,
    pub bt_12_heater_outlet_deg_c: f64,
    pub pipe_1b_temp_degc: f32,
    pub pipe_2a_temp_degc: f32,
    pub pipe_2_temp_degc: f32,
    pub pipe_3_temp_degc: f32,
    pub pipe_4_temp_degc: f32,
    // dhx branch
    pub pipe_5a_temp_degc: f32,
    pub pipe_26_temp_degc: f32,
    pub pipe_25_temp_degc: f32,
    pub pipe_25a_temp_degc: f32,
    pub bt_21_dhx_shell_inlet_deg_c: f64,
    pub pipe_23_temp_degc: f32,
    pub bt_27_dhx_shell_outlet_deg_c: f64,
    pub pipe_23a_temp_degc: f32,
    pub pipe_22_temp_degc: f32,
    pub fm20_label_21a_temp_degc: f32,
    pub fm20_dhx_branch_kg_per_s: f32,
    pub pipe_21_temp_degc: f32,
    pub pipe_20_temp_degc: f32,
    pub pipe_19_temp_degc: f32,
    pub pipe_17b_temp_degc: f32,
    // dracs loop (hot branch)
    pub bt_21_dhx_tube_outlet_deg_c: f64,
    pub pipe_30b_temp_degc: f32,
    pub pipe_31a_temp_degc: f32,
    pub pipe_31_temp_degc: f32,
    pub pipe_32_temp_degc: f32,
    pub pipe_33_temp_degc: f32,
    pub pipe_34_temp_degc: f32,
    pub bt_65_tchx_inlet_deg_c: f64,
    // dracs loop (cold branch)
    pub bt_66_tchx_outlet_deg_c: f64,
    pub pipe_36a_temp_degc: f32,
    pub pipe_36_temp_degc: f32,
    pub pipe_37_temp_degc: f32,
    pub fm_60_dracs_kg_per_s: f64,
    pub fm60_label_37a_temp_degc: f32,
    pub pipe_38_temp_degc: f32,
    pub pipe_39_temp_degc: f32,
    pub pipe_30a_temp_degc: f32,
    pub bt_60_dhx_tube_inlet_deg_c: f64,
    // ctah branch
    pub pipe_5b_temp_degc: f32,
    pub pipe_6a_temp_degc: f32,
    pub pipe_6_temp_degc: f32,
    pub bt_43_ctah_inlet_deg_c: f64,
    pub bt_41_ctah_outlet_deg_c: f64,
    pub pipe_8a_temp_degc: f32,
    pub pipe_8_temp_degc: f32,
    pub pipe_9_temp_degc: f32,
    pub pipe_10_temp_degc: f32,
    pub pipe_11_temp_degc: f32,
    pub pipe_12_temp_degc: f32,
    pub pipe_13_temp_degc: f32,
    pub pipe_14_temp_degc: f32,
    pub fm40_label_14a_temp_degc: f32,
    pub fm40_ctah_branch_kg_per_s: f64,
    pub pipe_15_temp_degc: f32,
    pub pipe_16_temp_degc: f32,
    pub pipe_17a_temp_degc: f32,
    // mixing node temperatures
    pub top_mixing_node_5a_5b_4_temp_degc: f32,
    pub bottom_mixing_node_17a_17b_18_temp_degc: f32,

    // timestep settings are user settable 
    timestep_seconds: f32,
    pub fast_forward_settings_turned_on: bool,

    // pump pressure (loop pressure drop) across ctah pump 
    // is also user settable 
    // but must be less than 17000 Pa
    pub ctah_pump_pressure_pascals: f32,
    // this allows the user to block flow across the ctah branch
    pub is_ctah_branch_blocked: bool,
    pub is_dhx_branch_blocked: bool,


}

impl Default for CIETState {
    fn default() -> Self {
        CIETState {
            heater_power_kilowatts: 0.0,
            ctah_pump_massrate_set_point: 0.0,
            bt_11_heater_inlet_deg_c: 21.0,
            bt_12_heater_outlet_deg_c: 21.0,
            bt_43_ctah_inlet_deg_c: 21.0,
            bt_41_ctah_outlet_deg_c: 21.0,
            bt_41_ctah_outlet_set_pt_deg_c: 21.0,
            bt_60_dhx_tube_inlet_deg_c: 21.0,
            bt_21_dhx_tube_outlet_deg_c: 21.0,
            bt_65_tchx_inlet_deg_c: 21.0,
            bt_66_tchx_outlet_deg_c: 21.0,
            bt_66_tchx_outlet_set_pt_deg_c: 21.0,
            fm_60_dracs_kg_per_s: 0.0,
            fm20_dhx_branch_kg_per_s: 0.0,
            fm40_ctah_branch_kg_per_s: 0.0,
            simulation_time_seconds: 0.0,
            elapsed_time_seconds: 0.0,
            calc_time_ms: 0.0,
            //heater branch
            pipe_1a_temp_degc: 21.0,
            pipe_1b_temp_degc: 21.0,
            pipe_18_temp_degc: 21.0,
            pipe_2a_temp_degc: 21.0,
            pipe_2_temp_degc: 21.0,
            pipe_3_temp_degc: 21.0,
            pipe_4_temp_degc: 21.0,
            // dhx branch
            pipe_5a_temp_degc: 21.0,
            pipe_26_temp_degc: 21.0,
            pipe_25_temp_degc: 21.0,
            bt_21_dhx_shell_inlet_deg_c: 21.0,
            pipe_25a_temp_degc: 21.0,
            bt_27_dhx_shell_outlet_deg_c: 21.0,
            pipe_23_temp_degc: 21.0,
            pipe_23a_temp_degc: 21.0,
            pipe_22_temp_degc: 21.0,
            fm20_label_21a_temp_degc: 21.0,
            pipe_21_temp_degc: 21.0,
            pipe_20_temp_degc: 21.0,
            pipe_19_temp_degc: 21.0,
            pipe_17b_temp_degc: 21.0,
            // dracs loop
            pipe_30b_temp_degc: 21.0,
            pipe_31a_temp_degc: 21.0,
            pipe_31_temp_degc: 21.0,
            pipe_32_temp_degc: 21.0,
            pipe_33_temp_degc: 21.0,
            pipe_34_temp_degc: 21.0,
            pipe_36a_temp_degc: 21.0,
            pipe_36_temp_degc: 21.0,
            pipe_37_temp_degc: 21.0,
            fm60_label_37a_temp_degc: 21.0,
            pipe_38_temp_degc: 21.0,
            pipe_39_temp_degc: 21.0,
            pipe_30a_temp_degc: 21.0,
            // ctah branch
            pipe_5b_temp_degc: 21.0,
            pipe_6a_temp_degc: 21.0,
            pipe_6_temp_degc: 21.0,
            pipe_8a_temp_degc: 21.0,
            pipe_8_temp_degc: 21.0,
            pipe_9_temp_degc: 21.0,
            pipe_10_temp_degc: 21.0,
            pipe_11_temp_degc: 21.0,
            pipe_12_temp_degc: 21.0,
            pipe_13_temp_degc: 21.0,
            pipe_14_temp_degc: 21.0,
            fm40_label_14a_temp_degc: 21.0,
            pipe_15_temp_degc: 21.0,
            pipe_16_temp_degc: 21.0,
            pipe_17a_temp_degc: 21.0,
            // mixing nodes 
            top_mixing_node_5a_5b_4_temp_degc: 21.0,
            bottom_mixing_node_17a_17b_18_temp_degc: 21.0,
            
            // timestep settings are user settable as well
            timestep_seconds: 0.1,
            fast_forward_settings_turned_on: false,
            // valve and pump settings 
            //
            ctah_pump_pressure_pascals: 0.0,
            is_ctah_branch_blocked: false,
            is_dhx_branch_blocked: false,
        }
    }
}

impl CIETState {
    /// takes another ciet_state object and overwrites it
    pub fn overwrite_state(&mut self, ciet_state: Self){
        *self = ciet_state;
    }

    /// reads heater power from the state 
    pub fn get_heater_power_kilowatts(&self) -> f64 {
        return self.heater_power_kilowatts;
    }

    /// heater
    pub fn set_heater_power_kilowatts(&mut self, heater_power_kw: f64){
        self.heater_power_kilowatts = heater_power_kw;
    }

    pub fn get_heater_outlet_temp_degc(&self) -> f64 {
        return self.bt_12_heater_outlet_deg_c;
    }

    pub fn get_heater_inlet_temp_degc(&self) -> f64 {
        return self.bt_11_heater_inlet_deg_c;
    }

    /// dhx methods
    pub fn get_dhx_shell_outlet_temp_degc(&self) -> f64 {
        return self.bt_27_dhx_shell_outlet_deg_c;
    }

    pub fn get_dhx_shell_inlet_temp_degc(&self) -> f64 {
        return self.bt_21_dhx_shell_inlet_deg_c;
    }

    pub fn get_dhx_tube_outlet_temp_degc(&self) -> f64 {
        return self.bt_21_dhx_tube_outlet_deg_c;
    }

    pub fn get_dhx_tube_inlet_temp_degc(&self) -> f64 {
        return self.bt_60_dhx_tube_inlet_deg_c;
    }

    /// tchx methods
    pub fn get_tchx_outlet_temp_degc(&self) -> f64 {
        return self.bt_66_tchx_outlet_deg_c;
    }

    pub fn set_tchx_outlet_setpt_degc(&mut self, tchx_out_degc: f64){
        self.bt_66_tchx_outlet_set_pt_deg_c = tchx_out_degc;
    }

    pub fn get_tchx_inlet_temp_degc(&self) -> f64 {
        return self.bt_65_tchx_inlet_deg_c;
    }

    /// ctah methods
    pub fn get_ctah_outlet_temp_degc(&self) -> f64 {
        return self.bt_41_ctah_outlet_deg_c;
    }

    pub fn set_ctah_outlet_setpt_degc(&mut self, ctah_out_degc: f64){
        self.bt_41_ctah_outlet_set_pt_deg_c = ctah_out_degc;
    }

    pub fn get_ctah_inlet_temp_degc(&self) -> f64 {
        return self.bt_43_ctah_inlet_deg_c;
    }

    // timestep settings 
    pub fn set_timestep_seconds(&mut self, timestep_seconds: f64){
        let mut user_timestep = timestep_seconds;

        // have a minimum of 0.04s 
        let min_timestep_seconds = 0.04;

        if user_timestep < min_timestep_seconds {
            user_timestep = min_timestep_seconds;
        }


        self.timestep_seconds = user_timestep as f32;

    }

    // gets the timestep in seconds
    pub fn get_timestep_seconds(&self) -> f32 {
        return self.timestep_seconds;
    }

    // toggles the fast forward settings and returns the current state
    pub fn toggle_fast_fwd_settings_and_return_current_state(&mut self) -> bool{

        // basically, the user has a switch to turn on and off 
        // the fast forward button for simulation.
        if self.fast_forward_settings_turned_on == true {
            self.fast_forward_settings_turned_on = false;
        } else {
            self.fast_forward_settings_turned_on = true;
        }
        return self.fast_forward_settings_turned_on;
    }

    // gets the fast fwd settings 
    pub fn is_fast_fwd_on(&self) -> bool{
        return self.fast_forward_settings_turned_on;
    }

    // 

    pub fn set_ctah_pump_pressure(&mut self,
        ctah_pump_pressure_pascals: f64){

        let mut user_set_ctah_pump_pressure_pascals = 
            ctah_pump_pressure_pascals;
        // bounds ctah pump pressure from going beyond this number
        let max_pump_pressure_pascals = 17000.0;
        if ctah_pump_pressure_pascals.abs() > max_pump_pressure_pascals {

            // check sign 

            if ctah_pump_pressure_pascals.is_sign_positive() {
                user_set_ctah_pump_pressure_pascals = 
                    max_pump_pressure_pascals;
            } else {

                user_set_ctah_pump_pressure_pascals = 
                    -max_pump_pressure_pascals;
            }

        }

        self.ctah_pump_pressure_pascals = 
            user_set_ctah_pump_pressure_pascals as f32;

    }

    pub fn get_ctah_pump_pressure_f64(&self) -> f64 {
        return self.ctah_pump_pressure_pascals as f64;
    }



}

use uom::{si::f64::*, ConstZero};

/// this is the struct used to store data for graph plotting and 
/// csv extraction
/// have to lock this in an Arc Mutex pointer for parallelism
#[derive(Debug,Clone,Copy)]
pub struct PagePlotData {
    /// the heater data here is a tuple, 
    ///
    /// simulation time, heater power, inlet temp and outlet temp
    pub heater_plot_data: [(Time,Power,ThermodynamicTemperature,ThermodynamicTemperature); 500],
}


impl PagePlotData {

    /// inserts a data point, most recent being on top 
    pub fn insert_heater_data(&mut self, 
        simulation_time: Time,
        heater_power: Power,
        inlet_temp_bt11: ThermodynamicTemperature,
        outlet_temp_bt12: ThermodynamicTemperature){

        // first convert into a tuple,

        let data_tuple = 
            (simulation_time,heater_power,
             inlet_temp_bt11,outlet_temp_bt12);

        // now insert this into the heater
        // how?
        // map the vectors out first 
        let mut current_heater_data_vec: Vec< (Time,Power,
            ThermodynamicTemperature,ThermodynamicTemperature)>;

        current_heater_data_vec = self.heater_plot_data.iter().map(|&values|{
            values
        }).collect();

        // now, insert the latest data at the top
        current_heater_data_vec.insert(0, data_tuple);

        // take the first 500 pieces as a fixed size array 
        // which is basically the array size

        let mut new_array_to_be_put_back: [(Time,Power,
            ThermodynamicTemperature,ThermodynamicTemperature); 500] = 
            [ (Time::ZERO, Power::ZERO, 
             ThermodynamicTemperature::ZERO,
             ThermodynamicTemperature::ZERO); 500
            ];

        // map the first 500 values of the current heater data vec
        
        for n in 0..500 {
            new_array_to_be_put_back[n] = current_heater_data_vec[n];
        }

        self.heater_plot_data = new_array_to_be_put_back;

    }
}

impl Default for PagePlotData {
    fn default() -> Self {

        // basically a whole array of dimensioned zeroes
        let heater_data_default = 
            [ (Time::ZERO, Power::ZERO, 
             ThermodynamicTemperature::ZERO,
             ThermodynamicTemperature::ZERO); 500
            ];


        Self { 
            // first, a blank dataset
            heater_plot_data: heater_data_default,

        }
    }
}

