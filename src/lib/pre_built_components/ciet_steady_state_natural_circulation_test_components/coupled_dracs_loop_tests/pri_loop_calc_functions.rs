use uom::si::f64::*;

use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component::FluidComponent;
use crate::
array_control_vol_and_fluid_component_collections::
fluid_component_collection::
fluid_component_collection::FluidComponentCollection;
// let's construct the branches with test pressures and obtain 
use crate::
array_control_vol_and_fluid_component_collections::
fluid_component_collection::
fluid_component_collection::FluidComponentCollectionMethods;
use crate::pre_built_components::shell_and_tube_heat_exchanger::SimpleShellAndTubeHeatExchanger;
use uom::ConstZero;

use uom::si::thermodynamic_temperature::degree_celsius;
use crate::
array_control_vol_and_fluid_component_collections::
fluid_component_collection::
fluid_component_super_collection::FluidComponentSuperCollection;

use crate::pre_built_components::
insulated_pipes_and_fluid_components::InsulatedFluidComponent;
use crate::pre_built_components::
non_insulated_fluid_components::NonInsulatedFluidComponent;

use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::heat_transfer_correlations::heat_transfer_interactions::
heat_transfer_interaction_enums::HeatTransferInteractionType;


/// fluid mechanics bit 
/// calculate the fluid mechanics for the two branches in parallel
///
/// In actual fact though, it is just one branch and we are getting 
/// the mass flowrate through that branch,
/// can be used for
/// DHX + Heater branch (both branches form one loop)
pub fn get_mass_flowrate_across_two_branches(dhx_and_heater_branches: &FluidComponentSuperCollection) -> 
MassRate {
    let pressure_change_across_each_branch = 
        dhx_and_heater_branches.get_pressure_change(MassRate::ZERO);

    let mass_flowrate_across_each_branch: Vec<MassRate> = 
        dhx_and_heater_branches.
        get_mass_flowrate_across_each_parallel_branch(
            pressure_change_across_each_branch
        );

    let mut mass_flowrate: MassRate = 
        mass_flowrate_across_each_branch[0];


    // get absolute value
    mass_flowrate = mass_flowrate.abs();

    mass_flowrate

}

/// fluid mechanics calcs, specific to the primary (DHX plus Heater branch) loop
/// note that this only works if the components are correct
/// obtains mass flowrate across the primary (DHX plus Heater branch) loop
/// for hydrostatic pressure, all component angles are taken with 
/// reference to the branching point (around 5a)
///
/// note: only Flowmeter is  non insulated, all other components 
/// should be insulated
///
/// for hydrostatic pressure calculations, note that the angle 
/// of the dhx shell side is going from top to bottom 
///
pub fn coupled_dracs_pri_loop_branches_fluid_mechanics_calc_abs_mass_rate(
    pipe_4: &InsulatedFluidComponent,
    pipe_3: &InsulatedFluidComponent,
    pipe_2a: &InsulatedFluidComponent,
    static_mixer_10_label_2: &InsulatedFluidComponent,
    heater_top_head_1a: &InsulatedFluidComponent,
    heater_version1_1: &InsulatedFluidComponent,
    heater_bottom_head_1b: &InsulatedFluidComponent,
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
)-> MassRate {

    let mut heater_branch = 
        FluidComponentCollection::new_series_component_collection();

    heater_branch.clone_and_add_component(pipe_4);
    heater_branch.clone_and_add_component(pipe_3);
    heater_branch.clone_and_add_component(pipe_2a);
    heater_branch.clone_and_add_component(static_mixer_10_label_2);
    heater_branch.clone_and_add_component(heater_top_head_1a);
    heater_branch.clone_and_add_component(heater_version1_1);
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

    let mut pri_loop_branches = 
        FluidComponentSuperCollection::default();

    pri_loop_branches.set_orientation_to_parallel();
    pri_loop_branches.fluid_component_super_vector.push(heater_branch);
    pri_loop_branches.fluid_component_super_vector.push(dhx_branch);

    let mass_rate = get_mass_flowrate_across_two_branches(&pri_loop_branches);

    mass_rate

}

/// now the heat transfer for the DRACS loop 
/// for a single timestep, given mass flowrate in a counter clockwise 
/// fashion in the DRACS
///
/// you also must specify the heat transfer coefficient to ambient 
/// which is assumed to be the same throughout the loop
/// 
/// flow goes downwards by default through the DHX
/// to facilitate this, components are linked in a counter clockwise 
/// fashion in the primary loop
pub fn coupled_dracs_pri_loop_dhx_heater_link_up_components(
    mass_flowrate_counter_clockwise: MassRate,
    heat_rate_through_heater: Power,
    average_temperature_for_density_calcs: ThermodynamicTemperature,
    ambient_htc: HeatTransfer,
    pipe_4: &mut InsulatedFluidComponent,
    pipe_3: &mut InsulatedFluidComponent,
    pipe_2a: &mut InsulatedFluidComponent,
    static_mixer_10_label_2: &mut InsulatedFluidComponent,
    heater_top_head_1a: &mut InsulatedFluidComponent,
    heater_version1_1: &mut InsulatedFluidComponent,
    heater_bottom_head_1b: &mut InsulatedFluidComponent,
    pipe_18: &mut InsulatedFluidComponent,
    pipe_5a: &mut InsulatedFluidComponent,
    pipe_26: &mut InsulatedFluidComponent,
    pipe_25a: &mut InsulatedFluidComponent,
    static_mixer_21_label_25: &mut InsulatedFluidComponent,
    dhx_sthe: &mut SimpleShellAndTubeHeatExchanger,
    static_mixer_20_label_23: &mut InsulatedFluidComponent,
    pipe_23a: &mut InsulatedFluidComponent,
    pipe_22: &mut InsulatedFluidComponent,
    flowmeter_20_21a: &mut NonInsulatedFluidComponent,
    pipe_21: &mut InsulatedFluidComponent,
    pipe_20: &mut InsulatedFluidComponent,
    pipe_19: &mut InsulatedFluidComponent,
    pipe_17b: &mut InsulatedFluidComponent,
    ){


        // create the heat transfer interaction 
        let advection_heat_transfer_interaction: HeatTransferInteractionType;

        // I'm going to create the advection interaction
        //
        // and probably for the sake of density calcs, I'll take the 
        // average density using DHX outlet and 
        // TCHX outlet temperatures, average them for the whole loop 
        // doesn't make much diff tho based on Boussinesq approximation

        let average_therminol_density = 
            LiquidMaterial::TherminolVP1.try_get_density(
                average_temperature_for_density_calcs).unwrap();

        advection_heat_transfer_interaction = 
            HeatTransferInteractionType::
            new_advection_interaction(mass_flowrate_counter_clockwise, 
                average_therminol_density, 
                average_therminol_density);

        // now, let's link the fluid arrays using advection 
        // (no conduction here axially between arrays)
        //
        // note that by default, flow will always go downwards for the 
        // DHX so components should be linked in a counter clockwise fashion
        {
            // first is flow from heater branch to DHX branch
            pipe_4.pipe_fluid_array.link_to_front(
                &mut pipe_5a.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            // then flow downwards in DHX branch

            pipe_5a.pipe_fluid_array.link_to_front(
                &mut pipe_26.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_26.pipe_fluid_array.link_to_front(
                &mut pipe_25a.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_25a.pipe_fluid_array.link_to_front(
                &mut static_mixer_21_label_25.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            //note: for shell side fluid array, linking normally is okay 
            //because there is only one entity
            //
            // for tube side fluid array, link normally as well, because 
            // the advance timestep portion takes care of the parallel 
            // tube treatment

            static_mixer_21_label_25.pipe_fluid_array.link_to_front(
                &mut dhx_sthe.shell_side_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            // for dhx, the flow convention in both shell and tube is 
            // from top to bottom of the branch

            dhx_sthe.shell_side_fluid_array.link_to_front(
                &mut static_mixer_20_label_23.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            static_mixer_20_label_23.pipe_fluid_array.link_to_front(
                &mut pipe_23a.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_23a.pipe_fluid_array.link_to_front(
                &mut pipe_22.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_22.pipe_fluid_array.link_to_front(
                &mut flowmeter_20_21a.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            flowmeter_20_21a.pipe_fluid_array.link_to_front(
                &mut pipe_21.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_21.pipe_fluid_array.link_to_front(
                &mut pipe_20.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();


            pipe_20.pipe_fluid_array.link_to_front(
                &mut pipe_19.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_19.pipe_fluid_array.link_to_front(
                &mut pipe_17b.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            // now from DHX flow to heater branch
            //
            pipe_17b.pipe_fluid_array.link_to_front(
                &mut pipe_18.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();
            // heater branch

            pipe_18.pipe_fluid_array.link_to_front(
                &mut heater_bottom_head_1b.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            heater_bottom_head_1b.pipe_fluid_array.link_to_front(
                &mut heater_version1_1.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            heater_version1_1.pipe_fluid_array.link_to_front(
                &mut heater_top_head_1a.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            heater_top_head_1a.pipe_fluid_array.link_to_front(
                &mut static_mixer_10_label_2.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            static_mixer_10_label_2.pipe_fluid_array.link_to_front(
                &mut pipe_2a.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_2a.pipe_fluid_array.link_to_front(
                &mut pipe_3.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_3.pipe_fluid_array.link_to_front(
                &mut pipe_4.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

        }
        // set the relevant heat transfer coefficients 
        // based on the heat transfer to ambient from insulation 
        // coeff
        //
        // also set the ambient temperature for each component
        {
            // heater branch
            pipe_18.heat_transfer_to_ambient = ambient_htc;
            heater_bottom_head_1b.heat_transfer_to_ambient = ambient_htc;
            heater_version1_1.heat_transfer_to_ambient = ambient_htc;
            heater_top_head_1a.heat_transfer_to_ambient = ambient_htc;
            static_mixer_10_label_2.heat_transfer_to_ambient = ambient_htc;
            pipe_2a.heat_transfer_to_ambient = ambient_htc;
            pipe_3.heat_transfer_to_ambient = ambient_htc;
            pipe_4.heat_transfer_to_ambient = ambient_htc;

            // DHX branch 
            pipe_5a.heat_transfer_to_ambient = ambient_htc;
            pipe_26.heat_transfer_to_ambient = ambient_htc;
            pipe_25a.heat_transfer_to_ambient = ambient_htc;
            static_mixer_21_label_25.heat_transfer_to_ambient = ambient_htc;
            dhx_sthe.heat_transfer_to_ambient = ambient_htc;
            static_mixer_20_label_23.heat_transfer_to_ambient = ambient_htc;
            pipe_23a.heat_transfer_to_ambient = ambient_htc;
            pipe_22.heat_transfer_to_ambient = ambient_htc;
            flowmeter_20_21a.heat_transfer_to_ambient = ambient_htc;
            pipe_21.heat_transfer_to_ambient = ambient_htc;
            pipe_20.heat_transfer_to_ambient = ambient_htc;
            pipe_19.heat_transfer_to_ambient = ambient_htc;

            // ambient temp
            let ambient_temp_user_set = 
                ThermodynamicTemperature::new::<degree_celsius>(20.0);

            // heater branch
            pipe_18.ambient_temperature = ambient_temp_user_set;
            heater_bottom_head_1b.ambient_temperature = ambient_temp_user_set;
            heater_version1_1.ambient_temperature = ambient_temp_user_set;
            heater_top_head_1a.ambient_temperature = ambient_temp_user_set;
            static_mixer_10_label_2.ambient_temperature = ambient_temp_user_set;
            pipe_2a.ambient_temperature = ambient_temp_user_set;
            pipe_3.ambient_temperature = ambient_temp_user_set;
            pipe_4.ambient_temperature = ambient_temp_user_set;
            pipe_5a.ambient_temperature = ambient_temp_user_set;

            // DHX branch 
            pipe_5a.ambient_temperature = ambient_temp_user_set;
            pipe_26.ambient_temperature = ambient_temp_user_set;
            pipe_25a.ambient_temperature = ambient_temp_user_set;
            static_mixer_21_label_25.ambient_temperature = ambient_temp_user_set;
            dhx_sthe.ambient_temperature = ambient_temp_user_set;
            static_mixer_20_label_23.ambient_temperature = ambient_temp_user_set;
            pipe_23a.ambient_temperature = ambient_temp_user_set;
            pipe_22.ambient_temperature = ambient_temp_user_set;
            flowmeter_20_21a.ambient_temperature = ambient_temp_user_set;
            pipe_21.ambient_temperature = ambient_temp_user_set;
            pipe_20.ambient_temperature = ambient_temp_user_set;
            pipe_19.ambient_temperature = ambient_temp_user_set;
            
        }
        // add lateral heat losses and power through heater
        // for everything except the DHX STHE
        // because DHX sthe requires mass flowrates in both sides of the loop
        {
            let zero_power: Power = Power::ZERO;
            // heater branch
            pipe_18.lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, zero_power).unwrap();
            heater_bottom_head_1b.lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, zero_power).unwrap();

            heater_version1_1.lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, heat_rate_through_heater).unwrap();

            heater_top_head_1a.lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, zero_power).unwrap();

            static_mixer_10_label_2.lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, zero_power).unwrap();

            pipe_2a.lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, zero_power).unwrap();

            pipe_3.lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, zero_power).unwrap();

            pipe_4.lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, zero_power).unwrap();


            // DHX branch 
            pipe_5a.lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, zero_power).unwrap();

            pipe_26.lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, zero_power).unwrap();
            pipe_25a.lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, zero_power).unwrap();
            static_mixer_21_label_25.lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, zero_power).unwrap();
            static_mixer_20_label_23.lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, zero_power).unwrap();
            pipe_23a.lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, zero_power).unwrap();
            pipe_22.lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, zero_power).unwrap();
            flowmeter_20_21a.lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, zero_power).unwrap();
            pipe_21.lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, zero_power).unwrap();
            pipe_20.lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, zero_power).unwrap();
            pipe_19.lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, zero_power).unwrap();
            pipe_17b.lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, zero_power).unwrap();
        }

        // now we are done
        //

}

/// advances timestep for all components in primary loop except DHX
pub fn pri_loop_advance_timestep_except_dhx(
    timestep: Time,
    pipe_4: &mut InsulatedFluidComponent,
    pipe_3: &mut InsulatedFluidComponent,
    pipe_2a: &mut InsulatedFluidComponent,
    static_mixer_10_label_2: &mut InsulatedFluidComponent,
    heater_top_head_1a: &mut InsulatedFluidComponent,
    heater_version1_1: &mut InsulatedFluidComponent,
    heater_bottom_head_1b: &mut InsulatedFluidComponent,
    pipe_18: &mut InsulatedFluidComponent,
    pipe_5a: &mut InsulatedFluidComponent,
    pipe_26: &mut InsulatedFluidComponent,
    pipe_25a: &mut InsulatedFluidComponent,
    static_mixer_21_label_25: &mut InsulatedFluidComponent,
    static_mixer_20_label_23: &mut InsulatedFluidComponent,
    pipe_23a: &mut InsulatedFluidComponent,
    pipe_22: &mut InsulatedFluidComponent,
    flowmeter_20_21a: &mut NonInsulatedFluidComponent,
    pipe_21: &mut InsulatedFluidComponent,
    pipe_20: &mut InsulatedFluidComponent,
    pipe_19: &mut InsulatedFluidComponent,
    pipe_17b: &mut InsulatedFluidComponent,
){

    // heater branch
    pipe_4.advance_timestep(timestep).unwrap();
    pipe_3.advance_timestep(timestep).unwrap();
    pipe_2a.advance_timestep(timestep).unwrap();
    static_mixer_10_label_2.advance_timestep(timestep).unwrap();
    heater_top_head_1a.advance_timestep(timestep).unwrap();
    heater_version1_1.advance_timestep(timestep).unwrap();
    heater_bottom_head_1b.advance_timestep(timestep).unwrap();
    pipe_18.advance_timestep(timestep).unwrap();


    // DHX branch (except DHX shell side)
    pipe_5a.advance_timestep(timestep).unwrap();
    pipe_26.advance_timestep(timestep).unwrap();
    pipe_25a.advance_timestep(timestep).unwrap();
    static_mixer_21_label_25.advance_timestep(timestep).unwrap();
    static_mixer_20_label_23.advance_timestep(timestep).unwrap();
    pipe_23a.advance_timestep(timestep).unwrap();
    pipe_22.advance_timestep(timestep).unwrap();
    flowmeter_20_21a.advance_timestep(timestep).unwrap();
    pipe_21.advance_timestep(timestep).unwrap();
    pipe_20.advance_timestep(timestep).unwrap();
    pipe_19.advance_timestep(timestep).unwrap();
    pipe_17b.advance_timestep(timestep).unwrap();

}

/// these are temperature diagnostic 
/// functions to check bulk and wall temperature before 
/// and after the heater 
///
/// before heater: BT-11, WT-10 
/// after heater and MX-10: BT-12, WT-13
///
/// so can take heater bottom head temperature (1b) at wall 
/// and at bulk
///
/// I'm also using the bulk fluid temperature inside the static mixer 
/// and its wall temperature
/// as a proxy for BT-12 and WT-12
pub fn pri_loop_heater_temperature_diagnostics(
    heater_bottom_head_1b: &mut InsulatedFluidComponent,
    static_mixer_10_label_2: &mut InsulatedFluidComponent,
    print_debug_results: bool)
-> ((ThermodynamicTemperature,ThermodynamicTemperature),
(ThermodynamicTemperature,ThermodynamicTemperature)){

    // bulk and wall temperatures before entering heater
    let bt_11 = heater_bottom_head_1b.
        pipe_fluid_array.try_get_bulk_temperature().unwrap();
    let wt_10 = heater_bottom_head_1b.
        pipe_shell.try_get_bulk_temperature().unwrap();

    // bulk and wall temperatures after entering heater 
    let bt_12 = static_mixer_10_label_2.
        pipe_fluid_array.try_get_bulk_temperature().unwrap();
    let wt_13 = static_mixer_10_label_2 
        .pipe_shell.try_get_bulk_temperature().unwrap();

    // debug 
    if print_debug_results {
        dbg!(&(
                "bulk and wall temp degC, before and after heater respectively",
                bt_11.get::<degree_celsius>(),
                wt_10.get::<degree_celsius>(),
                bt_12.get::<degree_celsius>(),
                wt_13.get::<degree_celsius>(),
                ));
    }


    return ((bt_11,wt_10),(bt_12,wt_13));

}
/// these are temperature diagnostic 
/// functions to check bulk and wall temperature before 
/// and after the DHX shell side
///
/// before dhx shell: BT-21, WT-20 (just before MX-21)
/// use pipe_25a
/// after dhx shell and MX-20: BT-27, WT-26
/// use MX-20
///
pub fn pri_loop_dhx_shell_temperature_diagnostics(
    pipe_25a: &mut InsulatedFluidComponent,
    static_mixer_20_label_23: &mut InsulatedFluidComponent,
    print_debug_results: bool)
-> ((ThermodynamicTemperature,ThermodynamicTemperature),
(ThermodynamicTemperature,ThermodynamicTemperature)){

    // bulk and wall temperatures before entering dhx_shell
    let bt_21 = pipe_25a.
        pipe_fluid_array.try_get_bulk_temperature().unwrap();
    let wt_20 = pipe_25a.
        pipe_shell.try_get_bulk_temperature().unwrap();

    // bulk and wall temperatures after entering dhx_shell 
    let bt_27 = static_mixer_20_label_23.
        pipe_fluid_array.try_get_bulk_temperature().unwrap();
    let wt_26 = static_mixer_20_label_23 
        .pipe_shell.try_get_bulk_temperature().unwrap();

    // debug 
    if print_debug_results {
        dbg!(&(
                "bulk and wall temp degC, before and after dhx_shell respectively",
                bt_21.get::<degree_celsius>(),
                wt_20.get::<degree_celsius>(),
                bt_27.get::<degree_celsius>(),
                wt_26.get::<degree_celsius>(),
                ));
    }


    return ((bt_21,wt_20),(bt_27,wt_26));

}
