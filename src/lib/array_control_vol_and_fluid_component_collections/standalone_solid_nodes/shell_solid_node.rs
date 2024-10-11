use ndarray::*;
use ndarray_linalg::error::LinalgError;

use uom::si::f64::*;



use crate::thermal_hydraulics_error::TuasLibError;


use crate::array_control_vol_and_fluid_component_collections::standalone_fluid_nodes::solve_conductance_matrix_power_vector;


/// for most pipe flows, we can consider radial conduction without
/// considering axial conduction
///
/// There will be a conductance array which contains information of how 
/// much thermal conductance connects the inner nodes to the outer 
/// metallic pipe array
///
/// [outer side]
/// T_back[0]         T[1]           T[1]          T[n-1](T_front)
///
/// ----------------boundary with thermal resistance ---------
///
///
/// [solid]
/// T_back            T[0]           T[1]          T[n-1](T_front)
///
/// ----------------boundary with thermal resistance ---------
///
/// [inner side]
/// T_back[0]         T[1]           T[1]          T[n-1](T_front)
///
/// 
/// You can also add heat volumetrically to the solid as if it were 
/// generating heat, but you can also set it to zero
///
/// there is no axial conduction in this case, so the equations 
/// are set up very simply
///
///
pub fn advance_timestep_solid_cylindrical_shell_node_no_axial_conduction(
    number_of_nodes: usize,
    dt: Time,
    total_volume: Volume,
    q: Power,
    last_timestep_temperature_inner_side: &mut Array1<ThermodynamicTemperature>,
    solid_inner_conductance_array: &mut Array1<ThermalConductance>,
    last_timestep_temperature_outer_side: &mut Array1<ThermodynamicTemperature>,
    solid_outer_conductance_array: &mut Array1<ThermalConductance>,
    last_timestep_temperature_solid: &mut Array1<ThermodynamicTemperature>,
    volume_fraction_array: &mut Array1<f64>,
    rho_cp: &mut Array1<VolumetricHeatCapacity>,
    q_fraction: &mut Array1<f64>)
-> Result<Array1<ThermodynamicTemperature>,TuasLibError>{


    // this front and back nodes will be an extra term added to the 
    // heat source vector S
    //
    // The old fluid temperature will need to be used to calculate 
    // new specific enthalpy for the system

    let mut new_timestep_temperature_vector: Array1<ThermodynamicTemperature> = 
    last_timestep_temperature_solid.map(
        |temp_kelvin_ptr: &ThermodynamicTemperature| {

            return *temp_kelvin_ptr;
        }

    );

    // now let's start calculation 
    //
    // there will always be at least 2 nodes

    if number_of_nodes <= 1 {
        return Err(
            TuasLibError::LinalgError(
            LinalgError::Shape(
            ShapeError::from_kind(
                ErrorKind::OutOfBounds
            ))));
    } else if number_of_nodes > 1 {

        let mut coefficient_matrix: Array2<ThermalConductance> = 
        Array::zeros((number_of_nodes, number_of_nodes));

        let mut power_source_vector: 
        Array1<Power> = Array::zeros(number_of_nodes);


        // back node calculation (first node)
        {
            // for the first node, also called the back node
            // energy balance for each node is: 
            //
            // m dh/dt = -H_{inner} (T_solid - T_inner_side) -H_{outer}
            // (T_solid - T_outer_side)
            //
            // if we can afford not to use enthalpy mapping, it may be 
            // faster,
            //
            // We assign T_solid as T_new
            //
            //
            // m cp(T_old) 
            // (T_new-T_old)/dt = -H_{inner} (T_new - T_inner_side) -H_{outer}
            // (T_new - T_outer_side) + q
            //
            // cp by default is based on T_old
            //
            // m cp/dt T_new + H_{inner} T_new + H_{outer} T_new 
            // =  m cp/dt T_old + H_inner T_inner + H_outer T_outer + q
            //
            //
            //  coefficient matrix is the coefficient of the LHS
            coefficient_matrix[[0,0]] = volume_fraction_array[0] * rho_cp[0] 
            * total_volume / dt + solid_inner_conductance_array[0]
            + solid_outer_conductance_array[0];

            // power source matrix is the RHS
            power_source_vector[0] = 
                last_timestep_temperature_solid[0] * total_volume * 
                volume_fraction_array[0] * rho_cp[0] / dt 
                + solid_inner_conductance_array[0] *
                last_timestep_temperature_inner_side[0] 
                + solid_outer_conductance_array[0] *
                last_timestep_temperature_outer_side[0]
                + q * q_fraction[0];

        }

        // bulk node calculations 
        if number_of_nodes > 2 {
            // loop over all nodes from 1 to n-2 (n-1 is not included)
            for i in 1..number_of_nodes-1 {

                // repeat the same thing but with index i 
                // should be quite straightforward without axial 
                // conduction
                coefficient_matrix[[i,i]] = volume_fraction_array[i] * rho_cp[i] 
                    * total_volume / dt + solid_inner_conductance_array[i]
                    + solid_outer_conductance_array[i];

                power_source_vector[i] = 
                    last_timestep_temperature_solid[i] * total_volume * 
                    volume_fraction_array[i] * rho_cp[i] / dt 
                    + solid_inner_conductance_array[i] *
                    last_timestep_temperature_inner_side[i] 
                    + solid_outer_conductance_array[i] *
                    last_timestep_temperature_outer_side[i]
                    + q * q_fraction[i];
            }
        }

        // front node (last node) calculation 
        {
            // same for this as well
            let i = number_of_nodes-1;

            // repeat the same thing but with index i 
            // should be quite straightforward without axial 
            // conduction
            coefficient_matrix[[i,i]] = volume_fraction_array[i] * rho_cp[i] 
                * total_volume / dt + solid_inner_conductance_array[i]
                + solid_outer_conductance_array[i];

            power_source_vector[i] = 
                last_timestep_temperature_solid[i] * total_volume * 
                volume_fraction_array[i] * rho_cp[i] / dt 
                + solid_inner_conductance_array[i] *
                last_timestep_temperature_inner_side[i] 
                + solid_outer_conductance_array[i] *
                last_timestep_temperature_outer_side[i]
                + q * q_fraction[i];

        }

        // solve for new temperature 

        new_timestep_temperature_vector = 
            solve_conductance_matrix_power_vector(
                coefficient_matrix,power_source_vector)?;
    } 

    // let's also update the previous temperature vector 

    *last_timestep_temperature_solid = new_timestep_temperature_vector.mapv(
        |temperature_value| {
            return temperature_value;
        }
    );

    // that's it, unlike for fluid nodes, 
    // we don't need to do any kind of coupling with solid nodes, which 
    // makes things a lot easier


    return Ok(new_timestep_temperature_vector);

}

/// note: race conditions were seen for array type matrix calculations 
/// using ndarray (intel mkl libraries)
///
/// Usually this happens when run concurrently with other threads using 
/// intel mkl as well... 
///
/// Or maybe I was running two cargo watch tests simultaneously
///
/// either way, it's very rare
///
/// TODO: change test
#[test]
//#[ignore="not ready yet"]
fn fluid_solid_node_calculation_initial_test(){

    use std::{sync::{Arc, Mutex}, time::SystemTime, ops::{Deref, DerefMut}, thread, f64::consts::PI};

    use csv::Writer;
    use ndarray::*;
    
    use uom::si::f64::*;
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::time::second;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::area::square_meter;
    use uom::si::pressure::atmosphere;
    use uom::si::thermodynamic_temperature::kelvin;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::si::length::{meter, centimeter};
    use uom::si::power::kilowatt;
    use uom::si::power::watt;

    use crate::single_control_vol::SingleCVNode;
    use crate::boussinesq_thermophysical_properties::Material;
    use crate::boussinesq_thermophysical_properties::SolidMaterial;
    use crate::boussinesq_thermophysical_properties::LiquidMaterial;
    use crate::boussinesq_thermophysical_properties::volumetric_heat_capacity::try_get_rho_cp;
    use crate::control_volume_dimensions::*;
    use crate::heat_transfer_correlations::heat_transfer_interactions::
        heat_transfer_interaction_enums::*;
    use crate::array_control_vol_and_fluid_component_collections::standalone_fluid_nodes::
        core_fluid_node::advance_timestep_fluid_node_array_pipe_high_peclet_number;

    use crate::boussinesq_thermophysical_properties::specific_enthalpy::try_get_h;
    // okay, let's make two control volumes 
    // one cylinder and then the other a shell
    //
    // cylinder needs diameter and z 
    // shell needs id, od and z
    let therminol = Material::Liquid(LiquidMaterial::TherminolVP1);
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

    let flow_area = Area::new::<square_meter>(0.00105);
    let number_of_nodes: usize = 8;
    let ambient_air_temp = ThermodynamicTemperature::new::<
        degree_celsius>(21.67);

    let heater_steady_state_power = Power::new::<kilowatt>(8.0);
    let therminol_mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);

    let inlet_temperature = ThermodynamicTemperature::new::<degree_celsius>
        (79.12);
    // now, to construct the heater
    // solid fluid interactions, we use a row of 
    // temperature vectors, a front CV and a back CV to 
    // The front and back CV should be identical 
    //
    // the 
    let node_length: Length = heated_length/number_of_nodes as f64;

    let fluid_back_entity: SingleCVNode = 
    SingleCVNode::new_odd_shaped_pipe(
        node_length,
        flow_area,
        therminol,
        initial_temperature,
        atmospheric_pressure,
    ).unwrap();

    // get the cv inside, 
    // real ugly match statement, but whatever.

    let fluid_back_cv: SingleCVNode = fluid_back_entity;
    let fluid_front_cv: SingleCVNode = fluid_back_cv.clone();
    let total_fluid_volume = flow_area * heated_length;

    // total steel volume is also needed 

    let total_steel_volume = heated_length * 
    (PI * 0.25 * od * od 
    - PI * 0.25 * id * id);


    let mut initial_temperature_array: Array1<ThermodynamicTemperature> = 
    Array::default(number_of_nodes);
    initial_temperature_array.fill(initial_temperature);

    let fluid_temperature_array = initial_temperature_array.clone();

    let mut steel_temperature_array: Array1<ThermodynamicTemperature> = 
    Array::default(number_of_nodes);
    
    steel_temperature_array.fill(initial_temperature);

    let mut ambient_air_temperature_array: Array1<ThermodynamicTemperature> = 
    Array::default(number_of_nodes);

    ambient_air_temperature_array.fill(ambient_air_temp);




    // we'll need solid fluid conductance array, volume fraction array 
    // rho_cp array and q_fraction array 
    //
    // for conductance, what I'd like to do is 
    // take an average temperature for both solid and fluid 
    // and then based on that, obtain a conductance value, thus 
    // reducing the number of times we calculate with conductance
    //
    // however, the solid fluid conductance is periodically updated, 
    // so do it in a while loop, same for rhocp

    let mut q_fraction: Array1<f64> = Array::zeros(number_of_nodes);
    q_fraction.fill(1.0/number_of_nodes as f64);

    let vol_fraction_equal_split: f64 = 1.0/number_of_nodes as f64;
    let mut volume_fraction_array: Array1<f64> = Array::zeros(number_of_nodes);
    volume_fraction_array.fill(vol_fraction_equal_split);

    // move the fluid temp arrays into arc ptrs with mutex lock 
    // as well as the single cvs and such 

    let back_cv_ptr = Arc::new(Mutex::new(
        fluid_back_cv
    ));
    let front_cv_ptr = Arc::new(Mutex::new(
        fluid_front_cv
    ));
    let fluid_temp_at_present_timestep_ptr = Arc::new(Mutex::new(
        fluid_temperature_array
    ));
    let ambient_air_temp_at_present_timestep_ptr = Arc::new(Mutex::new(
        ambient_air_temperature_array
    ));
    let q_fraction_ptr = Arc::new(Mutex::new(
        q_fraction
    ));
    let fluid_vol_fraction_ptr = Arc::new(Mutex::new(
        volume_fraction_array.clone()
    ));
    let solid_vol_fraction_ptr = Arc::new(Mutex::new(
        volume_fraction_array
    ));
    let steel_temperature_array_present_timestep_ptr = Arc::new(Mutex::new(
        steel_temperature_array
    ));



    // clone the ptrs for the loop
    let back_cv_ptr_clone_for_loop = back_cv_ptr.clone();
    let front_cv_ptr_clone_for_loop = front_cv_ptr.clone();
    let fluid_temp_vec_ptr_for_loop = fluid_temp_at_present_timestep_ptr.clone();
    let ambient_air_temp_at_present_timestep_ptr_for_loop = 
    ambient_air_temp_at_present_timestep_ptr.clone();
    let q_fraction_ptr_for_loop = q_fraction_ptr.clone();
    let fluid_vol_fraction_ptr_for_loop = fluid_vol_fraction_ptr.clone();
    let solid_vol_fraction_ptr_for_loop = solid_vol_fraction_ptr.clone();
    let steel_temperature_array_present_timestep_ptr_for_loop = 
    steel_temperature_array_present_timestep_ptr.clone();

    // time settings
    let max_time: Time = Time::new::<second>(10.0);
    let max_time_ptr = Arc::new(max_time);

    let calculation_time_elapsed = SystemTime::now();
    let calculation_loop = move || {
        // derference ptrs 

        let mut current_time_simulation_time = Time::new::<second>(0.0);
        let timestep = Time::new::<second>(0.01);

        // csv writer 

        let mut time_wtr = Writer::from_path("cht_nodes_calc_time_profile.csv")
            .unwrap();

        time_wtr.write_record(&["loop_calculation_time_nanoseconds",
            "mutex_lock_time_ns",
            "node_connection_time_ns",
            "data_record_time_ns",
            "timestep_advance_time_ns",])
            .unwrap();

        let mut temp_profile_wtr = Writer::from_path(
            "cht_nodes_temp_profile.csv")
            .unwrap();

        // this is code for writing the array of required temperatures
        {

            // I want the mid node length of this temperature

            let node_length: Length = heated_length/number_of_nodes as f64;

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


        while current_time_simulation_time <= *max_time_ptr.deref(){

            let arc_mutex_lock_start = SystemTime::now();
            // obtain arc mutex locks 

            let mut back_cv_ptr_in_loop = 
            back_cv_ptr_clone_for_loop.lock().unwrap();
            let mut front_cv_ptr_in_loop = 
            front_cv_ptr_clone_for_loop.lock().unwrap();
            let mut fluid_temp_vec_ptr_in_loop
            = fluid_temp_vec_ptr_for_loop.lock().unwrap();
            let mut ambient_air_temp_at_present_timestep_ptr_in_loop
            = ambient_air_temp_at_present_timestep_ptr_for_loop.lock().unwrap();
            let mut q_fraction_ptr_in_loop 
            = q_fraction_ptr_for_loop.lock().unwrap();
            let mut fluid_vol_fraction_ptr_in_loop 
            = fluid_vol_fraction_ptr_for_loop.lock().unwrap();
            let mut solid_vol_fraction_ptr_in_loop 
            = solid_vol_fraction_ptr_for_loop.lock().unwrap();
            let mut steel_temperature_array_present_timestep_ptr_in_loop 
            = steel_temperature_array_present_timestep_ptr_for_loop.lock().unwrap();

            
            // time for arc mutex derefs

            let arc_mutex_lock_elapsed_ns = 
            arc_mutex_lock_start.elapsed().unwrap().as_nanos();
            
            // next, obtain average temperature 
            // average by volume for both fluid and solid vec
            //

            let node_connection_start = SystemTime::now();
            let mut fluid_temp_kelvin_times_vol_frac_average: 
            Vec<f64> = vec![];
            
            for (index,fluid_temp) in fluid_temp_vec_ptr_in_loop.iter().enumerate() {

                let fluid_temp_times_vol_frac = 
                fluid_vol_fraction_ptr_in_loop[index] *
                (*fluid_temp);

                fluid_temp_kelvin_times_vol_frac_average.push(
                    fluid_temp_times_vol_frac.get::<kelvin>());

            }

            // find average in kelvin, convert back to correct units 

            let fluid_avg_temp_kelvin: f64 = 
            fluid_temp_kelvin_times_vol_frac_average.iter().sum();

            let fluid_avg_temp = ThermodynamicTemperature::new::
                <kelvin>(fluid_avg_temp_kelvin);

            // we do the same for solid temp
            //
            //

            let mut steel_temp_kelvin_times_vol_frac_average: 
            Vec<f64> = vec![];
            
            for (index,steel_temp) in 
                steel_temperature_array_present_timestep_ptr_in_loop.iter().enumerate() {

                let steel_temp_times_vol_frac = 
                solid_vol_fraction_ptr_in_loop[index] *
                (*steel_temp);

                steel_temp_kelvin_times_vol_frac_average.push(
                    steel_temp_times_vol_frac.get::<kelvin>());

            }

            // find average in kelvin, convert back to correct units 

            let steel_avg_temp_kelvin: f64 = 
            steel_temp_kelvin_times_vol_frac_average.iter().sum();

            let steel_avg_temp = ThermodynamicTemperature::new::
                <kelvin>(steel_avg_temp_kelvin);

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
            let h_to_therminol: HeatTransfer = 
            HeatTransfer::new::<watt_per_square_meter_kelvin>(35.0);

            let therminol_steel_conductance_interaction: HeatTransferInteractionType
            = HeatTransferInteractionType::
                CylindricalConductionConvectionLiquidInside(
                    (steel, 
                    radial_thickness_thermal_conduction,
                    steel_avg_temp,
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
                fluid_avg_temp,
                steel_avg_temp,
                atmospheric_pressure,
                atmospheric_pressure,
            ).unwrap();

            // now, create conductance vector 

            let mut steel_therminol_conductance_vector: Array1<ThermalConductance> = 
            Array::zeros(number_of_nodes);

            steel_therminol_conductance_vector.fill(therminol_steel_nodal_thermal_conductance);

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
                    steel_avg_temp,
                    atmospheric_pressure),
                    (h_to_air,
                    outer_diameter_thermal_conduction,
                    node_length.clone().into())
            );

            let steel_air_nodal_thermal_conductance: ThermalConductance = 
            steel_air_conductance_interaction.get_thermal_conductance_based_on_interaction(
                fluid_avg_temp,
                steel_avg_temp,
                atmospheric_pressure,
                atmospheric_pressure,
            ).unwrap();

            // now, create conductance vector 

            let mut steel_air_conductance_vector: Array1<ThermalConductance> = 
            Array::zeros(number_of_nodes);

            steel_air_conductance_vector.fill(steel_air_nodal_thermal_conductance);


            // next is rho_cp vector for the fluid 
            // it has to be calculated based on each node temperature 
            // and should not be like the conductance bit
            //
            // hopefully it isn't too computationally expensive
            //

            let mut fluid_rho_cp_array: Array1<VolumetricHeatCapacity> 
            = Array::zeros(number_of_nodes);

            for (index,fluid_temp) in fluid_temp_vec_ptr_in_loop.iter().enumerate() {

                let fluid_nodal_rho_cp: VolumetricHeatCapacity = try_get_rho_cp(
                    therminol,
                    *fluid_temp,
                    atmospheric_pressure
                ).unwrap();

                fluid_rho_cp_array[index] = fluid_nodal_rho_cp;

            }

            // same for solid 
            let mut solid_rho_cp_array: Array1<VolumetricHeatCapacity> 
            = Array::zeros(number_of_nodes);

            for (index,solid_temp) in 
                steel_temperature_array_present_timestep_ptr_in_loop.iter().enumerate() {

                let solid_nodal_rho_cp: VolumetricHeatCapacity = try_get_rho_cp(
                    steel,
                    *solid_temp,
                    atmospheric_pressure
                ).unwrap();

                solid_rho_cp_array[index] = solid_nodal_rho_cp;

            }


            // now I'm going to manually make enthalpy inflows and 
            // outflows 

            let enthalpy_inflow_in_back_cv: Power 
            = therminol_mass_flowrate * try_get_h(
                therminol,
                inlet_temperature,
                atmospheric_pressure,
            ).unwrap();

            let enthalpy_outflow_in_front_cv: Power 
            = therminol_mass_flowrate * 
            front_cv_ptr_in_loop.deref().
            current_timestep_control_volume_specific_enthalpy;

            // add these to the power vectors inside each cv 

            back_cv_ptr_in_loop.deref_mut().rate_enthalpy_change_vector
            .push(enthalpy_inflow_in_back_cv);

            front_cv_ptr_in_loop.deref_mut().rate_enthalpy_change_vector
            .push(-enthalpy_outflow_in_front_cv);

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

                for node_temp in steel_temperature_array_present_timestep_ptr_in_loop.deref_mut().iter() {

                    let node_temp_deg_c: f64 = 
                    node_temp.get::<degree_celsius>();

                    let node_temp_c_string: String = 
                    node_temp_deg_c.to_string();

                    temp_profile_data_vec.push(node_temp_c_string);
                }

                temp_profile_wtr.write_record(&temp_profile_data_vec).unwrap();
            }


            let data_recording_time_end_ns = 
            data_recording_time_start.elapsed().unwrap().as_nanos();
            // now we are ready to start

            let mut fluid_temp_vec_last_timestep = 
            fluid_temp_vec_ptr_in_loop.deref().clone();

            let timestep_advance_start = 
            SystemTime::now();
            let _new_therminol_temperature_vec = 
            advance_timestep_fluid_node_array_pipe_high_peclet_number(
                back_cv_ptr_in_loop.deref_mut(),
                front_cv_ptr_in_loop.deref_mut(),
                number_of_nodes,
                timestep,
                total_fluid_volume,
                Power::new::<watt>(0.0),
                steel_temperature_array_present_timestep_ptr_in_loop.deref_mut(),
                &mut steel_therminol_conductance_vector,
                fluid_temp_vec_ptr_in_loop.deref_mut(),
                therminol_mass_flowrate,
                fluid_vol_fraction_ptr_in_loop.deref_mut(),
                &mut fluid_rho_cp_array,
                q_fraction_ptr_in_loop.deref_mut(),
            ).unwrap();

            let _new_steel_temperature_vec = 
            advance_timestep_solid_cylindrical_shell_node_no_axial_conduction(
                number_of_nodes,
                timestep,
                total_steel_volume,
                heater_steady_state_power,
                &mut fluid_temp_vec_last_timestep,
                &mut steel_therminol_conductance_vector,
                ambient_air_temp_at_present_timestep_ptr_in_loop.deref_mut(),
                &mut steel_air_conductance_vector,
                steel_temperature_array_present_timestep_ptr_in_loop.deref_mut(),
                solid_vol_fraction_ptr_in_loop.deref_mut(),
                &mut solid_rho_cp_array,
                &mut q_fraction_ptr_in_loop
            ).unwrap();

            
            let timestep_advance_end_ns = 
            timestep_advance_start.elapsed().unwrap().as_nanos();

            // write timestep diagnostics

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
        }
        // with csvs being written,
        // use cargo watch -x test --ignore '*.csv'
        time_wtr.flush().unwrap();
        temp_profile_wtr.flush().unwrap();
    };

    let main_thread_handle = thread::spawn(calculation_loop);

    main_thread_handle.join().unwrap();


    return ();

}

