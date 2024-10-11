use ndarray::*;
use ndarray_linalg::error::LinalgError;
use uom::num_traits::Zero;
use uom::si::f64::*;
use uom::si::power::watt;

use crate::boussinesq_thermophysical_properties::specific_enthalpy::try_get_h;
use crate::thermal_hydraulics_error::TuasLibError;

use super::solve_conductance_matrix_power_vector;
use crate::single_control_vol::SingleCVNode;

/// for high peclet number flows, we can advance timestep without 
/// considering axial conduction
///
/// However, this fluid node array will be connected to a shell of some 
/// sort, like a metallic pipe 
///
/// There will be a conductance array which contains information of how 
/// much thermal conductance connects the inner nodes to the outer 
/// metallic pipe array
///
/// [solid]
/// T_back            T[0]           T[1]          T[n-1]         T_front 
///
/// ----------------fluid solid boundary with thermal resistance ---------
///
/// [fluid]
/// T_back            T[0]           T[1]          T[n-1]         T_front 
///
/// At the back and front node, there will be a single_cv which is able 
/// to link up to other cvs
///
/// You can also add heat volumetrically to the fluid as if it were 
/// generating heat, but you can also set it to zero
///
/// This function is standalone, and is not really used inside the 
/// array control volumes, but you are free to use it
pub fn advance_timestep_fluid_node_array_pipe_high_peclet_number(
    back_single_cv: &mut SingleCVNode,
    front_single_cv: &mut SingleCVNode,
    number_of_nodes: usize,
    dt: Time,
    total_volume: Volume,
    q: Power,
    last_timestep_temperature_solid: &mut Array1<ThermodynamicTemperature>,
    solid_fluid_conductance_array: &mut Array1<ThermalConductance>,
    last_timestep_temperature_fluid: &mut Array1<ThermodynamicTemperature>,
    mass_flowrate: MassRate,
    volume_fraction_array: &mut Array1<f64>,
    rho_cp: &mut Array1<VolumetricHeatCapacity>,
    q_fraction: &mut Array1<f64>)
-> Result<Array1<ThermodynamicTemperature>,TuasLibError>{
    // there will always be at least 2 nodes

    if number_of_nodes <= 1 {
        return Err(
            TuasLibError::LinalgError(
            LinalgError::Shape(
            ShapeError::from_kind(
                ErrorKind::OutOfBounds
            ))));
    }
    // First things first, we need to set up 
    // how the CV interacts with the internal array
    // here is heat added to CV

    let back_cv_rate_enthalpy_change_vector: Vec<Power> = 
    back_single_cv.rate_enthalpy_change_vector.clone();

    let front_cv_rate_enthalpy_change_vector: Vec<Power> = 
    front_single_cv.rate_enthalpy_change_vector.clone();

    // compute power source for back node

    let mut total_enthalpy_rate_change_back_node = 
    Power::new::<watt>(0.0);

    for enthalpy_chg_rate in 
        back_cv_rate_enthalpy_change_vector.iter() {

            total_enthalpy_rate_change_back_node += *enthalpy_chg_rate;
        }

    // then the front node,

    let mut total_enthalpy_rate_change_front_node = 
    Power::new::<watt>(0.0);

    for enthalpy_chg_rate in 
        front_cv_rate_enthalpy_change_vector.iter() {

            total_enthalpy_rate_change_front_node += *enthalpy_chg_rate;
        }

    // this front and back nodes will be an extra term added to the 
    // heat source vector S
    //
    // The old fluid temperature will need to be used to calculate 
    // new specific enthalpy for the system

    let mut temperature_vector: Array1<ThermodynamicTemperature> = 
    last_timestep_temperature_fluid.map(
        |temp_kelvin_ptr: &ThermodynamicTemperature| {

            return *temp_kelvin_ptr;
        }

    );

    // now let's start calculation 
    //
    if number_of_nodes > 1 {

        let mut coefficient_matrix: Array2<ThermalConductance> = 
        Array::zeros((number_of_nodes, number_of_nodes));

        let mut power_source_vector: 
        Array1<Power> = Array::zeros(number_of_nodes);

        // ascertain if we have forward flow 

        let forward_flow: bool = mass_flowrate.ge(&MassRate::zero());

        // back node calculation (first node)
        {
            // for the first node, also called the back node
            // energy balance is: 
            // m c_p dT/dt = -H (T - T_solid) - m_flow h_fluid(T) 
            // + m_flow h_fluid(adjacent T) + q
            // of all these terms, only the m cp dT/dt term and HT 
            // 
            // We separate this out to get:
            //
            // m cp T / dt + HT = 
            //
            // HT_solid - m_flow h_fluid(T_old) 
            // + m_flow h_fluid(adjacent T_old) + m cp / dt (Told)
            // + q
            //
            // belong in the M matrix, the rest belong in S
            coefficient_matrix[[0,0]] = volume_fraction_array[0] * rho_cp[0] 
            * total_volume / dt + solid_fluid_conductance_array[0];

            // the first part of the source term deals with 
            // the flow direction independent terms

            let h_fluid_last_timestep: AvailableEnergy = 
            back_single_cv.current_timestep_control_volume_specific_enthalpy;

            // now this makes the scheme semi implicit, and we should then 
            // treat the scheme as explicit

            power_source_vector[0] = solid_fluid_conductance_array[0] *
                last_timestep_temperature_solid[0] 
                - mass_flowrate * h_fluid_last_timestep 
                + last_timestep_temperature_fluid[0] * total_volume * 
                volume_fraction_array[0] * rho_cp[0] / dt 
                + q * q_fraction[0] 
                + total_enthalpy_rate_change_back_node ;

            // the next part deals with the inflow
            // m_flow h_fluid(adjacent T_old)
            //
            // now if the advection interaction is done correctly, 
            //
            // (advection) ----- (back cv) --------> fwd
            //
            // then in a frontal flow condition, the enthalpy flows in 
            // would already have been accounted for
            //
            // but in the case of backflow, then fluid from the node
            // in front will flow into this fluid node 
            // that is node 1 

            // so if mass flowrate is <= 0 , then we will calculate 
            // backflow conditions

            if !forward_flow {
                // first, get enthalpy of the node in front 

                let enthalpy_of_adjacent_node_to_the_front: AvailableEnergy = 
                try_get_h(
                    back_single_cv.material_control_volume,
                    last_timestep_temperature_fluid[1],
                    back_single_cv.pressure_control_volume)?;

                // now if mass flowrate is less than zero, then 
                // we receive enthalpy from the front cv 
                //
                // But we need to subtract the negative mass flow if 
                // that makes sense, or at least make it absolute
                // - (-m) * h = m * h
                //

                power_source_vector[0] += 
                mass_flowrate.abs() * enthalpy_of_adjacent_node_to_the_front;

                // additionally, in backflow situations, the mass 
                // flow out of this cv is already accounted for 
                // so don't double count 

                power_source_vector[0] += 
                mass_flowrate * h_fluid_last_timestep;



            }
        }

        // bulk node calculations 
        if number_of_nodes > 2 {
            // loop over all nodes from 1 to n-2 (n-1 is not included)
            for i in 1..number_of_nodes-1 {
                // the coefficient matrix is pretty much the same, 
                // we only consider solid fluid conduction, and the 
                // thermal inertia terms

                coefficient_matrix[[i,i]] = volume_fraction_array[i] * rho_cp[i] 
                    * total_volume / dt + solid_fluid_conductance_array[i];

                // we also consider outflow using previous timestep 
                // temperature, 
                // assume back cv and front cv material are the same

                let h_fluid_last_timestep: AvailableEnergy = 
                try_get_h(
                    back_single_cv.material_control_volume,
                    last_timestep_temperature_fluid[i],
                    back_single_cv.pressure_control_volume)?;

                // basically, all the power terms remain 
                power_source_vector[i] = solid_fluid_conductance_array[i] *
                    last_timestep_temperature_solid[i] 
                    - mass_flowrate.abs() * h_fluid_last_timestep 
                    + last_timestep_temperature_fluid[i] * total_volume * 
                    volume_fraction_array[i] * rho_cp[i] / dt 
                    + q * q_fraction[i];

                // account for enthalpy inflow

                if forward_flow {

                    // enthalpy must be based on the the cv at i-1

                    let h_fluid_adjacent_node: AvailableEnergy = 
                    try_get_h(
                        back_single_cv.material_control_volume,
                        last_timestep_temperature_fluid[i-1],
                        back_single_cv.pressure_control_volume)?;

                    
                    power_source_vector[i] += 
                    h_fluid_adjacent_node * mass_flowrate.abs();

                } else {

                    // enthalpy must be based on cv at i+1
                    let h_fluid_adjacent_node: AvailableEnergy = 
                    try_get_h(
                        back_single_cv.material_control_volume,
                        last_timestep_temperature_fluid[i+1],
                        back_single_cv.pressure_control_volume)?;

                    
                    power_source_vector[i] += 
                    h_fluid_adjacent_node * mass_flowrate.abs();

                }

            }
        }

        // front node (last node) calculation 
        {
            // we still follow the same guideline
            // m cp T / dt + HT = 
            //
            // HT_solid - m_flow h_fluid(T_old) 
            // + m_flow h_fluid(adjacent T_old) + m cp / dt (Told)
            // + q
            let i = number_of_nodes-1;

            coefficient_matrix[[i,i]] = volume_fraction_array[i] * rho_cp[i] 
            * total_volume / dt + solid_fluid_conductance_array[i];
            // the first part of the source term deals with 
            // the flow direction independent terms

            // now this makes the scheme semi implicit, and we should then 
            // treat the scheme as explicit
            //
            // now I should subtract the enthalpy outflow from the 
            // power source vector here, 
            // but if done correctly, it should already be accounted 
            // for in total_enthalpy_rate_change_front_node
            //
            // so i shouldn't double count

            power_source_vector[i] = solid_fluid_conductance_array[i] *
                last_timestep_temperature_solid[i] 
                //- mass_flowrate * h_fluid_last_timestep 
                + last_timestep_temperature_fluid[i] * total_volume * 
                volume_fraction_array[i] * rho_cp[i] / dt 
                + q * q_fraction[i] 
                + total_enthalpy_rate_change_front_node ;

            // now advection causing heat transfer between the frontal 
            // node and some other single cv has already been accounted 
            // for in total_enthalpy_rate_change_front_node 
            // If flow is forward facing though, then enthalpy will 
            // be coming in from the i-1 th node

            if forward_flow {
                // first, get enthalpy of the adjacent node to the 
                // back

                let enthalpy_of_adjacent_node_to_the_rear: AvailableEnergy = 
                try_get_h(
                    back_single_cv.material_control_volume,
                    last_timestep_temperature_fluid[i-1],
                    back_single_cv.pressure_control_volume)?;

                // now if mass flowrate is less than zero, then 
                // we receive enthalpy from the front cv 
                //
                // But we need to subtract the negative mass flow if 
                // that makes sense, or at least make it absolute
                // - (-m) * h = m * h
                //

                power_source_vector[i] += 
                mass_flowrate.abs() * enthalpy_of_adjacent_node_to_the_rear;


            } else {
                // if there's backflow, 
                // the front cv (last node) will receive enthalpy from 
                // outside based on the enthalpy rate change vector in the 
                // front node, 
                // however it must also lose enthalpy, this is no 
                // longer accounted for in the 
                let h_fluid_last_timestep: AvailableEnergy = 
                front_single_cv.current_timestep_control_volume_specific_enthalpy;

                power_source_vector[i] -= 
                mass_flowrate.abs() * h_fluid_last_timestep;
            }
        }

        // solve for new temperature 

        temperature_vector = 
            solve_conductance_matrix_power_vector(
                coefficient_matrix,power_source_vector)?;
    } 
    // update the single cvs at the front and back with new enthalpies 

    // Todo: probably need to synchronise error types in future
    let back_node_enthalpy_next_timestep: AvailableEnergy = 
    try_get_h(
        back_single_cv.material_control_volume,
        temperature_vector[0],
        back_single_cv.pressure_control_volume)?;

    back_single_cv.current_timestep_control_volume_specific_enthalpy 
    = back_node_enthalpy_next_timestep;

    let front_node_enthalpy_next_timestep: AvailableEnergy = 
    try_get_h(
        front_single_cv.material_control_volume,
        temperature_vector[number_of_nodes-1],
        front_single_cv.pressure_control_volume)?;

    front_single_cv.current_timestep_control_volume_specific_enthalpy 
    = front_node_enthalpy_next_timestep;

    // let's also update the previous temperature vector 

    *last_timestep_temperature_fluid = temperature_vector.mapv(
        |temperature_value| {
            return temperature_value;
        }
    );

    // set liquid cv mass 
    // probably also need to update error types in future
    back_single_cv.set_liquid_cv_mass_from_temperature()?;
    back_single_cv.rate_enthalpy_change_vector.clear();
    back_single_cv.max_timestep_vector.clear();

    front_single_cv.set_liquid_cv_mass_from_temperature()?;
    front_single_cv.rate_enthalpy_change_vector.clear();
    front_single_cv.max_timestep_vector.clear();

    return Ok(temperature_vector);

}

#[test]
pub fn fluid_node_calculation_initial_test(){

    use std::{sync::{Arc, Mutex}, time::SystemTime, ops::{Deref, DerefMut}, thread};

    use csv::Writer;
    use ndarray::*;
    use uom::si::f64::*;
    use uom::si::power::kilowatt;
    use uom::si::length::meter;
    use uom::si::length::centimeter;
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::time::second;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::area::square_meter;
    use uom::si::pressure::atmosphere;
    use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};

    use crate::boussinesq_thermophysical_properties::Material;
    use crate::boussinesq_thermophysical_properties::SolidMaterial;
    use crate::boussinesq_thermophysical_properties::LiquidMaterial;
    use crate::boussinesq_thermophysical_properties::volumetric_heat_capacity::try_get_rho_cp;
    use crate::control_volume_dimensions::RadialCylindricalThicknessThermalConduction;
    use crate::control_volume_dimensions::InnerDiameterThermalConduction;
    use crate::heat_transfer_correlations::heat_transfer_interactions::
        heat_transfer_interaction_enums::*;


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
    let total_volume = flow_area * heated_length;


    let mut initial_temperature_array: Array1<ThermodynamicTemperature> = 
    Array::default(number_of_nodes);
    initial_temperature_array.fill(initial_temperature);

    let fluid_temperature_array = initial_temperature_array.clone();

    let mut steel_temperature_array: Array1<ThermodynamicTemperature> = 
    Array::default(number_of_nodes);

    steel_temperature_array.fill(ambient_air_temp);




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
    let steel_temp_at_present_timestep_ptr = Arc::new(Mutex::new(
        steel_temperature_array
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

    // clone the ptrs for the loop
    let back_cv_ptr_clone_for_loop = back_cv_ptr.clone();
    let front_cv_ptr_clone_for_loop = front_cv_ptr.clone();
    let fluid_temp_vec_ptr_for_loop = fluid_temp_at_present_timestep_ptr.clone();
    let steel_temp_at_present_timestep_ptr_for_loop = 
    steel_temp_at_present_timestep_ptr.clone();
    let q_fraction_ptr_for_loop = q_fraction_ptr.clone();
    let fluid_vol_fraction_ptr_for_loop = fluid_vol_fraction_ptr.clone();
    let solid_vol_fraction_ptr_for_loop = solid_vol_fraction_ptr.clone();

    // time settings
    let max_time: Time = Time::new::<second>(10.0);
    let max_time_ptr = Arc::new(max_time);

    let calculation_time_elapsed = SystemTime::now();
    let calculation_loop = move || {
        // timestep

        let mut current_time_simulation_time = Time::new::<second>(0.0);
        let timestep = Time::new::<second>(0.01);

        // csv writer 

        let mut time_wtr = Writer::from_path("fluid_node_calc_time_profile.csv")
            .unwrap();

        time_wtr.write_record(&["loop_calculation_time_nanoseconds",
            "mutex_lock_time_ns",
            "node_connection_time_ns",
            "data_record_time_ns",
            "timestep_advance_time_ns",])
            .unwrap();

        let mut temp_profile_wtr = Writer::from_path(
            "fluid_node_temp_profile.csv")
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
            let mut steel_temp_at_present_timestep_ptr_in_loop
            = steel_temp_at_present_timestep_ptr_for_loop.lock().unwrap();
            let mut q_fraction_ptr_in_loop 
            = q_fraction_ptr_for_loop.lock().unwrap();
            let mut fluid_vol_fraction_ptr_in_loop 
            = fluid_vol_fraction_ptr_for_loop.lock().unwrap();
            let solid_vol_fraction_ptr_in_loop 
            = solid_vol_fraction_ptr_for_loop.lock().unwrap();

            
            // time for arc mutex derefs
            // this code causes race conditions
            //
            // found through debugging,
            //
            // just use the elapsed function 
            // cos the arc mutex lock end time is practically zero
            // it will cause some error
            //let arc_mutex_lock_end = SystemTime::now();

            //let arc_mutex_lock_elapsed_ns = 
            //arc_mutex_lock_start.elapsed().unwrap().as_nanos()
            //- arc_mutex_lock_end.elapsed().unwrap().as_nanos();

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
                steel_temp_at_present_timestep_ptr_in_loop.iter().enumerate() {

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


            // i'm using a dummy heat transfer coefficient here
            let h_to_therminol_dummy: HeatTransfer = 
            HeatTransfer::new::<watt_per_square_meter_kelvin>(35.0);


            let conductance_interaction: HeatTransferInteractionType
            = HeatTransferInteractionType::
                CylindricalConductionConvectionLiquidInside(
                    (steel, 
                    radial_thickness_thermal_conduction,
                    steel_avg_temp,
                    atmospheric_pressure),
                    (h_to_therminol_dummy,
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

            let nodal_thermal_conductance: ThermalConductance = 
            conductance_interaction.get_thermal_conductance_based_on_interaction(
                fluid_avg_temp,
                steel_avg_temp,
                atmospheric_pressure,
                atmospheric_pressure,
            ).unwrap();
            
            //let nodal_thermal_conductance = ThermalConductance::new::<
            //    uom::si::thermal_conductance::watt_per_kelvin>(20.0);

            // now, create conductance vector 

            let mut conductance_vector: Array1<ThermalConductance> = 
            Array::zeros(number_of_nodes);

            conductance_vector.fill(nodal_thermal_conductance);

            // next is rho_cp vector for the fluid 
            // it has to be calculated based on each node temperature 
            // and should not be like the conductance bit
            //
            // hopefully it isn't too computationally expensive

            let mut fluid_rho_cp_array: Array1<VolumetricHeatCapacity> 
            = Array::zeros(number_of_nodes);

            for (index,fluid_temp) in fluid_temp_vec_ptr_in_loop.iter().enumerate() {

                let fluid_nodal_rho_cp: VolumetricHeatCapacity = try_get_rho_cp(
                    steel,
                    *fluid_temp,
                    atmospheric_pressure
                ).unwrap();

                fluid_rho_cp_array[index] = fluid_nodal_rho_cp;

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

                for node_temp in fluid_temp_vec_ptr_in_loop.deref_mut().iter() {

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

            let timestep_advance_start = 
            SystemTime::now();
            let _new_temperature_vec = 
            advance_timestep_fluid_node_array_pipe_high_peclet_number(
                back_cv_ptr_in_loop.deref_mut(),
                front_cv_ptr_in_loop.deref_mut(),
                number_of_nodes,
                timestep,
                total_volume,
                heater_steady_state_power,
                steel_temp_at_present_timestep_ptr_in_loop.deref_mut(),
                &mut conductance_vector,
                fluid_temp_vec_ptr_in_loop.deref_mut(),
                therminol_mass_flowrate,
                fluid_vol_fraction_ptr_in_loop.deref_mut(),
                &mut fluid_rho_cp_array,
                q_fraction_ptr_in_loop.deref_mut(),
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

#[test]
fn fluid_node_backflow_calculation_initial_test(){

    use std::{sync::{Arc, Mutex}, time::SystemTime, ops::{Deref, DerefMut}, thread};

    use csv::Writer;
    use ndarray::*;
    use uom::si::f64::*;
    use uom::si::power::kilowatt;
    use uom::si::length::meter;
    use uom::si::length::centimeter;
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::time::second;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::area::square_meter;
    use uom::si::pressure::atmosphere;
    use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};

    use crate::boussinesq_thermophysical_properties::Material;
    use crate::boussinesq_thermophysical_properties::SolidMaterial;
    use crate::boussinesq_thermophysical_properties::LiquidMaterial;
    use crate::boussinesq_thermophysical_properties::volumetric_heat_capacity::try_get_rho_cp;
    use crate::control_volume_dimensions::RadialCylindricalThicknessThermalConduction;
    use crate::control_volume_dimensions::InnerDiameterThermalConduction;
    use crate::heat_transfer_correlations::heat_transfer_interactions::
        heat_transfer_interaction_enums::*;


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
    let therminol_mass_flowrate = 
    MassRate::new::<kilogram_per_second>(-0.18);

    let inlet_temperature = ThermodynamicTemperature::new::<degree_celsius>
        (79.12);
    // now, to construct the heater
    // solid fluid interactions, we use a row of 
    // temperature vectors, a front CV and a back CV to 
    // The front and back CV should be identical 
    //
    // the 
    let node_length: Length = heated_length/number_of_nodes as f64;

    let fluid_back_cv: SingleCVNode = 
    SingleCVNode::new_odd_shaped_pipe(
        node_length,
        flow_area,
        therminol,
        initial_temperature,
        atmospheric_pressure,
    ).unwrap();

    // get the cv inside, 
    // real ugly match statement, but whatever.

    let fluid_front_cv: SingleCVNode = fluid_back_cv.clone();
    let total_volume = flow_area * heated_length;


    let mut initial_temperature_array: Array1<ThermodynamicTemperature> = 
    Array::default(number_of_nodes);
    initial_temperature_array.fill(initial_temperature);

    let fluid_temperature_array = initial_temperature_array.clone();

    let mut steel_temperature_array: Array1<ThermodynamicTemperature> = 
    Array::default(number_of_nodes);

    steel_temperature_array.fill(ambient_air_temp);




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
    let steel_temp_at_present_timestep_ptr = Arc::new(Mutex::new(
        steel_temperature_array
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

    // clone the ptrs for the loop
    let back_cv_ptr_clone_for_loop = back_cv_ptr.clone();
    let front_cv_ptr_clone_for_loop = front_cv_ptr.clone();
    let fluid_temp_vec_ptr_for_loop = fluid_temp_at_present_timestep_ptr.clone();
    let steel_temp_at_present_timestep_ptr_for_loop = 
    steel_temp_at_present_timestep_ptr.clone();
    let q_fraction_ptr_for_loop = q_fraction_ptr.clone();
    let fluid_vol_fraction_ptr_for_loop = fluid_vol_fraction_ptr.clone();
    let solid_vol_fraction_ptr_for_loop = solid_vol_fraction_ptr.clone();

    // time settings
    let max_time: Time = Time::new::<second>(10.0);
    let max_time_ptr = Arc::new(max_time);

    let calculation_time_elapsed = SystemTime::now();
    let calculation_loop = move || {
        // derference ptrs 

        let mut current_time_simulation_time = Time::new::<second>(0.0);
        let timestep = Time::new::<second>(0.01);

        // csv writer 

        let mut time_wtr = Writer::from_path("fluid_node_backflow_calc_time_profile.csv")
            .unwrap();

        time_wtr.write_record(&["loop_calculation_time_nanoseconds",
            "mutex_lock_time_ns",
            "node_connection_time_ns",
            "data_record_time_ns",
            "timestep_advance_time_ns",])
            .unwrap();

        let mut temp_profile_wtr = Writer::from_path(
            "fluid_node_backflow_temp_profile.csv")
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
            let mut steel_temp_at_present_timestep_ptr_in_loop
            = steel_temp_at_present_timestep_ptr_for_loop.lock().unwrap();
            let mut q_fraction_ptr_in_loop 
            = q_fraction_ptr_for_loop.lock().unwrap();
            let mut fluid_vol_fraction_ptr_in_loop 
            = fluid_vol_fraction_ptr_for_loop.lock().unwrap();
            let solid_vol_fraction_ptr_in_loop 
            = solid_vol_fraction_ptr_for_loop.lock().unwrap();

            
            // time for arc mutex derefs
            // deprecate this, has been causing race conditions
            //
            // let arc_mutex_lock_end = SystemTime::now();

            //let arc_mutex_lock_elapsed_ns = 
            //arc_mutex_lock_start.elapsed().unwrap().as_nanos()
            //- arc_mutex_lock_end.elapsed().unwrap().as_nanos();
            
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
                steel_temp_at_present_timestep_ptr_in_loop.iter().enumerate() {

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

            let h: HeatTransfer = 
            HeatTransfer::new::<watt_per_square_meter_kelvin>(35.0);

            let conductance_interaction: HeatTransferInteractionType
            = HeatTransferInteractionType::
                CylindricalConductionConvectionLiquidInside(
                    (steel, 
                    radial_thickness_thermal_conduction,
                    steel_avg_temp,
                    atmospheric_pressure),
                    (h,
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

            let nodal_thermal_conductance: ThermalConductance = 
            conductance_interaction.get_thermal_conductance_based_on_interaction(
                fluid_avg_temp,
                steel_avg_temp,
                atmospheric_pressure,
                atmospheric_pressure,
            ).unwrap();

            // now, create conductance vector 

            let mut conductance_vector: Array1<ThermalConductance> = 
            Array::zeros(number_of_nodes);

            conductance_vector.fill(nodal_thermal_conductance);

            // next is rho_cp vector for the fluid 
            // it has to be calculated based on each node temperature 
            // and should not be like the conductance bit
            //
            // hopefully it isn't too computationally expensive

            let mut fluid_rho_cp_array: Array1<VolumetricHeatCapacity> 
            = Array::zeros(number_of_nodes);

            for (index,fluid_temp) in fluid_temp_vec_ptr_in_loop.iter().enumerate() {

                let fluid_nodal_rho_cp: VolumetricHeatCapacity = try_get_rho_cp(
                    steel,
                    *fluid_temp,
                    atmospheric_pressure
                ).unwrap();

                fluid_rho_cp_array[index] = fluid_nodal_rho_cp;

            }

            // now I'm going to manually make enthalpy inflows and 
            // outflows 
            //
            // in backflow situation, we have flow going into the 
            // front cv 
            // and 
            // flow leaving the back cv

            let enthalpy_inflow_from_bc: Power 
            = therminol_mass_flowrate.abs() * try_get_h(
                therminol,
                inlet_temperature,
                atmospheric_pressure,
            ).unwrap();

            front_cv_ptr_in_loop.deref_mut().rate_enthalpy_change_vector
            .push(enthalpy_inflow_from_bc);

            // flow leaves the back cv, carrying away enthalpy

            let enthalpy_outflow_to_bc: Power 
            = therminol_mass_flowrate.abs() * 
            back_cv_ptr_in_loop.deref().
            current_timestep_control_volume_specific_enthalpy;

            // add these to the power vectors inside each cv 

            back_cv_ptr_in_loop.deref_mut().rate_enthalpy_change_vector
            .push(-enthalpy_outflow_to_bc);


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

                for node_temp in fluid_temp_vec_ptr_in_loop.deref_mut().iter() {

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

            let timestep_advance_start = 
            SystemTime::now();
            let _new_temperature_vec = 
            advance_timestep_fluid_node_array_pipe_high_peclet_number(
                back_cv_ptr_in_loop.deref_mut(),
                front_cv_ptr_in_loop.deref_mut(),
                number_of_nodes,
                timestep,
                total_volume,
                heater_steady_state_power,
                steel_temp_at_present_timestep_ptr_in_loop.deref_mut(),
                &mut conductance_vector,
                fluid_temp_vec_ptr_in_loop.deref_mut(),
                therminol_mass_flowrate,
                fluid_vol_fraction_ptr_in_loop.deref_mut(),
                &mut fluid_rho_cp_array,
                q_fraction_ptr_in_loop.deref_mut(),
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
