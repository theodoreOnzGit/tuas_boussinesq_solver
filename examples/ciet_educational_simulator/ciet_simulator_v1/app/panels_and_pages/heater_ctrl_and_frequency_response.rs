
use uom::si::power::kilowatt;
use uom::si::angular_velocity::radian_per_second;
use uom::si::f64::*;

use egui::Ui;
use crate::ciet_simulator_v1::CIETApp;

impl CIETApp {

    pub fn ciet_sim_freq_response_page(&mut self, ui: &mut Ui){



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
            ui.label(self.frequency_response_settings.get_sin_wave_label());

            ui.label("Sine Wave Amplitude (kW)");
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

            // note: frequency response updates are done in the app.rs 
        }




        //// citations please
        //self.citation_disclaimer_and_acknowledgements(ui);

        ui.separator();

        self.ciet_sim_heater_page_graph(ui);



        // update state 

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

impl FreqResponseSettings {

    pub fn get_frequency_response_signal(&self,
        current_sim_time: Time) -> Power {

            let omega = 
                AngularVelocity::new::<radian_per_second>(
                    self.angular_velocity_rad_per_s
                    );


            let angular_phase: Ratio = omega * current_sim_time;
            let angular_phase_f64: f64 = angular_phase.into();

            let sinusoid_amplitude = 
                self.total_amplitude_kw;

            let sinusoid_signal = 
                sinusoid_amplitude * (angular_phase_f64.sin());

            // next I want to have some frequency response

            let total_heater_power = 
                self.steady_state_power_kw
                + sinusoid_signal;

            return Power::new::<kilowatt>(total_heater_power);
    }

    pub fn get_steady_state_power_signal(&self) -> Power{

            let total_heater_power = self.steady_state_power_kw;

            return Power::new::<kilowatt>(total_heater_power);
    }

    pub fn get_sin_wave_label(&self) -> String {

        let mut label: String = "".to_string();

        label += "Perturbation Signal: ";
        label += &self.total_amplitude_kw.to_string();
        label += " (kW) *";
        label += " sin (";
        label += &self.angular_velocity_rad_per_s.to_string();
        label += " (rad/s) * t)";

        return label;
    }
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
