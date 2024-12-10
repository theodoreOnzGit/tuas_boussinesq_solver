#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
pub fn ciet_simulator_v1() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "CIET Simulator V1",
        native_options,
        Box::new(|cc| {
            // image support,
            // from 
            // https://github.com/emilk/egui/tree/master/examples/images
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(CIETApp::new(cc)))

    }

        ),
    )
}

mod app;
pub use app::CIETApp;
