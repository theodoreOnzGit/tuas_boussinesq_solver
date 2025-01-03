
use uom::si::angle::degree;
use uom::si::area::square_meter;
use uom::si::f64::*;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::length::{meter, millimeter};
use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::pressure::atmosphere;

use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
use crate::pre_built_components::insulated_pipes_and_fluid_components::InsulatedFluidComponent;
use crate::pre_built_components::non_insulated_fluid_components::NonInsulatedFluidComponent;
use crate::pre_built_components::non_insulated_parallel_fluid_components::NonInsulatedParallelFluidComponent;

/// In the original SAM publication
///
/// Zou, L., Hu, R., & Charpentier, A. (2019). SAM code 
/// validation using the compact integral effects test (CIET) experimental 
/// data (No. ANL/NSE-19/11). Argonne National 
/// Lab.(ANL), Argonne, IL (United States).
///
/// I found it hard to distinguish what TCHX temperatures case A,
/// B and C were.
///
/// But there was another publication which shows which is test group 
/// corresponds to which temperature:
///
/// Zou, L., Hu, G., O'Grady, D., & Hu, R. (2021). Code validation of 
/// SAM using natural-circulation experimental data from the compact 
/// integral effects test (CIET) facility. 
/// Nuclear Engineering and Design, 377, 111144.
///
/// According to table 2,
///
/// Case A has 7 tests and TCHX out temperature of 46 C
/// Case B has 9 tests and TCHX out temperature of 35 C
/// Case C has 9 tests and TCHX out temperature of 40 C
///
/// Table 3 also provides the data 
///
/// to ensure that the flow reached steady state,
/// the mass flowrate values at 4000s were recorded to within 1e-4 
/// 0.01%
///
/// Then the test is carried out at 3800s. If values match to within 
/// 0.44%, test is considered to have reached steady state 
/// This is because 0.44% is max error between SAM and its 
/// "analytical" solution
///
/// this specific test is for mesh refinement
///
#[test]
//#[ignore = "comment out for debugging"]
pub fn mesh_refined_case_c_tchx_out_313_kelvin_40_celsius(){

    use uom::si::{frequency::hertz, ratio::ratio, time::millisecond};
    use uom::si::thermodynamic_temperature::kelvin;
    use uom::si::f64::*;
    use uom::si::mass_rate::kilogram_per_second;
    use std::thread;

    use crate::tuas_lib_error::TuasLibError;

    use crate::
        array_control_vol_and_fluid_component_collections::
        fluid_component_collection::
        fluid_component_collection::FluidComponentCollection;
    // let's construct the branches with test pressures and obtain 
    use crate::
        array_control_vol_and_fluid_component_collections::
        fluid_component_collection::
        fluid_component_collection::FluidComponentCollectionMethods;
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
    use crate::heat_transfer_correlations::heat_transfer_interactions::
        heat_transfer_interaction_enums::HeatTransferInteractionType;
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::power::watt;
    use uom::si::time::second;
    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;


    use chem_eng_real_time_process_control_simulator::alpha_nightly::transfer_fn_wrapper_and_enums::TransferFnTraits;
    use chem_eng_real_time_process_control_simulator::alpha_nightly::controllers::ProportionalController;
    use chem_eng_real_time_process_control_simulator::alpha_nightly::controllers::AnalogController;

    fn verify_isolated_dhx_sam_solution(
        input_power_watts: f64,
        sam_solution_mass_flowrate_kg_per_sm: f64,
        tuas_boussinesq_solver_flowrate_kg_per_s_at_4000s_time: f64,
        max_error_tolerance_fraction: f64) -> 
        Result<(),TuasLibError>{

            let input_power = Power::new::<watt>(input_power_watts);
            let max_error_tolerance = max_error_tolerance_fraction;

            // max error is 0.5% according to SAM 
            // is okay, because typical flowmeter measurement error is 2% anyway
            //
            // but without calibration, usually, we'll have overestimation
            //
            // setup 
            // set point is 313 kelvin (40C)
            let tchx_outlet_temperature_set_point = 
                ThermodynamicTemperature::new::<degree_celsius>(40.0);
            let initial_temperature = tchx_outlet_temperature_set_point;

            let timestep = Time::new::<second>(0.25);
            let heat_rate_through_dhx = input_power;
            let mut tchx_heat_transfer_coeff: HeatTransfer;

            let reference_tchx_htc = 
                HeatTransfer::new::<watt_per_square_meter_kelvin>(40.0);
            let average_temperature_for_density_calcs = 
                ThermodynamicTemperature::new::<degree_celsius>(80.0);
            // let's calculate 3800 seconds of simulated time 
            // it takes about that long for the temperature to settle down

            let mut current_simulation_time = Time::ZERO;
            let max_simulation_time = Time::new::<second>(3800.0);

            // PID controller settings
            let controller_gain = Ratio::new::<ratio>(1.75);
            let integral_time: Time = controller_gain / Frequency::new::<hertz>(1.0);
            let derivative_time: Time = Time::new::<second>(1.0);
            // derivative time ratio
            let alpha: Ratio = Ratio::new::<ratio>(1.0);

            let mut pid_controller: AnalogController = 
                AnalogController::new_filtered_pid_controller(controller_gain,
                    integral_time,
                    derivative_time,
                    alpha).unwrap();

            // we also have a measurement delay of 0.0001 s 
            // or 0.1 ms
            let measurement_delay = Time::new::<millisecond>(0.1);

            let mut measurement_delay_block: AnalogController = 
                ProportionalController::new(Ratio::new::<ratio>(1.0)).unwrap().into();

            measurement_delay_block.set_dead_time(measurement_delay);


            // hot branch or (mostly) hot leg
            let mut pipe_34 = new_mesh_refined_pipe_34(initial_temperature);
            let mut pipe_33 = new_mesh_refined_pipe_33(initial_temperature);
            let mut pipe_32 = new_mesh_refined_pipe_32(initial_temperature);
            let mut pipe_31a = new_mesh_refined_pipe_31a(initial_temperature);
            let mut static_mixer_61_label_31 = new_mesh_refined_static_mixer_61_label_31(initial_temperature);
            let mut dhx_tube_side_30b = new_mesh_refined_dhx_tube_side_30b(initial_temperature);
            let mut dhx_tube_side_heat_exchanger_30 = new_mesh_refined_isolated_dhx_tube_side_30(initial_temperature);
            let mut dhx_tube_side_30a = new_mesh_refined_dhx_tube_side_30a(initial_temperature);

            // cold branch or (mostly) cold leg
            let mut tchx_35a = new_mesh_refined_ndhx_tchx_horizontal_35a(initial_temperature);
            let mut tchx_35b = new_mesh_refined_ndhx_tchx_vertical_35b(initial_temperature);
            let mut static_mixer_60_label_36 = new_mesh_refined_static_mixer_60_label_36(initial_temperature);
            let mut pipe_36a = new_mesh_refined_pipe_36a(initial_temperature);
            let mut pipe_37 = new_mesh_refined_pipe_37(initial_temperature);
            let mut flowmeter_60_37a = new_mesh_refined_flowmeter_60_37a(initial_temperature);
            let mut pipe_38 = new_mesh_refined_pipe_38(initial_temperature);
            let mut pipe_39 = new_mesh_refined_pipe_39(initial_temperature);

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
                        flowmeter_60_37a.heat_transfer_to_ambient
                            = adiabatic_heat_transfer_coeff;

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
                        // ambient temperature of tchx is 20C  
                        tchx_35a.ambient_temperature = 
                            ThermodynamicTemperature::new::<degree_celsius>(20.0);
                        tchx_35b.ambient_temperature = 
                            ThermodynamicTemperature::new::<degree_celsius>(20.0);

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
                        flowmeter_60_37a
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
                        flowmeter_60_37a
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

            // I also want to find the final temperature, which should be 
            // around the set point within thermocouple error (+/- 0.5 K)
            let mut _final_tchx_outlet_temperature: ThermodynamicTemperature 
                = ThermodynamicTemperature::ZERO;
            let mut final_mass_flowrate: MassRate 
                = MassRate::ZERO;

            // main simulation loop
            while current_simulation_time < max_simulation_time {
                // show the outlet temperature of tchx 

                let tchx_outlet_temperature: ThermodynamicTemperature = {

                    // the front of the tchx is connected to static mixer 
                    // 60 label 36
                    let tchx35b_pipe_fluid_array_clone: FluidArray = 
                        tchx_35b.pipe_fluid_array
                        .clone()
                        .try_into()
                        .unwrap();

                    // take the front single cv temperature 
                    //
                    // front single cv temperature is defunct
                    // probably need to debug this

                    let tchx_35b_front_single_cv_temperature: ThermodynamicTemperature 
                        = tchx35b_pipe_fluid_array_clone
                        .front_single_cv
                        .temperature;



                    let _tchx_35b_array_temperature: Vec<ThermodynamicTemperature>
                        = tchx_35b
                        .pipe_fluid_array_temperature()
                        .unwrap();

                    //dbg!(&tchx_35b_array_temperature);

                    tchx_35b_front_single_cv_temperature

                };

                // we will need to change the tchx heat transfer coefficient 
                // using the PID controller
                //
                // record tchx outlet temperature if it is last 5s of time 

                let tchx_temperature_record_time_threshold = max_simulation_time - 
                    Time::new::<second>(5.0);

                if current_simulation_time > tchx_temperature_record_time_threshold {
                    _final_tchx_outlet_temperature = tchx_outlet_temperature;
                }



                tchx_heat_transfer_coeff = {
                    // first, calculate the set point error 

                    let reference_temperature_interval_deg_celsius = 80.0;

                    // error = y_sp - y_measured
                    let set_point_abs_error_deg_celsius = 
                        - tchx_outlet_temperature_set_point.get::<kelvin>()
                        + tchx_outlet_temperature.get::<kelvin>();

                    let nondimensional_error: Ratio = 
                        (set_point_abs_error_deg_celsius/
                         reference_temperature_interval_deg_celsius).into();

                    // let's get the output 

                    let dimensionless_heat_trf_input: Ratio
                        = pid_controller.set_user_input_and_calc(
                            nondimensional_error, 
                            current_simulation_time).unwrap();

                    // the dimensionless output is:
                    //
                    // (desired output - ref_val)/ref_val = dimensionless_input
                    // 
                    //
                    // the reference value is decided by the user 
                    // in this case 250 W/(m^2 K)

                    let mut tchx_heat_trf_output = 
                        dimensionless_heat_trf_input * reference_tchx_htc
                        + reference_tchx_htc;

                    // make sure it cannot be less than a certain amount 
                    let tchx_minimum_heat_transfer = 
                        HeatTransfer::new::<watt_per_square_meter_kelvin>(
                            5.0);

                    // this makes it physically realistic
                    if tchx_heat_trf_output < tchx_minimum_heat_transfer {
                        tchx_heat_trf_output = tchx_minimum_heat_transfer;
                    }

                    tchx_heat_trf_output

                };
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

                if current_simulation_time > tchx_temperature_record_time_threshold {
                    final_mass_flowrate = mass_flowrate_absolute;
                }

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






                current_simulation_time += timestep;
                let debug: bool = false;
                if debug {
                    // show the mass flowrate
                    // tchx outlet temperature 
                    // current sim time 
                    // and tchx heat trf coeff
                    dbg!(&input_power);
                    dbg!(&mass_flowrate_absolute);
                    dbg!(&tchx_outlet_temperature);
                    dbg!(&current_simulation_time);
                    dbg!(&tchx_heat_transfer_coeff);
                }

            }

            // panic to see debug messages

            //panic!();
            dbg!(&(sam_solution_mass_flowrate_kg_per_sm,final_mass_flowrate));

            approx::assert_relative_eq!(
                tuas_boussinesq_solver_flowrate_kg_per_s_at_4000s_time,
                final_mass_flowrate.get::<kilogram_per_second>(),
                max_relative = 0.44e-2 // this is the max error between SAM and analytical
            );
            // final assertion 

            approx::assert_relative_eq!(
                sam_solution_mass_flowrate_kg_per_sm,
                final_mass_flowrate.get::<kilogram_per_second>(),
                max_relative = max_error_tolerance
                );

            Ok(())

    }

    let thread_0 = thread::spawn(
        ||{
            verify_isolated_dhx_sam_solution(
                582.6, // dhx power (watts)
                2.8033e-2, // SAM/ mass flowrate kg/s
                2.8358e-2, // tuas_boussinesq_solver_flowrate_kg_per_s (for regression)
                0.026, // max error tolerance
            ).unwrap();
        }
    );
    let thread_1 = thread::spawn(
        ||{
            verify_isolated_dhx_sam_solution(
                785.9, // dhx power (watts)
                3.1807e-2,// SAM/ mass flowrate kg/s
                3.2272e-2, // tuas_boussinesq_solver_flowrate_kg_per_s (for regression)
                0.026, // max error tolerance
            ).unwrap();
        }
    );
    let thread_2 = thread::spawn(
        ||{
            verify_isolated_dhx_sam_solution(
                971.4, // dhx power (watts)
                3.4690e-2,// SAM/ mass flowrate kg/s
                3.5319e-2, // tuas_boussinesq_solver_flowrate_kg_per_s (for regression)
                0.026, // max error tolerance
            ).unwrap();
        }
    );
    let thread_3 = thread::spawn(
        ||{
            verify_isolated_dhx_sam_solution(
                1185.2, // dhx power (watts)
                3.7765e-2,// SAM/ mass flowrate kg/s
                3.8408e-2, // tuas_boussinesq_solver_flowrate_kg_per_s (for regression)
                0.026, // max error tolerance
            ).unwrap();
        }
    );
    let thread_4 = thread::spawn(
        ||{
            verify_isolated_dhx_sam_solution(
                1369.1, // dhx power (watts)
                4.0100e-2,// SAM/ mass flowrate kg/s
                4.0793e-2, // tuas_boussinesq_solver_flowrate_kg_per_s (for regression)
                0.026, // max error tolerance
            ).unwrap();
        }
    );
    let thread_5 = thread::spawn(
        ||{
            verify_isolated_dhx_sam_solution(
                1584.1, // dhx power (watts)
                4.2499e-2,// SAM/ mass flowrate kg/s
                4.3334e-2, // tuas_boussinesq_solver_flowrate_kg_per_s (for regression)
                0.026, // max error tolerance
            ).unwrap();
        }
    );
    let thread_6 = thread::spawn(
        ||{
            verify_isolated_dhx_sam_solution(
                1763.7, // dhx power (watts)
                4.4448e-2,// SAM/ mass flowrate kg/s
                4.5292e-2, // tuas_boussinesq_solver_flowrate_kg_per_s (for regression)
                0.026, // max error tolerance
            ).unwrap();
        }
    );
    let thread_7 = thread::spawn(
        ||{
            verify_isolated_dhx_sam_solution(
                1970.0, // dhx power (watts)
                4.6455e-2,// SAM/ mass flowrate kg/s
                4.7387e-2, // tuas_boussinesq_solver_flowrate_kg_per_s (for regression)
                0.026, // max error tolerance
            ).unwrap();
        }
    );
    let thread_8 = thread::spawn(
        ||{
            verify_isolated_dhx_sam_solution(
                2177.0, // dhx power (watts)
                4.8315e-2,// SAM/ mass flowrate kg/s
                4.9350e-2, // tuas_boussinesq_solver_flowrate_kg_per_s (for regression)
                0.026, // max error tolerance
            ).unwrap();
        }
    );
    // max error tolerance is about 3.0%
    // for this, it's usually an overprediction of mass flowrate 
    // similar to SAM
    // join threads
    thread_0.join().unwrap();
    thread_1.join().unwrap();
    thread_2.join().unwrap();
    thread_3.join().unwrap();
    thread_4.join().unwrap();
    thread_5.join().unwrap();
    thread_6.join().unwrap();
    thread_7.join().unwrap();
    thread_8.join().unwrap();
}

/// hot leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// pipe 34, the horizontal pipe just besides the NDHX, the 
/// heat exchanger cooling the DRACS loop
///
pub fn new_mesh_refined_pipe_34(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    // note that for zweibaum's model,
    // the hydraulic diameter is 2.79e-2 m
    // whereas for SAM, the hydraulic diameter 
    // is 2.78638e-2 m
    // this is slightly smaller in SAM
    //
    // in Zweibaum's relap model, flow area is 6.11e-4 m^2
    // whereas for SAM
    // flow area is 6.099776e-4 m^2
    //
    // again, slightly smaller
    //
    // I tested if this impacts, 
    // it's quite insensitive
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.55245);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(0.0);
    let form_loss = Ratio::new::<ratio>(4.25);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 5 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 5-2
    let user_specified_inner_nodes_sam = 5-2; 
    // double number of nodes, then find inner nodes
    let mesh_refined_inner_nodes = 
        2 * (user_specified_inner_nodes_sam + 2) - 2;

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
        mesh_refined_inner_nodes, 
        surface_roughness);

    insulated_component
}


/// hot leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// pipe 33, the long vertical pipe just in the hot leg
///
pub fn new_mesh_refined_pipe_33(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(3.0099);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(2.75);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 28 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 28-2
    let user_specified_inner_nodes_sam = 28-2; 
    let mesh_refined_inner_nodes = 
        2 * (user_specified_inner_nodes_sam + 2) - 2;

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
        mesh_refined_inner_nodes, 
        surface_roughness);

    insulated_component
}


/// hot leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// pipe 32, the slanted pipe just in the hot leg next to the static mixer
///
pub fn new_mesh_refined_pipe_32(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.238125);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(54.422897-180.0);
    let form_loss = Ratio::new::<ratio>(0.8);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    let user_specified_inner_nodes_sam = 0; 
    let mesh_refined_inner_nodes = 
        2 * (user_specified_inner_nodes_sam + 2) - 2;

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
        mesh_refined_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// hot leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// static mixer pipe label 31, it is static mixer 61 on the P&ID for CIET
/// 
///
pub fn new_mesh_refined_static_mixer_61_label_31(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(0.33);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(21.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(4000.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    let user_specified_inner_nodes_sam = 0; 
    let mesh_refined_inner_nodes = 
        2 * (user_specified_inner_nodes_sam + 2) - 2;

    let insulated_component = InsulatedFluidComponent::new_custom_component(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        reynolds_coefficient, 
        reynolds_power, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        component_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        mesh_refined_inner_nodes);

    insulated_component
}

/// hot leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// static mixer pipe 31a, the slanted pipe just in the hot leg next to the static mixer
///
pub fn new_mesh_refined_pipe_31a(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.143075);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(1.35);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    let user_specified_inner_nodes_sam = 0; 
    let mesh_refined_inner_nodes = 
        2 * (user_specified_inner_nodes_sam + 2) - 2;

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
        mesh_refined_inner_nodes, 
        surface_roughness);



    insulated_component
}


/// hot leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// DHX tube side (top head) 30b
///
pub fn new_mesh_refined_dhx_tube_side_30b(initial_temperature: ThermodynamicTemperature) -> 
NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    // dhx tubes are modelled in SAM as 19 tubes of diameter 
    // 0.00635 m 
    // and flow area of 6.1072e-4 m^2
    //
    // in Zweibaum's RELAP model,
    // it is quite different from the SAM model 
    // which follows Bickel's data
    let hydraulic_diameter = Length::new::<meter>(6.35e-3);
    let pipe_length = Length::new::<meter>(0.18415);
    let flow_area = Area::new::<square_meter>(6.0172e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.00079375);
    let od = id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    let user_specified_inner_nodes_sam = 2-2; 
    let mesh_refined_inner_nodes = 
        2 * (user_specified_inner_nodes_sam + 2) - 2;

    let non_insulated_component = NonInsulatedFluidComponent::new_bare_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        id, 
        od, 
        pipe_length, 
        hydraulic_diameter, 
        surface_roughness, 
        pipe_shell_material, 
        pipe_fluid, 
        htc_to_ambient, 
        mesh_refined_inner_nodes);

    non_insulated_component
}

/// hot leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// DHX tube side 30
///
/// Here is where the main heat exchange happens. 
/// This one is for an isolated DHX.
///
/// Alternate code is needed for a coupled DHX
///
pub fn new_mesh_refined_isolated_dhx_tube_side_30(initial_temperature: ThermodynamicTemperature) -> 
NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    // for dhx modelling,
    //
    // dhx tubes are modelled in SAM as 19 tubes of diameter 
    // 0.00635 m 
    // and flow area of 6.1072e-4 m^2
    //
    // in Zweibaum's RELAP model,
    // it is quite different from the SAM model 
    // which follows Bickel's data
    let hydraulic_diameter = Length::new::<meter>(6.35e-3);
    let pipe_length = Length::new::<meter>(1.18745);
    let flow_area = Area::new::<square_meter>(6.0172e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(3.3);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.00079375);
    let od = id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 11 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 11-2
    let user_specified_inner_nodes_sam = 11-2; 
    let mesh_refined_inner_nodes = 
        2 * (user_specified_inner_nodes_sam + 2) - 2;

    let mut non_insulated_component = NonInsulatedFluidComponent::new_bare_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        id, 
        od, 
        pipe_length, 
        hydraulic_diameter, 
        surface_roughness, 
        pipe_shell_material, 
        pipe_fluid, 
        htc_to_ambient, 
        mesh_refined_inner_nodes);

    // for heat exchangers, I give an ideal Nusselt number correlation 
    // as an approximation so that film thermal resistance is minimised
    let mut fluid_array_ideal_nusslet: FluidArray = 
        non_insulated_component.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_ideal_nusslet.nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    non_insulated_component.pipe_fluid_array = 
        fluid_array_ideal_nusslet.into();

    non_insulated_component
}

/// hot leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// DHX tube side 30 with parallel tube modelling
///
/// Here is where the main heat exchange happens. 
/// This one is for an isolated DHX.
///
/// Alternate code is needed for a coupled DHX
///
pub fn _new_mesh_refined_isolated_dhx_tube_side_30_parallel_tubes(
    initial_temperature: ThermodynamicTemperature) -> 
NonInsulatedParallelFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    // for dhx modelling,
    //
    // dhx tubes are modelled in SAM as 19 tubes of diameter 
    // 0.00635 m 
    // and flow area of 6.1072e-4 m^2
    //
    // in Zweibaum's RELAP model,
    // it is quite different from the SAM model 
    // which follows Bickel's data
    let hydraulic_diameter = Length::new::<meter>(6.35e-3);
    let pipe_length = Length::new::<meter>(1.18745);
    let flow_area = Area::new::<square_meter>(6.0172e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(3.3);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.00079375);
    let od = id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 11 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 11-2
    let user_specified_inner_nodes_sam = 11-2; 
    let mesh_refined_inner_nodes = 
        2 * (user_specified_inner_nodes_sam + 2) - 2;
    let number_of_tubes: u32 = 19;

    let mut non_insulated_component_parallel = NonInsulatedParallelFluidComponent::new_bare_pipe_parallel_array(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        id, 
        od, 
        pipe_length, 
        hydraulic_diameter, 
        surface_roughness, 
        pipe_shell_material, 
        pipe_fluid, 
        htc_to_ambient, 
        mesh_refined_inner_nodes,
        number_of_tubes);

    // for heat exchangers, I give an ideal Nusselt number correlation 
    // as an approximation so that film thermal resistance is minimised
    let mut fluid_array_ideal_nusslet: FluidArray = 
        non_insulated_component_parallel.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_ideal_nusslet.nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    non_insulated_component_parallel.pipe_fluid_array = 
        fluid_array_ideal_nusslet.into();

    non_insulated_component_parallel
}

/// hot leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// DHX tube side (bottom head) 30a
///
pub fn new_mesh_refined_dhx_tube_side_30a(initial_temperature: ThermodynamicTemperature) -> 
NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    // dhx tubes are modelled in SAM as 19 tubes of diameter 
    // 0.00635 m 
    // and flow area of 6.1072e-4 m^2
    //
    // in Zweibaum's RELAP model,
    // it is quite different from the SAM model 
    // which follows Bickel's data
    let hydraulic_diameter = Length::new::<meter>(6.35e-3);
    let flow_area = Area::new::<square_meter>(6.0172e-4);
    let pipe_length = Length::new::<meter>(0.111125);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.00079375);
    let od = id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    let user_specified_inner_nodes_sam = 2-2; 
    let mesh_refined_inner_nodes = 
        2 * (user_specified_inner_nodes_sam + 2) - 2;

    let non_insulated_component = NonInsulatedFluidComponent::new_bare_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        id, 
        od, 
        pipe_length, 
        hydraulic_diameter, 
        surface_roughness, 
        pipe_shell_material, 
        pipe_fluid, 
        htc_to_ambient, 
        mesh_refined_inner_nodes);

    non_insulated_component
}

/// cold leg of DRACS (or what I consider the cold branch)
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
///
/// label 35a on RELAP model by Zweibaum
/// horizontal part of the TCHX or NDHX, 
/// has the same loss correlations as the CTAH (horizontal)
///
pub fn new_mesh_refined_ndhx_tchx_horizontal_35a(
    initial_temperature: ThermodynamicTemperature) -> NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(1.19e-2);
    let component_length = Length::new::<meter>(1.148475);
    let flow_area = Area::new::<square_meter>(1.33E-03);
    let incline_angle = Angle::new::<degree>(0.0);
    let form_loss = Ratio::new::<ratio>(400.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(52000_f64);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.000406);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 11 nodes only, 
    // now because there are two outer nodes, 
    // we subtract 2 
    let user_specified_inner_nodes_sam = 11-2; 
    let mesh_refined_inner_nodes = 
        2 * (user_specified_inner_nodes_sam + 2) - 2;

    let mut non_insulated_component = NonInsulatedFluidComponent::
        new_custom_component(
            initial_temperature, 
            ambient_temperature, 
            fluid_pressure, 
            solid_pressure, 
            flow_area, 
            incline_angle, 
            form_loss, 
            reynolds_coefficient, 
            reynolds_power, 
            shell_id, 
            shell_od, 
            component_length, 
            hydraulic_diameter, 
            pipe_shell_material, 
            pipe_fluid, 
            htc_to_ambient, 
            mesh_refined_inner_nodes);

    // for heat exchangers, I give an ideal Nusselt number correlation 
    // as an approximation so that film thermal resistance is minimised
    let mut fluid_array_ideal_nusslet: FluidArray = 
        non_insulated_component.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_ideal_nusslet.nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    non_insulated_component.pipe_fluid_array = 
        fluid_array_ideal_nusslet.into();


    non_insulated_component
}

/// cold leg of DRACS (or what I consider the cold branch)
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
///
/// label 35b on RELAP model by Zweibaum
/// horizontal part of the TCHX or NDHX, 
/// has the same loss correlations as the CTAH (horizontal)
///
pub fn new_mesh_refined_ndhx_tchx_vertical_35b(initial_temperature: ThermodynamicTemperature) -> 
NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(1.19e-2);
    let pipe_length = Length::new::<meter>(0.415925);
    let flow_area = Area::new::<square_meter>(1.33E-03);
    let incline_angle = Angle::new::<degree>(-90.0);
    let form_loss = Ratio::new::<ratio>(5.8);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.000406);
    let od = id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 4 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 4-2
    let user_specified_inner_nodes_sam = 4-2; 
    let mesh_refined_inner_nodes = 
        2 * (user_specified_inner_nodes_sam + 2) - 2;

    let mut non_insulated_component = NonInsulatedFluidComponent::new_bare_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        id, 
        od, 
        pipe_length, 
        hydraulic_diameter, 
        surface_roughness, 
        pipe_shell_material, 
        pipe_fluid, 
        htc_to_ambient, 
        mesh_refined_inner_nodes);

    // for heat exchangers, I give an ideal Nusselt number correlation 
    // as an approximation so that film thermal resistance is minimised
    let mut fluid_array_ideal_nusslet: FluidArray = 
        non_insulated_component.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_ideal_nusslet.nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    non_insulated_component.pipe_fluid_array = 
        fluid_array_ideal_nusslet.into();

    non_insulated_component
}

/// cold leg of DRACS (or what I consider the cold branch)
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
/// note that for coupled natural circulation dracs loop calibration 
/// tchx pipe 35b is evenly split into 35b-1 and 35b-2
/// 35b-1 is adiabatic towards the environment
///
/// label 35b-1 on SAM model by Zweibaum
/// horizontal part of the TCHX or NDHX, 
/// has the same loss correlations as the CTAH (horizontal)
///
pub fn _new_mesh_refined_ndhx_tchx_vertical_35b_1(initial_temperature: ThermodynamicTemperature) -> 
NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(1.19e-2);
    // even splitting here
    let pipe_length = Length::new::<meter>(0.415925*0.5);
    let flow_area = Area::new::<square_meter>(1.33E-03);
    let incline_angle = Angle::new::<degree>(-90.0);
    let form_loss = Ratio::new::<ratio>(5.8);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.000406);
    let od = id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    // tchx 35b1 is adiabatic
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(0.0);
    // from SAM nodalisation, we have 4 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 4-2
    let user_specified_inner_nodes_sam = 4-2; 
    let mesh_refined_inner_nodes = 
        2 * (user_specified_inner_nodes_sam + 2) - 2;

    let mut non_insulated_component = NonInsulatedFluidComponent::new_bare_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        id, 
        od, 
        pipe_length, 
        hydraulic_diameter, 
        surface_roughness, 
        pipe_shell_material, 
        pipe_fluid, 
        htc_to_ambient, 
        mesh_refined_inner_nodes);

    // for heat exchangers, I give an ideal Nusselt number correlation 
    // as an approximation so that film thermal resistance is minimised
    let mut fluid_array_ideal_nusslet: FluidArray = 
        non_insulated_component.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_ideal_nusslet.nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    non_insulated_component.pipe_fluid_array = 
        fluid_array_ideal_nusslet.into();

    non_insulated_component
}
/// cold leg of DRACS (or what I consider the cold branch)
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
/// note that for coupled natural circulation dracs loop calibration 
/// tchx pipe 35b is evenly split into 35b-1 and 35b-2
/// 35b-2 is not adiabatic towards the environment
///
/// label 35b-2 on SAM model by Zweibaum
/// horizontal part of the TCHX or NDHX, 
/// has the same loss correlations as the CTAH (horizontal)
///
pub fn _new_mesh_refined_ndhx_tchx_vertical_35b_2(initial_temperature: ThermodynamicTemperature) -> 
NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(1.19e-2);
    // even splitting here
    let pipe_length = Length::new::<meter>(0.415925*0.5);
    let flow_area = Area::new::<square_meter>(1.33E-03);
    let incline_angle = Angle::new::<degree>(-90.0);
    let form_loss = Ratio::new::<ratio>(5.8);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.000406);
    let od = id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    // tchx 35b1 is non adiabatic
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 4 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 4-2
    let user_specified_inner_nodes_sam = 4-2; 
    let mesh_refined_inner_nodes = 
        2 * (user_specified_inner_nodes_sam + 2) - 2;

    let mut non_insulated_component = NonInsulatedFluidComponent::new_bare_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        id, 
        od, 
        pipe_length, 
        hydraulic_diameter, 
        surface_roughness, 
        pipe_shell_material, 
        pipe_fluid, 
        htc_to_ambient, 
        mesh_refined_inner_nodes);

    // for heat exchangers, I give an ideal Nusselt number correlation 
    // as an approximation so that film thermal resistance is minimised
    let mut fluid_array_ideal_nusslet: FluidArray = 
        non_insulated_component.pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_ideal_nusslet.nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    non_insulated_component.pipe_fluid_array = 
        fluid_array_ideal_nusslet.into();

    non_insulated_component
}



/// cold leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// static mixer pipe 36a, the static mixer pipe next to the NDHX a.k.a TCHX
///
pub fn new_mesh_refined_pipe_36a(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.2034);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(-58.99728);
    let form_loss = Ratio::new::<ratio>(3.75);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    let user_specified_inner_nodes_sam = 0; 
    let mesh_refined_inner_nodes = 
        2 * (user_specified_inner_nodes_sam + 2) - 2;

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
        mesh_refined_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// cold leg of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// static mixer MX-60 label 36
///
pub fn new_mesh_refined_static_mixer_60_label_36(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(0.33);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(-58.99728);
    let form_loss = Ratio::new::<ratio>(21.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(4000.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    let user_specified_inner_nodes_sam = 0; 
    let mesh_refined_inner_nodes = 
        2 * (user_specified_inner_nodes_sam + 2) - 2;

    let insulated_component = InsulatedFluidComponent::new_custom_component(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        reynolds_coefficient, 
        reynolds_power, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        component_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        mesh_refined_inner_nodes);

    insulated_component
}


/// cold leg (or branch) of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// pipe 37, a pipe next to MX-60
///
pub fn new_mesh_refined_pipe_37(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(1.7736);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(-90.0);
    // pipe 37 has no form losses in Zweibaum's dissertation (probably 
    // a misprint) but it shows up as 14.0 on Zou's paper
    let form_loss = Ratio::new::<ratio>(14.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 16 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 16-2
    let user_specified_inner_nodes_sam = 16-2; 
    let mesh_refined_inner_nodes = 
        2 * (user_specified_inner_nodes_sam + 2) - 2;

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
        mesh_refined_inner_nodes, 
        surface_roughness);

    insulated_component
}


/// cold leg (or branch) of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// static flowmeter label 37a
///
pub fn new_mesh_refined_flowmeter_60_37a(initial_temperature: ThermodynamicTemperature) -> 
NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(0.36);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(-90.0);
    let form_loss = Ratio::new::<ratio>(18.1);
    let reynolds_power = -1.3476_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(93006.9_f64);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, we subtract 2
    let user_specified_inner_nodes_sam = 2-2; 
    let mesh_refined_inner_nodes = 
        2 * (user_specified_inner_nodes_sam + 2) - 2;


    let non_insulated_component = NonInsulatedFluidComponent::
        new_custom_component(
            initial_temperature, 
            ambient_temperature, 
            fluid_pressure, 
            solid_pressure, 
            flow_area, 
            incline_angle, 
            form_loss, 
            reynolds_coefficient, 
            reynolds_power, 
            shell_id, 
            shell_od, 
            component_length, 
            hydraulic_diameter, 
            pipe_shell_material, 
            pipe_fluid, 
            htc_to_ambient, 
            mesh_refined_inner_nodes);

    non_insulated_component

}


/// cold leg (or branch) of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// pipe 38 
///
pub fn new_mesh_refined_pipe_38(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.33655);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(-52.41533);
    let form_loss = Ratio::new::<ratio>(0.8);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 3 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 3-2
    let user_specified_inner_nodes_sam = 3-2; 
    let mesh_refined_inner_nodes = 
        2 * (user_specified_inner_nodes_sam + 2) - 2;

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
        mesh_refined_inner_nodes, 
        surface_roughness);

    insulated_component
}


/// cold leg (or branch) of DRACS
///
/// note that we will rotate these components by 180 degrees
/// for only the hot leg, as the DRACS loop in RELAP is programmed 
/// in a counter clockwise fashion (see Nico Zweibaum's thesis)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
/// pipe 39, bottom of cold leg
///
pub fn new_mesh_refined_pipe_39(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(1.91135);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(-80.64882);
    let form_loss = Ratio::new::<ratio>(2.65);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 18 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 18-2
    let user_specified_inner_nodes_sam = 18-2; 
    let mesh_refined_inner_nodes = 
        2 * (user_specified_inner_nodes_sam + 2) - 2;

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
        mesh_refined_inner_nodes, 
        surface_roughness);

    insulated_component
}
