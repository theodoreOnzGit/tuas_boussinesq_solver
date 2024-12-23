
use uom::si::thermal_conductance::watt_per_kelvin;

use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::boundary_conditions::BCType;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::pre_built_components::non_insulated_porous_media_fluid_components::NonInsulatedPorousMediaFluidComponent;
use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
#[test]
//#[ignore = "debugging"]
pub fn example_heated_section_regression_new_and_old(){
    use std::time::SystemTime;

    use uom::{si::{time::second, power::kilowatt}, ConstZero};

    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use uom::si::mass_rate::kilogram_per_second;

    // bare heater example
    let initial_temperature: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(79.12);
    let inlet_temperature = initial_temperature;
    let ambient_air_temp: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(21.76);

    let number_of_temperature_nodes: usize = 8;
    
    let mut heater_v2_bare_original = NonInsulatedPorousMediaFluidComponent::new_dewet_model_heater_v2(
        initial_temperature,
        ambient_air_temp,
        number_of_temperature_nodes
    );

    // I'm cloning this heater v2 bare so as to test the new advancing 
    // timestep and such
    let mut heater_v2_bare_new_code = 
        heater_v2_bare_original.clone();

    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    // time settings 

    let max_time = Time::new::<second>(100.0);
    let timestep = Time::new::<second>(0.3);
    let mut simulation_time = Time::ZERO;
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);
    let heater_power = Power::new::<kilowatt>(8.0);

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
        = heater_v2_bare_original.pipe_fluid_array.clone().try_into().unwrap();

        let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
        therminol_array_clone.get_temperature_vector().unwrap();


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

        heater_v2_bare_original.pipe_fluid_array.link_to_back(
            &mut inlet_bc,
            inlet_interaction
        ).unwrap();

        heater_v2_bare_original.pipe_fluid_array.link_to_front(
            &mut outlet_bc,
            outlet_interaction
        ).unwrap();

        // now axial connections to heater v2 bare the new one 

        heater_v2_bare_new_code.pipe_fluid_array.link_to_back(
            &mut inlet_bc, 
            inlet_interaction
        ).unwrap();

        heater_v2_bare_new_code.pipe_fluid_array.link_to_front(
            &mut outlet_bc, 
            outlet_interaction
        ).unwrap();
            

        // make other connections first for the old heater
        //
        // this is the serial version
        //heater_v2_bare.lateral_and_miscellaneous_connections(
        //    mass_flowrate,
        //    heater_power
        //);

        // make other connections by spawning a new thread 
        // this is the parallel version
        heater_v2_bare_original.
            ciet_heater_v2_lateral_and_miscellaneous_connections(
                mass_flowrate,
                heater_power);


        heater_v2_bare_original.advance_timestep(timestep);

        let twisted_tape_power = Power::ZERO;


        // 
        // note that prandtl wall correction is switched off in the 
        // original setup, I'm going to do the same
        let prandtl_wall_correction_setting = false;
        heater_v2_bare_new_code.lateral_and_miscellaneous_connections(
            prandtl_wall_correction_setting, 
            mass_flowrate,
            heater_power,
            twisted_tape_power)
            .unwrap();

        // then advance timestep 
        heater_v2_bare_new_code.advance_timestep(timestep);


        // print outlet temperature 
        dbg!(heated_section_exit_temperature
        .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));




        //// print surface temperature 
        //dbg!("Steel array Temp: ", steel_array_temperature);

        //// print therminol temperature 
        //dbg!("Therminol Array Temp: ", therminol_array_temperature);

        //// print twisted tape temperature 
        //dbg!("twisted tape Temp: 
        //note: conduction occurs, so first node is hotter\n 
        //than the therminol fluid", twisted_tape_temperature);

        // print loop time 
        simulation_time += timestep;

        let time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();
        dbg!(time_taken_for_calculation_loop);
    }

    // once simulation completed, write data

    // assert that outlet temperatures are the SAME for both 

    let heater_v2_bare_original_temp_vector = 
        heater_v2_bare_original
        .pipe_fluid_array
        .get_temperature_vector()
        .unwrap();

    let heater_v2_bare_new_code_temp_vector = 
        heater_v2_bare_new_code
        .pipe_fluid_array
        .get_temperature_vector()
        .unwrap();

    let heater_v2_bare_outlet_temp_original = 
        heater_v2_bare_original_temp_vector
        .iter()
        .last()
        .unwrap();

    let heater_v2_bare_outlet_temp_new_code = 
        heater_v2_bare_new_code_temp_vector
        .iter()
        .last()
        .unwrap();

    // assert that both temperatures are equal to within
    // 1e-2 (1%) at every timestep
    approx::assert_relative_eq!(
        heater_v2_bare_outlet_temp_original.get::<degree_celsius>(),
        heater_v2_bare_outlet_temp_new_code.get::<degree_celsius>(),
        max_relative=1e-2
    );

    //todo!("haven't coded csv writing file")

}


#[test]
pub fn regression_new_and_old_nodal_conductance_steel_shell_to_ambient(){
    use std::time::SystemTime;

    use uom::{si::time::second, ConstZero};

    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use uom::si::mass_rate::kilogram_per_second;

    // bare heater example
    let initial_temperature: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(79.12);
    let inlet_temperature = initial_temperature;
    let ambient_air_temp: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(21.76);

    let number_of_temperature_nodes: usize = 8;
    
    let mut heater_v2_bare_original = NonInsulatedPorousMediaFluidComponent::new_dewet_model_heater_v2(
        initial_temperature,
        ambient_air_temp,
        number_of_temperature_nodes
    );

    // I'm cloning this heater v2 bare so as to test the new advancing 
    // timestep and such
    let mut heater_v2_bare_new_code = 
        heater_v2_bare_original.clone();

    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    // time settings 

    let timestep = Time::new::<second>(0.3);
    let mut simulation_time = Time::ZERO;
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);

    // main loop


    // time start 
    let loop_time_start = SystemTime::now();

    // create interactions 

    // inlet fluid density 

    let inlet_fluid_density: MassDensity = 
        LiquidMaterial::TherminolVP1.try_get_density(
            inlet_temperature).unwrap();

    // first node of heater fluid density 

    let therminol_array_clone: FluidArray 
        = heater_v2_bare_original.pipe_fluid_array.clone().try_into().unwrap();

    let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
        therminol_array_clone.get_temperature_vector().unwrap();


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

    heater_v2_bare_original.pipe_fluid_array.link_to_back(
        &mut inlet_bc,
        inlet_interaction
    ).unwrap();

    heater_v2_bare_original.pipe_fluid_array.link_to_front(
        &mut outlet_bc,
        outlet_interaction
    ).unwrap();

    // now axial connections to heater v2 bare the new one 

    heater_v2_bare_new_code.pipe_fluid_array.link_to_back(
        &mut inlet_bc, 
        inlet_interaction
    ).unwrap();

    heater_v2_bare_new_code.pipe_fluid_array.link_to_front(
        &mut outlet_bc, 
        outlet_interaction
    ).unwrap();

    // now let's compare bit by bit the conductances 
    //

    // ambient to shell 
    {

        let heat_transfer_to_air_original 
            = heater_v2_bare_original.heat_transfer_to_ambient;

        let heat_transfer_to_air_new_code 
            = heater_v2_bare_new_code.heat_transfer_to_ambient;

        let ambient_conductance_original  
            = heater_v2_bare_original.ciet_heater_v2_get_air_steel_nodal_shell_conductance(
                heat_transfer_to_air_original);

        let ambient_conductance_new_code  
            = heater_v2_bare_new_code.get_ambient_to_pipe_shell_nodal_conductance(
                heat_transfer_to_air_new_code).unwrap();


        assert_eq!(
            ambient_conductance_original
            ,ambient_conductance_new_code
        );
    }




    // print outlet temperature 
    dbg!(heated_section_exit_temperature
        .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));




    //// print surface temperature 
    //dbg!("Steel array Temp: ", steel_array_temperature);

    //// print therminol temperature 
    //dbg!("Therminol Array Temp: ", therminol_array_temperature);

    //// print twisted tape temperature 
    //dbg!("twisted tape Temp: 
    //note: conduction occurs, so first node is hotter\n 
    //than the therminol fluid", twisted_tape_temperature);

    // print loop time 
    simulation_time += timestep;

    let time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();
    dbg!(time_taken_for_calculation_loop);


}


#[test]
pub fn regression_new_and_old_nodal_conductance_steel_shell_to_pipe_fluid_array(){
    use std::time::SystemTime;

    use uom::{si::time::second, ConstZero};

    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use uom::si::mass_rate::kilogram_per_second;

    // bare heater example
    let initial_temperature: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(79.12);
    let inlet_temperature = initial_temperature;
    let ambient_air_temp: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(21.76);

    let number_of_temperature_nodes: usize = 8;
    
    let mut heater_v2_bare_original = NonInsulatedPorousMediaFluidComponent::new_dewet_model_heater_v2(
        initial_temperature,
        ambient_air_temp,
        number_of_temperature_nodes
    );


    // I'm cloning this heater v2 bare so as to test the new advancing 
    // timestep and such
    let mut heater_v2_bare_new_code = 
        heater_v2_bare_original.clone();

    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    // time settings 

    let timestep = Time::new::<second>(0.3);
    let mut simulation_time = Time::ZERO;
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);

    // main loop


    // time start 
    let loop_time_start = SystemTime::now();

    // create interactions 

    // inlet fluid density 

    let inlet_fluid_density: MassDensity = 
        LiquidMaterial::TherminolVP1.try_get_density(
            inlet_temperature).unwrap();

    // first node of heater fluid density 

    let therminol_array_clone: FluidArray 
        = heater_v2_bare_original.pipe_fluid_array.clone().try_into().unwrap();

    let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
        therminol_array_clone.get_temperature_vector().unwrap();


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

    heater_v2_bare_original.pipe_fluid_array.link_to_back(
        &mut inlet_bc,
        inlet_interaction
    ).unwrap();

    heater_v2_bare_original.pipe_fluid_array.link_to_front(
        &mut outlet_bc,
        outlet_interaction
    ).unwrap();

    // now axial connections to heater v2 bare the new one 

    heater_v2_bare_new_code.pipe_fluid_array.link_to_back(
        &mut inlet_bc, 
        inlet_interaction
    ).unwrap();

    heater_v2_bare_new_code.pipe_fluid_array.link_to_front(
        &mut outlet_bc, 
        outlet_interaction
    ).unwrap();

    // now let's compare bit by bit the conductances 
    //

    // shell to fluid
    {

        // set mass flowrate first 

        heater_v2_bare_original.set_mass_flowrate(mass_flowrate);
        heater_v2_bare_new_code.set_mass_flowrate(mass_flowrate);

        let pipe_shell_to_fluid_conductance_original  
            = heater_v2_bare_original
            .ciet_heater_v2_get_therminol_node_steel_shell_conductance();

        let prandtl_wall_correction_setting = false;
        let pipe_shell_to_fluid_conductance_new_code  
            = heater_v2_bare_new_code.get_pipe_shell_to_fluid_nodal_conductance(
                prandtl_wall_correction_setting).unwrap();


        approx::assert_relative_eq!(
            pipe_shell_to_fluid_conductance_original.get::<watt_per_kelvin>(),
            pipe_shell_to_fluid_conductance_new_code.get::<watt_per_kelvin>(),
            max_relative=1e-2
        );
    }




    // print outlet temperature 
    dbg!(heated_section_exit_temperature
        .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));




    //// print surface temperature 
    //dbg!("Steel array Temp: ", steel_array_temperature);

    //// print therminol temperature 
    //dbg!("Therminol Array Temp: ", therminol_array_temperature);

    //// print twisted tape temperature 
    //dbg!("twisted tape Temp: 
    //note: conduction occurs, so first node is hotter\n 
    //than the therminol fluid", twisted_tape_temperature);

    // print loop time 
    simulation_time += timestep;

    let time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();
    dbg!(time_taken_for_calculation_loop);


}
