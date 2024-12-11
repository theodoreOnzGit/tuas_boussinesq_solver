use egui::{include_image, Color32, Image, TextStyle, Ui};
use egui_extras::{Size, StripBuilder};

use crate::ciet_simulator_v1::{app::useful_functions::{hot_to_cold_colour, new_temp_sensitive_button}, CIETApp};

use super::ciet_data::CIETState;

impl CIETApp {

    pub fn ciet_sim_main_page(&mut self, ui: &mut Ui){

        // obtain a lock first to display the information 
        
        egui::ScrollArea::both().show(ui, |ui| {
            self.insert_read_and_update_widgets(ui);

            self.insert_pictures(ui);
        });

    }

    /// inserts sliders and other widgets for ciet 
    fn insert_read_and_update_widgets(&mut self, ui: &mut Ui,){

        // obtain a lock for ciet state first, clone it
        // and drop the lock
        let mut ciet_state_local: CIETState 
            = self.ciet_state.lock().unwrap().clone();

        // manually set coordinates
        let (tchx_x, tchx_y): (f32, f32) = (150.0, 260.0);
        let (tchx_x_width, tchx_y_width): (f32, f32) = (150.0, 150.0);
        let dhx_x = tchx_x + 250.0;
        let dhx_y = tchx_y + 250.0;
        let dhx_x_width = tchx_x_width;
        let dhx_y_width = tchx_y_width;
        let heater_x = dhx_x + 350.0;
        let heater_y = dhx_y + 50.0;
        let heater_x_width = dhx_x_width;
        let heater_y_width = dhx_y_width;
        let ctah_x = heater_x + 750.0;
        let ctah_y = tchx_y;
        let ctah_x_width = dhx_x_width;
        let ctah_y_width = dhx_y_width;
        let ctah_pump_x = ctah_x - 50.0;
        let ctah_pump_y = heater_y + 270.0;
        let ctah_pump_x_width = dhx_x_width;
        let ctah_pump_y_width = dhx_y_width;

        // time display 
        let sim_time_seconds = ciet_state_local.simulation_time_seconds;
        let elapsed_time_seconds = ciet_state_local.elapsed_time_seconds;
        let calc_time_ms = ciet_state_local.calc_time_ms;

        let time_display_text = 
            "Simulation Time (s): ".to_string() 
            + &sim_time_seconds.to_string()
            + " ; Elapsed Time (s) :"
            + &elapsed_time_seconds.to_string()
            + " ;  Calc Time (ms): "
            + &calc_time_ms.to_string();
        let time_display_label = egui::Label::new(time_display_text);
        ui.add(time_display_label);

        ui.separator();

        // buttons with custom colour 
        //let colour_fill = Color32::from_rgb(155, 100, 100);
        //let coloured_button = egui::Button::new("test button")
        //    .fill(colour_fill);
        //ui.add_enabled(false, coloured_button);

        ui.separator();

        // for user to set heater power
        let heater_set_pt_slider_kw = egui::Slider::new(
            &mut ciet_state_local.heater_power_kilowatts, 0.0..=10.0)
            .vertical()
            .text("Heater Power (kW)");

        let heater_slider_x = heater_x + 0.7 * heater_x_width;
        let heater_slider_y = heater_y + 10.0;
        let heater_slider_x_width = 30.0;
        let heater_slider_y_width = heater_y_width;

        self.put_widget_with_size_and_centre(
            ui, 
            heater_set_pt_slider_kw, 
            heater_slider_x, 
            heater_slider_y, 
            heater_slider_x_width, 
            heater_slider_y_width);

        // heater outlet temp and inlet temp
        let heater_out_temp_degc: f64 = 
            ciet_state_local.get_heater_outlet_temp_degc();

        let heater_display_text_outlet: String = 
            "Outlet BT-12 (degC): ".to_string() + &heater_out_temp_degc.to_string();

        let heater_outlet_label = egui::Label::new(&heater_display_text_outlet);

        self.put_widget_with_size_and_centre(
            ui, 
            heater_outlet_label, 
            heater_slider_x + 45.0, 
            heater_slider_y - 90.0, 
            heater_slider_x_width + 220.0, 
            heater_slider_y_width * 0.2);

        let heater_in_temp_degc: f64 = 
            ciet_state_local.get_heater_inlet_temp_degc();

        let heater_display_text_inlet: String = 
            "Inlet BT-11 (degC): ".to_string() + &heater_in_temp_degc.to_string();

        let heater_inlet_label = egui::Label::new(
            &heater_display_text_inlet);

        self.put_widget_with_size_and_centre(
            ui, 
            heater_inlet_label, 
            heater_slider_x + 45.0, 
            heater_slider_y + 90.0, 
            heater_slider_x_width + 220.0, 
            heater_slider_y_width*0.2);


        // for user to set CTAH and TCHX cooler set points
        let tchx_slider_outlet_set_pt_degc = egui::Slider::new(
            &mut ciet_state_local.bt_66_tchx_outlet_set_pt_deg_c, 25.0..=110.0)
            .vertical()
            .text("TCHX Outlet Set Pt (degC)");
        let tchx_slider_x = tchx_x + 0.7 * tchx_x_width;
        let tchx_slider_y = tchx_y + 10.0;
        let tchx_slider_x_width = 30.0;
        let tchx_slider_y_width = tchx_y_width;

        self.put_widget_with_size_and_centre(
            ui, 
            tchx_slider_outlet_set_pt_degc, 
            tchx_slider_x, 
            tchx_slider_y, 
            tchx_slider_x_width, 
            tchx_slider_y_width);

        let tchx_top_temp = ciet_state_local.get_tchx_inlet_temp_degc();
        let tchx_bottom_temp = ciet_state_local.get_tchx_outlet_temp_degc();

        let tchx_top_label = egui::Label::new(
            "Inlet BT-65 (degC): ".to_string() 
            + &tchx_top_temp.to_string()
            );

        let tchx_bottom_label = egui::Label::new(
            "Outlet BT-66 (degC): ".to_string() 
            + &tchx_bottom_temp.to_string()
            );

        self.put_widget_with_size_and_centre(
            ui, 
            tchx_top_label, 
            tchx_slider_x + 55.0, 
            tchx_slider_y - 90.0, 
            tchx_slider_x_width + 120.0, 
            tchx_slider_y_width * 0.2);

        self.put_widget_with_size_and_centre(
            ui, 
            tchx_bottom_label, 
            tchx_slider_x + 55.0, 
            tchx_slider_y + 90.0, 
            tchx_slider_x_width + 120.0, 
            tchx_slider_y_width * 0.2);

        let ctah_slider_outlet_set_pt_degc = egui::Slider::new(
            &mut ciet_state_local.bt_41_ctah_outlet_set_pt_deg_c, 25.0..=110.0)
            .vertical()
            .text("CTAH Outlet Set Pt (degC)");

        let ctah_slider_x = ctah_x + 0.7 * ctah_x_width;
        let ctah_slider_y = ctah_y + 10.0;
        let ctah_slider_x_width = 30.0;
        let ctah_slider_y_width = ctah_y_width;

        self.put_widget_with_size_and_centre(
            ui, 
            ctah_slider_outlet_set_pt_degc, 
            ctah_slider_x, 
            ctah_slider_y, 
            ctah_slider_x_width, 
            ctah_slider_y_width);

        let ctah_top_temp = ciet_state_local.get_ctah_inlet_temp_degc();
        let ctah_bottom_temp = ciet_state_local.get_ctah_outlet_temp_degc();

        let ctah_top_label = egui::Label::new(
            "Inlet BT-43 (degC): ".to_string() 
            + &ctah_top_temp.to_string()
            );

        let ctah_bottom_label = egui::Label::new(
            "Outlet BT-41 (degC): ".to_string() 
            + &ctah_bottom_temp.to_string()
            );

        self.put_widget_with_size_and_centre(
            ui, 
            ctah_top_label, 
            ctah_slider_x + 55.0, 
            ctah_slider_y - 90.0, 
            ctah_slider_x_width + 120.0, 
            ctah_slider_y_width * 0.2);

        self.put_widget_with_size_and_centre(
            ui, 
            ctah_bottom_label, 
            ctah_slider_x + 55.0, 
            ctah_slider_y + 90.0, 
            ctah_slider_x_width + 120.0, 
            ctah_slider_y_width * 0.2);

        // temperature sensitive buttons for all pipes
        let min_temp_degc = 20.0;
        let max_temp_degc = 100.0;

        // hot branch
        let button_temp_degc = ciet_state_local.pipe_1a_temp_degc;
        let pipe_1a = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "1a");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_1a, 
            heater_x , 
            heater_y - 90.0, 
            50.0, 
            4.0);

        let button_temp_degc = ciet_state_local.pipe_2a_temp_degc;
        let pipe_2a = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "2a");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_2a, 
            heater_x , 
            heater_y - 140.0, 
            50.0, 
            4.0);

        let button_temp_degc = ciet_state_local.pipe_2_temp_degc;
        let pipe_2 = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "2");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_2, 
            heater_x , 
            heater_y - 190.0, 
            50.0, 
            4.0);

        let button_temp_degc = ciet_state_local.pipe_3_temp_degc;
        let pipe_3 = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "3");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_3, 
            heater_x , 
            heater_y - 190.0, 
            50.0, 
            4.0);

        let button_temp_degc = ciet_state_local.pipe_4_temp_degc;
        let pipe_4 = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "4");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_4, 
            heater_x , 
            heater_y - 230.0, 
            50.0, 
            4.0);

        let button_temp_degc = ciet_state_local.pipe_1b_temp_degc;
        let pipe_1b = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "1b");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_1b, 
            heater_x , 
            heater_y + 120.0, 
            150.0, 
            4.0);


        let button_temp_degc = ciet_state_local.pipe_18_temp_degc;
        let pipe_18_vertical = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "18");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_18_vertical, 
            heater_x - 140.0 , 
            heater_y + 230.0, 
            80.0, 
            4.0);

        let pipe_18_horizontal = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "18");

        self.place_horizontal_widget_with_length(
            ui, 
            pipe_18_horizontal, 
            heater_x - 55.0 , 
            heater_y + 210.0, 
            150.0, 
            4.0);

        // dhx branch

        let button_temp_degc = ciet_state_local.pipe_17b_temp_degc;
        let pipe_17b_horizontal = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "17b");

        self.place_horizontal_widget_with_length(
            ui, 
            pipe_17b_horizontal, 
            heater_x - 180.0 , 
            heater_y + 290.0, 
            40.0, 
            4.0);
        
        let button_temp_degc = ciet_state_local.pipe_19_temp_degc;
        let pipe_19_horizontal = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "19");

        self.place_horizontal_widget_with_length(
            ui, 
            pipe_19_horizontal, 
            heater_x - 220.0 , 
            heater_y + 290.0, 
            40.0, 
            4.0);


        let button_temp_degc = ciet_state_local.pipe_20_temp_degc;
        let pipe_20_horizontal = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "20");

        self.place_horizontal_widget_with_length(
            ui, 
            pipe_20_horizontal, 
            heater_x - 260.0 , 
            heater_y + 290.0, 
            40.0, 
            4.0);

        let button_temp_degc = ciet_state_local.pipe_21_temp_degc;
        let pipe_21_horizontal = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "21");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_21_horizontal, 
            heater_x - 290.0 , 
            heater_y + 285.0, 
            40.0, 
            4.0);

        // fm20 (21a)
        let button_temp_degc = ciet_state_local.fm20_label_21a_temp_degc;
        let fm21a_mass_rate_kg_per_s = ciet_state_local.fm20_dhx_branch_kg_per_s;
        let fm21_text: String = "FM20 (21a). flowrate (kg/s) : ".to_string() 
            + &fm21a_mass_rate_kg_per_s.to_string();
        let pipe_21a_vertical = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            &fm21_text);

        self.place_vertical_widget_with_length(
            ui, 
            pipe_21a_vertical, 
            heater_x - 290.0 , 
            heater_y + 225.0, 
            80.0, 
            1.2);

        let button_temp_degc = ciet_state_local.pipe_22_temp_degc;
        let pipe_22_vertical = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "22");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_22_vertical, 
            heater_x - 290.0 , 
            heater_y + 165.0, 
            40.0, 
            4.0);

        let button_temp_degc = ciet_state_local.pipe_23a_temp_degc;
        let pipe_23a_vertical = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "23a");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_23a_vertical, 
            heater_x - 290.0 , 
            heater_y + 125.0, 
            40.0, 
            4.0);


        let button_temp_degc = ciet_state_local.pipe_23_temp_degc;
        let pipe_23_vertical = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "23");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_23_vertical, 
            heater_x - 290.0 , 
            heater_y + 55.0, 
            100.0, 
            10.0);


        // i kinda mix up 25a and 25 here. but this makes it more 
        // correct in the diagram
        let button_temp_degc = ciet_state_local.pipe_25a_temp_degc;
        let pipe_25a_vertical = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "25a");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_25a_vertical, 
            heater_x - 260.0 , 
            heater_y - 255.0, 
            50.0, 
            10.0);

        let button_temp_degc = ciet_state_local.pipe_25_temp_degc;
        let pipe_25_vertical = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "25");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_25_vertical, 
            heater_x - 260.0 , 
            heater_y - 155.0, 
            150.0, 
            10.0);


        let button_temp_degc = ciet_state_local.pipe_26_temp_degc;
        let pipe_26_horizontal = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "26");

        self.place_horizontal_widget_with_length(
            ui, 
            pipe_26_horizontal, 
            heater_x - 190.0 , 
            heater_y - 270.0, 
            120.0, 
            10.0);

        let button_temp_degc = ciet_state_local.pipe_5a_temp_degc;
        let pipe_5a_horizontal = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "5a");

        self.place_horizontal_widget_with_length(
            ui, 
            pipe_5a_horizontal, 
            heater_x - 70.0 , 
            heater_y - 270.0, 
            120.0, 
            10.0);

        // mixing nodes

        let button_temp_degc = ciet_state_local.top_mixing_node_5a_5b_4_temp_degc;
        let top_mixing_node = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "");

        self.place_horizontal_widget_with_length(
            ui, 
            top_mixing_node, 
            heater_x, 
            heater_y - 270.0, 
            40.0, 
            1.0);



        // dracs loop
        let button_temp_degc = ciet_state_local.pipe_30a_temp_degc;
        let pipe_30a_vertical = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "30a");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_30a_vertical, 
            dhx_x - 40.0 , 
            dhx_y + 120.0, 
            120.0, 
            10.0);


        let button_temp_degc = ciet_state_local.pipe_39_temp_degc;
        let pipe_39_horizontal = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "39");

        self.place_horizontal_widget_with_length(
            ui, 
            pipe_39_horizontal, 
            dhx_x - 70.0 , 
            dhx_y + 180.0, 
            90.0, 
            10.0);

        let pipe_39_vertical = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "39");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_39_vertical, 
            dhx_x - 130.0 , 
            dhx_y + 135.0, 
            120.0, 
            10.0);


        let button_temp_degc = ciet_state_local.pipe_38_temp_degc;
        let pipe_38_horizontal = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "38");

        self.place_horizontal_widget_with_length(
            ui, 
            pipe_38_horizontal, 
            dhx_x - 170.0 , 
            dhx_y + 90.0, 
            50.0, 
            10.0);

        // dracs flowmeter fm60
        //

        let button_temp_degc = ciet_state_local.fm60_label_37a_temp_degc;
        let fm37a_mass_rate_kg_per_s = ciet_state_local.fm_60_dracs_kg_per_s;
        let fm21_text: String = "FM60 (37a). flowrate (kg/s) : ".to_string() 
            + &fm37a_mass_rate_kg_per_s.to_string();
        let pipe_37a_vertical = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            &fm21_text);

        self.place_vertical_widget_with_length(
            ui, 
            pipe_37a_vertical, 
            dhx_x - 225.0 , 
            dhx_y + 65.0, 
            80.0, 
            1.2);

        let button_temp_degc = ciet_state_local.pipe_37_temp_degc;
        let pipe_37_horizontal = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "37");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_37_horizontal, 
            dhx_x - 270.0 , 
            dhx_y - 5.0, 
            100.0, 
            10.0);

        let button_temp_degc = ciet_state_local.pipe_36a_temp_degc;
        let pipe_36a_horizontal = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "36a");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_36a_horizontal, 
            dhx_x - 300.0 , 
            dhx_y - 65.0, 
            80.0, 
            10.0);


        let button_temp_degc = ciet_state_local.pipe_36_temp_degc;
        let pipe_36_horizontal = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "36");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_36_horizontal, 
            dhx_x - 300.0 , 
            dhx_y - 135.0, 
            80.0, 
            10.0);

        // dracs hot leg
        let button_temp_degc = ciet_state_local.pipe_30b_temp_degc;
        let pipe_30b_horizontal = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "30b");

        self.place_horizontal_widget_with_length(
            ui, 
            pipe_30b_horizontal, 
            dhx_x - 100.0 , 
            dhx_y - 50.0, 
            80.0, 
            10.0);

        let button_temp_degc = ciet_state_local.pipe_31_temp_degc;
        let pipe_31_vertical = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "31");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_31_vertical, 
            dhx_x - 125.0 , 
            dhx_y - 80.0, 
            30.0, 
            10.0);

        let button_temp_degc = ciet_state_local.pipe_31a_temp_degc;
        let pipe_31a_vertical = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "31a");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_31a_vertical, 
            dhx_x - 155.0 , 
            dhx_y - 80.0, 
            30.0, 
            10.0);



        let button_temp_degc = ciet_state_local.pipe_32_temp_degc;
        let pipe_32_vertical = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "32");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_32_vertical, 
            dhx_x - 185.0 , 
            dhx_y - 80.0, 
            30.0, 
            10.0);

        let button_temp_degc = ciet_state_local.pipe_33_temp_degc;
        let pipe_33_vertical = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "33");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_33_vertical, 
            dhx_x - 185.0 , 
            dhx_y - 110.0, 
            30.0, 
            10.0);

        let button_temp_degc = ciet_state_local.pipe_34_temp_degc;
        let pipe_34_vertical = new_temp_sensitive_button(
            min_temp_degc, 
            max_temp_degc, 
            button_temp_degc, 
            "34");

        self.place_vertical_widget_with_length(
            ui, 
            pipe_34_vertical, 
            dhx_x - 185.0 , 
            dhx_y - 150.0, 
            50.0, 
            10.0);


        // ctah pump 

        // for user to set ctah pump 
        let ctah_pump_slider_pa = egui::Slider::new(
            &mut ciet_state_local.ctah_pump_pressure_pascals, -15000.0..=15000.0)
            .vertical()
            .text("CTAH pump pressure, loop pressure drop (Pa)");

        let ctah_pump_slider_x = ctah_pump_x + 0.7 * ctah_pump_x_width;
        let ctah_pump_slider_y = ctah_pump_y + 10.0;
        let ctah_pump_slider_x_width = 30.0;
        let ctah_pump_slider_y_width = ctah_pump_y_width;

        self.put_widget_with_size_and_centre(
            ui, 
            ctah_pump_slider_pa, 
            ctah_pump_slider_x, 
            ctah_pump_slider_y, 
            ctah_pump_slider_x_width, 
            ctah_pump_slider_y_width);

        // obtain a lock for ciet state, set it 
        self.ciet_state.lock().unwrap().overwrite_state(ciet_state_local);
    }

    /// inserts static image widgets for ciet
    fn insert_pictures(&mut self, ui: &mut Ui,){

        let tchx_pic = Image::new(
            include_image!("../../cooler.png")
            ).rounding(5.0);

        let dhx_pic = Image::new(
            include_image!("../../heat-exchanger.png")
            ).rounding(5.0);

        let heater_pic = Image::new(
            include_image!("../../heater.png")
            ).rounding(5.0);

        let ctah_pump_pic = Image::new(
            include_image!("../../pump.png")
            ).rounding(5.0);

        let ctah_pic = Image::new(
            include_image!("../../cooler.png")
            ).rounding(5.0);

        let (tchx_x, tchx_y): (f32, f32) = (150.0, 260.0);
        let (tchx_x_width, tchx_y_width): (f32, f32) = (150.0, 150.0);

        // for tchx
        self.put_widget_with_size_and_centre(
            ui, 
            tchx_pic, 
            tchx_x, 
            tchx_y, 
            tchx_x_width, 
            tchx_y_width);

        // for dhx
        let dhx_x = tchx_x + 250.0;
        let dhx_y = tchx_y + 250.0;
        let dhx_x_width = tchx_x_width;
        let dhx_y_width = tchx_y_width;

        self.put_widget_with_size_and_centre(
            ui, 
            dhx_pic, 
            dhx_x, 
            dhx_y, 
            dhx_x_width, 
            dhx_y_width);

        // for heater
        let heater_x = dhx_x + 350.0;
        let heater_y = dhx_y + 50.0;
        let heater_x_width = dhx_x_width;
        let heater_y_width = dhx_y_width;

        self.put_widget_with_size_and_centre(
            ui, 
            heater_pic, 
            heater_x, 
            heater_y, 
            heater_x_width, 
            heater_y_width);

        // for ctah
        let ctah_x = heater_x + 750.0;
        let ctah_y = tchx_y;
        let ctah_x_width = dhx_x_width;
        let ctah_y_width = dhx_y_width;

        self.put_widget_with_size_and_centre(
            ui, 
            ctah_pic, 
            ctah_x, 
            ctah_y, 
            ctah_x_width, 
            ctah_y_width);

        // for ctah_pump
        let ctah_pump_x = ctah_x - 50.0;
        let ctah_pump_y = heater_y + 270.0;
        let ctah_pump_x_width = dhx_x_width;
        let ctah_pump_y_width = dhx_y_width;

        self.put_widget_with_size_and_centre(
            ui, 
            ctah_pump_pic, 
            ctah_pump_x, 
            ctah_pump_y, 
            ctah_pump_x_width, 
            ctah_pump_y_width);

        ui.separator();
    }
}
