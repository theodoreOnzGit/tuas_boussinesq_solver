use core::fmt;
use std::ops::Deref;

use egui::Ui;

use crate::ciet_simulator_v1::CIETApp;

use uom::si::{f64::*, heat_transfer::watt_per_square_meter_kelvin, thermodynamic_temperature::degree_celsius, time::second};
use egui_plot::{Legend, Line, Plot, PlotPoints};
use super::ciet_data::{CIETState, PagePlotData};

impl CIETApp {

    pub fn ciet_sim_online_calibration_page(&mut self, ui: &mut Ui){

        ui.heading("Heater Calibration");

        // allows user to select heater type
        let heater_type_display: &String = 
            &self.user_desired_heater_type.clone().to_string();
        egui::ComboBox::from_label("Choose Heater Type")
            .selected_text(format!("{heater_type_display}"))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.user_desired_heater_type, HeaterType::InsulatedHeaterV1Fine15Mesh, "Heater V1 Fine (15 Nodes)");
                ui.selectable_value(&mut self.user_desired_heater_type, HeaterType::InsulatedHeaterV1Coarse8Mesh, "Heater V1 Corase (8 Nodes)");
            });
        // displays current heater type 
        ui.label("current_heater_type:");

        let local_ciet_state: CIETState = 
            self.ciet_state.lock().unwrap().deref().clone();

        let current_heater_type = local_ciet_state.current_heater_type;

        ui.label(format!("{current_heater_type}"));


    }
}

#[derive(Debug,PartialEq,Clone,Copy)]
pub enum HeaterType {
    InsulatedHeaterV1Fine15Mesh,
    InsulatedHeaterV1Coarse8Mesh,
}

impl fmt::Display for HeaterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        fmt::Debug::fmt(self, f)

    }
}
