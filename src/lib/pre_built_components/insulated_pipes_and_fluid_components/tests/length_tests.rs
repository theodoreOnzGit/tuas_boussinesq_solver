use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::boundary_conditions::BCType;
use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
use crate::pre_built_components::insulated_pipes_and_fluid_components::tests::preliminaries::calc_overall_thermal_resistance_for_pipe;
use crate::pre_built_components::insulated_pipes_and_fluid_components::InsulatedFluidComponent;
use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
use uom::si::angle::degree;
use uom::si::area::square_meter;
use uom::si::ratio::ratio;
use uom::si::f64::*;
use std::time::SystemTime;
use std::thread::JoinHandle;
use uom::si::pressure::atmosphere;

use uom::{si::time::second, ConstZero};

use uom::si::length::meter;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::thermodynamic_temperature::degree_celsius;

use uom::si::mass_rate::kilogram_per_second;

// 1m test
#[test]
pub fn static_mixer_41_label_6_1_meter_test_reduced_insulation_thickness_ambient_resistance_and_increase_nusselt(){

    // testings 
    let (l_meters, 
        t_out_expected_regression_degc, 
        t_out_calculated_by_pipe_degc) 
        = (1.00, 99.760,99.786);


    // temperature

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(l_meters);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(51.526384);
    let form_loss = Ratio::new::<ratio>(21.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(4000.0);
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

    let mut static_mixer_41_label_6 = InsulatedFluidComponent::new_custom_component(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        reynolds_coefficient, 
        reynolds_power, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        component_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes);

    // now, i want to replace the inner nusselt number by 4000.0
    // just for verification 
    let laminar_nusselt_correlation: NusseltCorrelation = 
        NusseltCorrelation::FixedNusselt(4000.0.into());

    let mut themrinol_array: FluidArray = 
        static_mixer_41_label_6.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    themrinol_array.nusselt_correlation = laminar_nusselt_correlation;

    static_mixer_41_label_6.pipe_fluid_array = 
        themrinol_array.into();

    // first calculate analytical solution

    let average_expected_temp = 
        ThermodynamicTemperature::new::<degree_celsius>(100.0);
    let total_thermal_resistance_estimate = 
        calc_overall_thermal_resistance_for_pipe(
            htc_to_ambient, 
            shell_id, 
            shell_od, 
            insulation_thickness, 
            component_length, 
            laminar_nusselt_correlation, 
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

    // now this is the simulation 

    let inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(inlet_temp_degc);
    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    // time settings 

    let max_time = Time::new::<second>(3000.0);
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
            = static_mixer_41_label_6.pipe_fluid_array.clone().try_into().unwrap();

        let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            therminol_array_clone.get_temperature_vector().unwrap();

        let steel_array_clone: SolidColumn 
            = static_mixer_41_label_6.pipe_shell.clone().try_into().unwrap();

        let _steel_array_temperature: Vec<ThermodynamicTemperature> = 
            steel_array_clone.get_temperature_vector().unwrap();


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

        // make axial connections to BCs 

        static_mixer_41_label_6.pipe_fluid_array.link_to_back(
            &mut inlet_bc,
            inlet_interaction
        ).unwrap();

        static_mixer_41_label_6.pipe_fluid_array.link_to_front(
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
        let insulated_fluid_component_join_handle: JoinHandle<InsulatedFluidComponent> 
            = static_mixer_41_label_6.
            lateral_connection_thread_spawn(
                mass_flowrate,
                Power::ZERO);

        static_mixer_41_label_6 = insulated_fluid_component_join_handle.join().unwrap();

        //// calculate timestep (serial method)
        //heater_v2_bare.advance_timestep(
        //    timestep);

        // calculate timestep (thread spawn method, parallel) 
        let insulated_fluid_component_join_handle: JoinHandle<InsulatedFluidComponent> 
            = static_mixer_41_label_6.advance_timestep_thread_spawn(
                timestep);

        static_mixer_41_label_6 = insulated_fluid_component_join_handle.join().unwrap();

        simulation_time += timestep;

        let _time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();

        // print outlet temperature 

        simulated_outlet_temperature = exit_temperature;

    }


    approx::assert_relative_eq!(
        analytical_outlet_temp_degc,
        t_out_expected_regression_degc,
        max_relative=1e-5
        );


    approx::assert_relative_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        t_out_calculated_by_pipe_degc,
        max_relative=1e-5
        );

    approx::assert_abs_diff_eq!(
        analytical_outlet_temp_degc,
        simulated_outlet_temperature.get::<degree_celsius>(),
        epsilon=0.05
        );

}


/// 3m test
/// note: probably hasn't reached steady state yet
/// but that takes real long to test
#[test]
pub fn static_mixer_3_meter_test_reduced_insulation_thickness_ambient_resistance_and_increase_nusselt(){

    // testings 
    let (l_meters, 
        t_out_expected_regression_degc, 
        t_out_calculated_by_pipe_degc) 
        = (3.00, 99.283,99.359);


    // temperature

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(l_meters);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(51.526384);
    let form_loss = Ratio::new::<ratio>(21.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(4000.0);
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

    let mut static_mixer_41_label_6 = InsulatedFluidComponent::new_custom_component(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        reynolds_coefficient, 
        reynolds_power, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        component_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes);

    // now, i want to replace the inner nusselt number by 4000.0
    // just for verification 
    let laminar_nusselt_correlation: NusseltCorrelation = 
        NusseltCorrelation::FixedNusselt(4000.0.into());

    let mut themrinol_array: FluidArray = 
        static_mixer_41_label_6.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    themrinol_array.nusselt_correlation = laminar_nusselt_correlation;

    static_mixer_41_label_6.pipe_fluid_array = 
        themrinol_array.into();

    // first calculate analytical solution

    let average_expected_temp = 
        ThermodynamicTemperature::new::<degree_celsius>(100.0);
    let total_thermal_resistance_estimate = 
        calc_overall_thermal_resistance_for_pipe(
            htc_to_ambient, 
            shell_id, 
            shell_od, 
            insulation_thickness, 
            component_length, 
            laminar_nusselt_correlation, 
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

    // now this is the simulation 

    let inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(inlet_temp_degc);
    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    // time settings 

    let max_time = Time::new::<second>(3000.0);
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
            = static_mixer_41_label_6.pipe_fluid_array.clone().try_into().unwrap();

        let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            therminol_array_clone.get_temperature_vector().unwrap();

        let steel_array_clone: SolidColumn 
            = static_mixer_41_label_6.pipe_shell.clone().try_into().unwrap();

        let _steel_array_temperature: Vec<ThermodynamicTemperature> = 
            steel_array_clone.get_temperature_vector().unwrap();


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

        // make axial connections to BCs 

        static_mixer_41_label_6.pipe_fluid_array.link_to_back(
            &mut inlet_bc,
            inlet_interaction
        ).unwrap();

        static_mixer_41_label_6.pipe_fluid_array.link_to_front(
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
        let insulated_fluid_component_join_handle: JoinHandle<InsulatedFluidComponent> 
            = static_mixer_41_label_6.
            lateral_connection_thread_spawn(
                mass_flowrate,
                Power::ZERO);

        static_mixer_41_label_6 = insulated_fluid_component_join_handle.join().unwrap();

        //// calculate timestep (serial method)
        //heater_v2_bare.advance_timestep(
        //    timestep);

        // calculate timestep (thread spawn method, parallel) 
        let insulated_fluid_component_join_handle: JoinHandle<InsulatedFluidComponent> 
            = static_mixer_41_label_6.advance_timestep_thread_spawn(
                timestep);

        static_mixer_41_label_6 = insulated_fluid_component_join_handle.join().unwrap();

        simulation_time += timestep;

        let _time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();

        // print outlet temperature 

        simulated_outlet_temperature = exit_temperature;

    }


    approx::assert_relative_eq!(
        analytical_outlet_temp_degc,
        t_out_expected_regression_degc,
        max_relative=1e-5
        );


    approx::assert_relative_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        t_out_calculated_by_pipe_degc,
        max_relative=1e-5
        );

    approx::assert_abs_diff_eq!(
        analytical_outlet_temp_degc,
        simulated_outlet_temperature.get::<degree_celsius>(),
        epsilon=0.1
        );

}


/// 5m test
/// 52 nodes
/// note: probably hasn't reached steady state yet
/// but that takes real long to test
#[test]
pub fn static_mixer_5_meter_test_reduced_insulation_thickness_ambient_resistance_and_increase_nusselt(){

    // testings 
    let (l_meters, 
        t_out_expected_regression_degc, 
        t_out_calculated_by_pipe_degc) 
        = (5.00, 98.808,98.934);


    // temperature

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(l_meters);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(51.526384);
    let form_loss = Ratio::new::<ratio>(21.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(4000.0);
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
    // however, I'm having about 50 inner nodes here to make it work better
    // for verification
    let user_specified_inner_nodes = 50; 
    let initial_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(100.0);

    let mut static_mixer_41_label_6 = InsulatedFluidComponent::new_custom_component(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        reynolds_coefficient, 
        reynolds_power, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        component_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes);

    // now, i want to replace the inner nusselt number by 4000.0
    // just for verification 
    let laminar_nusselt_correlation: NusseltCorrelation = 
        NusseltCorrelation::FixedNusselt(4000.0.into());

    let mut themrinol_array: FluidArray = 
        static_mixer_41_label_6.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    themrinol_array.nusselt_correlation = laminar_nusselt_correlation;

    static_mixer_41_label_6.pipe_fluid_array = 
        themrinol_array.into();

    // first calculate analytical solution

    let average_expected_temp = 
        ThermodynamicTemperature::new::<degree_celsius>(100.0);
    let total_thermal_resistance_estimate = 
        calc_overall_thermal_resistance_for_pipe(
            htc_to_ambient, 
            shell_id, 
            shell_od, 
            insulation_thickness, 
            component_length, 
            laminar_nusselt_correlation, 
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

    // now this is the simulation 

    let inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(inlet_temp_degc);
    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    // time settings 

    let max_time = Time::new::<second>(4000.0);
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
            = static_mixer_41_label_6.pipe_fluid_array.clone().try_into().unwrap();

        let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            therminol_array_clone.get_temperature_vector().unwrap();

        let steel_array_clone: SolidColumn 
            = static_mixer_41_label_6.pipe_shell.clone().try_into().unwrap();

        let _steel_array_temperature: Vec<ThermodynamicTemperature> = 
            steel_array_clone.get_temperature_vector().unwrap();


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

        // make axial connections to BCs 

        static_mixer_41_label_6.pipe_fluid_array.link_to_back(
            &mut inlet_bc,
            inlet_interaction
        ).unwrap();

        static_mixer_41_label_6.pipe_fluid_array.link_to_front(
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
        let insulated_fluid_component_join_handle: JoinHandle<InsulatedFluidComponent> 
            = static_mixer_41_label_6.
            lateral_connection_thread_spawn(
                mass_flowrate,
                Power::ZERO);

        static_mixer_41_label_6 = insulated_fluid_component_join_handle.join().unwrap();

        //// calculate timestep (serial method)
        //heater_v2_bare.advance_timestep(
        //    timestep);

        // calculate timestep (thread spawn method, parallel) 
        let insulated_fluid_component_join_handle: JoinHandle<InsulatedFluidComponent> 
            = static_mixer_41_label_6.advance_timestep_thread_spawn(
                timestep);

        static_mixer_41_label_6 = insulated_fluid_component_join_handle.join().unwrap();

        simulation_time += timestep;

        let _time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();

        // print outlet temperature 

        simulated_outlet_temperature = exit_temperature;

        // print loop time 
    }


    approx::assert_relative_eq!(
        analytical_outlet_temp_degc,
        t_out_expected_regression_degc,
        max_relative=1e-5
        );


    approx::assert_relative_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        t_out_calculated_by_pipe_degc,
        max_relative=1e-5
        );

    approx::assert_abs_diff_eq!(
        analytical_outlet_temp_degc,
        simulated_outlet_temperature.get::<degree_celsius>(),
        epsilon=0.2
        );

}


/// 7m test
/// 72 nodes
/// note: probably hasn't reached steady state yet
/// but that takes real long to test
#[test]
pub fn static_mixer_7_meter_test_reduced_insulation_thickness_ambient_resistance_and_increase_nusselt(){

    // testings 
    let (l_meters, 
        t_out_expected_regression_degc, 
        t_out_calculated_by_pipe_degc) 
        = (7.00, 98.337,98.512);


    // temperature

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(l_meters);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(51.526384);
    let form_loss = Ratio::new::<ratio>(21.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(4000.0);
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
    // however, I'm having about 70 inner nodes here to make it work better
    // for verification
    let user_specified_inner_nodes = 70; 
    let initial_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(100.0);

    let mut static_mixer_41_label_6 = InsulatedFluidComponent::new_custom_component(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        reynolds_coefficient, 
        reynolds_power, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        component_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes);

    // now, i want to replace the inner nusselt number by 4000.0
    // just for verification 
    let laminar_nusselt_correlation: NusseltCorrelation = 
        NusseltCorrelation::FixedNusselt(4000.0.into());

    let mut themrinol_array: FluidArray = 
        static_mixer_41_label_6.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    themrinol_array.nusselt_correlation = laminar_nusselt_correlation;

    static_mixer_41_label_6.pipe_fluid_array = 
        themrinol_array.into();

    // first calculate analytical solution

    let average_expected_temp = 
        ThermodynamicTemperature::new::<degree_celsius>(100.0);
    let total_thermal_resistance_estimate = 
        calc_overall_thermal_resistance_for_pipe(
            htc_to_ambient, 
            shell_id, 
            shell_od, 
            insulation_thickness, 
            component_length, 
            laminar_nusselt_correlation, 
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

    // now this is the simulation 

    let inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(inlet_temp_degc);
    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    // time settings 

    let max_time = Time::new::<second>(4000.0);
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
            = static_mixer_41_label_6.pipe_fluid_array.clone().try_into().unwrap();

        let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            therminol_array_clone.get_temperature_vector().unwrap();

        let steel_array_clone: SolidColumn 
            = static_mixer_41_label_6.pipe_shell.clone().try_into().unwrap();

        let _steel_array_temperature: Vec<ThermodynamicTemperature> = 
            steel_array_clone.get_temperature_vector().unwrap();


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

        // make axial connections to BCs 

        static_mixer_41_label_6.pipe_fluid_array.link_to_back(
            &mut inlet_bc,
            inlet_interaction
        ).unwrap();

        static_mixer_41_label_6.pipe_fluid_array.link_to_front(
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
        let insulated_fluid_component_join_handle: JoinHandle<InsulatedFluidComponent> 
            = static_mixer_41_label_6.
            lateral_connection_thread_spawn(
                mass_flowrate,
                Power::ZERO);

        static_mixer_41_label_6 = insulated_fluid_component_join_handle.join().unwrap();

        //// calculate timestep (serial method)
        //heater_v2_bare.advance_timestep(
        //    timestep);

        // calculate timestep (thread spawn method, parallel) 
        let insulated_fluid_component_join_handle: JoinHandle<InsulatedFluidComponent> 
            = static_mixer_41_label_6.advance_timestep_thread_spawn(
                timestep);

        static_mixer_41_label_6 = insulated_fluid_component_join_handle.join().unwrap();

        simulation_time += timestep;

        let _time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();

        // print outlet temperature 

        simulated_outlet_temperature = exit_temperature;

        // print loop time 
    }


    approx::assert_relative_eq!(
        analytical_outlet_temp_degc,
        t_out_expected_regression_degc,
        max_relative=1e-5
        );


    approx::assert_relative_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        t_out_calculated_by_pipe_degc,
        max_relative=1e-5
        );

    approx::assert_abs_diff_eq!(
        analytical_outlet_temp_degc,
        simulated_outlet_temperature.get::<degree_celsius>(),
        epsilon=0.2
        );

}


/// 9m test
/// 12 nodes (testing for mesh refinement)
/// note: probably hasn't reached steady state yet
/// but that takes real long to test
#[test]
pub fn static_mixer_9_meter_test_reduced_insulation_thickness_ambient_resistance_and_increase_nusselt(){

    // testings 
    let (l_meters, 
        t_out_expected_regression_degc, 
        t_out_calculated_by_pipe_degc) 
        = (9.00, 97.868,98.0925);


    // temperature

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(l_meters);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(51.526384);
    let form_loss = Ratio::new::<ratio>(21.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(4000.0);
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

    let mut static_mixer_41_label_6 = InsulatedFluidComponent::new_custom_component(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        reynolds_coefficient, 
        reynolds_power, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        component_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes);

    // now, i want to replace the inner nusselt number by 4000.0
    // just for verification 
    let laminar_nusselt_correlation: NusseltCorrelation = 
        NusseltCorrelation::FixedNusselt(4000.0.into());

    let mut themrinol_array: FluidArray = 
        static_mixer_41_label_6.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    themrinol_array.nusselt_correlation = laminar_nusselt_correlation;

    static_mixer_41_label_6.pipe_fluid_array = 
        themrinol_array.into();

    // first calculate analytical solution

    let average_expected_temp = 
        ThermodynamicTemperature::new::<degree_celsius>(100.0);
    let total_thermal_resistance_estimate = 
        calc_overall_thermal_resistance_for_pipe(
            htc_to_ambient, 
            shell_id, 
            shell_od, 
            insulation_thickness, 
            component_length, 
            laminar_nusselt_correlation, 
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

    // now this is the simulation 

    let inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(inlet_temp_degc);
    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    // time settings 

    let max_time = Time::new::<second>(4000.0);
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
            = static_mixer_41_label_6.pipe_fluid_array.clone().try_into().unwrap();

        let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            therminol_array_clone.get_temperature_vector().unwrap();

        let steel_array_clone: SolidColumn 
            = static_mixer_41_label_6.pipe_shell.clone().try_into().unwrap();

        let _steel_array_temperature: Vec<ThermodynamicTemperature> = 
            steel_array_clone.get_temperature_vector().unwrap();


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

        // make axial connections to BCs 

        static_mixer_41_label_6.pipe_fluid_array.link_to_back(
            &mut inlet_bc,
            inlet_interaction
        ).unwrap();

        static_mixer_41_label_6.pipe_fluid_array.link_to_front(
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
        let insulated_fluid_component_join_handle: JoinHandle<InsulatedFluidComponent> 
            = static_mixer_41_label_6.
            lateral_connection_thread_spawn(
                mass_flowrate,
                Power::ZERO);

        static_mixer_41_label_6 = insulated_fluid_component_join_handle.join().unwrap();

        //// calculate timestep (serial method)
        //heater_v2_bare.advance_timestep(
        //    timestep);

        // calculate timestep (thread spawn method, parallel) 
        let insulated_fluid_component_join_handle: JoinHandle<InsulatedFluidComponent> 
            = static_mixer_41_label_6.advance_timestep_thread_spawn(
                timestep);

        static_mixer_41_label_6 = insulated_fluid_component_join_handle.join().unwrap();

        simulation_time += timestep;

        let _time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();

        // print outlet temperature 

        simulated_outlet_temperature = exit_temperature;

    }


    approx::assert_relative_eq!(
        analytical_outlet_temp_degc,
        t_out_expected_regression_degc,
        max_relative=1e-5
        );


    approx::assert_relative_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        t_out_calculated_by_pipe_degc,
        max_relative=1e-5
        );

    approx::assert_abs_diff_eq!(
        analytical_outlet_temp_degc,
        simulated_outlet_temperature.get::<degree_celsius>(),
        epsilon=0.3
        );

}


/// 9m test
/// 92 nodes, extra long time given
/// note: probably hasn't reached steady state yet
/// but that takes real long to test
///
/// This shows mesh refinement has little significant impact 
/// comapred with the 10 node version for this kind of setup
#[test]
pub fn static_mixer_9_meter_test_reduced_insulation_thickness_ambient_resistance_and_increase_nusselt_extra_time(){

    // testings 
    let (l_meters, 
        t_out_expected_regression_degc, 
        t_out_calculated_by_pipe_degc) 
        = (9.00, 97.868,98.09097);


    // temperature

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(l_meters);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(51.526384);
    let form_loss = Ratio::new::<ratio>(21.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(4000.0);
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
    // however, I'm having about 92 inner nodes here to make it work better
    // for verification
    let user_specified_inner_nodes = 92; 
    let initial_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(100.0);

    let mut static_mixer_41_label_6 = InsulatedFluidComponent::new_custom_component(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        reynolds_coefficient, 
        reynolds_power, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        component_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes);

    // now, i want to replace the inner nusselt number by 4000.0
    // just for verification 
    let laminar_nusselt_correlation: NusseltCorrelation = 
        NusseltCorrelation::FixedNusselt(4000.0.into());

    let mut themrinol_array: FluidArray = 
        static_mixer_41_label_6.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    themrinol_array.nusselt_correlation = laminar_nusselt_correlation;

    static_mixer_41_label_6.pipe_fluid_array = 
        themrinol_array.into();

    // first calculate analytical solution

    let average_expected_temp = 
        ThermodynamicTemperature::new::<degree_celsius>(100.0);
    let total_thermal_resistance_estimate = 
        calc_overall_thermal_resistance_for_pipe(
            htc_to_ambient, 
            shell_id, 
            shell_od, 
            insulation_thickness, 
            component_length, 
            laminar_nusselt_correlation, 
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

    // now this is the simulation 

    let inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(inlet_temp_degc);
    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    // time settings 

    let max_time = Time::new::<second>(9000.0);
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
            = static_mixer_41_label_6.pipe_fluid_array.clone().try_into().unwrap();

        let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            therminol_array_clone.get_temperature_vector().unwrap();

        let steel_array_clone: SolidColumn 
            = static_mixer_41_label_6.pipe_shell.clone().try_into().unwrap();

        let _steel_array_temperature: Vec<ThermodynamicTemperature> = 
            steel_array_clone.get_temperature_vector().unwrap();


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

        // make axial connections to BCs 

        static_mixer_41_label_6.pipe_fluid_array.link_to_back(
            &mut inlet_bc,
            inlet_interaction
        ).unwrap();

        static_mixer_41_label_6.pipe_fluid_array.link_to_front(
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
        let insulated_fluid_component_join_handle: JoinHandle<InsulatedFluidComponent> 
            = static_mixer_41_label_6.
            lateral_connection_thread_spawn(
                mass_flowrate,
                Power::ZERO);

        static_mixer_41_label_6 = insulated_fluid_component_join_handle.join().unwrap();

        //// calculate timestep (serial method)
        //heater_v2_bare.advance_timestep(
        //    timestep);

        // calculate timestep (thread spawn method, parallel) 
        let insulated_fluid_component_join_handle: JoinHandle<InsulatedFluidComponent> 
            = static_mixer_41_label_6.advance_timestep_thread_spawn(
                timestep);

        static_mixer_41_label_6 = insulated_fluid_component_join_handle.join().unwrap();

        simulation_time += timestep;

        let _time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();

        // print outlet temperature 

        simulated_outlet_temperature = exit_temperature;

    }


    approx::assert_relative_eq!(
        analytical_outlet_temp_degc,
        t_out_expected_regression_degc,
        max_relative=1e-5
        );


    approx::assert_relative_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        t_out_calculated_by_pipe_degc,
        max_relative=1e-5
        );

    approx::assert_abs_diff_eq!(
        analytical_outlet_temp_degc,
        simulated_outlet_temperature.get::<degree_celsius>(),
        epsilon=0.3
        );

}



// I suspect from the 9m tests that axial conduction may have something 
// to do with the discrepancies. 
//
// In that case, I want to try the 1m test with greatly increased 
// radial thermal conductance so that the axial conduction effect 
// may be reduced.
//
// This should make for better agreement with expt data
//
//
// 1m test
// with greatly reduced thermal resistance
//
// 
// the other thing was that perhaps axial conduction in the fluid may 
// cause it to be problematic of sorts
// So i'm reducing the hydraulic diameter
//
// Anyhow, there is about a 1K discrepancy here.. I'm not entirely sure why
//
#[test]
pub fn static_mixer_41_label_6_1_meter_test_reduced_thickness_increased_ua(){

    // testings 
    let (l_meters, 
        t_out_expected_regression_degc, 
        t_out_calculated_by_pipe_degc) 
        = (1.00, 88.622, 89.616);


    // temperature

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(1.79e-2);
    let component_length = Length::new::<meter>(l_meters);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(51.526384);
    let form_loss = Ratio::new::<ratio>(21.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(4000.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0000508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20000.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    //
    // however, I'm having about 10 inner nodes here to make it work better
    // for verification
    let user_specified_inner_nodes = 10; 
    let initial_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(100.0);

    let mut static_mixer_41_label_6 = InsulatedFluidComponent::new_custom_component(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        reynolds_coefficient, 
        reynolds_power, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        component_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes);

    // now, i want to replace the inner nusselt number by 4000.0
    // just for verification 
    let laminar_nusselt_correlation: NusseltCorrelation = 
        NusseltCorrelation::FixedNusselt(400000.0.into());

    let mut themrinol_array: FluidArray = 
        static_mixer_41_label_6.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    themrinol_array.nusselt_correlation = laminar_nusselt_correlation;

    static_mixer_41_label_6.pipe_fluid_array = 
        themrinol_array.into();

    // first calculate analytical solution

    let average_expected_temp = 
        ThermodynamicTemperature::new::<degree_celsius>(95.0);
    let total_thermal_resistance_estimate = 
        calc_overall_thermal_resistance_for_pipe(
            htc_to_ambient, 
            shell_id, 
            shell_od, 
            insulation_thickness, 
            component_length, 
            laminar_nusselt_correlation, 
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

    // now this is the simulation 

    let inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(inlet_temp_degc);
    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    // time settings 

    let max_time = Time::new::<second>(2000.0);
    let timestep = Time::new::<second>(0.05);
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
            = static_mixer_41_label_6.pipe_fluid_array.clone().try_into().unwrap();

        let therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            therminol_array_clone.get_temperature_vector().unwrap();

        let steel_array_clone: SolidColumn 
            = static_mixer_41_label_6.pipe_shell.clone().try_into().unwrap();

        let _steel_array_temperature: Vec<ThermodynamicTemperature> = 
            steel_array_clone.get_temperature_vector().unwrap();


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

        // make axial connections to BCs 

        static_mixer_41_label_6.pipe_fluid_array.link_to_back(
            &mut inlet_bc,
            inlet_interaction
        ).unwrap();

        static_mixer_41_label_6.pipe_fluid_array.link_to_front(
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
        let insulated_fluid_component_join_handle: JoinHandle<InsulatedFluidComponent> 
            = static_mixer_41_label_6.
            lateral_connection_thread_spawn(
                mass_flowrate,
                Power::ZERO);

        static_mixer_41_label_6 = insulated_fluid_component_join_handle.join().unwrap();

        //// calculate timestep (serial method)
        //heater_v2_bare.advance_timestep(
        //    timestep);

        // calculate timestep (thread spawn method, parallel) 
        let insulated_fluid_component_join_handle: JoinHandle<InsulatedFluidComponent> 
            = static_mixer_41_label_6.advance_timestep_thread_spawn(
                timestep);

        static_mixer_41_label_6 = insulated_fluid_component_join_handle.join().unwrap();

        simulation_time += timestep;

        let _time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();

        // print outlet temperature 

        simulated_outlet_temperature = exit_temperature;

    }


    approx::assert_relative_eq!(
        analytical_outlet_temp_degc,
        t_out_expected_regression_degc,
        max_relative=1e-5
        );


    approx::assert_relative_eq!(
        simulated_outlet_temperature.get::<degree_celsius>(),
        t_out_calculated_by_pipe_degc,
        max_relative=1e-5
        );

    approx::assert_abs_diff_eq!(
        analytical_outlet_temp_degc,
        simulated_outlet_temperature.get::<degree_celsius>(),
        epsilon=0.05
        );

}
