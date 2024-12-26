use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use tuas_boussinesq_solver::pre_built_components::ciet_three_branch_plus_dracs::solver_functions::get_mass_flowrate_two_branches;
use tuas_boussinesq_solver::pre_built_components::ciet_three_branch_plus_dracs::solver_functions::get_mass_flowrate_vector_for_dhx_heater_and_ctah_branches;
use tuas_boussinesq_solver::prelude::beta_testing::InsulatedPorousMediaFluidComponent;
// first, copy and paste the dracs loop functions over 
//
use uom::si::f64::*;

use tuas_boussinesq_solver::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component::FluidComponent;
use tuas_boussinesq_solver::
array_control_vol_and_fluid_component_collections::
fluid_component_collection::
fluid_component_collection::FluidComponentCollection;
// let's construct the branches with test pressures and obtain 
use tuas_boussinesq_solver::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
use uom::ConstZero;

use tuas_boussinesq_solver::
array_control_vol_and_fluid_component_collections::
fluid_component_collection::
fluid_component_super_collection::FluidComponentSuperCollection;

use tuas_boussinesq_solver::pre_built_components::
insulated_pipes_and_fluid_components::InsulatedFluidComponent;
use tuas_boussinesq_solver::pre_built_components::
non_insulated_fluid_components::NonInsulatedFluidComponent;


pub fn three_branch_pri_loop_flowrates_parallel_ver_4(
    pump_pressure: Pressure,
    ctah_branch_blocked: bool,
    dhx_branch_blocked: bool,
    pipe_4: &InsulatedFluidComponent,
    pipe_3: &InsulatedFluidComponent,
    pipe_2a: &InsulatedFluidComponent,
    static_mixer_10_label_2: &InsulatedFluidComponent,
    heater_top_head_1a: &InsulatedPorousMediaFluidComponent,
    heater_ver_1: &InsulatedPorousMediaFluidComponent,
    heater_bottom_head_1b: &InsulatedPorousMediaFluidComponent,
    pipe_18: &InsulatedFluidComponent,
    pipe_5a: &InsulatedFluidComponent,
    pipe_26: &InsulatedFluidComponent,
    pipe_25a: &InsulatedFluidComponent,
    static_mixer_21_label_25: &InsulatedFluidComponent,
    dhx_shell_side_pipe_24: &FluidComponent,
    static_mixer_20_label_23: &InsulatedFluidComponent,
    pipe_23a: &InsulatedFluidComponent,
    pipe_22: &InsulatedFluidComponent,
    flowmeter_20_21a: &NonInsulatedFluidComponent,
    pipe_21: &InsulatedFluidComponent,
    pipe_20: &InsulatedFluidComponent,
    pipe_19: &InsulatedFluidComponent,
    pipe_17b: &InsulatedFluidComponent,
    // ctah branch
    pipe_5b: &InsulatedFluidComponent,
    static_mixer_41_label_6 :&InsulatedFluidComponent,
    pipe_6a :&InsulatedFluidComponent,
    ctah_vertical_label_7a :&NonInsulatedFluidComponent,
    ctah_horizontal_label_7b :&NonInsulatedFluidComponent,
    pipe_8a :&InsulatedFluidComponent,
    static_mixer_40_label_8 :&InsulatedFluidComponent,
    pipe_9 :&InsulatedFluidComponent,
    pipe_10 :&InsulatedFluidComponent,
    pipe_11 :&InsulatedFluidComponent,
    pipe_12 :&InsulatedFluidComponent,
    ctah_pump :&NonInsulatedFluidComponent,
    pipe_13 : &InsulatedFluidComponent,
    pipe_14 : &InsulatedFluidComponent,
    flowmeter_40_14a :&NonInsulatedFluidComponent,
    pipe_15 :&InsulatedFluidComponent,
    pipe_16 :&InsulatedFluidComponent,
    pipe_17a :&InsulatedFluidComponent,
    ) ->
(MassRate, MassRate, MassRate) {


    let mut heater_branch = 
        FluidComponentCollection::new_series_component_collection();

    heater_branch.clone_and_add_component(pipe_4);
    heater_branch.clone_and_add_component(pipe_3);
    heater_branch.clone_and_add_component(pipe_2a);
    heater_branch.clone_and_add_component(static_mixer_10_label_2);
    heater_branch.clone_and_add_component(heater_top_head_1a);
    heater_branch.clone_and_add_component(heater_ver_1);
    heater_branch.clone_and_add_component(heater_bottom_head_1b);
    heater_branch.clone_and_add_component(pipe_18);


    let mut dhx_branch = 
        FluidComponentCollection::new_series_component_collection();

    dhx_branch.clone_and_add_component(pipe_5a);
    dhx_branch.clone_and_add_component(pipe_26);
    dhx_branch.clone_and_add_component(pipe_25a);
    dhx_branch.clone_and_add_component(static_mixer_21_label_25);
    dhx_branch.clone_and_add_component(dhx_shell_side_pipe_24);
    dhx_branch.clone_and_add_component(static_mixer_20_label_23);
    dhx_branch.clone_and_add_component(pipe_23a);
    dhx_branch.clone_and_add_component(pipe_22);
    dhx_branch.clone_and_add_component(flowmeter_20_21a);
    dhx_branch.clone_and_add_component(pipe_21);
    dhx_branch.clone_and_add_component(pipe_20);
    dhx_branch.clone_and_add_component(pipe_19);
    dhx_branch.clone_and_add_component(pipe_17b);

    let mut ctah_branch = FluidComponentCollection::new_series_component_collection();
    
    ctah_branch.clone_and_add_component(pipe_5b);
    ctah_branch.clone_and_add_component(static_mixer_41_label_6);
    ctah_branch.clone_and_add_component(pipe_6a);
    ctah_branch.clone_and_add_component(ctah_vertical_label_7a);
    ctah_branch.clone_and_add_component(ctah_horizontal_label_7b);
    ctah_branch.clone_and_add_component(pipe_8a);
    ctah_branch.clone_and_add_component(static_mixer_40_label_8);
    ctah_branch.clone_and_add_component(pipe_9);
    ctah_branch.clone_and_add_component(pipe_10);
    ctah_branch.clone_and_add_component(pipe_11);
    ctah_branch.clone_and_add_component(pipe_12);
    let mut ctah_pump_clone: NonInsulatedFluidComponent 
        = ctah_pump.clone();
    ctah_pump_clone.set_internal_pressure_source(pump_pressure);
    ctah_branch.clone_and_add_component(&ctah_pump_clone);
    ctah_branch.clone_and_add_component(pipe_13);
    ctah_branch.clone_and_add_component(pipe_14);
    ctah_branch.clone_and_add_component(flowmeter_40_14a);
    ctah_branch.clone_and_add_component(pipe_15);
    ctah_branch.clone_and_add_component(pipe_16);
    ctah_branch.clone_and_add_component(pipe_17a);

    let mut pri_loop_branches = 
        FluidComponentSuperCollection::default();

    pri_loop_branches.set_orientation_to_parallel();

    if ctah_branch_blocked && !dhx_branch_blocked {

        pri_loop_branches.fluid_component_super_vector.push(dhx_branch);
        pri_loop_branches.fluid_component_super_vector.push(heater_branch);

        let (dhx_flow, heater_flow) = 
            get_mass_flowrate_two_branches(
                &pri_loop_branches);

        return (dhx_flow, heater_flow, MassRate::ZERO);

    } else if dhx_branch_blocked && !ctah_branch_blocked {

        pri_loop_branches.fluid_component_super_vector.push(heater_branch);
        pri_loop_branches.fluid_component_super_vector.push(ctah_branch);

        let (heater_flow, ctah_flow) = 
            get_mass_flowrate_two_branches(
                &pri_loop_branches);
        return (MassRate::ZERO, heater_flow, ctah_flow);

    } else if ctah_branch_blocked && dhx_branch_blocked {
        // all flows blocked, no nothing to see here

        return (MassRate::ZERO, MassRate::ZERO, MassRate::ZERO);
    } else {
        
        // two loops open scenario 

        let mut pri_loop_ctah_and_heater_br =
            FluidComponentSuperCollection::default();

        pri_loop_ctah_and_heater_br.fluid_component_super_vector.push(heater_branch.clone());
        pri_loop_ctah_and_heater_br.fluid_component_super_vector.push(ctah_branch.clone());

        // create pointers for two branch flow 

        let heater_br_flow_two_br_scenario = Arc::new(Mutex::new(
                MassRate::ZERO));
        let ctah_br_flow_two_br_scenario = Arc::new(Mutex::new(
                MassRate::ZERO));

        let heater_br_flow_two_br_scenario_clone = heater_br_flow_two_br_scenario.clone();
        let ctah_br_flow_two_br_scenario_clone = ctah_br_flow_two_br_scenario.clone();

        // spawn a thread 

        let two_branch_flow_scenario_join_handle = 
            thread::spawn(move ||{
                let (heater_flow_two_br, ctah_flow_two_br) = 
                    get_mass_flowrate_two_branches(
                        &pri_loop_ctah_and_heater_br);

                // load this into the arc mutex ptr 

                *heater_br_flow_two_br_scenario_clone.lock().unwrap().deref_mut()
                    = heater_flow_two_br;
                *ctah_br_flow_two_br_scenario_clone.lock().unwrap().deref_mut()
                    = ctah_flow_two_br;
            });

        // all loops opened scenario
        pri_loop_branches.fluid_component_super_vector.push(dhx_branch);
        pri_loop_branches.fluid_component_super_vector.push(heater_branch);
        pri_loop_branches.fluid_component_super_vector.push(ctah_branch);

        let (dhx_flow, heater_flow, ctah_flow) = 
            get_mass_flowrate_vector_for_dhx_heater_and_ctah_branches(
                &pri_loop_branches);



        // if dhx flow is downwards, (positive flow, is ok)
        // if negative flow, then block it 

        let flow_diode_block_flow: bool = dhx_flow < MassRate::ZERO;

        // load the two branch solution
        two_branch_flow_scenario_join_handle.join().unwrap();

        if flow_diode_block_flow {


            let dhx_flow = MassRate::ZERO;
            let heater_flow: MassRate = *heater_br_flow_two_br_scenario.lock().unwrap().deref();
            let ctah_flow: MassRate = *ctah_br_flow_two_br_scenario.lock().unwrap().deref();
            return (dhx_flow, heater_flow, ctah_flow);

        } else {

            return (dhx_flow, heater_flow, ctah_flow);
        }


    }

}
