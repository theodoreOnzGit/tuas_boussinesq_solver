use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
use crate::pre_built_components::insulated_pipes_and_fluid_components::InsulatedFluidComponent;
use crate::pre_built_components::non_insulated_fluid_components::NonInsulatedFluidComponent;
use uom::si::angle::degree;
use uom::si::area::{square_centimeter, square_meter};
use uom::si::f64::*;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::length::{centimeter, meter, millimeter};
use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::pressure::atmosphere;


/// creates a new pipe 1 for the fhr simulator, this goes from bottom 
/// to top of the pebble bed
///
/// it has 5 nodes, and the middle node is used to cool the reactor
///
/// it is then joined to two mixing nodes at the top and bottom of the 
/// reactor
///
/// we make it roughly 
/// 310 cm in height 
/// 120 cm in radius
///
/// core barrel thickness is 2 cm 
/// vessel thickness is 4 cm
/// downcomer width is 5cm
///
/// expected mass flowrate of FLiBe is about 1173 kg/s for a 280 MWth reactor
/// 
/// https://kairospower.com/generic-fhr-core-model/
///
/// we can scale it down
pub fn new_reactor_vessel_pipe_1(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    // hydraulic diameter = 4 * void vol/surface area of pebbles
    // = 4 * void frac/(1-void frac)
    // = 2.67 cm
    let hydraulic_diameter = Length::new::<centimeter>(2.67);
    let pipe_length = Length::new::<meter>(3.10);
    // area of a 120 cm radius circle is about 11310 cm^2 
    // assume 60% filled by pebbles 
    // we get about 4523 cm2
    let flow_area = Area::new::<square_centimeter>(4500.0);
    let incline_angle = Angle::new::<degree>(90.0);
    // not putting in ergun equation yet
    let form_loss = Ratio::new::<ratio>(550.05);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    // we get reactor vessel around 120 cm in diameter
    let shell_id = Length::new::<centimeter>(120.0*2.0);
    // the pipe at this point just functions as thermal inertia 
    // it isn't meant to conduct heat to graphite and so on, even though it 
    // can. 
    // It is a quick an dirty way to gprogram this
    let pipe_thickness = Length::new::<centimeter>(5.0);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // insulate the pipe totally from environment
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(0.0);
    // we want 5 total nodes,
    // so two outer nodes on each end, plus 3 inner nodes
    let user_specified_inner_nodes = 3; 

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
/// creates a new pipe 2 for the fhr simulator, this goes from bottom 
/// to parallel to pebble bed
///
/// it is then joined to two mixing nodes at the top and bottom of the 
/// reactor
///
/// we make it roughly 
/// 310 cm in height 
/// 5 cm in radius
///
/// this is based on 
///
/// core barrel thickness of 2 cm 
/// vessel thickness of 4 cm
/// downcomer width of 5cm
///
/// expected mass flowrate of FLiBe is about 1173 kg/s for a 280 MWth reactor
/// 
/// https://kairospower.com/generic-fhr-core-model/
///
/// we can scale it down
pub fn new_downcomer_pipe_2(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<centimeter>(5.0);
    let pipe_length = Length::new::<meter>(3.10);
    let flow_area = Area::new::<square_centimeter>(100.0);
    let incline_angle = Angle::new::<degree>(90.0);
    // not putting in ergun equation yet
    let form_loss = Ratio::new::<ratio>(5505.05);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    // the pipe at this point just functions as thermal inertia 
    // it isn't meant to conduct heat to graphite and so on, even though it 
    // can. 
    // It is a quick an dirty way to program this
    let pipe_thickness = Length::new::<centimeter>(4.0);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::FLiBe;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // we want 5 total nodes,
    // so two outer nodes on each end, plus 3 inner nodes
    let user_specified_inner_nodes = 3; 

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


/// creates a new pipe 3 for the fhr simulator, this goes from bottom 
/// to top of the pebble bed
///
/// pretty much identical to pipe 2
/// except it cools more efficiently to the environment 
/// just for effect
///
/// it is then joined to two mixing nodes at the top and bottom of the 
/// reactor
///
/// we make it roughly 
/// 310 cm in height 
/// 5 cm in radius
///
/// this is based on 
///
/// core barrel thickness of 2 cm 
/// vessel thickness of 4 cm
/// downcomer width of 5cm
///
/// expected mass flowrate of FLiBe is about 1173 kg/s for a 280 MWth reactor
/// 
/// https://kairospower.com/generic-fhr-core-model/
///
/// we can scale it down
pub fn new_downcomer_pipe_3(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<centimeter>(5.0);
    let pipe_length = Length::new::<meter>(3.10);
    let flow_area = Area::new::<square_centimeter>(100.0);
    let incline_angle = Angle::new::<degree>(90.0);
    // not putting in ergun equation yet
    let form_loss = Ratio::new::<ratio>(5505.05);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    // the pipe at this point just functions as thermal inertia 
    // it isn't meant to conduct heat to graphite and so on, even though it 
    // can. 
    // It is a quick an dirty way to gprogram this
    let pipe_thickness = Length::new::<centimeter>(4.0);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // I just made this side more conductive to environment
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(200.0);
    // we want 5 total nodes,
    // so two outer nodes on each end, plus 3 inner nodes
    let user_specified_inner_nodes = 3; 

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
/// creates a new pipe 4 for the fhr simulator, this goes from bottom 
/// to top of the pebble bed
///
/// this is supposed to be part of the forced cooling primary loop
///
/// it is then joined to two mixing nodes at the top and bottom of the 
/// reactor
///
/// we make it roughly 
/// 310 cm in height 
/// 5 cm in radius
///
/// this is based on 
///
/// core barrel thickness of 2 cm 
/// vessel thickness of 4 cm
/// downcomer width of 5cm
///
/// expected mass flowrate of FLiBe is about 1173 kg/s for a 280 MWth reactor
/// 
/// https://kairospower.com/generic-fhr-core-model/
///
/// we can scale it down
pub fn new_fhr_pipe_4(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<centimeter>(5.0);
    let pipe_length = Length::new::<meter>(3.10);
    let flow_area = Area::new::<square_meter>(20.0);
    let incline_angle = Angle::new::<degree>(90.0);
    // not putting in ergun equation yet
    let form_loss = Ratio::new::<ratio>(5505.05);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    // the pipe at this point just functions as thermal inertia 
    // it isn't meant to conduct heat to graphite and so on, even though it 
    // can. 
    // It is a quick an dirty way to gprogram this
    let pipe_thickness = Length::new::<centimeter>(4.0);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // I just made this side more conductive to environment
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // we want 5 total nodes,
    // so two outer nodes on each end, plus 3 inner nodes
    let user_specified_inner_nodes = 3; 

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


/// creates a new pump component for the primary loop
pub fn new_fhr_pri_loop_pump(initial_temperature: ThermodynamicTemperature) -> NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(5.0);
    let component_length = Length::new::<meter>(0.36);
    let flow_area = Area::new::<square_centimeter>(20.0);
    let incline_angle = Angle::new::<degree>(0.0);
    let form_loss = Ratio::new::<ratio>(0.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::FLiBe;
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

