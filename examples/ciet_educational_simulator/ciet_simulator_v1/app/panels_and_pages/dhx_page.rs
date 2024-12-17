
use egui_plot::{Legend, Line, Plot, PlotPoints};
use uom::si::{f64::*, mass_rate::kilogram_per_second, pressure::pascal, thermodynamic_temperature::degree_celsius, time::second};

use egui::Ui;


use crate::ciet_simulator_v1::CIETApp;

use super::ciet_data::{PagePlotData, NUM_DATA_PTS_IN_PLOTS};
impl CIETApp {

    pub fn ciet_sim_dhx_page_csv(&mut self, ui: &mut Ui,){
        // first, get local plot page for reading only 
        // show this on the side panel

        let local_ciet_plot: PagePlotData = 
            self.ciet_plot_data;

        let latest_dhx_sthe_data: [(Time,MassRate,MassRate,
            ThermodynamicTemperature,
            ThermodynamicTemperature,
            ThermodynamicTemperature,
            ThermodynamicTemperature,
            ); NUM_DATA_PTS_IN_PLOTS] = 
            local_ciet_plot.dhx_plot_data;

        // left panel
        egui::ScrollArea::both().show(ui, |ui| {



            ui.label("Time (s), dhx shell side flowrate (kg/s),
            dhx tube side flowrate (kg/s),
            dhx shell inlet temp (degC),
            dhx shell outlet temp (degC),
            dhx tube inlet temp (degC),
            dhx tube outlet temp (degC),
            ");
            latest_dhx_sthe_data.map(|data_tuple|{
                let (time, dhx_shell_side_mass_flowrate,
                    dhx_tube_side_mass_flowrate,
                    dhx_shell_inlet_temp,
                    dhx_shell_outlet_temp,
                    dhx_tube_inlet_temp,
                    dhx_tube_outlet_temp,) = 
                    data_tuple;

                let time_seconds: f64 = 
                    (time.get::<second>()*1000.0).round()/1000.0;

                let dhx_shell_side_flowrate_kg_per_s = 
                    (dhx_shell_side_mass_flowrate.get::<kilogram_per_second>()*1000.0).round()/1000.0;
                let dhx_tube_side_flowrate_kg_per_s = 
                    (dhx_tube_side_mass_flowrate.get::<kilogram_per_second>()*1000.0).round()/1000.0;
                let dhx_shell_side_inlet_temp_degc = 
                    (dhx_shell_inlet_temp.get::<degree_celsius>()*1000.0).round()/1000.0;
                let dhx_shell_side_outlet_temp_degc = 
                    (dhx_shell_outlet_temp.get::<degree_celsius>()*1000.0).round()/1000.0;
                let dhx_tube_side_inlet_temp_degc = 
                    (dhx_tube_inlet_temp.get::<degree_celsius>()*1000.0).round()/1000.0;
                let dhx_tube_side_outlet_temp_degc = 
                    (dhx_tube_outlet_temp.get::<degree_celsius>()*1000.0).round()/1000.0;




                let dhx_data_row: String = 
                    time_seconds.to_string() + ","
                    + &dhx_shell_side_flowrate_kg_per_s.to_string() + ","
                    + &dhx_tube_side_flowrate_kg_per_s.to_string() + ","
                    + &dhx_shell_side_inlet_temp_degc.to_string() + ","
                    + &dhx_shell_side_outlet_temp_degc.to_string() + ","
                    + &dhx_tube_side_inlet_temp_degc.to_string() + ","
                    + &dhx_tube_side_outlet_temp_degc.to_string() + "," ;

                // only add the label if heater time is not equal zero 
                if time_seconds.round() as u32 != 0 {
                    ui.label(dhx_data_row);
                }


            });


        });

        // adding the return here because there are too many closing 
        // parantheses
        // just demarcates the end
        return ();

    }
    pub fn ciet_sim_dhx_branch_page(&mut self, ui: &mut Ui){
        ui.separator();

        ui.horizontal(|ui| {
            let local_ciet_state = 
                self.ciet_state.lock().unwrap().clone();
            let current_dhx_br_blocked_state: bool = 
                local_ciet_state.is_dhx_branch_blocked;
            if ui.button("Toggle DHX Branch Flow Blocking Mechanism").clicked() {


                let user_toggled_dhx_br_blocked_state: bool;

                if current_dhx_br_blocked_state == true {
                    user_toggled_dhx_br_blocked_state = false;
                } else {
                    user_toggled_dhx_br_blocked_state = true;
                };

                self.ciet_state.lock().unwrap().is_dhx_branch_blocked 
                    = user_toggled_dhx_br_blocked_state;


            }
            ui.label("DHX Branch Blocked? : ");
            ui.label(current_dhx_br_blocked_state.to_string());
        });

    }
}
