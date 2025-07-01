use uom::si::angle::degree;
use uom::si::area::square_meter;
use uom::si::f64::*;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::length::{meter, millimeter};
use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::pressure::atmosphere;

use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
use crate::pre_built_components::insulated_pipes_and_fluid_components::InsulatedFluidComponent;
use crate::pre_built_components::non_insulated_fluid_components::NonInsulatedFluidComponent;
use crate::pre_built_components::non_insulated_parallel_fluid_components::NonInsulatedParallelFluidComponent;


/// hot leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// pipe 34, the horizontal pipe just besides the NDHX, the 
/// heat exchanger cooling the DRACS loop
///
pub fn new_pipe_34(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    // note that for zweibaum's model,
    // the hydraulic diameter is 2.79e-2 m
    // whereas for SAM, the hydraulic diameter 
    // is 2.78638e-2 m
    // this is slightly smaller in SAM
    //
    // in Zweibaum's relap model, flow area is 6.11e-4 m^2
    // whereas for SAM
    // flow area is 6.099776e-4 m^2
    //
    // again, slightly smaller
    //
    // I tested if this impacts, 
    // it's quite insensitive
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.55245);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(0.0);
    let form_loss = Ratio::new::<ratio>(4.25);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 5 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 5-2
    let user_specified_inner_nodes = 5-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}


/// hot leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// pipe 33, the long vertical pipe just in the hot leg
///
pub fn new_pipe_33(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(3.0099);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(2.75);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 28 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 28-2
    let user_specified_inner_nodes = 28-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}


/// hot leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// pipe 32, the slanted pipe just in the hot leg next to the static mixer
///
pub fn new_pipe_32(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.238125);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(54.422897-180.0);
    let form_loss = Ratio::new::<ratio>(0.8);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
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
    let user_specified_inner_nodes = 0; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// hot leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// static mixer pipe label 31, it is static mixer 61 on the P&ID for CIET
/// 
///
pub fn new_static_mixer_61_label_31(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(0.33);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
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
    let user_specified_inner_nodes = 0; 

    let insulated_component = InsulatedFluidComponent::new_custom_component(
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

    insulated_component
}

/// hot leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// static mixer pipe 31a, the slanted pipe just in the hot leg next to the static mixer
///
pub fn new_pipe_31a(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.143075);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(1.35);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
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
    let user_specified_inner_nodes = 0; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);



    insulated_component
}


/// hot leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// DHX tube side (top head) 30b
///
pub fn new_dhx_tube_side_30b(initial_temperature: ThermodynamicTemperature) -> 
NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    // dhx tubes are modelled in SAM as 19 tubes of diameter 
    // 0.00635 m 
    // and flow area of 6.1072e-4 m^2
    //
    // in Zweibaum's RELAP model,
    // it is quite different from the SAM model 
    // which follows Bickel's data
    let hydraulic_diameter = Length::new::<meter>(6.35e-3);
    let pipe_length = Length::new::<meter>(0.18415);
    let flow_area = Area::new::<square_meter>(6.0172e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.00079375);
    let od = id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    let user_specified_inner_nodes = 2-2; 

    let non_insulated_component = NonInsulatedFluidComponent::new_bare_pipe(
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

    non_insulated_component
}

/// hot leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// DHX tube side 30
///
/// Here is where the main heat exchange happens. 
/// This one is for an isolated DHX.
///
/// Alternate code is needed for a coupled DHX
///
pub fn new_isolated_dhx_tube_side_30(initial_temperature: ThermodynamicTemperature) -> 
NonInsulatedFluidComponent {
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
    let od = id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 11 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 11-2
    let user_specified_inner_nodes = 11-2; 

    let mut non_insulated_component = NonInsulatedFluidComponent::new_bare_pipe(
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
        non_insulated_component.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_ideal_nusslet.nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    non_insulated_component.pipe_fluid_array = 
        fluid_array_ideal_nusslet.into();

    non_insulated_component
}

/// hot leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// DHX tube side 30 with parallel tube modelling
///
/// Here is where the main heat exchange happens. 
/// This one is for an isolated DHX.
///
/// Alternate code is needed for a coupled DHX
///
pub fn new_isolated_dhx_tube_side_30_parallel_tubes(
    initial_temperature: ThermodynamicTemperature) -> 
NonInsulatedParallelFluidComponent {
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
    let od = id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 11 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 11-2
    let user_specified_inner_nodes = 11-2; 
    let number_of_tubes: u32 = 19;

    let mut non_insulated_component_parallel = NonInsulatedParallelFluidComponent::new_bare_pipe_parallel_array(
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

    // for heat exchangers, I give an ideal Nusselt number correlation 
    // as an approximation so that film thermal resistance is minimised
    let mut fluid_array_ideal_nusslet: FluidArray = 
        non_insulated_component_parallel.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_ideal_nusslet.nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    non_insulated_component_parallel.pipe_fluid_array = 
        fluid_array_ideal_nusslet.into();

    non_insulated_component_parallel
}

/// hot leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// DHX tube side (bottom head) 30a
///
pub fn new_dhx_tube_side_30a(initial_temperature: ThermodynamicTemperature) -> 
NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    // dhx tubes are modelled in SAM as 19 tubes of diameter 
    // 0.00635 m 
    // and flow area of 6.1072e-4 m^2
    //
    // in Zweibaum's RELAP model,
    // it is quite different from the SAM model 
    // which follows Bickel's data
    let hydraulic_diameter = Length::new::<meter>(6.35e-3);
    let flow_area = Area::new::<square_meter>(6.0172e-4);
    let pipe_length = Length::new::<meter>(0.111125);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.00079375);
    let od = id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    let user_specified_inner_nodes = 2-2; 

    let non_insulated_component = NonInsulatedFluidComponent::new_bare_pipe(
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

    non_insulated_component
}

/// cold leg of DRACS (or what I consider the cold branch)
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
///
/// label 35a on RELAP model by Zweibaum
/// horizontal part of the TCHX or NDHX, 
/// has the same loss correlations as the CTAH (horizontal)
///
pub fn new_ndhx_tchx_horizontal_35a(
    initial_temperature: ThermodynamicTemperature) -> NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(1.19e-2);
    let component_length = Length::new::<meter>(1.148475);
    let flow_area = Area::new::<square_meter>(1.33E-03);
    let incline_angle = Angle::new::<degree>(0.0);
    let form_loss = Ratio::new::<ratio>(400.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(52000_f64);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.000406);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 11 nodes only, 
    // now because there are two outer nodes, 
    // we subtract 2 
    let user_specified_inner_nodes = 11-2; 

    let mut non_insulated_component = NonInsulatedFluidComponent::
        new_custom_component(
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
            component_length, 
            hydraulic_diameter, 
            pipe_shell_material, 
            pipe_fluid, 
            htc_to_ambient, 
            user_specified_inner_nodes);

    // for heat exchangers, I give an ideal Nusselt number correlation 
    // as an approximation so that film thermal resistance is minimised
    let mut fluid_array_ideal_nusslet: FluidArray = 
        non_insulated_component.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_ideal_nusslet.nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    non_insulated_component.pipe_fluid_array = 
        fluid_array_ideal_nusslet.into();


    non_insulated_component
}

/// cold leg of DRACS (or what I consider the cold branch)
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
///
/// label 35b on RELAP model by Zweibaum
/// horizontal part of the TCHX or NDHX, 
/// has the same loss correlations as the CTAH (horizontal)
///
pub fn new_ndhx_tchx_vertical_35b(initial_temperature: ThermodynamicTemperature) -> 
NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(1.19e-2);
    let pipe_length = Length::new::<meter>(0.415925);
    let flow_area = Area::new::<square_meter>(1.33E-03);
    let incline_angle = Angle::new::<degree>(-90.0);
    let form_loss = Ratio::new::<ratio>(5.8);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.000406);
    let od = id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::Copper;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 4 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 4-2
    let user_specified_inner_nodes = 4-2; 

    let mut non_insulated_component = NonInsulatedFluidComponent::new_bare_pipe(
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
        non_insulated_component.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_ideal_nusslet.nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    non_insulated_component.pipe_fluid_array = 
        fluid_array_ideal_nusslet.into();

    non_insulated_component
}

/// cold leg of DRACS (or what I consider the cold branch)
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
/// note that for coupled natural circulation dracs loop calibration 
/// tchx pipe 35b is evenly split into 35b-1 and 35b-2
/// 35b-1 is adiabatic towards the environment
///
/// label 35b-1 on SAM model by Zweibaum
/// horizontal part of the TCHX or NDHX, 
/// has the same loss correlations as the CTAH (horizontal)
///
pub fn new_ndhx_tchx_vertical_35b_1(initial_temperature: ThermodynamicTemperature) -> 
NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(1.19e-2);
    // even splitting here
    let pipe_length = Length::new::<meter>(0.415925*0.5);
    let flow_area = Area::new::<square_meter>(1.33E-03);
    let incline_angle = Angle::new::<degree>(-90.0);
    let form_loss = Ratio::new::<ratio>(5.8);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.000406);
    let od = id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::Copper;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    // tchx 35b1 is adiabatic
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(0.0);
    // from SAM nodalisation, we have 4 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 4-2
    let user_specified_inner_nodes = 4-2; 

    let mut non_insulated_component = NonInsulatedFluidComponent::new_bare_pipe(
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
        non_insulated_component.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_ideal_nusslet.nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    non_insulated_component.pipe_fluid_array = 
        fluid_array_ideal_nusslet.into();

    non_insulated_component
}
/// cold leg of DRACS (or what I consider the cold branch)
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
/// note that for coupled natural circulation dracs loop calibration 
/// tchx pipe 35b is evenly split into 35b-1 and 35b-2
/// 35b-2 is not adiabatic towards the environment
///
/// label 35b-2 on SAM model by Zweibaum
/// horizontal part of the TCHX or NDHX, 
/// has the same loss correlations as the CTAH (horizontal)
///
pub fn new_ndhx_tchx_vertical_35b_2(initial_temperature: ThermodynamicTemperature) -> 
NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(1.19e-2);
    // even splitting here
    let pipe_length = Length::new::<meter>(0.415925*0.5);
    let flow_area = Area::new::<square_meter>(1.33E-03);
    let incline_angle = Angle::new::<degree>(-90.0);
    let form_loss = Ratio::new::<ratio>(5.8);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let id = hydraulic_diameter;
    // these were the  original relap parameters
    let _relap_zweibaum_pipe_thickness = Length::new::<meter>(0.000406);
    // 
    // I'm calibrating pipe thickness down for decreased thermal resistance
    let pipe_thickness = Length::new::<meter>(0.000203);
    let od = id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::Copper;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    // tchx 35b1 is non adiabatic
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 4 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 4-2
    let user_specified_inner_nodes = 4-2; 

    let mut non_insulated_component = NonInsulatedFluidComponent::new_bare_pipe(
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
        non_insulated_component.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_ideal_nusslet.nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    non_insulated_component.pipe_fluid_array = 
        fluid_array_ideal_nusslet.into();

    non_insulated_component
}



/// cold leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// static mixer pipe 36a, the static mixer pipe next to the NDHX a.k.a TCHX
///
pub fn new_pipe_36a(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.2034);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(-58.99728);
    let form_loss = Ratio::new::<ratio>(3.75);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
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
    let user_specified_inner_nodes = 0; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// cold leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// static mixer MX-60 label 36
///
pub fn new_static_mixer_60_label_36(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(0.33);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(-58.99728);
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
    let user_specified_inner_nodes = 0; 

    let insulated_component = InsulatedFluidComponent::new_custom_component(
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

    insulated_component
}


/// cold leg (or branch) of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// pipe 37, a pipe next to MX-60
///
pub fn new_pipe_37(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(1.7736);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(-90.0);
    // pipe 37 has no form losses in Zweibaum's dissertation (probably 
    // a misprint) but it shows up as 14.0 on Zou's paper
    let form_loss = Ratio::new::<ratio>(14.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 16 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 16-2
    let user_specified_inner_nodes = 16-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}


/// cold leg (or branch) of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// static flowmeter label 37a
///
pub fn new_flowmeter_60_37a(initial_temperature: ThermodynamicTemperature) -> 
NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(0.36);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(-90.0);
    let form_loss = Ratio::new::<ratio>(18.1);
    let reynolds_power = -1.3476_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(93006.9_f64);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, we subtract 2
    let user_specified_inner_nodes = 2-2; 


    let non_insulated_component = NonInsulatedFluidComponent::
        new_custom_component(
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
            component_length, 
            hydraulic_diameter, 
            pipe_shell_material, 
            pipe_fluid, 
            htc_to_ambient, 
            user_specified_inner_nodes);

    non_insulated_component

}


/// cold leg (or branch) of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// pipe 38 
///
pub fn new_pipe_38(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.33655);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(-52.41533);
    let form_loss = Ratio::new::<ratio>(0.8);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 3 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 3-2
    let user_specified_inner_nodes = 3-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}


/// cold leg (or branch) of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// pipe 39, bottom of cold leg
///
pub fn new_pipe_39(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(1.91135);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(-80.64882);
    let form_loss = Ratio::new::<ratio>(2.65);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 18 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 18-2
    let user_specified_inner_nodes = 18-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}
