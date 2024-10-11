#[test]
pub fn test_inclusive_heater_top_bottom_head(){
    use core::time;
    use std::{time::SystemTime, thread::{JoinHandle, self}};

    use uom::{si::{time::second, power::kilowatt}, ConstZero};

    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use crate::prelude::beta_testing::*;

    use uom::si::mass_rate::kilogram_per_second;


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

            let mut therminol_array_clone: FluidArray 
            = heater_v2_bare.therminol_array.clone().try_into().unwrap();


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
                &mut outlet_bc,
                generic_advection_interaction
            ).unwrap();
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



                heater_v2_bare = heater_2_join_handle.join().unwrap();
                heater_bottom_head_bare = heater_bottom_join_handle.join().unwrap();
                heater_top_head_bare = heater_top_head_join_handle.join().unwrap();

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
