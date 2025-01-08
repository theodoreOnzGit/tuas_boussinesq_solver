use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::boundary_conditions::BCType;
use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
use crate::heat_transfer_correlations::thermal_resistance::try_get_thermal_conductance_annular_cylinder;
use crate::pre_built_components::insulated_pipes_and_fluid_components::InsulatedFluidComponent;
use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
use uom::si::angle::degree;
use uom::si::area::square_meter;
use uom::si::ratio::ratio;
use uom::si::{f64::*, specific_heat_capacity::joule_per_kilogram_kelvin};
use std::f64::consts::PI;
use std::time::SystemTime;
use std::thread::JoinHandle;
use uom::si::pressure::atmosphere;

use uom::{si::time::second, ConstZero};

use uom::si::length::meter;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::thermodynamic_temperature::degree_celsius;

use uom::si::mass_rate::kilogram_per_second;
/// this checks for cp of therminol_vp_1 
/// at temp of 95C
#[test]
pub fn cp_for_therminol_vp_1(){

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
        = (fluid_htc_to_pipe * PI * insulation_od * pipe_length).recip();


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


// 1m test
#[test]
pub fn static_mixer_41_label_6_1_meter_test(){

    // testings 
    let (l_meters, 
        t_out_expected_regression_degc, 
        t_out_calculated_by_pipe_degc) 
        = (1.00, 99.956,99.965);


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
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
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

    // now, i want to replace the inner nusselt number by 4.36 
    // just for verification 
    let laminar_nusselt_correlation: NusseltCorrelation = 
        NusseltCorrelation::FixedNusselt(4.36.into());

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

        let time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();

        // print outlet temperature 
        dbg!(exit_temperature
            .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));

        simulated_outlet_temperature = exit_temperature;

        // print loop time 
        dbg!(time_taken_for_calculation_loop);
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

    approx::assert_relative_eq!(
        analytical_outlet_temp_degc,
        simulated_outlet_temperature.get::<degree_celsius>(),
        max_relative=1e-4
        );

}


// 1m test
// with reduced insulation thickness 
// instead of 5.08 cm,
// it's 0.508 cm
#[test]
pub fn static_mixer_41_label_6_reduced_insulation_thickness_1_meter_test(){

    // testings 
    let (l_meters, 
        t_out_expected_regression_degc, 
        t_out_calculated_by_pipe_degc) 
        = (1.00, 99.860,99.883);


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
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
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

    // now, i want to replace the inner nusselt number by 4.36 
    // just for verification 
    let laminar_nusselt_correlation: NusseltCorrelation = 
        NusseltCorrelation::FixedNusselt(4.36.into());

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

        let time_taken_for_calculation_loop = loop_time_start.elapsed().unwrap();

        // print outlet temperature 
        dbg!(exit_temperature
            .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));

        simulated_outlet_temperature = exit_temperature;

        // print loop time 
        dbg!(time_taken_for_calculation_loop);
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

    approx::assert_relative_eq!(
        analytical_outlet_temp_degc,
        simulated_outlet_temperature.get::<degree_celsius>(),
        max_relative=5e-4
        );

}
