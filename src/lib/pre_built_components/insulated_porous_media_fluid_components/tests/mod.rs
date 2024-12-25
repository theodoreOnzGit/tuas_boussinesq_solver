use uom::si::length::meter;

use crate::pre_built_components::ciet_isothermal_test_components::new_heated_section_version_1_label_1_without_inner_annular_pipe;

#[test]
pub fn transient_test_for_heater_v1(){
    unimplemented!()
}
#[test]
pub fn steady_state_test_for_heater_v1(){
    unimplemented!()
}
/// will probably use insulated pipes as a reference for this
/// possibly the heater v1 without internal
#[test]
pub fn test_to_assert_if_conductance_from_insulation_to_ambient_is_correct(){

    use uom::si::thermal_conductance::watt_per_kelvin;

    use crate::pre_built_components::insulated_porous_media_fluid_components::InsulatedPorousMediaFluidComponent;
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
    use crate::boundary_conditions::BCType;
    use crate::boussinesq_thermophysical_properties::LiquidMaterial;
    use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
    use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
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

    let number_of_inner_temperature_nodes: usize = 15-2;
    

    let mut heater_v1_without_inner_annular_pipe = 
        new_heated_section_version_1_label_1_without_inner_annular_pipe(
            initial_temperature);

    let mut heater_v1_with_annular_pipe = 
        InsulatedPorousMediaFluidComponent::new_ciet_heater_v1_with_annular_pipe(
        initial_temperature, 
        ambient_air_temp, 
        number_of_inner_temperature_nodes
    );



    // I'm cloning this heater v2 bare so as to test the new advancing 
    // timestep and such

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
        = heater_v1_without_inner_annular_pipe.pipe_fluid_array.clone().try_into().unwrap();

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


    // now axial connections to heater v2 bare 

    heater_v1_without_inner_annular_pipe.pipe_fluid_array.link_to_back(
        &mut inlet_bc, 
        inlet_interaction
    ).unwrap();

    heater_v1_without_inner_annular_pipe.pipe_fluid_array.link_to_front(
        &mut outlet_bc, 
        outlet_interaction
    ).unwrap();

    // then for the insulated version 
    heater_v1_with_annular_pipe.pipe_fluid_array.link_to_back(
        &mut inlet_bc, 
        inlet_interaction
    ).unwrap();

    heater_v1_with_annular_pipe.pipe_fluid_array.link_to_front(
        &mut outlet_bc, 
        outlet_interaction
    ).unwrap();
    

    // now let's compare bit by bit the conductances 
    //

    // shell to fluid
    {

        let htc_to_ambient = heater_v1_without_inner_annular_pipe.heat_transfer_to_ambient;
        // heater v2 bare first

        heater_v1_without_inner_annular_pipe.set_mass_flowrate(mass_flowrate);

        

        let ambient_to_insulation_conductance_without_inner_annular_pipe  
            = heater_v1_without_inner_annular_pipe
            .get_ambient_surroundings_to_insulation_thermal_conductance(
                htc_to_ambient).unwrap();
        // heater v2 insulated next

        heater_v1_with_annular_pipe.set_mass_flowrate(mass_flowrate);


        let ambient_to_insulation_conductance_insulated  
            = heater_v1_with_annular_pipe
            .get_ambient_to_insulation_nodal_conductance(
                htc_to_ambient).unwrap();


        // at the initial state, both must be the same
        approx::assert_relative_eq!(
            ambient_to_insulation_conductance_without_inner_annular_pipe.get::<watt_per_kelvin>(),
            ambient_to_insulation_conductance_insulated.get::<watt_per_kelvin>(),
            max_relative=1e-5
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

/// will probably use insulated pipes as a reference for this
/// possibly the heater v1 without internal
#[test]
pub fn regression_heater_v1_porous_and_non_porous_conductance_pipe_to_insulation_arr(){

    use uom::si::thermal_conductance::watt_per_kelvin;

    use crate::pre_built_components::insulated_porous_media_fluid_components::InsulatedPorousMediaFluidComponent;
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
    use crate::boundary_conditions::BCType;
    use crate::boussinesq_thermophysical_properties::LiquidMaterial;
    use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
    use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
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

    let number_of_inner_temperature_nodes: usize = 15-2;
    

    let mut heater_v1_without_inner_annular_pipe = 
        new_heated_section_version_1_label_1_without_inner_annular_pipe(
            initial_temperature);

    let mut heater_v1_with_annular_pipe = 
        InsulatedPorousMediaFluidComponent::new_ciet_heater_v1_with_annular_pipe(
        initial_temperature, 
        ambient_air_temp, 
        number_of_inner_temperature_nodes
    );



    // I'm cloning this heater v2 bare so as to test the new advancing 
    // timestep and such

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
        = heater_v1_without_inner_annular_pipe.pipe_fluid_array.clone().try_into().unwrap();

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


    // now axial connections to heater v2 bare 

    heater_v1_without_inner_annular_pipe.pipe_fluid_array.link_to_back(
        &mut inlet_bc, 
        inlet_interaction
    ).unwrap();

    heater_v1_without_inner_annular_pipe.pipe_fluid_array.link_to_front(
        &mut outlet_bc, 
        outlet_interaction
    ).unwrap();

    // then for the insulated version 
    heater_v1_with_annular_pipe.pipe_fluid_array.link_to_back(
        &mut inlet_bc, 
        inlet_interaction
    ).unwrap();

    heater_v1_with_annular_pipe.pipe_fluid_array.link_to_front(
        &mut outlet_bc, 
        outlet_interaction
    ).unwrap();
    

    // now let's compare bit by bit the conductances 
    //

    // shell to fluid
    {

        // heater v2 bare first

        heater_v1_without_inner_annular_pipe.set_mass_flowrate(mass_flowrate);


        let pipe_shell_to_insulation_conductance_without_inner_annular_pipe  
            = heater_v1_without_inner_annular_pipe
            .get_pipe_shell_to_insulation_nodal_conductance().unwrap();
        // heater v2 insulated next

        heater_v1_with_annular_pipe.set_mass_flowrate(mass_flowrate);


        let pipe_shell_to_insulation_conductance_insulated  
            = heater_v1_with_annular_pipe
            .get_pipe_shell_to_insulation_nodal_conductance().unwrap();


        // at the initial state, both must be the same
        approx::assert_relative_eq!(
            pipe_shell_to_insulation_conductance_without_inner_annular_pipe.get::<watt_per_kelvin>(),
            pipe_shell_to_insulation_conductance_insulated.get::<watt_per_kelvin>(),
            max_relative=1e-5
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
pub fn assert_if_hydraulic_diameter_of_heater_v1_is_correct(){


    use crate::pre_built_components::insulated_porous_media_fluid_components::InsulatedPorousMediaFluidComponent;
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::degree_celsius;


    // heater v1 hydraulic diameter should be about 6.60e-3m
    // assert in unit test
    let reference_hydraulic_diameter = Length::new::<meter>(6.60e-3);
    let initial_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(79.12);
    let ambient_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(21.76);

    let user_specified_inner_nodes = 15-2;
    let mut ciet_heater_v1 = 
        InsulatedPorousMediaFluidComponent::new_ciet_heater_v1_with_annular_pipe(
            initial_temperature, 
            ambient_temperature, 
            user_specified_inner_nodes);

    let test_hydraulic_diameter = ciet_heater_v1.get_hydraulic_diameter();

    approx::assert_relative_eq!(
        reference_hydraulic_diameter.get::<meter>(),
        test_hydraulic_diameter.get::<meter>(),
        max_relative=0.01
    );

}

/// for heater v2, with insulation slapped on it, and the heater v2 without 
/// insulation, conductance from pipe to the fluid should be the same
#[test] 
pub fn regression_heater_v2_insulated_and_non_insulated_conductance_fluid_arr_to_pipe(){

    use uom::si::thermal_conductance::watt_per_kelvin;

    use crate::pre_built_components::insulated_porous_media_fluid_components::InsulatedPorousMediaFluidComponent;
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
    use crate::boundary_conditions::BCType;
    use crate::boussinesq_thermophysical_properties::LiquidMaterial;
    use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
    use crate::pre_built_components::non_insulated_porous_media_fluid_components::NonInsulatedPorousMediaFluidComponent;
    use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
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
    
    let mut heater_v2_bare = NonInsulatedPorousMediaFluidComponent::new_dewet_model_heater_v2(
        initial_temperature,
        ambient_air_temp,
        number_of_temperature_nodes
    );

    let mut heater_v2_insulated = InsulatedPorousMediaFluidComponent::new_dewet_model_heater_v2_insulated(
        initial_temperature, 
        ambient_air_temp, 
        number_of_temperature_nodes
    );



    // I'm cloning this heater v2 bare so as to test the new advancing 
    // timestep and such

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
        = heater_v2_bare.pipe_fluid_array.clone().try_into().unwrap();

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


    // now axial connections to heater v2 bare 

    heater_v2_bare.pipe_fluid_array.link_to_back(
        &mut inlet_bc, 
        inlet_interaction
    ).unwrap();

    heater_v2_bare.pipe_fluid_array.link_to_front(
        &mut outlet_bc, 
        outlet_interaction
    ).unwrap();

    // then for the insulated version 
    heater_v2_insulated.pipe_fluid_array.link_to_back(
        &mut inlet_bc, 
        inlet_interaction
    ).unwrap();

    heater_v2_insulated.pipe_fluid_array.link_to_front(
        &mut outlet_bc, 
        outlet_interaction
    ).unwrap();
    

    // now let's compare bit by bit the conductances 
    //

    // shell to fluid
    {

        // heater v2 bare first

        heater_v2_bare.set_mass_flowrate(mass_flowrate);


        let prandtl_wall_correction_setting = false;
        let pipe_shell_to_fluid_conductance_bare  
            = heater_v2_bare.get_pipe_shell_to_fluid_nodal_conductance(
                prandtl_wall_correction_setting).unwrap();
        // heater v2 insulated next

        heater_v2_insulated.set_mass_flowrate(mass_flowrate);


        let prandtl_wall_correction_setting = false;
        let pipe_shell_to_fluid_conductance_insulated  
            = heater_v2_insulated.get_pipe_shell_to_fluid_nodal_conductance(
                prandtl_wall_correction_setting).unwrap();


        // at the initial state, both must be the same
        approx::assert_relative_eq!(
            pipe_shell_to_fluid_conductance_bare.get::<watt_per_kelvin>(),
            pipe_shell_to_fluid_conductance_insulated.get::<watt_per_kelvin>(),
            max_relative=1e-5
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


/// for heater v2, with insulation slapped on it, and the heater v2 without 
/// insulation, conductance from twisted tape to the fluid should be the same
#[test]
pub fn regression_heater_v2_insulated_and_non_insulated_conductance_twisted_tape_to_pipe_fluid_array(){

    use uom::si::thermal_conductance::watt_per_kelvin;

    use crate::pre_built_components::insulated_porous_media_fluid_components::InsulatedPorousMediaFluidComponent;
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
    use crate::boundary_conditions::BCType;
    use crate::boussinesq_thermophysical_properties::LiquidMaterial;
    use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
    use crate::pre_built_components::non_insulated_porous_media_fluid_components::NonInsulatedPorousMediaFluidComponent;
    use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
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
    
    let mut heater_v2_bare = NonInsulatedPorousMediaFluidComponent::new_dewet_model_heater_v2(
        initial_temperature,
        ambient_air_temp,
        number_of_temperature_nodes
    );

    let mut heater_v2_insulated = InsulatedPorousMediaFluidComponent::new_dewet_model_heater_v2_insulated(
        initial_temperature, 
        ambient_air_temp, 
        number_of_temperature_nodes
    );



    // I'm cloning this heater v2 bare so as to test the new advancing 
    // timestep and such

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
        = heater_v2_bare.pipe_fluid_array.clone().try_into().unwrap();

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


    // now axial connections to heater v2 bare 

    heater_v2_bare.pipe_fluid_array.link_to_back(
        &mut inlet_bc, 
        inlet_interaction
    ).unwrap();

    heater_v2_bare.pipe_fluid_array.link_to_front(
        &mut outlet_bc, 
        outlet_interaction
    ).unwrap();

    // then for the insulated version 
    heater_v2_insulated.pipe_fluid_array.link_to_back(
        &mut inlet_bc, 
        inlet_interaction
    ).unwrap();

    heater_v2_insulated.pipe_fluid_array.link_to_front(
        &mut outlet_bc, 
        outlet_interaction
    ).unwrap();
    

    // now let's compare bit by bit the conductances 
    //

    // shell to fluid
    {

        // heater v2 bare first

        heater_v2_bare.set_mass_flowrate(mass_flowrate);


        let prandtl_wall_correction_setting = false;
        let twisted_tape_to_fluid_conductance_bare  
            = heater_v2_bare.get_interior_to_fluid_nodal_conductance(
                prandtl_wall_correction_setting).unwrap();
        // heater v2 insulated next

        heater_v2_insulated.set_mass_flowrate(mass_flowrate);


        let prandtl_wall_correction_setting = false;
        let twisted_tape_to_fluid_conductance_insulated  
            = heater_v2_insulated.get_interior_to_fluid_nodal_conductance(
                prandtl_wall_correction_setting).unwrap();


        // at the initial state, both must be the same
        approx::assert_relative_eq!(
            twisted_tape_to_fluid_conductance_bare.get::<watt_per_kelvin>(),
            twisted_tape_to_fluid_conductance_insulated.get::<watt_per_kelvin>(),
            max_relative=1e-5
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
