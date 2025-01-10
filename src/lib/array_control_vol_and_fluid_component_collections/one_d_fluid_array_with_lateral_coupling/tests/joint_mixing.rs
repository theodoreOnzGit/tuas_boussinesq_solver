
/// this test checks if FluidArrays can form adiabatic mixing joints 
/// with single cvs 
///
/// so let's say, two pipes with 0.05 kg/s of therminol vp1 
/// flowing into a mixing joint (singleCV)
///
/// one is 50C, one is 100C
///
/// and 0.10 kg/s flows out. it should be 75 C is adiabatically mixed
///
/// this is to ensure things are working correctly even before the 
/// complication of having heat transfer entities
///
#[cfg(test)]
#[test]
pub fn fluid_array_adiabatic_mixing_joint_test(){
    use uom::si::angle::radian;
    use uom::si::length::{centimeter, foot};
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::pressure::atmosphere;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::si::f64::*;
    use uom::si::time::second;
    use uom::ConstZero;

    use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
    use crate::prelude::beta_testing::{FluidArray, HeatTransferInteractionType};
    use crate::single_control_vol::SingleCVNode;


    let hot_temp = ThermodynamicTemperature::new::<degree_celsius>(100.0);
    let cold_temp = ThermodynamicTemperature::new::<degree_celsius>(50.0);

    let mixing_node_diameter = Length::new::<centimeter>(50.84);
    let mixing_node_material = LiquidMaterial::TherminolVP1;
    let mixing_node_pressure = Pressure::new::<atmosphere>(1.0);
    let mut mixing_node = SingleCVNode::new_sphere(
        mixing_node_diameter, 
        mixing_node_material.into(), 
        cold_temp, 
        mixing_node_pressure)
        .unwrap();


    // three pipes 
    let user_specified_inner_nodes = 2;
    let pipe_incline_angle = Angle::new::<radian>(0.0);
    let pipe_form_loss = Ratio::ZERO;
    let liquid_material = mixing_node_material;
    let adjacent_solid_material = SolidMaterial::SteelSS304L;
    let initial_pressure = mixing_node_pressure;
    let initial_temperature = cold_temp;
    let length = Length::new::<foot>(1.0);
    let hydraulic_diameter = Length::new::<centimeter>(2.79);

    let mut inlet_pipe_1 = FluidArray::new_cylinder(
            length, 
            hydraulic_diameter, 
            initial_temperature, 
            initial_pressure, 
            adjacent_solid_material, 
            liquid_material, 
            pipe_form_loss, 
            user_specified_inner_nodes, 
            pipe_incline_angle);

    let mut inlet_pipe_2 = FluidArray::new_cylinder(
            length, 
            hydraulic_diameter, 
            initial_temperature, 
            initial_pressure, 
            adjacent_solid_material, 
            liquid_material, 
            pipe_form_loss, 
            user_specified_inner_nodes, 
            pipe_incline_angle);


    let mut outlet_pipe = FluidArray::new_cylinder(
            length, 
            hydraulic_diameter, 
            initial_temperature, 
            initial_pressure, 
            adjacent_solid_material, 
            liquid_material, 
            pipe_form_loss, 
            user_specified_inner_nodes, 
            pipe_incline_angle);

    // advection heat transfer interaction
    let advection_heat_transfer_interaction_pre_joint: 
        HeatTransferInteractionType;

    let mass_flowrate_inlets = 
        MassRate::new::<kilogram_per_second>(0.05);
    let mass_flowrate_outlet = 
        MassRate::new::<kilogram_per_second>(0.1);

    let average_therminol_density = 
        LiquidMaterial::TherminolVP1.try_get_density(
            cold_temp).unwrap();

    advection_heat_transfer_interaction_pre_joint = 
        HeatTransferInteractionType::
        new_advection_interaction(mass_flowrate_inlets, 
            average_therminol_density, 
            average_therminol_density);

    let advection_heat_transfer_interaction_post_joint: 
        HeatTransferInteractionType;

    advection_heat_transfer_interaction_post_joint = 
        HeatTransferInteractionType::
        new_advection_interaction(mass_flowrate_outlet, 
            average_therminol_density, 
            average_therminol_density);

    let timestep = Time::new::<second>(0.5);
    let max_time = Time::new::<second>(3000.0);
    let mut simulation_time = Time::ZERO;
    // connect inlet pipe to bc 

    while simulation_time < max_time {
        inlet_pipe_1.link_constant_temperature_to_back_of_this_cv(
            hot_temp, 
            advection_heat_transfer_interaction_pre_joint)
            .unwrap();

        inlet_pipe_2.link_constant_temperature_to_back_of_this_cv(
            cold_temp, 
            advection_heat_transfer_interaction_pre_joint)
            .unwrap();

        inlet_pipe_1.link_single_cv_to_higher_side(
            &mut mixing_node, 
            advection_heat_transfer_interaction_pre_joint)
            .unwrap();

        inlet_pipe_2.link_single_cv_to_higher_side(
            &mut mixing_node, 
            advection_heat_transfer_interaction_pre_joint)
            .unwrap();

        outlet_pipe.link_single_cv_to_lower_side(
            &mut mixing_node, 
            advection_heat_transfer_interaction_post_joint)
            .unwrap();

        outlet_pipe.link_heat_addition_to_front_of_this_cv(
            Power::ZERO, 
            advection_heat_transfer_interaction_post_joint)
            .unwrap();

        inlet_pipe_1.advance_timestep_with_mass_flowrate(
            timestep,
            mass_flowrate_inlets
            ).unwrap();

        inlet_pipe_2.advance_timestep_with_mass_flowrate(
            timestep,
            mass_flowrate_inlets
            ).unwrap();

        mixing_node.advance_timestep(timestep)
            .unwrap();

        outlet_pipe.advance_timestep_with_mass_flowrate(
            timestep,
            mass_flowrate_outlet
            ).unwrap();

        simulation_time += timestep;


    }

    let inlet_pipe_1_temp = 
        inlet_pipe_1.back_single_cv.temperature;


    let inlet_pipe_2_temp = 
        inlet_pipe_2.back_single_cv.temperature;



    let mixing_joint_temp = 
        mixing_node.temperature;

    let mixing_joint_temp_degc = 
        mixing_joint_temp.get::<degree_celsius>();


    let outlet_pipe_temp = 
        outlet_pipe.front_single_cv.temperature;


    dbg!(&(
            simulation_time.get::<second>(),
            inlet_pipe_1_temp.get::<degree_celsius>(),
            inlet_pipe_2_temp.get::<degree_celsius>(),
            mixing_joint_temp.get::<degree_celsius>(),
            outlet_pipe_temp
    ));
    approx::assert_abs_diff_eq!(
        inlet_pipe_1_temp.get::<degree_celsius>(),
        100.0,
        epsilon=0.5);
    approx::assert_abs_diff_eq!(
        inlet_pipe_2_temp.get::<degree_celsius>(),
        50.0,
        epsilon=0.5);
    approx::assert_abs_diff_eq!(
        mixing_joint_temp_degc,
        75.0,
        epsilon=0.5);
    approx::assert_abs_diff_eq!(
        outlet_pipe_temp.get::<degree_celsius>(),
        75.0,
        epsilon=0.5);

}
