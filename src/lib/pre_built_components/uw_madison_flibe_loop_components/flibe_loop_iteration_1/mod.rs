use std::f64::consts::PI;

use uom::si::angle::degree;
use uom::si::f64::*;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::length::{inch, meter, millimeter};
use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::pressure::atmosphere;

use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::pre_built_components::insulated_pipes_and_fluid_components::InsulatedFluidComponent;
// [Component no.],[description],[length (m)],[angle],[Delta x],[Delta y],
// [1],[hot leg insulated heater vertical],[1.47],[90],[9.00115397373305E-17],[1.47],
// [2],[opening to tank],[0.12],[0],[0.12],[0],
// [3],[top left bend vertical],[0.0897],[-90],[5.49254089417588E-18],[-0.0897],
// [4],[top left bend diagonal],[0.0897],[-58.9],[0.0463330395993378],[-0.0768071574886494],
// [5],[cold leg horizontal],[1.35],[-10],[1.32949046656648],[-0.234425039850356],
// [6],[Cold leg corner bend],[0.0897],[-20],[0.084290428084496],[-0.0306792068563125],
// [7],[cold leg vertical],[1.53],[-90],[9.36854801347725E-17],[-1.53],
// [8],[cold to hot leg bend 1],[0.0598],[-100],[-0.0103841610244824],[-0.05889150363013],
// [9],[cold to hot leg bend 2],[0.0598],[-150],[-0.0517883191463094],[-0.0299],
// [10],[cold to hot leg bend 3],[0.0598],[180],[-0.0598],[7.32338785890117E-18],
// [11],[hot leg horizontal-ish],[1.42],[160],[-1.33436352151599],[0.48566860352245],
// [12],[hot leg bend 1],[0.0897],[130],[-0.0576580485888826],[0.0687141865477723],
// [13],[hot leg bend 2],[0.0697],[158],[-0.0646247146633051],[0.0261100795610891],

/// new university of wisconsin madison flibe pipe 1
///
/// OD of 2.54 cm (1 in) and 3mm thick wall 
/// means ID is 2.54cm - 2*3mm
///
///
/// for heat losses, and insulation, the paper used pyrogel. 
/// However, thicknesses and heat loss aren't really calibrated well yet. 
///
/// Pyrogel data seems to be limited. But potentially useful to add to 
/// the library. The actual one is Pyrogel HPS, but what I found online is 
/// Pyrogel XT and Pyrogel HT. There are several kinds of pyrogel.
pub fn new_uw_flibe_pipe_1(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {


    let shell_od = Length::new::<inch>(1.0);
    let pipe_thickness = Length::new::<millimeter>(3.0);
    let shell_id = shell_od - 2.0*pipe_thickness;

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = shell_id;
    let pipe_length = Length::new::<meter>(1.47);
    let flow_area = PI/4.0 * shell_id * shell_id;
    let incline_angle = Angle::new::<degree>(90.0);

    // form losses TBD, need to calibrate
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::PyrogelHPS;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // for first iteration, just get it adiabatic first 
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(0.0);
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


/// top left opening to tank in iteration 1 (see fig A.16)
pub fn new_uw_flibe_pipe_2(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {


    let shell_od = Length::new::<inch>(1.0);
    let pipe_thickness = Length::new::<millimeter>(3.0);
    let shell_id = shell_od - 2.0*pipe_thickness;

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = shell_id;
    let pipe_length = Length::new::<meter>(0.12);
    let flow_area = PI/4.0 * shell_id * shell_id;
    let incline_angle = Angle::new::<degree>(0.0);

    // form losses TBD, need to calibrate
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::PyrogelHPS;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // for first iteration, just get it adiabatic first 
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(0.0);
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

/// first part of the top left bend in iteration 1 (see fig A.16)
pub fn new_uw_flibe_pipe_3(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {


    let shell_od = Length::new::<inch>(1.0);
    let pipe_thickness = Length::new::<millimeter>(3.0);
    let shell_id = shell_od - 2.0*pipe_thickness;

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = shell_id;
    let pipe_length = Length::new::<meter>(0.0897);
    let flow_area = PI/4.0 * shell_id * shell_id;
    let incline_angle = Angle::new::<degree>(-90.0);

    // form losses TBD, need to calibrate
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::PyrogelHPS;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // for first iteration, just get it adiabatic first 
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(0.0);
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


/// second part of the top left bend in iteration 1 (see fig A.16)
pub fn new_uw_flibe_pipe_4(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {


    let shell_od = Length::new::<inch>(1.0);
    let pipe_thickness = Length::new::<millimeter>(3.0);
    let shell_id = shell_od - 2.0*pipe_thickness;

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = shell_id;
    let pipe_length = Length::new::<meter>(0.0897);
    let flow_area = PI/4.0 * shell_id * shell_id;
    let incline_angle = Angle::new::<degree>(-58.9);

    // form losses TBD, need to calibrate
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::PyrogelHPS;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // for first iteration, just get it adiabatic first 
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(0.0);
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


/// cold leg horizontal-ish in iteration 1 (see fig A.16)
pub fn new_uw_flibe_pipe_5(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {


    let shell_od = Length::new::<inch>(1.0);
    let pipe_thickness = Length::new::<millimeter>(3.0);
    let shell_id = shell_od - 2.0*pipe_thickness;

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = shell_id;
    let pipe_length = Length::new::<meter>(1.35);
    let flow_area = PI/4.0 * shell_id * shell_id;
    let incline_angle = Angle::new::<degree>(-10.0);

    // form losses TBD, need to calibrate
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::PyrogelHPS;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // for first iteration, just get it adiabatic first 
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(0.0);
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


/// Y-joint in iteration 1 (see fig A.16)
/// angle is wrong though, it should be -45 degrees rather than -20
pub fn new_uw_flibe_pipe_6(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {


    let shell_od = Length::new::<inch>(1.0);
    let pipe_thickness = Length::new::<millimeter>(3.0);
    let shell_id = shell_od - 2.0*pipe_thickness;

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = shell_id;
    let pipe_length = Length::new::<meter>(0.0897);
    let flow_area = PI/4.0 * shell_id * shell_id;
    let incline_angle = Angle::new::<degree>(-20.0);

    // form losses TBD, need to calibrate
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::PyrogelHPS;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // for first iteration, just get it adiabatic first 
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(0.0);
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


/// cold leg vertical in iteration 1 (see fig A.16)
pub fn new_uw_flibe_pipe_7(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {


    let shell_od = Length::new::<inch>(1.0);
    let pipe_thickness = Length::new::<millimeter>(3.0);
    let shell_id = shell_od - 2.0*pipe_thickness;

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = shell_id;
    let pipe_length = Length::new::<meter>(1.53);
    let flow_area = PI/4.0 * shell_id * shell_id;
    let incline_angle = Angle::new::<degree>(-90.0);

    // form losses TBD, need to calibrate
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::PyrogelHPS;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // for first iteration, just get it adiabatic first 
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(0.0);
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


/// cold leg to hot leg bend part 1 (bottom left) in iteration 1 (see fig A.16)
pub fn new_uw_flibe_pipe_8(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {


    let shell_od = Length::new::<inch>(1.0);
    let pipe_thickness = Length::new::<millimeter>(3.0);
    let shell_id = shell_od - 2.0*pipe_thickness;

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = shell_id;
    let pipe_length = Length::new::<meter>(0.0598);
    let flow_area = PI/4.0 * shell_id * shell_id;
    let incline_angle = Angle::new::<degree>(-100.0);

    // form losses TBD, need to calibrate
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::PyrogelHPS;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // for first iteration, just get it adiabatic first 
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(0.0);
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

/// cold leg to hot leg bend part 2 (bottom left) in iteration 1 (see fig A.16)
pub fn new_uw_flibe_pipe_9(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {


    let shell_od = Length::new::<inch>(1.0);
    let pipe_thickness = Length::new::<millimeter>(3.0);
    let shell_id = shell_od - 2.0*pipe_thickness;

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = shell_id;
    let pipe_length = Length::new::<meter>(0.0598);
    let flow_area = PI/4.0 * shell_id * shell_id;
    let incline_angle = Angle::new::<degree>(-150.0);

    // form losses TBD, need to calibrate
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::PyrogelHPS;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // for first iteration, just get it adiabatic first 
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(0.0);
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

/// cold leg to hot leg bend part 3 (bottom left) in iteration 1 (see fig A.16)
pub fn new_uw_flibe_pipe_10(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {


    let shell_od = Length::new::<inch>(1.0);
    let pipe_thickness = Length::new::<millimeter>(3.0);
    let shell_id = shell_od - 2.0*pipe_thickness;

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = shell_id;
    let pipe_length = Length::new::<meter>(0.0598);
    let flow_area = PI/4.0 * shell_id * shell_id;
    let incline_angle = Angle::new::<degree>(180.0);

    // form losses TBD, need to calibrate
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::PyrogelHPS;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // for first iteration, just get it adiabatic first 
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(0.0);
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


/// hot leg horizontal ish (diagonal) in iteration 1 (see fig A.16)
pub fn new_uw_flibe_pipe_11(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {


    let shell_od = Length::new::<inch>(1.0);
    let pipe_thickness = Length::new::<millimeter>(3.0);
    let shell_id = shell_od - 2.0*pipe_thickness;

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = shell_id;
    let pipe_length = Length::new::<meter>(1.42);
    let flow_area = PI/4.0 * shell_id * shell_id;
    let incline_angle = Angle::new::<degree>(160.0);

    // form losses TBD, need to calibrate
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::PyrogelHPS;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // for first iteration, just get it adiabatic first 
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(0.0);
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


/// hot bend 1 bottom left in iteration 1 (see fig A.16)
pub fn new_uw_flibe_pipe_12(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {


    let shell_od = Length::new::<inch>(1.0);
    let pipe_thickness = Length::new::<millimeter>(3.0);
    let shell_id = shell_od - 2.0*pipe_thickness;

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = shell_id;
    let pipe_length = Length::new::<meter>(0.0897);
    let flow_area = PI/4.0 * shell_id * shell_id;
    let incline_angle = Angle::new::<degree>(130.0);

    // form losses TBD, need to calibrate
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::PyrogelHPS;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // for first iteration, just get it adiabatic first 
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(0.0);
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


/// hot bend 1 bottom left in iteration 1 (see fig A.16)
pub fn new_uw_flibe_pipe_13(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {


    let shell_od = Length::new::<inch>(1.0);
    let pipe_thickness = Length::new::<millimeter>(3.0);
    let shell_id = shell_od - 2.0*pipe_thickness;

    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = shell_id;
    let pipe_length = Length::new::<meter>(0.0897);
    let flow_area = PI/4.0 * shell_id * shell_id;
    let incline_angle = Angle::new::<degree>(158.0);

    // form losses TBD, need to calibrate
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::PyrogelHPS;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // for first iteration, just get it adiabatic first 
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(0.0);
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
