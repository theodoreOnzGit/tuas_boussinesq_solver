
use egui_plot::{Legend, Line, Plot, PlotPoints};
use uom::si::{f64::*, mass_rate::kilogram_per_second, power::kilowatt, pressure::pascal, thermodynamic_temperature::degree_celsius, time::second};

use egui::{vec2, Color32, Sense, Stroke, Ui, Vec2};


use crate::ciet_simulator_v1::CIETApp;

use super::ciet_data::{PagePlotData, NUM_DATA_PTS_IN_PLOTS};

impl CIETApp {

    pub fn ciet_sim_ctah_pump_page_csv(&mut self, ui: &mut Ui,){
        // first, get local plot page for reading only 
        // show this on the side panel

        let local_ciet_plot: PagePlotData = 
            self.ciet_plot_data;

        let latest_ctah_pump_data: [(Time,Pressure,MassRate,ThermodynamicTemperature); NUM_DATA_PTS_IN_PLOTS] = 
            local_ciet_plot.ctah_pump_plot_data;

        // left panel
        egui::ScrollArea::both().show(ui, |ui| {



            ui.label("Time (s), CTAH Pump Pressure loop pressure drop (Pa), CTAH Branch Mass Flowrate (kg/s), CTAH Pump Temp (degC)");
            latest_ctah_pump_data.map(|data_tuple|{
                let (time, pump_pressure, ctah_br_mass_flowrate, ctah_pump_temp) = 
                    data_tuple;

                let time_seconds: f64 = 
                    (time.get::<second>()*1000.0).round()/1000.0;

                let pump_pressure_pascal: f64 = 
                    (pump_pressure.get::<pascal>()*1000.0).round()/1000.0;
                let ctah_branch_mass_flowrate_kg_per_s: f64 = 
                    (ctah_br_mass_flowrate.get::<kilogram_per_second>()*1000.0).round()/1000.0;

                let ctah_pump_temp_degc: f64 = 
                    (ctah_pump_temp.get::<degree_celsius>()*1000.0).round()/1000.0;


                let heater_data_row: String = 
                    time_seconds.to_string() + ","
                    + &pump_pressure_pascal.to_string() + ","
                    + &ctah_branch_mass_flowrate_kg_per_s.to_string() + ","
                    + &ctah_pump_temp_degc.to_string() + "," ;

                // only add the label if heater time is not equal zero 
                if time_seconds.round() as u32 != 0 {
                    ui.label(heater_data_row);
                }


            });


        });

    }
    pub fn ciet_sim_ctah_pump_page_and_graphs(&mut self, ui: &mut Ui){
        // headings 

        //
        ui.horizontal(|ui| {
            ui.label("CTAH Pump Page");
            if ui.button("Update CSV Data").clicked(){
                // spawn a new window with csv data
                let latest_ciet_plot_data: PagePlotData = 
                    self.ciet_plot_data_mutex_ptr_for_parallel_data_transfer.lock().unwrap().clone();

                self.ciet_plot_data = latest_ciet_plot_data;

            };
        });
        // toggle flow blocking in CTAH branch
        ui.separator();
        ui.horizontal(|ui| {
            let local_ciet_state = 
                self.ciet_state.lock().unwrap().clone();
            let current_ctah_br_blocked_state: bool = 
                local_ciet_state.is_ctah_branch_blocked;
            if ui.button("Toggle CTAH Branch Flow Blocking Mechanism").clicked() {


                let user_toggled_ctah_br_blocked_state: bool;

                if current_ctah_br_blocked_state == true {
                    user_toggled_ctah_br_blocked_state = false;
                } else {
                    user_toggled_ctah_br_blocked_state = true;
                };

                self.ciet_state.lock().unwrap().is_ctah_branch_blocked 
                    = user_toggled_ctah_br_blocked_state;


            }
            ui.label("CTAH Branch Blocked? : ");
            ui.label(current_ctah_br_blocked_state.to_string());
        });
        ui.separator();
        // graphs
        egui::ScrollArea::both().show(ui, |ui| {

            ui.heading("CTAH Pump Temperature vs Time");
            let mut ctah_pump_temp_plot = Plot::new("CTAH Pump temp degC").legend(Legend::default());
            ctah_pump_temp_plot = ctah_pump_temp_plot.width(800.0);
            ctah_pump_temp_plot = ctah_pump_temp_plot.view_aspect(16.0/9.0);
            ctah_pump_temp_plot = ctah_pump_temp_plot.x_axis_label(
                "time (seconds), current time (seconds): ".to_owned() 
            );
            ctah_pump_temp_plot = ctah_pump_temp_plot.y_axis_label(
                "temperature degree_celsius".to_owned());
            let latest_ciet_plot_data: PagePlotData = 
                self.ciet_plot_data_mutex_ptr_for_parallel_data_transfer.lock().unwrap().clone();
            let time_ctah_pump_temp_vec: Vec<[f64;2]> = 
                latest_ciet_plot_data.get_ctah_pump_temp_degc_vs_time_secs_vec();

            ctah_pump_temp_plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::from(
                            time_ctah_pump_temp_vec.clone()
                )).name("CTAH Pump temperature deg C"));
                //plot_ui.line(Line::new(PlotPoints::from(
                //            time_simulated_reactor_feedback_outlet_temp_vec.clone()
                //)).name("simulated reactivity bt12 (heater outlet) temperature deg C"));
            });

            // 


            // to be completed
            let time_ctah_pump_pressure_pascals_vec: Vec<[f64;2]> = 
                latest_ciet_plot_data.get_ctah_pump_pressure_pascals_vs_time_secs_vec();
            let time_ctah_mass_flowrate_vec: Vec<[f64;2]> = 
                latest_ciet_plot_data.get_ctah_br_mass_kg_per_s_vs_time_secs_vec();

            ui.heading("CTAH Pump Pressure vs Time");
            let mut ctah_pump_temp_plot = Plot::new("CTAH Pump Pressure Pascals").legend(Legend::default());
            ctah_pump_temp_plot = ctah_pump_temp_plot.width(800.0);
            ctah_pump_temp_plot = ctah_pump_temp_plot.view_aspect(16.0/9.0);
            ctah_pump_temp_plot = ctah_pump_temp_plot.x_axis_label(
                "time (seconds), current time (seconds): ".to_owned() 
            );
            ctah_pump_temp_plot = ctah_pump_temp_plot.y_axis_label(
                "Pressure (Pa)".to_owned());

            ctah_pump_temp_plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::from(
                            time_ctah_pump_pressure_pascals_vec.clone()
                )).name("CTAH Pump Pressure Pascals"));
                //plot_ui.line(Line::new(PlotPoints::from(
                //            time_simulated_reactor_feedback_outlet_temp_vec.clone()
                //)).name("simulated reactivity bt12 (heater outlet) temperature deg C"));
            });

            ui.heading("CTAH Branch Mass Flowrate vs Time");
            let mut ctah_pump_temp_plot = Plot::new("CTAH Pump Mass Flowrate kg/s").legend(Legend::default());
            ctah_pump_temp_plot = ctah_pump_temp_plot.width(800.0);
            ctah_pump_temp_plot = ctah_pump_temp_plot.view_aspect(16.0/9.0);
            ctah_pump_temp_plot = ctah_pump_temp_plot.x_axis_label(
                "time (seconds), current time (seconds): ".to_owned() 
            );
            ctah_pump_temp_plot = ctah_pump_temp_plot.y_axis_label(
                "Mass Flowrate (kg/s)".to_owned());

            ctah_pump_temp_plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::from(
                            time_ctah_mass_flowrate_vec.clone()
                )).name("CTAH Branch Mass Flowrate"));
                //plot_ui.line(Line::new(PlotPoints::from(
                //            time_simulated_reactor_feedback_outlet_temp_vec.clone()
                //)).name("simulated reactivity bt12 (heater outlet) temperature deg C"));
            });

            self.citation_disclaimer_and_acknowledgements(ui);

        });

        // ends everything, 
        // adding this return (); for code readability 
        // cos there are too many closing parantheses
        return ();


    }
}
