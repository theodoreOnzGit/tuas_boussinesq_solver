use egui::Ui;


use crate::ciet_simulator_v1::CIETApp;

impl CIETApp {
    pub fn citation_disclaimer_and_acknowledgements(&mut self, ui: &mut Ui){

        ui.heading("DISCLAIMER");
        
        ui.label(" ");

        ui.label("This is an educational simulator under testing and development");
        ui.label("Limited Validation has been done on transient forced circulation");
        ui.label("and steady statenatural circulation");
        ui.label("Validation is still work in progress");
        ui.label("This is given under GPLv3 without ANY WARRANY");
        ui.label("Results are not guaranteed to be physically accurate");
        ui.label("USE AT YOUR OWN RISK");

        ui.label(" ");
        ui.label(" ");

        ui.heading("COPYRIGHT");
        
        ui.label(" ");

        ui.label("Theodore Kay Chen Ong, SiCong Xiao, SNRSI, and Per F. Peterson");

        ui.label(" ");
        ui.label(" ");
        ui.heading("CREDITS");
        
        ui.label(" ");

        ui.label("Heater, cooler and heat exchanger artwork from DWSIM released under GPLv3");

        ui.label(" ");
        ui.label(" ");

        ui.heading("Citations appreciated:");
        ui.label(" ");
        
        ui.label("@phdthesis{ong2024digital,");
        ui.label("title={Digital Twins as Testbeds for Iterative Simulated Neutronics Feedback Controller Development},");
        ui.label("author={Ong, Theodore Kay Chen},");
        ui.label("year={2024},");
        ui.label("school={UC Berkeley}");
        ui.label("}");

        ui.label(" ");

        ui.label("@article{ong2024open,");
        ui.label("title={An open-source Thermo-hydraulic Uniphase Advection and Convection Solver for Salt Flows (TUAS)},");
        ui.label("author={Ong, Theodore Kay Chen and Xiao, Sicong and Peterson, Per F},");
        ui.label("journal={International Journal of Advanced Nuclear Reactor Design and Technology},");
        ui.label("volume={6},");
        ui.label("number={4},");
        ui.label("pages={281--301},");
        ui.label("year={2024},");
        ui.label("publisher={Elsevier}");
        ui.label("}");
    }
}
