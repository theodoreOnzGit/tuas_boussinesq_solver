use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::boundary_conditions::BCType;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::pre_built_components::non_insulated_fluid_components::NonInsulatedFluidComponent;
use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
#[test]
pub fn heated_section_test_v2_3kw(){
    use std::time::SystemTime;
    use std::thread::JoinHandle;

    use uom::{si::{time::second, power::kilowatt}, ConstZero};

    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use uom::si::mass_rate::kilogram_per_second;

    // bare heater example
    let initial_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(78.75);
    let experimental_outlet_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(86.93);
    let regression_outlet_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(87.15);
    let inlet_temperature = initial_temperature;
    let ambient_air_temp: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(21.76);
    let heater_power = Power::new::<kilowatt>(3.0);

    let number_of_inner_temperature_nodes: usize = 8;

    let mut heater_v2_bare = NonInsulatedFluidComponent::new_dewet_model_heater_v2_no_twisted_tape(
        initial_temperature,
        ambient_air_temp,
        number_of_inner_temperature_nodes
    );


    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    // time settings 

    let max_time = Time::new::<second>(200.0);
    let timestep = Time::new::<second>(0.1);
    let mut simulation_time = Time::ZERO;
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);

    let mut simulated_outlet_temperature = 
        ThermodynamicTemperature::ZERO;


    // main loop

    while max_time > simulation_time {

        // time start 
        let loop_time_start = SystemTime::now();

        // create interactions 

        // inlet fluid density 

        let inlet_fluid_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                inlet_temperature).unwrap();

        // first node of heater fluid density 

        let therminol_array_clone: FluidArray 
            = heater_v2_bare.pipe_fluid_array.clone().try_into().unwrap();

        let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            therminol_array_clone.get_temperature_vector().unwrap();

        let steel_array_clone: SolidColumn 
            = heater_v2_bare.pipe_shell.clone().try_into().unwrap();

        let _steel_array_temperature: Vec<ThermodynamicTemperature> = 
            steel_array_clone.get_temperature_vector().unwrap();


        let back_cv_temperature: ThermodynamicTemperature = 
            therminol_array_temperature[0];

        let heated_section_exit_temperature: ThermodynamicTemperature = 
            *therminol_array_temperature.iter().last().unwrap();

        let back_cv_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                back_cv_temperature).unwrap();

        let front_cv_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                heated_section_exit_temperature).unwrap();

        // probably want to make this bit a little more user friendly
        let inlet_interaction: HeatTransferInteractionType = 
            HeatTransferInteractionType::new_advection_interaction(
                mass_flowrate,
                inlet_fluid_density,
                back_cv_density);

        let outlet_interaction = 
            HeatTransferInteractionType::new_advection_interaction(
                mass_flowrate,
                front_cv_density,
                front_cv_density,
            );

        // make axial connections to BCs 

        heater_v2_bare.pipe_fluid_array.link_to_back(
            &mut inlet_bc,
            inlet_interaction
        ).unwrap();

        heater_v2_bare.pipe_fluid_array.link_to_front(
            &mut outlet_bc,
            outlet_interaction
        ).unwrap();

        // make other connections
        //
        // this is the serial version
        //heater_v2_bare.lateral_and_miscellaneous_connections(
        //    mass_flowrate,
        //    heater_power
        //);

        // make other connections by spawning a new thread 
        // this is the parallel version
        let heater_2_join_handle: JoinHandle<NonInsulatedFluidComponent> 
            = heater_v2_bare.
            lateral_connection_thread_spawn(
                mass_flowrate,
                heater_power,);

        heater_v2_bare = heater_2_join_handle.join().unwrap();

        //// calculate timestep (serial method)
        //heater_v2_bare.advance_timestep(
        //    timestep);

        // calculate timestep (thread spawn method, parallel) 
        let heater_2_join_handle: JoinHandle<NonInsulatedFluidComponent> 
            = heater_v2_bare.advance_timestep_thread_spawn(
                timestep);

        heater_v2_bare = heater_2_join_handle.join().unwrap();

        simulation_time += timestep;

        let time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();

        // print outlet temperature 
        dbg!(heated_section_exit_temperature
            .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));

        simulated_outlet_temperature = heated_section_exit_temperature;

        // print loop time 
        dbg!(time_taken_for_calculation_loop);
    }

    // once simulation completed, assert outlet temperature to within 0.5K 
    // of experiment

    approx::assert_abs_diff_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        experimental_outlet_temperature.get::<degree_celsius>(),
        epsilon=0.5);

    // then assert the simulated outlet temperature to within 0.01K
    // regression temperature

    approx::assert_abs_diff_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        regression_outlet_temperature.get::<degree_celsius>(),
        epsilon=0.01);

    //todo!("haven't coded csv writing file")

}


#[test]
pub fn heated_section_test_v2_4kw(){
    use std::time::SystemTime;
    use std::thread::JoinHandle;

    use uom::{si::{time::second, power::kilowatt}, ConstZero};

    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use uom::si::mass_rate::kilogram_per_second;

    // bare heater example
    let initial_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(79.0);
    let experimental_outlet_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(90.25);
    let regression_outlet_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(90.40);
    let inlet_temperature = initial_temperature;
    let ambient_air_temp: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(21.76);
    let heater_power = Power::new::<kilowatt>(4.0);

    let number_of_inner_temperature_nodes: usize = 8;

    let mut heater_v2_bare = NonInsulatedFluidComponent::new_dewet_model_heater_v2_no_twisted_tape(
        initial_temperature,
        ambient_air_temp,
        number_of_inner_temperature_nodes
    );


    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    // time settings 

    let max_time = Time::new::<second>(200.0);
    let timestep = Time::new::<second>(0.1);
    let mut simulation_time = Time::ZERO;
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);

    let mut simulated_outlet_temperature = 
        ThermodynamicTemperature::ZERO;


    // main loop

    while max_time > simulation_time {

        // time start 
        let loop_time_start = SystemTime::now();

        // create interactions 

        // inlet fluid density 

        let inlet_fluid_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                inlet_temperature).unwrap();

        // first node of heater fluid density 

        let therminol_array_clone: FluidArray 
            = heater_v2_bare.pipe_fluid_array.clone().try_into().unwrap();

        let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            therminol_array_clone.get_temperature_vector().unwrap();

        let steel_array_clone: SolidColumn 
            = heater_v2_bare.pipe_shell.clone().try_into().unwrap();

        let _steel_array_temperature: Vec<ThermodynamicTemperature> = 
            steel_array_clone.get_temperature_vector().unwrap();


        let back_cv_temperature: ThermodynamicTemperature = 
            therminol_array_temperature[0];

        let heated_section_exit_temperature: ThermodynamicTemperature = 
            *therminol_array_temperature.iter().last().unwrap();

        let back_cv_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                back_cv_temperature).unwrap();

        let front_cv_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                heated_section_exit_temperature).unwrap();

        // probably want to make this bit a little more user friendly
        let inlet_interaction: HeatTransferInteractionType = 
            HeatTransferInteractionType::new_advection_interaction(
                mass_flowrate,
                inlet_fluid_density,
                back_cv_density);

        let outlet_interaction = 
            HeatTransferInteractionType::new_advection_interaction(
                mass_flowrate,
                front_cv_density,
                front_cv_density,
            );

        // make axial connections to BCs 

        heater_v2_bare.pipe_fluid_array.link_to_back(
            &mut inlet_bc,
            inlet_interaction
        ).unwrap();

        heater_v2_bare.pipe_fluid_array.link_to_front(
            &mut outlet_bc,
            outlet_interaction
        ).unwrap();

        // make other connections
        //
        // this is the serial version
        //heater_v2_bare.lateral_and_miscellaneous_connections(
        //    mass_flowrate,
        //    heater_power
        //);

        // make other connections by spawning a new thread 
        // this is the parallel version
        let heater_2_join_handle: JoinHandle<NonInsulatedFluidComponent> 
            = heater_v2_bare.
            lateral_connection_thread_spawn(
                mass_flowrate,
                heater_power,);

        heater_v2_bare = heater_2_join_handle.join().unwrap();

        //// calculate timestep (serial method)
        //heater_v2_bare.advance_timestep(
        //    timestep);

        // calculate timestep (thread spawn method, parallel) 
        let heater_2_join_handle: JoinHandle<NonInsulatedFluidComponent> 
            = heater_v2_bare.advance_timestep_thread_spawn(
                timestep);

        heater_v2_bare = heater_2_join_handle.join().unwrap();

        simulation_time += timestep;

        let time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();

        // print outlet temperature 
        dbg!(heated_section_exit_temperature
            .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));

        simulated_outlet_temperature = heated_section_exit_temperature;

        // print loop time 
        dbg!(time_taken_for_calculation_loop);
    }

    // once simulation completed, assert outlet temperature to within 0.5K 
    // of experiment

    approx::assert_abs_diff_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        experimental_outlet_temperature.get::<degree_celsius>(),
        epsilon=0.5);

    // then assert the simulated outlet temperature to within 0.01K
    // regression temperature

    approx::assert_abs_diff_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        regression_outlet_temperature.get::<degree_celsius>(),
        epsilon=0.01);

    //todo!("haven't coded csv writing file")

}


#[test]
pub fn heated_section_test_v2_6kw(){
    use std::time::SystemTime;
    use std::thread::JoinHandle;

    use uom::{si::{time::second, power::kilowatt}, ConstZero};

    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use uom::si::mass_rate::kilogram_per_second;

    // bare heater example
    let initial_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(79.4);
    let experimental_outlet_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(96.5);
    let regression_outlet_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(96.77);
    let inlet_temperature = initial_temperature;
    let ambient_air_temp: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(21.76);
    let heater_power = Power::new::<kilowatt>(6.0);

    let number_of_inner_temperature_nodes: usize = 8;

    let mut heater_v2_bare = NonInsulatedFluidComponent::new_dewet_model_heater_v2_no_twisted_tape(
        initial_temperature,
        ambient_air_temp,
        number_of_inner_temperature_nodes
    );


    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    // time settings 

    let max_time = Time::new::<second>(200.0);
    let timestep = Time::new::<second>(0.1);
    let mut simulation_time = Time::ZERO;
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);

    let mut simulated_outlet_temperature = 
        ThermodynamicTemperature::ZERO;


    // main loop

    while max_time > simulation_time {

        // time start 
        let loop_time_start = SystemTime::now();

        // create interactions 

        // inlet fluid density 

        let inlet_fluid_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                inlet_temperature).unwrap();

        // first node of heater fluid density 

        let therminol_array_clone: FluidArray 
            = heater_v2_bare.pipe_fluid_array.clone().try_into().unwrap();

        let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            therminol_array_clone.get_temperature_vector().unwrap();

        let steel_array_clone: SolidColumn 
            = heater_v2_bare.pipe_shell.clone().try_into().unwrap();

        let _steel_array_temperature: Vec<ThermodynamicTemperature> = 
            steel_array_clone.get_temperature_vector().unwrap();


        let back_cv_temperature: ThermodynamicTemperature = 
            therminol_array_temperature[0];

        let heated_section_exit_temperature: ThermodynamicTemperature = 
            *therminol_array_temperature.iter().last().unwrap();

        let back_cv_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                back_cv_temperature).unwrap();

        let front_cv_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                heated_section_exit_temperature).unwrap();

        // probably want to make this bit a little more user friendly
        let inlet_interaction: HeatTransferInteractionType = 
            HeatTransferInteractionType::new_advection_interaction(
                mass_flowrate,
                inlet_fluid_density,
                back_cv_density);

        let outlet_interaction = 
            HeatTransferInteractionType::new_advection_interaction(
                mass_flowrate,
                front_cv_density,
                front_cv_density,
            );

        // make axial connections to BCs 

        heater_v2_bare.pipe_fluid_array.link_to_back(
            &mut inlet_bc,
            inlet_interaction
        ).unwrap();

        heater_v2_bare.pipe_fluid_array.link_to_front(
            &mut outlet_bc,
            outlet_interaction
        ).unwrap();

        // make other connections
        //
        // this is the serial version
        //heater_v2_bare.lateral_and_miscellaneous_connections(
        //    mass_flowrate,
        //    heater_power
        //);

        // make other connections by spawning a new thread 
        // this is the parallel version
        let heater_2_join_handle: JoinHandle<NonInsulatedFluidComponent> 
            = heater_v2_bare.
            lateral_connection_thread_spawn(
                mass_flowrate,
                heater_power,);

        heater_v2_bare = heater_2_join_handle.join().unwrap();

        //// calculate timestep (serial method)
        //heater_v2_bare.advance_timestep(
        //    timestep);

        // calculate timestep (thread spawn method, parallel) 
        let heater_2_join_handle: JoinHandle<NonInsulatedFluidComponent> 
            = heater_v2_bare.advance_timestep_thread_spawn(
                timestep);

        heater_v2_bare = heater_2_join_handle.join().unwrap();

        simulation_time += timestep;

        let time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();

        // print outlet temperature 
        dbg!(heated_section_exit_temperature
            .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));

        simulated_outlet_temperature = heated_section_exit_temperature;

        // print loop time 
        dbg!(time_taken_for_calculation_loop);
    }

    // once simulation completed, assert outlet temperature to within 0.5K 
    // of experiment

    approx::assert_abs_diff_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        experimental_outlet_temperature.get::<degree_celsius>(),
        epsilon=0.5);

    // then assert the simulated outlet temperature to within 0.01K
    // regression temperature

    approx::assert_abs_diff_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        regression_outlet_temperature.get::<degree_celsius>(),
        epsilon=0.01);

    //todo!("haven't coded csv writing file")

}


#[test]
pub fn heated_section_test_v2_8kw(){
    use std::time::SystemTime;
    use std::thread::JoinHandle;

    use uom::{si::{time::second, power::kilowatt}, ConstZero};

    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use uom::si::mass_rate::kilogram_per_second;

    // bare heater example
    let initial_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(79.12);
    let experimental_outlet_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(102.2);
    let regression_outlet_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(102.44);
    let inlet_temperature = initial_temperature;
    let ambient_air_temp: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(21.76);
    let heater_power = Power::new::<kilowatt>(8.0);

    let number_of_inner_temperature_nodes: usize = 8;

    let mut heater_v2_bare = NonInsulatedFluidComponent::new_dewet_model_heater_v2_no_twisted_tape(
        initial_temperature,
        ambient_air_temp,
        number_of_inner_temperature_nodes
    );


    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    // time settings 

    let max_time = Time::new::<second>(200.0);
    let timestep = Time::new::<second>(0.1);
    let mut simulation_time = Time::ZERO;
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);

    let mut simulated_outlet_temperature = 
        ThermodynamicTemperature::ZERO;


    // main loop

    while max_time > simulation_time {

        // time start 
        let loop_time_start = SystemTime::now();

        // create interactions 

        // inlet fluid density 

        let inlet_fluid_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                inlet_temperature).unwrap();

        // first node of heater fluid density 

        let therminol_array_clone: FluidArray 
            = heater_v2_bare.pipe_fluid_array.clone().try_into().unwrap();

        let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            therminol_array_clone.get_temperature_vector().unwrap();

        let steel_array_clone: SolidColumn 
            = heater_v2_bare.pipe_shell.clone().try_into().unwrap();

        let _steel_array_temperature: Vec<ThermodynamicTemperature> = 
            steel_array_clone.get_temperature_vector().unwrap();


        let back_cv_temperature: ThermodynamicTemperature = 
            therminol_array_temperature[0];

        let heated_section_exit_temperature: ThermodynamicTemperature = 
            *therminol_array_temperature.iter().last().unwrap();

        let back_cv_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                back_cv_temperature).unwrap();

        let front_cv_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                heated_section_exit_temperature).unwrap();

        // probably want to make this bit a little more user friendly
        let inlet_interaction: HeatTransferInteractionType = 
            HeatTransferInteractionType::new_advection_interaction(
                mass_flowrate,
                inlet_fluid_density,
                back_cv_density);

        let outlet_interaction = 
            HeatTransferInteractionType::new_advection_interaction(
                mass_flowrate,
                front_cv_density,
                front_cv_density,
            );

        // make axial connections to BCs 

        heater_v2_bare.pipe_fluid_array.link_to_back(
            &mut inlet_bc,
            inlet_interaction
        ).unwrap();

        heater_v2_bare.pipe_fluid_array.link_to_front(
            &mut outlet_bc,
            outlet_interaction
        ).unwrap();

        // make other connections
        //
        // this is the serial version
        //heater_v2_bare.lateral_and_miscellaneous_connections(
        //    mass_flowrate,
        //    heater_power
        //);

        // make other connections by spawning a new thread 
        // this is the parallel version
        let heater_2_join_handle: JoinHandle<NonInsulatedFluidComponent> 
            = heater_v2_bare.
            lateral_connection_thread_spawn(
                mass_flowrate,
                heater_power,);

        heater_v2_bare = heater_2_join_handle.join().unwrap();

        //// calculate timestep (serial method)
        //heater_v2_bare.advance_timestep(
        //    timestep);

        // calculate timestep (thread spawn method, parallel) 
        let heater_2_join_handle: JoinHandle<NonInsulatedFluidComponent> 
            = heater_v2_bare.advance_timestep_thread_spawn(
                timestep);

        heater_v2_bare = heater_2_join_handle.join().unwrap();

        simulation_time += timestep;

        let time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();

        // print outlet temperature 
        dbg!(heated_section_exit_temperature
            .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));

        simulated_outlet_temperature = heated_section_exit_temperature;

        // print loop time 
        dbg!(time_taken_for_calculation_loop);
    }

    // once simulation completed, assert outlet temperature to within 0.5K 
    // of experiment

    approx::assert_abs_diff_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        experimental_outlet_temperature.get::<degree_celsius>(),
        epsilon=0.5);

    // then assert the simulated outlet temperature to within 0.01K
    // regression temperature

    approx::assert_abs_diff_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        regression_outlet_temperature.get::<degree_celsius>(),
        epsilon=0.01);

    //todo!("haven't coded csv writing file")

}
#[test]
pub fn heated_section_test_v2_10kw(){
    use std::time::SystemTime;
    use std::thread::JoinHandle;

    use uom::{si::{time::second, power::kilowatt}, ConstZero};

    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use uom::si::mass_rate::kilogram_per_second;

    // bare heater example
    let initial_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(78.9);
    let experimental_outlet_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(107.75);
    let regression_outlet_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(108.12);
    let inlet_temperature = initial_temperature;
    let ambient_air_temp: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(21.76);
    let heater_power = Power::new::<kilowatt>(10.0);

    let number_of_inner_temperature_nodes: usize = 8;

    let mut heater_v2_bare = NonInsulatedFluidComponent::new_dewet_model_heater_v2_no_twisted_tape(
        initial_temperature,
        ambient_air_temp,
        number_of_inner_temperature_nodes
    );


    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    // time settings 

    let max_time = Time::new::<second>(200.0);
    let timestep = Time::new::<second>(0.1);
    let mut simulation_time = Time::ZERO;
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);

    let mut simulated_outlet_temperature = 
        ThermodynamicTemperature::ZERO;


    // main loop

    while max_time > simulation_time {

        // time start 
        let loop_time_start = SystemTime::now();

        // create interactions 

        // inlet fluid density 

        let inlet_fluid_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                inlet_temperature).unwrap();

        // first node of heater fluid density 

        let therminol_array_clone: FluidArray 
            = heater_v2_bare.pipe_fluid_array.clone().try_into().unwrap();

        let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            therminol_array_clone.get_temperature_vector().unwrap();

        let steel_array_clone: SolidColumn 
            = heater_v2_bare.pipe_shell.clone().try_into().unwrap();

        let _steel_array_temperature: Vec<ThermodynamicTemperature> = 
            steel_array_clone.get_temperature_vector().unwrap();


        let back_cv_temperature: ThermodynamicTemperature = 
            therminol_array_temperature[0];

        let heated_section_exit_temperature: ThermodynamicTemperature = 
            *therminol_array_temperature.iter().last().unwrap();

        let back_cv_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                back_cv_temperature).unwrap();

        let front_cv_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                heated_section_exit_temperature).unwrap();

        // probably want to make this bit a little more user friendly
        let inlet_interaction: HeatTransferInteractionType = 
            HeatTransferInteractionType::new_advection_interaction(
                mass_flowrate,
                inlet_fluid_density,
                back_cv_density);

        let outlet_interaction = 
            HeatTransferInteractionType::new_advection_interaction(
                mass_flowrate,
                front_cv_density,
                front_cv_density,
            );

        // make axial connections to BCs 

        heater_v2_bare.pipe_fluid_array.link_to_back(
            &mut inlet_bc,
            inlet_interaction
        ).unwrap();

        heater_v2_bare.pipe_fluid_array.link_to_front(
            &mut outlet_bc,
            outlet_interaction
        ).unwrap();

        // make other connections
        //
        // this is the serial version
        //heater_v2_bare.lateral_and_miscellaneous_connections(
        //    mass_flowrate,
        //    heater_power
        //);

        // make other connections by spawning a new thread 
        // this is the parallel version
        let heater_2_join_handle: JoinHandle<NonInsulatedFluidComponent> 
            = heater_v2_bare.
            lateral_connection_thread_spawn(
                mass_flowrate,
                heater_power,);

        heater_v2_bare = heater_2_join_handle.join().unwrap();

        //// calculate timestep (serial method)
        //heater_v2_bare.advance_timestep(
        //    timestep);

        // calculate timestep (thread spawn method, parallel) 
        let heater_2_join_handle: JoinHandle<NonInsulatedFluidComponent> 
            = heater_v2_bare.advance_timestep_thread_spawn(
                timestep);

        heater_v2_bare = heater_2_join_handle.join().unwrap();

        simulation_time += timestep;

        let time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();

        // print outlet temperature 
        dbg!(heated_section_exit_temperature
            .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));

        simulated_outlet_temperature = heated_section_exit_temperature;

        // print loop time 
        dbg!(time_taken_for_calculation_loop);
    }

    // once simulation completed, assert outlet temperature to within 0.5K 
    // of experiment

    approx::assert_abs_diff_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        experimental_outlet_temperature.get::<degree_celsius>(),
        epsilon=0.5);

    // then assert the simulated outlet temperature to within 0.01K
    // regression temperature

    approx::assert_abs_diff_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        regression_outlet_temperature.get::<degree_celsius>(),
        epsilon=0.01);

    //todo!("haven't coded csv writing file")

}

