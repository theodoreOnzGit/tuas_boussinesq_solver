use std::ops::{DerefMut, Deref};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;

use csv::Writer;

use crate::prelude::beta_testing::*;




use uom::si::angle::radian;
use uom::si::angular_velocity::radian_per_second;
use uom::si::area::square_meter;
use uom::si::length::{centimeter, meter};
use uom::si::mass_rate::kilogram_per_second;
use uom::si::power::{watt, kilowatt};
use uom::si::ratio::ratio;
use uom::si::pressure::atmosphere;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::time::second;
use uom::si::f64::*;



/// Creates a CIET heater v2.0 steady state test with 8 axial nodes and 
/// two metallic shell nodes in the radial direction using FluidArray and 
/// SolidColumn objects wrapped in HeatTransferEntity Enums
///
/// This test checks the impact of 
/// using the matrix solvers on calculation speed in comparison to using 
/// only SingleCV objects,
///
/// However, it also doubles as an example for which to construct 
/// a component using HeatTransferEntity enums

/// In this test, we have a nodalised representation of the 
/// this is 8 nodes in the axial direction and 2 nodes for metal 
/// in the radial direction
///
/// This is for heater v2.0
///
/// I realised that we needed a major speedup because in debug mode 
/// a 2.3ms timestep was taking ~ 602 ms to calculate
///
/// This was unacceptable, especially when we need about 30 plus 
/// components for in forced convection loop in CTAH plus heater branch
///
/// Hence there is a need not only to speed up calculations, but also 
/// to quickly initialise nodalised control volumes. For example, 
/// the heater has 8 nodes
///
/// For this test, we shall design a heater with 8 nodes axially 
/// one fluid node 
/// and one steel node
/// 
/// (ambient air)
///
/// | 
/// | 
/// | 
/// 
/// (steel shell) ---- (heater power)
///
/// | 
/// |  
/// | 
///
/// (fluid node)  --- (subsequent nodes, advection)
///
/// each of these nodes will be represented by matrices
///
/// 
///
#[test]
//#[ignore = "data collected"]
pub fn ciet_heater_v2_speedup_using_fluid_array_solid_column_hte(){


    use uom::si::length::inch;
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;

    use ndarray::*;
    // okay, let's make two control volumes 
    // one cylinder and then the other a shell
    //
    // cylinder needs diameter and z 
    // shell needs id, od and z
    let _therminol = Material::Liquid(LiquidMaterial::TherminolVP1);
    let steel = Material::Solid(SolidMaterial::SteelSS304L);
    let id = Length::new::<meter>(0.0381);
    let od = Length::new::<meter>(0.04);
    let _inner_tube_od = Length::new::<centimeter>(3.175);
    // z is heated length
    let _total_length = Length::new::<meter>(1.983333);
    let heated_length = Length::new::<meter>(1.676);
    let initial_temperature = ThermodynamicTemperature::new::
        <degree_celsius>(80.0);
    let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(0.01467);

    let flow_area = Area::new::<square_meter>(0.00105);
    let number_of_nodes: usize = 8;
    let ambient_air_temp = ThermodynamicTemperature::new::<
        degree_celsius>(21.67);

    let heater_steady_state_power = Power::new::<kilowatt>(8.0);
    let therminol_mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);
    let node_length: Length = heated_length/number_of_nodes as f64;
    let inlet_temperature = ThermodynamicTemperature::new::<degree_celsius>
        (79.12);

    // the form loss of the pipe is meant to help construct the inner 
    // pipe object, that is to help it return the nusselt number 
    // that is expected for this pipe
    // 
    // However, one is free to specify whatever nusselt number works
    let dummy_pipe_form_loss = Ratio::new::<ratio>(0.1);

    // heater is inclined 90 degrees upwards, not that this is 
    // particularly important for this scenario
    let pipe_incline_angle = Angle::new::<uom::si::angle::degree>(90.0);

    // now, to construct the heater
    //
    // I'll construct the inner fluid tube first 

    let therminol_array: HeatTransferEntity = 
    FluidArray::new_odd_shaped_pipe(
        heated_length,
        hydraulic_diameter,
        flow_area,
        initial_temperature,
        atmospheric_pressure,
        SolidMaterial::SteelSS304L,
        LiquidMaterial::TherminolVP1,
        dummy_pipe_form_loss,
        6,
        pipe_incline_angle
    ).into();

    // now the outer steel array

    let steel_shell_array: HeatTransferEntity = 
    SolidColumn::new_cylindrical_shell(
        heated_length,
        id,
        od,
        initial_temperature,
        atmospheric_pressure,
        SolidMaterial::SteelSS304L,
        6
    ).into();

    // move arrays to Arc ptrs 

    let fluid_array_ptr = Arc::new(Mutex::new(
        therminol_array
    ));

    let solid_array_ptr = Arc::new(Mutex::new( 
        steel_shell_array 
    ));

    // clone ptrs for moving into loop 
    
    let fluid_array_ptr_for_loop = fluid_array_ptr.clone();
    let solid_array_ptr_for_loop = solid_array_ptr.clone();

    // time settings
    let max_time: Time = Time::new::<second>(75.0);
    let max_time_ptr = Arc::new(max_time);

    let calculation_time_elapsed = SystemTime::now();
    let calculation_loop = move || {
        // timestep settings

        let mut current_time_simulation_time = Time::new::<second>(0.0);
        let timestep = Time::new::<second>(0.01);


        //csv writer
        //
        let mut wtr = Writer::from_path("array_cv_test_ciet_heater_v_2_0_steady_state.csv")
            .unwrap();

        wtr.write_record(&["time_seconds",
            "heater_power_kilowatts",
            "therminol_temperature_celsius",
            "shell_temperature_celsius",
            "timestep_seconds",])
            .unwrap();

        let mut time_wtr = Writer::from_path("array_cv_test_ciet_heater_v_2_0_calc_time_profile.csv")
            .unwrap();

        time_wtr.write_record(&["loop_calculation_time_nanoseconds",
            "mutex_lock_time_ns",
            "node_connection_time_ns",
            "data_record_time_ns",
            "timestep_advance_time_ns",])
            .unwrap();

        let mut temp_profile_wtr = Writer::from_path(
            "array_cv_test_ciet_heater_v_2_0_temp_profile.csv")
            .unwrap();

        // this is code for writing the array of required temperatures
        {

            // I want the mid node length of this temperature


            let half_node_length: Length = 0.5 * node_length;

            let mut header_vec: Vec<String> = vec![];

            header_vec.push("simulation_time_seconds".to_string());
            header_vec.push("elapsed_time_seconds".to_string());

            for index in 0..number_of_nodes {

                let mid_node_length: Length = 
                index as f64 * node_length + half_node_length;

                let prefix: String = "heater_temp_celsius_at_".to_string();

                let suffix: String = "_cm".to_string();

                let mid_node_length_cm: f64 = 
                mid_node_length.get::<centimeter>();

                let mid_node_length_string: String = 
                mid_node_length_cm.to_string();

                let header: String = prefix + &mid_node_length_string + &suffix;

                header_vec.push(header);


            }

            temp_profile_wtr.write_record(&header_vec).unwrap();

        }

        // main calculation loop

        while current_time_simulation_time <= *max_time_ptr.deref(){

            let arc_mutex_lock_start = SystemTime::now();
            // obtain arc mutex locks 


            let mut therminol_entity_ptr_in_loop = 
            fluid_array_ptr_for_loop.lock().unwrap();

            let mut steel_entity_ptr_in_loop 
            = solid_array_ptr_for_loop.lock().unwrap();
            
            let arc_mutex_lock_elapsed_ns = 
            arc_mutex_lock_start.elapsed().unwrap().as_nanos();

            // next, obtain average temperature 
            // average by volume for both fluid and solid vec
            // so we can get an average conductance value
            //
            let node_connection_start = SystemTime::now();

            // now I'm going to clone the heat transfer entity 
            // as a fluid array and solid column respectively to 
            // get the underlying bulk temp

            let mut therminol_array_clone: FluidArray = 
            therminol_entity_ptr_in_loop.deref_mut().clone().try_into().unwrap();
            let mut steel_array_clone: SolidColumn = 
            steel_entity_ptr_in_loop.deref_mut().clone().try_into().unwrap();

            let fluid_average_temp: ThermodynamicTemperature = 
            therminol_array_clone.try_get_bulk_temperature().unwrap();

            let solid_average_temp: ThermodynamicTemperature = 
            steel_array_clone.try_get_bulk_temperature().unwrap();

            // given these two, we can calculate an average conductance 
            // value across all solid-fluid boundaries. 
            //
            // This is a time saving measure, rather than calculating 
            // all the conductances node by node

            // now, we make a cylindrical conductance interaction with 
            // fluid inside

            // we'll need the thickness 

            let radial_thickness_thermal_conduction = 0.5*(od - id);
            let radial_thickness_thermal_conduction: 
            RadialCylindricalThicknessThermalConduction = 
            radial_thickness_thermal_conduction.into();

            let inner_diameter_thermal_conduction: InnerDiameterThermalConduction 
            = id.into();


            // need to change this the nusselt correlation for the actual 
            // test
            // heater 2.0
            let h_to_therminol: HeatTransfer = 
            _heat_transfer_coefficient_ciet_v_2_0(
                therminol_mass_flowrate,
                fluid_average_temp,
                atmospheric_pressure);

            // looks like h_to_therminol is the issue
            // severely underestimated in new ciet heater

            let therminol_steel_conductance_interaction: HeatTransferInteractionType
            = HeatTransferInteractionType::
                CylindricalConductionConvectionLiquidInside(
                    (steel, 
                    radial_thickness_thermal_conduction,
                    solid_average_temp,
                    atmospheric_pressure),
                    (h_to_therminol,
                    inner_diameter_thermal_conduction,
                    node_length.clone().into())
            );

            // now based on conductance interaction, 
            // we can obtain thermal conductance, the temperatures 
            // and pressures don't really matter
            //
            // this is because all the thermal conductance data 
            // has already been loaded into the thermal conductance 
            // interaction object

            let therminol_steel_nodal_thermal_conductance: ThermalConductance = 
            therminol_steel_conductance_interaction.get_thermal_conductance_based_on_interaction(
                fluid_average_temp,
                solid_average_temp,
                atmospheric_pressure,
                atmospheric_pressure,
            ).unwrap();


            // now we need conductance for steel and air, the procedure 
            // is quite similar
            let radial_thickness_thermal_conduction = 0.5*(od - id);
            let radial_thickness_thermal_conduction: 
            RadialCylindricalThicknessThermalConduction = 
            radial_thickness_thermal_conduction.into();

            let outer_diameter_thermal_conduction: OuterDiameterThermalConduction
            = od.into();

            let h_to_air: HeatTransfer = 
            HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);


            let steel_air_conductance_interaction: HeatTransferInteractionType
            = HeatTransferInteractionType::
                CylindricalConductionConvectionLiquidOutside(
                    (steel, 
                    radial_thickness_thermal_conduction,
                    solid_average_temp,
                    atmospheric_pressure),
                    (h_to_air,
                    outer_diameter_thermal_conduction,
                    node_length.clone().into())
            );

            let steel_air_nodal_thermal_conductance: ThermalConductance = 
            steel_air_conductance_interaction.get_thermal_conductance_based_on_interaction(
                fluid_average_temp,
                solid_average_temp,
                atmospheric_pressure,
                atmospheric_pressure,
            ).unwrap();

            // we need a heat source also, and for that, a suitable 
            // heat source distribution within the tube (assumed uniform)
            
            let q_fraction_per_node: f64 = 1.0/ number_of_nodes as f64;
            let mut q_frac_arr: Array1<f64> = Array::default(number_of_nodes);
            q_frac_arr.fill(q_fraction_per_node);




            // lateral connections 
            {
                // first i will need to create temperature vectors 

                let mut ambient_temperature_vector: Vec<ThermodynamicTemperature> 
                = Array1::default(number_of_nodes)
                    .iter().map( |&temp| {
                        temp
                    }
                    ).collect();

                ambient_temperature_vector.fill(ambient_air_temp);

                let solid_temp_vector: Vec<ThermodynamicTemperature> 
                = steel_array_clone.get_temperature_vector().unwrap();

                let fluid_temp_vector: Vec<ThermodynamicTemperature> 
                = therminol_array_clone.get_temperature_vector().unwrap();


                // second, fill them into the each array 

                steel_array_clone.lateral_link_new_temperature_vector_avg_conductance(
                    steel_air_nodal_thermal_conductance,
                    ambient_temperature_vector
                ).unwrap();

                steel_array_clone.lateral_link_new_temperature_vector_avg_conductance(
                    therminol_steel_nodal_thermal_conductance,
                    fluid_temp_vector
                ).unwrap();
                // we also want to add a heat source
                
                steel_array_clone.lateral_link_new_power_vector(
                    heater_steady_state_power,
                    q_frac_arr
                ).unwrap();

                therminol_array_clone.lateral_link_new_temperature_vector_avg_conductance(
                    therminol_steel_nodal_thermal_conductance,
                    solid_temp_vector
                ).unwrap();

                // note, must set mass flowrate first 
                // otherwise there is by default zero flow through 
                // the array

                therminol_array_clone.set_mass_flowrate(
                    therminol_mass_flowrate);



                // now that lateral connections are done, 
                // modify the heat transfer entity 
                therminol_entity_ptr_in_loop.deref_mut()
                    .set(therminol_array_clone.clone().into()).unwrap();

                steel_entity_ptr_in_loop.deref_mut()
                    .set(steel_array_clone.clone().into()).unwrap();

                
                

            }

            

            // axial connection 
            {

                // now I'm going to make adiabatic bcs and ambient temp 
                // bcs 
                let mut adiabatic_bc: HeatTransferEntity 
                = BCType::new_adiabatic_bc().into();
                let mut inlet_temperature_bc: HeatTransferEntity 
                = BCType::new_const_temperature(
                    inlet_temperature).into();

                // the axial interactions are constant power heat addition
                // and then advection

                let constant_heat_addition: HeatTransferInteractionType 
                = HeatTransferInteractionType::UserSpecifiedHeatAddition;

                let inlet_advection_interaction: HeatTransferInteractionType 
                = DataAdvection::new_from_heat_transfer_entity(
                    therminol_mass_flowrate,
                    LiquidMaterial::TherminolVP1.into(),
                    &mut inlet_temperature_bc,
                    &mut therminol_array_clone.back_single_cv.clone().into()
                ).into();

                let outlet_advection_interaction: HeatTransferInteractionType
                = DataAdvection::new_from_temperature_and_liquid_material(
                    therminol_mass_flowrate,
                    LiquidMaterial::TherminolVP1.into(),
                    therminol_array_clone.front_single_cv.get_temperature_from_enthalpy_and_set().unwrap(),
                    therminol_array_clone.front_single_cv.get_temperature_from_enthalpy_and_set().unwrap(),
                ).into();
                
                therminol_entity_ptr_in_loop.deref_mut().link_to_back(
                    &mut inlet_temperature_bc,
                    inlet_advection_interaction).unwrap();

                therminol_entity_ptr_in_loop.deref_mut().link_to_front(
                    &mut adiabatic_bc,
                    outlet_advection_interaction).unwrap();

                steel_entity_ptr_in_loop.deref_mut().link_to_front(
                    &mut adiabatic_bc,
                    constant_heat_addition).unwrap();

                steel_entity_ptr_in_loop.deref_mut().link_to_back(
                    &mut adiabatic_bc,
                    constant_heat_addition).unwrap();
                
            }

            // end loop timer
            
            let node_connection_end_ns = 
            node_connection_start.elapsed().unwrap().as_nanos();

            // auto calculate timestep (not done)
            //
            // and also writing temperature profile 
            let data_recording_time_start = SystemTime::now();

            // record current fluid temperature profile
            {
                let mut temp_profile_data_vec: Vec<String> = vec![];

                let current_time_string = 
                current_time_simulation_time.get::<second>().to_string();

                // next simulation time string 
                let elapsed_calc_time_seconds_string = 
                calculation_time_elapsed.elapsed().unwrap().as_secs().to_string();
                temp_profile_data_vec.push(current_time_string);
                temp_profile_data_vec.push(elapsed_calc_time_seconds_string);

                let steel_column: SolidColumn = 
                steel_entity_ptr_in_loop.deref_mut().clone().try_into().unwrap();

                let steel_temperature_array = 
                steel_column.get_temperature_vector().unwrap();



                for node_temp in steel_temperature_array.iter() {

                    let node_temp_deg_c: f64 = 
                    node_temp.get::<degree_celsius>();

                    let node_temp_c_string: String = 
                    node_temp_deg_c.to_string();

                    temp_profile_data_vec.push(node_temp_c_string);
                }

                temp_profile_wtr.write_record(&temp_profile_data_vec).unwrap();
            }

            // outlet temperature profile 
            {
                // csv data writing

                let mut therminol_fluid_arr: FluidArray = 
                therminol_entity_ptr_in_loop.deref_mut().clone().
                    try_into().unwrap();


                let therminol_outlet_temp:ThermodynamicTemperature = 
                therminol_fluid_arr.front_single_cv.get_temperature_from_enthalpy_and_set().unwrap();

                let therminol_outlet_temp_string = 
                therminol_outlet_temp.get::<degree_celsius>().to_string();

                let current_time_string = 
                current_time_simulation_time.get::<second>().to_string();

                let heater_power_kilowatt_string = 
                heater_steady_state_power.get::<kilowatt>().to_string();

                // for st 11 

                // code block for recording inlet, shell and outlet 
                // temperatures
                //
                // now for shell temperatures, we are going to assume that 
                // ST-11 is used. 
                //
                // ST-11 is the thermocouple measuring surface temperature 
                // roughly 19 inches from the bottom of the heater 
                // The entire heated length excluding heater top and 
                // bottom heads is about 64 inches 
                //
                // So 19/64 is about 0.30 of the way through
                let st_11_length: Length = Length::new::<inch>(19_f64);


                // now I want to find out which node it is,
                // so i need the node length first 
                //
                
                let node_length: Length = heated_length/number_of_nodes as f64;

                // then use st_11 divide by node length 

                let st_11_rough_node_number: Ratio = st_11_length / node_length;

                // now, st_11 is about 19 inches, out of 64, and we have 
                // 8 equal nodes, each node is 
                // 12.5% of the heated length
                //
                // so this is about 30% of the way through
                //
                // so this is node three. 
                //
                // if we take st_11_length/node_length 
                // we would get about 2.375 for this ratio 
                //
                // we need to round up to get 3 
                // but the third node is the 2nd index in the matrix 
                // because the index starts from zero
                //
                //
                // so round it up and then minus 1 
                // most of the time, round down is ok, but rounding up 
                // makes more logical sense given this derivation

                let st_11_node_number: usize = 
                st_11_rough_node_number.get::<ratio>().ceil() as usize;

                let st_11_index_number: usize = st_11_node_number - 1;

                // now that we got the index number, we can get the 
                // outer surface temperature 
                let steel_column: SolidColumn = 
                steel_entity_ptr_in_loop.deref_mut().clone().try_into().unwrap();

                let steel_temperature_array = 
                steel_column.get_temperature_vector().unwrap();

                let st_11_node_temp: ThermodynamicTemperature = 
                steel_temperature_array.deref()[st_11_index_number];

                let shell_celsius_string = 
                st_11_node_temp.get::<degree_celsius>().to_string();
                
                // timestep in seconds
                //

                let timestep_string = 
                timestep.get::<second>().to_string();


                wtr.write_record(&[current_time_string,
                    heater_power_kilowatt_string,
                    therminol_outlet_temp_string,
                    shell_celsius_string,
                    timestep_string])
                    .unwrap();

            }
            let data_recording_time_end_ns = 
            data_recording_time_start.elapsed().unwrap().as_nanos();

            // advance timestep
            let timestep_advance_start = SystemTime::now();
            //
            {
                therminol_entity_ptr_in_loop.deref_mut().
                advance_timestep_mut_self(timestep).unwrap();

                steel_entity_ptr_in_loop.deref_mut().
                advance_timestep_mut_self(timestep).unwrap();

                current_time_simulation_time += timestep;
            }
            // end advance timestep
            let timestep_advance_end_ns = 
            timestep_advance_start.elapsed().unwrap().as_nanos();

            //time statistics 
            {
                let total_time_ns = 
                arc_mutex_lock_elapsed_ns 
                + node_connection_end_ns 
                + data_recording_time_end_ns 
                + timestep_advance_end_ns;

                time_wtr.write_record(&[total_time_ns.to_string(),
                    arc_mutex_lock_elapsed_ns.to_string(),
                    node_connection_end_ns.to_string(),
                    data_recording_time_end_ns.to_string(),
                    timestep_advance_end_ns.to_string()])
                    .unwrap();
                // timestep addition to current simulation time
                current_time_simulation_time += timestep;
            } // end time statistics
        }
        // end for loop 

        // with csvs being written,
        // use cargo watch -x test --ignore '*.csv'
        time_wtr.flush().unwrap();
        temp_profile_wtr.flush().unwrap();
        wtr.flush().unwrap();
    };

    let main_thread_handle = thread::spawn(calculation_loop);

    main_thread_handle.join().unwrap();



    return ();

}



fn _mfbs_power_signal_logspace_custom(simulation_time: Time) -> Power {

    // get simulation time in seconds 

    // obtain power, which is 9 kW plus 1kW amplitude 
    
    let steady_state_power = Power::new::<kilowatt>(9.0);
    let power_amplitude = Power::new::<kilowatt>(1.0);

    let mut power_vector: Vec<Power> = vec![];

    let angular_frequency_vector_values: Vec<f64> 
    = _logspace(0.001, 1_f64, 15);

    let mut angular_frequency_vector: Vec<AngularVelocity>
    = vec![];

    for angular_freq_val in angular_frequency_vector_values {

        angular_frequency_vector.push(
            AngularVelocity::new::<radian_per_second>(angular_freq_val));
    }


    for angular_frequency_ptr in angular_frequency_vector.iter() {
        let phase: Angle = (*angular_frequency_ptr * simulation_time).into();

        let phase_angle_radian_value: f64 = phase.get::<radian>();

        let phase_angle_sine: f64 = phase_angle_radian_value.sin();

        let power_component = steady_state_power + 
        power_amplitude * phase_angle_sine;

        power_vector.push(power_component);

    }

    // sum all power components 
    let mut power_analog_signal = Power::new::<watt>(0.0);

    for power_ptr in power_vector.iter() {
        // get average
        power_analog_signal += *power_ptr/power_vector.len() as f64;
    }
    
    // change to mfbs
    if power_analog_signal.get::<kilowatt>() < 9.0 {
        steady_state_power - power_amplitude
    } else {
        steady_state_power + power_amplitude
    }

}

/// generated by perplexity AI,
/// I still needed some mild correction
fn _logspace(start: f64, end: f64, n: usize) -> Vec<f64> {
    let base = 10.0_f64;
    let start_log = start.log10();
    let end_log = end.log10();
    let step = (end_log - start_log) / (n - 1) as f64;
    (0..n)
        .map(|i| base.powf(start_log + i as f64 * step))
        .collect()
}

// nusselt number correlation 
#[inline]
fn _ciet_heater_v_2_0_nusselt_number(reynolds:Ratio, 
    prandtl:Ratio) -> Ratio {

    let reynolds_power_0_836 = reynolds.value.powf(0.836);
    let prandtl_power_0_333 = prandtl.value.powf(0.333333333333333);

    Ratio::new::<ratio>(
    0.04179 * reynolds_power_0_836 * prandtl_power_0_333)

}
#[inline]
fn _ciet_heater_v_2_0_reynolds_nunber(mass_flowrate: MassRate,
    mu: DynamicViscosity) -> Ratio {

    // Re = m* D_H/ A_{XS}/mu
    let hydraulic_diameter = Length::new::<meter>(0.01467);
    let flow_area = Area::new::<square_meter>(0.00105);

    mass_flowrate*hydraulic_diameter/mu/flow_area
}

fn _heat_transfer_coefficient_ciet_v_2_0(mass_flowrate: MassRate,
    therminol_temperature: ThermodynamicTemperature,
    pressure: Pressure) -> HeatTransfer {

    // let's calculate mu and k 

    let therminol = Material::Liquid(LiquidMaterial::TherminolVP1);
    let mu: DynamicViscosity = try_get_mu_viscosity(therminol,
        therminol_temperature,
        pressure).unwrap();

    let k: ThermalConductivity = try_get_kappa_thermal_conductivity(
        therminol,
        therminol_temperature,
        pressure).unwrap();


    let reynolds: Ratio = _ciet_heater_v_2_0_reynolds_nunber(
        mass_flowrate, mu);

    let prandtl: Ratio = try_get_prandtl(
        therminol,
        therminol_temperature,
        pressure).unwrap();

    let nusselt: Ratio = _ciet_heater_v_2_0_nusselt_number(
        reynolds,
        prandtl);

    let hydraulic_diameter = Length::new::<meter>(0.01467);

    let heat_transfer_coeff: HeatTransfer = 
    nusselt * k / hydraulic_diameter;

    heat_transfer_coeff

}
