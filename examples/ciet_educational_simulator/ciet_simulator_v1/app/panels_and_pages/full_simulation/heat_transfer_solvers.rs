
use tuas_boussinesq_solver::prelude::beta_testing::InsulatedPorousMediaFluidComponent;
// first, copy and paste the dracs loop functions over 
//
use uom::si::f64::*;

// let's construct the branches with test pressures and obtain 
use tuas_boussinesq_solver::pre_built_components::shell_and_tube_heat_exchanger::SimpleShellAndTubeHeatExchanger;
use tuas_boussinesq_solver::prelude::beta_testing::HeatTransferEntity;
use uom::ConstZero;

use uom::si::thermodynamic_temperature::degree_celsius;

use tuas_boussinesq_solver::pre_built_components::
insulated_pipes_and_fluid_components::InsulatedFluidComponent;
use tuas_boussinesq_solver::pre_built_components::
non_insulated_fluid_components::NonInsulatedFluidComponent;

use tuas_boussinesq_solver::boussinesq_thermophysical_properties::LiquidMaterial;
use tuas_boussinesq_solver::heat_transfer_correlations::heat_transfer_interactions::
heat_transfer_interaction_enums::HeatTransferInteractionType;

/// heat transfer for pri loop, all three branch flowrates 
/// required 

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
///
/// todo: conduction between branches
pub fn ciet_pri_loop_three_branch_link_up_components_ver_4(
    dhx_flow: MassRate,
    heater_flow: MassRate,
    ctah_flow: MassRate,
    heat_rate_through_heater: Power,
    average_temperature_for_density_calcs: ThermodynamicTemperature,
    ambient_htc: HeatTransfer,
    ctah_heat_transfer_coeff: HeatTransfer,
    pipe_4: &mut InsulatedFluidComponent,
    pipe_3: &mut InsulatedFluidComponent,
    pipe_2a: &mut InsulatedFluidComponent,
    static_mixer_10_label_2: &mut InsulatedFluidComponent,
    heater_top_head_1a: &mut InsulatedFluidComponent,
    heater_version1_1: &mut InsulatedPorousMediaFluidComponent,
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
    pipe_5b: &mut InsulatedFluidComponent,
    static_mixer_41_label_6 :&mut InsulatedFluidComponent,
    pipe_6a :&mut InsulatedFluidComponent,
    ctah_vertical_label_7a :&mut NonInsulatedFluidComponent,
    ctah_horizontal_label_7b :&mut NonInsulatedFluidComponent,
    pipe_8a :&mut InsulatedFluidComponent,
    static_mixer_40_label_8 :&mut InsulatedFluidComponent,
    pipe_9 :&mut InsulatedFluidComponent,
    pipe_10 :&mut InsulatedFluidComponent,
    pipe_11 :&mut InsulatedFluidComponent,
    pipe_12 :&mut InsulatedFluidComponent,
    ctah_pump :&mut NonInsulatedFluidComponent,
    pipe_13 : &mut InsulatedFluidComponent,
    pipe_14 : &mut InsulatedFluidComponent,
    flowmeter_40_14a :&mut NonInsulatedFluidComponent,
    pipe_15 :&mut InsulatedFluidComponent,
    pipe_16 :&mut InsulatedFluidComponent,
    pipe_17a :&mut InsulatedFluidComponent,
    top_mixing_node_5a_5b_4: &mut HeatTransferEntity,
    bottom_mixing_node_17a_17b_18: &mut HeatTransferEntity,
    ){

        // create the heat transfer interaction 
        let dhx_advection_heat_transfer_interaction: HeatTransferInteractionType;

        // I'm going to create the advection interaction
        //
        // and probably for the sake of density calcs, I'll take the 
        // average density using DHX outlet and 
        // TCHX outlet temperatures, average them for the whole loop 
        // doesn't make much diff tho based on Boussinesq approximation

        let average_therminol_density = 
            LiquidMaterial::TherminolVP1.try_get_density(
                average_temperature_for_density_calcs).unwrap();

        dhx_advection_heat_transfer_interaction = 
            HeatTransferInteractionType::
            new_advection_interaction(dhx_flow, 
                average_therminol_density, 
                average_therminol_density);

        let heater_advection_heat_transfer_interaction = 
            HeatTransferInteractionType::
            new_advection_interaction(heater_flow, 
                average_therminol_density, 
                average_therminol_density);

        let ctah_advection_heat_transfer_interaction = 
            HeatTransferInteractionType::
            new_advection_interaction(ctah_flow, 
                average_therminol_density, 
                average_therminol_density);

        // now, let's link the fluid arrays using advection 
        // (no conduction here axially between arrays)
        //
        // note that by default, flow will always go downwards for the 
        // DHX so components should be linked in a counter clockwise fashion
        {
            // first is flow from heater branch to DHX branch
            //
            // note that trying to put a mixing node here is 
            // problematic. I have to find out why...

            top_mixing_node_5a_5b_4.link_to_front(
                &mut pipe_5a.pipe_fluid_array, 
                dhx_advection_heat_transfer_interaction)
                .unwrap();

            // then flow downwards in DHX branch

            pipe_5a.pipe_fluid_array.link_to_front(
                &mut pipe_26.pipe_fluid_array, 
                dhx_advection_heat_transfer_interaction)
                .unwrap();

            pipe_26.pipe_fluid_array.link_to_front(
                &mut pipe_25a.pipe_fluid_array, 
                dhx_advection_heat_transfer_interaction)
                .unwrap();

            pipe_25a.pipe_fluid_array.link_to_front(
                &mut static_mixer_21_label_25.pipe_fluid_array, 
                dhx_advection_heat_transfer_interaction)
                .unwrap();

            //note: for shell side fluid array, linking normally is okay 
            //because there is only one entity
            //
            // for tube side fluid array, link normally as well, because 
            // the advance timestep portion takes care of the parallel 
            // tube treatment

            static_mixer_21_label_25.pipe_fluid_array.link_to_front(
                &mut dhx_sthe.shell_side_fluid_array, 
                dhx_advection_heat_transfer_interaction)
                .unwrap();

            // for dhx, the flow convention in both shell and tube is 
            // from top to bottom of the branch

            dhx_sthe.shell_side_fluid_array.link_to_front(
                &mut static_mixer_20_label_23.pipe_fluid_array, 
                dhx_advection_heat_transfer_interaction)
                .unwrap();

            static_mixer_20_label_23.pipe_fluid_array.link_to_front(
                &mut pipe_23a.pipe_fluid_array, 
                dhx_advection_heat_transfer_interaction)
                .unwrap();

            pipe_23a.pipe_fluid_array.link_to_front(
                &mut pipe_22.pipe_fluid_array, 
                dhx_advection_heat_transfer_interaction)
                .unwrap();

            pipe_22.pipe_fluid_array.link_to_front(
                &mut flowmeter_20_21a.pipe_fluid_array, 
                dhx_advection_heat_transfer_interaction)
                .unwrap();

            flowmeter_20_21a.pipe_fluid_array.link_to_front(
                &mut pipe_21.pipe_fluid_array, 
                dhx_advection_heat_transfer_interaction)
                .unwrap();

            pipe_21.pipe_fluid_array.link_to_front(
                &mut pipe_20.pipe_fluid_array, 
                dhx_advection_heat_transfer_interaction)
                .unwrap();


            pipe_20.pipe_fluid_array.link_to_front(
                &mut pipe_19.pipe_fluid_array, 
                dhx_advection_heat_transfer_interaction)
                .unwrap();

            pipe_19.pipe_fluid_array.link_to_front(
                &mut pipe_17b.pipe_fluid_array, 
                dhx_advection_heat_transfer_interaction)
                .unwrap();

            pipe_17b.pipe_fluid_array.link_to_front(
                bottom_mixing_node_17a_17b_18, 
                dhx_advection_heat_transfer_interaction)
                .unwrap();

            // heater branch

            top_mixing_node_5a_5b_4.link_to_front(
                &mut pipe_4.pipe_fluid_array, 
                heater_advection_heat_transfer_interaction)
                .unwrap();

            pipe_4.pipe_fluid_array.link_to_front(
                &mut pipe_3.pipe_fluid_array, 
                heater_advection_heat_transfer_interaction)
                .unwrap();

            pipe_3.pipe_fluid_array.link_to_front(
                &mut pipe_2a.pipe_fluid_array, 
                heater_advection_heat_transfer_interaction)
                .unwrap();

            pipe_2a.pipe_fluid_array.link_to_front(
                &mut static_mixer_10_label_2.pipe_fluid_array, 
                heater_advection_heat_transfer_interaction)
                .unwrap();

            static_mixer_10_label_2.pipe_fluid_array.link_to_front(
                &mut heater_top_head_1a.pipe_fluid_array, 
                heater_advection_heat_transfer_interaction)
                .unwrap();


            heater_top_head_1a.pipe_fluid_array.link_to_front(
                &mut heater_version1_1.pipe_fluid_array, 
                heater_advection_heat_transfer_interaction)
                .unwrap();

            heater_version1_1.pipe_fluid_array.link_to_front(
                &mut heater_bottom_head_1b.pipe_fluid_array, 
                heater_advection_heat_transfer_interaction)
                .unwrap();


            heater_bottom_head_1b.pipe_fluid_array.link_to_front(
                &mut pipe_18.pipe_fluid_array, 
                heater_advection_heat_transfer_interaction)
                .unwrap();

            pipe_18.pipe_fluid_array.link_to_front(
                bottom_mixing_node_17a_17b_18, 
                heater_advection_heat_transfer_interaction)
                .unwrap();


            // ctah branch 
            top_mixing_node_5a_5b_4.link_to_front(
                &mut pipe_5b.pipe_fluid_array, 
                ctah_advection_heat_transfer_interaction)
                .unwrap();

            pipe_5b.pipe_fluid_array.link_to_front(
                &mut static_mixer_41_label_6.pipe_fluid_array, 
                ctah_advection_heat_transfer_interaction)
                .unwrap();

            static_mixer_41_label_6.pipe_fluid_array.link_to_front(
                &mut pipe_6a.pipe_fluid_array, 
                ctah_advection_heat_transfer_interaction)
                .unwrap();


            pipe_6a.pipe_fluid_array.link_to_front(
                &mut ctah_vertical_label_7a.pipe_fluid_array, 
                ctah_advection_heat_transfer_interaction)
                .unwrap();

            ctah_vertical_label_7a.pipe_fluid_array.link_to_front(
                &mut ctah_horizontal_label_7b.pipe_fluid_array, 
                ctah_advection_heat_transfer_interaction)
                .unwrap();


            ctah_horizontal_label_7b.pipe_fluid_array.link_to_front(
                &mut pipe_8a.pipe_fluid_array, 
                ctah_advection_heat_transfer_interaction)
                .unwrap();


            pipe_8a.pipe_fluid_array.link_to_front(
                &mut static_mixer_40_label_8.pipe_fluid_array, 
                ctah_advection_heat_transfer_interaction)
                .unwrap();


            static_mixer_40_label_8.pipe_fluid_array.link_to_front(
                &mut pipe_9.pipe_fluid_array, 
                ctah_advection_heat_transfer_interaction)
                .unwrap();


            pipe_9.pipe_fluid_array.link_to_front(
                &mut pipe_10.pipe_fluid_array, 
                ctah_advection_heat_transfer_interaction)
                .unwrap();


            pipe_10.pipe_fluid_array.link_to_front(
                &mut pipe_11.pipe_fluid_array, 
                ctah_advection_heat_transfer_interaction)
                .unwrap();


            pipe_11.pipe_fluid_array.link_to_front(
                &mut pipe_12.pipe_fluid_array, 
                ctah_advection_heat_transfer_interaction)
                .unwrap();


            pipe_12.pipe_fluid_array.link_to_front(
                &mut ctah_pump.pipe_fluid_array, 
                ctah_advection_heat_transfer_interaction)
                .unwrap();


            ctah_pump.pipe_fluid_array.link_to_front(
                &mut pipe_13.pipe_fluid_array, 
                ctah_advection_heat_transfer_interaction)
                .unwrap();

            pipe_13.pipe_fluid_array.link_to_front(
                &mut pipe_14.pipe_fluid_array, 
                ctah_advection_heat_transfer_interaction)
                .unwrap();

            pipe_14.pipe_fluid_array.link_to_front(
                &mut flowmeter_40_14a.pipe_fluid_array, 
                ctah_advection_heat_transfer_interaction)
                .unwrap();

            flowmeter_40_14a.pipe_fluid_array.link_to_front(
                &mut pipe_15.pipe_fluid_array, 
                ctah_advection_heat_transfer_interaction)
                .unwrap();

            pipe_15.pipe_fluid_array.link_to_front(
                &mut pipe_16.pipe_fluid_array, 
                ctah_advection_heat_transfer_interaction)
                .unwrap();

            pipe_16.pipe_fluid_array.link_to_front(
                &mut pipe_17a.pipe_fluid_array, 
                ctah_advection_heat_transfer_interaction)
                .unwrap();

            pipe_17a.pipe_fluid_array.link_to_front(
                bottom_mixing_node_17a_17b_18, 
                ctah_advection_heat_transfer_interaction)
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

            // ctah branch 
            pipe_5b.heat_transfer_to_ambient = ambient_htc;
            pipe_6a.heat_transfer_to_ambient = ambient_htc;
            static_mixer_41_label_6.heat_transfer_to_ambient = ambient_htc;
            ctah_vertical_label_7a.heat_transfer_to_ambient = ctah_heat_transfer_coeff;
            ctah_horizontal_label_7b.heat_transfer_to_ambient = ctah_heat_transfer_coeff;
            pipe_8a.heat_transfer_to_ambient = ambient_htc;
            static_mixer_40_label_8.heat_transfer_to_ambient = ambient_htc;
            pipe_9.heat_transfer_to_ambient = ambient_htc;
            pipe_10.heat_transfer_to_ambient = ambient_htc;
            pipe_11.heat_transfer_to_ambient = ambient_htc;
            pipe_12.heat_transfer_to_ambient = ambient_htc;
            ctah_pump.heat_transfer_to_ambient = ambient_htc;
            pipe_13.heat_transfer_to_ambient = ambient_htc;
            pipe_14.heat_transfer_to_ambient = ambient_htc;
            flowmeter_40_14a.heat_transfer_to_ambient = ambient_htc;
            pipe_15.heat_transfer_to_ambient = ambient_htc;
            pipe_16.heat_transfer_to_ambient = ambient_htc;
            pipe_17a.heat_transfer_to_ambient = ambient_htc;

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

            // ctah branch 
            pipe_5b.ambient_temperature = ambient_temp_user_set;
            pipe_6a.ambient_temperature = ambient_temp_user_set;
            static_mixer_41_label_6.ambient_temperature = ambient_temp_user_set;
            ctah_vertical_label_7a.ambient_temperature = ambient_temp_user_set;
            ctah_horizontal_label_7b.ambient_temperature = ambient_temp_user_set;
            pipe_8a.ambient_temperature = ambient_temp_user_set;
            static_mixer_40_label_8.ambient_temperature = ambient_temp_user_set;
            pipe_9.ambient_temperature = ambient_temp_user_set;
            pipe_10.ambient_temperature = ambient_temp_user_set;
            pipe_11.ambient_temperature = ambient_temp_user_set;
            pipe_12.ambient_temperature = ambient_temp_user_set;
            ctah_pump.ambient_temperature = ambient_temp_user_set;
            pipe_13.ambient_temperature = ambient_temp_user_set;
            pipe_14.ambient_temperature = ambient_temp_user_set;
            flowmeter_40_14a.ambient_temperature = ambient_temp_user_set;
            pipe_15.ambient_temperature = ambient_temp_user_set;
            pipe_16.ambient_temperature = ambient_temp_user_set;
            pipe_17a.ambient_temperature = ambient_temp_user_set;
            
        }
        // add lateral heat losses and power through heater
        // for everything except the DHX STHE
        // because DHX sthe requires mass flowrates in both sides of the loop
        {
            let zero_power: Power = Power::ZERO;
            // heater branch
            //
            // note that flow direction must be set correctly 
            // in order for this thing to work properly






            pipe_4.lateral_and_miscellaneous_connections_no_wall_correction(
                heater_flow, zero_power).unwrap();
            pipe_3.lateral_and_miscellaneous_connections_no_wall_correction(
                heater_flow, zero_power).unwrap();
            pipe_2a.lateral_and_miscellaneous_connections_no_wall_correction(
                heater_flow, zero_power).unwrap();
            static_mixer_10_label_2.lateral_and_miscellaneous_connections_no_wall_correction(
                heater_flow, zero_power).unwrap();
            heater_top_head_1a.lateral_and_miscellaneous_connections_no_wall_correction(
                heater_flow, zero_power).unwrap();
            let heat_rate_in_annular_pipe = Power::ZERO;
            let prandtl_wall_correction_setting = false;
            heater_version1_1.lateral_and_miscellaneous_connections(
                prandtl_wall_correction_setting,
                heater_flow, 
                heat_rate_through_heater,
                heat_rate_in_annular_pipe).unwrap();
            heater_bottom_head_1b.lateral_and_miscellaneous_connections_no_wall_correction(
                heater_flow, zero_power).unwrap();
            pipe_18.lateral_and_miscellaneous_connections_no_wall_correction(
                heater_flow, zero_power).unwrap();


            // DHX branch 
            pipe_5a.lateral_and_miscellaneous_connections_no_wall_correction(
                dhx_flow, zero_power).unwrap();

            pipe_26.lateral_and_miscellaneous_connections_no_wall_correction(
                dhx_flow, zero_power).unwrap();
            pipe_25a.lateral_and_miscellaneous_connections_no_wall_correction(
                dhx_flow, zero_power).unwrap();
            static_mixer_21_label_25.lateral_and_miscellaneous_connections_no_wall_correction(
                dhx_flow, zero_power).unwrap();
            static_mixer_20_label_23.lateral_and_miscellaneous_connections_no_wall_correction(
                dhx_flow, zero_power).unwrap();
            pipe_23a.lateral_and_miscellaneous_connections_no_wall_correction(
                dhx_flow, zero_power).unwrap();
            pipe_22.lateral_and_miscellaneous_connections_no_wall_correction(
                dhx_flow, zero_power).unwrap();
            flowmeter_20_21a.lateral_and_miscellaneous_connections_no_wall_correction(
                dhx_flow, zero_power).unwrap();
            pipe_21.lateral_and_miscellaneous_connections_no_wall_correction(
                dhx_flow, zero_power).unwrap();
            pipe_20.lateral_and_miscellaneous_connections_no_wall_correction(
                dhx_flow, zero_power).unwrap();
            pipe_19.lateral_and_miscellaneous_connections_no_wall_correction(
                dhx_flow, zero_power).unwrap();
            pipe_17b.lateral_and_miscellaneous_connections_no_wall_correction(
                dhx_flow, zero_power).unwrap();

            // ctah branch 
            pipe_5b.lateral_and_miscellaneous_connections_no_wall_correction(
                ctah_flow, zero_power).unwrap();
            pipe_6a.lateral_and_miscellaneous_connections_no_wall_correction(
                ctah_flow, zero_power).unwrap();
            static_mixer_41_label_6.lateral_and_miscellaneous_connections_no_wall_correction(
                ctah_flow, zero_power).unwrap();
            ctah_vertical_label_7a.lateral_and_miscellaneous_connections_no_wall_correction(
                ctah_flow, zero_power).unwrap();
            ctah_horizontal_label_7b.lateral_and_miscellaneous_connections_no_wall_correction(
                ctah_flow, zero_power).unwrap();
            pipe_8a.lateral_and_miscellaneous_connections_no_wall_correction(
                ctah_flow, zero_power).unwrap();
            static_mixer_40_label_8.lateral_and_miscellaneous_connections_no_wall_correction(
                ctah_flow, zero_power).unwrap();
            pipe_9.lateral_and_miscellaneous_connections_no_wall_correction(
                ctah_flow, zero_power).unwrap();
            pipe_10.lateral_and_miscellaneous_connections_no_wall_correction(
                ctah_flow, zero_power).unwrap();
            pipe_11.lateral_and_miscellaneous_connections_no_wall_correction(
                ctah_flow, zero_power).unwrap();
            pipe_12.lateral_and_miscellaneous_connections_no_wall_correction(
                ctah_flow, zero_power).unwrap();
            ctah_pump.lateral_and_miscellaneous_connections_no_wall_correction(
                ctah_flow, zero_power).unwrap();
            pipe_13.lateral_and_miscellaneous_connections_no_wall_correction(
                ctah_flow, zero_power).unwrap();
            pipe_14.lateral_and_miscellaneous_connections_no_wall_correction(
                ctah_flow, zero_power).unwrap();
            flowmeter_40_14a.lateral_and_miscellaneous_connections_no_wall_correction(
                ctah_flow, zero_power).unwrap();
            pipe_15.lateral_and_miscellaneous_connections_no_wall_correction(
                ctah_flow, zero_power).unwrap();
            pipe_16.lateral_and_miscellaneous_connections_no_wall_correction(
                ctah_flow, zero_power).unwrap();
            pipe_17a.lateral_and_miscellaneous_connections_no_wall_correction(
                ctah_flow, zero_power).unwrap();

        }

        // now we are done
        //

}


/// pri loop timestep advance for three loops 
/// except dhx

pub fn pri_loop_three_branch_advance_timestep_except_dhx_ver_4(
    timestep: Time,
    pipe_4: &mut InsulatedFluidComponent,
    pipe_3: &mut InsulatedFluidComponent,
    pipe_2a: &mut InsulatedFluidComponent,
    static_mixer_10_label_2: &mut InsulatedFluidComponent,
    heater_top_head_1a: &mut InsulatedFluidComponent,
    heater_version1_1: &mut InsulatedPorousMediaFluidComponent,
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
    pipe_5b: &mut InsulatedFluidComponent,
    static_mixer_41_label_6 :&mut InsulatedFluidComponent,
    pipe_6a :&mut InsulatedFluidComponent,
    ctah_vertical_label_7a :&mut NonInsulatedFluidComponent,
    ctah_horizontal_label_7b :&mut NonInsulatedFluidComponent,
    pipe_8a :&mut InsulatedFluidComponent,
    static_mixer_40_label_8 :&mut InsulatedFluidComponent,
    pipe_9 :&mut InsulatedFluidComponent,
    pipe_10 :&mut InsulatedFluidComponent,
    pipe_11 :&mut InsulatedFluidComponent,
    pipe_12 :&mut InsulatedFluidComponent,
    ctah_pump :&mut NonInsulatedFluidComponent,
    pipe_13 : &mut InsulatedFluidComponent,
    pipe_14 : &mut InsulatedFluidComponent,
    flowmeter_40_14a :&mut NonInsulatedFluidComponent,
    pipe_15 :&mut InsulatedFluidComponent,
    pipe_16 :&mut InsulatedFluidComponent,
    pipe_17a :&mut InsulatedFluidComponent,
    top_mixing_node_5a_5b_4: &mut HeatTransferEntity,
    bottom_mixing_node_17a_17b_18: &mut HeatTransferEntity,
    ){

    // heater branch
    pipe_4.advance_timestep(timestep).unwrap();
    pipe_3.advance_timestep(timestep).unwrap();
    pipe_2a.advance_timestep(timestep).unwrap();
    static_mixer_10_label_2.advance_timestep(timestep).unwrap();
    heater_top_head_1a.advance_timestep(timestep).unwrap();
    heater_version1_1.advance_timestep(timestep);
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

    pipe_5b.advance_timestep(timestep).unwrap();
    static_mixer_41_label_6.advance_timestep(timestep).unwrap();
    pipe_6a.advance_timestep(timestep).unwrap();
    ctah_vertical_label_7a.advance_timestep(timestep).unwrap();
    ctah_horizontal_label_7b.advance_timestep(timestep).unwrap();
    pipe_8a.advance_timestep(timestep).unwrap();
    static_mixer_40_label_8.advance_timestep(timestep).unwrap();
    pipe_9.advance_timestep(timestep).unwrap();
    pipe_10.advance_timestep(timestep).unwrap();
    pipe_11.advance_timestep(timestep).unwrap();
    pipe_12.advance_timestep(timestep).unwrap();
    ctah_pump.advance_timestep(timestep).unwrap();
    pipe_13.advance_timestep(timestep).unwrap();
    pipe_14.advance_timestep(timestep).unwrap();
    flowmeter_40_14a.advance_timestep(timestep).unwrap();
    pipe_15.advance_timestep(timestep).unwrap();
    pipe_16.advance_timestep(timestep).unwrap();
    pipe_17a.advance_timestep(timestep).unwrap();

    // two mixing nodes
    top_mixing_node_5a_5b_4.advance_timestep_mut_self(timestep).unwrap();
    bottom_mixing_node_17a_17b_18.advance_timestep_mut_self(timestep).unwrap();

}
