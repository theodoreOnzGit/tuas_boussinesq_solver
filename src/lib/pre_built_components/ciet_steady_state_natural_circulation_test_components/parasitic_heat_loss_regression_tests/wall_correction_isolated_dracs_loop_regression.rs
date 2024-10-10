/// Zou, L., Hu, R., & Charpentier, A. (2019). SAM code
/// validation using the compact integral effects test (CIET) experimental
/// data (No. ANL/NSE-19/11). Argonne National
/// Lab.(ANL), Argonne, IL (United States).
///
/// Suppose the isolated DRACS loop had some parasitic heat loss
/// the heat losses primarily come due to a convective heat transfer to
/// the environment and the convective heat transfer to the wall.
/// The convective heat transfer to the wall is in turn governed
/// by the Gnielinski correlation, where there is an option to
/// turn on or off wall correction
///
/// The wall correction factor is (Pr_f/Pr_wall)^0.11 for liquids (not
/// gases)
///
/// for FLiBe, HITEC and Dowtherm A, Pr decreases with increasing
/// temperature.
///
/// So for parasitic heat loss, the wall temperature is cooler than
/// the center.
///
/// so (Pr_f/Pr_wall) < 1
/// (Pr_f/Pr_wall)^0.11 < 1
///
/// So with wall correction, parasitic heat loss is often a lot less
/// for (Pr_f/Pr_wall) = 0.5,
/// (Pr_f/Pr_wall)^0.11 = 0.92 (about 8% less that of the uncorrected heat loss)
///
/// For the opposite, where heating is concerned, where Pr_f/Pr_wall = 2
/// (Pr_f/Pr_wall)^0.11 = 1.08 (about 8% more that of the uncorrected heat loss)
///
/// In other words, wall correction factors account for directionality of
/// the heating and cooling on nusselt number
///
/// Now, because I have done a dracs loop already, I intend to use it
/// to test out the wall correction thing.
///
/// The DRACS loop usually operates in natural circulation for the
/// given flowrates. In natural-circulation, we usually have laminar flow
/// where wall correction is not important. Wall correction is
/// only important in turbulent flow.
///
/// In this section, I'm going to have a base case for
/// the isolated DRACS loop and introduce forced circulation to make it
/// turbulent
///
/// The key metric of testing is the DHX inlet temperature
/// or the dhx_tube_side_30a bulk fluid temp
///
/// For this case, wall correction does work, but 
/// it is not noticeable to within 0.5K, you have to go 1e-7K to see the 
/// difference
///
#[test]
//#[ignore = "comment out for debugging"]
pub fn parasitic_heat_loss_regression_tchx_out_319_kelvin_46_celsius() {
    use uom::si::{f64::*, power::watt};

    use crate::thermal_hydraulics_error::ThermalHydraulicsLibError;
    use std::thread;
    use uom::si::thermodynamic_temperature::kelvin;
    use uom::si::{frequency::hertz, ratio::ratio, time::millisecond};

    use crate::pre_built_components::
        ciet_steady_state_natural_circulation_test_components::dracs_loop_components::*;
    use uom::ConstZero;

    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use crate::pre_built_components::insulated_pipes_and_fluid_components::InsulatedFluidComponent;
    use crate::pre_built_components::non_insulated_fluid_components::NonInsulatedFluidComponent;

    use crate::boussinesq_thermophysical_properties::LiquidMaterial;
    use crate::heat_transfer_correlations::heat_transfer_interactions::
        heat_transfer_interaction_enums::HeatTransferInteractionType;
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::time::second;
    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;

    use chem_eng_real_time_process_control_simulator::alpha_nightly::transfer_fn_wrapper_and_enums::TransferFnTraits;
    use chem_eng_real_time_process_control_simulator::alpha_nightly::controllers::ProportionalController;
    use chem_eng_real_time_process_control_simulator::alpha_nightly::controllers::AnalogController;

    fn verify_isolated_dhx_forced_circ(
        input_power_watts: f64,
        mass_flowrate_counter_clockwise: MassRate,
        dhx_tube_side_30a_ref_temp_celsius: f64,
        turn_on_non_insulated_fluid_components_wall_correction: bool,
        turn_on_insulated_fluid_components_wall_correction: bool,
        test_name: &str
    ) -> Result<(), ThermalHydraulicsLibError> {
        let input_power = Power::new::<watt>(input_power_watts);

        // setup
        // set point is 308 kelvin (35C)
        let tchx_outlet_temperature_set_point =
            ThermodynamicTemperature::new::<degree_celsius>(35.0);
        let initial_temperature = tchx_outlet_temperature_set_point;

        let timestep = Time::new::<second>(0.2);
        let heat_rate_through_dhx = input_power;
        let mut tchx_heat_transfer_coeff: HeatTransfer;

        let reference_tchx_htc = HeatTransfer::new::<watt_per_square_meter_kelvin>(40.0);
        let average_temperature_for_density_calcs =
            ThermodynamicTemperature::new::<degree_celsius>(80.0);
        // let's calculate 750 seconds of simulated time
        // compared 800
        // hopefully reached steady state by then

        let mut current_simulation_time = Time::ZERO;
        let max_simulation_time = Time::new::<second>(800.0);

        // PID controller settings
        let controller_gain = Ratio::new::<ratio>(1.75);
        let integral_time: Time = controller_gain / Frequency::new::<hertz>(1.0);
        let derivative_time: Time = Time::new::<second>(1.0);
        // derivative time ratio
        let alpha: Ratio = Ratio::new::<ratio>(1.0);

        let mut pid_controller: AnalogController = AnalogController::new_filtered_pid_controller(
            controller_gain,
            integral_time,
            derivative_time,
            alpha,
        )
        .unwrap();

        // we also have a measurement delay of 0.0001 s
        // or 0.1 ms
        let measurement_delay = Time::new::<millisecond>(0.1);

        let mut measurement_delay_block: AnalogController =
            ProportionalController::new(Ratio::new::<ratio>(1.0))
                .unwrap()
                .into();

        measurement_delay_block.set_dead_time(measurement_delay);

        // hot branch or (mostly) hot leg
        let mut pipe_34 = new_pipe_34(initial_temperature);
        let mut pipe_33 = new_pipe_33(initial_temperature);
        let mut pipe_32 = new_pipe_32(initial_temperature);
        let mut pipe_31a = new_pipe_31a(initial_temperature);
        let mut static_mixer_61_label_31 = new_static_mixer_61_label_31(initial_temperature);
        let mut dhx_tube_side_30b = new_dhx_tube_side_30b(initial_temperature);
        let mut dhx_tube_side_heat_exchanger_30 =
            new_isolated_dhx_tube_side_30(initial_temperature);
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
            flowmeter_60_37a: &mut NonInsulatedFluidComponent,
            pipe_38: &mut InsulatedFluidComponent,
            pipe_39: &mut InsulatedFluidComponent,
            turn_on_insulated_fluid_components_wall_correction: bool,
            turn_on_non_insulated_fluid_components_wall_correction: bool,
        ) {
            // for an ideal situation, we have zero parasitic heat losses
            // therefore, for each component, except tchx, heat transfer
            // coeff is zero
            //
            // heat transfer coeff at 90 to make parasitic heat loss more noticeable

            let pipe_to_air_heat_transfer_coeff =
                HeatTransfer::new::<watt_per_square_meter_kelvin>(90.0);

            // create the heat transfer interaction
            let advection_heat_transfer_interaction: HeatTransferInteractionType;

            // I'm going to create the advection interaction

            let average_therminol_density = LiquidMaterial::TherminolVP1
                .try_get_density(average_temperature_for_density_calcs)
                .unwrap();

            advection_heat_transfer_interaction =
                HeatTransferInteractionType::new_advection_interaction(
                    mass_flowrate_counter_clockwise,
                    average_therminol_density,
                    average_therminol_density,
                );

            // now, let's link the fluid arrays using advection
            // (no conduction here axially between arrays)
            {
                dhx_tube_side_30a
                    .pipe_fluid_array
                    .link_to_front(
                        &mut dhx_tube_side_heat_exchanger_30.pipe_fluid_array,
                        advection_heat_transfer_interaction,
                    )
                    .unwrap();

                dhx_tube_side_heat_exchanger_30
                    .pipe_fluid_array
                    .link_to_front(
                        &mut dhx_tube_side_30b.pipe_fluid_array,
                        advection_heat_transfer_interaction,
                    )
                    .unwrap();

                dhx_tube_side_30b
                    .pipe_fluid_array
                    .link_to_front(
                        &mut static_mixer_61_label_31.pipe_fluid_array,
                        advection_heat_transfer_interaction,
                    )
                    .unwrap();

                static_mixer_61_label_31
                    .pipe_fluid_array
                    .link_to_front(
                        &mut pipe_31a.pipe_fluid_array,
                        advection_heat_transfer_interaction,
                    )
                    .unwrap();

                pipe_31a
                    .pipe_fluid_array
                    .link_to_front(
                        &mut pipe_32.pipe_fluid_array,
                        advection_heat_transfer_interaction,
                    )
                    .unwrap();

                pipe_32
                    .pipe_fluid_array
                    .link_to_front(
                        &mut pipe_33.pipe_fluid_array,
                        advection_heat_transfer_interaction,
                    )
                    .unwrap();

                pipe_33
                    .pipe_fluid_array
                    .link_to_front(
                        &mut pipe_34.pipe_fluid_array,
                        advection_heat_transfer_interaction,
                    )
                    .unwrap();

                pipe_34
                    .pipe_fluid_array
                    .link_to_front(
                        &mut tchx_35a.pipe_fluid_array,
                        advection_heat_transfer_interaction,
                    )
                    .unwrap();

                tchx_35a
                    .pipe_fluid_array
                    .link_to_front(
                        &mut tchx_35b.pipe_fluid_array,
                        advection_heat_transfer_interaction,
                    )
                    .unwrap();

                tchx_35b
                    .pipe_fluid_array
                    .link_to_front(
                        &mut static_mixer_60_label_36.pipe_fluid_array,
                        advection_heat_transfer_interaction,
                    )
                    .unwrap();

                static_mixer_60_label_36
                    .pipe_fluid_array
                    .link_to_front(
                        &mut pipe_36a.pipe_fluid_array,
                        advection_heat_transfer_interaction,
                    )
                    .unwrap();

                pipe_36a
                    .pipe_fluid_array
                    .link_to_front(
                        &mut pipe_37.pipe_fluid_array,
                        advection_heat_transfer_interaction,
                    )
                    .unwrap();

                pipe_37
                    .pipe_fluid_array
                    .link_to_front(
                        &mut flowmeter_60_37a.pipe_fluid_array,
                        advection_heat_transfer_interaction,
                    )
                    .unwrap();

                flowmeter_60_37a
                    .pipe_fluid_array
                    .link_to_front(
                        &mut pipe_38.pipe_fluid_array,
                        advection_heat_transfer_interaction,
                    )
                    .unwrap();

                pipe_38
                    .pipe_fluid_array
                    .link_to_front(
                        &mut pipe_39.pipe_fluid_array,
                        advection_heat_transfer_interaction,
                    )
                    .unwrap();

                pipe_39
                    .pipe_fluid_array
                    .link_to_front(
                        &mut dhx_tube_side_30a.pipe_fluid_array,
                        advection_heat_transfer_interaction,
                    )
                    .unwrap();
            }
            // set the relevant heat transfer coefficients
            // all zero except for tchx
            {
                // hot branch
                dhx_tube_side_30a.heat_transfer_to_ambient = pipe_to_air_heat_transfer_coeff;
                dhx_tube_side_heat_exchanger_30.heat_transfer_to_ambient =
                    pipe_to_air_heat_transfer_coeff;
                dhx_tube_side_30b.heat_transfer_to_ambient = pipe_to_air_heat_transfer_coeff;

                static_mixer_61_label_31.heat_transfer_to_ambient = pipe_to_air_heat_transfer_coeff;
                pipe_31a.heat_transfer_to_ambient = pipe_to_air_heat_transfer_coeff;

                pipe_32.heat_transfer_to_ambient = pipe_to_air_heat_transfer_coeff;
                pipe_33.heat_transfer_to_ambient = pipe_to_air_heat_transfer_coeff;
                pipe_34.heat_transfer_to_ambient = pipe_to_air_heat_transfer_coeff;

                // cold branch
                tchx_35a.heat_transfer_to_ambient = tchx_heat_transfer_coeff;
                tchx_35b.heat_transfer_to_ambient = tchx_heat_transfer_coeff;

                static_mixer_60_label_36.heat_transfer_to_ambient = pipe_to_air_heat_transfer_coeff;
                pipe_36a.heat_transfer_to_ambient = pipe_to_air_heat_transfer_coeff;

                flowmeter_60_37a.heat_transfer_to_ambient = pipe_to_air_heat_transfer_coeff;

                pipe_37.heat_transfer_to_ambient = pipe_to_air_heat_transfer_coeff;
                pipe_38.heat_transfer_to_ambient = pipe_to_air_heat_transfer_coeff;
                pipe_39.heat_transfer_to_ambient = pipe_to_air_heat_transfer_coeff;
            }
            // add lateral heat losses and power through dhx
            {
                let zero_power: Power = Power::ZERO;

                // hot branch
                //
                // we add heat in through dhx 30
                // everywhere else is zero heater power

                if turn_on_insulated_fluid_components_wall_correction {
                    dhx_tube_side_30a
                        .lateral_and_miscellaneous_connections_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                    dhx_tube_side_heat_exchanger_30
                        .lateral_and_miscellaneous_connections_wall_correction(
                            mass_flowrate_counter_clockwise,
                            heat_rate_through_dhx,
                        )
                        .unwrap();
                    dhx_tube_side_30b
                        .lateral_and_miscellaneous_connections_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();

                    static_mixer_61_label_31
                        .lateral_and_miscellaneous_connections_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                    pipe_31a
                        .lateral_and_miscellaneous_connections_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                    pipe_32
                        .lateral_and_miscellaneous_connections_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                    pipe_33
                        .lateral_and_miscellaneous_connections_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                    pipe_34
                        .lateral_and_miscellaneous_connections_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();

                    // cold branch
                    // ambient temperature of tchx is 20C
                    tchx_35a.ambient_temperature =
                        ThermodynamicTemperature::new::<degree_celsius>(20.0);
                    tchx_35b.ambient_temperature =
                        ThermodynamicTemperature::new::<degree_celsius>(20.0);

                    tchx_35a
                        .lateral_and_miscellaneous_connections_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                    tchx_35b
                        .lateral_and_miscellaneous_connections_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();

                    static_mixer_60_label_36
                        .lateral_and_miscellaneous_connections_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                    pipe_36a
                        .lateral_and_miscellaneous_connections_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                    pipe_37
                        .lateral_and_miscellaneous_connections_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                    pipe_38
                        .lateral_and_miscellaneous_connections_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                    pipe_39
                        .lateral_and_miscellaneous_connections_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                } else {
                    dhx_tube_side_30a
                        .lateral_and_miscellaneous_connections_no_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                    dhx_tube_side_heat_exchanger_30
                        .lateral_and_miscellaneous_connections_no_wall_correction(
                            mass_flowrate_counter_clockwise,
                            heat_rate_through_dhx,
                        )
                        .unwrap();
                    dhx_tube_side_30b
                        .lateral_and_miscellaneous_connections_no_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();

                    static_mixer_61_label_31
                        .lateral_and_miscellaneous_connections_no_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                    pipe_31a
                        .lateral_and_miscellaneous_connections_no_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                    pipe_32
                        .lateral_and_miscellaneous_connections_no_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                    pipe_33
                        .lateral_and_miscellaneous_connections_no_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                    pipe_34
                        .lateral_and_miscellaneous_connections_no_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();

                    // cold branch
                    // ambient temperature of tchx is 20C
                    tchx_35a.ambient_temperature =
                        ThermodynamicTemperature::new::<degree_celsius>(20.0);
                    tchx_35b.ambient_temperature =
                        ThermodynamicTemperature::new::<degree_celsius>(20.0);

                    tchx_35a
                        .lateral_and_miscellaneous_connections_no_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                    tchx_35b
                        .lateral_and_miscellaneous_connections_no_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();

                    static_mixer_60_label_36
                        .lateral_and_miscellaneous_connections_no_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                    pipe_36a
                        .lateral_and_miscellaneous_connections_no_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                    pipe_37
                        .lateral_and_miscellaneous_connections_no_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                    pipe_38
                        .lateral_and_miscellaneous_connections_no_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                    pipe_39
                        .lateral_and_miscellaneous_connections_no_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                }

                if turn_on_non_insulated_fluid_components_wall_correction {
                    flowmeter_60_37a
                        .lateral_and_miscellaneous_connections_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                } else {
                    flowmeter_60_37a
                        .lateral_and_miscellaneous_connections_no_wall_correction(
                            mass_flowrate_counter_clockwise,
                            zero_power,
                        )
                        .unwrap();
                }
            }

            // now we should be ready to advance timestep
            {
                dhx_tube_side_30a.advance_timestep(timestep).unwrap();
                dhx_tube_side_heat_exchanger_30
                    .advance_timestep(timestep)
                    .unwrap();
                dhx_tube_side_30b.advance_timestep(timestep).unwrap();

                static_mixer_61_label_31.advance_timestep(timestep).unwrap();
                pipe_31a.advance_timestep(timestep).unwrap();

                pipe_32.advance_timestep(timestep).unwrap();
                pipe_33.advance_timestep(timestep).unwrap();
                pipe_34.advance_timestep(timestep).unwrap();

                // cold branch
                tchx_35a.advance_timestep(timestep).unwrap();
                tchx_35b.advance_timestep(timestep).unwrap();

                static_mixer_60_label_36.advance_timestep(timestep).unwrap();
                pipe_36a.advance_timestep(timestep).unwrap();

                flowmeter_60_37a.advance_timestep(timestep).unwrap();

                pipe_37.advance_timestep(timestep).unwrap();
                pipe_38.advance_timestep(timestep).unwrap();
                pipe_39.advance_timestep(timestep).unwrap();
            }

            // we do it in serial, so it keeps things simple
            // now we are done
        }

        // I also want to find the final temperature, which should be
        // around the set point within thermocouple error (+/- 0.5 K)
        let mut _final_tchx_outlet_temperature: ThermodynamicTemperature =
            ThermodynamicTemperature::ZERO;

        // main simulation loop
        while current_simulation_time < max_simulation_time {
            // show the outlet temperature of tchx

            let tchx_outlet_temperature: ThermodynamicTemperature = {
                // the front of the tchx is connected to static mixer
                // 60 label 36
                let tchx35b_pipe_fluid_array_clone: FluidArray =
                    tchx_35b.pipe_fluid_array.clone().try_into().unwrap();

                // take the front single cv temperature
                //
                // front single cv temperature is defunct
                // probably need to debug this

                let tchx_35b_front_single_cv_temperature: ThermodynamicTemperature =
                    tchx35b_pipe_fluid_array_clone.front_single_cv.temperature;

                let _tchx_35b_array_temperature: Vec<ThermodynamicTemperature> =
                    tchx_35b.pipe_fluid_array_temperature().unwrap();

                //dbg!(&tchx_35b_array_temperature);

                tchx_35b_front_single_cv_temperature
            };

            // we will need to change the tchx heat transfer coefficient
            // using the PID controller
            //
            // record tchx outlet temperature if it is last 5s of time

            let tchx_temperature_record_time_threshold =
                max_simulation_time - Time::new::<second>(5.0);

            if current_simulation_time > tchx_temperature_record_time_threshold {
                _final_tchx_outlet_temperature = tchx_outlet_temperature;
            }

            tchx_heat_transfer_coeff = {
                // first, calculate the set point error

                let reference_temperature_interval_deg_celsius = 80.0;

                // error = y_sp - y_measured
                let set_point_abs_error_deg_celsius = -tchx_outlet_temperature_set_point
                    .get::<kelvin>()
                    + tchx_outlet_temperature.get::<kelvin>();

                let nondimensional_error: Ratio = (set_point_abs_error_deg_celsius
                    / reference_temperature_interval_deg_celsius)
                    .into();

                // let's get the output

                let dimensionless_heat_trf_input: Ratio = pid_controller
                    .set_user_input_and_calc(nondimensional_error, current_simulation_time)
                    .unwrap();

                // the dimensionless output is:
                //
                // (desired output - ref_val)/ref_val = dimensionless_input
                //
                //
                // the reference value is decided by the user
                // in this case 250 W/(m^2 K)

                let mut tchx_heat_trf_output =
                    dimensionless_heat_trf_input * reference_tchx_htc + reference_tchx_htc;

                // make sure it cannot be less than a certain amount
                let tchx_minimum_heat_transfer =
                    HeatTransfer::new::<watt_per_square_meter_kelvin>(5.0);

                // this makes it physically realistic
                if tchx_heat_trf_output < tchx_minimum_heat_transfer {
                    tchx_heat_trf_output = tchx_minimum_heat_transfer;
                }

                tchx_heat_trf_output
            };
            // fluid first
            //

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
                &mut pipe_39,
                turn_on_insulated_fluid_components_wall_correction,
                turn_on_non_insulated_fluid_components_wall_correction,
                );

            current_simulation_time += timestep;
            let debug: bool = false;
            if debug {
                // show the mass flowrate
                // tchx outlet temperature
                // current sim time
                // and tchx heat trf coeff
                dbg!(&(
                        input_power,
                        tchx_outlet_temperature,
                        current_simulation_time,
                        tchx_heat_transfer_coeff
                ));
            }


        }

        // panic to see debug messages
        // todo!();
        //
        // final iteration, checking for the temperatures

        let dhx_tube_side_30a_temp: ThermodynamicTemperature = dhx_tube_side_30a
            .pipe_shell
            .try_get_bulk_temperature()
            .unwrap();
        dbg!(&(test_name,
                input_power, 
                dhx_tube_side_30a_ref_temp_celsius,
                dhx_tube_side_30a_temp.get::<degree_celsius>(),
                dhx_tube_side_30a_temp,));

        //
        // assert temperature of tchx and dhx tube side 30a
        // note, regression test is quite sensitive
        approx::assert_abs_diff_eq!(
            dhx_tube_side_30a_ref_temp_celsius,
            dhx_tube_side_30a_temp.get::<degree_celsius>(),
            epsilon = 1e-7
        );

        Ok(())
    }
    // spawn threads for faster testing
    //
    // first thread is baseline case
    let thread_1 = thread::spawn(|| {
        let turn_on_non_insulated_fluid_components_wall_correction = false;
        let turn_on_insulated_fluid_components_wall_correction = false;

        verify_isolated_dhx_forced_circ(
            2000.0, // 2000 watts
            MassRate::new::<kilogram_per_second>(0.18),
            31.6738342657, // dhx temp degree celsius at 800s to within 0.01K
                   // comparing to that at 750s to within 0.3 K
            turn_on_non_insulated_fluid_components_wall_correction,
            turn_on_insulated_fluid_components_wall_correction,
            "baseline_case"
        )
            .unwrap()
    });
    // second thread turns on the non_insulated_fluid_components
    // wall correction (less heat loss expected)
    let thread_2 = thread::spawn(|| {
        let turn_on_non_insulated_fluid_components_wall_correction = true;
        let turn_on_insulated_fluid_components_wall_correction = false;

        verify_isolated_dhx_forced_circ(
            2000.0, // 2000 watts
            MassRate::new::<kilogram_per_second>(0.18),
            31.673882867, // dhx temp degree celsius to within 0.01K
            turn_on_non_insulated_fluid_components_wall_correction,
            turn_on_insulated_fluid_components_wall_correction,
            "non_insulated_fluid_components wall correction"
        )
            .unwrap()
    });
    // third thread turns on the InsulatedFluidComponent
    // wall correction (less heat loss expected)
    let thread_3 = thread::spawn(|| {
        let turn_on_non_insulated_fluid_components_wall_correction = false;
        let turn_on_insulated_fluid_components_wall_correction = true;

        verify_isolated_dhx_forced_circ(
            2000.0, // 2000 watts
            MassRate::new::<kilogram_per_second>(0.18),
            31.67383298, // dhx temp degree celsius to within 0.01K
            turn_on_non_insulated_fluid_components_wall_correction,
            turn_on_insulated_fluid_components_wall_correction,
            "insulated_fluid_components wall correction"
        )
            .unwrap()
    });
    // fourth thread turns on the all
    // wall correction (less heat loss expected)
    let thread_4 = thread::spawn(|| {
        let turn_on_non_insulated_fluid_components_wall_correction = true;
        let turn_on_insulated_fluid_components_wall_correction = true;

        verify_isolated_dhx_forced_circ(
            2000.0, // 2000 watts
            MassRate::new::<kilogram_per_second>(0.18),
            31.67388158, // dhx temp degree celsius to within 0.01K
            turn_on_non_insulated_fluid_components_wall_correction,
            turn_on_insulated_fluid_components_wall_correction,
            "all components wall correction"
        )
            .unwrap()
    });

    thread_1.join().unwrap();
    thread_2.join().unwrap();
    thread_3.join().unwrap();
    thread_4.join().unwrap();

}
