
//! The heater module here represents components from 
//!
//! BT-11 to BT-12
//! This is because heater inlet and outlet temperatures are measured 
//! using BT-11 and BT-12
//! 
//! However, BT-11 is located before the heater bottom head 
//! and BT-12 is located after the static mixer MX-10 
//!
//! Hence, we have four sections,
//!
//! the heater top head, heater bottom head, heated section,
//! mixer MX-10 and the static mixer pipe attached to MX-10 modelled 
//! in the Relap and SAM model
//!
//! So there is not only some residence time, but also other mechanisms 
//! for parasitic heat loss 
//!
//! Dr Dane De Wet's transform model does callibrate for this using 
//! a heat transfer coefficient of 20 W/(m^2 K) instead of 6 W/(m^2 K)
//!
//!
//! I intend to connect structural supports to the heater top and bottom 
//! head and callibrate the length of those structural supports 
//! as part of model callibration such that the heater inlet is 
//! 80C approximately, and the heater outlet is 102.45 C 
//!
//! at nominal heater power of 8 kW
//!
//! For this, I also want to ensure that the code runs fast enough,
//! at least faster than real time, so it is suitable for digital 
//! twin applications
//!
//!
//! 
//!
///
///
/// it has twisted tape

use core::time;
use std::thread::{self};
use std::thread::JoinHandle;
use std::time::SystemTime;

use csv::Writer;

use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::boundary_conditions::BCType;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::pre_built_components::ciet_heater_top_and_bottom_head_bare::HeaterTopBottomHead;
use crate::pre_built_components::non_insulated_porous_media_fluid_components::NonInsulatedPorousMediaFluidComponent;
use crate::pre_built_components::insulated_porous_media_fluid_components::InsulatedPorousMediaFluidComponent;
use crate::pre_built_components::ciet_struct_supports::StructuralSupport;
use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
use uom::si::f64::*;
use uom::si::mass_rate::kilogram_per_second;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::ConstZero;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::length::{centimeter, inch};
use uom::si::length::foot;
use uom::si::power::kilowatt;
use uom::si::ratio::ratio;
use uom::si::time::second;

#[test]
pub fn example_heater_with_struct_supports_and_mx10(){



    // bare heater plus heads exaample
    let initial_temperature: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(79.12);
    let inlet_temperature = initial_temperature;
    let ambient_air_temp: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(21.67);

    let number_of_inner_temperature_nodes: usize = 6;

    let mut heater_v2_bare = NonInsulatedPorousMediaFluidComponent::new_dewet_model(
        initial_temperature,
        ambient_air_temp,
        number_of_inner_temperature_nodes
    );



    let mut heater_top_head_bare: HeaterTopBottomHead 
    = HeaterTopBottomHead::new_top_head(
        initial_temperature,
        ambient_air_temp);

    let mut heater_bottom_head_bare: HeaterTopBottomHead 
    = HeaterTopBottomHead::new_bottom_head(
        initial_temperature,
        ambient_air_temp);

    // calibration of heat transfer coeff
    let calibration_mode = true; 

    if calibration_mode {

        let h_to_air = HeatTransfer::new::<watt_per_square_meter_kelvin>
            (20.0);
        heater_v2_bare = NonInsulatedPorousMediaFluidComponent::_user_callibrated_htc_to_air_model(
            initial_temperature,
            ambient_air_temp,
            number_of_inner_temperature_nodes,
            h_to_air
        );

        heater_top_head_bare = HeaterTopBottomHead:: 
            _new_user_callibrated_top_head(
                initial_temperature,
                ambient_air_temp,
                h_to_air
            );
        heater_bottom_head_bare = HeaterTopBottomHead:: 
            _new_user_callibrated_bottom_head(
                initial_temperature,
                ambient_air_temp,
                h_to_air
            );
    }

    let mut static_mixer_mx_10_object: InsulatedPorousMediaFluidComponent 
    = InsulatedPorousMediaFluidComponent::new_static_mixer_2_mx10(
        initial_temperature,
        ambient_air_temp);

    let mut static_mixer_mx_10_pipe: InsulatedPorousMediaFluidComponent 
    = InsulatedPorousMediaFluidComponent::new_static_mixer_pipe_2a_mx10(
        initial_temperature,
        ambient_air_temp);

    let struct_support_equiv_diameter: Length = Length::new::<inch>(0.5);
    let struc_support_equiv_length: Length = Length::new::<foot>(1.0);


    let mut structural_support_heater_top_head = 
    StructuralSupport::new_steel_support_cylinder(
        struc_support_equiv_length,
        struct_support_equiv_diameter,
        initial_temperature,
        ambient_air_temp);

    let mut structural_support_heater_bottom_head = 
    structural_support_heater_top_head.clone();

    let mut structural_support_mx_10 = 
    structural_support_heater_top_head.clone();

    let approx_support_conductance: ThermalConductance = 
    structural_support_heater_top_head.get_axial_node_to_bc_conductance();


    let support_conductance_interaction = HeatTransferInteractionType::
        UserSpecifiedThermalConductance(approx_support_conductance);


    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    let mut ambient_air_temp_bc: HeatTransferEntity = 
    inlet_bc.clone();

    // time settings 

    let max_time = Time::new::<second>(300.0);
    // on my pc, the simulation time using 
    // cargo run --release 
    // is less than 10ms
    let timestep = Time::new::<second>(0.015);
    let mut simulation_time = Time::ZERO;
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);
    let mut heater_power = Power::new::<kilowatt>(8.0);

    let transient_start_time = Time::new::<second>(100.0);

    let loop_time = SystemTime::now();


    // csv writer
    let mut wtr = Writer::from_path("lib_heater_example_steady_state.csv")
        .unwrap();
    wtr.write_record(&["time_seconds",
        "heater_power_kilowatts",
        "heated_section_exit_temperature_celsius",
        "bt_12_temperature_celsius",
        "shell_temperature_celsius",
        "timestep_seconds",])
        .unwrap();

    let mut temp_profile_wtr = Writer::from_path(
        "lib_heater_example_temp_profile.csv")
        .unwrap();

    let number_of_nodes_heated_section = number_of_inner_temperature_nodes + 2;
    let node_length_heated_section = heater_v2_bare.get_component_length()
    / number_of_nodes_heated_section as f64;

    // for temperature profile
    let mut header_vec: Vec<String> = vec![];
    header_vec.push("simulation_time_seconds".to_string());
    header_vec.push("elapsed_time_seconds".to_string());
    for index in 0..number_of_nodes_heated_section {

        let half_node_length = 0.5 * node_length_heated_section;
        let mid_node_length: Length = 
        index as f64 * node_length_heated_section + half_node_length;

        let prefix: String = "heater_temp_celsius_at_".to_string();

        let suffix: String = "_cm".to_string();

        let mid_node_length_cm: f64 = 
        mid_node_length.get::<centimeter>();

        let mid_node_length_string: String = 
        mid_node_length_cm.to_string();

        let header: String = prefix + &mid_node_length_string + &suffix;

        header_vec.push(header);


    }
    temp_profile_wtr.write_record(&header_vec).unwrap();

    // main loop

    let main_loop = thread::spawn( move || {
        let calculation_time_elapsed = SystemTime::now();
        while max_time > simulation_time {

            // time start 
            let loop_time_start = loop_time.elapsed().unwrap();

            // heater power changes 

            if simulation_time > transient_start_time {

                // step down 500 watts
                heater_power = Power::new::<kilowatt>(7.5);

            }

            // create interactions 


            // let's get heater temperatures for post processing
            // as well as the interaction
            // for simplicity, i use the boussineseq approximation,
            // which assumes that heat transfer is governed by 
            // average density (which doesn't change much for liquid 
            // anyway)


            let mut therminol_array_clone: FluidArray 
            = heater_v2_bare.therminol_array.clone().try_into().unwrap();

            let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            therminol_array_clone.get_temperature_vector().unwrap();

            let heater_surface_array_clone: SolidColumn 
            = heater_v2_bare.steel_shell.clone().try_into().unwrap();

            let heater_surface_array_temp: Vec<ThermodynamicTemperature> = 
            heater_surface_array_clone.get_temperature_vector().unwrap();

            let heater_fluid_bulk_temp: ThermodynamicTemperature = 
            therminol_array_clone.try_get_bulk_temperature().unwrap();

            let heater_top_head_bare_therminol_clone: FluidArray = 
            heater_top_head_bare.therminol_array.clone().try_into().unwrap();

            let heater_top_head_exit_temperature: ThermodynamicTemperature = 
            heater_top_head_bare_therminol_clone.get_temperature_vector()
                .unwrap().into_iter().last().unwrap();

            let static_mixer_therminol_clone: FluidArray = 
            static_mixer_mx_10_object.pipe_fluid_array.clone().try_into().unwrap();

            let static_mixer_exit_temperature: ThermodynamicTemperature
            = static_mixer_therminol_clone.get_temperature_vector().unwrap()
                .into_iter().last().unwrap();

            let static_mixer_pipe_therminol_clone: FluidArray = 
            static_mixer_mx_10_pipe.pipe_fluid_array.clone().try_into().unwrap();


            let heater_therminol_avg_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                heater_fluid_bulk_temp).unwrap();

            let generic_advection_interaction = 
            HeatTransferInteractionType::new_advection_interaction(
                mass_flowrate,
                heater_therminol_avg_density,
                heater_therminol_avg_density,
            );
            {
                // prints heater surface temperature 
                let heater_surf_temp_degc: Vec<f64> = heater_surface_array_temp
                    .iter().map(
                        |&temperature|{
                            temperature.get::<degree_celsius>()
                        }
                    ).collect();

                // print surface temperature 
                dbg!(heater_surf_temp_degc);
                // print therminol temperature, toggle this to true 
                // if you want to print the temperature of the fluid 
                // control volumes
                let print_therminol_temp = false;
                if print_therminol_temp {
                    let therminol_temp_degc: Vec<f64> = therminol_array_temperature
                        .iter().map(
                            |&temperature|{
                                temperature.get::<degree_celsius>()
                            }
                            ).collect();
                    dbg!(therminol_temp_degc);
                }

                let bt_12_temperature: ThermodynamicTemperature = 
                static_mixer_pipe_therminol_clone.get_temperature_vector().unwrap() 
                    .into_iter().last().unwrap();
                // print outlet temperature 
                dbg!(heater_top_head_exit_temperature
                .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));

                // bt_12_temperature, which is actually the output temperature of static 
                // mixer 10
                dbg!(bt_12_temperature
                .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));

                //// print therminol temperature 
                //dbg!("Therminol Array Temp: ", therminol_array_temperature);

                //// print twisted tape temperature 
                //dbg!("twisted tape Temp: 
                //note: conduction occurs, so first node is hotter\n 
                //than the therminol fluid", twisted_tape_temperature);

                // print simulation time
                // dbg diagnostics probably not the cause of mem leaks
                dbg!(simulation_time);
            }
            // record current fluid temperature profile
            {
                let mut temp_profile_data_vec: Vec<String> = vec![];

                let current_time_string = 
                simulation_time.get::<second>().to_string();

                // next simulation time string 
                let elapsed_calc_time_seconds_string = 
                calculation_time_elapsed.elapsed().unwrap().as_secs().to_string();
                temp_profile_data_vec.push(current_time_string);
                temp_profile_data_vec.push(elapsed_calc_time_seconds_string);

                let steel_temperature_vec = 
                heater_v2_bare.steel_shell_temperature();



                for node_temp in steel_temperature_vec.iter() {

                    let node_temp_deg_c: f64 = 
                    node_temp.get::<degree_celsius>();

                    let node_temp_c_string: String = 
                    node_temp_deg_c.to_string();

                    temp_profile_data_vec.push(node_temp_c_string);
                }

                temp_profile_wtr.write_record(&temp_profile_data_vec).unwrap();
            }
            // outlet temperature profile 
            {
                // csv data writing

                let mut therminol_fluid_arr: FluidArray = 
                therminol_array_clone;

                let therminol_outlet_temp:ThermodynamicTemperature = 
                therminol_fluid_arr.front_single_cv.get_temperature_from_enthalpy_and_set().unwrap();

                let therminol_outlet_temp_string = 
                therminol_outlet_temp.get::<degree_celsius>().to_string();

                let static_mixer_outlet_temp_string = 
                static_mixer_exit_temperature.get::<degree_celsius>().to_string();

                let current_time_string = 
                simulation_time.get::<second>().to_string();

                // probably want to have a method to set heater power
                let heater_power_kilowatt_string = 
                heater_power.get::<kilowatt>().to_string();

                // for st 11 

                // code block for recording inlet, shell and outlet 
                // temperatures
                //
                // now for shell temperatures, we are going to assume that 
                // ST-11 is used. 
                //
                // ST-11 is the thermocouple measuring surface temperature 
                // roughly 19 inches from the bottom of the heater 
                // The entire heated length excluding heater top and 
                // bottom heads is about 64 inches 
                //
                // So 19/64 is about 0.30 of the way through
                let st_11_length: Length = Length::new::<inch>(19_f64);


                // now I want to find out which node it is,
                // so i need the node length first 
                //

                let number_of_nodes = number_of_inner_temperature_nodes + 2;

                let node_length: Length = 
                heater_v2_bare.get_component_length()
                / number_of_nodes as f64;

                // then use st_11 divide by node length 

                let st_11_rough_node_number: Ratio = st_11_length / node_length;

                // now, st_11 is about 19 inches, out of 64, and we have 
                // 8 equal nodes, each node is 
                // 12.5% of the heated length
                //
                // so this is about 30% of the way through
                //
                // so this is node three. 
                //
                // if we take st_11_length/node_length 
                // we would get about 2.375 for this ratio 
                //
                // we need to round up to get 3 
                // but the third node is the 2nd index in the matrix 
                // because the index starts from zero
                //
                //
                // so round it up and then minus 1 
                // most of the time, round down is ok, but rounding up 
                // makes more logical sense given this derivation

                let st_11_node_number: usize = 
                st_11_rough_node_number.get::<ratio>().ceil() as usize;

                let st_11_index_number: usize = st_11_node_number - 1;

                // now that we got the index number, we can get the 
                // outer surface temperature 
                let steel_shell_clone: SolidColumn = 
                heater_v2_bare.steel_shell.clone().try_into().unwrap();

                let steel_temperature_array = 
                steel_shell_clone.get_temperature_vector().unwrap();

                let st_11_node_temp: ThermodynamicTemperature = 
                steel_temperature_array[st_11_index_number];

                let shell_celsius_string = 
                st_11_node_temp.get::<degree_celsius>().to_string();

                // timestep in seconds
                //

                let timestep_string = 
                timestep.get::<second>().to_string();


                wtr.write_record(&[current_time_string,
                    heater_power_kilowatt_string,
                    therminol_outlet_temp_string,
                    static_mixer_outlet_temp_string,
                    shell_celsius_string,
                    timestep_string])
                    .unwrap();

            }

            // make axial connections to BCs 
            //
            // note: need to speed up this part, too slow

            heater_bottom_head_bare.therminol_array.link_to_back(
                &mut inlet_bc,
                generic_advection_interaction
            ).unwrap();

            heater_v2_bare.therminol_array.link_to_back(
                &mut heater_bottom_head_bare.therminol_array,
                generic_advection_interaction
            ).unwrap();

            heater_v2_bare.therminol_array.link_to_front(
                &mut heater_top_head_bare.therminol_array,
                generic_advection_interaction
            ).unwrap();


            heater_top_head_bare.therminol_array.link_to_front(
                &mut static_mixer_mx_10_object.pipe_fluid_array,
                generic_advection_interaction
            ).unwrap();

            static_mixer_mx_10_object.pipe_fluid_array.link_to_front(
                &mut static_mixer_mx_10_pipe.pipe_fluid_array,
                generic_advection_interaction
            ).unwrap();

            static_mixer_mx_10_pipe.pipe_fluid_array.link_to_front(
                &mut outlet_bc,
                generic_advection_interaction
            ).unwrap();


            // make other connections by spawning a new thread 
            // this is the parallel version
            let heater_2_join_handle: JoinHandle<NonInsulatedPorousMediaFluidComponent> 
            = heater_v2_bare.
                lateral_connection_thread_spawn(
                    mass_flowrate,
                    heater_power);

            let heater_bottom_join_handle: JoinHandle<HeaterTopBottomHead> 
            = heater_bottom_head_bare. 
                lateral_connection_thread_spawn(
                    mass_flowrate);

            let heater_top_head_join_handle = 
            heater_top_head_bare.lateral_connection_thread_spawn(
                mass_flowrate);


            let static_mixer_join_handle = 
            static_mixer_mx_10_object.lateral_connection_thread_spawn(
                mass_flowrate);

            let static_mixer_pipe_join_handle = 
            static_mixer_mx_10_pipe.lateral_connection_thread_spawn(
                mass_flowrate);

            // link struct supports to ambient air
            // axially 

            // this gives the user ability to switch support 
            // structures on and off

            let support_structures_enabled = true;
            let dummy_axial_conduction_enabled = false;

            if support_structures_enabled == true {
                structural_support_heater_bottom_head. 
                    support_array.link_to_front(
                        &mut ambient_air_temp_bc,
                        support_conductance_interaction
                        ).unwrap();

                structural_support_heater_top_head. 
                    support_array.link_to_front(
                        &mut ambient_air_temp_bc,
                        support_conductance_interaction
                        ).unwrap();

                structural_support_mx_10.support_array.link_to_front(
                    &mut ambient_air_temp_bc,
                    support_conductance_interaction
                    ).unwrap();
            }


            static_mixer_mx_10_object = static_mixer_join_handle.join().unwrap();
            static_mixer_mx_10_pipe = static_mixer_pipe_join_handle.join().unwrap();
            heater_v2_bare = heater_2_join_handle.join().unwrap();
            heater_bottom_head_bare = heater_bottom_join_handle.join().unwrap();
            heater_top_head_bare = heater_top_head_join_handle.join().unwrap();


            // link struct supports to heater top/bottom heads
            if support_structures_enabled == true {
                structural_support_heater_top_head.
                    support_array.link_to_back(
                        &mut heater_top_head_bare.steel_shell,
                        support_conductance_interaction
                        ).unwrap();
                structural_support_heater_bottom_head. 
                    support_array.link_to_back(
                        &mut heater_bottom_head_bare.steel_shell,
                        support_conductance_interaction
                        ).unwrap();

                structural_support_mx_10.support_array.link_to_back(
                    &mut static_mixer_mx_10_pipe.pipe_shell,
                    support_conductance_interaction
                    ).unwrap();
            }

            // note, the heater top and bottom head area changed 
            // during course of this interaction, so should be okay


            // i will also connect heater shell to the structural support 
            // via the head as in ciet 

            if dummy_axial_conduction_enabled == true {
                heater_v2_bare.steel_shell.link_to_back(
                    &mut heater_bottom_head_bare.steel_shell,
                    support_conductance_interaction
                    ).unwrap();

                heater_v2_bare.steel_shell.link_to_front(
                    &mut heater_top_head_bare.steel_shell,
                    support_conductance_interaction
                    ).unwrap();

                // probably edit this to include twisted tape conductance
                heater_v2_bare.twisted_tape_interior.link_to_back(
                    &mut heater_bottom_head_bare.twisted_tape_interior,
                    support_conductance_interaction
                    ).unwrap();

                heater_v2_bare.twisted_tape_interior.link_to_front(
                    &mut heater_top_head_bare.twisted_tape_interior,
                    support_conductance_interaction
                    ).unwrap();
            }

            // now link it laterally to ambient temperatures
            let struct_support_top_head_join_handle = 
            structural_support_heater_top_head.lateral_connection_thread_spawn();
            let structural_support_heater_bottom_head_join_handle = 
            structural_support_heater_bottom_head.lateral_connection_thread_spawn();

            structural_support_mx_10.
                lateral_and_miscellaneous_connections();

            structural_support_heater_top_head = 
                struct_support_top_head_join_handle.join().unwrap();
            structural_support_heater_bottom_head = 
                structural_support_heater_bottom_head_join_handle.join().unwrap();

            //// calculate timestep (serial method)
            //heater_v2_bare.advance_timestep(
            //    timestep);

            // calculate timestep (thread spawn method, parallel) 

            let heater_2_join_handle: JoinHandle<NonInsulatedPorousMediaFluidComponent> 
            = heater_v2_bare.advance_timestep_thread_spawn(
                timestep);

            let heater_bottom_join_handle: JoinHandle<HeaterTopBottomHead> 
            = heater_bottom_head_bare. 
                advance_timestep_thread_spawn(
                    timestep);

            let heater_top_head_join_handle = 
            heater_top_head_bare.advance_timestep_thread_spawn(
                timestep);

            let static_mixer_join_handle = 
            static_mixer_mx_10_object.advance_timestep_thread_spawn(
                timestep);

            let static_mixer_pipe_join_handle = 
            static_mixer_mx_10_pipe.advance_timestep_thread_spawn(
                timestep);


            let structural_support_heater_bottom_head_join_handle = 
            structural_support_heater_bottom_head.
                advance_timestep_thread_spawn(timestep);
            let structural_support_heater_top_head_join_handle = 
            structural_support_heater_top_head.
                advance_timestep_thread_spawn(timestep);

            structural_support_mx_10._advance_timestep(
                timestep);

            structural_support_heater_bottom_head 
                =  structural_support_heater_bottom_head_join_handle.join().unwrap();
            structural_support_heater_top_head 
                =  structural_support_heater_top_head_join_handle.join().unwrap();


            static_mixer_mx_10_object = static_mixer_join_handle.join().unwrap();
            static_mixer_mx_10_pipe = static_mixer_pipe_join_handle.join().unwrap();
            heater_v2_bare = heater_2_join_handle.join().unwrap();
            heater_bottom_head_bare = heater_bottom_join_handle.join().unwrap();
            heater_top_head_bare = heater_top_head_join_handle.join().unwrap();


            simulation_time += timestep;

            let time_taken_for_calculation_loop = loop_time.elapsed().unwrap()
            - loop_time_start;

            dbg!(time_taken_for_calculation_loop);

        }

    });

    main_loop.join().unwrap();

    let thread_sleep = false;

    if thread_sleep {
        let ten_seconds = time::Duration::from_millis(10000);

        thread::sleep(ten_seconds);
    }


    // once simulation completed, write data

}


