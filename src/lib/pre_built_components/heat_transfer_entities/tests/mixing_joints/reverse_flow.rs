

/// this test checks if FluidArrays can form adiabatic mixing joints 
/// with single cvs 
///
/// so let's say, two pipes with 0.05 kg/s of therminol vp1 
/// flowing into a mixing joint (singleCV)
///
/// one is 50C, one is 100C
///
/// and 0.10 kg/s flows out. it should be 75 C is adiabatically mixed
#[cfg(test)]
#[test]
pub fn adiabatic_mixing_joint_test_link_to_front_and_back_reverse_flow(){
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
    
    let mut inlet_bc_hot = HeatTransferEntity::new_const_temperature_bc(
        hot_temp);
    let mut inlet_bc_cold = HeatTransferEntity::new_const_temperature_bc(
        cold_temp);
    let mut outlet_bc_adiabatic = HeatTransferEntity::new_adiabatic_bc();


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
    

    let mut inlet_pipe_1: HeatTransferEntity = 
        FluidArray::new_cylinder(
            length, 
            hydraulic_diameter, 
            initial_temperature, 
            initial_pressure, 
            adjacent_solid_material, 
            liquid_material, 
            pipe_form_loss, 
            user_specified_inner_nodes, 
            pipe_incline_angle).into();

    let mut inlet_pipe_2: HeatTransferEntity = 
        FluidArray::new_cylinder(
            length, 
            hydraulic_diameter, 
            initial_temperature, 
            initial_pressure, 
            adjacent_solid_material, 
            liquid_material, 
            pipe_form_loss, 
            user_specified_inner_nodes, 
            pipe_incline_angle).into();


    let mut outlet_pipe: HeatTransferEntity = 
        FluidArray::new_cylinder(
            length, 
            hydraulic_diameter, 
            initial_temperature, 
            initial_pressure, 
            adjacent_solid_material, 
            liquid_material, 
            pipe_form_loss, 
            user_specified_inner_nodes, 
            pipe_incline_angle).into();


    let advection_heat_transfer_interaction_pre_joint: 
        HeatTransferInteractionType;

    let mass_flowrate_inlets = 
        MassRate::new::<kilogram_per_second>(-0.05);
    let mass_flowrate_outlet = 
        MassRate::new::<kilogram_per_second>(-0.1);

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



    while simulation_time < max_time {
        // this linking with heat transfer entities is buggy 
        // probably want to check code

        // link inlet pipes 
        inlet_pipe_1.link_to_front(
            &mut inlet_bc_hot, 
            advection_heat_transfer_interaction_pre_joint)
            .unwrap();

        inlet_pipe_2.link_to_front(
            &mut inlet_bc_cold, 
            advection_heat_transfer_interaction_pre_joint)
            .unwrap();

        inlet_pipe_1.link_to_back(
            &mut mixing_joint_cv, 
            advection_heat_transfer_interaction_pre_joint)
            .unwrap();

        inlet_pipe_2.link_to_back(
            &mut mixing_joint_cv, 
            advection_heat_transfer_interaction_pre_joint)
            .unwrap();

        // you MUST set mass flowrate before advancing timestep

        inlet_pipe_1.try_set_flowrate_for_fluid_array(
            mass_flowrate_inlets)
            .unwrap();
        inlet_pipe_2.try_set_flowrate_for_fluid_array(
            mass_flowrate_inlets)
            .unwrap();

        // link to outlet 

        mixing_joint_cv.link_to_back(
            &mut outlet_pipe, 
            advection_heat_transfer_interaction_post_joint)
            .unwrap();

        // you MUST set mass flowrate before advancing timestep
        outlet_pipe.try_set_flowrate_for_fluid_array(
            mass_flowrate_outlet)
            .unwrap();

        outlet_pipe.link_to_back(
            &mut outlet_bc_adiabatic, 
            advection_heat_transfer_interaction_post_joint)
            .unwrap();

        // before advancing timestep, te inlet pipes need to have flowrates set 

        

        // advance timestep 
        inlet_pipe_1.advance_timestep_mut_self(timestep).unwrap();
        inlet_pipe_2.advance_timestep_mut_self(timestep).unwrap();
        outlet_pipe.advance_timestep_mut_self(timestep).unwrap();
        mixing_joint_cv.advance_timestep_mut_self(timestep).unwrap();



        simulation_time += timestep;
    }




    let inlet_pipe_1_temp = 
        inlet_pipe_1.try_get_bulk_temperature()
        .unwrap();
    let inlet_pipe_2_temp = 
        inlet_pipe_2.try_get_bulk_temperature()
        .unwrap();
    let mixing_joint_temp = 
        mixing_joint_cv.try_get_bulk_temperature().unwrap();

    let outlet_pipe_temp = 
        outlet_pipe.try_get_bulk_temperature().unwrap();

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
        mixing_joint_temp.get::<degree_celsius>(),
        75.0,
        epsilon=0.5);
    approx::assert_abs_diff_eq!(
        outlet_pipe_temp.get::<degree_celsius>(),
        75.0,
        epsilon=0.5);


}


/// when two streams of equal mass flowrate meet, 
/// one of 50C and the other of 100C, 
/// the outlet temperature should be 75C 
///
/// This assumes that cp is constant with temperature.
/// Which it is not. 
///
/// It is approximate that:
///
/// Delta H = cp Delta T 
///
/// In reality it is an integral, so the it is only about 75C 
/// in regression, it is 75.51 C
#[cfg(test)]
#[test]
pub fn adiabatic_mixing_joint_test_hte_single_cv_only(){
    use uom::si::length::centimeter;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::pressure::atmosphere;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::si::f64::*;
    use uom::si::time::second;
    use uom::ConstZero;

    use crate::boussinesq_thermophysical_properties::LiquidMaterial;
    use crate::prelude::beta_testing::{link_heat_transfer_entity, HeatTransferEntity, HeatTransferInteractionType};
    use crate::single_control_vol::SingleCVNode;


    let hot_temp = ThermodynamicTemperature::new::<degree_celsius>(100.0);
    let cold_temp = ThermodynamicTemperature::new::<degree_celsius>(50.0);
    
    let mut inlet_bc_hot = HeatTransferEntity::new_const_temperature_bc(
        hot_temp);
    let mut inlet_bc_cold = HeatTransferEntity::new_const_temperature_bc(
        cold_temp);
    let mut outlet_bc_adiabatic = HeatTransferEntity::new_adiabatic_bc();


    let mixing_node_diameter = Length::new::<centimeter>(2.84);
    let mixing_node_material = LiquidMaterial::TherminolVP1;
    let mixing_node_pressure = Pressure::new::<atmosphere>(1.0);
    let mixing_node = SingleCVNode::new_sphere(
        mixing_node_diameter, 
        mixing_node_material.into(), 
        cold_temp, 
        mixing_node_pressure)
        .unwrap();

    let mut mixing_joint_cv: HeatTransferEntity = mixing_node.clone().into();

    // three pipes 
    

    let mut inlet_pipe_1: HeatTransferEntity = 
        mixing_node.clone().into();

    let mut inlet_pipe_2: HeatTransferEntity = 
        mixing_node.clone().into();


    let mut outlet_pipe: HeatTransferEntity = 
        mixing_node.clone().into();


    let advection_heat_transfer_interaction_pre_joint_cold: 
        HeatTransferInteractionType;

    let mass_flowrate_inlets = 
        MassRate::new::<kilogram_per_second>(-0.05);
    let mass_flowrate_outlet = 
        MassRate::new::<kilogram_per_second>(-0.1);

    let average_therminol_density_cold = 
        LiquidMaterial::TherminolVP1.try_get_density(
            cold_temp).unwrap();
    let average_therminol_density_hot = 
        LiquidMaterial::TherminolVP1.try_get_density(
            hot_temp).unwrap();
    let average_therminol_density_mid = 
        LiquidMaterial::TherminolVP1.try_get_density(
            ThermodynamicTemperature::new::<degree_celsius>(75.0)
            ).unwrap();

    advection_heat_transfer_interaction_pre_joint_cold = 
        HeatTransferInteractionType::
        new_advection_interaction(mass_flowrate_inlets, 
            average_therminol_density_cold, 
            average_therminol_density_cold);

    let advection_heat_transfer_interaction_pre_joint_hot = 
        HeatTransferInteractionType::
        new_advection_interaction(mass_flowrate_inlets, 
            average_therminol_density_hot, 
            average_therminol_density_hot);

    let advection_heat_transfer_interaction_post_joint: 
        HeatTransferInteractionType;

    advection_heat_transfer_interaction_post_joint = 
        HeatTransferInteractionType::
        new_advection_interaction(mass_flowrate_outlet, 
            average_therminol_density_mid, 
            average_therminol_density_mid);

    let timestep = Time::new::<second>(0.1);
    let max_time = Time::new::<second>(500.0);
    let mut simulation_time = Time::ZERO;



    while simulation_time < max_time {
        // this linking with heat transfer entities is buggy 
        // probably want to check code

        // link inlet pipes 
        link_heat_transfer_entity(
            &mut inlet_pipe_1, 
            &mut inlet_bc_hot, 
            advection_heat_transfer_interaction_pre_joint_hot)
            .unwrap();

        link_heat_transfer_entity(
            &mut inlet_pipe_2,
            &mut inlet_bc_cold, 
            advection_heat_transfer_interaction_pre_joint_cold)
            .unwrap();

        inlet_pipe_1.link_to_back(
            &mut mixing_joint_cv, 
            advection_heat_transfer_interaction_pre_joint_hot)
            .unwrap();

        inlet_pipe_2.link_to_back(
            &mut mixing_joint_cv, 
            advection_heat_transfer_interaction_pre_joint_cold)
            .unwrap();

        // link to outlet 

        mixing_joint_cv.link_to_back(
            &mut outlet_pipe, 
            advection_heat_transfer_interaction_post_joint)
            .unwrap();
        outlet_pipe.link_to_back(
            &mut outlet_bc_adiabatic, 
            advection_heat_transfer_interaction_post_joint)
            .unwrap();

        // advance timestep 
        inlet_pipe_1.advance_timestep_mut_self(timestep).unwrap();
        inlet_pipe_2.advance_timestep_mut_self(timestep).unwrap();
        outlet_pipe.advance_timestep_mut_self(timestep).unwrap();
        mixing_joint_cv.advance_timestep_mut_self(timestep).unwrap();


        let inlet_pipe_1_temp = 
            inlet_pipe_1.try_get_bulk_temperature()
            .unwrap();
        let inlet_pipe_2_temp = 
            inlet_pipe_2.try_get_bulk_temperature()
            .unwrap();
        let mixing_joint_temp = 
            mixing_joint_cv.try_get_bulk_temperature().unwrap();

        let outlet_pipe_temp = 
            outlet_pipe.try_get_bulk_temperature().unwrap();

        dbg!(&(
                simulation_time.get::<second>(),
                inlet_pipe_1_temp.get::<degree_celsius>(),
                inlet_pipe_2_temp.get::<degree_celsius>(),
                mixing_joint_temp.get::<degree_celsius>(),
                outlet_pipe_temp.get::<degree_celsius>(),
                ));

        simulation_time += timestep;
    }

    let inlet_pipe_1_temp = 
        inlet_pipe_1.try_get_bulk_temperature()
        .unwrap();

    approx::assert_abs_diff_eq!(
        inlet_pipe_1_temp.get::<degree_celsius>(),
        100.0,
        epsilon=0.5);

    let inlet_pipe_2_temp = 
        inlet_pipe_2.try_get_bulk_temperature()
        .unwrap();

    approx::assert_abs_diff_eq!(
        inlet_pipe_2_temp.get::<degree_celsius>(),
        50.0,
        epsilon=0.5);


    let mixing_joint_temp = 
        mixing_joint_cv.try_get_bulk_temperature().unwrap();

    let mixing_joint_temp_degc = 
        mixing_joint_temp.get::<degree_celsius>();

    approx::assert_abs_diff_eq!(
        mixing_joint_temp_degc,
        75.0,
        epsilon=0.6);

    let outlet_temp = 
        outlet_pipe.try_get_bulk_temperature().unwrap();

    let outlet_temp_degc = 
        outlet_temp.get::<degree_celsius>();

    approx::assert_abs_diff_eq!(
        outlet_temp_degc,
        75.0,
        epsilon=0.6);

}
