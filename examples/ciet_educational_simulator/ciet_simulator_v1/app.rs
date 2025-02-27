use std::{sync::{Arc,Mutex}, thread, time::Duration};

use panels_and_pages::{ciet_data::{CIETState, PagePlotData}, frequency_response_and_transients::FreqResponseAndTransientSettings, full_simulation::educational_ciet_loop_version_4, online_calibration::HeaterType, Panel};
use uom::si::{power::kilowatt, time::second};
use useful_functions::update_ciet_plot_from_ciet_state;
use uom::si::f64::*;



/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct CIETApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f64,

    open_panel: Panel,
    #[serde(skip)]
    ciet_state: Arc<Mutex<CIETState>>,

    // we also need plot data here 
    // this is for the data to be transferred between threads
    #[serde(skip)]
    ciet_plot_data_mutex_ptr_for_parallel_data_transfer: Arc<Mutex<PagePlotData>>,

    // this is for direct use in plots 
    #[serde(skip)]
    ciet_plot_data: PagePlotData,

    #[serde(skip)]
    frequency_response_settings: FreqResponseAndTransientSettings,

    // checks whether user wants fast fwd or slow motion
    user_wants_fast_fwd_on: bool,
    // checks whether user wants fast fwd or slow motion
    user_wants_slow_motion_on: bool,

    // for the user to select the heater type desierd
    #[serde(skip)]
    user_desired_heater_type: HeaterType,

}

pub mod panels_and_pages;

pub mod useful_functions;

impl Default for CIETApp {
    fn default() -> Self {

        let ciet_state = Arc::new(Mutex::new(CIETState::default()));
        let ciet_plot_data = Arc::new(Mutex::new(PagePlotData::default()));

        Self {
            // Example stuff:
            label: "CIET simulator v1".to_owned(),
            value: 3.6,
            open_panel: Panel::MainPage,
            ciet_state,
            ciet_plot_data_mutex_ptr_for_parallel_data_transfer: ciet_plot_data,
            ciet_plot_data: PagePlotData::default(),
            frequency_response_settings: FreqResponseAndTransientSettings::default(),
            user_wants_fast_fwd_on: false,
            user_wants_slow_motion_on: false,
            user_desired_heater_type: HeaterType::InsulatedHeaterV1Fine15Mesh,

        }
    }
}

impl CIETApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        //// Load previous app state (if any).
        //// Note that you must enable the `persistence` feature for this to work.
        //if let Some(storage) = cc.storage {
        //    return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //}

        let new_ciet_app: CIETApp = Default::default();

        // I'll clone the pointer and start a thread 

        let ciet_state_ptr: Arc<Mutex<CIETState>> = 
            new_ciet_app.ciet_state.clone();

        // this is the current state of ciet for plotting
        // like the instantaneous temperature and such
        let ciet_state_ptr_for_plotting: Arc<Mutex<CIETState>> = 
            new_ciet_app.ciet_state.clone();
        // for data recording, 
        // I'll also clone the pointer and start a thread
        // this contains arrays with historical data
        let ciet_plot_ptr: Arc<Mutex<PagePlotData>> = 
            new_ciet_app.ciet_plot_data_mutex_ptr_for_parallel_data_transfer.clone();

        // now spawn a thread moving in the pointer 
        //
        thread::spawn(move ||{
            educational_ciet_loop_version_4(ciet_state_ptr);
        });

        // spawn a thread to update the plotting bits
        thread::spawn(move ||{
            update_ciet_plot_from_ciet_state(
                ciet_state_ptr_for_plotting, 
                ciet_plot_ptr);
        });

        new_ciet_app
    }

    
}

impl eframe::App for CIETApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui


        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {

                egui::widgets::global_theme_preference_buttons(ui);
            });


            ui.heading("CIET Educational Simulator v1");
            ui.separator();
            // allow user to select which panel is open
            ui.horizontal( 
                |ui| {
                    ui.selectable_value(&mut self.open_panel, Panel::MainPage, "Main Page"); 
                    ui.selectable_value(&mut self.open_panel, Panel::Heater, "Heater"); 
                    ui.selectable_value(&mut self.open_panel, Panel::CTAH, "CTAH"); 
                    ui.selectable_value(&mut self.open_panel, Panel::CTAHPump, "CTAH Pump"); 
                    ui.selectable_value(&mut self.open_panel, Panel::TCHX, "TCHX"); 
                    ui.selectable_value(&mut self.open_panel, Panel::DHX, "DHX STHE"); 
                    ui.selectable_value(&mut self.open_panel, Panel::FrequencyResponseAndTransients, "Frequency Response and Transients"); 
                    ui.selectable_value(&mut self.open_panel, Panel::OnlineCalibration, "Online Calibration"); 
                    ui.selectable_value(&mut self.open_panel, Panel::NodalisedDiagram, "CIET Nodalised Diagram"); 
            }
            );
            ui.separator();
        });

        egui::SidePanel::right("Supplementary Info").show(ctx, |ui|{
            match self.open_panel {
                Panel::MainPage => {
                    egui::ScrollArea::both().show(ui, |ui| {
                        self.ciet_main_page_side_panel(ui);
                        self.citation_disclaimer_and_acknowledgements(ui);
                    });
                },
                Panel::CTAHPump => {
                    self.ciet_sim_ctah_pump_page_csv(ui);
                },
                Panel::CTAH => {
                    self.ciet_sim_ctah_page_csv(ui);
                },
                Panel::Heater => {
                    
                    // display csv file on side panel when heater page 
                    // is open
                    self.ciet_sim_heater_page_csv(ui);
                },
                Panel::DHX => {
                    self.ciet_sim_dhx_page_csv(ui);
                },
                Panel::TCHX => {
                    self.ciet_sim_tchx_page_csv(ui);
                },
                Panel::FrequencyResponseAndTransients => {

                    self.ciet_sim_heater_page_csv(ui);
                },
                Panel::NodalisedDiagram => {},
                Panel::OnlineCalibration => {},

            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's





            // show correct panel or page based on user selection

            match self.open_panel {
                Panel::FrequencyResponseAndTransients => {
                    // enables scrolling within the image
                    //egui::ScrollArea::both().show(ui, |ui| {
                    //    ui.image(egui::include_image!("ciet_gui_schematics.png"));
                    //});
                    self.ciet_sim_transients_and_freq_response_page(ui);

                    
                },
                Panel::MainPage => {
                    self.ciet_sim_main_page_central_panel(ui);

                },
                Panel::CTAHPump => {
                    self.ciet_sim_ctah_pump_page_and_graphs(ui);
                },
                Panel::CTAH => {
                    self.ciet_sim_ctah_page_graph(ui);
                },
                Panel::Heater => {
                    self.ciet_sim_heater_page_graph(ui);
                },
                Panel::DHX => {
                    self.ciet_sim_dhx_branch_page_graph(ui);
                },
                Panel::TCHX => {
                    self.ciet_sim_tchx_page_graph(ui);
                },
                Panel::NodalisedDiagram => {
                    // enables scrolling within the image
                    egui::ScrollArea::both().show(ui, |ui| {
                        ui.image(egui::include_image!("ciet_sam_diagram_replica.jpg"));
                    });
                },
                Panel::OnlineCalibration => {
                    self.ciet_sim_online_calibration_page(ui);

                },

            }

            ui.add(egui::github_link_file!(
                    "https://github.com/theodoreOnzGit/tuas_boussinesq_solver/blob/develop/",
                    "TUAS Github Repo (develop)"
            ));




        });

        egui::TopBottomPanel::bottom("github").show(ctx, |ui|{

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });

        });

        

        // frequency response should only switch on IF 
        // both advanced heater control and frequency response are 
        // switched on
        if self.frequency_response_settings.advanced_heater_control_switched_on
            && self.frequency_response_settings.frequency_response_switched_on {

                // frequency response controls
                // first get current state
                let mut ciet_state_local: CIETState 
                    = self.ciet_state.lock().unwrap().clone();
                let current_sim_time = 
                    Time::new::<second>(
                        ciet_state_local.simulation_time_seconds
                    );

                let total_heater_power_kw = 
                    self.frequency_response_settings
                    .get_frequency_response_signal(current_sim_time)
                    .get::<kilowatt>();
                ciet_state_local.heater_power_kilowatts = 
                    total_heater_power_kw;
                // update frequency response back into state 
                self.ciet_state.lock().unwrap().overwrite_state(ciet_state_local);
        } else if self.frequency_response_settings.advanced_heater_control_switched_on
            && !self.frequency_response_settings.frequency_response_switched_on {
                // if advanced heater control is switched on and 
                // frequency response off, only take steady 
                // state power

                // frequency response controls
                // first get current state
                let mut ciet_state_local: CIETState 
                    = self.ciet_state.lock().unwrap().clone();

                let total_heater_power_kw = 
                    self.frequency_response_settings
                    .get_steady_state_power_signal()
                    .get::<kilowatt>();

                ciet_state_local.heater_power_kilowatts = 
                    total_heater_power_kw;
                // update frequency response back into state 
                self.ciet_state.lock().unwrap().overwrite_state(ciet_state_local);
        }

        // now for calibration functions, eg. heater type 

        {
            let user_desired_heater_type: HeaterType = self.user_desired_heater_type;

            let mut ciet_state_local: CIETState 
                = self.ciet_state.lock().unwrap().clone();
            ciet_state_local.current_heater_type = user_desired_heater_type;
            self.ciet_state.lock().unwrap().overwrite_state(ciet_state_local);
        }

        // request update every 0.1 s 

        ctx.request_repaint_after(Duration::from_millis(50));

        // adding the return here because there are too many closing 
        // parantheses
        // just demarcates the end
        return ();
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
