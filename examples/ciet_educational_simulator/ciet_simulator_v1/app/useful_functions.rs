use std::{ops::DerefMut, sync::{Arc, Mutex}, time::Duration};

use super::{panels_and_pages::ciet_data::{CIETState, PagePlotData}, CIETApp};

use egui::{Color32, Pos2, Rect, Ui, Widget};

impl CIETApp {
    // places a widget at some area
    pub fn put_widget_with_size_and_centre(
        &mut self, ui: &mut Ui, widget: impl Widget,
        centre_x_pixels: f32,
        centre_y_pixels: f32,
        x_width_pixels: f32,
        y_width_pixels: f32){

        let top_left_x: f32 = centre_x_pixels - 0.5 * x_width_pixels;
        let top_left_y: f32 = centre_y_pixels - 0.5 * y_width_pixels;
        let bottom_right_x: f32 = centre_x_pixels + 0.5 * x_width_pixels;
        let bottom_right_y: f32 = centre_y_pixels + 0.5 * y_width_pixels;

        let rect: Rect = Rect {
            // top left
            min: Pos2 { x: top_left_x, y: top_left_y },
            // bottom right
            max: Pos2 { x: bottom_right_x, y: bottom_right_y },
        };

        ui.put(rect, widget);

    }

    pub fn place_vertical_widget_with_length(
        &mut self, ui: &mut Ui, widget: impl Widget,
        centre_x_pixels: f32,
        centre_y_pixels: f32,
        button_length: f32,
        aspect_ratio: f32,
        ){

        // aspect ratio is length by breadth (longer side by shorter side)
        
        let y_width_pixels = button_length;
        let mut x_width_pixels = button_length/aspect_ratio;

        // min width is 30 px 
        if x_width_pixels < 30.0 {
            x_width_pixels = 30.0;
        }

        self.put_widget_with_size_and_centre(
            ui, 
            widget, 
            centre_x_pixels, 
            centre_y_pixels, 
            x_width_pixels, 
            y_width_pixels);
    }

    pub fn place_horizontal_widget_with_length(
        &mut self, ui: &mut Ui, widget: impl Widget,
        centre_x_pixels: f32,
        centre_y_pixels: f32,
        button_length: f32,
        aspect_ratio: f32,
        ){

        // aspect ratio is length by breadth (longer side by shorter side)
        
        let x_width_pixels = button_length;
        let mut y_width_pixels = button_length/aspect_ratio;
        // min width is 30 px 
        if y_width_pixels < 30.0 {
            y_width_pixels = 30.0;
        }

        self.put_widget_with_size_and_centre(
            ui, 
            widget, 
            centre_x_pixels, 
            centre_y_pixels, 
            x_width_pixels, 
            y_width_pixels);
    }

    
}



pub fn new_temp_sensitive_button(
    min_temp_degc: f32, 
    max_temp_degc: f32,
    button_temp_degc: f32,
    name: &str,
) -> egui::Button {

    let hotness: f32 = 
        (button_temp_degc - min_temp_degc)/(max_temp_degc- min_temp_degc);

    let colour_temp = hot_to_cold_colour(hotness);
    let temp_sensitive_button = egui::Button::new(name)
        .fill(colour_temp);

    temp_sensitive_button

}



/// From ChatGPT
/// Steps:
/// Cold colors (blue) start with high values in the blue channel (B = 1, G = 0).
/// Hot colors (red) end with high values in the red channel (R = 1, G = 0).
pub fn hot_to_cold_colour(hotness: f32) -> Color32 {
    let mut hotness_clone = hotness.clone();

    // ensures hotness is between 0 and 1
    if hotness_clone < 0.0 {
        hotness_clone = 0.0;
    } else if hotness_clone > 1.0 {
        hotness_clone = 1.0
    }

    let red: f32 = 255.0 * hotness_clone;
    let green: f32 = 135.0 * (1.0 - hotness_clone);
    let blue: f32 = 255.0 * (1.0 - hotness_clone);

    return Color32::from_rgb(
        red as u8, 
        green as u8, 
        blue as u8);
}


/// this is for updating plots within ciet 
use uom::si::{f64::*, heat_transfer::watt_per_square_meter_kelvin, mass_rate::kilogram_per_second, power::kilowatt, pressure::pascal, thermodynamic_temperature::degree_celsius, time::second};
pub fn update_ciet_plot_from_ciet_state(
    ciet_state_ptr: Arc<Mutex<CIETState>>,
    ciet_plot_ptr: Arc<Mutex<PagePlotData>>){

    // get current ciet state first 
    loop {

        let local_ciet_state: CIETState = 
            ciet_state_ptr.lock().unwrap().clone();

        // get the current plot object
        let mut local_ciet_plot: PagePlotData = 
            ciet_plot_ptr.lock().unwrap().clone();

        // let's get the heater data 

        let current_time: Time = Time::new::<second>(
            local_ciet_state.simulation_time_seconds);
        {

            let heater_power: Power = 
                Power::new::<kilowatt>(
                    local_ciet_state.heater_power_kilowatts);

            let inlet_temp_bt11: ThermodynamicTemperature = 
                ThermodynamicTemperature::new::<degree_celsius>(
                    local_ciet_state.get_heater_inlet_temp_degc()
                );
            let outlet_temp_bt12: ThermodynamicTemperature = 
                ThermodynamicTemperature::new::<degree_celsius>(
                    local_ciet_state.get_heater_outlet_temp_degc()
                );

            local_ciet_plot.insert_heater_data(
                current_time, heater_power, 
                inlet_temp_bt11, 
                outlet_temp_bt12);
        }

        // now let's get ctah data 

        {
            let ctah_heat_transfer_coeff: HeatTransfer = 
                HeatTransfer::new::<watt_per_square_meter_kelvin>(
                    local_ciet_state.ctah_htc_watt_per_m2_kelvin);

            let inlet_temp_bt43: ThermodynamicTemperature = 
                ThermodynamicTemperature::new::<degree_celsius>(
                    local_ciet_state.get_ctah_inlet_temp_degc());

            let outlet_temp_bt41: ThermodynamicTemperature = 
                ThermodynamicTemperature::new::<degree_celsius>(
                    local_ciet_state.get_ctah_outlet_temp_degc());

            let outlet_temp_set_pt: ThermodynamicTemperature = 
                ThermodynamicTemperature::new::<degree_celsius>(
                    local_ciet_state.bt_41_ctah_outlet_set_pt_deg_c);

            local_ciet_plot.insert_ctah_data(
                current_time, 
                ctah_heat_transfer_coeff, 
                inlet_temp_bt43, outlet_temp_bt41, 
                outlet_temp_set_pt);

        }
        // ctah pump data 
        {
            let ctah_pump_pressure: Pressure = 
                Pressure::new::<pascal>(
                    local_ciet_state.get_ctah_pump_pressure_f64());

            let ctah_br_mass_flowrate: MassRate = 
                MassRate::new::<kilogram_per_second>(
                    local_ciet_state.fm40_ctah_branch_kg_per_s
                );

            let ctah_pump_temperature: ThermodynamicTemperature = 
                ThermodynamicTemperature::new::<degree_celsius>(
                    local_ciet_state.ctah_pump_temp_degc);

            local_ciet_plot.insert_ctah_pump_data(
                current_time, 
                ctah_pump_pressure, 
                ctah_br_mass_flowrate, 
                ctah_pump_temperature);

        }

        // dhx data 
        {
            let dhx_shell_side_mass_flowrate = 
                MassRate::new::<kilogram_per_second>(
                    local_ciet_state.fm20_dhx_branch_kg_per_s as f64
                );
            let dhx_tube_side_mass_flowrate = 
                MassRate::new::<kilogram_per_second>(
                    local_ciet_state.fm_60_dracs_kg_per_s
                );
            let dhx_shell_side_inlet_temp = 
                ThermodynamicTemperature::new::<degree_celsius>(
                    local_ciet_state.get_dhx_shell_inlet_temp_degc()
                );
            let dhx_shell_side_outlet_temp = 
                ThermodynamicTemperature::new::<degree_celsius>(
                    local_ciet_state.get_dhx_shell_outlet_temp_degc()
                );
            let dhx_tube_side_inlet_temp = 
                ThermodynamicTemperature::new::<degree_celsius>(
                    local_ciet_state.get_dhx_tube_inlet_temp_degc()
                );
            let dhx_tube_side_outlet_temp = 
                ThermodynamicTemperature::new::<degree_celsius>(
                    local_ciet_state.get_dhx_tube_outlet_temp_degc()
                );

            
            local_ciet_plot.insert_dhx_data(
                current_time, 
                dhx_shell_side_mass_flowrate, 
                dhx_tube_side_mass_flowrate, 
                dhx_shell_side_inlet_temp, 
                dhx_shell_side_outlet_temp, 
                dhx_tube_side_inlet_temp, 
                dhx_tube_side_outlet_temp);
        }


        // tchx data
        {
            let tchx_heat_transfer_coeff: HeatTransfer = 
                HeatTransfer::new::<watt_per_square_meter_kelvin>(
                    local_ciet_state.tchx_htc_watt_per_m2_kelvin);

            let inlet_temp_bt43: ThermodynamicTemperature = 
                ThermodynamicTemperature::new::<degree_celsius>(
                    local_ciet_state.get_tchx_inlet_temp_degc());

            let outlet_temp_bt41: ThermodynamicTemperature = 
                ThermodynamicTemperature::new::<degree_celsius>(
                    local_ciet_state.get_tchx_outlet_temp_degc());

            let outlet_temp_set_pt: ThermodynamicTemperature = 
                ThermodynamicTemperature::new::<degree_celsius>(
                    local_ciet_state.bt_66_tchx_outlet_set_pt_deg_c);

            local_ciet_plot.insert_tchx_data(
                current_time, 
                tchx_heat_transfer_coeff, 
                inlet_temp_bt43, outlet_temp_bt41, 
                outlet_temp_set_pt);

        }

        

        // update the plot
        *ciet_plot_ptr.lock().unwrap().deref_mut()
            = local_ciet_plot;

        // historian records every 100ms 
        std::thread::sleep(Duration::from_millis(100));
    }

}
