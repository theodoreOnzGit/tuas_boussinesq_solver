
use crate::array_control_vol_and_fluid_component_collections::
fluid_component_collection::fluid_component_traits::FluidComponentTrait;
use crate::array_control_vol_and_fluid_component_collections::
fluid_component_collection::fluid_component_collection::FluidComponentCollection;

use super::*;

/// builds a dhx branch to simulate isothermal testing of ciet
pub fn dhx_branch_builder_isothermal_test(
    initial_temperature: ThermodynamicTemperature) -> FluidComponentCollection {


    // from top to bottom, (no check valve)

    let pipe_26 = new_pipe_26(initial_temperature);
    let static_mixer_21_label_25 = new_static_mixer_21_label_25(initial_temperature);
    let static_mixer_pipe_25a = new_pipe_25a(initial_temperature);
    let dhx_shell_side_24 = new_inactive_dhx_shell_side_heat_exchanger(initial_temperature);
    let static_mixer_20_label_23 = new_static_mixer_20_label_23(initial_temperature);
    let static_mixer_pipe_23a = new_pipe_23a(initial_temperature);
    let pipe_22 = new_pipe_22_relap_model(initial_temperature);
    let flowmeter_20_21a = new_flowmeter_20_label_21a(initial_temperature);
    let pipe_21 = new_pipe_21(initial_temperature);
    let pipe_20 = new_pipe_20(initial_temperature);
    let pipe_19 = new_pipe_19(initial_temperature);

    let mut dhx_branch = FluidComponentCollection::new_series_component_collection();


    dhx_branch.clone_and_add_component(&pipe_26);
    dhx_branch.clone_and_add_component(&static_mixer_21_label_25);
    dhx_branch.clone_and_add_component(&static_mixer_pipe_25a);
    dhx_branch.clone_and_add_component(&dhx_shell_side_24);
    dhx_branch.clone_and_add_component(&static_mixer_20_label_23);
    dhx_branch.clone_and_add_component(&static_mixer_pipe_23a);
    dhx_branch.clone_and_add_component(&pipe_22);
    dhx_branch.clone_and_add_component(&flowmeter_20_21a);
    dhx_branch.clone_and_add_component(&pipe_21);
    dhx_branch.clone_and_add_component(&pipe_20);
    dhx_branch.clone_and_add_component(&pipe_19);


    dhx_branch
}

/// builds a heater branch to simulate isothermal testing of ciet
pub fn heater_branch_builder_isothermal_test(
    initial_temperature: ThermodynamicTemperature) -> FluidComponentCollection {

    let pipe_4 = new_pipe_4(initial_temperature);
    let pipe_3 = new_pipe_3_relap_model(initial_temperature);
    let static_mixer_2 = new_static_mixer_10_label_2(initial_temperature);
    let static_mixer_pipe_2a = new_pipe_2a(initial_temperature);
    let heater_top_head_1a = new_heater_top_head_1a(initial_temperature);
    let heated_section_1 = new_heated_section_version_1_label_1(initial_temperature);
    let heater_bottom_head_1b = new_heater_bottom_head_1b(initial_temperature);
    let pipe_18 = new_pipe_18(initial_temperature);

    let mut heater_branch = FluidComponentCollection::new_series_component_collection();

    heater_branch.clone_and_add_component(&pipe_4);
    heater_branch.clone_and_add_component(&pipe_3);
    heater_branch.clone_and_add_component(&static_mixer_2);
    heater_branch.clone_and_add_component(&static_mixer_pipe_2a);
    heater_branch.clone_and_add_component(&heater_top_head_1a);
    heater_branch.clone_and_add_component(&heated_section_1);
    heater_branch.clone_and_add_component(&heater_bottom_head_1b);
    heater_branch.clone_and_add_component(&pipe_18);

    heater_branch
}

/// builds the ctah branch to simulate isothermal testing of ciet 
/// allows user to supply a pump pressure or loop pressure drop
pub fn ctah_branch_builder_isothermal_test(
    pump_pressure: Pressure,
    initial_temperature: ThermodynamicTemperature) -> FluidComponentCollection {

    let branch_5 = new_branch_5(initial_temperature);
    let static_mixer_41_label_6 = new_static_mixer_41(initial_temperature);
    let pipe_6a = new_pipe_6a(initial_temperature);
    let ctah_vertical_label_7a = new_inactive_ctah_vertical(initial_temperature);
    let ctah_horizontal_label_7b = new_inactive_ctah_horizontal(initial_temperature);
    let pipe_8a = new_pipe_8a(initial_temperature);
    let static_mixer_40_label_8 = new_static_mixer_40(initial_temperature);
    let pipe_9 = new_pipe_9(initial_temperature);
    let pipe_10 = new_pipe_10(initial_temperature);
    let pipe_11 = new_pipe_11(initial_temperature);
    let pipe_12 = new_pipe_12(initial_temperature);
    let mut ctah_pump = new_ctah_pump(initial_temperature);
    ctah_pump.set_internal_pressure_source(pump_pressure);
    let pipe_13 = new_pipe_13(initial_temperature);
    let pipe_14 = new_pipe_14(initial_temperature);
    let flowmeter_40_14a = new_flowmeter_40_14a(initial_temperature);
    let pipe_15 = new_pipe_15(initial_temperature);
    let pipe_16 = new_pipe_16(initial_temperature);
    let branch_17 = new_branch_17(initial_temperature);


    // now I want to add each of these to the fluid component 
    // collection without the constant hassle of having to convert types

    let mut ctah_branch = FluidComponentCollection::new_series_component_collection();
    
    ctah_branch.clone_and_add_component(&branch_5);
    ctah_branch.clone_and_add_component(&static_mixer_41_label_6);
    ctah_branch.clone_and_add_component(&pipe_6a);
    ctah_branch.clone_and_add_component(&ctah_vertical_label_7a);
    ctah_branch.clone_and_add_component(&ctah_horizontal_label_7b);
    ctah_branch.clone_and_add_component(&pipe_8a);
    ctah_branch.clone_and_add_component(&static_mixer_40_label_8);
    ctah_branch.clone_and_add_component(&pipe_9);
    ctah_branch.clone_and_add_component(&pipe_10);
    ctah_branch.clone_and_add_component(&pipe_11);
    ctah_branch.clone_and_add_component(&pipe_12);
    ctah_branch.clone_and_add_component(&ctah_pump);
    ctah_branch.clone_and_add_component(&pipe_13);
    ctah_branch.clone_and_add_component(&pipe_14);
    ctah_branch.clone_and_add_component(&flowmeter_40_14a);
    ctah_branch.clone_and_add_component(&pipe_15);
    ctah_branch.clone_and_add_component(&pipe_16);
    ctah_branch.clone_and_add_component(&branch_17);

    ctah_branch
}

