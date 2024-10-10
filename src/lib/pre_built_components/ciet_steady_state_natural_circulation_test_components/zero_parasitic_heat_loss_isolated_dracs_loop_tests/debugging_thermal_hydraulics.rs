use uom::si::f64::*;

use crate::pre_built_components::ciet_steady_state_natural_circulation_test_components::dracs_loop_components::*;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_collection::FluidComponentCollection;

/// builds the hot branch of the DRACS loop (somewhat like the hot leg,
/// but with some other stuff)
/// this is pipe 30a all the way to 34 
/// but I build the components from top down
pub fn dracs_hot_branch_builder(initial_temperature: ThermodynamicTemperature) -> 
FluidComponentCollection {
    let pipe_34 = new_pipe_34(initial_temperature);
    let pipe_33 = new_pipe_33(initial_temperature);
    let pipe_32 = new_pipe_32(initial_temperature);
    let pipe_31a = new_pipe_31a(initial_temperature);
    let static_mixer_61_label_31 = new_static_mixer_61_label_31(initial_temperature);
    let dhx_tube_side_30b = new_dhx_tube_side_30b(initial_temperature);
    let dhx_tube_side_heat_exchanger_30 = new_isolated_dhx_tube_side_30(initial_temperature);
    let dhx_tube_side_30a = new_dhx_tube_side_30a(initial_temperature);



    let mut dracs_hot_branch = 
        FluidComponentCollection::new_series_component_collection();

    dracs_hot_branch.clone_and_add_component(&pipe_34);
    dracs_hot_branch.clone_and_add_component(&pipe_33);
    dracs_hot_branch.clone_and_add_component(&pipe_32);
    dracs_hot_branch.clone_and_add_component(&pipe_31a);
    dracs_hot_branch.clone_and_add_component(&static_mixer_61_label_31);
    dracs_hot_branch.clone_and_add_component(&dhx_tube_side_30b);
    dracs_hot_branch.clone_and_add_component(&dhx_tube_side_heat_exchanger_30);
    dracs_hot_branch.clone_and_add_component(&dhx_tube_side_30a);

    dracs_hot_branch
}


/// builds the cold branch of the DRACS loop (somewhat like the cold leg,
/// but with some other stuff)

pub fn dracs_cold_branch_builder(initial_temperature: ThermodynamicTemperature) -> 
FluidComponentCollection {
    let tchx_35a = new_ndhx_tchx_horizontal_35a(initial_temperature);
    let tchx_35b = new_ndhx_tchx_vertical_35b(initial_temperature);
    let static_mixer_60_label_36 = new_static_mixer_60_label_36(initial_temperature);
    let pipe_36a = new_pipe_36a(initial_temperature);
    let pipe_37 = new_pipe_37(initial_temperature);
    let flowmeter_60_37a = new_flowmeter_60_37a(initial_temperature);
    let pipe_38 = new_pipe_38(initial_temperature);
    let pipe_39 = new_pipe_39(initial_temperature);



    let mut dracs_cold_branch = 
        FluidComponentCollection::new_series_component_collection();

    dracs_cold_branch.clone_and_add_component(&tchx_35a);
    dracs_cold_branch.clone_and_add_component(&tchx_35b);
    dracs_cold_branch.clone_and_add_component(&static_mixer_60_label_36);
    dracs_cold_branch.clone_and_add_component(&pipe_36a);
    dracs_cold_branch.clone_and_add_component(&pipe_37);
    dracs_cold_branch.clone_and_add_component(&flowmeter_60_37a);
    dracs_cold_branch.clone_and_add_component(&pipe_38);
    dracs_cold_branch.clone_and_add_component(&pipe_39);

    dracs_cold_branch

}


/// the most basic of tests is to check that at 0 kg/s and constant 
/// temperature, the branch pressure changes are the same 
///
/// The second is to increase the hot leg temperature uniformly,
/// then check if the pressure change is less (at 0 kg/s)
#[test] 
pub fn dracs_branch_pressure_change_test(){

    // let's construct the branches with test pressures and obtain 
    use crate::
        array_control_vol_and_fluid_component_collections::
        fluid_component_collection::
        fluid_component_collection::FluidComponentCollectionMethods;
    use uom::si::f64::*;
    use uom::ConstZero;
    use approx::assert_abs_diff_eq;
    use uom::si::pressure::pascal;

    use uom::si::thermodynamic_temperature::degree_celsius;

    let test_temperature = ThermodynamicTemperature::
        new::<degree_celsius>(21.7);
    let mut dracs_hot_branch = dracs_hot_branch_builder(test_temperature);

    // pressure change at 0 kg/s 
    let pressure_change_at_zero_kg_per_s_hot_branch = 
        dracs_hot_branch.get_pressure_change(MassRate::ZERO);

    // pressure change should be 53627 +/- 1 Pa
    assert_abs_diff_eq!(pressure_change_at_zero_kg_per_s_hot_branch.get::<pascal>(), 
        53627.0, epsilon=1.0,);

    let dracs_cold_branch = dracs_cold_branch_builder(test_temperature);

    let pressure_change_at_zero_kg_per_s_cold_branch = 
        dracs_cold_branch.get_pressure_change(MassRate::ZERO);

    // pressure change should be same for both branches at equal flow
    assert_abs_diff_eq!(pressure_change_at_zero_kg_per_s_cold_branch.get::<pascal>(), 
        pressure_change_at_zero_kg_per_s_hot_branch.get::<pascal>(), 
        epsilon=1.0,);

    // now suppose I set the hot branch to 80 degrees C, there should 
    // be some buoyancy force thus, pressure change is less than 53627 Pa

    dracs_hot_branch = dracs_hot_branch_builder(
        ThermodynamicTemperature::new::<degree_celsius>(80.0));

    // pressure change at 0 kg/s 
    let pressure_change_at_zero_kg_per_s_hot_branch = 
        dracs_hot_branch.get_pressure_change(MassRate::ZERO);

    // pressure change should be less than 53627 +/- 1 Pa
    // in this case 51119 Pa
    assert_abs_diff_eq!(pressure_change_at_zero_kg_per_s_hot_branch.get::<pascal>(), 
        51119.0, epsilon=1.0,);
}




/// This next set of tests shows explicitly what we need to do in 
/// the fluid component collection in order to get natural circulation
///
/// prototype test two, 
///
/// found that I can't use closures woops
/// I'll just use simple functions instead 
/// quite lehcheh (troublesome) to type, but ok lah
#[test]
pub fn dracs_natural_circ_thermal_hydraulics_regression_test_prototype_2(){

    // let's construct the branches with test pressures and obtain 
    use crate::
        array_control_vol_and_fluid_component_collections::
        fluid_component_collection::
        fluid_component_collection::FluidComponentCollectionMethods;
    use uom::si::f64::*;
    use uom::ConstZero;

    use uom::si::thermodynamic_temperature::degree_celsius;
    use crate::
        array_control_vol_and_fluid_component_collections::
        fluid_component_collection::
        fluid_component_super_collection::FluidComponentSuperCollection;

    use crate::pre_built_components::
        insulated_pipes_and_fluid_components::InsulatedFluidComponent;
    use crate::pre_built_components::
        non_insulated_fluid_components::NonInsulatedFluidComponent;

    use crate::boussinesq_thermophysical_properties::LiquidMaterial;
    use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::power::watt;
    use uom::si::time::second;
    // setup 
    let initial_temperature = ThermodynamicTemperature::
        new::<degree_celsius>(50.0);
    let timestep = Time::new::<second>(0.5);
    let heat_rate_through_dhx = Power::new::<watt>(460.0);
    let tchx_heat_transfer_coeff = 
        HeatTransfer::new
        ::<watt_per_square_meter_kelvin>(300.0);
    let average_temperature_for_density_calcs = 
        ThermodynamicTemperature::
        new::<degree_celsius>(80.0);


    // hot branch or (mostly) hot leg
    let mut pipe_34 = new_pipe_34(initial_temperature);
    let mut pipe_33 = new_pipe_33(initial_temperature);
    let mut pipe_32 = new_pipe_32(initial_temperature);
    let mut pipe_31a = new_pipe_31a(initial_temperature);
    let mut static_mixer_61_label_31 = new_static_mixer_61_label_31(initial_temperature);
    let mut dhx_tube_side_30b = new_dhx_tube_side_30b(initial_temperature);
    let mut dhx_tube_side_heat_exchanger_30 = new_isolated_dhx_tube_side_30(initial_temperature);
    let mut dhx_tube_side_30a = new_dhx_tube_side_30a(initial_temperature);

    // cold branch or (mostly) cold leg
    let mut tchx_35a = new_ndhx_tchx_horizontal_35a(initial_temperature);
    let mut tchx_35b = new_ndhx_tchx_vertical_35b(initial_temperature);
    let mut static_mixer_60_label_36 = new_static_mixer_60_label_36(initial_temperature);
    let mut pipe_36a = new_pipe_36a(initial_temperature);
    let mut pipe_37 = new_pipe_37(initial_temperature);
    let mut flowmeter_60_37a = new_flowmeter_60_37a(initial_temperature);
    let mut pipe_38 = new_pipe_38(initial_temperature);
    let mut pipe_39 = new_pipe_39(initial_temperature);

    // fluid mechanics bit 
    fn get_dracs_flowrate(dracs_branches: &FluidComponentSuperCollection) -> 
        MassRate {
            let pressure_change_across_each_branch = 
                dracs_branches.get_pressure_change(MassRate::ZERO);

            let mass_flowrate_across_each_branch: Vec<MassRate> = 
                dracs_branches.
                get_mass_flowrate_across_each_parallel_branch(
                    pressure_change_across_each_branch
                    );

            let mut mass_flowrate: MassRate = 
                mass_flowrate_across_each_branch[0];


            // get absolute value
            mass_flowrate = mass_flowrate.abs();

            mass_flowrate

        }
    // fluid mechanics calcs
    // now in a closure
    // I should probably code this as a function instead
    fn dracs_fluid_mechanics_calc_mass_rate(
        pipe_34: &InsulatedFluidComponent,
        pipe_33: &InsulatedFluidComponent,
        pipe_32: &InsulatedFluidComponent,
        pipe_31a: &InsulatedFluidComponent,
        static_mixer_61_label_31: &InsulatedFluidComponent,
        dhx_tube_side_30b: &NonInsulatedFluidComponent,
        dhx_tube_side_heat_exchanger_30: &NonInsulatedFluidComponent,
        dhx_tube_side_30a: &NonInsulatedFluidComponent,
        tchx_35a: &NonInsulatedFluidComponent,
        tchx_35b: &NonInsulatedFluidComponent,
        static_mixer_60_label_36: &InsulatedFluidComponent,
        pipe_36a: &InsulatedFluidComponent,
        pipe_37: &InsulatedFluidComponent,
        flowmeter_60_37a: &NonInsulatedFluidComponent,
        pipe_38: &InsulatedFluidComponent,
        pipe_39: &InsulatedFluidComponent,
    )-> MassRate {

        let mut dracs_hot_branch = 
            FluidComponentCollection::new_series_component_collection();

        dracs_hot_branch.clone_and_add_component(pipe_34);
        dracs_hot_branch.clone_and_add_component(pipe_33);
        dracs_hot_branch.clone_and_add_component(pipe_32);
        dracs_hot_branch.clone_and_add_component(pipe_31a);
        dracs_hot_branch.clone_and_add_component(static_mixer_61_label_31);
        dracs_hot_branch.clone_and_add_component(dhx_tube_side_30b);
        dracs_hot_branch.clone_and_add_component(dhx_tube_side_heat_exchanger_30);
        dracs_hot_branch.clone_and_add_component(dhx_tube_side_30a);


        let mut dracs_cold_branch = 
            FluidComponentCollection::new_series_component_collection();

        dracs_cold_branch.clone_and_add_component(tchx_35a);
        dracs_cold_branch.clone_and_add_component(tchx_35b);
        dracs_cold_branch.clone_and_add_component(static_mixer_60_label_36);
        dracs_cold_branch.clone_and_add_component(pipe_36a);
        dracs_cold_branch.clone_and_add_component(pipe_37);
        dracs_cold_branch.clone_and_add_component(flowmeter_60_37a);
        dracs_cold_branch.clone_and_add_component(pipe_38);
        dracs_cold_branch.clone_and_add_component(pipe_39);

        let mut dracs_branches = 
            FluidComponentSuperCollection::default();

        dracs_branches.set_orientation_to_parallel();
        dracs_branches.fluid_component_super_vector.push(dracs_hot_branch);
        dracs_branches.fluid_component_super_vector.push(dracs_cold_branch);

        let mass_rate = get_dracs_flowrate(&dracs_branches);

        mass_rate

    }

    // now the thermal hydraulics bit 
    fn calculate_dracs_thermal_hydraulics(
        mass_flowrate_counter_clockwise: MassRate,
        heat_rate_through_dhx: Power,
        tchx_heat_transfer_coeff: HeatTransfer,
        average_temperature_for_density_calcs: ThermodynamicTemperature,
        timestep: Time,
        pipe_34: &mut InsulatedFluidComponent,
        pipe_33: &mut InsulatedFluidComponent,
        pipe_32: &mut InsulatedFluidComponent,
        pipe_31a: &mut InsulatedFluidComponent,
        static_mixer_61_label_31: &mut InsulatedFluidComponent,
        dhx_tube_side_30b: &mut NonInsulatedFluidComponent,
        dhx_tube_side_heat_exchanger_30: &mut NonInsulatedFluidComponent,
        dhx_tube_side_30a: &mut NonInsulatedFluidComponent,
        tchx_35a: &mut NonInsulatedFluidComponent,
        tchx_35b: &mut NonInsulatedFluidComponent,
        static_mixer_60_label_36: &mut InsulatedFluidComponent,
        pipe_36a: &mut InsulatedFluidComponent,
        pipe_37: &mut InsulatedFluidComponent,
        flowmeter_60_37a: &mut NonInsulatedFluidComponent,
        pipe_38: &mut InsulatedFluidComponent,
        pipe_39: &mut InsulatedFluidComponent,
        ){

        // for an ideal situation, we have zero parasitic heat losses
        // therefore, for each component, except tchx, heat transfer 
        // coeff is zero

        let adiabatic_heat_transfer_coeff = HeatTransfer::ZERO;

        // create the heat transfer interaction 
        let advection_heat_transfer_interaction: HeatTransferInteractionType;

        // I'm going to create the advection interaction

        let average_therminol_density = 
            LiquidMaterial::TherminolVP1.try_get_density(
                average_temperature_for_density_calcs).unwrap();

        advection_heat_transfer_interaction = 
            HeatTransferInteractionType::
            new_advection_interaction(mass_flowrate_counter_clockwise, 
                                      average_therminol_density, 
                                      average_therminol_density);

        // now, let's link the fluid arrays using advection 
        // (no conduction here axially between arrays)
        {
            dhx_tube_side_30a.pipe_fluid_array.link_to_front(
                &mut dhx_tube_side_heat_exchanger_30.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            dhx_tube_side_heat_exchanger_30.pipe_fluid_array.link_to_front(
                &mut dhx_tube_side_30b.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            dhx_tube_side_30b.pipe_fluid_array.link_to_front(
                &mut static_mixer_61_label_31.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            static_mixer_61_label_31.pipe_fluid_array.link_to_front(
                &mut pipe_31a.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_31a.pipe_fluid_array.link_to_front(
                &mut pipe_32.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_32.pipe_fluid_array.link_to_front(
                &mut pipe_33.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_33.pipe_fluid_array.link_to_front(
                &mut pipe_34.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_34.pipe_fluid_array.link_to_front(
                &mut tchx_35a.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            tchx_35a.pipe_fluid_array.link_to_front(
                &mut tchx_35b.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            tchx_35b.pipe_fluid_array.link_to_front(
                &mut static_mixer_60_label_36.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            static_mixer_60_label_36.pipe_fluid_array.link_to_front(
                &mut pipe_36a.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_36a.pipe_fluid_array.link_to_front(
                &mut pipe_37.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_37.pipe_fluid_array.link_to_front(
                &mut flowmeter_60_37a.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            flowmeter_60_37a.pipe_fluid_array.link_to_front(
                &mut pipe_38.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_38.pipe_fluid_array.link_to_front(
                &mut pipe_39.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

            pipe_39.pipe_fluid_array.link_to_front(
                &mut dhx_tube_side_30a.pipe_fluid_array, 
                advection_heat_transfer_interaction)
                .unwrap();

        }
        // set the relevant heat transfer coefficients 
        // all zero except for tchx
        {
            // hot branch
            dhx_tube_side_30a.heat_transfer_to_ambient = 
                adiabatic_heat_transfer_coeff;
            dhx_tube_side_heat_exchanger_30.heat_transfer_to_ambient = 
                adiabatic_heat_transfer_coeff;
            dhx_tube_side_30b.heat_transfer_to_ambient = 
                adiabatic_heat_transfer_coeff;

            static_mixer_61_label_31.heat_transfer_to_ambient = 
                adiabatic_heat_transfer_coeff;
            pipe_31a.heat_transfer_to_ambient = 
                adiabatic_heat_transfer_coeff;

            pipe_32.heat_transfer_to_ambient = 
                adiabatic_heat_transfer_coeff;
            pipe_33.heat_transfer_to_ambient = 
                adiabatic_heat_transfer_coeff;
            pipe_34.heat_transfer_to_ambient = 
                adiabatic_heat_transfer_coeff;

            // cold branch 
            tchx_35a.heat_transfer_to_ambient = 
                tchx_heat_transfer_coeff;
            tchx_35b.heat_transfer_to_ambient = 
                tchx_heat_transfer_coeff;

            static_mixer_60_label_36.heat_transfer_to_ambient = 
                adiabatic_heat_transfer_coeff;
            pipe_36a.heat_transfer_to_ambient = 
                adiabatic_heat_transfer_coeff;

            pipe_37.heat_transfer_to_ambient = 
                adiabatic_heat_transfer_coeff;
            pipe_38.heat_transfer_to_ambient = 
                adiabatic_heat_transfer_coeff;
            pipe_39.heat_transfer_to_ambient = 
                adiabatic_heat_transfer_coeff;

        }
        // add lateral heat losses and power through dhx
        {
            let zero_power: Power = Power::ZERO;

            // hot branch
            //
            // we add heat in through dhx 30 
            // everywhere else is zero heater power
            dhx_tube_side_30a
                .lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, 
                zero_power)
                .unwrap();
            dhx_tube_side_heat_exchanger_30
                .lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, 
                heat_rate_through_dhx)
                .unwrap();
            dhx_tube_side_30b
                .lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, 
                zero_power)
                .unwrap();

            static_mixer_61_label_31
                .lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, 
                zero_power)
                .unwrap();
            pipe_31a
                .lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, 
                zero_power)
                .unwrap();
            pipe_32
                .lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, 
                zero_power)
                .unwrap();
            pipe_33
                .lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, 
                zero_power)
                .unwrap();
            pipe_34
                .lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, 
                zero_power)
                .unwrap();

            // cold branch 
            tchx_35a
                .lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, 
                zero_power)
                .unwrap();
            tchx_35b
                .lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, 
                zero_power)
                .unwrap();

            static_mixer_60_label_36
                .lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, 
                zero_power)
                .unwrap();
            pipe_36a
                .lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, 
                zero_power)
                .unwrap();

            pipe_37
                .lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, 
                zero_power)
                .unwrap();
            pipe_38
                .lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, 
                zero_power)
                .unwrap();
            pipe_39
                .lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, 
                zero_power)
                .unwrap();

        }
        
        // now we should be ready to advance timestep
        {
            dhx_tube_side_30a
                .advance_timestep(timestep)
                .unwrap();
            dhx_tube_side_heat_exchanger_30
                .advance_timestep(timestep)
                .unwrap();
            dhx_tube_side_30b
                .advance_timestep(timestep)
                .unwrap();

            static_mixer_61_label_31
                .advance_timestep(timestep)
                .unwrap();
            pipe_31a
                .advance_timestep(timestep)
                .unwrap();

            pipe_32
                .advance_timestep(timestep)
                .unwrap();
            pipe_33
                .advance_timestep(timestep)
                .unwrap();
            pipe_34
                .advance_timestep(timestep)
                .unwrap();

            // cold branch 
            tchx_35a
                .advance_timestep(timestep)
                .unwrap();
            tchx_35b
                .advance_timestep(timestep)
                .unwrap();

            static_mixer_60_label_36
                .advance_timestep(timestep)
                .unwrap();
            pipe_36a
                .advance_timestep(timestep)
                .unwrap();

            pipe_37
                .advance_timestep(timestep)
                .unwrap();
            pipe_38
                .advance_timestep(timestep)
                .unwrap();
            pipe_39
                .advance_timestep(timestep)
                .unwrap();
        }

        // we do it in serial, so it keeps things simple 
        // now we are done

    }
    
    // let's calculate for 100 timesteps of 

    for iteration_idx_i in 0..100 {
        // fluid first 
        // 
        let mass_flowrate_absolute: MassRate = 
            dracs_fluid_mechanics_calc_mass_rate(
                &pipe_34, 
                &pipe_33, 
                &pipe_32, 
                &pipe_31a, 
                &static_mixer_61_label_31, 
                &dhx_tube_side_30b, 
                &dhx_tube_side_heat_exchanger_30, 
                &dhx_tube_side_30a, 
                &tchx_35a, 
                &tchx_35b, 
                &static_mixer_60_label_36, 
                &pipe_36a, 
                &pipe_37, 
                &flowmeter_60_37a, 
                &pipe_38, 
                &pipe_39);


        // I assume the mass_flowrate_counter_clockwise 
        // is the same as absolute flowrate
        let mass_flowrate_counter_clockwise = 
            mass_flowrate_absolute;

        // next, thermal hydraulics calcs 

        calculate_dracs_thermal_hydraulics(
            mass_flowrate_counter_clockwise, 
            heat_rate_through_dhx, 
            tchx_heat_transfer_coeff, 
            average_temperature_for_density_calcs, 
            timestep, 
            &mut pipe_34, 
            &mut pipe_33, 
            &mut pipe_32, 
            &mut pipe_31a, 
            &mut static_mixer_61_label_31, 
            &mut dhx_tube_side_30b, 
            &mut dhx_tube_side_heat_exchanger_30, 
            &mut dhx_tube_side_30a, 
            &mut tchx_35a, 
            &mut tchx_35b, 
            &mut static_mixer_60_label_36, 
            &mut pipe_36a, 
            &mut pipe_37, 
            &mut flowmeter_60_37a, 
            &mut pipe_38, 
            &mut pipe_39);



        //// show the mass flowrate
        //dbg!(&mass_flowrate_absolute);

        // if this is the last loop, 
        // assert that mass flowrate is equal some value 
        // 0.008090506 kg/s
        if iteration_idx_i == 99 {

            use approx::assert_abs_diff_eq;
            assert_abs_diff_eq!(
                mass_flowrate_absolute.get::<uom::si::mass_rate::kilogram_per_second>(),
                0.008090506,
                epsilon = 1e-7);
        }

    }

    // panic to see debug messages

    //panic!();


}

/// This next set of tests shows explicitly what we need to do in 
/// the fluid component collection in order to get natural circulation
///
/// prototype test one, I tried using the branch builders and 
/// mutating the FluidComponents within them 
/// but this proved to be futile
#[test]
pub fn dracs_natural_circ_thermal_hydraulics_test_prototype_1(){

    // let's construct the branches with test pressures and obtain 
    use crate::
        array_control_vol_and_fluid_component_collections::
        fluid_component_collection::
        fluid_component_collection::FluidComponentCollectionMethods;
    use uom::si::f64::*;
    use uom::ConstZero;
    use uom::si::mass_rate::kilogram_per_second;

    use uom::si::thermodynamic_temperature::degree_celsius;
    use crate::
        array_control_vol_and_fluid_component_collections::
        fluid_component_collection::
        fluid_component_super_collection::FluidComponentSuperCollection;

    let cold_temperature = ThermodynamicTemperature::
        new::<degree_celsius>(46.0);
    let hot_temperature = ThermodynamicTemperature::
        new::<degree_celsius>(80.0);

    // we have our hot and cold branches first
    let dracs_hot_branch = dracs_hot_branch_builder(hot_temperature);
    let dracs_cold_branch = dracs_cold_branch_builder(cold_temperature);


    // make our super component collection 
    
    let mut dracs_branches = 
        FluidComponentSuperCollection::default();

    dracs_branches.set_orientation_to_parallel();
    dracs_branches.fluid_component_super_vector.push(dracs_hot_branch);
    dracs_branches.fluid_component_super_vector.push(dracs_cold_branch);

    // let's get an initial flowrate
    //
    // the mass flowrate through both branches nets to zero

    let pressure_change_across_each_branch = 
        dracs_branches.get_pressure_change(MassRate::ZERO);

    let mass_flowrate_across_each_branch: Vec<MassRate> = 
        dracs_branches.
        get_mass_flowrate_across_each_parallel_branch(
            pressure_change_across_each_branch
            );

    let mut mass_flowrate_initial: MassRate = 
        mass_flowrate_across_each_branch[0];


    // get absolute value
    mass_flowrate_initial = mass_flowrate_initial.abs();


    // initial mass flowrate is 0.0.064491 kg/s
    approx::assert_abs_diff_eq!(
        mass_flowrate_initial.get::<kilogram_per_second>(),
        0.0644914,
        epsilon=0.000001);

    // it would be good to code a function that just takes a clone 
    // of the super collection and gets the flowrate
    //
    // you could just as well do an immutable reference and that would be 
    // okay
    fn get_dracs_flowrate(dracs_branches: &FluidComponentSuperCollection) -> 
        MassRate {
            let pressure_change_across_each_branch = 
                dracs_branches.get_pressure_change(MassRate::ZERO);

            let mass_flowrate_across_each_branch: Vec<MassRate> = 
                dracs_branches.
                get_mass_flowrate_across_each_parallel_branch(
                    pressure_change_across_each_branch
                    );

            let mut mass_flowrate: MassRate = 
                mass_flowrate_across_each_branch[0];


            // get absolute value
            mass_flowrate = mass_flowrate.abs();

            mass_flowrate

        }


    // let's try this function, it should make it very succinct to get 
    // the flowrate now

    mass_flowrate_initial = get_dracs_flowrate(&dracs_branches);

    // initial mass flowrate is 0.0644914 kg/s
    approx::assert_abs_diff_eq!(
        mass_flowrate_initial.get::<kilogram_per_second>(),
        0.0644914,
        epsilon=0.000001);

    // now we just have to obtain the temperatures given this 
    // mass flowrate
    // basically, we need to attach advection interactions all 
    // across the loop, and ensure that the DHX receives a constant 
    // heat input boundary condition
    //
    // Each component then experiences either an adiabatic BC to ambient 
    // or an ambient temperature BC with some ambient temperature
    //
    // it is kind of cumbersome to connect the pipes one at a time
    // to each boundary condition so we should automate it
    //
    // one way is to add a method to the fluid component collection
    // as a trait at least for advection
    //
    // this supposes they are in series though!
    //
    // I can prototype a function here, and then move it over to a 
    // trait implementation

    pub fn _dracs_thermal_hydraulics_calcs_hot_branch(
        hot_branch_ref: &mut FluidComponentCollection,
        _top_to_bottom_flowrate: MassRate){

        // now the manual work starts, 
        // the first prototype is quite naive, I'll just clone the 
        // components out one by one

        let _pipe_34 = hot_branch_ref.components[0].clone();
        let _pipe_33 = hot_branch_ref.components[1].clone();
        let _pipe_32 = hot_branch_ref.components[2].clone();
        let _pipe_31a = hot_branch_ref.components[3].clone();
        let _static_mixer_61_label_31 = hot_branch_ref.components[4].clone();
        let _dhx_tube_side_30b = hot_branch_ref.components[5].clone();
        let _dhx_tube_side_heat_exchanger_30 = hot_branch_ref.components[6].clone();
        let _dhx_tube_side_30a = hot_branch_ref.components[7].clone();

        // for the hot leg, all pipes experience advection given a mass 
        // flowrate
        // my convention is a top to bottom flowrate 
        // ie. from pipe_34 all the way to 30a
        //
        // Now, there should of course be advection between pipe 34 
        // and the cold leg as well. so that will be an issue to solve 
        // later for multiple parallel branches. But that's for later..
        //
        // oops, there is a problem,
        // the FluidComponentCollection does not contain any data about 
        // heat transfer entities. But only the fluid arrays
        //
        // So I cannot really clone from this component collection

    }
    
}
