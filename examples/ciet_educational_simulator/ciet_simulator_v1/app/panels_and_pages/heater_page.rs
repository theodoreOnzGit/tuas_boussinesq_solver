
use egui_plot::{Legend, Line, Plot, PlotPoints};
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
            self.ciet_plot_data.lock().unwrap().clone();


        egui::ScrollArea::both().show(ui, |ui| {

            // have a horizontal ui portion
            ui.horizontal(|ui|{
                // get a button called obtain ciet data
                if ui.button("Get CIET Heater CSV Data").clicked(){
                    // spawn a new window with csv data
                    // or just display the current updated data

                }
            });

            Frame::canvas(ui.style()).show(ui, |ui| {
                self.semicircle_drawing(ui);
            });

            // painting image over whatever is in the ui
            let rect: Rect = Rect {
                // top left
                min: Pos2 { x: 350.5, y: 350.5 },
                // bottom right
                max: Pos2 { x: 550.5, y: 500.5 },
            };
            //let rect = egui::Rect::from_min_size(Default::default(), egui::Vec2::splat(100.0));
            //let _ferris = egui::Image::new(include_image!("../../ferris.png"))
            //    .rounding(5.0)
            //    .paint_at(ui, rect);
            // now I'd like to paint widgets too, at specific spots, so as to show values of 
            // the temperature values in and out next to the picture of the 
            // heater
            let _ferris2 = egui::Image::new(include_image!("../../ferris.png"))
                .rounding(5.0);

            ui.add(
                egui::Slider::new(&mut self.value, 0.0..=100.0)
            );

            let rect_two: Rect = Rect {
                // top left
                min: Pos2 { x: 300.5, y: 350.5 },
                // bottom right
                max: Pos2 { x: 550.5, y: 500.5 },
            };

            let slider_vert = egui::Slider::new(
                &mut self.value, 0.0..=100.0)
                .vertical();

            ui.put(rect_two, slider_vert);

            // it seems images can also be widgets
            // it may be easier/more consistent to do things like that
            let _ferris2 = egui::Image::new(include_image!("../../ferris.png"))
                .rounding(5.0);

            ui.put(rect, _ferris2);
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
