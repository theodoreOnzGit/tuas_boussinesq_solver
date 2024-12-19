

#[test]
pub fn heater_plus_mx_10_without_supports(){
    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use crate::prelude::beta_testing::*;

    use core::time;
    use std::{time::SystemTime, thread::{JoinHandle, self}};
    use uom::{si::{time::second, power::kilowatt}, ConstZero};
    use uom::si::mass_rate::kilogram_per_second;
    // construct structs



    // bare heater plus heads exaample
    let initial_temperature: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(79.12);
    let inlet_temperature = initial_temperature;
    let ambient_air_temp: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(21.76);

    let number_of_temperature_nodes: usize = 8;
    
    let mut heater_v2_bare = HeaterVersion2Bare::new_dewet_model(
        initial_temperature,
        ambient_air_temp,
        number_of_temperature_nodes
    );


    let mut heater_top_head_bare: HeaterTopBottomHead 
    = HeaterTopBottomHead::new_top_head(
        initial_temperature,
        ambient_air_temp);

    let mut heater_bottom_head_bare: HeaterTopBottomHead 
    = HeaterTopBottomHead::new_bottom_head(
        initial_temperature,
        ambient_air_temp);

    // note: mx10 potentially has a memory leak
    let mut static_mixer_mx_10_object: InsulatedPorousMediaFluidComponent 
    = InsulatedPorousMediaFluidComponent::new_static_mixer_2_mx10(
        initial_temperature,
        ambient_air_temp);

    let mut static_mixer_mx_10_pipe: InsulatedPorousMediaFluidComponent 
    = InsulatedPorousMediaFluidComponent::new_static_mixer_pipe_2a_mx10(
        initial_temperature,
        ambient_air_temp);

    //let struct_support_equiv_diameter: Length = Length::new::<inch>(0.5);
    //let struc_support_equiv_length: Length = Length::new::<foot>(1.0);

    //let mut structural_support_heater_bottom_head: HeatTransferEntity 
    //= SingleCVNode::new_cylinder(
    //    struc_support_equiv_length,
    //    struct_support_equiv_diameter,
    //    SolidMaterial::SteelSS304L.into(),
    //    initial_temperature,
    //    Pressure::new::<atmosphere>(1.0),
    //).unwrap();

    //let mut structural_support_heater_top_head: HeatTransferEntity = 
    //structural_support_heater_bottom_head.clone();

    //let approx_support_conductance: ThermalConductance = {

    //    // for conductance, it is the half length that counts 
    //    //
    //    // bc -------- (support cv) ------------- heater head

    //    let conductivity = SolidMaterial::SteelSS304L.try_get_thermal_conductivity(
    //        initial_temperature
    //    ).unwrap();

    //    let xs_area_support = PI * 0.25 * struct_support_equiv_diameter 
    //    * struct_support_equiv_diameter;
    //    

    //    0.5 * conductivity * xs_area_support / struc_support_equiv_length

    //};

    //let support_conductance_interaction = HeatTransferInteractionType::
    //    UserSpecifiedThermalConductance(approx_support_conductance);


    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    //let mut ambient_air_temp_bc: HeatTransferEntity = 
    //inlet_bc.clone();

    // time settings 

    let max_time = Time::new::<second>(10.0);
    let timestep = Time::new::<second>(0.01);
    let mut simulation_time = Time::ZERO;
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);
    let heater_power = Power::new::<kilowatt>(8.0);

    let loop_time = SystemTime::now();
    // main loop
    // note: possible memory leak
    
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

            let connect_static_mixer_10 = true; 

            let mut therminol_array_clone: FluidArray 
            = heater_v2_bare.therminol_array.clone().try_into().unwrap();

            let _therminol_array_temperature: Vec<ThermodynamicTemperature> = 
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

            if connect_static_mixer_10 {
                let static_mixer_therminol_clone: FluidArray = 
                static_mixer_mx_10_object.pipe_fluid_array.clone().try_into().unwrap();

                let _static_mixer_exit_temperature: ThermodynamicTemperature
                = static_mixer_therminol_clone.get_temperature_vector().unwrap()
                    .into_iter().last().unwrap();

                let static_mixer_pipe_therminol_clone: FluidArray = 
                static_mixer_mx_10_pipe.pipe_fluid_array.clone().try_into().unwrap();

                let bt_12_temperature: ThermodynamicTemperature = 
                static_mixer_pipe_therminol_clone.get_temperature_vector().unwrap() 
                    .into_iter().last().unwrap();

                // bt_12_temperature, which is actually the output temperature of static 
                // mixer 10
                dbg!(bt_12_temperature
                .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));
            }

            let heater_therminol_avg_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                heater_fluid_bulk_temp).unwrap();

            let generic_advection_interaction = 
            HeatTransferInteractionType::new_advection_interaction(
                mass_flowrate,
                heater_therminol_avg_density,
                heater_therminol_avg_density,
            );
            // all unused values to try and mitigate memory leaking
            {
                // prints therminol temperature 

                // print outlet temperature 
                dbg!(heater_top_head_exit_temperature
                .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));

                // print surface temperature 
                dbg!(heater_surface_array_temp);

                //// print therminol temperature 
                //dbg!("Therminol Array Temp: ", therminol_array_temperature);

                //// print twisted tape temperature 
                //dbg!("twisted tape Temp: 
                //note: conduction occurs, so first node is hotter\n 
                //than the therminol fluid", twisted_tape_temperature);

                // print loop time 
                // dbg diagnostics probably not the cause of mem leaks
                //println!("{:?}",time_taken_for_calculation_loop.as_micros());
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

            
            if connect_static_mixer_10 {
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

            } else {

                heater_top_head_bare.therminol_array.link_to_front(
                    &mut outlet_bc,
                    generic_advection_interaction
                ).unwrap();
            }
            
            //// and axial connections for heater top and bottom heads 
            //// to support 
            ////
            //// parallelise this

            //heater_bottom_head_bare.steel_shell.link_to_back(
            //    &mut structural_support_heater_bottom_head,
            //    support_conductance_interaction
            //).unwrap();

            //heater_top_head_bare.steel_shell.link_to_front(
            //    &mut structural_support_heater_top_head,
            //    support_conductance_interaction
            //).unwrap();

            //// link the top and bottom head support to the environment 
            //// parallelise this
            //
            //plus potential memory leak here

            //structural_support_heater_bottom_head.link_to_front(
            //    &mut ambient_air_temp_bc,
            //    support_conductance_interaction
            //).unwrap();
            //structural_support_heater_top_head.link_to_front(
            //    &mut ambient_air_temp_bc,
            //    support_conductance_interaction
            //).unwrap();


            // make other connections
            //
            // this is the serial version
            //heater_v2_bare.lateral_and_miscellaneous_connections(
            //    mass_flowrate,
            //    heater_power
            //);
            let wait: bool = false;

            // parallel calc probably not the cause of memory leak
            if wait {

                let ten_millis = time::Duration::from_millis(10);

                thread::sleep(ten_millis);

            } else {
                // make other connections by spawning a new thread 
                // this is the parallel version
                let heater_2_join_handle: JoinHandle<HeaterVersion2Bare> 
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

                if connect_static_mixer_10 {

                    let static_mixer_join_handle = 
                    static_mixer_mx_10_object.lateral_connection_thread_spawn(
                        mass_flowrate);

                    let static_mixer_pipe_join_handle = 
                    static_mixer_mx_10_pipe.lateral_connection_thread_spawn(
                        mass_flowrate);

                    static_mixer_mx_10_object = static_mixer_join_handle.join().unwrap();
                    static_mixer_mx_10_pipe = static_mixer_pipe_join_handle.join().unwrap();
                }


                heater_v2_bare = heater_2_join_handle.join().unwrap();
                heater_bottom_head_bare = heater_bottom_join_handle.join().unwrap();
                heater_top_head_bare = heater_top_head_join_handle.join().unwrap();

                //// calculate timestep (serial method)
                //heater_v2_bare.advance_timestep(
                //    timestep);

                // calculate timestep (thread spawn method, parallel) 

                let heater_2_join_handle: JoinHandle<HeaterVersion2Bare> 
                = heater_v2_bare.advance_timestep_thread_spawn(
                    timestep);

                let heater_bottom_join_handle: JoinHandle<HeaterTopBottomHead> 
                = heater_bottom_head_bare. 
                    advance_timestep_thread_spawn(
                        timestep);

                let heater_top_head_join_handle = 
                heater_top_head_bare.advance_timestep_thread_spawn(
                    timestep);

                if connect_static_mixer_10 {
                    let static_mixer_join_handle = 
                    static_mixer_mx_10_object.advance_timestep_thread_spawn(
                        timestep);

                    let static_mixer_pipe_join_handle = 
                    static_mixer_mx_10_pipe.advance_timestep_thread_spawn(
                        timestep);
                    static_mixer_mx_10_object = static_mixer_join_handle.join().unwrap();
                    static_mixer_mx_10_pipe = static_mixer_pipe_join_handle.join().unwrap();

                }

                //let structural_support_heater_bottom_head_join_handle = 
                //structural_support_heater_bottom_head.
                //advance_timestep_mut_self_thread_spawn(timestep);

                //let structural_support_heater_top_head_join_handle = 
                //structural_support_heater_top_head.
                //advance_timestep_mut_self_thread_spawn(timestep);

                heater_v2_bare = heater_2_join_handle.join().unwrap();
                heater_bottom_head_bare = heater_bottom_join_handle.join().unwrap();
                heater_top_head_bare = heater_top_head_join_handle.join().unwrap();

                //structural_support_heater_bottom_head 
                //=  structural_support_heater_bottom_head_join_handle.join().unwrap();
                //structural_support_heater_top_head 
                //=  structural_support_heater_top_head_join_handle.join().unwrap();

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
