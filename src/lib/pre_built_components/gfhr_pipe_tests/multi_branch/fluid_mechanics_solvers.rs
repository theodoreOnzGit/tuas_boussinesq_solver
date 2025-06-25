use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_collection::FluidComponentCollection;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_collection::FluidComponentCollectionMethods;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_super_collection::FluidComponentSuperCollection;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::super_collection_series_and_parallel_functions::FluidComponentSuperCollectionParallelAssociatedFunctions;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::pre_built_components::gfhr_pipe_tests::multi_branch::multi_branch_solvers::calculate_pressure_change_using_guessed_branch_mass_flowrate_fhr_sim_v1_custom;
use crate::pre_built_components::gfhr_pipe_tests::multi_branch::single_branch_solvers::calculate_mass_flowrate_from_pressure_change_for_single_branch_fhr_sim_custom;
use crate::pre_built_components::insulated_pipes_and_fluid_components::InsulatedFluidComponent;
use crate::pre_built_components::non_insulated_fluid_components::NonInsulatedFluidComponent;
use crate::pre_built_components::shell_and_tube_heat_exchanger::SimpleShellAndTubeHeatExchanger;
use crate::prelude::beta_testing::FluidArray;
use crate::prelude::beta_testing::HeatTransferEntity;
use crate::prelude::beta_testing::HeatTransferInteractionType;
use ndarray::Array;
use ndarray::Array1;
use uom::si::mass_rate::kilogram_per_second;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::ConstZero;
use uom::si::f64::*;

/// for the gFHR primary loop, and intermediate loop 
/// there are four branches that need to be solved for flowrate 
///
/// this code handles the solution procedure
/// using the tuas_boussinesq_solver library code
///
/// and handles fluid mechanics and heat transfer for one time step
pub(crate) fn four_branch_pri_and_intermediate_loop_single_time_step(
    pri_loop_pump_pressure: Pressure,
    intrmd_loop_pump_pressure: Pressure,
    reactor_power: Power,
    timestep: Time,
    // diagnostics 
    simulation_time: Time,
    // reactor branch
    reactor_pipe_1: &mut InsulatedFluidComponent,
    // downcomer branch 1
    downcomer_pipe_2: &mut InsulatedFluidComponent,
    // downcomer branch 2
    downcomer_pipe_3: &mut InsulatedFluidComponent,
    // mixing nodes for pri loop
    bottom_mixing_node_pri_loop: &mut HeatTransferEntity,
    top_mixing_node_pri_loop: &mut HeatTransferEntity,
    // Intermediate heat exchanger branch in pri loop
    fhr_pipe_11: &mut InsulatedFluidComponent,
    fhr_pipe_10: &mut InsulatedFluidComponent,
    fhr_pri_loop_pump_9: &mut NonInsulatedFluidComponent,
    fhr_pipe_8: &mut InsulatedFluidComponent,
    fhr_pipe_7: &mut InsulatedFluidComponent,
    ihx_sthe_6: &mut SimpleShellAndTubeHeatExchanger,
    fhr_pipe_5: &mut InsulatedFluidComponent,
    fhr_pipe_4: &mut InsulatedFluidComponent,
    // intermediate loop ihx side
    fhr_pipe_17: &mut InsulatedFluidComponent,
    fhr_pipe_12: &mut InsulatedFluidComponent,
    // intermediate loop steam generator side
    fhr_intrmd_loop_pump_16: &mut NonInsulatedFluidComponent,
    fhr_pipe_15: &mut InsulatedFluidComponent,
    fhr_steam_generator_shell_side_14: &mut NonInsulatedFluidComponent,
    fhr_pipe_13: &mut InsulatedFluidComponent,
    // mixing nodes for intermediate loop
    bottom_mixing_node_intrmd_loop: &mut HeatTransferEntity,
    top_mixing_node_intrmd_loop: &mut HeatTransferEntity,

    ) -> FHRState {

        // fluid mechnaics portion for both loops


        let (reactor_branch_flow, downcomer_branch_1_flow,
            downcomer_branch_2_flow, pri_loop_intermediate_heat_exchanger_branch_flow,
            intrmd_loop_ihx_br_flow, intrmd_loop_steam_gen_br_flow)
            = four_branch_pri_and_intermediate_loop_fluid_mechanics_only(
                pri_loop_pump_pressure, 
                intrmd_loop_pump_pressure, 
                reactor_pipe_1, 
                downcomer_pipe_2, 
                downcomer_pipe_3, 
                fhr_pipe_11, 
                fhr_pipe_10, 
                fhr_pri_loop_pump_9, 
                fhr_pipe_8, 
                fhr_pipe_7, 
                ihx_sthe_6, 
                fhr_pipe_5, 
                fhr_pipe_4, 
                fhr_pipe_17, 
                fhr_pipe_12, 
                fhr_intrmd_loop_pump_16, 
                fhr_pipe_15, 
                fhr_steam_generator_shell_side_14, 
                fhr_pipe_13);

        // thermal hydraulics part
        //
        // first, we are going to make heat transfer interactions
        // need rough temperature for density calcs, not super 
        // important as we assume boussineq approximation 
        // ie density differences only important for buoyancy calcs
        let average_temperature_for_density_calcs_pri_loop = 
            ThermodynamicTemperature::new::<degree_celsius>(600.0);


        let average_flibe_density = 
            LiquidMaterial::FLiBe.try_get_density(
                average_temperature_for_density_calcs_pri_loop).unwrap();

        let downcomer_branch_1_advection_heat_transfer_interaction = 
            HeatTransferInteractionType::
            new_advection_interaction(downcomer_branch_1_flow, 
                average_flibe_density, 
                average_flibe_density);

        let downcomer_branch_2_advection_heat_transfer_interaction = 
            HeatTransferInteractionType::
            new_advection_interaction(downcomer_branch_2_flow, 
                average_flibe_density, 
                average_flibe_density);

        let reactor_branch_advection_heat_transfer_interaction = 
            HeatTransferInteractionType::
            new_advection_interaction(reactor_branch_flow, 
                average_flibe_density, 
                average_flibe_density);

        let ihx_advection_heat_transfer_interaction = 
            HeatTransferInteractionType::
            new_advection_interaction(pri_loop_intermediate_heat_exchanger_branch_flow, 
                average_flibe_density, 
                average_flibe_density);
        // for intermediate loop, we use lower temp, 
        // about 450 C
        //
        // as it is a HITEC salt (nitrate salt)
        let average_temperature_for_density_calcs_intrmd_loop = 
            ThermodynamicTemperature::new::<degree_celsius>(450.0);

        let average_hitec_density = 
            LiquidMaterial::HITEC.try_get_density(
                average_temperature_for_density_calcs_intrmd_loop).unwrap();

        let intrmd_loop_ihx_br_heat_transfer_interaction = 
            HeatTransferInteractionType::
            new_advection_interaction(intrmd_loop_ihx_br_flow, 
                average_hitec_density, 
                average_hitec_density);
        let intrmd_loop_steam_gen_br_heat_transfer_interaction = 
            HeatTransferInteractionType::
            new_advection_interaction(intrmd_loop_steam_gen_br_flow, 
                average_hitec_density, 
                average_hitec_density);

        // note that reactor branch flow, 
        // downcomer_branch_1_flow, 
        // downcomer_branch_1_flow and 
        // intermediate_heat_exchanger_branch_flow in the pri loop 
        // all go from bottom mixing node to top mixing node
        //
        // with this in mind, we now link up the components 

        // downcomer 1 branch
        {
            bottom_mixing_node_pri_loop.link_to_front(
                &mut downcomer_pipe_2.pipe_fluid_array, 
                downcomer_branch_1_advection_heat_transfer_interaction)
                .unwrap();

            downcomer_pipe_2.pipe_fluid_array.link_to_front(
                top_mixing_node_pri_loop, 
                downcomer_branch_1_advection_heat_transfer_interaction)
                .unwrap();

            println!("downcomer br 1 linking complete");
        }
        // downcomer 2 branch
        {
            bottom_mixing_node_pri_loop.link_to_front(
                &mut downcomer_pipe_3.pipe_fluid_array, 
                downcomer_branch_2_advection_heat_transfer_interaction)
                .unwrap();
            
            downcomer_pipe_3.pipe_fluid_array.link_to_front(
                top_mixing_node_pri_loop, 
                downcomer_branch_2_advection_heat_transfer_interaction)
                .unwrap();
            println!("downcomer br 2 linking complete");
        }
        // pri loop 
        // ihx branch 
        {

            bottom_mixing_node_pri_loop.link_to_front(
                &mut fhr_pipe_11.pipe_fluid_array, 
                ihx_advection_heat_transfer_interaction)
                .unwrap();

            fhr_pipe_11.pipe_fluid_array.link_to_front(
                &mut fhr_pipe_10.pipe_fluid_array, 
                ihx_advection_heat_transfer_interaction)
                .unwrap();

            fhr_pipe_10.pipe_fluid_array.link_to_front(
                &mut fhr_pri_loop_pump_9.pipe_fluid_array, 
                ihx_advection_heat_transfer_interaction)
                .unwrap();

            fhr_pri_loop_pump_9.pipe_fluid_array.link_to_front(
                &mut fhr_pipe_8.pipe_fluid_array, 
                ihx_advection_heat_transfer_interaction)
                .unwrap();

            fhr_pipe_8.pipe_fluid_array.link_to_front(
                &mut fhr_pipe_7.pipe_fluid_array, 
                ihx_advection_heat_transfer_interaction)
                .unwrap();

            fhr_pipe_7.pipe_fluid_array.link_to_front(
                &mut ihx_sthe_6.shell_side_fluid_array, 
                ihx_advection_heat_transfer_interaction)
                .unwrap();

            ihx_sthe_6.shell_side_fluid_array.link_to_front(
                &mut fhr_pipe_5.pipe_fluid_array,
                ihx_advection_heat_transfer_interaction)
                .unwrap();

            fhr_pipe_5.pipe_fluid_array.link_to_front(
                &mut fhr_pipe_4.pipe_fluid_array,
                ihx_advection_heat_transfer_interaction)
                .unwrap();
            
            fhr_pipe_4.pipe_fluid_array.link_to_front(
                top_mixing_node_pri_loop, 
                ihx_advection_heat_transfer_interaction)
                .unwrap();
            println!("primary ihx br linking complete");
        }

        // intermediate loop ihx branch 
        {

            bottom_mixing_node_intrmd_loop.link_to_front(
                &mut fhr_pipe_17.pipe_fluid_array, 
                intrmd_loop_ihx_br_heat_transfer_interaction)
                .unwrap();

            ihx_sthe_6.tube_side_fluid_array_for_single_tube.link_to_back(
                &mut fhr_pipe_17.pipe_fluid_array, 
                intrmd_loop_ihx_br_heat_transfer_interaction)
                .unwrap();

            ihx_sthe_6.tube_side_fluid_array_for_single_tube.link_to_front(
                &mut fhr_pipe_12.pipe_fluid_array, 
                intrmd_loop_ihx_br_heat_transfer_interaction)
                .unwrap();

            fhr_pipe_12.pipe_fluid_array.link_to_front(
                top_mixing_node_intrmd_loop, 
                intrmd_loop_ihx_br_heat_transfer_interaction)
                .unwrap();

            println!("intermediate loop ihx br linking complete");
        }

        // intermediate loop steam generator branch
        {

            bottom_mixing_node_intrmd_loop.link_to_front(
                &mut fhr_intrmd_loop_pump_16.pipe_fluid_array, 
                intrmd_loop_steam_gen_br_heat_transfer_interaction)
                .unwrap();

            fhr_intrmd_loop_pump_16.pipe_fluid_array.link_to_front(
                &mut fhr_pipe_15.pipe_fluid_array, 
                intrmd_loop_steam_gen_br_heat_transfer_interaction)
                .unwrap();

            fhr_pipe_15.pipe_fluid_array.link_to_front(
                &mut fhr_steam_generator_shell_side_14.pipe_fluid_array, 
                intrmd_loop_steam_gen_br_heat_transfer_interaction)
                .unwrap();

            fhr_steam_generator_shell_side_14.pipe_fluid_array.link_to_front(
                &mut fhr_pipe_13.pipe_fluid_array, 
                intrmd_loop_steam_gen_br_heat_transfer_interaction)
                .unwrap();

            fhr_pipe_13.pipe_fluid_array.link_to_front(
                top_mixing_node_intrmd_loop, 
                intrmd_loop_steam_gen_br_heat_transfer_interaction)
                .unwrap();
            println!("intermediate loop SteamGen br linking complete");
        }
        // now for the reactor branch, we must have some kind of 
        // power input here 
        {

            // i'll use the lateral link new power vector code 
            //
            // this sets the reactor power in the middle part of the 
            // pipe
            let number_of_temperature_nodes = 5;
            let mut q_frac_arr: Array1<f64> = Array::default(number_of_temperature_nodes);
            // we want the middle node to contain all the power
            q_frac_arr[0] = 0.0;
            q_frac_arr[1] = 0.0;
            q_frac_arr[2] = 1.0;
            q_frac_arr[3] = 0.0;
            q_frac_arr[4] = 0.0;
            
            // now i need to get the fluid array out first 

            let mut reactor_fluid_array_clone: FluidArray = 
                reactor_pipe_1
                .pipe_fluid_array
                .clone()
                .try_into()
                .unwrap();

            reactor_fluid_array_clone
                .lateral_link_new_power_vector(
                    reactor_power, 
                    q_frac_arr)
                .unwrap();

            reactor_pipe_1.pipe_fluid_array = 
                reactor_fluid_array_clone.into();

            // now, add the connections

            reactor_pipe_1.pipe_fluid_array.link_to_front(
                top_mixing_node_pri_loop, 
                reactor_branch_advection_heat_transfer_interaction)
                .unwrap();
            reactor_pipe_1.pipe_fluid_array.link_to_back(
                bottom_mixing_node_pri_loop, 
                reactor_branch_advection_heat_transfer_interaction)
                .unwrap();
            println!("Reactor br linking complete");
        }

        // now we are ready to advance timesteps for all components 
        // and mixing nodes 

        let zero_power = Power::ZERO;
        // for pri loop 
        // I'm not going to add another round of power 
        // because I already added it to the top
        // so i'll just add zero power
        //
        // this is reactor and downcomer branches
        {
            reactor_pipe_1
                .lateral_and_miscellaneous_connections_no_wall_correction(
                reactor_branch_flow, 
                zero_power)
                .unwrap();

            downcomer_pipe_2
                .lateral_and_miscellaneous_connections_no_wall_correction(
                downcomer_branch_1_flow, 
                zero_power)
                .unwrap();

            downcomer_pipe_3
                .lateral_and_miscellaneous_connections_no_wall_correction(
                downcomer_branch_2_flow, 
                zero_power)
                .unwrap();
            println!("lateral linking: reactor and downcomer branches complete");
        }

        // this is the pri loop ihx branch
        // except for the ihx itself
        {

            fhr_pipe_11
                .lateral_and_miscellaneous_connections_no_wall_correction(
                pri_loop_intermediate_heat_exchanger_branch_flow, 
                zero_power)
                .unwrap();
            fhr_pipe_10
                .lateral_and_miscellaneous_connections_no_wall_correction(
                pri_loop_intermediate_heat_exchanger_branch_flow, 
                zero_power)
                .unwrap();
            fhr_pri_loop_pump_9
                .lateral_and_miscellaneous_connections_no_wall_correction(
                pri_loop_intermediate_heat_exchanger_branch_flow, 
                zero_power)
                .unwrap();
            fhr_pipe_8
                .lateral_and_miscellaneous_connections_no_wall_correction(
                pri_loop_intermediate_heat_exchanger_branch_flow, 
                zero_power)
                .unwrap();
            fhr_pipe_7
                .lateral_and_miscellaneous_connections_no_wall_correction(
                pri_loop_intermediate_heat_exchanger_branch_flow, 
                zero_power)
                .unwrap();
            fhr_pipe_5
                .lateral_and_miscellaneous_connections_no_wall_correction(
                pri_loop_intermediate_heat_exchanger_branch_flow, 
                zero_power)
                .unwrap();
            fhr_pipe_4
                .lateral_and_miscellaneous_connections_no_wall_correction(
                pri_loop_intermediate_heat_exchanger_branch_flow, 
                zero_power)
                .unwrap();
            println!("lateral linking: pri loop ihx branches complete");
        }

        // ihx 
        {

            let prandtl_wall_correction_setting = true; 
            let tube_side_total_mass_flowrate = intrmd_loop_ihx_br_flow;
            let shell_side_total_mass_flowrate = pri_loop_intermediate_heat_exchanger_branch_flow;

            ihx_sthe_6.lateral_and_miscellaneous_connections(
                prandtl_wall_correction_setting, 
                tube_side_total_mass_flowrate, 
                shell_side_total_mass_flowrate).unwrap();
            println!("lateral linking: IHX complete");

        }
        // hitec intrmd loop 
        //
        // except for ihx itself
        {
            // ihx branch
            fhr_pipe_17
                .lateral_and_miscellaneous_connections_no_wall_correction(
                intrmd_loop_ihx_br_flow, 
                zero_power)
                .unwrap();
            fhr_pipe_12
                .lateral_and_miscellaneous_connections_no_wall_correction(
                intrmd_loop_ihx_br_flow, 
                zero_power)
                .unwrap();

            // steam gen branch
            fhr_intrmd_loop_pump_16
                .lateral_and_miscellaneous_connections_no_wall_correction(
                intrmd_loop_steam_gen_br_flow, 
                zero_power)
                .unwrap();
            fhr_pipe_15
                .lateral_and_miscellaneous_connections_no_wall_correction(
                intrmd_loop_steam_gen_br_flow, 
                zero_power)
                .unwrap();
            fhr_steam_generator_shell_side_14
                .lateral_and_miscellaneous_connections_no_wall_correction(
                intrmd_loop_steam_gen_br_flow, 
                zero_power)
                .unwrap();
            fhr_pipe_13
                .lateral_and_miscellaneous_connections_no_wall_correction(
                intrmd_loop_steam_gen_br_flow, 
                zero_power)
                .unwrap();
            println!("lateral linking: intermediate loop IHX Branch complete");
        }

        // timestep advance for all heat transfer entities
        {
            // pri loop (with ihx)
            reactor_pipe_1
                .advance_timestep(timestep)
                .unwrap();
            downcomer_pipe_2
                .advance_timestep(timestep)
                .unwrap();
            downcomer_pipe_3
                .advance_timestep(timestep)
                .unwrap();

            println!("Advance timestep: Reactor and Downcomer branches complete");

            fhr_pipe_4
                .advance_timestep(timestep)
                .unwrap();
            fhr_pipe_5
                .advance_timestep(timestep)
                .unwrap();
            fhr_pipe_7
                .advance_timestep(timestep)
                .unwrap();
            fhr_pipe_8
                .advance_timestep(timestep)
                .unwrap();
            fhr_pri_loop_pump_9
                .advance_timestep(timestep)
                .unwrap();
            fhr_pipe_10
                .advance_timestep(timestep)
                .unwrap();
            fhr_pipe_11
                .advance_timestep(timestep)
                .unwrap();
            println!("Advance timestep: pri loop complete except for IHX");

            // intermediate branch (less ihx)
            fhr_pipe_12
                .advance_timestep(timestep)
                .unwrap();
            fhr_pipe_17
                .advance_timestep(timestep)
                .unwrap();
            println!("Advance timestep: intermediate loop IHX Branch complete");
            fhr_pipe_13
                .advance_timestep(timestep)
                .unwrap();
            fhr_steam_generator_shell_side_14
                .advance_timestep(timestep)
                .unwrap();
            fhr_pipe_15
                .advance_timestep(timestep)
                .unwrap();
            fhr_intrmd_loop_pump_16
                .advance_timestep(timestep)
                .unwrap();
            println!("Advance timestep: intermediate loop SG Branch complete");

            // all mixing nodes
            top_mixing_node_pri_loop
                .advance_timestep_mut_self(timestep)
                .unwrap();
            bottom_mixing_node_pri_loop
                .advance_timestep_mut_self(timestep)
                .unwrap();
            top_mixing_node_intrmd_loop
                .advance_timestep_mut_self(timestep)
                .unwrap();
            bottom_mixing_node_intrmd_loop
                .advance_timestep_mut_self(timestep)
                .unwrap();
            println!("Advance timestep: Mixing Nodes complete");

            ihx_sthe_6
                .advance_timestep(timestep)
                .unwrap();
            println!("Advance timestep: IHX complete");
        }

        // now I want reactor temperature profile 
        let reactor_temp_profile: Vec<ThermodynamicTemperature> = 
            reactor_pipe_1
            .pipe_fluid_array_temperature()
            .unwrap();
        let reactor_temp_profile_degc: Vec<f64> = 
            reactor_temp_profile
            .into_iter()
            .map(|temperature|{
                (temperature.get::<degree_celsius>()*100.0).round()/100.0
            })
            .collect();

        let ihx_shell_side_temp_profile: Vec<ThermodynamicTemperature> = 
            ihx_sthe_6 
            .shell_side_fluid_array_temperature()
            .unwrap();

        let ihx_shell_side_temp_profile_degc: Vec<f64> = 
            ihx_shell_side_temp_profile
            .into_iter()
            .map(|temperature|{
                (temperature.get::<degree_celsius>()*100.0).round()/100.0
            })
            .collect();

        
        let fhr_state = FHRState {
            reactor_branch_flow,
            downcomer_branch_1_flow,
            downcomer_branch_2_flow,
            intermediate_heat_exchanger_branch_flow: pri_loop_intermediate_heat_exchanger_branch_flow,
            intrmd_loop_ihx_br_flow,
            intrmd_loop_steam_gen_br_flow,
            simulation_time,
            reactor_temp_profile_degc,
            ihx_shell_side_temp_profile_degc,
        };
        dbg!(&fhr_state);
        return fhr_state;
}

#[derive(Debug,Clone)]
pub(crate) struct FHRState {
    /// reactor branch flow (upwards through the core)
    /// note that positive flow means from bottom mixing node to top
    pub reactor_branch_flow: MassRate,
    /// downcomer 1 branch flow (upwards through the core)
    /// note that positive flow means from bottom mixing node to top
    pub downcomer_branch_1_flow: MassRate,
    /// downcomer 2 branch flow (upwards through the core)
    /// note that positive flow means from bottom mixing node to top
    pub downcomer_branch_2_flow: MassRate,
    /// ihx branch flow 
    /// note that positive flow means from bottom mixing node to top
    pub intermediate_heat_exchanger_branch_flow: MassRate,
    /// ihx branch flow 
    /// note that positive flow means from bottom 
    /// (between pipe 17 and pump 16) 
    /// to top
    /// (between pipe 12 and pipe 13)
    pub intrmd_loop_ihx_br_flow: MassRate,
    /// steam generator branch
    /// note that positive flow means from bottom 
    /// (between pipe 17 and pump 16) 
    /// to top
    /// (between pipe 12 and pipe 13)
    pub intrmd_loop_steam_gen_br_flow: MassRate,

    // other diagnostics 
    /// shows the current simulation time
    pub simulation_time: Time,

    // temperature diagnostics 
    /// shows the current reactor temperature profile in degc (2dp)
    pub reactor_temp_profile_degc: Vec<f64>,
    /// shows the current reactor temperature profile in degc (2dp)
    pub ihx_shell_side_temp_profile_degc: Vec<f64>,


}

/// for the gFHR primary loop, and intermediate loop 
/// there are four branches that need to be solved for flowrate 
///
/// this code handles the solution procedure
/// using the tuas_boussinesq_solver library code
///
/// and only handles fluid mechanics (isothermal)
pub fn four_branch_pri_and_intermediate_loop_fluid_mechanics_only(
    pri_loop_pump_pressure: Pressure,
    intrmd_loop_pump_pressure: Pressure,
    // reactor branch
    reactor_pipe_1: &InsulatedFluidComponent,
    // downcomer branch 1
    downcomer_pipe_2: &InsulatedFluidComponent,
    // downcomer branch 2
    downcomer_pipe_3: &InsulatedFluidComponent,
    // Intermediate heat exchanger branch in pri loop
    fhr_pipe_11: &InsulatedFluidComponent,
    fhr_pipe_10: &InsulatedFluidComponent,
    fhr_pri_loop_pump_9: &NonInsulatedFluidComponent,
    fhr_pipe_8: &InsulatedFluidComponent,
    fhr_pipe_7: &InsulatedFluidComponent,
    ihx_sthe_6: &SimpleShellAndTubeHeatExchanger,
    fhr_pipe_5: &InsulatedFluidComponent,
    fhr_pipe_4: &InsulatedFluidComponent,
    // intermediate loop ihx side
    fhr_pipe_17: &InsulatedFluidComponent,
    fhr_pipe_12: &InsulatedFluidComponent,
    // intermediate loop steam generator side
    fhr_intrmd_loop_pump_16: &NonInsulatedFluidComponent,
    fhr_pipe_15: &InsulatedFluidComponent,
    fhr_steam_generator_shell_side_14: &NonInsulatedFluidComponent,
    fhr_pipe_13: &InsulatedFluidComponent,

    ) -> (MassRate, MassRate, MassRate, MassRate, MassRate, MassRate){

        // pri loop

        let mut reactor_branch = 
            FluidComponentCollection::new_series_component_collection();

        reactor_branch.clone_and_add_component(reactor_pipe_1);




        let mut pri_downcomer_branch_1 = 
            FluidComponentCollection::new_series_component_collection();

        pri_downcomer_branch_1.clone_and_add_component(downcomer_pipe_2);




        let mut pri_downcomer_branch_2 = 
            FluidComponentCollection::new_series_component_collection();

        pri_downcomer_branch_2.clone_and_add_component(downcomer_pipe_3);




        let mut pri_loop_intermediate_heat_exchanger_branch =
            FluidComponentCollection::new_series_component_collection();

        let mut fhr_pri_loop_pump_9_with_pressure_set = fhr_pri_loop_pump_9.clone();
        fhr_pri_loop_pump_9_with_pressure_set.set_internal_pressure_source(pri_loop_pump_pressure);
        let ihx_shell_side_6_clone = ihx_sthe_6.get_clone_of_shell_side_fluid_component();

        pri_loop_intermediate_heat_exchanger_branch.clone_and_add_component(
            fhr_pipe_11);
        pri_loop_intermediate_heat_exchanger_branch.clone_and_add_component(
            fhr_pipe_10);
        pri_loop_intermediate_heat_exchanger_branch.clone_and_add_component(
            &fhr_pri_loop_pump_9_with_pressure_set);
        pri_loop_intermediate_heat_exchanger_branch.clone_and_add_component(
            fhr_pipe_8);
        pri_loop_intermediate_heat_exchanger_branch.clone_and_add_component(
            fhr_pipe_7);
        pri_loop_intermediate_heat_exchanger_branch.clone_and_add_component(
            &ihx_shell_side_6_clone);
        pri_loop_intermediate_heat_exchanger_branch.clone_and_add_component(
            fhr_pipe_5);
        pri_loop_intermediate_heat_exchanger_branch.clone_and_add_component(
            fhr_pipe_4);




        // intermediate loop
        // ihx side

        let mut intrmd_loop_intermediate_heat_exchanger_branch =
            FluidComponentCollection::new_series_component_collection();

        intrmd_loop_intermediate_heat_exchanger_branch.clone_and_add_component(
            fhr_pipe_17);
        intrmd_loop_intermediate_heat_exchanger_branch.clone_and_add_component(
            &ihx_sthe_6.get_clone_of_tube_side_parallel_tube_fluid_component());
        intrmd_loop_intermediate_heat_exchanger_branch.clone_and_add_component(
            fhr_pipe_12);

        // intermediate loop
        // steam generator side

        let mut intrmd_loop_steam_generator_branch =
            FluidComponentCollection::new_series_component_collection();

        let mut fhr_intrmd_loop_pump_16_with_pressure_set = 
            fhr_intrmd_loop_pump_16.clone();
        fhr_intrmd_loop_pump_16_with_pressure_set
            .set_internal_pressure_source(intrmd_loop_pump_pressure);
        intrmd_loop_steam_generator_branch.clone_and_add_component(
            &fhr_intrmd_loop_pump_16_with_pressure_set);
        intrmd_loop_steam_generator_branch.clone_and_add_component(
            fhr_pipe_15);
        intrmd_loop_steam_generator_branch.clone_and_add_component(
            fhr_steam_generator_shell_side_14);
        intrmd_loop_steam_generator_branch.clone_and_add_component(
            fhr_pipe_13);


        // calculate pri loop side fluid mechanics
        let mut pri_loop_branches = 
            FluidComponentSuperCollection::default();

        pri_loop_branches.set_orientation_to_parallel();

        pri_loop_branches.fluid_component_super_vector.push(reactor_branch);
        pri_loop_branches.fluid_component_super_vector.push(pri_downcomer_branch_1);
        pri_loop_branches.fluid_component_super_vector.push(pri_downcomer_branch_2);

        pri_loop_branches.fluid_component_super_vector.push(pri_loop_intermediate_heat_exchanger_branch);

        let pressure_change_across_each_branch_pri_loop = 
            pri_loop_branches.get_pressure_change(MassRate::ZERO);


        let pri_loop_mass_rate_vector 
            = pri_loop_branches.get_mass_flowrate_across_each_parallel_branch(
                pressure_change_across_each_branch_pri_loop);

        let (reactor_branch_flow, downcomer_branch_1_flow,
            downcomer_branch_2_flow, intermediate_heat_exchanger_branch_flow)
            = (pri_loop_mass_rate_vector[0], pri_loop_mass_rate_vector[1],
                pri_loop_mass_rate_vector[2], pri_loop_mass_rate_vector[3]);

        // calculate intermediate loop side fluid mechanics

        let mut intrmd_loop_branches = 
            FluidComponentSuperCollection::default();

        intrmd_loop_branches.set_orientation_to_parallel();

        intrmd_loop_branches.fluid_component_super_vector.push(
            intrmd_loop_intermediate_heat_exchanger_branch);
        intrmd_loop_branches.fluid_component_super_vector.push(
            intrmd_loop_steam_generator_branch);

        let pressure_change_across_each_branch_intrmd_loop = 
            intrmd_loop_branches.get_pressure_change(MassRate::ZERO);

        let intrmd_loop_mass_rate_vector 
            = intrmd_loop_branches.get_mass_flowrate_across_each_parallel_branch(
                pressure_change_across_each_branch_intrmd_loop);
        let (intrmd_loop_ihx_br_flow, intrmd_loop_steam_gen_br_flow) = 
            (intrmd_loop_mass_rate_vector[0],
             intrmd_loop_mass_rate_vector[1]);


        return (reactor_branch_flow, downcomer_branch_1_flow,
            downcomer_branch_2_flow, intermediate_heat_exchanger_branch_flow,
            intrmd_loop_ihx_br_flow, intrmd_loop_steam_gen_br_flow);
}
/// for the gFHR primary loop,
/// there are four branches that need to be solved for flowrate 
///
/// this code handles the solution procedure
/// using custom code
pub fn four_branch_pri_loop_flowrates_parallel_debug(
    pump_pressure: Pressure,
    // reactor branch
    reactor_pipe_1: &InsulatedFluidComponent,
    // downcomer branch 1
    downcomer_pipe_2: &InsulatedFluidComponent,
    // downcomer branch 2
    downcomer_pipe_3: &InsulatedFluidComponent,
    // Intermediate heat exchanger branch 
    fhr_pipe_7: &InsulatedFluidComponent,
    _fhr_pri_loop_pump: &NonInsulatedFluidComponent
) -> (MassRate, MassRate, MassRate, MassRate,){

    // note: this crashes due to non convergency issues...
    //thread '<unnamed>' panicked at C:\Users\fifad\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tuas_boussinesq_solver-0.0.7\src\lib\array_control_vol_an
    //d_fluid_component_collections\fluid_component_collection\collection_series_and_parallel_functions.rs:444:74:
    //called `Result::unwrap()` on an `Err` value: NoConvergency
    //note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //
    //
    //now, even by having more flowrate options, I'm still getting a no 
    //convergency error especially once pump pressure exceeds 
    // 10.0 - 13.0 Pa, and there is actually flowrate
    // I'm getting flowrates in excess of 10 kg/s 
    // 554 kg/s, and thats okay 
    //
    // but then I get NoConvergency errors

    let mut reactor_branch = 
        FluidComponentCollection::new_series_component_collection();

    reactor_branch.clone_and_add_component(reactor_pipe_1);




    let mut downcomer_branch_1 = 
        FluidComponentCollection::new_series_component_collection();

    downcomer_branch_1.clone_and_add_component(downcomer_pipe_2);




    let mut downcomer_branch_2 = 
        FluidComponentCollection::new_series_component_collection();

    downcomer_branch_2.clone_and_add_component(downcomer_pipe_3);




    let mut intermediate_heat_exchanger_branch =
        FluidComponentCollection::new_series_component_collection();

    let mut fhr_pipe_4_clone = fhr_pipe_7.clone();
    fhr_pipe_4_clone.set_internal_pressure_source(pump_pressure);
    intermediate_heat_exchanger_branch.clone_and_add_component(&fhr_pipe_4_clone);
    //let mut fhr_pump_clone: NonInsulatedFluidComponent 
    //    = fhr_pri_loop_pump.clone();
    //fhr_pump_clone.set_internal_pressure_source(pump_pressure);
    //intermediate_heat_exchanger_branch.clone_and_add_component(&fhr_pump_clone);




    let mut pri_loop_branches = 
        FluidComponentSuperCollection::default();

    pri_loop_branches.set_orientation_to_parallel();

    pri_loop_branches.fluid_component_super_vector.push(reactor_branch);
    pri_loop_branches.fluid_component_super_vector.push(downcomer_branch_1);
    pri_loop_branches.fluid_component_super_vector.push(downcomer_branch_2);
    pri_loop_branches.fluid_component_super_vector.push(intermediate_heat_exchanger_branch);

    let (reactor_branch_flow, downcomer_branch_1_flow,
        downcomer_branch_2_flow, intermediate_heat_exchanger_branch_flow)
        = calculate_iterative_mass_flowrate_across_branches_for_fhr_sim_v1(
            &pri_loop_branches);


    return (reactor_branch_flow, downcomer_branch_1_flow,
        downcomer_branch_2_flow, intermediate_heat_exchanger_branch_flow);
}
/// for the gFHR primary loop,
/// there are four branches that need to be solved for flowrate 
///
/// this code handles the solution procedure
/// using the tuas_boussinesq_solver library code
///
/// I have tested that even with the change in the code 
/// that all regression tests still pass: 
/// 
/// took 40 mins on my aftershock desktop
/// note that the coupled 
pub fn four_branch_pri_loop_flowrates_parallel_debug_library(
    pump_pressure: Pressure,
    // reactor branch
    reactor_pipe_1: &InsulatedFluidComponent,
    // downcomer branch 1
    downcomer_pipe_2: &InsulatedFluidComponent,
    // downcomer branch 2
    downcomer_pipe_3: &InsulatedFluidComponent,
    // Intermediate heat exchanger branch 
    fhr_pipe_7: &InsulatedFluidComponent,
    _fhr_pri_loop_pump: &NonInsulatedFluidComponent
) -> (MassRate, MassRate, MassRate, MassRate,){

    // note: this crashes due to non convergency issues...
    //thread '<unnamed>' panicked at C:\Users\fifad\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tuas_boussinesq_solver-0.0.7\src\lib\array_control_vol_an
    //d_fluid_component_collections\fluid_component_collection\collection_series_and_parallel_functions.rs:444:74:
    //called `Result::unwrap()` on an `Err` value: NoConvergency
    //note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //
    //
    //now, even by having more flowrate options, I'm still getting a no 
    //convergency error especially once pump pressure exceeds 
    // 10.0 - 13.0 Pa, and there is actually flowrate
    // I'm getting flowrates in excess of 10 kg/s 
    // 554 kg/s, and thats okay 
    //
    // but then I get NoConvergency errors

    let mut reactor_branch = 
        FluidComponentCollection::new_series_component_collection();

    reactor_branch.clone_and_add_component(reactor_pipe_1);




    let mut downcomer_branch_1 = 
        FluidComponentCollection::new_series_component_collection();

    downcomer_branch_1.clone_and_add_component(downcomer_pipe_2);




    let mut downcomer_branch_2 = 
        FluidComponentCollection::new_series_component_collection();

    downcomer_branch_2.clone_and_add_component(downcomer_pipe_3);




    let mut intermediate_heat_exchanger_branch =
        FluidComponentCollection::new_series_component_collection();

    let mut fhr_pipe_4_clone = fhr_pipe_7.clone();
    fhr_pipe_4_clone.set_internal_pressure_source(pump_pressure);
    intermediate_heat_exchanger_branch.clone_and_add_component(&fhr_pipe_4_clone);
    //let mut fhr_pump_clone: NonInsulatedFluidComponent 
    //    = fhr_pri_loop_pump.clone();
    //fhr_pump_clone.set_internal_pressure_source(pump_pressure);
    //intermediate_heat_exchanger_branch.clone_and_add_component(&fhr_pump_clone);




    let mut pri_loop_branches = 
        FluidComponentSuperCollection::default();

    pri_loop_branches.set_orientation_to_parallel();

    pri_loop_branches.fluid_component_super_vector.push(reactor_branch);
    pri_loop_branches.fluid_component_super_vector.push(downcomer_branch_1);
    pri_loop_branches.fluid_component_super_vector.push(downcomer_branch_2);
    pri_loop_branches.fluid_component_super_vector.push(intermediate_heat_exchanger_branch);

    let pressure_change_across_each_branch = 
        pri_loop_branches.get_pressure_change(MassRate::ZERO);


    let mass_rate_vector 
        = pri_loop_branches.get_mass_flowrate_across_each_parallel_branch(
            pressure_change_across_each_branch);

    let (reactor_branch_flow, downcomer_branch_1_flow,
        downcomer_branch_2_flow, intermediate_heat_exchanger_branch_flow)
        = (mass_rate_vector[0], mass_rate_vector[1],
            mass_rate_vector[2], mass_rate_vector[3]);

    return (reactor_branch_flow, downcomer_branch_1_flow,
        downcomer_branch_2_flow, intermediate_heat_exchanger_branch_flow);
}



// debug log: 
// 
// thought it's a fluid properties bug
// tried changing from FLiBe to HITEC but not working
//
// basically from what I observed, the mass flowrate solvers 
// do get stuck at some value and yet do not converge there...
//
// I believe the tolerance is an issue... 
// so upon reducing tolerance, the mass flowrates are able to solve and  
// converge!
// but this produces wonky results
//
/// calculates pressure change given a mass
/// flowrate through a parallel collection of
/// fluid pipes or components
///
/// TODO: needs work and testing, doesn't work now
pub fn calculate_iterative_mass_flowrate_across_branches_for_fhr_sim_v1(
    fluid_component_super_collection: 
    &FluidComponentSuperCollection) -> (MassRate,
    MassRate, MassRate, MassRate) {

        let mass_flowrate = MassRate::ZERO;
        let fluid_component_collection_vector = 
            fluid_component_super_collection.get_immutable_vector();

        // for calculating pressure change in a parallel super
        // collection from
        // mass flowrate, 
        // i will need to iteratively guess the pressure change
        // across each pipe to get the specified mass flowrate

        // only thing is how do i do so?
        //
        // First thing first, I will need to guess some bounds for the brent
        // calculator, ie what pressure change bounds are appropriate?
        //
        // There are no standardised pressure change bounds for any of
        // these
        //
        // Nevertheless, they can be calculated,
        //
        // For reference, at zero mass flowrate, each parallel branch would
        // have a default pressure change. This may differ for each
        // branch. 
        //
        // taking the average of these pressure changes at zero flow case
        // i would get a pretty good guess of what the pressure change may
        // be like at zero flow
        //
        // this will then be my starting point and if i bound it by
        // the change between maximum and minimum pressure,
        // i should be able to get my bounds for zero flow
        // this case is simpler
        //
        //
        //
        //
        //
        // And then, when I supply a mass flowrate for each of these branches
        // there would be some pressure losses associated with that
        // mass flowrate
        // Again, the pressure losses expected from each branch would
        // be different
        //
        // since i supply a mass flowrate here already, I can use this
        // combined mass flowrate through all pipes
        //
        // the minimum pressure loss from any one of these branches
        // and subtract that from the maximum pressure loss
        //
        //
        //
        // This will form a pressure bound which i can plus and minus
        // minus from my default pressure change
        // 
        // Lastly, I need to add the difference between the maximum
        // and minimum of the pressure change at zero flow
        // perhaps multiply that by 1.5 to obtain pressure bounds as
        // well
        //
        // In this way, both flows due to pressure changes outside the      
        // parallel branches
        // and changes inside the parallel branches are accounted for
        //
        // in dynamic setting of bounds. 
        // and this should provide decent-ish initial guesses
        //

        // if mass flowrate over this series is zero, then we can calculate the bound
        // straightaway

        let user_requested_mass_flowrate = 
            mass_flowrate;

        let zero_mass_flowrate = 
            MassRate::new::<kilogram_per_second>(0.0);

        // if the mass flowrate is almost zero (1e-9 kg/s)
        // we assume flow is zero 
        // this is zero NET flow through the parallel structure
        // the branches themselves may still have flow going 
        // through them
        if user_requested_mass_flowrate.value.abs() < 1e-9_f64 {

            dbg!("zero flowrate through parallel branches");
            // in this case, the average mass flowrate through each of these
            // loops is very close to zero,
            //
            //
            // for a trivial solution zero flowrate is supplied
            // as a guess
            //
            // That is we have zero mass flowrate through the network 
            // of branches, 
            //
            // the easiest solution is each branch has zero mass flowrate


            // however, more often than not, the trivial solution doesn't work
            // I then need to obtain the largest difference in pressure changes 
            // between each branch if it has zero flow rate
            // we can get the max pressure difference between each branch 
            //
            let max_pressure_change_between_branches: Pressure = 
                <FluidComponentSuperCollection as FluidComponentSuperCollectionParallelAssociatedFunctions>:: 
                calculate_maximum_pressure_difference_between_branches(
                    zero_mass_flowrate, 
                    &fluid_component_collection_vector
                );


            // with this max pressure change, we can guess a maximum 
            // flowrate across each branch
            dbg!(&max_pressure_change_between_branches);
            dbg!("calculating max flow between brances");

            let debugging = true;
            // TODO: this is a temp fix
            let mut max_mass_flowrate_across_each_branch = 
                MassRate::new::<kilogram_per_second>(5000.0);
            // TODO: this is the buggy spot
            if !debugging {
                max_mass_flowrate_across_each_branch = 
                    <FluidComponentSuperCollection as FluidComponentSuperCollectionParallelAssociatedFunctions>:: 
                    calculate_maximum_mass_flowrate_given_pressure_drop_across_each_branch(
                        max_pressure_change_between_branches, 
                        &fluid_component_collection_vector);
            }
            // the above is an algorithm which allows us to calculate an 
            // upper limit for mass flowrate across each branch
            //
            // this may fail (or not) given a pressure bound 
            //
            // to avoid this issue, i can comment this out and set 
            // an upper limit of 5000 kg/s 




            // with a hypothetical mass flowrate across each branch 
            //
            // now we need to change the limits of the pressure change 
            // instead of +/- 10 kg/s to something larger,
            // say 100,000 kg/s
            //
            // I remember that +/- 10 kg/s is for CIET
            // but for FHR, the value is much larger. 
            // perhaps 100,000 kg/s is sufficient
            //
            // this is giving me problems here!
            // calculate_pressure_change_using_guessed_branch_mass_flowrate
            //

            dbg!("calculating pressure chg through branches..");
            // now with pressure change through the branches... 
            // I'm getting an oscillation issue
            // we do get to about zero 
            // -5.68e-13 and 7.105e-13 
            //
            // this is indeed about zero but not quite
            // I think for large mass flowrates, the tolerance is too tight
            //
            // I think normalisation would work. 
            let pressure_change = 
                calculate_pressure_change_using_guessed_branch_mass_flowrate_fhr_sim_v1_custom(
                    max_mass_flowrate_across_each_branch, 
                    user_requested_mass_flowrate, 
                    &fluid_component_collection_vector);

            dbg!("pressure chg calculated");
            dbg!(&pressure_change);


            let mut mass_flowrate_vector: Vec<MassRate> =
                vec![];

            // the mass_flowrate vector will have a length
            // equal to the fluid_component vector

            let new_vector_length =
                fluid_component_collection_vector.len();

            let default_mass_flowrate_value = 
                MassRate::new::<kilogram_per_second>(0.0);

            mass_flowrate_vector.resize(
                new_vector_length,
                default_mass_flowrate_value
            );

            for (index,fluid_component_pointer_collection) in 
                fluid_component_collection_vector.iter().enumerate() {

                    // first we get an immutable reference from
                    // the mutable reference

                    let fluid_component_collection = 
                        &*fluid_component_pointer_collection;



                    dbg!("calculating mass flowrate for...");
                    dbg!(&fluid_component_collection);

                    // let me get the vector of fluid component 
                    // for single branch first 
                    let fluid_component_vector = 
                        fluid_component_collection
                        .get_immutable_fluid_component_vector();
                    let fluid_component_mass_flowrate: MassRate;

                    if !debugging {
                        fluid_component_mass_flowrate = 
                            fluid_component_collection.get_mass_flowrate_from_pressure_change(
                                pressure_change);
                    } else {
                        fluid_component_mass_flowrate = 
                            calculate_mass_flowrate_from_pressure_change_for_single_branch_fhr_sim_custom(
                                pressure_change, 
                                &fluid_component_vector);

                    };

                    dbg!(&fluid_component_mass_flowrate);

                    mass_flowrate_vector[index] = 
                        fluid_component_mass_flowrate;

                }

            // for fhr, sim specifically, we have 4 branches

            return (mass_flowrate_vector[0],
                mass_flowrate_vector[1],
                mass_flowrate_vector[2],
                mass_flowrate_vector[3]);
        }


        // for any other flowrate cases, we are not debugging here 
        // so I will leave this blank

        todo!();

    }


