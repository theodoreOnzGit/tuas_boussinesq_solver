



/// This next set of tests shows explicitly what we need to do in 
/// the fluid component collection in order to get natural circulation
///
/// prototype test two, 
///
/// found that I can't use closures woops
/// I'll just use simple functions instead 
/// quite lehcheh (troublesome) to type, but ok lah
#[test]
pub fn steel_properties_debugging(){

    use crate::pre_built_components::
        ciet_steady_state_natural_circulation_test_components::dracs_loop_components::*;
    // let's construct the branches with test pressures and obtain 
    use uom::si::f64::*;
    use uom::ConstZero;

    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use crate::pre_built_components::
        insulated_pipes_and_fluid_components::InsulatedFluidComponent;
    use crate::pre_built_components::
        non_insulated_fluid_components::NonInsulatedFluidComponent;

    use crate::boussinesq_thermophysical_properties::
        LiquidMaterial;
    use crate::heat_transfer_correlations::
        heat_transfer_interactions::
        heat_transfer_interaction_enums::HeatTransferInteractionType;
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::power::watt;
    use uom::si::time::second;
    // setup 
    // with a very low initial temperature, some of the thermal hydraulics 
    // tend to throw random errors. I wonder why
    // that is below 26.85 C 
    //
    // Now the graves correlation is from 300K - 700K and 
    // 20C is outside this range
    let initial_temperature = ThermodynamicTemperature::
        new::<degree_celsius>(20.0);
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
        _flowmeter_60_37a: &mut NonInsulatedFluidComponent,
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

            //dhx_tube_side_heat_exchanger_30.pipe_fluid_array.link_to_front(
            //    &mut dhx_tube_side_30b.pipe_fluid_array, 
            //    advection_heat_transfer_interaction)
            //    .unwrap();

            //dhx_tube_side_30b.pipe_fluid_array.link_to_front(
            //    &mut static_mixer_61_label_31.pipe_fluid_array, 
            //    advection_heat_transfer_interaction)
            //    .unwrap();

            //static_mixer_61_label_31.pipe_fluid_array.link_to_front(
            //    &mut pipe_31a.pipe_fluid_array, 
            //    advection_heat_transfer_interaction)
            //    .unwrap();

            //pipe_31a.pipe_fluid_array.link_to_front(
            //    &mut pipe_32.pipe_fluid_array, 
            //    advection_heat_transfer_interaction)
            //    .unwrap();

            //pipe_32.pipe_fluid_array.link_to_front(
            //    &mut pipe_33.pipe_fluid_array, 
            //    advection_heat_transfer_interaction)
            //    .unwrap();

            //pipe_33.pipe_fluid_array.link_to_front(
            //    &mut pipe_34.pipe_fluid_array, 
            //    advection_heat_transfer_interaction)
            //    .unwrap();

            //pipe_34.pipe_fluid_array.link_to_front(
            //    &mut tchx_35a.pipe_fluid_array, 
            //    advection_heat_transfer_interaction)
            //    .unwrap();

            //tchx_35a.pipe_fluid_array.link_to_front(
            //    &mut tchx_35b.pipe_fluid_array, 
            //    advection_heat_transfer_interaction)
            //    .unwrap();

            //tchx_35b.pipe_fluid_array.link_to_front(
            //    &mut static_mixer_60_label_36.pipe_fluid_array, 
            //    advection_heat_transfer_interaction)
            //    .unwrap();

            //static_mixer_60_label_36.pipe_fluid_array.link_to_front(
            //    &mut pipe_36a.pipe_fluid_array, 
            //    advection_heat_transfer_interaction)
            //    .unwrap();

            //pipe_36a.pipe_fluid_array.link_to_front(
            //    &mut pipe_37.pipe_fluid_array, 
            //    advection_heat_transfer_interaction)
            //    .unwrap();

            //pipe_37.pipe_fluid_array.link_to_front(
            //    &mut flowmeter_60_37a.pipe_fluid_array, 
            //    advection_heat_transfer_interaction)
            //    .unwrap();

            //flowmeter_60_37a.pipe_fluid_array.link_to_front(
            //    &mut pipe_38.pipe_fluid_array, 
            //    advection_heat_transfer_interaction)
            //    .unwrap();

            //pipe_38.pipe_fluid_array.link_to_front(
            //    &mut pipe_39.pipe_fluid_array, 
            //    advection_heat_transfer_interaction)
            //    .unwrap();

            //pipe_39.pipe_fluid_array.link_to_front(
            //    &mut dhx_tube_side_30a.pipe_fluid_array, 
            //    advection_heat_transfer_interaction)
            //    .unwrap();

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

            // buggy code here
            // turns out i forgot to switch thermal conductivity over to 
            // the Zweibaum version
            static_mixer_61_label_31
                .lateral_and_miscellaneous_connections_no_wall_correction(
                mass_flowrate_counter_clockwise, 
                zero_power)
                .unwrap();
            //pipe_31a
            //    .lateral_and_miscellaneous_connections(
            //    mass_flowrate_counter_clockwise, 
            //    zero_power)
            //    .unwrap();
            //pipe_32
            //    .lateral_and_miscellaneous_connections(
            //    mass_flowrate_counter_clockwise, 
            //    zero_power)
            //    .unwrap();
            //pipe_33
            //    .lateral_and_miscellaneous_connections(
            //    mass_flowrate_counter_clockwise, 
            //    zero_power)
            //    .unwrap();
            //pipe_34
            //    .lateral_and_miscellaneous_connections(
            //    mass_flowrate_counter_clockwise, 
            //    zero_power)
            //    .unwrap();

            //// cold branch 
            //tchx_35a
            //    .lateral_and_miscellaneous_connections(
            //    mass_flowrate_counter_clockwise, 
            //    zero_power)
            //    .unwrap();
            //tchx_35b
            //    .lateral_and_miscellaneous_connections(
            //    mass_flowrate_counter_clockwise, 
            //    zero_power)
            //    .unwrap();

            //static_mixer_60_label_36
            //    .lateral_and_miscellaneous_connections(
            //    mass_flowrate_counter_clockwise, 
            //    zero_power)
            //    .unwrap();
            //pipe_36a
            //    .lateral_and_miscellaneous_connections(
            //    mass_flowrate_counter_clockwise, 
            //    zero_power)
            //    .unwrap();

            //pipe_37
            //    .lateral_and_miscellaneous_connections(
            //    mass_flowrate_counter_clockwise, 
            //    zero_power)
            //    .unwrap();
            //pipe_38
            //    .lateral_and_miscellaneous_connections(
            //    mass_flowrate_counter_clockwise, 
            //    zero_power)
            //    .unwrap();
            //pipe_39
            //    .lateral_and_miscellaneous_connections(
            //    mass_flowrate_counter_clockwise, 
            //    zero_power)
            //    .unwrap();

        }
        
        // now we should be ready to advance timestep
        {
            dhx_tube_side_30a
                .advance_timestep(timestep)
                .unwrap();
        }

        // we do it in serial, so it keeps things simple 
        // now we are done

    }
    
    // let's calculate for 100 timesteps of 
        // I assume the mass_flowrate_counter_clockwise 
        // is the same as absolute flowrate
        let mass_flowrate_counter_clockwise = 
            MassRate::new::<kilogram_per_second>(0.1);

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


    // panic to see debug messages

    //panic!();


}
