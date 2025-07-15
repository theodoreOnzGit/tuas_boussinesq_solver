
use std::f64::consts::PI;

use uom::si::angle::degree;
use uom::si::f64::*;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::length::{centimeter, foot, inch, meter, millimeter};
use uom::si::mass_rate::kilogram_per_second;
use uom::si::pressure::{atmosphere, pascal};
use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::degree_celsius;

use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
use crate::pre_built_components::insulated_pipes_and_fluid_components::InsulatedFluidComponent;


/// This second test documents how to use TUAS 
/// to solve for fluid flow in a single pipe
///
/// getting mass flowrate using pressure drop
/// 
#[test]
pub fn fluid_mechanics_get_mass_flowrate_from_pressure_drop(){

    // now, we know how to make piping components, and 
    // obtain pressure drop given mass flowrate, we shall 
    // move on to obtaining pressure drop given mass flowrate 
    //
    // now, this is usually done via iteration. At this beginner 
    // level, you won't be dealing with the iteration algorithms 
    // directly, but rather you'll just use the API to 
    // obtain mass flowrates from pressure drop
    //
    // To start, we shall 
    // repeat the procedure in the first one

    let initial_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(50.0);

    let ambient_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(25.0);

    let fluid_pressure = 
        Pressure::new::<atmosphere>(1.0);
    
    let solid_pressure = 
        Pressure::new::<atmosphere>(1.0);

    let incline_angle = 
        Angle::new::<degree>(0.0);


    let shell_id = Length::new::<meter>(0.5);
    let hydraulic_diameter = shell_id;
    let flow_area = PI * 0.25 * hydraulic_diameter * hydraulic_diameter;
    let shell_od = shell_id + Length::new::<centimeter>(2.0);
    let insulation_thickness = Length::new::<inch>(1.0);
    let pipe_length = Length::new::<foot>(30.0);
    

    let form_loss = Ratio::new::<ratio>(5.0);
    let surface_roughness = Length::new::<millimeter>(1.0);


    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);


    let user_specified_inner_nodes = 5;


    let pipe_1 = 
        InsulatedFluidComponent::new_insulated_pipe(
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

    // now, in tutorial 1, we used a mass flowrate of 100 kg/s 
    // and obtained a pressure drop 

    let test_mass_flowrate_from_tutorial_1 = 
        MassRate::new::<kilogram_per_second>(100.0);

    let test_pressure_drop_from_tutorial_1 = 
        pipe_1.get_pressure_loss_immutable(
            test_mass_flowrate_from_tutorial_1);

    // the result is that we had a pressure drop of about 
    // 683.38 Pa.
    
    approx::assert_relative_eq!(
        test_pressure_drop_from_tutorial_1.get::<pascal>(),
        683.38,
        max_relative=1e-5
        );

    // So, if we supply a pressure drop of 683.38 Pa,
    // we should get back the 100 kg/s flowrate 
    let pressure_loss = Pressure::new::<pascal>(683.38);

    let test_mass_flowrate_for_tutorial_2 = 
        pipe_1.get_mass_flowrate_from_pressure_loss_immutable(
            pressure_loss);

    // so we should get back 100 kg/s
    approx::assert_relative_eq!(
        test_mass_flowrate_for_tutorial_2.get::<kilogram_per_second>(),
        100.0,
        max_relative=1e-5
        );

    // now, flows also work in reverse, so if we supply a pressure 
    // drop of -683.38 Pa, we get a mass flowrate of 
    // -100 kg/s

    let pressure_loss_reverse = Pressure::new::<pascal>(-683.38);

    let test_mass_flowrate_for_tutorial_2_reverse = 
        pipe_1.get_mass_flowrate_from_pressure_loss_immutable(
            pressure_loss_reverse);
    //
    // so in this case 
    // we should get back 100 kg/s
    // but in reverse flow, so -100 kg/s
    approx::assert_relative_eq!(
        test_mass_flowrate_for_tutorial_2_reverse.get::<kilogram_per_second>(),
        -100.0,
        max_relative=1e-5
        );
}
