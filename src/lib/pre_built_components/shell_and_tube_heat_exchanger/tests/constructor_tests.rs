/// checks whether constructor for Du shell and tube heat 
/// exchanger is working properly
#[test]
pub fn du_heat_exchanger_constructor_test(){

    use std::f64::consts::PI;


    use uom::si::angle::degree;
    //use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::length::meter;
    use uom::si::pressure::atmosphere;
    use uom::si::ratio::ratio;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::ConstZero;

    use crate::heat_transfer_correlations::nusselt_number_correlations::input_structs::GnielinskiData;


    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
    use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
    use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
    use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::pre_built_components::shell_and_tube_heat_exchanger::SimpleShellAndTubeHeatExchanger;

    use uom::si::f64::*;
    let number_of_tubes = 19_u32;
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
        - number_of_tubes as f64 *
        PI * 0.25 * tube_side_od * tube_side_od;

    let shell_side_fluid_hydraulic_diameter: Length = 
        (shell_side_id * shell_side_id - number_of_tubes as f64 *
         tube_side_od * tube_side_od)/
        (shell_side_id + number_of_tubes as f64 * tube_side_od);

    let hitec: LiquidMaterial = LiquidMaterial::HITEC;
    let yd325: LiquidMaterial = LiquidMaterial::YD325;
    let steel: SolidMaterial = SolidMaterial::SteelSS304L;

    let form_loss = Ratio::new::<ratio>(0.0);

    let inlet_temp_salt = 
        ThermodynamicTemperature::new::<degree_celsius>(214.93);
    let inlet_temp_oil = 
        ThermodynamicTemperature::new::<degree_celsius>(74.49);
    // ambient temperature is 25C 
    let ambient_temperature = ThermodynamicTemperature::
        new::<degree_celsius>(25.0);

    // adiabatic, heat transfer for ambient is zero 
    let heat_transfer_to_ambient = HeatTransfer::ZERO;

    // perfomed mesh refinement up to 25 inner nodes, 
    // 12 is sufficient
    let number_of_inner_nodes = 12;

    let incline_angle = Angle::new::<degree>(0.0);

    // inner fluid_array
    // the nusselt correlation here is a standard pipe correlation 
    // but I don't use that
    let tube_side_fluid_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            pipe_length,
            tube_side_id,
            tube_side_flow_area,
            inlet_temp_oil,
            fluid_pressure,
            steel,
            yd325,
            form_loss,
            number_of_inner_nodes,
            incline_angle
        );

    // shell side fluid_array
    // the nusselt correlation here is a standard pipe correlation 
    // but I don't use that
    //
    // the reason is because for the shell side, the heat transfer 
    // to the outer shell and heat transfer to inner tubes will 
    // be different. 
    //
    // The fluid array, unfortunately, only has one nusselt correlation 
    // by default.
    let shell_side_fluid_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            pipe_length,
            shell_side_fluid_hydraulic_diameter,
            shell_side_flow_area,
            inlet_temp_salt,
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
            inlet_temp_salt, 
            solid_pressure, 
            steel, 
            number_of_inner_nodes
        );

    let inner_shell: SolidColumn 
        = SolidColumn::new_cylindrical_shell(
            pipe_length, 
            tube_side_id, 
            tube_side_od, 
            inlet_temp_salt, 
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
    // nusselt correlations
    let shell_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            pipe_length, 
            SolidMaterial::SteelSS304L.surface_roughness().unwrap(), 
            shell_side_fluid_hydraulic_diameter, 
            form_loss
        );


    // for tube loss correlations, we need to use the 
    // darcy_friction_factor
    let tube_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            pipe_length, 
            SolidMaterial::SteelSS304L.surface_roughness().unwrap(), 
            tube_side_id, 
            form_loss
        );

    // nusselt correlations, 4.36 by default

    let shell_side_length_to_diameter: Ratio = 
        pipe_length/shell_side_fluid_hydraulic_diameter;

    // note that prandtl, reynolds and darcy friction factor for shell 
    // side are all arbitrary, will get overwritten later
    let dummy_ratio = Ratio::new::<ratio>(0.1);
    let shell_side_gnielinski_data: GnielinskiData = 
        GnielinskiData {
            reynolds: dummy_ratio,
            prandtl_bulk: dummy_ratio,
            prandtl_wall: dummy_ratio,
            darcy_friction_factor: dummy_ratio,
            length_to_diameter: shell_side_length_to_diameter,
        };

    let c: Ratio = Ratio::new::<ratio>(0.04318);
    let m: f64 = 0.7797;
    let shell_side_nusselt_correlation_to_tubes = 
        NusseltCorrelation::CustomGnielinskiGenericPrandtlBulk(
            shell_side_gnielinski_data, c, m);

    let tube_side_length_to_diameter: Ratio = 
        pipe_length/tube_side_id;

    // note that prandtl, reynolds and darcy friction factor for shell 
    // side are all arbitrary, will get overwritten later
    let tube_side_gnielinski_data: GnielinskiData = 
        GnielinskiData {
            reynolds: dummy_ratio,
            prandtl_bulk: dummy_ratio,
            prandtl_wall: dummy_ratio,
            darcy_friction_factor: dummy_ratio,
            length_to_diameter: tube_side_length_to_diameter,
        };

    let tube_side_nusselt_correlation = 
        NusseltCorrelation::PipeGnielinskiGeneric(tube_side_gnielinski_data);

    let shell_side_nusselt_correlation_to_outer_shell = 
        NusseltCorrelation::FixedNusselt(Ratio::ZERO);

    // we are not going to use this anyway
    let dummy_insulation_thickness =
        Length::new::<meter>(1.0);


    let du_heat_exchanger_reference 
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
            shell_side_nusselt_correlation_to_tubes: shell_side_nusselt_correlation_to_tubes.clone(), 
            shell_side_nusselt_correlation_parasitic: shell_side_nusselt_correlation_to_outer_shell, 
            tube_side_nusselt_correlation: tube_side_nusselt_correlation.clone(), 
            insulation_thickness: dummy_insulation_thickness,
        };

    let du_heat_exchanger_test = 
        SimpleShellAndTubeHeatExchanger::new_du_et_al_sthe();

    assert_eq!(du_heat_exchanger_reference,
        du_heat_exchanger_test)
}


/// checks whether constructor for insulated sthe
/// is working properly
#[test]
#[ignore="debugging"]
pub fn insulated_sthe_constructor_test(){

    use std::f64::consts::PI;


    use uom::si::angle::degree;
    //use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::length::meter;
    use uom::si::pressure::atmosphere;
    use uom::si::ratio::ratio;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::ConstZero;

    use crate::heat_transfer_correlations::nusselt_number_correlations::input_structs::GnielinskiData;


    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
    use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
    use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
    use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::pre_built_components::shell_and_tube_heat_exchanger::SimpleShellAndTubeHeatExchanger;

    use uom::si::f64::*;
    let number_of_tubes = 19_u32;
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
        - number_of_tubes as f64 *
        PI * 0.25 * tube_side_od * tube_side_od;

    let shell_side_fluid_hydraulic_diameter: Length = 
        (shell_side_id * shell_side_id - number_of_tubes as f64 *
         tube_side_od * tube_side_od)/
        (shell_side_id + number_of_tubes as f64 * tube_side_od);

    let hitec: LiquidMaterial = LiquidMaterial::HITEC;
    let yd325: LiquidMaterial = LiquidMaterial::YD325;
    let steel: SolidMaterial = SolidMaterial::SteelSS304L;

    let form_loss = Ratio::new::<ratio>(0.0);

    let inlet_temp_salt = 
        ThermodynamicTemperature::new::<degree_celsius>(214.93);
    let inlet_temp_oil = 
        ThermodynamicTemperature::new::<degree_celsius>(74.49);
    // ambient temperature is 25C 
    let ambient_temperature = ThermodynamicTemperature::
        new::<degree_celsius>(25.0);

    // adiabatic, heat transfer for ambient is zero 
    let heat_transfer_to_ambient = HeatTransfer::ZERO;

    // perfomed mesh refinement up to 25 inner nodes, 
    // 12 is sufficient
    let number_of_inner_nodes = 12;

    let incline_angle = Angle::new::<degree>(0.0);

    // inner fluid_array
    // the nusselt correlation here is a standard pipe correlation 
    // but I don't use that
    let tube_side_fluid_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            pipe_length,
            tube_side_id,
            tube_side_flow_area,
            inlet_temp_oil,
            fluid_pressure,
            steel,
            yd325,
            form_loss,
            number_of_inner_nodes,
            incline_angle
        );

    // shell side fluid_array
    // the nusselt correlation here is a standard pipe correlation 
    // but I don't use that
    //
    // the reason is because for the shell side, the heat transfer 
    // to the outer shell and heat transfer to inner tubes will 
    // be different. 
    //
    // The fluid array, unfortunately, only has one nusselt correlation 
    // by default.
    let shell_side_fluid_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            pipe_length,
            shell_side_fluid_hydraulic_diameter,
            shell_side_flow_area,
            inlet_temp_salt,
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
            inlet_temp_salt, 
            solid_pressure, 
            steel, 
            number_of_inner_nodes
        );

    let inner_shell: SolidColumn 
        = SolidColumn::new_cylindrical_shell(
            pipe_length, 
            tube_side_id, 
            tube_side_od, 
            inlet_temp_oil, 
            solid_pressure, 
            steel, 
            number_of_inner_nodes
        );
    // we are not going to use this anyway
    let dummy_insulation_thickness =
        Length::new::<meter>(1.0);

    // dummy insulation array, will not be used,
    // just clone the inner shell 
    let dummy_insulation_array 
            = SolidColumn::new_cylindrical_shell(
                pipe_length, 
                shell_side_od, 
                shell_side_od + dummy_insulation_thickness, 
                inlet_temp_salt, 
                solid_pressure, 
                steel, 
                number_of_inner_nodes
            );

    // loss correlations, use pipe by default 
    // but none are used in calculations depending on 
    // nusselt correlations
    let shell_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            pipe_length, 
            SolidMaterial::SteelSS304L.surface_roughness().unwrap(), 
            shell_side_fluid_hydraulic_diameter, 
            form_loss
        );


    // for tube loss correlations, we need to use the 
    // darcy_friction_factor
    let tube_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            pipe_length, 
            SolidMaterial::SteelSS304L.surface_roughness().unwrap(), 
            tube_side_id, 
            form_loss
        );

    // nusselt correlations, 4.36 by default

    let shell_side_length_to_diameter: Ratio = 
        pipe_length/shell_side_fluid_hydraulic_diameter;

    // note that prandtl, reynolds and darcy friction factor for shell 
    // side are all arbitrary, will get overwritten later
    let dummy_ratio = Ratio::new::<ratio>(0.1);
    let shell_side_gnielinski_data: GnielinskiData = 
        GnielinskiData {
            reynolds: dummy_ratio,
            prandtl_bulk: dummy_ratio,
            prandtl_wall: dummy_ratio,
            darcy_friction_factor: dummy_ratio,
            length_to_diameter: shell_side_length_to_diameter,
        };

    let c: Ratio = Ratio::new::<ratio>(0.04318);
    let m: f64 = 0.7797;
    let shell_side_nusselt_correlation_to_tubes = 
        NusseltCorrelation::CustomGnielinskiGenericPrandtlBulk(
            shell_side_gnielinski_data, c, m);

    let tube_side_length_to_diameter: Ratio = 
        pipe_length/tube_side_id;

    // note that prandtl, reynolds and darcy friction factor for shell 
    // side are all arbitrary, will get overwritten later
    let tube_side_gnielinski_data: GnielinskiData = 
        GnielinskiData {
            reynolds: dummy_ratio,
            prandtl_bulk: dummy_ratio,
            prandtl_wall: dummy_ratio,
            darcy_friction_factor: dummy_ratio,
            length_to_diameter: tube_side_length_to_diameter,
        };

    let tube_side_nusselt_correlation = 
        NusseltCorrelation::PipeGnielinskiGeneric(tube_side_gnielinski_data);

    let shell_side_nusselt_correlation_to_outer_shell = 
        NusseltCorrelation::FixedNusselt(Ratio::ZERO);



    // i construct a modified version of du's heat exchanger
    let modified_du_heat_exchanger_reference 
        = SimpleShellAndTubeHeatExchanger{ 
            inner_nodes: number_of_inner_nodes, 
            inner_pipe_shell_array_for_single_tube: inner_shell.into(), 
            tube_side_fluid_array_for_single_tube: tube_side_fluid_array.into(), 
            shell_side_fluid_array: shell_side_fluid_array.into(), 
            outer_shell: outer_shell.into(), 
            ambient_temperature: ambient_temperature.into(), 
            heat_transfer_to_ambient, 
            insulation_array: dummy_insulation_array.into(), 
            heat_exchanger_has_insulation: true, 
            tube_side_od, 
            tube_side_id, 
            tube_side_flow_area, 
            tube_side_custom_component_loss_correlation: tube_loss_correlations, 
            shell_side_custom_component_loss_correlation: shell_loss_correlations, 
            number_of_tubes, 
            shell_side_id, 
            shell_side_od, 
            shell_side_flow_area, 
            shell_side_nusselt_correlation_to_tubes: shell_side_nusselt_correlation_to_tubes.clone(), 
            shell_side_nusselt_correlation_parasitic: shell_side_nusselt_correlation_to_outer_shell, 
            tube_side_nusselt_correlation: tube_side_nusselt_correlation.clone(), 
            insulation_thickness: dummy_insulation_thickness,
        };

    let sthe_length = pipe_length;
    let tube_side_form_loss = form_loss;
    let shell_side_form_loss = form_loss;
    let insulation_thickness = dummy_insulation_thickness;
    let tube_side_incline_angle = incline_angle;
    let shell_side_incline_angle = incline_angle;
    let shell_side_liquid = hitec;
    let tube_side_liquid = yd325;
    let inner_tube_material = steel;
    let outer_tube_material = steel;
    let insulation_material = steel;
    let tube_side_initial_temperature = inlet_temp_oil;
    let shell_side_initial_temperature = inlet_temp_salt;


    let sthe_with_insulation = 
        SimpleShellAndTubeHeatExchanger::_new_sthe_with_insulation(
            number_of_tubes, 
            number_of_inner_nodes, 
            tube_side_od, 
            tube_side_id, 
            shell_side_od, 
            shell_side_id, 
            sthe_length, 
            tube_side_form_loss, 
            shell_side_form_loss, 
            insulation_thickness, 
            tube_side_incline_angle, 
            shell_side_incline_angle, 
            shell_side_liquid, 
            tube_side_liquid, 
            inner_tube_material, 
            outer_tube_material, 
            insulation_material, 
            ambient_temperature, 
            heat_transfer_to_ambient, 
            tube_side_initial_temperature, 
            shell_side_initial_temperature, 
            shell_loss_correlations, 
            tube_loss_correlations, 
            tube_side_nusselt_correlation, 
            shell_side_nusselt_correlation_to_tubes, 
            shell_side_nusselt_correlation_to_outer_shell);

    // have to assert equal bit by bit, cos i wasn't getting it right
    assert_eq!(
        modified_du_heat_exchanger_reference.outer_shell,
        sthe_with_insulation.outer_shell);

    assert_eq!(
        modified_du_heat_exchanger_reference.inner_pipe_shell_array_for_single_tube,
        sthe_with_insulation.inner_pipe_shell_array_for_single_tube);

    assert_eq!(
        modified_du_heat_exchanger_reference.ambient_temperature,
        sthe_with_insulation.ambient_temperature);

    assert_eq!(
        modified_du_heat_exchanger_reference.heat_transfer_to_ambient,
        sthe_with_insulation.heat_transfer_to_ambient);

    assert_eq!(
        modified_du_heat_exchanger_reference.tube_side_flow_area,
        sthe_with_insulation.tube_side_flow_area);

    assert_eq!(
        modified_du_heat_exchanger_reference.tube_side_custom_component_loss_correlation,
        sthe_with_insulation.tube_side_custom_component_loss_correlation);

    assert_eq!(
        modified_du_heat_exchanger_reference.shell_side_custom_component_loss_correlation,
        sthe_with_insulation.shell_side_custom_component_loss_correlation);

    assert_eq!(
        modified_du_heat_exchanger_reference.insulation_array,
        sthe_with_insulation.insulation_array);


    assert_eq!(
        modified_du_heat_exchanger_reference,
        sthe_with_insulation);
}
