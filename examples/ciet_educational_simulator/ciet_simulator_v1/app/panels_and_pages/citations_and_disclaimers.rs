use egui::Ui;


use crate::ciet_simulator_v1::CIETApp;

impl CIETApp {
    pub fn citation_disclaimer_and_acknowledgements(&mut self, ui: &mut Ui){

        ui.heading("DISCLAIMER");
        
        ui.label(" ");

        ui.label("This is an educational simulator");
        ui.label("Validation is still work in progress");
        ui.label("This is given under GPLv3 without ANY WARRANY");
        ui.label("Results are not guaranteed to be physically accurate");
        ui.label("USE AT YOUR OWN RISK");

        ui.separator();
        ui.separator();

        ui.heading("Citations appreciated:");
        ui.label(" ");
        
        ui.label("@phdthesis{ong2024digital,");
        ui.label("title={Digital Twins as Testbeds for Iterative Simulated Neutronics Feedback Controller Development},");
        ui.label("author={Ong, Theodore Kay Chen},");
        ui.label("year={2024},");
        ui.label("school={UC Berkeley}");
        ui.label("}");

        ui.label(" ");

        ui.label("@article{ong4998548open,");
        ui.label("title={An Open Source Thermo-Hydraulic Uniphase Solver for Advection and Convection in Salt Flows (Tuas)},");
        ui.label("author={Ong, Theodore Kay Chen and Xiao, Sicong and Peterson, Per F},");
        ui.label("journal={Available at SSRN 4998548}}");
        ui.label(" ");
    }
}
