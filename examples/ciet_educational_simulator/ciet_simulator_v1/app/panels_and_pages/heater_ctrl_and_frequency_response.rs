
use egui_plot::{Legend, Line, Plot, PlotPoints};
use uom::si::{f64::*, mass_rate::kilogram_per_second, pressure::pascal, thermodynamic_temperature::degree_celsius, time::second};

use egui::Ui;
use crate::ciet_simulator_v1::CIETApp;

use super::ciet_data::{CIETState, PagePlotData, NUM_DATA_PTS_IN_PLOTS};

impl CIETApp {

    pub fn ciet_sim_freq_response_page(&mut self, ui: &mut Ui){

        let mut ciet_state_local: CIETState 
            = self.ciet_state.lock().unwrap().clone();



        let heater_set_pt_slider_kw = egui::Slider::new(
            &mut ciet_state_local.heater_power_kilowatts, 
            0.0..=10.0)
            .vertical()
            .text("Heater Power (kW)");

        ui.add(heater_set_pt_slider_kw);

        ui.checkbox(&mut self.frequency_response_settings.frequency_response_switched_on, 
            "Turn on Frequency Response Control");

        // 
        if self.frequency_response_settings.frequency_response_switched_on {

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
    pub frequency_response_switched_on: bool,
}

impl Default for FreqResponseSettings {
    fn default() -> Self {
        return Self {
            frequency_response_switched_on: false,
        };
    }
}
