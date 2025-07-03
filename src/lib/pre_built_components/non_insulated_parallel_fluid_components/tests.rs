

/// for coupled dracs loop, first thing first is to 
/// debug parallel bare pipes 
///
/// first thing first, heat addition 
///
/// suppose we have 9 kW of heat into one pipe 
/// at 0.18 kg/s TherminolVP1
///
/// whether you have the flow split into 1 single pipe or 10 parallel 
/// pipes should not matter. After a set time, the temperature should 
/// be the same. presuming of course, no heat loss to environment
///
/// I'm taking dhx tube side 30b as a reference
/// but setting heat transfer to ambient as 0
///
/// then I'll do the parallel tube setup, and add 20 tubes. 
/// the mass flow and heater power is increased by 20 times. 
///
/// the heating time is 15 s long, both outlet temperatures should be 
/// the same
///
#[test]
pub fn parallel_bare_pipes_debugging_heat_addition(){

    use uom::si::f64::*;
    use uom::si::ratio::ratio;
    use uom::si::length::{meter, millimeter};
    use uom::si::area::square_meter;
    use uom::si::angle::degree;

    use crate::pre_built_components::
        non_insulated_fluid_components::NonInsulatedFluidComponent;
    use crate::heat_transfer_correlations::
        nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::boussinesq_thermophysical_properties::SolidMaterial;
    use crate::
        array_control_vol_and_fluid_component_collections::
        one_d_fluid_array_with_lateral_coupling::FluidArray;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use uom::si::power::kilowatt;
    use uom::ConstZero;
    use uom::si::time::second;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::pressure::atmosphere;

    use crate::boundary_conditions::BCType;
    use crate::
        boussinesq_thermophysical_properties::LiquidMaterial;
    use crate::heat_transfer_correlations::
        heat_transfer_interactions::heat_transfer_interaction_enums::
        HeatTransferInteractionType;
    use crate::pre_built_components::
        heat_transfer_entities::HeatTransferEntity;

    use crate::pre_built_components::
        non_insulated_parallel_fluid_components::
        NonInsulatedParallelFluidComponent;
    // conditions 
    let initial_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(25.0);


    let htc_to_ambient_zero = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(0.0);


    // parameters for new_isolated_dhx_tube_side_30
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    // for dhx modelling,
    //
    // dhx tubes are modelled in SAM as 19 tubes of diameter 
    // 0.00635 m 
    // and flow area of 6.1072e-4 m^2
    //
    // in Zweibaum's RELAP model,
    // it is quite different from the SAM model 
    // which follows Bickel's data
    let hydraulic_diameter = Length::new::<meter>(6.35e-3);
    let pipe_length = Length::new::<meter>(1.18745);
    let flow_area = Area::new::<square_meter>(6.0172e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(3.3);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.00079375);
    let od = id + pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 11 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 11-2
    let user_specified_inner_nodes = 11-2; 



    let mut adiabatic_dhx_tube_side_30 = 
        NonInsulatedFluidComponent::new_bare_pipe(
            initial_temperature, 
            ambient_temperature, 
            fluid_pressure, 
            solid_pressure, 
            flow_area, 
            incline_angle, 
            form_loss, 
            id, 
            od, 
            pipe_length, 
            hydraulic_diameter, 
            surface_roughness, 
            pipe_shell_material, 
            pipe_fluid, 
            htc_to_ambient, 
            user_specified_inner_nodes);

    // for heat exchangers, I give an ideal Nusselt number correlation 
    // as an approximation so that film thermal resistance is minimised
    let mut fluid_array_ideal_nusslet: FluidArray = 
        adiabatic_dhx_tube_side_30.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_ideal_nusslet.nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    adiabatic_dhx_tube_side_30.pipe_fluid_array = 
        fluid_array_ideal_nusslet.into();

    adiabatic_dhx_tube_side_30.heat_transfer_to_ambient = 
        htc_to_ambient_zero;

    // now let's do a simple loop to check temperature after 15s
    let max_time = Time::new::<second>(15.0);
    let timestep = Time::new::<second>(0.01);
    let mut simulation_time = Time::ZERO;
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);
    let heater_power = Power::new::<kilowatt>(9.0);

    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        initial_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    let mut adiabatic_dhx_tube_side_30_outlet_temp = 
        ThermodynamicTemperature::ZERO;

    // main loop for single tube
    
    while max_time > simulation_time {

        // get average temperature and advection first 
        // we use boussinesq approximation, so the average temperature 
        // for density doesn't change
        // i'll just arbitrarily do 50 C
        let average_temperature_for_density_calcs = 
            ThermodynamicTemperature::
            new::<degree_celsius>(50.0);

        let average_therminol_density = 
            LiquidMaterial::TherminolVP1.try_get_density(
                average_temperature_for_density_calcs).unwrap();

        let advection_heat_transfer_interaction = 
            HeatTransferInteractionType::
            new_advection_interaction(mass_flowrate, 
                                      average_therminol_density, 
                                      average_therminol_density);

        // now let's link the fluid array part first 

        {
            adiabatic_dhx_tube_side_30.pipe_fluid_array.link_to_back(
                &mut inlet_bc, 
                advection_heat_transfer_interaction
                ).unwrap();

            adiabatic_dhx_tube_side_30.pipe_fluid_array.link_to_front(
                &mut outlet_bc, 
                advection_heat_transfer_interaction
                ).unwrap();
        }

        // set htc to zero (again, just kiasu)
        {
            adiabatic_dhx_tube_side_30.heat_transfer_to_ambient 
                = htc_to_ambient_zero;
        }
        // add 9 kW as a heater power for reference 
        {
            adiabatic_dhx_tube_side_30
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate, 
                    heater_power).unwrap();
        }

        // now advance timestep 
        {
            adiabatic_dhx_tube_side_30.
                advance_timestep(timestep).unwrap();
        }

        // let's obtain temperature 

        // first i get the array, then take the last element of the array
        // this is the "front_cv"
        let outlet_temperature: ThermodynamicTemperature 
            = *adiabatic_dhx_tube_side_30
            .pipe_fluid_array_temperature()
            .unwrap()
            .clone()
            .last()
            .unwrap();


        adiabatic_dhx_tube_side_30_outlet_temp = 
            outlet_temperature;



        
        // moving on timestep
        simulation_time += timestep;

    }
    // reset simulation time
    simulation_time = Time::ZERO;

    // suppose there is a parallel tube bundle of 20 equivalent 
    // dhx tube side 30
    let number_of_tubes = 20;
    let mass_flowrate_through_tube_bundle = 
        number_of_tubes as f64 * mass_flowrate;

    let heater_power_for_tube_bundle = 
        number_of_tubes as f64 * heater_power;

    let mut parallel_adiabatic_dhx_tube_side_30_outlet_temp = 
        ThermodynamicTemperature::ZERO;

    let mut parallel_adiabatic_dhx_tube_side_30 = 
        NonInsulatedParallelFluidComponent::new_bare_pipe_parallel_array(
            initial_temperature, 
            ambient_temperature, 
            fluid_pressure, 
            solid_pressure, 
            flow_area, 
            incline_angle, 
            form_loss, 
            id, 
            od, 
            pipe_length, 
            hydraulic_diameter, 
            surface_roughness, 
            pipe_shell_material, 
            pipe_fluid, 
            htc_to_ambient, 
            user_specified_inner_nodes,
            number_of_tubes);

    let mut fluid_array_ideal_nusslet: FluidArray = 
        parallel_adiabatic_dhx_tube_side_30.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_ideal_nusslet.nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    parallel_adiabatic_dhx_tube_side_30.pipe_fluid_array = 
        fluid_array_ideal_nusslet.into();

    parallel_adiabatic_dhx_tube_side_30.heat_transfer_to_ambient = 
        htc_to_ambient_zero;


    // main loop for parallel tube
    
    while max_time > simulation_time {

        // get average temperature and advection first 
        // we use boussinesq approximation, so the average temperature 
        // for density doesn't change
        // i'll just arbitrarily do 50 C
        let average_temperature_for_density_calcs = 
            ThermodynamicTemperature::
            new::<degree_celsius>(50.0);

        let average_therminol_density = 
            LiquidMaterial::TherminolVP1.try_get_density(
                average_temperature_for_density_calcs).unwrap();

        let advection_heat_transfer_interaction = 
            HeatTransferInteractionType::
            new_advection_interaction(mass_flowrate_through_tube_bundle, 
                                      average_therminol_density, 
                                      average_therminol_density);

        // now let's link the fluid array part first 

        {
            parallel_adiabatic_dhx_tube_side_30.pipe_fluid_array.link_to_back(
                &mut inlet_bc, 
                advection_heat_transfer_interaction
                ).unwrap();

            parallel_adiabatic_dhx_tube_side_30.pipe_fluid_array.link_to_front(
                &mut outlet_bc, 
                advection_heat_transfer_interaction
                ).unwrap();
        }

        // set htc to zero (again, just kiasu)
        {
            parallel_adiabatic_dhx_tube_side_30.heat_transfer_to_ambient 
                = htc_to_ambient_zero;
        }
        // add 9 kW as a heater power for reference 
        // times 20 for the bundle
        {
            parallel_adiabatic_dhx_tube_side_30
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_through_tube_bundle, 
                    heater_power_for_tube_bundle).unwrap();
        }

        // now advance timestep 
        {
            parallel_adiabatic_dhx_tube_side_30.
                advance_timestep(timestep).unwrap();
        }

        // let's obtain temperature 

        // first i get the array, then take the last element of the array
        // this is the "front_cv"
        let outlet_temperature: ThermodynamicTemperature 
            = *parallel_adiabatic_dhx_tube_side_30
            .pipe_fluid_array_temperature()
            .unwrap()
            .clone()
            .last()
            .unwrap();


        parallel_adiabatic_dhx_tube_side_30_outlet_temp = 
            outlet_temperature;



        
        // moving on timestep
        simulation_time += timestep;

    }


    approx::assert_relative_eq!(
        adiabatic_dhx_tube_side_30_outlet_temp.value,
        parallel_adiabatic_dhx_tube_side_30_outlet_temp.value,
        max_relative = 0.0001
        );

}


///
/// first thing first, heat addition 
///
/// suppose we have 9 kW of heat into one pipe 
/// at 0.18 kg/s TherminolVP1
///
/// whether you have the flow split into 1 single pipe or 10 parallel 
/// pipes should not matter. After a set time, the temperature should 
/// be the same. presuming of course, no heat loss to environment
///
/// I'm taking dhx tube side 30b as a reference
/// but setting heat transfer to ambient as 1000 W/(m2 K)
///
///
/// then I'll do the parallel tube setup, and add 20 tubes. 
/// the mass flow and heater power is increased by 20 times. 
///
/// the heating time is 25 s long, both outlet temperatures should be 
/// the same
///
#[test]
pub fn parallel_bare_pipes_debugging_heat_addition_with_parasitic_heat_loss(){

    use uom::si::f64::*;
    use uom::si::ratio::ratio;
    use uom::si::length::{meter, millimeter};
    use uom::si::area::square_meter;
    use uom::si::angle::degree;

    use crate::pre_built_components::
        non_insulated_fluid_components::NonInsulatedFluidComponent;
    use crate::heat_transfer_correlations::
        nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::boussinesq_thermophysical_properties::SolidMaterial;
    use crate::
        array_control_vol_and_fluid_component_collections::
        one_d_fluid_array_with_lateral_coupling::FluidArray;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use uom::si::power::kilowatt;
    use uom::ConstZero;
    use uom::si::time::second;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::pressure::atmosphere;

    use crate::boundary_conditions::BCType;
    use crate::
        boussinesq_thermophysical_properties::LiquidMaterial;
    use crate::heat_transfer_correlations::
        heat_transfer_interactions::heat_transfer_interaction_enums::
        HeatTransferInteractionType;
    use crate::pre_built_components::
        heat_transfer_entities::HeatTransferEntity;

    use crate::pre_built_components::
        non_insulated_parallel_fluid_components::
        NonInsulatedParallelFluidComponent;
    // conditions 
    let initial_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(25.0);


    let htc_to_ambient_high_value = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(1000.0);


    // parameters for new_isolated_dhx_tube_side_30
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    // for dhx modelling,
    //
    // dhx tubes are modelled in SAM as 19 tubes of diameter 
    // 0.00635 m 
    // and flow area of 6.1072e-4 m^2
    //
    // in Zweibaum's RELAP model,
    // it is quite different from the SAM model 
    // which follows Bickel's data
    let hydraulic_diameter = Length::new::<meter>(6.35e-3);
    let pipe_length = Length::new::<meter>(1.18745);
    let flow_area = Area::new::<square_meter>(6.0172e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(3.3);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.00079375);
    let od = id + pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 11 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 11-2
    let user_specified_inner_nodes = 11-2; 



    let mut adiabatic_dhx_tube_side_30 = 
        NonInsulatedFluidComponent::new_bare_pipe(
            initial_temperature, 
            ambient_temperature, 
            fluid_pressure, 
            solid_pressure, 
            flow_area, 
            incline_angle, 
            form_loss, 
            id, 
            od, 
            pipe_length, 
            hydraulic_diameter, 
            surface_roughness, 
            pipe_shell_material, 
            pipe_fluid, 
            htc_to_ambient, 
            user_specified_inner_nodes);

    // for heat exchangers, I give an ideal Nusselt number correlation 
    // as an approximation so that film thermal resistance is minimised
    let mut fluid_array_ideal_nusslet: FluidArray = 
        adiabatic_dhx_tube_side_30.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_ideal_nusslet.nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    adiabatic_dhx_tube_side_30.pipe_fluid_array = 
        fluid_array_ideal_nusslet.into();

    adiabatic_dhx_tube_side_30.heat_transfer_to_ambient = 
        htc_to_ambient_high_value;

    // now let's do a simple loop to check temperature after 25s
    let max_time = Time::new::<second>(25.0);
    let timestep = Time::new::<second>(0.01);
    let mut simulation_time = Time::ZERO;
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);
    let heater_power = Power::new::<kilowatt>(9.0);

    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        initial_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    let mut adiabatic_dhx_tube_side_30_outlet_temp = 
        ThermodynamicTemperature::ZERO;

    // main loop for single tube
    
    while max_time > simulation_time {

        // get average temperature and advection first 
        // we use boussinesq approximation, so the average temperature 
        // for density doesn't change
        // i'll just arbitrarily do 50 C
        let average_temperature_for_density_calcs = 
            ThermodynamicTemperature::
            new::<degree_celsius>(50.0);

        let average_therminol_density = 
            LiquidMaterial::TherminolVP1.try_get_density(
                average_temperature_for_density_calcs).unwrap();

        let advection_heat_transfer_interaction = 
            HeatTransferInteractionType::
            new_advection_interaction(mass_flowrate, 
                                      average_therminol_density, 
                                      average_therminol_density);

        // now let's link the fluid array part first 

        {
            adiabatic_dhx_tube_side_30.pipe_fluid_array.link_to_back(
                &mut inlet_bc, 
                advection_heat_transfer_interaction
                ).unwrap();

            adiabatic_dhx_tube_side_30.pipe_fluid_array.link_to_front(
                &mut outlet_bc, 
                advection_heat_transfer_interaction
                ).unwrap();
        }

        // set htc to high value (again, just kiasu)
        {
            adiabatic_dhx_tube_side_30.heat_transfer_to_ambient 
                = htc_to_ambient_high_value;
        }
        // add 9 kW as a heater power for reference 
        {
            adiabatic_dhx_tube_side_30
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate, 
                    heater_power).unwrap();
        }

        // now advance timestep 
        {
            adiabatic_dhx_tube_side_30.
                advance_timestep(timestep).unwrap();
        }

        // let's obtain temperature 

        // first i get the array, then take the last element of the array
        // this is the "front_cv"
        let outlet_temperature: ThermodynamicTemperature 
            = *adiabatic_dhx_tube_side_30
            .pipe_fluid_array_temperature()
            .unwrap()
            .clone()
            .last()
            .unwrap();


        adiabatic_dhx_tube_side_30_outlet_temp = 
            outlet_temperature;



        
        // moving on timestep
        simulation_time += timestep;

    }
    // reset simulation time
    simulation_time = Time::ZERO;

    // suppose there is a parallel tube bundle of 20 equivalent 
    // dhx tube side 30
    let number_of_tubes = 20;
    let mass_flowrate_through_tube_bundle = 
        number_of_tubes as f64 * mass_flowrate;

    let heater_power_for_tube_bundle = 
        number_of_tubes as f64 * heater_power;

    let mut parallel_adiabatic_dhx_tube_side_30_outlet_temp = 
        ThermodynamicTemperature::ZERO;

    let mut parallel_adiabatic_dhx_tube_side_30 = 
        NonInsulatedParallelFluidComponent::new_bare_pipe_parallel_array(
            initial_temperature, 
            ambient_temperature, 
            fluid_pressure, 
            solid_pressure, 
            flow_area, 
            incline_angle, 
            form_loss, 
            id, 
            od, 
            pipe_length, 
            hydraulic_diameter, 
            surface_roughness, 
            pipe_shell_material, 
            pipe_fluid, 
            htc_to_ambient, 
            user_specified_inner_nodes,
            number_of_tubes);

    let mut fluid_array_ideal_nusslet: FluidArray = 
        parallel_adiabatic_dhx_tube_side_30.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_ideal_nusslet.nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    parallel_adiabatic_dhx_tube_side_30.pipe_fluid_array = 
        fluid_array_ideal_nusslet.into();

    parallel_adiabatic_dhx_tube_side_30.heat_transfer_to_ambient = 
        htc_to_ambient_high_value;


    // main loop for parallel tube
    
    while max_time > simulation_time {

        // get average temperature and advection first 
        // we use boussinesq approximation, so the average temperature 
        // for density doesn't change
        // i'll just arbitrarily do 50 C
        let average_temperature_for_density_calcs = 
            ThermodynamicTemperature::
            new::<degree_celsius>(50.0);

        let average_therminol_density = 
            LiquidMaterial::TherminolVP1.try_get_density(
                average_temperature_for_density_calcs).unwrap();

        let advection_heat_transfer_interaction = 
            HeatTransferInteractionType::
            new_advection_interaction(mass_flowrate_through_tube_bundle, 
                                      average_therminol_density, 
                                      average_therminol_density);

        // now let's link the fluid array part first 

        {
            parallel_adiabatic_dhx_tube_side_30.pipe_fluid_array.link_to_back(
                &mut inlet_bc, 
                advection_heat_transfer_interaction
                ).unwrap();

            parallel_adiabatic_dhx_tube_side_30.pipe_fluid_array.link_to_front(
                &mut outlet_bc, 
                advection_heat_transfer_interaction
                ).unwrap();
        }

        // set htc to high value (again, just kiasu)
        {
            parallel_adiabatic_dhx_tube_side_30.heat_transfer_to_ambient 
                = htc_to_ambient_high_value;
        }
        // add 9 kW as a heater power for reference 
        // times 20 for the bundle
        {
            parallel_adiabatic_dhx_tube_side_30
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_through_tube_bundle, 
                    heater_power_for_tube_bundle).unwrap();
        }

        // now advance timestep 
        {
            parallel_adiabatic_dhx_tube_side_30.
                advance_timestep(timestep).unwrap();
        }

        // let's obtain temperature 

        // first i get the array, then take the last element of the array
        // this is the "front_cv"
        let outlet_temperature: ThermodynamicTemperature 
            = *parallel_adiabatic_dhx_tube_side_30
            .pipe_fluid_array_temperature()
            .unwrap()
            .clone()
            .last()
            .unwrap();


        parallel_adiabatic_dhx_tube_side_30_outlet_temp = 
            outlet_temperature;



        
        // moving on timestep
        simulation_time += timestep;

    }
    


    approx::assert_relative_eq!(
        adiabatic_dhx_tube_side_30_outlet_temp.value,
        parallel_adiabatic_dhx_tube_side_30_outlet_temp.value,
        max_relative = 0.0001
        );


}


///
/// next, cooling
///
/// suppose we have 0 kW of heat into one pipe 
/// at 0.18 kg/s TherminolVP1
///
/// inlet temperature is 80C
///
/// whether you have the flow split into 1 single pipe or 10 parallel 
/// pipes should not matter. After a set time, the temperature should 
/// be the same. presuming of course, no heat loss to environment
///
/// I'm taking dhx tube side 30b as a reference
/// but setting heat transfer to ambient as 1000 W/(m2 K)
///
///
/// then I'll do the parallel tube setup, and add 20 tubes. 
/// the mass flow and heater power is increased by 20 times. 
///
/// the heating time is 5 s long, both outlet temperatures should be 
/// the same thermal inertia wise anyway
///
/// the previous tests worked at 25s and 15s, but not 5s
/// disparity was great, likely thermal inertia problem
///
/// thermal inertia seems buggy, especially from 4-8s for this case.
/// After that however, steady state values seem to work out just fine
///
#[test]
pub fn parallel_bare_pipes_debugging_parasitic_heat_loss_thermal_inertia(){

    use uom::si::f64::*;
    use uom::si::ratio::ratio;
    use uom::si::length::{meter, millimeter};
    use uom::si::area::square_meter;
    use uom::si::angle::degree;

    use crate::pre_built_components::
        non_insulated_fluid_components::NonInsulatedFluidComponent;
    use crate::heat_transfer_correlations::
        nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::boussinesq_thermophysical_properties::SolidMaterial;
    use crate::
        array_control_vol_and_fluid_component_collections::
        one_d_fluid_array_with_lateral_coupling::FluidArray;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use uom::si::power::kilowatt;
    use uom::ConstZero;
    use uom::si::time::second;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::pressure::atmosphere;

    use crate::boundary_conditions::BCType;
    use crate::
        boussinesq_thermophysical_properties::LiquidMaterial;
    use crate::heat_transfer_correlations::
        heat_transfer_interactions::heat_transfer_interaction_enums::
        HeatTransferInteractionType;
    use crate::pre_built_components::
        heat_transfer_entities::HeatTransferEntity;

    use crate::pre_built_components::
        non_insulated_parallel_fluid_components::
        NonInsulatedParallelFluidComponent;
    // conditions 
    let initial_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(25.0);

    let inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(80.0);


    let htc_to_ambient_high_value = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(1000.0);


    // parameters for new_isolated_dhx_tube_side_30
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    // for dhx modelling,
    //
    // dhx tubes are modelled in SAM as 19 tubes of diameter 
    // 0.00635 m 
    // and flow area of 6.1072e-4 m^2
    // but I'll jsut use 20 here
    //
    // in Zweibaum's RELAP model,
    // it is quite different from the SAM model 
    // which follows Bickel's data
    let hydraulic_diameter = Length::new::<meter>(6.35e-3);
    let pipe_length = Length::new::<meter>(1.18745);
    let flow_area = Area::new::<square_meter>(6.0172e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(3.3);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.00079375);
    let od = id + pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    let user_specified_inner_nodes = 3-2; 



    let mut adiabatic_dhx_tube_side_30 = 
        NonInsulatedFluidComponent::new_bare_pipe(
            initial_temperature, 
            ambient_temperature, 
            fluid_pressure, 
            solid_pressure, 
            flow_area, 
            incline_angle, 
            form_loss, 
            id, 
            od, 
            pipe_length, 
            hydraulic_diameter, 
            surface_roughness, 
            pipe_shell_material, 
            pipe_fluid, 
            htc_to_ambient, 
            user_specified_inner_nodes);

    // for heat exchangers, I give an ideal Nusselt number correlation 
    // as an approximation so that film thermal resistance is minimised
    let mut fluid_array_ideal_nusselt: FluidArray = 
        adiabatic_dhx_tube_side_30.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_ideal_nusselt.nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    adiabatic_dhx_tube_side_30.pipe_fluid_array = 
        fluid_array_ideal_nusselt.into();

    adiabatic_dhx_tube_side_30.heat_transfer_to_ambient = 
        htc_to_ambient_high_value;

    // now let's do a simple loop to check temperature after short time
    let max_time = Time::new::<second>(5.0);
    let timestep = Time::new::<second>(0.1);
    let mut simulation_time = Time::ZERO;
    let mass_flowrate_single_tube = MassRate::new::<kilogram_per_second>(0.18);
    let heater_power = Power::new::<kilowatt>(0.0);

    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    let mut adiabatic_dhx_tube_side_30_outlet_temp = 
        ThermodynamicTemperature::ZERO;

    // main loop for single tube
    
    while max_time > simulation_time {

        // get average temperature and advection first 
        // we use boussinesq approximation, so the average temperature 
        // for density doesn't change
        // i'll just arbitrarily do 50 C
        let average_temperature_for_density_calcs = 
            ThermodynamicTemperature::
            new::<degree_celsius>(50.0);

        let average_therminol_density = 
            LiquidMaterial::TherminolVP1.try_get_density(
                average_temperature_for_density_calcs).unwrap();

        let advection_heat_transfer_interaction = 
            HeatTransferInteractionType::
            new_advection_interaction(mass_flowrate_single_tube, 
                                      average_therminol_density, 
                                      average_therminol_density);

        // now let's link the fluid array part first 

        {
            adiabatic_dhx_tube_side_30.pipe_fluid_array.link_to_back(
                &mut inlet_bc, 
                advection_heat_transfer_interaction
                ).unwrap();

            adiabatic_dhx_tube_side_30.pipe_fluid_array.link_to_front(
                &mut outlet_bc, 
                advection_heat_transfer_interaction
                ).unwrap();
        }

        // set htc to high value (again, just kiasu)
        {
            adiabatic_dhx_tube_side_30.heat_transfer_to_ambient 
                = htc_to_ambient_high_value;
        }
        // add 9 kW as a heater power for reference 
        {
            adiabatic_dhx_tube_side_30
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_single_tube, 
                    heater_power).unwrap();
        }

        // now advance timestep 
        {
            adiabatic_dhx_tube_side_30.
                advance_timestep(timestep).unwrap();
        }

        // let's obtain temperature 

        // first i get the array, then take the last element of the array
        // this is the "front_cv"
        let outlet_temperature: ThermodynamicTemperature 
            = *adiabatic_dhx_tube_side_30
            .pipe_fluid_array_temperature()
            .unwrap()
            .clone()
            .last()
            .unwrap();


        adiabatic_dhx_tube_side_30_outlet_temp = 
            outlet_temperature;

        dbg!(&adiabatic_dhx_tube_side_30_outlet_temp);


        
        // moving on timestep
        simulation_time += timestep;

    }
    // reset simulation time
    simulation_time = Time::ZERO;

    // suppose there is a parallel tube bundle of 20 equivalent 
    // dhx tube side 30
    let number_of_tubes = 20;
    let mass_flowrate_through_tube_bundle = 
        number_of_tubes as f64 * mass_flowrate_single_tube;

    let heater_power_for_tube_bundle = 
        number_of_tubes as f64 * heater_power;

    let mut parallel_adiabatic_dhx_tube_side_30_outlet_temp = 
        ThermodynamicTemperature::ZERO;

    let mut parallel_adiabatic_dhx_tube_side_30 = 
        NonInsulatedParallelFluidComponent::new_bare_pipe_parallel_array(
            initial_temperature, 
            ambient_temperature, 
            fluid_pressure, 
            solid_pressure, 
            flow_area, 
            incline_angle, 
            form_loss, 
            id, 
            od, 
            pipe_length, 
            hydraulic_diameter, 
            surface_roughness, 
            pipe_shell_material, 
            pipe_fluid, 
            htc_to_ambient, 
            user_specified_inner_nodes,
            number_of_tubes);

    let mut fluid_array_ideal_nusselt: FluidArray = 
        parallel_adiabatic_dhx_tube_side_30.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_ideal_nusselt.nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    parallel_adiabatic_dhx_tube_side_30.pipe_fluid_array = 
        fluid_array_ideal_nusselt.into();

    parallel_adiabatic_dhx_tube_side_30.heat_transfer_to_ambient = 
        htc_to_ambient_high_value;


    // main loop for parallel tube
    
    while max_time > simulation_time {

        // get average temperature and advection first 
        // we use boussinesq approximation, so the average temperature 
        // for density doesn't change
        // i'll just arbitrarily do 50 C
        let average_temperature_for_density_calcs = 
            ThermodynamicTemperature::
            new::<degree_celsius>(50.0);

        let average_therminol_density = 
            LiquidMaterial::TherminolVP1.try_get_density(
                average_temperature_for_density_calcs).unwrap();

        let advection_heat_transfer_interaction = 
            HeatTransferInteractionType::
            new_advection_interaction(mass_flowrate_through_tube_bundle, 
                                      average_therminol_density, 
                                      average_therminol_density);

        // now let's link the fluid array part first 

        {
            parallel_adiabatic_dhx_tube_side_30.pipe_fluid_array.link_to_back(
                &mut inlet_bc, 
                advection_heat_transfer_interaction
                ).unwrap();

            parallel_adiabatic_dhx_tube_side_30.pipe_fluid_array.link_to_front(
                &mut outlet_bc, 
                advection_heat_transfer_interaction
                ).unwrap();
        }

        // set htc to high value (again, just kiasu)
        {
            parallel_adiabatic_dhx_tube_side_30.heat_transfer_to_ambient 
                = htc_to_ambient_high_value;
        }
        // add 9 kW as a heater power for reference 
        // times 20 for the bundle
        {
            parallel_adiabatic_dhx_tube_side_30
                .lateral_and_miscellaneous_connections_no_wall_correction(
                    mass_flowrate_through_tube_bundle, 
                    heater_power_for_tube_bundle).unwrap();
        }

        // now advance timestep 
        {
            parallel_adiabatic_dhx_tube_side_30.
                advance_timestep(timestep).unwrap();
        }

        // let's obtain temperature 

        // first i get the array, then take the last element of the array
        // this is the "front_cv"
        let outlet_temperature: ThermodynamicTemperature 
            = *parallel_adiabatic_dhx_tube_side_30
            .pipe_fluid_array_temperature()
            .unwrap()
            .clone()
            .last()
            .unwrap();


        parallel_adiabatic_dhx_tube_side_30_outlet_temp = 
            outlet_temperature;
        dbg!(&parallel_adiabatic_dhx_tube_side_30_outlet_temp);



        
        // moving on timestep
        simulation_time += timestep;

    }
    


    approx::assert_relative_eq!(
        adiabatic_dhx_tube_side_30_outlet_temp.value,
        parallel_adiabatic_dhx_tube_side_30_outlet_temp.value,
        max_relative = 0.0001
        );


}

/// regression test, checks if parallel fluid components working 
/// correctly
///
/// basically getting mass flowrate from pressure change and vice versa
/// except now we have sevearl tubes in parallel
#[test] 
pub fn parallel_fluid_component_test(){

    use uom::si::f64::*;
    use uom::si::ratio::ratio;
    use uom::si::length::{meter, millimeter};
    use uom::si::area::square_meter;
    use uom::si::angle::degree;

    use crate::pre_built_components::
        non_insulated_fluid_components::NonInsulatedFluidComponent;
    use crate::heat_transfer_correlations::
        nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::boussinesq_thermophysical_properties::SolidMaterial;
    use crate::
        array_control_vol_and_fluid_component_collections::
        one_d_fluid_array_with_lateral_coupling::FluidArray;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::pressure::atmosphere;

    use crate::
        boussinesq_thermophysical_properties::LiquidMaterial;
    use crate::
        array_control_vol_and_fluid_component_collections::
        fluid_component_collection::fluid_component_traits::FluidComponentTrait;
    use crate::
        array_control_vol_and_fluid_component_collections::
        fluid_component_collection::fluid_component::FluidComponent;

    use uom::si::pressure::pascal;
    use crate::pre_built_components::
        non_insulated_parallel_fluid_components::
        NonInsulatedParallelFluidComponent;
    // conditions 
    let initial_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(25.0);



    let htc_to_ambient_high_value = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(1000.0);


    // parameters for new_isolated_dhx_tube_side_30
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    // for dhx modelling,
    //
    // dhx tubes are modelled in SAM as 19 tubes of diameter 
    // 0.00635 m 
    // and flow area of 6.1072e-4 m^2
    // but I'll jsut use 20 here
    //
    // in Zweibaum's RELAP model,
    // it is quite different from the SAM model 
    // which follows Bickel's data
    let hydraulic_diameter = Length::new::<meter>(6.35e-3);
    let pipe_length = Length::new::<meter>(1.18745);
    let flow_area = Area::new::<square_meter>(6.0172e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(3.3);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.00079375);
    let od = id + pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    let user_specified_inner_nodes = 3-2; 



    let mut adiabatic_dhx_tube_side_30 = 
        NonInsulatedFluidComponent::new_bare_pipe(
            initial_temperature, 
            ambient_temperature, 
            fluid_pressure, 
            solid_pressure, 
            flow_area, 
            incline_angle, 
            form_loss, 
            id, 
            od, 
            pipe_length, 
            hydraulic_diameter, 
            surface_roughness, 
            pipe_shell_material, 
            pipe_fluid, 
            htc_to_ambient, 
            user_specified_inner_nodes);

    // for heat exchangers, I give an ideal Nusselt number correlation 
    // as an approximation so that film thermal resistance is minimised
    let mut fluid_array_ideal_nusselt: FluidArray = 
        adiabatic_dhx_tube_side_30.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_ideal_nusselt.nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    adiabatic_dhx_tube_side_30.pipe_fluid_array = 
        fluid_array_ideal_nusselt.into();

    adiabatic_dhx_tube_side_30.heat_transfer_to_ambient = 
        htc_to_ambient_high_value;


    // now let's do a simple loop to check temperature after short time
    let mass_flowrate_single_tube = MassRate::new::<kilogram_per_second>(0.18);


    // suppose there is a parallel tube bundle of 20 equivalent 
    // dhx tube side 30
    let number_of_tubes = 20;
    let mass_flowrate_through_tube_bundle = 
        number_of_tubes as f64 * mass_flowrate_single_tube;


    let mut parallel_adiabatic_dhx_tube_side_30 = 
        NonInsulatedParallelFluidComponent::new_bare_pipe_parallel_array(
            initial_temperature, 
            ambient_temperature, 
            fluid_pressure, 
            solid_pressure, 
            flow_area, 
            incline_angle, 
            form_loss, 
            id, 
            od, 
            pipe_length, 
            hydraulic_diameter, 
            surface_roughness, 
            pipe_shell_material, 
            pipe_fluid, 
            htc_to_ambient, 
            user_specified_inner_nodes,
            number_of_tubes);

    let mut fluid_array_ideal_nusselt: FluidArray = 
        parallel_adiabatic_dhx_tube_side_30.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_ideal_nusselt.nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    parallel_adiabatic_dhx_tube_side_30.pipe_fluid_array = 
        fluid_array_ideal_nusselt.into();

    parallel_adiabatic_dhx_tube_side_30.heat_transfer_to_ambient = 
        htc_to_ambient_high_value;

    // now pressure test first 
    // ie pressure change from mass flowrate
    {
        let single_pipe: FluidComponent 
            = adiabatic_dhx_tube_side_30.clone().into();

        let pressure_change_single_tube = single_pipe.get_pressure_change_immutable(
            mass_flowrate_single_tube);

        let pipe_bundle: FluidComponent 
            = parallel_adiabatic_dhx_tube_side_30.clone().into();

        let pressure_change_tube_bundle = 
            pipe_bundle.get_pressure_change_immutable(
                mass_flowrate_through_tube_bundle);
        // both should be the same, as mass flowrate through tube bundle 
        // is 20 times

        approx::assert_relative_eq!(
            pressure_change_single_tube.value,
            pressure_change_tube_bundle.value,
            max_relative = 0.0001
        );


        // in pascals, this is about 11 kPa
        approx::assert_relative_eq!(
            pressure_change_single_tube.value,
            11070.0263947,
            max_relative = 0.0001
        );
    }

    // now mass flowrate test second
    // ie mass flowrate from pressure change
    {
        let pressure_change = Pressure::new::<pascal>(11070.0263947);

        let single_pipe: FluidComponent 
            = adiabatic_dhx_tube_side_30.clone().into();

        let test_mass_flowrate_single_tube = 
            single_pipe.get_mass_flowrate_from_pressure_change_immutable(
            pressure_change);

        let pipe_bundle: FluidComponent 
            = parallel_adiabatic_dhx_tube_side_30.clone().into();

        let test_mass_flowrate_through_tube_bundle = 
            pipe_bundle.get_mass_flowrate_from_pressure_change_immutable(
                pressure_change);
        // mass flowrate through tube bundle is 20 times that of the single 
        // bundle
        //
        // assuming 20 is number of tubes 

        approx::assert_relative_eq!(
            test_mass_flowrate_single_tube.value,
            (test_mass_flowrate_through_tube_bundle.value / 
            (number_of_tubes as f64)),
            max_relative = 0.0001
        );

        // mass flow through single tube is about 0.18 kg/s

        
        approx::assert_relative_eq!(
            test_mass_flowrate_single_tube.value,
            0.18,
            max_relative = 0.0001
        );



    }




}
