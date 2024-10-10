
/// this is a basic test for shell and tube heat exchanger
///
/// I'm assuming an adiabatic bc to the outside
/// and switching off the insulation boolean
///
/// for this internal consistency test, 
/// 1 + (T_in,t - T_out,t)/(T_in,s - T_out,t) 
/// = exp ( [UA]/[m_t c_p] )
#[test]
pub fn overall_htc_postprocessing_basic_test_shell_and_tube_heat_exchanger(){

    use std::f64::consts::PI;

    use uom::si::angle::degree;
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::length::{meter, millimeter};
    use uom::si::pressure::atmosphere;
    use uom::si::ratio::ratio;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::ConstZero;

    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
    use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
    use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
    use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::pre_built_components::shell_and_tube_heat_exchanger::SimpleShellAndTubeHeatExchanger;
    use uom::si::f64::*;
    let number_of_tubes = 1_u32;
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);

    // from Du's heat exchanger type, except we use one inner tube
    let tube_side_od = Length::new::<meter>(0.014);
    let tube_side_id = Length::new::<meter>(0.01);
    let shell_side_od = Length::new::<meter>(0.108);
    let shell_side_id = Length::new::<meter>(0.1);
    let pipe_length = Length::new::<meter>(1.95);

    let tube_side_flow_area: Area 
        = PI * 0.25 * tube_side_id * tube_side_id;

    let shell_side_flow_area: Area 
        = PI * 0.25 * shell_side_id * shell_side_id 
        - PI * 0.25 * tube_side_od * tube_side_od;

    let shell_side_fluid_hydraulic_diameter: Length = 
        (shell_side_id * shell_side_id - tube_side_od * tube_side_od)/
        (shell_side_id + tube_side_od);


    let hitec: LiquidMaterial = LiquidMaterial::HITEC;
    let steel: SolidMaterial = SolidMaterial::SteelSS304L;

    let form_loss = Ratio::new::<ratio>(0.0);

    // initial temperature is 250C 
    let initial_temperature = ThermodynamicTemperature::
        new::<degree_celsius>(250.0);
    // ambient temperature is 25C 
    let ambient_temperature = ThermodynamicTemperature::
        new::<degree_celsius>(25.0);

    // adiabatic, heat transfer for ambient is zero 
    let heat_transfer_to_ambient = HeatTransfer::ZERO;

    let number_of_inner_nodes = 8;

    let incline_angle = Angle::new::<degree>(0.0);

    // inner fluid_array
    // the nusselt correlation here is a standard pipe correlation 
    // but I don't use that
    let tube_side_fluid_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            pipe_length,
            tube_side_id,
            tube_side_flow_area,
            initial_temperature,
            fluid_pressure,
            steel,
            hitec,
            form_loss,
            number_of_inner_nodes,
            incline_angle
        );

    // shell side fluid_array
    // the nusselt correlation here is a standard pipe correlation 
    // but I don't use that
    let shell_side_fluid_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            pipe_length,
            shell_side_fluid_hydraulic_diameter,
            shell_side_flow_area,
            initial_temperature,
            fluid_pressure,
            steel,
            hitec,
            form_loss,
            number_of_inner_nodes,
            incline_angle
        );

    let outer_shell: SolidColumn 
        = SolidColumn::new_cylindrical_shell(
            pipe_length, 
            shell_side_id, 
            shell_side_od, 
            initial_temperature, 
            solid_pressure, 
            steel, 
            number_of_inner_nodes
            );

    let inner_shell: SolidColumn 
        = SolidColumn::new_cylindrical_shell(
            pipe_length, 
            tube_side_id, 
            tube_side_od, 
            initial_temperature, 
            solid_pressure, 
            steel, 
            number_of_inner_nodes
            );

    // dummy insulation array, will not be used,
    // just clone the inner shell 
    let dummy_insulation_array 
        = inner_shell.clone();

    // loss correlations, use pipe by default 
    // but none are used in calculations depending on 
    // nusselt correlaitons
    let shell_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            pipe_length, 
            Length::new::<millimeter>(0.001), 
            shell_side_fluid_hydraulic_diameter, 
            form_loss
            );


    let tube_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            pipe_length, 
            Length::new::<millimeter>(0.001), 
            tube_side_id, 
            form_loss
            );

    // nusselt correlations, 4.36 by default

    let shell_side_nusselt_correlation_to_tubes = 
        NusseltCorrelation::PipeConstantHeatFluxFullyDeveloped;

    let tube_side_nusselt_correlation = 
        NusseltCorrelation::PipeConstantHeatFluxFullyDeveloped;

    let shell_side_nusselt_correlation_to_outer_shell = 
        NusseltCorrelation::PipeConstantHeatFluxFullyDeveloped;

    // we are not going to use this anyway
    let dummy_insulation_thickness =
        Length::new::<meter>(1.0);
    

    let sthe_one_shell_one_tube 
        = SimpleShellAndTubeHeatExchanger{ 
            inner_nodes: number_of_inner_nodes, 
            inner_pipe_shell_array_for_single_tube: inner_shell.into(), 
            tube_side_fluid_array_for_single_tube: tube_side_fluid_array.into(), 
            shell_side_fluid_array: shell_side_fluid_array.into(), 
            outer_shell: outer_shell.into(), 
            ambient_temperature: ambient_temperature.into(), 
            heat_transfer_to_ambient, 
            insulation_array: dummy_insulation_array.into(), 
            heat_exchanger_has_insulation: false, 
            tube_side_od, 
            tube_side_id, 
            tube_side_flow_area, 
            tube_side_custom_component_loss_correlation: tube_loss_correlations, 
            shell_side_custom_component_loss_correlation: shell_loss_correlations, 
            number_of_tubes, 
            shell_side_id, 
            shell_side_od, 
            shell_side_flow_area, 
            shell_side_nusselt_correlation_to_tubes, 
            shell_side_nusselt_correlation_parasitic: shell_side_nusselt_correlation_to_outer_shell, 
            tube_side_nusselt_correlation, 
            insulation_thickness: dummy_insulation_thickness,
        };

    let correct_for_prandtl_wall_temperatures = true;
    let overall_heat_exchg_coeff = 
        sthe_one_shell_one_tube.overall_heat_transfer_coeff_u_shell_side(
            correct_for_prandtl_wall_temperatures).unwrap();

    let u_val_watts_per_sqm_kelvin = 
        overall_heat_exchg_coeff.get::<watt_per_square_meter_kelvin>();

    // based on manual calculations and derivations,
    // the overall heat exchg coeff at this temperature should be 
    // 18.48 W/(m^2 K) based on Du's material property correlations 
    //
    // See document
    //
    // For for steel though, the thermophysical property correlation
    // I use is different from Du's constant thermal conductivity 
    // of 16.3 W/(m^2 K), so answers will be slightly different
    // 
    // I tested that I agrees to within 4%

    approx::assert_relative_eq!(
        18.48,
        u_val_watts_per_sqm_kelvin,
        max_relative = 0.04
        );
}


/// this is a basic test for shell and tube heat exchanger
///
/// I'm assuming an adiabatic bc to the outside
/// and switching off the insulation boolean
///
/// for this internal consistency test, 
/// 1 + (T_in,t - T_out,t)/(T_in,s - T_out,t) 
/// = exp ( [UA]/[m_t c_p] )
#[test]
pub fn shell_side_tube_bundle_area_basic_test_shell_and_tube_heat_exchanger(){

    use std::f64::consts::PI;

    use uom::si::angle::degree;
    use uom::si::length::{meter, millimeter};
    use uom::si::pressure::atmosphere;
    use uom::si::ratio::ratio;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::ConstZero;

    use uom::si::area::square_meter;
    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
    use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
    use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
    use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::pre_built_components::shell_and_tube_heat_exchanger::SimpleShellAndTubeHeatExchanger;
    use uom::si::f64::*;
    let number_of_tubes = 1_u32;
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);

    // from Du's heat exchanger type, except we use one inner tube
    let tube_side_od = Length::new::<meter>(0.014);
    let tube_side_id = Length::new::<meter>(0.01);
    let shell_side_od = Length::new::<meter>(0.108);
    let shell_side_id = Length::new::<meter>(0.1);
    let pipe_length = Length::new::<meter>(1.95);

    let tube_side_flow_area: Area 
        = PI * 0.25 * tube_side_id * tube_side_id;

    let shell_side_flow_area: Area 
        = PI * 0.25 * shell_side_id * shell_side_id 
        - PI * 0.25 * tube_side_od * tube_side_od;

    let shell_side_fluid_hydraulic_diameter: Length = 
        (shell_side_id * shell_side_id - PI * tube_side_od * tube_side_od)/
        (shell_side_id + PI * tube_side_od);

    let hitec: LiquidMaterial = LiquidMaterial::HITEC;
    let steel: SolidMaterial = SolidMaterial::SteelSS304L;

    let form_loss = Ratio::new::<ratio>(0.0);

    // initial temperature is 250C 
    let initial_temperature = ThermodynamicTemperature::
        new::<degree_celsius>(250.0);
    // ambient temperature is 25C 
    let ambient_temperature = ThermodynamicTemperature::
        new::<degree_celsius>(25.0);

    // adiabatic, heat transfer for ambient is zero 
    let heat_transfer_to_ambient = HeatTransfer::ZERO;

    let number_of_inner_nodes = 8;

    let incline_angle = Angle::new::<degree>(0.0);

    // inner fluid_array
    // the nusselt correlation here is a standard pipe correlation 
    // but I don't use that
    let tube_side_fluid_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            pipe_length,
            tube_side_id,
            tube_side_flow_area,
            initial_temperature,
            fluid_pressure,
            steel,
            hitec,
            form_loss,
            number_of_inner_nodes,
            incline_angle
        );

    // shell side fluid_array
    // the nusselt correlation here is a standard pipe correlation 
    // but I don't use that
    let shell_side_fluid_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            pipe_length,
            shell_side_fluid_hydraulic_diameter,
            shell_side_flow_area,
            initial_temperature,
            fluid_pressure,
            steel,
            hitec,
            form_loss,
            number_of_inner_nodes,
            incline_angle
        );

    let outer_shell: SolidColumn 
        = SolidColumn::new_cylindrical_shell(
            pipe_length, 
            shell_side_id, 
            shell_side_od, 
            initial_temperature, 
            solid_pressure, 
            steel, 
            number_of_inner_nodes
            );

    let inner_shell: SolidColumn 
        = SolidColumn::new_cylindrical_shell(
            pipe_length, 
            tube_side_id, 
            tube_side_od, 
            initial_temperature, 
            solid_pressure, 
            steel, 
            number_of_inner_nodes
            );

    // dummy insulation array, will not be used,
    // just clone the inner shell 
    let dummy_insulation_array 
        = inner_shell.clone();

    // loss correlations, use pipe by default 
    // but none are used in calculations depending on 
    // nusselt correlaitons
    let shell_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            pipe_length, 
            Length::new::<millimeter>(0.001), 
            shell_side_fluid_hydraulic_diameter, 
            form_loss
            );


    let tube_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            pipe_length, 
            Length::new::<millimeter>(0.001), 
            tube_side_id, 
            form_loss
            );

    // nusselt correlations, 4.36 by default

    let shell_side_nusselt_correlation_to_tubes = 
        NusseltCorrelation::PipeConstantHeatFluxFullyDeveloped;

    let tube_side_nusselt_correlation = 
        NusseltCorrelation::PipeConstantHeatFluxFullyDeveloped;

    let shell_side_nusselt_correlation_to_outer_shell = 
        NusseltCorrelation::PipeConstantHeatFluxFullyDeveloped;

    // we are not going to use this anyway
    let dummy_insulation_thickness =
        Length::new::<meter>(1.0);
    

    let sthe_one_shell_one_tube 
        = SimpleShellAndTubeHeatExchanger{ 
            inner_nodes: number_of_inner_nodes, 
            inner_pipe_shell_array_for_single_tube: inner_shell.into(), 
            tube_side_fluid_array_for_single_tube: tube_side_fluid_array.into(), 
            shell_side_fluid_array: shell_side_fluid_array.into(), 
            outer_shell: outer_shell.into(), 
            ambient_temperature: ambient_temperature.into(), 
            heat_transfer_to_ambient, 
            insulation_array: dummy_insulation_array.into(), 
            heat_exchanger_has_insulation: false, 
            tube_side_od, 
            tube_side_id, 
            tube_side_flow_area, 
            tube_side_custom_component_loss_correlation: tube_loss_correlations, 
            shell_side_custom_component_loss_correlation: shell_loss_correlations, 
            number_of_tubes, 
            shell_side_id, 
            shell_side_od, 
            shell_side_flow_area, 
            shell_side_nusselt_correlation_to_tubes, 
            shell_side_nusselt_correlation_parasitic: shell_side_nusselt_correlation_to_outer_shell, 
            tube_side_nusselt_correlation, 
            insulation_thickness: dummy_insulation_thickness,
        };


    let shell_side_tube_area: Area = 
        sthe_one_shell_one_tube.circular_tube_bundle_heat_transfer_area_shell_side();

    let shell_side_tube_area_m2 = 
        shell_side_tube_area.get::<square_meter>();

    approx::assert_relative_eq!(
        0.085765,
        shell_side_tube_area_m2,
        max_relative = 1e-5
        );

}
