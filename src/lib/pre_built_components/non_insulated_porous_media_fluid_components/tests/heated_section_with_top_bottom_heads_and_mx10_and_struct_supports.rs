#[test]
pub fn heater_plus_mx_10_with_supports(){

    use core::time;
    use std::{time::SystemTime, thread::{JoinHandle, self}};

    use uom::{si::{time::second, power::kilowatt}, ConstZero};

    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use crate::prelude::beta_testing::*;
    use uom::si::time::minute;

    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::length::foot;
    use uom::si::length::inch;
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;



    // bare heater plus heads exaample
    let initial_temperature: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(79.12);
    let inlet_temperature = initial_temperature;
    let ambient_air_temp: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(21.67);

    let number_of_inner_temperature_nodes: usize = 6;
    
    let mut heater_v2_bare = NonInsulatedPorousMediaFluidComponent::new_dewet_model_heater_v2(
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
        heater_v2_bare = NonInsulatedPorousMediaFluidComponent::ciet_heater_v2_generic_model(
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

    let max_time = Time::new::<minute>(0.2);
    // on my pc, the simulation time using 
    // cargo run --release 
    // is less than 10ms
    let timestep = Time::new::<second>(0.015);
    let mut simulation_time = Time::ZERO;
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);
    let heater_power = Power::new::<kilowatt>(8.0);

    let loop_time = SystemTime::now();
    // main loop
    
    let main_loop = thread::spawn( move || {
        while max_time > simulation_time {

            // time start 
            let loop_time_start = loop_time.elapsed().unwrap();
            // create interactions 


            // let's get heater temperatures for post processing
            // as well as the interaction
            // for simplicity, i use the boussineseq approximation,
            // which assumes that heat transfer is governed by 
            // average density (which doesn't change much for liquid 
            // anyway)

            let connect_struct_support = true; 

            let mut therminol_array_clone: FluidArray 
            = heater_v2_bare.pipe_fluid_array.clone().try_into().unwrap();

            let _therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            therminol_array_clone.get_temperature_vector().unwrap();

            let heater_surface_array_clone: SolidColumn 
            = heater_v2_bare.pipe_shell.clone().try_into().unwrap();

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

            let _static_mixer_exit_temperature: ThermodynamicTemperature
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
                // prints therminol temperature 
                let heater_surf_temp_degc: Vec<f64> = heater_surface_array_temp
                    .iter().map(
                        |&temperature|{
                            temperature.get::<degree_celsius>()
                        }
                    ).collect();

                // print surface temperature 
                dbg!(heater_surf_temp_degc);

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

            // make axial connections to BCs 
            //
            // note: need to speed up this part, too slow

            heater_bottom_head_bare.therminol_array.link_to_back(
                &mut inlet_bc,
                generic_advection_interaction
            ).unwrap();

            heater_v2_bare.pipe_fluid_array.link_to_back(
                &mut heater_bottom_head_bare.therminol_array,
                generic_advection_interaction
            ).unwrap();

            heater_v2_bare.pipe_fluid_array.link_to_front(
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

            
            let wait: bool = false;

            if wait {

                let ten_millis = time::Duration::from_millis(10);

                thread::sleep(ten_millis);

            } else {
                // make other connections by spawning a new thread 
                // this is the parallel version
                let heater_2_join_handle: JoinHandle<NonInsulatedPorousMediaFluidComponent> 
                = heater_v2_bare.
                    ciet_heater_v2_lateral_connection_thread_spawn(
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

                if connect_struct_support {
                    // link struct supports to ambient air
                    // axially 

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

                if connect_struct_support {

                    // link struct supports to heater top/bottom heads
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

                    // note, the heater top and bottom head area changed 
                    // during course of this interaction, so should be okay


                    // i will also connect heater shell to the structural support 
                    // via the head as in ciet 

                    heater_v2_bare.pipe_shell.link_to_back(
                        &mut heater_bottom_head_bare.steel_shell,
                        support_conductance_interaction
                    ).unwrap();

                    heater_v2_bare.pipe_shell.link_to_front(
                        &mut heater_top_head_bare.steel_shell,
                        support_conductance_interaction
                    ).unwrap();
                    
                    // probably edit this to include twisted tape conductance
                    heater_v2_bare.interior_solid_array_for_porous_media.link_to_back(
                        &mut heater_bottom_head_bare.twisted_tape_interior,
                        support_conductance_interaction
                    ).unwrap();

                    heater_v2_bare.interior_solid_array_for_porous_media.link_to_front(
                        &mut heater_top_head_bare.twisted_tape_interior,
                        support_conductance_interaction
                    ).unwrap();

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
                }

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


                if connect_struct_support {
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



                }

                static_mixer_mx_10_object = static_mixer_join_handle.join().unwrap();
                static_mixer_mx_10_pipe = static_mixer_pipe_join_handle.join().unwrap();
                heater_v2_bare = heater_2_join_handle.join().unwrap();
                heater_bottom_head_bare = heater_bottom_join_handle.join().unwrap();
                heater_top_head_bare = heater_top_head_join_handle.join().unwrap();


            } 
            simulation_time += timestep;

            let time_taken_for_calculation_loop = loop_time.elapsed().unwrap()
            - loop_time_start;

            dbg!(time_taken_for_calculation_loop);

        }

    });

    main_loop.join().unwrap();

    // once simulation completed, write data


    //todo!("haven't coded csv writing file")




}
