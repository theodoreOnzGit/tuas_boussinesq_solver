
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



            ui.label("Time (s), dhx shell side flowrate (kg/s), dhx tube side flowrate (kg/s), dhx shell inlet temp (degC), dhx shell outlet temp (degC), dhx tube inlet temp (degC), dhx tube outlet temp (degC), ");
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
    pub fn ciet_sim_dhx_branch_page_graph(&mut self, ui: &mut Ui){
        ui.horizontal(|ui| {
            ui.label("DHX Page");
            if ui.button("Update CSV Data").clicked(){
                // spawn a new window with csv data
                let latest_ciet_plot_data: PagePlotData = 
                    self.ciet_plot_data_mutex_ptr_for_parallel_data_transfer.lock().unwrap().clone();

                self.ciet_plot_data = latest_ciet_plot_data;

            };
        });
        ui.separator();
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

        ui.separator();

        // panel for graphs
        egui::ScrollArea::both().show(ui, |ui| {
            let mut bt43_bt41_temp_plot = Plot::new("heater inlet and outlet temp degC").legend(Legend::default());

            // sets the aspect for plot 
            bt43_bt41_temp_plot = bt43_bt41_temp_plot.width(800.0);
            bt43_bt41_temp_plot = bt43_bt41_temp_plot.view_aspect(16.0/9.0);
            //bt11_bt12_temp_plot = bt11_bt12_temp_plot.data_aspect(2.5);
            // deprecated methods
            //bt11_bt12_temp_plot = bt11_bt12_temp_plot.auto_bounds_x();
            //bt11_bt12_temp_plot = bt11_bt12_temp_plot.auto_bounds_y();

            bt43_bt41_temp_plot = bt43_bt41_temp_plot.x_axis_label(
                "time (seconds), current time (seconds): ".to_owned() 
            );
            bt43_bt41_temp_plot = bt43_bt41_temp_plot.y_axis_label(
                "temperature degree_celsius".to_owned());
            let latest_ciet_plot_data: PagePlotData = 
                self.ciet_plot_data_mutex_ptr_for_parallel_data_transfer.lock().unwrap().clone();


            // let's make the time and bt11 vector
            let time_bt43_vec: Vec<[f64;2]> = 
                latest_ciet_plot_data.get_bt_43_degc_vs_time_secs_vec();

            let time_bt41_vec: Vec<[f64;2]> = 
                latest_ciet_plot_data.get_bt_41_degc_vs_time_secs_vec();
            let time_bt41_setpt_vec: Vec<[f64;2]> = 
                latest_ciet_plot_data.get_bt_41_setpt_degc_vs_time_secs_vec();

            ui.heading("CTAH Inlet and Outlet Temperature vs Time");
            bt43_bt41_temp_plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::from(
                            time_bt43_vec.clone()
                )).name("bt43 (ctah inlet) temperature deg C"));
                plot_ui.line(Line::new(PlotPoints::from(
                            time_bt41_vec.clone()
                )).name("bt41 (ctah outlet) temperature deg C"));


                plot_ui.line(Line::new(PlotPoints::from(
                            time_bt41_setpt_vec.clone()
                )).name("bt41 (ctah outlet) set pt deg C"));
                //plot_ui.line(Line::new(PlotPoints::from(
                //            time_simulated_reactor_feedback_outlet_temp_vec.clone()
                //)).name("simulated reactivity bt12 (heater outlet) temperature deg C"));
            });


            let time_ctah_htc_vec: Vec<[f64;2]> = 
                latest_ciet_plot_data.get_ctah_htc_watts_per_m2_kelvin_vs_time_secs_vec();

            ui.separator();
            let mut power_plot = Plot::new("CTAH heat trf coeff plot").legend(Legend::default());

            // sets the aspect for plot 
            power_plot = power_plot.width(800.0);
            power_plot = power_plot.view_aspect(16.0/9.0);
            //power_plot = power_plot.data_aspect(2.5);

            power_plot = power_plot.x_axis_label(
                "time (seconds)");
            power_plot = power_plot.y_axis_label(
                "htc (watts per m2 kelvin)".to_owned());

            ui.heading("CTAH Heat Transfer Coefficient (HTC) vs Time");
            power_plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::from(
                            time_ctah_htc_vec
                )));
            });


            self.citation_disclaimer_and_acknowledgements(ui);

        });

        // adding the return here because there are too many closing 
        // parantheses
        // just demarcates the end
        return ();


    }
}
