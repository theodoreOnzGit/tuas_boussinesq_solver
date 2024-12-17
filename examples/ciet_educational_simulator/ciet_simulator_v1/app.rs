use std::{sync::{Arc,Mutex}, thread, time::Duration};

use panels_and_pages::{ciet_data::{CIETState, PagePlotData}, full_simulation::educational_ciet_loop_version_3, Panel};
use useful_functions::update_ciet_plot_from_ciet_state;



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

        }
    }
}

impl CIETApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
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
            educational_ciet_loop_version_3(ciet_state_ptr);
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
                    ui.selectable_value(&mut self.open_panel, Panel::SchematicDiagram, "CIET Schematic Diagram"); 
                    ui.selectable_value(&mut self.open_panel, Panel::NodalisedDiagram, "CIET NodalisedDiagram Diagram"); 
            }
            );
            ui.separator();
        });

        egui::SidePanel::right("Supplementary Info").show(ctx, |ui|{
            match self.open_panel {
                Panel::MainPage => {
                    self.ciet_main_page_side_panel(ui);
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
                Panel::SchematicDiagram => {},
                Panel::NodalisedDiagram => {},
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's





            // show correct panel or page based on user selection

            match self.open_panel {
                Panel::SchematicDiagram => {
                    // enables scrolling within the image
                    egui::ScrollArea::both().show(ui, |ui| {
                        ui.image(egui::include_image!("ciet_gui_schematics.png"));
                    });
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

        // request update every 0.1 s 

        ctx.request_repaint_after(Duration::from_millis(50));
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
