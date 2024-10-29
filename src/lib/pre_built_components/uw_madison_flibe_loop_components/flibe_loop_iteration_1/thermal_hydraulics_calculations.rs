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
pub fn get_abs_mass_flowrate_across_two_branches(flibe_loop_branches: &FluidComponentSuperCollection) -> 
MassRate {
    let pressure_change_across_each_branch = 
        flibe_loop_branches.get_pressure_change(MassRate::ZERO);

    let mass_flowrate_across_each_branch: Vec<MassRate> = 
        flibe_loop_branches.
        get_mass_flowrate_across_each_parallel_branch(
            pressure_change_across_each_branch
        );

    let mut mass_flowrate: MassRate = 
        mass_flowrate_across_each_branch[0];


    // get absolute value
    mass_flowrate = mass_flowrate.abs();

    mass_flowrate

}

/// fluid mechanics calcs, 
/// specific to the flibe loop
/// note that this only works if the components are correct
/// obtains mass flowrate across the FLiBe loop 
/// gets the absolute flowrate across the hot branch
pub fn uw_madison_flibe_fluid_mechanics_calc_abs_mass_rate(
    pipe_1: &InsulatedFluidComponent,
    pipe_2: &InsulatedFluidComponent,
    pipe_3: &InsulatedFluidComponent,
    pipe_4: &InsulatedFluidComponent,
    pipe_5: &NonInsulatedFluidComponent,
    pipe_6: &InsulatedFluidComponent,
    pipe_7: &NonInsulatedFluidComponent,
    pipe_8: &InsulatedFluidComponent,
    pipe_9: &InsulatedFluidComponent,
    pipe_10: &InsulatedFluidComponent,
    pipe_11: &InsulatedFluidComponent,
    pipe_12: &InsulatedFluidComponent,
    pipe_13: &InsulatedFluidComponent,
)-> MassRate {


    let mut flibe_hot_branch = 
        FluidComponentCollection::new_series_component_collection();

    flibe_hot_branch.clone_and_add_component(pipe_8);
    flibe_hot_branch.clone_and_add_component(pipe_9);
    flibe_hot_branch.clone_and_add_component(pipe_10);
    flibe_hot_branch.clone_and_add_component(pipe_11);
    flibe_hot_branch.clone_and_add_component(pipe_12);
    flibe_hot_branch.clone_and_add_component(pipe_13);
    flibe_hot_branch.clone_and_add_component(pipe_1);



    let mut flibe_cold_branch = 
        FluidComponentCollection::new_series_component_collection();

    flibe_cold_branch.clone_and_add_component(pipe_2);
    flibe_cold_branch.clone_and_add_component(pipe_3);
    flibe_cold_branch.clone_and_add_component(pipe_4);
    flibe_cold_branch.clone_and_add_component(pipe_5);
    flibe_cold_branch.clone_and_add_component(pipe_6);
    flibe_cold_branch.clone_and_add_component(pipe_7);



    let mut flibe_branches = 
        FluidComponentSuperCollection::default();

    flibe_branches.set_orientation_to_parallel();
    flibe_branches.fluid_component_super_vector.push(flibe_hot_branch);
    flibe_branches.fluid_component_super_vector.push(flibe_cold_branch);

    let abs_mass_rate = get_abs_mass_flowrate_across_two_branches(
        &flibe_branches);

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
pub fn flibe_loop_link_up_components(
    mass_flowrate_clockwise: MassRate,
    hot_leg_diagonal_heater_power: Power,
    hot_leg_vertical_heater_power: Power,
    cold_leg_diagonal_heat_transfer_coeff: HeatTransfer,
    cold_leg_vertical_heat_transfer_coeff: HeatTransfer,
    average_temperature_for_density_calcs: ThermodynamicTemperature,
    ambient_htc: HeatTransfer,
    pipe_1: &mut InsulatedFluidComponent,
    pipe_2: &mut InsulatedFluidComponent,
    pipe_3: &mut InsulatedFluidComponent,
    pipe_4: &mut InsulatedFluidComponent,
    pipe_5: &mut NonInsulatedFluidComponent,
    pipe_6: &mut InsulatedFluidComponent,
    pipe_7: &mut NonInsulatedFluidComponent,
    pipe_8: &mut InsulatedFluidComponent,
    pipe_9: &mut InsulatedFluidComponent,
    pipe_10: &mut InsulatedFluidComponent,
    pipe_11: &mut InsulatedFluidComponent,
    pipe_12: &mut InsulatedFluidComponent,
    pipe_13: &mut InsulatedFluidComponent,
    ){

        // for this function, we consider mass flowrate in clockwise 
        // fashion 

        // create the heat transfer interaction 
        let advection_clockwise_heat_transfer_interaction: HeatTransferInteractionType;

        // I'm going to create the advection interaction
        //
        // and probably for the sake of density calcs, I'll take the 
        // average density using DHX outlet and 
        // TCHX outlet temperatures, average them for the whole loop 
        // doesn't make much diff tho based on Boussinesq approximation
        //

        let average_flibe_density = 
            LiquidMaterial::FLiBe.try_get_density(
                average_temperature_for_density_calcs).unwrap();

        advection_clockwise_heat_transfer_interaction = 
            HeatTransferInteractionType::
            new_advection_interaction(mass_flowrate_clockwise, 
                average_flibe_density, 
                average_flibe_density);

        // now, let's link the fluid arrays using advection 
        // (no conduction here axially between arrays)
        // in a clockwise fashion
        //
        {
            // link the tube side arrays as per normal, 
            pipe_1.pipe_fluid_array.link_to_front(
                &mut pipe_2.pipe_fluid_array, 
                advection_clockwise_heat_transfer_interaction)
                .unwrap();

            pipe_2.pipe_fluid_array.link_to_front(
                &mut pipe_3.pipe_fluid_array, 
                advection_clockwise_heat_transfer_interaction)
                .unwrap();

            pipe_3.pipe_fluid_array.link_to_front(
                &mut pipe_4.pipe_fluid_array, 
                advection_clockwise_heat_transfer_interaction)
                .unwrap();

            pipe_4.pipe_fluid_array.link_to_front(
                &mut pipe_5.pipe_fluid_array, 
                advection_clockwise_heat_transfer_interaction)
                .unwrap();
            pipe_5.pipe_fluid_array.link_to_front(
                &mut pipe_6.pipe_fluid_array, 
                advection_clockwise_heat_transfer_interaction)
                .unwrap();
            pipe_6.pipe_fluid_array.link_to_front(
                &mut pipe_7.pipe_fluid_array, 
                advection_clockwise_heat_transfer_interaction)
                .unwrap();
            pipe_7.pipe_fluid_array.link_to_front(
                &mut pipe_8.pipe_fluid_array, 
                advection_clockwise_heat_transfer_interaction)
                .unwrap();
            pipe_8.pipe_fluid_array.link_to_front(
                &mut pipe_9.pipe_fluid_array, 
                advection_clockwise_heat_transfer_interaction)
                .unwrap();
            pipe_9.pipe_fluid_array.link_to_front(
                &mut pipe_10.pipe_fluid_array, 
                advection_clockwise_heat_transfer_interaction)
                .unwrap();
            pipe_10.pipe_fluid_array.link_to_front(
                &mut pipe_11.pipe_fluid_array, 
                advection_clockwise_heat_transfer_interaction)
                .unwrap();
            pipe_11.pipe_fluid_array.link_to_front(
                &mut pipe_12.pipe_fluid_array, 
                advection_clockwise_heat_transfer_interaction)
                .unwrap();
            pipe_12.pipe_fluid_array.link_to_front(
                &mut pipe_13.pipe_fluid_array, 
                advection_clockwise_heat_transfer_interaction)
                .unwrap();
            pipe_13.pipe_fluid_array.link_to_front(
                &mut pipe_1.pipe_fluid_array, 
                advection_clockwise_heat_transfer_interaction)
                .unwrap();

        }
        // set the relevant heat transfer coefficients
        // pipe 5 and 7 have specific heat transfer coefficients
        // set by the controller
        {
            // hot leg heater vertical side
            pipe_1.heat_transfer_to_ambient = 
                ambient_htc;

            // hot leg to cold leg opening to tank (top left)
            pipe_2.heat_transfer_to_ambient = 
                ambient_htc;
            // hot leg to cold leg bend segment 
            // approximated as two pipes (top left)
            pipe_3.heat_transfer_to_ambient = 
                ambient_htc;
            pipe_4.heat_transfer_to_ambient = 
                ambient_htc;

            // cold leg horizontal-ish (diagnoal) part
            pipe_5.heat_transfer_to_ambient = 
                cold_leg_diagonal_heat_transfer_coeff;
            // cold leg bend (y joint) top right
            pipe_6.heat_transfer_to_ambient = 
                ambient_htc;
            // cold leg vertical part
            pipe_7.heat_transfer_to_ambient = 
                cold_leg_vertical_heat_transfer_coeff;


            // cold leg bend circular segment
            // approximated as three pipes
            // (bottom right)
            pipe_8.heat_transfer_to_ambient = 
                ambient_htc;
            pipe_9.heat_transfer_to_ambient = 
                ambient_htc;
            pipe_10.heat_transfer_to_ambient = 
                ambient_htc;

            // hot leg horizontal-ish (diagonal) side 
            pipe_11.heat_transfer_to_ambient = 
                ambient_htc;

            // hot leg bend 
            // approximated as two pipes
            // (bottom left)
            pipe_12.heat_transfer_to_ambient = 
                ambient_htc;
            pipe_13.heat_transfer_to_ambient = 
                ambient_htc;


        }
        // add lateral heat losses, lateral connections for dhx not done here
        {
            let zero_power: Power = Power::ZERO;

            // hot branch
            //
            // everywhere else is zero heater power
            //
            pipe_1
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    hot_leg_vertical_heater_power)
                .unwrap();
            pipe_2
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();

            pipe_3
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();
            pipe_4
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();

            // cold branch air cooled segments
            // ambient temperature of tchx is 20C  
            pipe_5.ambient_temperature = 
                ThermodynamicTemperature::new::<degree_celsius>(20.0);
            pipe_7.ambient_temperature = 
                ThermodynamicTemperature::new::<degree_celsius>(20.0);

            pipe_5
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();
            pipe_6
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();
            pipe_7
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();

            // bottom right bend from cold to hot leg 
            pipe_8
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();
            pipe_9
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();
            pipe_10
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();

            // horizontal-ish (diagonal) heater 
            pipe_11
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    hot_leg_diagonal_heater_power)
                .unwrap();

            // hot leg bottom left bend
            pipe_12
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();
            pipe_13
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_clockwise, 
                    zero_power)
                .unwrap();

            }

        // now we should be ready to advance timestep

}


/// now the heat transfer for the DRACS loop 
/// for a single timestep, given mass flowrate in a counter clockwise 
/// fashion in the DRACS
///
/// you also must specify the heat transfer coefficient to ambient 
/// which is assumed to be the same throughout the loop
pub fn dracs_loop_advance_timestep_except_dhx_sam_tchx_calibration(
    timestep: Time,
    pipe_1: &mut InsulatedFluidComponent,
    pipe_2: &mut InsulatedFluidComponent,
    pipe_3: &mut InsulatedFluidComponent,
    pipe_4: &mut InsulatedFluidComponent,
    pipe_5: &mut NonInsulatedFluidComponent,
    pipe_6: &mut InsulatedFluidComponent,
    pipe_7: &mut NonInsulatedFluidComponent,
    pipe_8: &mut InsulatedFluidComponent,
    pipe_9: &mut InsulatedFluidComponent,
    pipe_10: &mut InsulatedFluidComponent,
    pipe_11: &mut InsulatedFluidComponent,
    pipe_12: &mut InsulatedFluidComponent,
    pipe_13: &mut InsulatedFluidComponent,
    ){


        pipe_1
            .advance_timestep(timestep)
            .unwrap();
        // cold branch
        pipe_2
            .advance_timestep(timestep)
            .unwrap();

        pipe_3
            .advance_timestep(timestep)
            .unwrap();
        pipe_4
            .advance_timestep(timestep)
            .unwrap();

        pipe_5
            .advance_timestep(timestep)
            .unwrap();
        pipe_6
            .advance_timestep(timestep)
            .unwrap();
        pipe_7
            .advance_timestep(timestep)
            .unwrap();

        pipe_8
            .advance_timestep(timestep)
            .unwrap();
        pipe_9
            .advance_timestep(timestep)
            .unwrap();
        pipe_10
            .advance_timestep(timestep)
            .unwrap();

        // hot branch 

        // horizontal-ish (diagonal) heater)
        pipe_11
            .advance_timestep(timestep)
            .unwrap();

        // bottom left heater bend
        pipe_12
            .advance_timestep(timestep)
            .unwrap();
        pipe_13
            .advance_timestep(timestep)
            .unwrap();

        // vertical heater
        pipe_1
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
