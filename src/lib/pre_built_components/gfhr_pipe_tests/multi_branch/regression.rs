use uom::si::{mass_rate::kilogram_per_second, pressure::kilopascal};
use uom::si::thermodynamic_temperature::degree_celsius;

use crate::pre_built_components::gfhr_pipe_tests::components::new_fhr_pri_loop_pump_9;
use crate::pre_built_components::gfhr_pipe_tests::multi_branch::fluid_mechanics_solvers::four_branch_pri_loop_flowrates_parallel_debug_library;
use uom::si::f64::*;
/// v0.0.9 
///
/// this test checks if the internal library is functioning correctly 
/// so as to give correct mass flowrates
#[test]
pub fn test_fhr_four_branch_solver_regression_library(){

    let initial_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(500.0);
    let reactor_pipe_1 = new_reactor_vessel_pipe_1_regression(initial_temperature);
    let downcomer_pipe_2 = new_downcomer_pipe_2_regression(initial_temperature);
    let downcomer_pipe_3 = new_downcomer_pipe_3_regression(initial_temperature);
    let fhr_pipe_7 = new_fhr_pipe_7_regression(initial_temperature);
    let fhr_pri_loop_pump = new_fhr_pri_loop_pump_9(initial_temperature);


    let pump_pressure = Pressure::new::<kilopascal>(15.0);

    let (reactor_flow, downcomer_branch_1_flow, 
        downcomer_branch_2_flow, intermediate_heat_exchanger_branch_flow)
        = four_branch_pri_loop_flowrates_parallel_debug_library(
            pump_pressure, 
            &reactor_pipe_1, 
            &downcomer_pipe_2, 
            &downcomer_pipe_3, 
            &fhr_pipe_7, 
            &fhr_pri_loop_pump);

    dbg!(&(reactor_flow, downcomer_branch_1_flow,
            downcomer_branch_2_flow, intermediate_heat_exchanger_branch_flow
    ));

    approx::assert_relative_eq!(
        reactor_flow.get::<kilogram_per_second>(),
        -147.871,
        max_relative=1e-5
        );
    approx::assert_relative_eq!(
        downcomer_branch_1_flow.get::<kilogram_per_second>(),
        -1.04956,
        max_relative=1e-5
        );
    approx::assert_relative_eq!(
        downcomer_branch_2_flow.get::<kilogram_per_second>(),
        -1.04956,
        max_relative=1e-5
        );
    approx::assert_relative_eq!(
        intermediate_heat_exchanger_branch_flow.get::<kilogram_per_second>(),
        149.9704,
        max_relative=1e-5
        );
}



use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
use crate::pre_built_components::insulated_pipes_and_fluid_components::InsulatedFluidComponent;
use uom::si::angle::degree;
use uom::si::area::{square_centimeter, square_meter};
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::length::{centimeter, meter, millimeter};
use uom::si::ratio::ratio;
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
/// forward flow direction going upwards 
///
/// the dimensions are wrong, but this was used for regression testing
pub(crate) fn new_reactor_vessel_pipe_1_regression(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
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
/// forward flow direction going upwards 
pub fn new_downcomer_pipe_2_regression(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
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
///
/// forward flow direction going upwards 
pub fn new_downcomer_pipe_3_regression(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
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
///
/// note: i found that this had a flow area of 20 square meters... way too 
/// big
///
/// Nevertheless, this component was useful for debugging
pub fn new_fhr_pipe_7_regression(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
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
