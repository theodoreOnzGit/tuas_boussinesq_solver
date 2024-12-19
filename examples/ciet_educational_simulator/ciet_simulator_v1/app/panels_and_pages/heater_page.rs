
use egui_plot::{Legend, Line, Plot, PlotPoints};
use uom::si::{f64::*, power::kilowatt, thermodynamic_temperature::degree_celsius, time::second};

use egui::{vec2, Color32, Sense, Stroke, Ui, Vec2};


use crate::ciet_simulator_v1::CIETApp;

use super::ciet_data::PagePlotData;

impl CIETApp {

    pub fn ciet_sim_heater_page_csv(&mut self, ui: &mut Ui,){
        // first, get local plot page for reading only 
        // show this on the side panel

        let local_ciet_plot: PagePlotData = 
            self.ciet_plot_data.clone();

        let latest_heater_data: Vec<(Time,Power,ThermodynamicTemperature,ThermodynamicTemperature)> = 
            local_ciet_plot.heater_plot_data;

        // left panel
        egui::ScrollArea::both().show(ui, |ui| {

            // obtain a lock for the ciet data 
            // ptr 
            let mut ciet_data_global_ptr_lock = 
                self.ciet_plot_data_mutex_ptr_for_parallel_data_transfer
                .lock().unwrap();
            
            let record_interval_seconds = egui::Slider::new(
                &mut ciet_data_global_ptr_lock.data_record_interval_seconds, 
                0.01..=1000.0)
                .logarithmic(true)
                .text("Recording Interval (Seconds)")
                .drag_value_speed(0.001);

            ui.add(record_interval_seconds);


            ui.label("Time (s), Heater Power (kW), BT-11 Inlet (degC), BT-12 Outlet (degC)");
            latest_heater_data.iter().for_each(|data_tuple|{
                let (time, power, bt11, bt12) = 
                    data_tuple;

                let time_seconds: f64 = 
                    (time.get::<second>()*1000.0).round()/1000.0;

                let power_kw: f64 = 
                    (power.get::<kilowatt>()*1000.0).round()/1000.0;
                let bt11_degc: f64 = 
                    (bt11.get::<degree_celsius>()*1000.0).round()/1000.0;

                let bt12_degc: f64 = 
                    (bt12.get::<degree_celsius>()*1000.0).round()/1000.0;


                let heater_data_row: String = 
                    time_seconds.to_string() + ","
                    + &power_kw.to_string() + ","
                    + &bt11_degc.to_string() + ","
                    + &bt12_degc.to_string() + "," ;

                // only add the label if heater time is not equal zero 
                if time_seconds.round() as u32 != 0 {
                    ui.label(heater_data_row);
                }


            });



        });


        // am placing this here because it is kind of hard to 
        // make sense of all the parantheses
        return ();

    }

    pub fn ciet_sim_heater_page_graph(&mut self, ui: &mut Ui){

        ui.horizontal(|ui| {
            ui.label("Heater Page");
            if ui.button("Update CSV Data").clicked(){
                // spawn a new window with csv data
                let latest_ciet_plot_data: PagePlotData = 
                    self.ciet_plot_data_mutex_ptr_for_parallel_data_transfer
                    .lock().unwrap().clone();

                self.ciet_plot_data = latest_ciet_plot_data;

            };
        });
        ui.separator();
        ui.separator();



        // panel for graphs
        //
        //

        egui::ScrollArea::both().show(ui, |ui| {
            let mut bt11_bt12_temp_plot = Plot::new("heater inlet and outlet temp degC").legend(Legend::default());

            // sets the aspect for plot 
            bt11_bt12_temp_plot = bt11_bt12_temp_plot.width(800.0);
            bt11_bt12_temp_plot = bt11_bt12_temp_plot.view_aspect(16.0/9.0);
            //bt11_bt12_temp_plot = bt11_bt12_temp_plot.data_aspect(2.5);
            // deprecated methods
            //bt11_bt12_temp_plot = bt11_bt12_temp_plot.auto_bounds_x();
            //bt11_bt12_temp_plot = bt11_bt12_temp_plot.auto_bounds_y();

            bt11_bt12_temp_plot = bt11_bt12_temp_plot.x_axis_label(
                "time (seconds), current time (seconds): ".to_owned() 
            );
            bt11_bt12_temp_plot = bt11_bt12_temp_plot.y_axis_label(
                "temperature degree_celsius".to_owned());
            let latest_ciet_plot_data: PagePlotData = 
                self.ciet_plot_data_mutex_ptr_for_parallel_data_transfer.lock().unwrap().clone();


            // let's make the time and bt11 vector
            let time_bt11_vec: Vec<[f64;2]> = 
                latest_ciet_plot_data.get_bt_11_degc_vs_time_secs_vec();

            let time_bt12_vec: Vec<[f64;2]> = 
                latest_ciet_plot_data.get_bt_12_degc_vs_time_secs_vec();

            ui.heading("Heater Inlet and Outlet Temperature vs Time");
            bt11_bt12_temp_plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::from(
                            time_bt11_vec.clone()
                )).name("bt11 (heater inlet) temperature deg C"));
                plot_ui.line(Line::new(PlotPoints::from(
                            time_bt12_vec.clone()
                )).name("bt12 (heater outlet) temperature deg C"));
                //plot_ui.line(Line::new(PlotPoints::from(
                //            time_simulated_reactor_feedback_outlet_temp_vec.clone()
                //)).name("simulated reactivity bt12 (heater outlet) temperature deg C"));
            });


            let time_heater_power_vec: Vec<[f64;2]> = 
                latest_ciet_plot_data.get_heater_power_kw_vs_time_secs_vec();

            ui.separator();
            let mut power_plot = Plot::new("heater power plot").legend(Legend::default());

            // sets the aspect for plot 
            power_plot = power_plot.width(800.0);
            power_plot = power_plot.view_aspect(16.0/9.0);
            //power_plot = power_plot.data_aspect(2.5);

            power_plot = power_plot.x_axis_label(
                "time (seconds)");
            power_plot = power_plot.y_axis_label(
                "heater power (kW)".to_owned());


            ui.heading("Heater Power vs Time");
            power_plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::from(
                            time_heater_power_vec
                )).name("Heater Power (kW)"));
            });

            self.citation_disclaimer_and_acknowledgements(ui);

        });


 


    }


    fn _semicircle_drawing(&mut self,ui: &mut Ui,) -> egui::Response {

        let size = Vec2::splat(160.0);
        //let (mut response, painter) =
        //    ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

        let (response, painter) = ui.allocate_painter(size, Sense::hover());
        let rect = response.rect;
        
        let c = rect.center();
        let r = rect.width() / 2.0 - 1.0;
        let color = Color32::from_gray(128);
        let stroke = Stroke::new(1.0, color);
        painter.circle_stroke(c, r, stroke);
        painter.line_segment([c - vec2(0.0, r), c + vec2(0.0, r)], stroke);
        //painter.line_segment([c, c + r * Vec2::angled(TAU * 1.0 / 8.0)], stroke);
        //painter.line_segment([c, c + r * Vec2::angled(TAU * 3.0 / 8.0)], stroke);
        

        response
    }
}
