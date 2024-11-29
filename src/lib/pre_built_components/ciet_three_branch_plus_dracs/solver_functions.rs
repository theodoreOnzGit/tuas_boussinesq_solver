// first, copy and paste the dracs loop functions over 
//
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
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
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


/// fluid mechanics bit for DRACS loop
/// calculate the fluid mechanics for the two branches in parallel
/// basically, mass flowrate
///
/// but its use is primarily for the DRACS branches in the DRACS 
/// loop
pub fn get_abs_mass_flowrate_across_dracs_branches(
    dracs_branches: &FluidComponentSuperCollection) -> 
MassRate {
    // basically the net flowrate through the two branches as a 
    // while is zero
    let pressure_change_across_each_branch = 
        dracs_branches.get_pressure_change(MassRate::ZERO);

    let mass_flowrate_across_each_branch: Vec<MassRate> = 
        dracs_branches.
        get_mass_flowrate_across_each_parallel_branch(
            pressure_change_across_each_branch
        );

    let mut mass_flowrate: MassRate = 
        mass_flowrate_across_each_branch[0];


    // get absolute value
    mass_flowrate = mass_flowrate.abs();

    mass_flowrate

}


/// fluid mechanics bit for pri loop
/// calculate the fluid mechanics for the two branches in parallel
/// basically, mass flowrate
///
pub fn get_mass_flowrate_two_branches(
    dracs_branches: &FluidComponentSuperCollection) -> 
(MassRate, MassRate) {
    // basically the net flowrate through the two branches as a 
    // while is zero
    let pressure_change_across_each_branch = 
        dracs_branches.get_pressure_change(MassRate::ZERO);

    let mass_flowrate_across_each_branch: Vec<MassRate> = 
        dracs_branches.
        get_mass_flowrate_across_each_parallel_branch(
            pressure_change_across_each_branch
        );

    // note, the mass flowrate order depends on how u add the branches 
    let mass_flow_branch_1 = mass_flowrate_across_each_branch[0];
    let mass_flow_branch_2 = mass_flowrate_across_each_branch[1];

    return (mass_flow_branch_1, mass_flow_branch_2);



}

/// fluid mechanics bit for primary loop 
/// calculate fluid 
/// calculate the fluid mechanics for the three branches in parallel
/// basically, mass flowrate
///
/// but its use is primarily for the DHX, Heater and CTAH branches
///
pub fn get_mass_flowrate_vector_for_dhx_heater_and_ctah_branches
(pri_loop_branches: &FluidComponentSuperCollection) -> 
(MassRate, MassRate, MassRate) {

    // basically the net flowrate through the three branches as a 
    // while is zero
    let pressure_change_across_each_branch = 
        pri_loop_branches.get_pressure_change(MassRate::ZERO);

    let mass_flowrate_across_each_branch: Vec<MassRate> = 
        pri_loop_branches.
        get_mass_flowrate_across_each_parallel_branch(
            pressure_change_across_each_branch
        );

    // note, the mass flowrate order depends on how u add the branches 
    let mass_flow_branch_1 = mass_flowrate_across_each_branch[0];
    let mass_flow_branch_2 = mass_flowrate_across_each_branch[1];
    let mass_flow_branch_3 = mass_flowrate_across_each_branch[2];

    return (mass_flow_branch_1, 
        mass_flow_branch_2, mass_flow_branch_3);


}

/// fluid mechanics calcs, specific to the primary loop
/// note that this only works if the components are correct
/// obtains mass flowrate across the primary loop 
/// gets flowrate across dhx, heater and ctah branches, in that order 
/// user must also specify a pump absolute pressure 
/// 
/// (pressure drop) not using pump curves here yet
/// 
pub fn three_branch_pri_loop_flowrates(
    pump_pressure: Pressure,
    ctah_branch_blocked: bool,
    dhx_branch_blocked: bool,
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
    flowmeter_40_14a :&InsulatedFluidComponent,
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

    if ctah_branch_blocked {

        pri_loop_branches.fluid_component_super_vector.push(dhx_branch);
        pri_loop_branches.fluid_component_super_vector.push(heater_branch);

        let (dhx_flow, heater_flow) = 
            get_mass_flowrate_two_branches(
                &pri_loop_branches);

        return (dhx_flow, heater_flow, MassRate::ZERO);

    } else if dhx_branch_blocked {

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

        // all loops opened
        pri_loop_branches.fluid_component_super_vector.push(dhx_branch);
        pri_loop_branches.fluid_component_super_vector.push(heater_branch);
        pri_loop_branches.fluid_component_super_vector.push(ctah_branch);

        let (dhx_flow, heater_flow, ctah_flow) = 
            get_mass_flowrate_vector_for_dhx_heater_and_ctah_branches(
                &pri_loop_branches);

        // if dhx flow is downwards, (positive flow, is ok)
        // if negative flow, then block it 

        let flow_diode_block_flow: bool = dhx_flow < MassRate::ZERO;

        let dhx_block_flow = flow_diode_block_flow;

        if flow_diode_block_flow {

            return three_branch_pri_loop_flowrates(
                pump_pressure, 
                ctah_branch_blocked, 
                dhx_block_flow, 
                pipe_4, 
                pipe_3, 
                pipe_2a, 
                static_mixer_10_label_2, 
                heater_top_head_1a, 
                heater_version1_1, 
                heater_bottom_head_1b, 
                pipe_18, 
                pipe_5a, 
                pipe_26, 
                pipe_25a, 
                static_mixer_21_label_25, 
                dhx_shell_side_pipe_24, 
                static_mixer_20_label_23, 
                pipe_23a, 
                pipe_22, 
                flowmeter_20_21a, 
                pipe_21, 
                pipe_20, 
                pipe_19, 
                pipe_17b, 
                pipe_5b, 
                static_mixer_41_label_6, 
                pipe_6a, 
                ctah_vertical_label_7a, 
                ctah_horizontal_label_7b, 
                pipe_8a, 
                static_mixer_40_label_8, 
                pipe_9, 
                pipe_10, 
                pipe_11,
                pipe_12, 
                ctah_pump, 
                pipe_13, 
                pipe_14, 
                flowmeter_40_14a, 
                pipe_15, 
                pipe_16, 
                pipe_17a);

        } else {

            return (dhx_flow, heater_flow, ctah_flow);
        }


    }

}



/// fluid mechanics calcs, specific to the DRACS loop
/// note that this only works if the components are correct
/// obtains mass flowrate across the DRACS loop 
/// gets the absolute flowrate across the hot branch
pub fn coupled_dracs_fluid_mechanics_calc_abs_mass_rate_sam_tchx_calibration(
    pipe_34: &InsulatedFluidComponent,
    pipe_33: &InsulatedFluidComponent,
    pipe_32: &InsulatedFluidComponent,
    pipe_31a: &InsulatedFluidComponent,
    static_mixer_61_label_31: &InsulatedFluidComponent,
    dhx_tube_side_30b: &NonInsulatedFluidComponent,
    dhx_tube_side_heat_exchanger_30: &FluidComponent,
    dhx_tube_side_30a: &NonInsulatedFluidComponent,
    tchx_35a: &NonInsulatedFluidComponent,
    tchx_35b_1: &NonInsulatedFluidComponent,
    tchx_35b_2: &NonInsulatedFluidComponent,
    static_mixer_60_label_36: &InsulatedFluidComponent,
    pipe_36a: &InsulatedFluidComponent,
    pipe_37: &InsulatedFluidComponent,
    flowmeter_60_37a: &NonInsulatedFluidComponent,
    pipe_38: &InsulatedFluidComponent,
    pipe_39: &InsulatedFluidComponent,
)-> MassRate {

    let mut dracs_hot_branch = 
        FluidComponentCollection::new_series_component_collection();

    dracs_hot_branch.clone_and_add_component(pipe_34);
    dracs_hot_branch.clone_and_add_component(pipe_33);
    dracs_hot_branch.clone_and_add_component(pipe_32);
    dracs_hot_branch.clone_and_add_component(pipe_31a);
    dracs_hot_branch.clone_and_add_component(static_mixer_61_label_31);
    dracs_hot_branch.clone_and_add_component(dhx_tube_side_30b);
    dracs_hot_branch.clone_and_add_component(dhx_tube_side_heat_exchanger_30);
    dracs_hot_branch.clone_and_add_component(dhx_tube_side_30a);


    let mut dracs_cold_branch = 
        FluidComponentCollection::new_series_component_collection();

    dracs_cold_branch.clone_and_add_component(tchx_35a);
    dracs_cold_branch.clone_and_add_component(tchx_35b_1);
    dracs_cold_branch.clone_and_add_component(tchx_35b_2);
    dracs_cold_branch.clone_and_add_component(static_mixer_60_label_36);
    dracs_cold_branch.clone_and_add_component(pipe_36a);
    dracs_cold_branch.clone_and_add_component(pipe_37);
    dracs_cold_branch.clone_and_add_component(flowmeter_60_37a);
    dracs_cold_branch.clone_and_add_component(pipe_38);
    dracs_cold_branch.clone_and_add_component(pipe_39);

    let mut dracs_branches = 
        FluidComponentSuperCollection::default();

    dracs_branches.set_orientation_to_parallel();
    dracs_branches.fluid_component_super_vector.push(dracs_hot_branch);
    dracs_branches.fluid_component_super_vector.push(dracs_cold_branch);

    let abs_mass_rate = get_abs_mass_flowrate_across_dracs_branches(&dracs_branches);

    abs_mass_rate

}

/// now the heat transfer for the DRACS loop 
/// for a single timestep, given mass flowrate in a counter clockwise 
/// fashion in the DRACS
///
/// you also must specify the heat transfer coefficient to ambient 
/// which is assumed to be the same throughout the loop
///
///
/// for DHX, the flow convention is going from top to bottom for both 
/// shell and tube. The code is written such that components are linked 
/// in a clockwise fashion, so that flow goes from top to bottom 
/// in the tube side of the DHX. 
///
/// the mass_flowrate_counter_clockwise you provide will be converted
/// into a mass_flowrate_clockwise and used for calculation
pub fn coupled_dracs_loop_link_up_components_sam_tchx_calibration(
    mass_flowrate_counter_clockwise: MassRate,
    tchx_heat_transfer_coeff: HeatTransfer,
    average_temperature_for_density_calcs: ThermodynamicTemperature,
    ambient_htc: HeatTransfer,
    pipe_34: &mut InsulatedFluidComponent,
    pipe_33: &mut InsulatedFluidComponent,
    pipe_32: &mut InsulatedFluidComponent,
    pipe_31a: &mut InsulatedFluidComponent,
    static_mixer_61_label_31: &mut InsulatedFluidComponent,
    dhx_tube_side_30b: &mut NonInsulatedFluidComponent,
    dhx_sthe: &mut SimpleShellAndTubeHeatExchanger,
    dhx_tube_side_30a: &mut NonInsulatedFluidComponent,
    tchx_35a: &mut NonInsulatedFluidComponent,
    tchx_35b_1: &mut NonInsulatedFluidComponent,
    tchx_35b_2: &mut NonInsulatedFluidComponent,
    static_mixer_60_label_36: &mut InsulatedFluidComponent,
    pipe_36a: &mut InsulatedFluidComponent,
    pipe_37: &mut InsulatedFluidComponent,
    flowmeter_60_37a: &mut NonInsulatedFluidComponent,
    pipe_38: &mut InsulatedFluidComponent,
    pipe_39: &mut InsulatedFluidComponent,
    ){

        // for this function, we consider mass flowrate in clockwise 
        // fashion rather than counter clockwise
        let mass_flowrate_clockwise = -mass_flowrate_counter_clockwise;

        // create the heat transfer interaction 
        let advection_heat_transfer_interaction: HeatTransferInteractionType;

        // I'm going to create the advection interaction
        //
        // and probably for the sake of density calcs, I'll take the 
        // average density using DHX outlet and 
        // TCHX outlet temperatures, average them for the whole loop 
        // doesn't make much diff tho based on Boussinesq approximation
        //

        let average_therminol_density = 
            LiquidMaterial::TherminolVP1.try_get_density(
                average_temperature_for_density_calcs).unwrap();

        advection_heat_transfer_interaction = 
            HeatTransferInteractionType::
            new_advection_interaction(mass_flowrate_clockwise, 
                average_therminol_density, 
                average_therminol_density);

        // now, let's link the fluid arrays using advection 
        // (no conduction here axially between arrays)
        //
        // for dhx, the flow convention in both shell and tube is 
        // from top to bottom of the branch
        // so there needs to be some inversion here
        // if counter clockwise is positive direction, then flow is 
        // flowing upward through the DHX,
        //
        // for the DHX, we should consider clockwise direction flow
        // and we take the clockwise mass flowrate through the DRACS 
        // loop as the mass flowrate through the tubes
        {
            // link the tube side arrays as per normal, the parallel tube 
            // treatment is accounted for in the advance timestep portion
            //

            dhx_tube_side_30a.pipe_fluid_array.link_to_back(
                &mut dhx_sthe.tube_side_fluid_array_for_single_tube, 
                advection_heat_transfer_interaction)
                .unwrap();


            dhx_sthe.tube_side_fluid_array_for_single_tube.link_to_back(
                &mut dhx_tube_side_30b.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            dhx_tube_side_30b.pipe_fluid_array.link_to_back(
                &mut static_mixer_61_label_31.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            static_mixer_61_label_31.pipe_fluid_array.link_to_back(
                &mut pipe_31a.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_31a.pipe_fluid_array.link_to_back(
                &mut pipe_32.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_32.pipe_fluid_array.link_to_back(
                &mut pipe_33.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_33.pipe_fluid_array.link_to_back(
                &mut pipe_34.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_34.pipe_fluid_array.link_to_back(
                &mut tchx_35a.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            tchx_35a.pipe_fluid_array.link_to_back(
                &mut tchx_35b_1.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            tchx_35b_1.pipe_fluid_array.link_to_back(
                &mut tchx_35b_2.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            tchx_35b_2.pipe_fluid_array.link_to_back(
                &mut static_mixer_60_label_36.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            static_mixer_60_label_36.pipe_fluid_array.link_to_back(
                &mut pipe_36a.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_36a.pipe_fluid_array.link_to_back(
                &mut pipe_37.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_37.pipe_fluid_array.link_to_back(
                &mut flowmeter_60_37a.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            flowmeter_60_37a.pipe_fluid_array.link_to_back(
                &mut pipe_38.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_38.pipe_fluid_array.link_to_back(
                &mut pipe_39.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_39.pipe_fluid_array.link_to_back(
                &mut dhx_tube_side_30a.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            }
        // set the relevant heat transfer coefficients 
        {
            // hot branch
            dhx_tube_side_30a.heat_transfer_to_ambient = 
                ambient_htc;
            dhx_tube_side_30b.heat_transfer_to_ambient = 
                ambient_htc;

            static_mixer_61_label_31.heat_transfer_to_ambient = 
                ambient_htc;
            pipe_31a.heat_transfer_to_ambient = 
                ambient_htc;

            pipe_32.heat_transfer_to_ambient = 
                ambient_htc;
            pipe_33.heat_transfer_to_ambient = 
                ambient_htc;
            pipe_34.heat_transfer_to_ambient = 
                ambient_htc;

            // cold branch 
            tchx_35a.heat_transfer_to_ambient = 
                HeatTransfer::ZERO;
            tchx_35b_1.heat_transfer_to_ambient = 
                HeatTransfer::ZERO;
            tchx_35b_2.heat_transfer_to_ambient = 
                tchx_heat_transfer_coeff;

            static_mixer_60_label_36.heat_transfer_to_ambient = 
                ambient_htc;
            pipe_36a.heat_transfer_to_ambient = 
                ambient_htc;

            pipe_37.heat_transfer_to_ambient = 
                ambient_htc;
            pipe_38.heat_transfer_to_ambient = 
                ambient_htc;
            pipe_39.heat_transfer_to_ambient = 
                ambient_htc;

        }
        // add lateral heat losses, lateral connections for dhx not done here
        {
            let zero_power: Power = Power::ZERO;

            // hot branch
            //
            // everywhere else is zero heater power
            //
            dhx_tube_side_30a
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();
            dhx_tube_side_30b
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();

            static_mixer_61_label_31
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();
            pipe_31a
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();
            pipe_32
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();
            pipe_33
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();
            pipe_34
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();

            // cold branch 
            // ambient temperature of tchx is 20C  
            tchx_35a.ambient_temperature = 
                ThermodynamicTemperature::new::<degree_celsius>(20.0);
            tchx_35b_1.ambient_temperature = 
                ThermodynamicTemperature::new::<degree_celsius>(20.0);
            tchx_35b_2.ambient_temperature = 
                ThermodynamicTemperature::new::<degree_celsius>(20.0);

            tchx_35a
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();
            tchx_35b_1
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();
            tchx_35b_2
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();


            static_mixer_60_label_36
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();
            pipe_36a
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();

            pipe_37
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();

            flowmeter_60_37a
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();
            pipe_38
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();
            pipe_39
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();

            }

        // now we should be ready to advance timestep
        // for all except the dhx itself

}


/// now the heat transfer for the DRACS loop 
/// for a single timestep, given mass flowrate in a counter clockwise 
/// fashion in the DRACS
///
/// you also must specify the heat transfer coefficient to ambient 
/// which is assumed to be the same throughout the loop
pub fn dracs_loop_advance_timestep_except_dhx_sam_tchx_calibration(
    timestep: Time,
    pipe_34: &mut InsulatedFluidComponent,
    pipe_33: &mut InsulatedFluidComponent,
    pipe_32: &mut InsulatedFluidComponent,
    pipe_31a: &mut InsulatedFluidComponent,
    static_mixer_61_label_31: &mut InsulatedFluidComponent,
    dhx_tube_side_30b: &mut NonInsulatedFluidComponent,
    dhx_tube_side_30a: &mut NonInsulatedFluidComponent,
    tchx_35a: &mut NonInsulatedFluidComponent,
    tchx_35b_1: &mut NonInsulatedFluidComponent,
    tchx_35b_2: &mut NonInsulatedFluidComponent,
    static_mixer_60_label_36: &mut InsulatedFluidComponent,
    pipe_36a: &mut InsulatedFluidComponent,
    pipe_37: &mut InsulatedFluidComponent,
    flowmeter_60_37a: &mut NonInsulatedFluidComponent,
    pipe_38: &mut InsulatedFluidComponent,
    pipe_39: &mut InsulatedFluidComponent,
    ){


        dhx_tube_side_30a
            .advance_timestep(timestep)
            .unwrap();
        dhx_tube_side_30b
            .advance_timestep(timestep)
            .unwrap();

        static_mixer_61_label_31
            .advance_timestep(timestep)
            .unwrap();
        pipe_31a
            .advance_timestep(timestep)
            .unwrap();

        pipe_32
            .advance_timestep(timestep)
            .unwrap();
        pipe_33
            .advance_timestep(timestep)
            .unwrap();
        pipe_34
            .advance_timestep(timestep)
            .unwrap();

        // cold branch 
        tchx_35a
            .advance_timestep(timestep)
            .unwrap();
        tchx_35b_1
            .advance_timestep(timestep)
            .unwrap();
        tchx_35b_2
            .advance_timestep(timestep)
            .unwrap();

        static_mixer_60_label_36
            .advance_timestep(timestep)
            .unwrap();
        pipe_36a
            .advance_timestep(timestep)
            .unwrap();
        flowmeter_60_37a
            .advance_timestep(timestep)
            .unwrap();

        pipe_37
            .advance_timestep(timestep)
            .unwrap();
        pipe_38
            .advance_timestep(timestep)
            .unwrap();
        pipe_39
            .advance_timestep(timestep)
            .unwrap();

}


/// these are temperature diagnostic 
/// functions to check bulk and wall temperature before 
/// and after the DHX tube side
///
/// before dhx tube: BT-60, WT-61 (not exactly sure where)
/// use pipe_30a
/// after dhx tube: BT-23, WT-22 
/// use pipe_30b
/// 
pub fn dracs_loop_dhx_tube_temperature_diagnostics(
    dhx_tube_side_30a: &mut NonInsulatedFluidComponent,
    dhx_tube_side_30b: &mut NonInsulatedFluidComponent,
    print_debug_results: bool)
-> ((ThermodynamicTemperature,ThermodynamicTemperature),
(ThermodynamicTemperature,ThermodynamicTemperature)){

    // bulk and wall temperatures before entering dhx_tube
    let bt_60 = dhx_tube_side_30a.
        pipe_fluid_array.try_get_bulk_temperature().unwrap();
    let wt_61 = dhx_tube_side_30a.
        pipe_shell.try_get_bulk_temperature().unwrap();

    // bulk and wall temperatures after entering dhx_tube
    let bt_23 = dhx_tube_side_30b.
        pipe_fluid_array.try_get_bulk_temperature().unwrap();
    let wt_22 = dhx_tube_side_30b 
        .pipe_shell.try_get_bulk_temperature().unwrap();

    // debug 
    if print_debug_results {
        dbg!(&(
                "bulk and wall temp degC, before and after dhx_tube respectively",
                bt_60.get::<degree_celsius>(),
                wt_61.get::<degree_celsius>(),
                bt_23.get::<degree_celsius>(),
                wt_22.get::<degree_celsius>(),
                ));
    }


    return ((bt_60,wt_61),(bt_23,wt_22));

}

