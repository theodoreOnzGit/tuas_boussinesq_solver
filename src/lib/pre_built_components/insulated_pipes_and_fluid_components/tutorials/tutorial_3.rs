
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


/// This third set of tests documents how to use TUAS 
/// to solve for fluid flow in a single pipe 
/// when considering hydrostatic pressure in addition to normal 
/// pressure drops
/// 
#[test]
pub fn fluid_mechanics_get_mass_flowrate_from_pressure_chg(){

    // now, we know how to obtain pressure loss from mass flowrates, 
    // and vice versa, we can now move onto tackling elevation changes 
    //
    // for mechanical energy considerations in bernoulli's eqn, 
    // we consider 
    // 1. Gravitational potential energy (hydrostatic pressure)
    // 2. Pressure energy or some sort of elastic potential energy
    // 3. Kinetic energy (dynamic pressure) 
    //
    // when fluid moves from point A to point B, some of this 
    // mechanical energy is lost to friction
    // however, sometimes the kinetic energy of the fluid is transformed 
    // into gravitational potential energy as well. 
    //
    // the sensible pressure at any point is due to hydraulic head
    // or piezometric head. That is sum of the hydrostatic pressure 
    // and pressure head (terms 1 and 2)
    // 
    // To account for this change in 
    // piezometric head, TUAS uses the term 
    // "pressure change". 
    //
    // The hydrostatic pressure changes and pressure losses constitute 
    // this change in piezometric head, or pressure change for short. 
    //
    // for a pipe with no elevation or incline angle, the pressure change 
    // and pressure loss are identical

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



    let test_mass_flowrate_100_kg_per_s = 
        MassRate::new::<kilogram_per_second>(100.0);

    let test_pressure_drop_from_tutorial_1 = 
        pipe_1.get_pressure_loss_immutable(
            test_mass_flowrate_100_kg_per_s);

    let test_pressure_change = 
        pipe_1.get_pressure_change_immutable(
            test_mass_flowrate_100_kg_per_s);

    // for a flat pipe, the pressure change and pressure drop are 
    // identical
    
    // however, pressure drop is by virtue a negative value 
    // so we have to reverse the sign to get pressure change
    //
    //
    // just like if we consider mass loss from a system,
    // a positive mass loss means a negative mass change 
    //
    approx::assert_relative_eq!(
        -test_pressure_drop_from_tutorial_1.get::<pascal>(),
        test_pressure_change.get::<pascal>(),
        max_relative=1e-5
        );

    // let's try angling the incline up 70 degrees, and call this 
    // pipe 2 

    let pipe_2_incline_angle = 
        Angle::new::<degree>(70.0);


    let pipe_2 = 
        InsulatedFluidComponent::new_insulated_pipe(
            initial_temperature, 
            ambient_temperature, 
            fluid_pressure, 
            solid_pressure, 
            flow_area, 
            pipe_2_incline_angle, 
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

    // if we get a pressure drop... 
    // the results are the same as pipe 1, 683.38 Pa
    //

    let test_pressure_drop_for_pipe_2 = 
        pipe_2.get_pressure_loss_immutable(
            test_mass_flowrate_100_kg_per_s);

    approx::assert_relative_eq!(
        test_pressure_drop_from_tutorial_1.get::<pascal>(),
        test_pressure_drop_for_pipe_2.get::<pascal>(),
        max_relative=1e-5
        );


    // now the pressure change is the sum of hydrostatic pressure 
    // change plus the contributions of pressure drop
    // we can obtain hydrostatic pressure change from the following:
    //
    // this function is called getting pressure change at some 
    // reference temperature because the hydrostatic pressure 
    // is sensitive to temperature changes.
    //
    // The reference temperature in this case determines rho in:
    // P_hydrostatic = h * rho * g 
    //
    // the reference temperature here is the bulk temperature of the fluid 

    let hydrstatic_pressure_pipe_2 = 
        pipe_2.
        get_hydrostatic_pressure_change_immutable_at_ref_temperature();

    // for this pipe, as it is going up, the hydrostatic change 
    // is negative 
    //
    // again 
    // Delta P_hydrostatic = Delta z * rho * g 
    //
    // Delta z = z_high - z_low
    //
    // Where z_high is the degree of descent for the high point 
    // and z_low is the degree of descent for the low point
    //
    // by TUAS convention a higher height has a lower z value
    // z is the degree of descent from some reference height
    // so in this case z_low > z_high 

    approx::assert_relative_eq!(
        hydrstatic_pressure_pipe_2.get::<pascal>(),
        -87285.31,
        max_relative=1e-5
        );

    // now based on this, a 100 kg/s flow produces a 683.38 Pa pressure drop 
    // and the hydrostatic pressure change is -87285.31
    //
    // the total pressure change is the sum of these two 

    let total_pressure_chg_reference = 
        hydrstatic_pressure_pipe_2 + (-test_pressure_drop_for_pipe_2);

    let total_pressure_chg_test = 
        pipe_2.get_pressure_change_immutable(
            test_mass_flowrate_100_kg_per_s);

    // we can do a quick check if these two are equal
    //
    // the test will pass if these two are equal

    approx::assert_relative_eq!(
        total_pressure_chg_reference.get::<pascal>(),
        total_pressure_chg_test.get::<pascal>(),
        max_relative=1e-5
        );


    // congratulations, you've finished tutorial 3



}

