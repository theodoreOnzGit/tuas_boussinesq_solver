
use egui::Ui;

use crate::ciet_simulator_v1::CIETApp;

impl CIETApp {

    pub fn ciet_sim_dhx_branch_page(&mut self, ui: &mut Ui){
        ui.separator();

        ui.horizontal(|ui| {
            let local_ciet_state = 
                self.ciet_state.lock().unwrap().clone();
            let current_dhx_br_blocked_state: bool = 
                local_ciet_state.is_dhx_branch_blocked;
            if ui.button("Toggle DHX Branch Flow Blocking Mechanism").clicked() {


                let user_toggled_dhx_br_blocked_state: bool;

                if current_dhx_br_blocked_state == true {
                    user_toggled_dhx_br_blocked_state = false;
                } else {
                    user_toggled_dhx_br_blocked_state = true;
                };

                self.ciet_state.lock().unwrap().is_dhx_branch_blocked 
                    = user_toggled_dhx_br_blocked_state;


            }
            ui.label("DHX Branch Blocked? : ");
            ui.label(current_dhx_br_blocked_state.to_string());
        });

    }
}
