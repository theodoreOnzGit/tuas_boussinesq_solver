use std::{sync::{Arc,Mutex}, thread, time::Duration};

use panels_and_pages::{ciet_data::CIETState, full_simulation::educational_ciet_loop_version_3, Panel};



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
}

pub mod panels_and_pages;

pub mod useful_functions;

impl Default for CIETApp {
    fn default() -> Self {

        let ciet_state = Arc::new(Mutex::new(CIETState::default()));

        Self {
            // Example stuff:
            label: "CIET simulator v1".to_owned(),
            value: 3.6,
            open_panel: Panel::MainPage,
            ciet_state,
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

        // now spawn a thread moving in the pointer 
        //
        thread::spawn(move ||{
            educational_ciet_loop_version_3(ciet_state_ptr);
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
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("ciet simualtor v1");


            ui.separator();
            // allow user to select which panel is open
            ui.horizontal( 
                |ui| {
                    ui.selectable_value(&mut self.open_panel, Panel::MainPage, "Main Page"); 
                    ui.selectable_value(&mut self.open_panel, Panel::Heater, "Heater"); 
                    ui.selectable_value(&mut self.open_panel, Panel::CTAH, "CTAH"); 
                    ui.selectable_value(&mut self.open_panel, Panel::CTAHPump, "CTAH Pump"); 
                    ui.selectable_value(&mut self.open_panel, Panel::TCHX, "TCHX"); 
                    ui.selectable_value(&mut self.open_panel, Panel::DHX, "DHX"); 
                    ui.selectable_value(&mut self.open_panel, Panel::SchematicDiagram, "CIET Schematic Diagram"); 
                    ui.selectable_value(&mut self.open_panel, Panel::NodalisedDiagram, "CIET NodalisedDiagram Diagram"); 
            }
            );
            ui.separator();



            // show correct panel or page based on user selection

            match self.open_panel {
                Panel::SchematicDiagram => {
                    // enables scrolling within the image
                    egui::ScrollArea::both().show(ui, |ui| {
                        ui.image(egui::include_image!("ciet_gui_schematics.png"));
                    });
                },
                Panel::MainPage => {
                    self.ciet_sim_main_page(ui);

                },
                Panel::CTAHPump => {
                    self.ciet_sim_ctah_pump_page(ui);
                },
                Panel::CTAH => {
                    self.ciet_sim_ctah_page(ui);
                },
                Panel::Heater => {
                    self.ciet_sim_heater_page(ui);
                },
                Panel::DHX => {},
                Panel::TCHX => {},
                Panel::NodalisedDiagram => {
                    // enables scrolling within the image
                    egui::ScrollArea::both().show(ui, |ui| {
                        ui.image(egui::include_image!("ciet_sam_diagram_replica.jpg"));
                    });
                },
            }

            ui.separator();


            ui.add(egui::github_link_file!(
                "https://github.com/theodoreOnzGit/outram-park-backend/blob/develop/",
                "outram-park-backend Github Repo (develop)"
            ));

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
