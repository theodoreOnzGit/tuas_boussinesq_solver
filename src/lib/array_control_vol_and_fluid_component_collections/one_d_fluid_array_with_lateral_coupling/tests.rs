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
    use crate::prelude::beta_testing::{FluidArray, HeatTransferEntity, HeatTransferInteractionType};
    use crate::single_control_vol::SingleCVNode;


    let hot_temp = ThermodynamicTemperature::new::<degree_celsius>(100.0);
    let cold_temp = ThermodynamicTemperature::new::<degree_celsius>(50.0);

    let mixing_node_diameter = Length::new::<centimeter>(50.84);
    let mixing_node_material = LiquidMaterial::TherminolVP1;
    let mixing_node_pressure = Pressure::new::<atmosphere>(1.0);
    let mixing_node = SingleCVNode::new_sphere(
        mixing_node_diameter, 
        mixing_node_material.into(), 
        cold_temp, 
        mixing_node_pressure)
        .unwrap();

    let mut mixing_joint_cv: HeatTransferEntity = mixing_node.into();

    // three pipes 
    let user_specified_inner_nodes = 0;
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



}
