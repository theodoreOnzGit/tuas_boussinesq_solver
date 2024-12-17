
use egui_plot::{Legend, Line, Plot, PlotPoints};
use uom::si::{angular_velocity::radian_per_second, mass_rate::kilogram_per_second, pressure::pascal, thermodynamic_temperature::degree_celsius, time::second};
use uom::si::f64::*;

use egui::Ui;
use crate::ciet_simulator_v1::CIETApp;

use super::ciet_data::{CIETState, PagePlotData, NUM_DATA_PTS_IN_PLOTS};

impl CIETApp {

    pub fn ciet_sim_freq_response_page(&mut self, ui: &mut Ui){

        let mut ciet_state_local: CIETState 
            = self.ciet_state.lock().unwrap().clone();




        ui.checkbox(&mut self.frequency_response_settings.advanced_heater_control_switched_on, 
            "Turn on Advanced Heater Control");

        // 
        if self.frequency_response_settings.advanced_heater_control_switched_on {

            ui.heading("Advanced Heater Controls");
            ui.separator();

            ui.label("Steady State Average Power (kW)");
            let heater_set_pt_slider_kw = egui::Slider::new(
                &mut self.frequency_response_settings.steady_state_power_kw, 
                0.0..=15.0)
                .text("Heater Power (kW)")
                .drag_value_speed(0.001);

            ui.add(heater_set_pt_slider_kw);

            ui.checkbox(&mut self.frequency_response_settings.frequency_response_switched_on, 
                "Frequency Response Control");

            ui.label("Steady State Average Power (kW)");
            let total_amplitude_slider_kw = egui::Slider::new(
                &mut self.frequency_response_settings.total_amplitude_kw, 
                0.0..=4.0)
                .text("Total Frequency Response Amplitude(kW)")
                .drag_value_speed(0.001);

            ui.add(total_amplitude_slider_kw);

            ui.label("Angular Velocity (rad/s)");
            let total_amplitude_slider_kw = egui::Slider::new(
                &mut self.frequency_response_settings.angular_velocity_rad_per_s, 
                0.0..=10.0)
                .text("Angular Velocity Settings")
                .logarithmic(true)
                .drag_value_speed(0.001);

            ui.add(total_amplitude_slider_kw);

            // now with amplitude, I can have a frequency as well 

            let omega = 
                AngularVelocity::new::<radian_per_second>(
                    self.frequency_response_settings.angular_velocity_rad_per_s
                    );

            let time = 
                Time::new::<second>(
                    ciet_state_local.simulation_time_seconds
                );

            let angular_phase: Ratio = omega * time;
            let angular_phase_f64: f64 = angular_phase.into();

            let sinusoid_amplitude = 
                self.frequency_response_settings.total_amplitude_kw;

            let sinusoid_signal = 
                sinusoid_amplitude * (angular_phase_f64.sin());

            // next I want to have some frequency response

            let total_heater_power = 
                self.frequency_response_settings.steady_state_power_kw
                + sinusoid_signal;

            ciet_state_local.heater_power_kilowatts = 
                total_heater_power;

        }




        //// citations please
        //self.citation_disclaimer_and_acknowledgements(ui);

        ui.separator();

        self.ciet_sim_heater_page_graph(ui);



        // update state 

        self.ciet_state.lock().unwrap().overwrite_state(ciet_state_local);


        // adding the return here because there are too many closing 
        // parantheses
        // just demarcates the end

        return ();
    }

}

pub struct FreqResponseSettings{
    pub advanced_heater_control_switched_on: bool,
    pub frequency_response_switched_on: bool,
    pub steady_state_power_kw: f64,
    pub total_amplitude_kw: f64,
    pub angular_velocity_rad_per_s: f64,
    
}

impl Default for FreqResponseSettings {
    fn default() -> Self {
        return Self {
            advanced_heater_control_switched_on: false,
            frequency_response_switched_on: false,
            steady_state_power_kw: 0.0,
            total_amplitude_kw: 0.0,
            angular_velocity_rad_per_s: 0.0,

        };
    }
}
