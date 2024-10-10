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
///
/// can be used for DRACS 
/// or the 
/// DHX + Heater branch (both branches form one loop)
///
/// but its use is primarily for the DRACS branch
pub fn get_abs_mass_flowrate_across_two_branches(dracs_branches: &FluidComponentSuperCollection) -> 
MassRate {
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

    let abs_mass_rate = get_abs_mass_flowrate_across_two_branches(&dracs_branches);

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
