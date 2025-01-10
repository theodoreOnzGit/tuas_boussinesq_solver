use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
use crate::heat_transfer_correlations::thermal_resistance::try_get_thermal_conductance_annular_cylinder;
use uom::si::f64::*;
use std::f64::consts::PI;

/// basically given mcp and an arbitrary ua, for heat loss
/// the pipe fluid temperature at outlet should reach the analytical 
/// solution very closely
///
/// should be a heat transfer entity test though
///
/// but it mainly tests the fluidarray
#[cfg(test)]
#[test]
pub fn parasitic_heat_loss_integration_test_with_hte_1m(){

    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
    use crate::boundary_conditions::BCType;
    use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
    use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
    use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
    use ndarray::Array1;
    use uom::si::angle::degree;
    use uom::si::ratio::ratio;
    use uom::si::thermal_conductance::watt_per_kelvin;
    use uom::si::f64::*;
    use std::time::SystemTime;
    use uom::si::pressure::atmosphere;

    use uom::{si::time::second, ConstZero};

    use uom::si::length::meter;
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use uom::si::mass_rate::kilogram_per_second;
    // suppose I have a default fluid array 
    // the parameters used only determine thermal inertia

    // testings 

    let (l_meters, 
        ua_expected_watts_per_kelvin,
        t_out_expected_regression_degc, 
        t_out_calculated_by_pipe_degc) 
        = (1.00,0.96690, 99.761,99.762);



    // temperature

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(l_meters);
    let incline_angle = Angle::new::<degree>(51.526384);
    let form_loss = Ratio::new::<ratio>(21.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.00508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(2000.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    //
    // however, I'm having about 10 inner nodes here to make it work better
    // for verification
    let user_specified_inner_nodes = 10; 
    let initial_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(100.0);

    let mut static_mixer_41_label_6_fluid_arr: HeatTransferEntity
        = FluidArray::new_cylinder(
            component_length, 
            hydraulic_diameter, 
            initial_temperature, 
            fluid_pressure, 
            pipe_shell_material, 
            pipe_fluid, 
            form_loss, 
            user_specified_inner_nodes, 
            incline_angle).into();

    // now 
    // first calculate analytical solution
    let nusselt_correlation: NusseltCorrelation = 
        NusseltCorrelation::FixedNusselt(400.0.into());

    let average_expected_temp = 
        ThermodynamicTemperature::new::<degree_celsius>(100.0);
    let total_thermal_resistance_estimate = 
        calc_overall_thermal_resistance_for_pipe(
            htc_to_ambient, 
            shell_id, 
            shell_od, 
            insulation_thickness, 
            component_length, 
            nusselt_correlation, 
            pipe_fluid.try_get_thermal_conductivity(
                average_expected_temp).unwrap(), 
            pipe_shell_material.try_get_thermal_conductivity(
                average_expected_temp).unwrap(), 
            insulation_material.try_get_thermal_conductivity(
                average_expected_temp).unwrap()
            );
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);
    let ua: ThermalConductance = total_thermal_resistance_estimate.recip();

    let inlet_temp_degc: f64 = 100.0;
    let ambient_temp_degc: f64 = ambient_temperature.get::<degree_celsius>();

    let m_cp = 
        mass_flowrate * pipe_fluid.try_get_cp(average_expected_temp).unwrap();

    let analytical_outlet_temp_degc = 
        (inlet_temp_degc - ambient_temp_degc)
        * ( -ua/m_cp ).exp().get::<ratio>() 
        + ambient_temp_degc;
    // first assert analytical solution
    approx::assert_relative_eq!(
        analytical_outlet_temp_degc,
        t_out_expected_regression_degc,
        max_relative=1e-5
        );

    // then assert ua 
    approx::assert_relative_eq!(
        ua.get::<watt_per_kelvin>(),
        ua_expected_watts_per_kelvin,
        max_relative=1e-5
        );


    // now this is the simulation 

    let inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(inlet_temp_degc);
    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    // time settings 

    let max_time = Time::new::<second>(100.0);
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


        let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            static_mixer_41_label_6_fluid_arr.get_temperature_vector().unwrap();



        let back_cv_temperature: ThermodynamicTemperature = 
            therminol_array_temperature[0];

        let exit_temperature: ThermodynamicTemperature = 
            *therminol_array_temperature.iter().last().unwrap();

        let back_cv_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                back_cv_temperature).unwrap();

        let front_cv_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                exit_temperature).unwrap();

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


        // make other connections
        // set mass flowrate first
        static_mixer_41_label_6_fluid_arr
            .try_set_flowrate_for_fluid_array(
            mass_flowrate).unwrap();
        {
            let number_of_temperature_nodes = user_specified_inner_nodes + 2;


            let mut ambient_temperature_vector: Vec<ThermodynamicTemperature> 
                = Array1::default(number_of_temperature_nodes)
                .iter().map( |&temp| {
                    temp
                }
                ).collect();

            ambient_temperature_vector.fill(ambient_temperature);
            let mut fluid_array_clone: FluidArray = 
                static_mixer_41_label_6_fluid_arr.clone().try_into().unwrap();
            let nodalised_ua = 
                ua/(number_of_temperature_nodes as f64);

            fluid_array_clone.lateral_link_new_temperature_vector_avg_conductance(
                nodalised_ua,
                ambient_temperature_vector
            ).unwrap();

            static_mixer_41_label_6_fluid_arr = 
                fluid_array_clone.into();
        }



        // make axial connections to BCs 


        static_mixer_41_label_6_fluid_arr.link_to_back(
            &mut inlet_bc,
            inlet_interaction
        ).unwrap();

        static_mixer_41_label_6_fluid_arr.link_to_front(
            &mut outlet_bc,
            outlet_interaction
        ).unwrap();
        //// calculate timestep (serial method)
        //heater_v2_bare.advance_timestep(
        //    timestep);
        static_mixer_41_label_6_fluid_arr.advance_timestep_mut_self(
            timestep).unwrap();

        // calculate timestep (thread spawn method, parallel) 

        simulation_time += timestep;

        let time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();

        // print outlet temperature 
        dbg!(exit_temperature
            .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));

        simulated_outlet_temperature = exit_temperature;

        // print loop time 
        dbg!(time_taken_for_calculation_loop);
    }
    approx::assert_relative_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        t_out_calculated_by_pipe_degc,
        max_relative=1e-5
        );

    approx::assert_relative_eq!(
        analytical_outlet_temp_degc,
        simulated_outlet_temperature.get::<degree_celsius>(),
        max_relative=1e-5
        );
}


/// basically given mcp and an arbitrary ua, for heat loss
/// the pipe fluid temperature at outlet should reach the analytical 
/// solution very closely
///
/// should be a heat transfer entity test though
///
/// but it mainly tests the fluidarray
#[cfg(test)]
#[test]
pub fn parasitic_heat_loss_integration_test_with_hte_3m(){

    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
    use crate::boundary_conditions::BCType;
    use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
    use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
    use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
    use ndarray::Array1;
    use uom::si::angle::degree;
    use uom::si::ratio::ratio;
    use uom::si::thermal_conductance::watt_per_kelvin;
    use uom::si::f64::*;
    use std::time::SystemTime;
    use uom::si::pressure::atmosphere;

    use uom::{si::time::second, ConstZero};

    use uom::si::length::meter;
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use uom::si::mass_rate::kilogram_per_second;
    // suppose I have a default fluid array 
    // the parameters used only determine thermal inertia

    // testings 

    let (l_meters, 
        ua_expected_watts_per_kelvin,
        t_out_expected_regression_degc, 
        t_out_calculated_by_pipe_degc) 
        = (3.00, 2.9007, 99.287,99.287);



    // temperature

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(l_meters);
    let incline_angle = Angle::new::<degree>(51.526384);
    let form_loss = Ratio::new::<ratio>(21.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.00508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(2000.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    //
    // however, I'm having about 10 inner nodes here to make it work better
    // for verification
    let user_specified_inner_nodes = 10; 
    let initial_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(100.0);

    let mut static_mixer_41_label_6_fluid_arr: HeatTransferEntity
        = FluidArray::new_cylinder(
            component_length, 
            hydraulic_diameter, 
            initial_temperature, 
            fluid_pressure, 
            pipe_shell_material, 
            pipe_fluid, 
            form_loss, 
            user_specified_inner_nodes, 
            incline_angle).into();

    // now 
    // first calculate analytical solution
    let nusselt_correlation: NusseltCorrelation = 
        NusseltCorrelation::FixedNusselt(400.0.into());

    let average_expected_temp = 
        ThermodynamicTemperature::new::<degree_celsius>(100.0);
    let total_thermal_resistance_estimate = 
        calc_overall_thermal_resistance_for_pipe(
            htc_to_ambient, 
            shell_id, 
            shell_od, 
            insulation_thickness, 
            component_length, 
            nusselt_correlation, 
            pipe_fluid.try_get_thermal_conductivity(
                average_expected_temp).unwrap(), 
            pipe_shell_material.try_get_thermal_conductivity(
                average_expected_temp).unwrap(), 
            insulation_material.try_get_thermal_conductivity(
                average_expected_temp).unwrap()
            );
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);
    let ua: ThermalConductance = total_thermal_resistance_estimate.recip();

    let inlet_temp_degc: f64 = 100.0;
    let ambient_temp_degc: f64 = ambient_temperature.get::<degree_celsius>();

    let m_cp = 
        mass_flowrate * pipe_fluid.try_get_cp(average_expected_temp).unwrap();

    let analytical_outlet_temp_degc = 
        (inlet_temp_degc - ambient_temp_degc)
        * ( -ua/m_cp ).exp().get::<ratio>() 
        + ambient_temp_degc;
    // first assert analytical solution
    approx::assert_relative_eq!(
        analytical_outlet_temp_degc,
        t_out_expected_regression_degc,
        max_relative=1e-5
        );

    // then assert ua 
    approx::assert_relative_eq!(
        ua.get::<watt_per_kelvin>(),
        ua_expected_watts_per_kelvin,
        max_relative=1e-5
        );


    // now this is the simulation 

    let inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(inlet_temp_degc);
    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    // time settings 

    let max_time = Time::new::<second>(100.0);
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


        let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            static_mixer_41_label_6_fluid_arr.get_temperature_vector().unwrap();



        let back_cv_temperature: ThermodynamicTemperature = 
            therminol_array_temperature[0];

        let exit_temperature: ThermodynamicTemperature = 
            *therminol_array_temperature.iter().last().unwrap();

        let back_cv_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                back_cv_temperature).unwrap();

        let front_cv_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                exit_temperature).unwrap();

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


        // make other connections
        // set mass flowrate first
        static_mixer_41_label_6_fluid_arr
            .try_set_flowrate_for_fluid_array(
            mass_flowrate).unwrap();
        {
            let number_of_temperature_nodes = user_specified_inner_nodes + 2;


            let mut ambient_temperature_vector: Vec<ThermodynamicTemperature> 
                = Array1::default(number_of_temperature_nodes)
                .iter().map( |&temp| {
                    temp
                }
                ).collect();

            ambient_temperature_vector.fill(ambient_temperature);
            let mut fluid_array_clone: FluidArray = 
                static_mixer_41_label_6_fluid_arr.clone().try_into().unwrap();
            let nodalised_ua = 
                ua/(number_of_temperature_nodes as f64);

            fluid_array_clone.lateral_link_new_temperature_vector_avg_conductance(
                nodalised_ua,
                ambient_temperature_vector
            ).unwrap();

            static_mixer_41_label_6_fluid_arr = 
                fluid_array_clone.into();
        }



        // make axial connections to BCs 


        static_mixer_41_label_6_fluid_arr.link_to_back(
            &mut inlet_bc,
            inlet_interaction
        ).unwrap();

        static_mixer_41_label_6_fluid_arr.link_to_front(
            &mut outlet_bc,
            outlet_interaction
        ).unwrap();
        //// calculate timestep (serial method)
        //heater_v2_bare.advance_timestep(
        //    timestep);
        static_mixer_41_label_6_fluid_arr.advance_timestep_mut_self(
            timestep).unwrap();

        // calculate timestep (thread spawn method, parallel) 

        simulation_time += timestep;

        let time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();

        // print outlet temperature 
        dbg!(exit_temperature
            .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));

        simulated_outlet_temperature = exit_temperature;

        // print loop time 
        dbg!(time_taken_for_calculation_loop);
    }
    approx::assert_relative_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        t_out_calculated_by_pipe_degc,
        max_relative=1e-5
        );

    approx::assert_relative_eq!(
        analytical_outlet_temp_degc,
        simulated_outlet_temperature.get::<degree_celsius>(),
        max_relative=1e-5
        );
}

/// basically given mcp and an arbitrary ua, for heat loss
/// the pipe fluid temperature at outlet should reach the analytical 
/// solution very closely
///
/// should be a heat transfer entity test though
///
/// but it mainly tests the fluidarray
#[cfg(test)]
#[test]
pub fn parasitic_heat_loss_integration_test_with_hte_5m(){

    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
    use crate::boundary_conditions::BCType;
    use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
    use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
    use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
    use ndarray::Array1;
    use uom::si::angle::degree;
    use uom::si::ratio::ratio;
    use uom::si::thermal_conductance::watt_per_kelvin;
    use uom::si::f64::*;
    use std::time::SystemTime;
    use uom::si::pressure::atmosphere;

    use uom::{si::time::second, ConstZero};

    use uom::si::length::meter;
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use uom::si::mass_rate::kilogram_per_second;
    // suppose I have a default fluid array 
    // the parameters used only determine thermal inertia

    // testings 

    let (l_meters, 
        ua_expected_watts_per_kelvin,
        t_out_expected_regression_degc, 
        t_out_calculated_by_pipe_degc) 
        = (5.00, 4.8345, 98.815,98.815);



    // temperature

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(l_meters);
    let incline_angle = Angle::new::<degree>(51.526384);
    let form_loss = Ratio::new::<ratio>(21.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.00508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(2000.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    //
    // however, I'm having about 10 inner nodes here to make it work better
    // for verification
    let user_specified_inner_nodes = 10; 
    let initial_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(100.0);

    let mut static_mixer_41_label_6_fluid_arr: HeatTransferEntity
        = FluidArray::new_cylinder(
            component_length, 
            hydraulic_diameter, 
            initial_temperature, 
            fluid_pressure, 
            pipe_shell_material, 
            pipe_fluid, 
            form_loss, 
            user_specified_inner_nodes, 
            incline_angle).into();

    // now 
    // first calculate analytical solution
    let nusselt_correlation: NusseltCorrelation = 
        NusseltCorrelation::FixedNusselt(400.0.into());

    let average_expected_temp = 
        ThermodynamicTemperature::new::<degree_celsius>(100.0);
    let total_thermal_resistance_estimate = 
        calc_overall_thermal_resistance_for_pipe(
            htc_to_ambient, 
            shell_id, 
            shell_od, 
            insulation_thickness, 
            component_length, 
            nusselt_correlation, 
            pipe_fluid.try_get_thermal_conductivity(
                average_expected_temp).unwrap(), 
            pipe_shell_material.try_get_thermal_conductivity(
                average_expected_temp).unwrap(), 
            insulation_material.try_get_thermal_conductivity(
                average_expected_temp).unwrap()
            );
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);
    let ua: ThermalConductance = total_thermal_resistance_estimate.recip();

    let inlet_temp_degc: f64 = 100.0;
    let ambient_temp_degc: f64 = ambient_temperature.get::<degree_celsius>();

    let m_cp = 
        mass_flowrate * pipe_fluid.try_get_cp(average_expected_temp).unwrap();

    let analytical_outlet_temp_degc = 
        (inlet_temp_degc - ambient_temp_degc)
        * ( -ua/m_cp ).exp().get::<ratio>() 
        + ambient_temp_degc;
    // first assert analytical solution
    approx::assert_relative_eq!(
        analytical_outlet_temp_degc,
        t_out_expected_regression_degc,
        max_relative=1e-5
        );

    // then assert ua 
    approx::assert_relative_eq!(
        ua.get::<watt_per_kelvin>(),
        ua_expected_watts_per_kelvin,
        max_relative=1e-5
        );


    // now this is the simulation 

    let inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(inlet_temp_degc);
    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    // time settings 

    let max_time = Time::new::<second>(100.0);
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


        let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            static_mixer_41_label_6_fluid_arr.get_temperature_vector().unwrap();



        let back_cv_temperature: ThermodynamicTemperature = 
            therminol_array_temperature[0];

        let exit_temperature: ThermodynamicTemperature = 
            *therminol_array_temperature.iter().last().unwrap();

        let back_cv_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                back_cv_temperature).unwrap();

        let front_cv_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                exit_temperature).unwrap();

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


        // make other connections
        // set mass flowrate first
        static_mixer_41_label_6_fluid_arr
            .try_set_flowrate_for_fluid_array(
            mass_flowrate).unwrap();
        {
            let number_of_temperature_nodes = user_specified_inner_nodes + 2;


            let mut ambient_temperature_vector: Vec<ThermodynamicTemperature> 
                = Array1::default(number_of_temperature_nodes)
                .iter().map( |&temp| {
                    temp
                }
                ).collect();

            ambient_temperature_vector.fill(ambient_temperature);
            let mut fluid_array_clone: FluidArray = 
                static_mixer_41_label_6_fluid_arr.clone().try_into().unwrap();
            let nodalised_ua = 
                ua/(number_of_temperature_nodes as f64);

            fluid_array_clone.lateral_link_new_temperature_vector_avg_conductance(
                nodalised_ua,
                ambient_temperature_vector
            ).unwrap();

            static_mixer_41_label_6_fluid_arr = 
                fluid_array_clone.into();
        }



        // make axial connections to BCs 


        static_mixer_41_label_6_fluid_arr.link_to_back(
            &mut inlet_bc,
            inlet_interaction
        ).unwrap();

        static_mixer_41_label_6_fluid_arr.link_to_front(
            &mut outlet_bc,
            outlet_interaction
        ).unwrap();
        //// calculate timestep (serial method)
        //heater_v2_bare.advance_timestep(
        //    timestep);
        static_mixer_41_label_6_fluid_arr.advance_timestep_mut_self(
            timestep).unwrap();

        // calculate timestep (thread spawn method, parallel) 

        simulation_time += timestep;

        let time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();

        // print outlet temperature 
        dbg!(exit_temperature
            .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));

        simulated_outlet_temperature = exit_temperature;

        // print loop time 
        dbg!(time_taken_for_calculation_loop);
    }
    approx::assert_relative_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        t_out_calculated_by_pipe_degc,
        max_relative=1e-5
        );

    approx::assert_relative_eq!(
        analytical_outlet_temp_degc,
        simulated_outlet_temperature.get::<degree_celsius>(),
        max_relative=1e-5
        );
}

/// basically given mcp and an arbitrary ua, for heat loss
/// the pipe fluid temperature at outlet should reach the analytical 
/// solution very closely
///
/// should be a heat transfer entity test though
///
/// but it mainly tests the fluidarray
#[cfg(test)]
#[test]
pub fn parasitic_heat_loss_integration_test_with_hte_7m(){

    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
    use crate::boundary_conditions::BCType;
    use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
    use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
    use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
    use ndarray::Array1;
    use uom::si::angle::degree;
    use uom::si::ratio::ratio;
    use uom::si::thermal_conductance::watt_per_kelvin;
    use uom::si::f64::*;
    use std::time::SystemTime;
    use uom::si::pressure::atmosphere;

    use uom::{si::time::second, ConstZero};

    use uom::si::length::meter;
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use uom::si::mass_rate::kilogram_per_second;
    // suppose I have a default fluid array 
    // the parameters used only determine thermal inertia

    // testings 

    let (l_meters, 
        ua_expected_watts_per_kelvin,
        t_out_expected_regression_degc, 
        t_out_calculated_by_pipe_degc) 
        = (7.00, 6.7683, 98.346,98.346);



    // temperature

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(l_meters);
    let incline_angle = Angle::new::<degree>(51.526384);
    let form_loss = Ratio::new::<ratio>(21.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.00508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(2000.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    //
    // however, I'm having about 10 inner nodes here to make it work better
    // for verification
    let user_specified_inner_nodes = 10; 
    let initial_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(100.0);

    let mut static_mixer_41_label_6_fluid_arr: HeatTransferEntity
        = FluidArray::new_cylinder(
            component_length, 
            hydraulic_diameter, 
            initial_temperature, 
            fluid_pressure, 
            pipe_shell_material, 
            pipe_fluid, 
            form_loss, 
            user_specified_inner_nodes, 
            incline_angle).into();

    // now 
    // first calculate analytical solution
    let nusselt_correlation: NusseltCorrelation = 
        NusseltCorrelation::FixedNusselt(400.0.into());

    let average_expected_temp = 
        ThermodynamicTemperature::new::<degree_celsius>(100.0);
    let total_thermal_resistance_estimate = 
        calc_overall_thermal_resistance_for_pipe(
            htc_to_ambient, 
            shell_id, 
            shell_od, 
            insulation_thickness, 
            component_length, 
            nusselt_correlation, 
            pipe_fluid.try_get_thermal_conductivity(
                average_expected_temp).unwrap(), 
            pipe_shell_material.try_get_thermal_conductivity(
                average_expected_temp).unwrap(), 
            insulation_material.try_get_thermal_conductivity(
                average_expected_temp).unwrap()
            );
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);
    let ua: ThermalConductance = total_thermal_resistance_estimate.recip();

    let inlet_temp_degc: f64 = 100.0;
    let ambient_temp_degc: f64 = ambient_temperature.get::<degree_celsius>();

    let m_cp = 
        mass_flowrate * pipe_fluid.try_get_cp(average_expected_temp).unwrap();

    let analytical_outlet_temp_degc = 
        (inlet_temp_degc - ambient_temp_degc)
        * ( -ua/m_cp ).exp().get::<ratio>() 
        + ambient_temp_degc;
    // first assert analytical solution
    approx::assert_relative_eq!(
        analytical_outlet_temp_degc,
        t_out_expected_regression_degc,
        max_relative=1e-5
        );

    // then assert ua 
    approx::assert_relative_eq!(
        ua.get::<watt_per_kelvin>(),
        ua_expected_watts_per_kelvin,
        max_relative=1e-5
        );


    // now this is the simulation 

    let inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(inlet_temp_degc);
    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    // time settings 

    let max_time = Time::new::<second>(100.0);
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


        let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            static_mixer_41_label_6_fluid_arr.get_temperature_vector().unwrap();



        let back_cv_temperature: ThermodynamicTemperature = 
            therminol_array_temperature[0];

        let exit_temperature: ThermodynamicTemperature = 
            *therminol_array_temperature.iter().last().unwrap();

        let back_cv_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                back_cv_temperature).unwrap();

        let front_cv_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                exit_temperature).unwrap();

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


        // make other connections
        // set mass flowrate first
        static_mixer_41_label_6_fluid_arr
            .try_set_flowrate_for_fluid_array(
            mass_flowrate).unwrap();
        {
            let number_of_temperature_nodes = user_specified_inner_nodes + 2;


            let mut ambient_temperature_vector: Vec<ThermodynamicTemperature> 
                = Array1::default(number_of_temperature_nodes)
                .iter().map( |&temp| {
                    temp
                }
                ).collect();

            ambient_temperature_vector.fill(ambient_temperature);
            let mut fluid_array_clone: FluidArray = 
                static_mixer_41_label_6_fluid_arr.clone().try_into().unwrap();
            let nodalised_ua = 
                ua/(number_of_temperature_nodes as f64);

            fluid_array_clone.lateral_link_new_temperature_vector_avg_conductance(
                nodalised_ua,
                ambient_temperature_vector
            ).unwrap();

            static_mixer_41_label_6_fluid_arr = 
                fluid_array_clone.into();
        }



        // make axial connections to BCs 


        static_mixer_41_label_6_fluid_arr.link_to_back(
            &mut inlet_bc,
            inlet_interaction
        ).unwrap();

        static_mixer_41_label_6_fluid_arr.link_to_front(
            &mut outlet_bc,
            outlet_interaction
        ).unwrap();
        //// calculate timestep (serial method)
        //heater_v2_bare.advance_timestep(
        //    timestep);
        static_mixer_41_label_6_fluid_arr.advance_timestep_mut_self(
            timestep).unwrap();

        // calculate timestep (thread spawn method, parallel) 

        simulation_time += timestep;

        let time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();

        // print outlet temperature 
        dbg!(exit_temperature
            .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));

        simulated_outlet_temperature = exit_temperature;

        // print loop time 
        dbg!(time_taken_for_calculation_loop);
    }
    approx::assert_relative_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        t_out_calculated_by_pipe_degc,
        max_relative=1e-5
        );

    approx::assert_relative_eq!(
        analytical_outlet_temp_degc,
        simulated_outlet_temperature.get::<degree_celsius>(),
        max_relative=1e-5
        );
}


/// basically given mcp and an arbitrary ua, for heat loss
/// the pipe fluid temperature at outlet should reach the analytical 
/// solution very closely
///
/// should be a heat transfer entity test though
///
/// but it mainly tests the fluidarray
#[cfg(test)]
#[test]
pub fn parasitic_heat_loss_integration_test_with_hte_9m(){

    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
    use crate::boundary_conditions::BCType;
    use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
    use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
    use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
    use ndarray::Array1;
    use uom::si::angle::degree;
    use uom::si::ratio::ratio;
    use uom::si::thermal_conductance::watt_per_kelvin;
    use uom::si::f64::*;
    use std::time::SystemTime;
    use uom::si::pressure::atmosphere;

    use uom::{si::time::second, ConstZero};

    use uom::si::length::meter;
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use uom::si::mass_rate::kilogram_per_second;
    // suppose I have a default fluid array 
    // the parameters used only determine thermal inertia

    // testings 

    let (l_meters, 
        ua_expected_watts_per_kelvin,
        t_out_expected_regression_degc, 
        t_out_calculated_by_pipe_degc) 
        = (9.00, 8.7021, 97.880,97.878);



    // temperature

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(l_meters);
    let incline_angle = Angle::new::<degree>(51.526384);
    let form_loss = Ratio::new::<ratio>(21.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.00508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(2000.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    //
    // however, I'm having about 10 inner nodes here to make it work better
    // for verification
    let user_specified_inner_nodes = 10; 
    let initial_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(100.0);

    let mut static_mixer_41_label_6_fluid_arr: HeatTransferEntity
        = FluidArray::new_cylinder(
            component_length, 
            hydraulic_diameter, 
            initial_temperature, 
            fluid_pressure, 
            pipe_shell_material, 
            pipe_fluid, 
            form_loss, 
            user_specified_inner_nodes, 
            incline_angle).into();

    // now 
    // first calculate analytical solution
    let nusselt_correlation: NusseltCorrelation = 
        NusseltCorrelation::FixedNusselt(400.0.into());

    let average_expected_temp = 
        ThermodynamicTemperature::new::<degree_celsius>(100.0);
    let total_thermal_resistance_estimate = 
        calc_overall_thermal_resistance_for_pipe(
            htc_to_ambient, 
            shell_id, 
            shell_od, 
            insulation_thickness, 
            component_length, 
            nusselt_correlation, 
            pipe_fluid.try_get_thermal_conductivity(
                average_expected_temp).unwrap(), 
            pipe_shell_material.try_get_thermal_conductivity(
                average_expected_temp).unwrap(), 
            insulation_material.try_get_thermal_conductivity(
                average_expected_temp).unwrap()
            );
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);
    let ua: ThermalConductance = total_thermal_resistance_estimate.recip();

    let inlet_temp_degc: f64 = 100.0;
    let ambient_temp_degc: f64 = ambient_temperature.get::<degree_celsius>();

    let m_cp = 
        mass_flowrate * pipe_fluid.try_get_cp(average_expected_temp).unwrap();

    let analytical_outlet_temp_degc = 
        (inlet_temp_degc - ambient_temp_degc)
        * ( -ua/m_cp ).exp().get::<ratio>() 
        + ambient_temp_degc;
    // first assert analytical solution
    approx::assert_relative_eq!(
        analytical_outlet_temp_degc,
        t_out_expected_regression_degc,
        max_relative=1e-5
        );

    // then assert ua 
    approx::assert_relative_eq!(
        ua.get::<watt_per_kelvin>(),
        ua_expected_watts_per_kelvin,
        max_relative=1e-5
        );


    // now this is the simulation 

    let inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(inlet_temp_degc);
    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    // time settings 

    let max_time = Time::new::<second>(100.0);
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


        let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            static_mixer_41_label_6_fluid_arr.get_temperature_vector().unwrap();



        let back_cv_temperature: ThermodynamicTemperature = 
            therminol_array_temperature[0];

        let exit_temperature: ThermodynamicTemperature = 
            *therminol_array_temperature.iter().last().unwrap();

        let back_cv_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                back_cv_temperature).unwrap();

        let front_cv_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                exit_temperature).unwrap();

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


        // make other connections
        // set mass flowrate first
        static_mixer_41_label_6_fluid_arr
            .try_set_flowrate_for_fluid_array(
            mass_flowrate).unwrap();
        {
            let number_of_temperature_nodes = user_specified_inner_nodes + 2;


            let mut ambient_temperature_vector: Vec<ThermodynamicTemperature> 
                = Array1::default(number_of_temperature_nodes)
                .iter().map( |&temp| {
                    temp
                }
                ).collect();

            ambient_temperature_vector.fill(ambient_temperature);
            let mut fluid_array_clone: FluidArray = 
                static_mixer_41_label_6_fluid_arr.clone().try_into().unwrap();
            let nodalised_ua = 
                ua/(number_of_temperature_nodes as f64);

            fluid_array_clone.lateral_link_new_temperature_vector_avg_conductance(
                nodalised_ua,
                ambient_temperature_vector
            ).unwrap();

            static_mixer_41_label_6_fluid_arr = 
                fluid_array_clone.into();
        }



        // make axial connections to BCs 


        static_mixer_41_label_6_fluid_arr.link_to_back(
            &mut inlet_bc,
            inlet_interaction
        ).unwrap();

        static_mixer_41_label_6_fluid_arr.link_to_front(
            &mut outlet_bc,
            outlet_interaction
        ).unwrap();
        //// calculate timestep (serial method)
        //heater_v2_bare.advance_timestep(
        //    timestep);
        static_mixer_41_label_6_fluid_arr.advance_timestep_mut_self(
            timestep).unwrap();

        // calculate timestep (thread spawn method, parallel) 

        simulation_time += timestep;

        let time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();

        // print outlet temperature 
        dbg!(exit_temperature
            .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));

        simulated_outlet_temperature = exit_temperature;

        // print loop time 
        dbg!(time_taken_for_calculation_loop);
    }
    approx::assert_relative_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        t_out_calculated_by_pipe_degc,
        max_relative=1e-5
        );

    // for 9m pipe, a 10 node solution starts to show mesh refinement problems
    // compared to the other pipes
    approx::assert_relative_eq!(
        analytical_outlet_temp_degc,
        simulated_outlet_temperature.get::<degree_celsius>(),
        max_relative=2e-5
        );
}

#[cfg(test)]
#[test]
pub fn cp_for_therminol_vp_1(){
    use crate::boussinesq_thermophysical_properties::LiquidMaterial;
    use uom::si::{f64::*, specific_heat_capacity::joule_per_kilogram_kelvin};
    use uom::si::thermodynamic_temperature::degree_celsius;

    let initial_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(95.0);
    let cp_therminol: SpecificHeatCapacity = 
        LiquidMaterial::TherminolVP1.try_get_cp(
            initial_temperature).unwrap();

    approx::assert_relative_eq!(
        cp_therminol.get::<joule_per_kilogram_kelvin>(),
        1785.9,
        max_relative=1e-5
        );


}
/// now, UA is the overall conductance 
///
/// and 1/UA is the overall resistance 
/// 1/UA = R_conv_to_ambient + R_conv_to_fluid + R_shell + R_insulation
///

/// this calculates convective resistance to ambient air
///
/// h = 20 W/(m^2 K)
///
/// A = PI * OD * L
///
///
pub fn calc_overall_thermal_resistance_for_pipe(
    htc_to_ambient: HeatTransfer,
    shell_id: Length,
    shell_od: Length,
    insulation_thickness: Length,
    pipe_length: Length,
    nusselt_correlation: NusseltCorrelation,
    fluid_thermal_conductivity: ThermalConductivity,
    pipe_thermal_conductivity: ThermalConductivity,
    insulation_thermal_conductivity: ThermalConductivity,
    ) -> ThermalResistance {


    let insulation_id = shell_od;
    let insulation_od = insulation_id + 2.0*insulation_thickness;
    let hydraulic_diameter = shell_id;

    // convective resistance to ambient
    let convective_resistance_to_ambient: ThermalResistance 
        = (htc_to_ambient * PI * insulation_od * pipe_length).recip();

    let nusselt_number = nusselt_correlation.try_get_nusselt().unwrap();

    let fluid_htc_to_pipe: HeatTransfer = 
        nusselt_number * fluid_thermal_conductivity / hydraulic_diameter;

    // convective resistance to pipe
    let convective_thermal_resistance_to_pipe 
        = (fluid_htc_to_pipe * PI * shell_id * pipe_length).recip();


    // insulation resistance
    let insulation_resistance = 
        try_get_thermal_conductance_annular_cylinder(
            insulation_id, 
            insulation_od, 
            pipe_length, 
            insulation_thermal_conductivity).unwrap()
        .recip();

    // pipe shell resistance
    let pipe_shell_resistance = 
        try_get_thermal_conductance_annular_cylinder(
            shell_id, 
            shell_od, 
            pipe_length, 
            pipe_thermal_conductivity).unwrap()
        .recip();
    
    // return total
    let total_resistance = convective_resistance_to_ambient
        + convective_thermal_resistance_to_pipe
        + pipe_shell_resistance
        + insulation_resistance;
    
    return total_resistance;
}
