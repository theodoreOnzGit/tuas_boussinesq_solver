use egui::Ui;

use crate::ciet_simulator_v1::CIETApp;

use uom::si::{f64::*, heat_transfer::watt_per_square_meter_kelvin, thermodynamic_temperature::degree_celsius, time::second};
use egui_plot::{Legend, Line, Plot, PlotPoints};
use super::ciet_data::{PagePlotData, NUM_DATA_PTS_IN_PLOTS};

impl CIETApp {
    pub fn ciet_sim_tchx_page_csv(&mut self, ui: &mut Ui){
        // show this on the side panel

        let local_ciet_plot: PagePlotData = 
            self.ciet_plot_data;

        let latest_tchx_data: [(Time,HeatTransfer,ThermodynamicTemperature,ThermodynamicTemperature
            ,ThermodynamicTemperature); NUM_DATA_PTS_IN_PLOTS] = 
            local_ciet_plot.tchx_plot_data;

        // left panel
        egui::ScrollArea::both().show(ui, |ui| {



            ui.label("Time (s), TCHX htc (watts per m2 kelvin), BT-65 Inlet (degC), BT-66 Outlet (degC), BT-66 setpt (degC)");
            latest_tchx_data.map(|data_tuple|{
                let (time, tchx_htc, bt65, bt66,bt66_setpt) = 
                    data_tuple;

                let time_seconds: f64 = 
                    (time.get::<second>()*1000.0).round()/1000.0;

                let tchx_htc_watts_per_m2_kelvin: f64 = 
                    (tchx_htc.get::<watt_per_square_meter_kelvin>()*1000.0).round()/1000.0;
                let bt65_degc: f64 = 
                    (bt65.get::<degree_celsius>()*1000.0).round()/1000.0;

                let bt66_degc: f64 = 
                    (bt66.get::<degree_celsius>()*1000.0).round()/1000.0;
                let bt66_setpt_degc: f64 = 
                    (bt66_setpt.get::<degree_celsius>()*1000.0).round()/1000.0;


                let tchx_data_row: String = 
                    time_seconds.to_string() + ","
                    + &tchx_htc_watts_per_m2_kelvin.to_string() + ","
                    + &bt65_degc.to_string() + ","
                    + &bt66_degc.to_string() + "," 
                    + &bt66_setpt_degc.to_string() + "," 
                    ;

                // only add the label if heater time is not equal zero 
                if time_seconds.round() as u32 != 0 {
                    ui.label(tchx_data_row);
                }


            });


        });

        // just putting this here because of all the indents, can 
        // be v confusing
        return ();

    }

    // 
    //
    pub fn ciet_sim_tchx_page_graph(&mut self, ui: &mut Ui,){

        ui.horizontal(|ui| {
            ui.label("CTAH Page");
            if ui.button("Update CSV Data").clicked(){
                // spawn a new window with csv data
                let latest_ciet_plot_data: PagePlotData = 
                    self.ciet_plot_data_mutex_ptr_for_parallel_data_transfer.lock().unwrap().clone();

                self.ciet_plot_data = latest_ciet_plot_data;

            };
        });
        ui.separator();
        ui.separator();



        // panel for graphs
        //
        //

        egui::ScrollArea::both().show(ui, |ui| {
            let mut bt65_bt66_temp_plot = Plot::new("heater inlet and outlet temp degC").legend(Legend::default());

            // sets the aspect for plot 
            bt65_bt66_temp_plot = bt65_bt66_temp_plot.width(800.0);
            bt65_bt66_temp_plot = bt65_bt66_temp_plot.view_aspect(16.0/9.0);
            //bt11_bt12_temp_plot = bt11_bt12_temp_plot.data_aspect(2.5);
            // deprecated methods
            //bt11_bt12_temp_plot = bt11_bt12_temp_plot.auto_bounds_x();
            //bt11_bt12_temp_plot = bt11_bt12_temp_plot.auto_bounds_y();

            bt65_bt66_temp_plot = bt65_bt66_temp_plot.x_axis_label(
                "time (seconds), current time (seconds): ".to_owned() 
            );
            bt65_bt66_temp_plot = bt65_bt66_temp_plot.y_axis_label(
                "temperature degree_celsius".to_owned());
            let latest_ciet_plot_data: PagePlotData = 
                self.ciet_plot_data_mutex_ptr_for_parallel_data_transfer.lock().unwrap().clone();


            // let's make the time and bt11 vector
            let time_bt65_vec: Vec<[f64;2]> = 
                latest_ciet_plot_data.get_bt_65_degc_vs_time_secs_vec();

            let time_bt66_vec: Vec<[f64;2]> = 
                latest_ciet_plot_data.get_bt_66_degc_vs_time_secs_vec();
            let time_bt66_setpt_vec: Vec<[f64;2]> = 
                latest_ciet_plot_data.get_bt_66_setpt_degc_vs_time_secs_vec();

            bt65_bt66_temp_plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::from(
                            time_bt65_vec.clone()
                )).name("bt65 (tchx inlet) temperature deg C"));
                plot_ui.line(Line::new(PlotPoints::from(
                            time_bt66_vec.clone()
                )).name("bt66 (tchx outlet) temperature deg C"));


                plot_ui.line(Line::new(PlotPoints::from(
                            time_bt66_setpt_vec.clone()
                )).name("bt66 (tchx outlet) set pt deg C"));
                //plot_ui.line(Line::new(PlotPoints::from(
                //            time_simulated_reactor_feedback_outlet_temp_vec.clone()
                //)).name("simulated reactivity bt12 (heater outlet) temperature deg C"));
            });


            let time_tchx_htc_vec: Vec<[f64;2]> = 
                latest_ciet_plot_data.get_tchx_htc_watts_per_m2_kelvin_vs_time_secs_vec();

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
            power_plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::from(
                            time_tchx_htc_vec
                )));
            });

            self.citation_disclaimer_and_acknowledgements(ui);
        });


        // i added this return to show the end of the function
        // just for readability 
        // because there are too many closing parantheses
        return ();




    }
}
