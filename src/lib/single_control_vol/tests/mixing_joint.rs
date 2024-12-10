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
pub fn adiabatic_mixing_joint_test_single_cv_only(){
    use uom::si::length::centimeter;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::pressure::atmosphere;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::si::f64::*;
    use uom::si::time::second;
    use uom::ConstZero;

    use crate::boussinesq_thermophysical_properties::LiquidMaterial;
    use crate::heat_transfer_correlations::heat_transfer_interactions::
        heat_transfer_interaction_enums::*;
    use crate::single_control_vol::SingleCVNode;


    let hot_temp = ThermodynamicTemperature::new::<degree_celsius>(100.0);
    let cold_temp = ThermodynamicTemperature::new::<degree_celsius>(50.0);
    


    let mixing_node_diameter = Length::new::<centimeter>(2.84);
    let mixing_node_material = LiquidMaterial::TherminolVP1;
    let mixing_node_pressure = Pressure::new::<atmosphere>(1.0);
    let mut mixing_joint_cv = SingleCVNode::new_sphere(
        mixing_node_diameter, 
        mixing_node_material.into(), 
        cold_temp, 
        mixing_node_pressure)
        .unwrap();

    // three "pipes"
    

    let mut inlet_pipe_1 = mixing_joint_cv.clone();

    let mut inlet_pipe_2 = mixing_joint_cv.clone();

    let mut outlet_pipe = mixing_joint_cv.clone();


    let advection_heat_transfer_interaction_pre_joint_cold_data: 
        DataAdvection;

    let mass_flowrate_inlets = 
        MassRate::new::<kilogram_per_second>(0.05);
    let mass_flowrate_outlet = 
        MassRate::new::<kilogram_per_second>(0.1);

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

    advection_heat_transfer_interaction_pre_joint_cold_data = 
        HeatTransferInteractionType::
        new_advection_interaction(mass_flowrate_inlets, 
            average_therminol_density_cold, 
            average_therminol_density_cold).try_into().unwrap();

    let advection_heat_transfer_interaction_pre_joint_hot_data:
        DataAdvection = 
        HeatTransferInteractionType::
        new_advection_interaction(mass_flowrate_inlets, 
            average_therminol_density_hot, 
            average_therminol_density_hot).try_into().unwrap();

    let advection_heat_transfer_interaction_post_joint_data: 
        DataAdvection;

    advection_heat_transfer_interaction_post_joint_data = 
        HeatTransferInteractionType::
        new_advection_interaction(mass_flowrate_outlet, 
            average_therminol_density_mid, 
            average_therminol_density_mid).try_into().unwrap();

    let timestep = Time::new::<second>(0.1);
    let max_time = Time::new::<second>(500.0);
    let mut simulation_time = Time::ZERO;



    while simulation_time < max_time {
        // this linking with heat transfer entities is buggy 
        // probably want to check code

        // link inlet pipes 
        SingleCVNode::calculate_single_cv_node_front_constant_temperature_back(
            cold_temp, 
            &mut inlet_pipe_1, 
            advection_heat_transfer_interaction_pre_joint_cold_data.into()
            ).unwrap();
        SingleCVNode::calculate_single_cv_node_front_constant_temperature_back(
            hot_temp, 
            &mut inlet_pipe_2, 
            advection_heat_transfer_interaction_pre_joint_hot_data.into()
            ).unwrap();

        inlet_pipe_1.calculate_advection_interaction_to_front_singular_cv_node(
            &mut mixing_joint_cv, 
            advection_heat_transfer_interaction_pre_joint_cold_data)
            .unwrap();


        inlet_pipe_2.calculate_advection_interaction_to_front_singular_cv_node(
            &mut mixing_joint_cv, 
            advection_heat_transfer_interaction_pre_joint_hot_data)
            .unwrap();



        // link to outlet 
        //

        mixing_joint_cv.calculate_advection_interaction_to_front_singular_cv_node(
            &mut outlet_pipe, 
            advection_heat_transfer_interaction_post_joint_data)
            .unwrap();

        outlet_pipe.calculate_bc_front_cv_back_advection_non_set_temperature(
            advection_heat_transfer_interaction_post_joint_data)
            .unwrap();


        // advance timestep 
        inlet_pipe_1.advance_timestep(timestep).unwrap();
        inlet_pipe_2.advance_timestep(timestep).unwrap();
        outlet_pipe.advance_timestep(timestep).unwrap();
        mixing_joint_cv.advance_timestep(timestep).unwrap();


        simulation_time += timestep;
    }

    let inlet_pipe_1_temp = inlet_pipe_1.temperature;
    let inlet_pipe_2_temp = inlet_pipe_2.temperature;
    let mixing_joint_temp = mixing_joint_cv.temperature;
    let mixing_joint_temp_degc = 
        mixing_joint_temp.get::<degree_celsius>();
    let outlet_pipe_temp = outlet_pipe.temperature;
    let outlet_temp_degc = 
        outlet_pipe_temp.get::<degree_celsius>();


    dbg!(&(
            simulation_time.get::<second>(),
            inlet_pipe_1_temp.get::<degree_celsius>(),
            inlet_pipe_2_temp.get::<degree_celsius>(),
            mixing_joint_temp.get::<degree_celsius>(),
            outlet_pipe_temp.get::<degree_celsius>(),
    ));


    approx::assert_abs_diff_eq!(
        inlet_pipe_1_temp.get::<degree_celsius>(),
        50.0,
        epsilon=0.5);


    approx::assert_abs_diff_eq!(
        inlet_pipe_2_temp.get::<degree_celsius>(),
        100.0,
        epsilon=0.5);



    approx::assert_abs_diff_eq!(
        mixing_joint_temp_degc,
        75.0,
        epsilon=0.6);


    approx::assert_abs_diff_eq!(
        outlet_temp_degc,
        75.0,
        epsilon=0.6);

}
