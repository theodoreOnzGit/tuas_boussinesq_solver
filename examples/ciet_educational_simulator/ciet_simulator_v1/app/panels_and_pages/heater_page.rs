
use egui_plot::{Legend, Line, Plot, PlotPoints};
use uom::si::{f64::*, power::kilowatt, thermodynamic_temperature::degree_celsius, time::second};
use std::f32::consts::TAU;

use egui::{include_image, vec2, Color32, Frame, Painter, Pos2, Rect, Sense, Stroke, TextStyle, Ui, Vec2};
use egui_extras::{Size, StripBuilder};


use crate::ciet_simulator_v1::CIETApp;

use super::ciet_data::PagePlotData;

impl CIETApp {

    pub fn ciet_sim_heater_page(&mut self, ui: &mut Ui){

        ui.horizontal(|ui| {
            ui.label("Heater Page");
        });
        ui.separator();
        ui.separator();

        // first, get local plot page for reading only 

        let local_ciet_plot: PagePlotData = 
            self.ciet_plot_data;

        let mut latest_heater_data: [(Time,Power,ThermodynamicTemperature,ThermodynamicTemperature); 500] = 
            local_ciet_plot.heater_plot_data;


        egui::ScrollArea::both().show(ui, |ui| {

            // get a text button first
            // have a horizontal ui portion
            ui.horizontal(|ui|{
                // get a button called obtain ciet data
                if ui.button("Update CIET Heater CSV Data").clicked(){
                    // spawn a new window with csv data
                    let latest_ciet_plot_data: PagePlotData = 
                        self.ciet_plot_data_mutex_ptr_for_parallel_data_transfer.lock().unwrap().clone();

                    self.ciet_plot_data = latest_ciet_plot_data;
                    latest_heater_data = latest_ciet_plot_data.heater_plot_data;



                };


            });
            ui.vertical(|ui|{

                ui.label("Time (s), Heater Power (kW), BT-11 Inlet (degC), BT-12 Outlet (degC)");
                latest_heater_data.map(|data_tuple|{
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

        });
    }

    fn semicircle_drawing(&mut self,ui: &mut Ui,) -> egui::Response {

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
