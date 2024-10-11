use ndarray::*;
use ndarray_linalg::error::LinalgError;
use uom::num_traits::Zero;
use uom::si::f64::*;
use uom::si::power::watt;

use crate::boussinesq_thermophysical_properties::specific_enthalpy::try_get_h;
use crate::tuas_lib_error::TuasLibError;

use super::solve_conductance_matrix_power_vector;
use crate::single_control_vol::SingleCVNode;

/// for high peclet number flows, we can advance timestep without 
/// considering axial conduction within the fluid
///
/// this fluid node array will be connected to two adjacent arrays,
/// they can be boundaries or pipes
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
/// ----------------fluid solid boundary with thermal resistance ---------
///
/// [solid]
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
///
pub fn advance_timestep_fluid_shell_array_high_peclet_number(
    back_single_cv: &mut SingleCVNode,
    front_single_cv: &mut SingleCVNode,
    number_of_nodes: usize,
    dt: Time,
    total_volume: Volume,
    q: Power,
    last_timestep_temperature_inner_side: &mut Array1<ThermodynamicTemperature>,
    inner_side_fluid_conductance_array: &mut Array1<ThermalConductance>,
    last_timestep_temperature_outer_side: &mut Array1<ThermodynamicTemperature>,
    outer_side_fluid_conductance_array: &mut Array1<ThermalConductance>,
    last_timestep_temperature_fluid: &mut Array1<ThermodynamicTemperature>,
    mass_flowrate: MassRate,
    volume_fraction_array: &mut Array1<f64>,
    rho_cp: &mut Array1<VolumetricHeatCapacity>,
    q_fraction: &mut Array1<f64>)
-> Result<Array1<ThermodynamicTemperature>,TuasLibError>{

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
    // there will always be at least 2 nodes

    if number_of_nodes <= 1 {
        return Err(TuasLibError::LinalgError(LinalgError::Shape(
            ShapeError::from_kind(
                ErrorKind::OutOfBounds
            ))));
    } else if number_of_nodes > 1 {

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
            // m c_p dT/dt = 
            // -H_{inner} (T - T_inner) 
            // -H_{outer} (T - T_outer) 
            // - m_flow h_fluid(T) 
            // + m_flow h_fluid(adjacent T) 
            // + q
            //
            // of all these terms, only the m cp dT/dt term,
            // H_{inner} T  and 
            // H_{outer} T
            //
            // contain the fluid temperature at next timestep, T
            // 
            // We separate this out to get:
            //
            // m cp T / dt + H_{inner}T + H_{outer}T = 
            //
            // H_{inner} T_inner 
            // + H_{outer} T_outer
            // - m_flow h_fluid(T_old) 
            // + m_flow h_fluid(adjacent T_old) + m cp / dt (Told)
            // + q
            //
            // belong in the M matrix, the rest belong in S
            coefficient_matrix[[0,0]] = volume_fraction_array[0] * rho_cp[0] 
            * total_volume / dt + inner_side_fluid_conductance_array[0]
            + outer_side_fluid_conductance_array[0];

            // the first part of the source term deals with 
            // the flow direction independent terms

            let h_fluid_last_timestep: AvailableEnergy = 
            back_single_cv.current_timestep_control_volume_specific_enthalpy;

            // now this makes the scheme semi implicit, and we should then 
            // treat the scheme as explicit

            power_source_vector[0] = inner_side_fluid_conductance_array[0] *
                last_timestep_temperature_inner_side[0] 
                + outer_side_fluid_conductance_array[0] * 
                last_timestep_temperature_outer_side[0]
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
                    back_single_cv.pressure_control_volume).unwrap();

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
                    * total_volume / dt + inner_side_fluid_conductance_array[i] 
                    + outer_side_fluid_conductance_array[i];

                // we also consider outflow using previous timestep 
                // temperature, 
                // assume back cv and front cv material are the same

                let h_fluid_last_timestep: AvailableEnergy = 
                try_get_h(
                    back_single_cv.material_control_volume,
                    last_timestep_temperature_fluid[i],
                    back_single_cv.pressure_control_volume).unwrap();

                // basically, all the power terms remain 
                power_source_vector[i] = inner_side_fluid_conductance_array[i] *
                    last_timestep_temperature_inner_side[i] 
                    + outer_side_fluid_conductance_array[i] * 
                    last_timestep_temperature_outer_side[i]
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
                        back_single_cv.pressure_control_volume).unwrap();

                    
                    power_source_vector[i] += 
                    h_fluid_adjacent_node * mass_flowrate.abs();

                } else {

                    // enthalpy must be based on cv at i+1
                    let h_fluid_adjacent_node: AvailableEnergy = 
                    try_get_h(
                        back_single_cv.material_control_volume,
                        last_timestep_temperature_fluid[i+1],
                        back_single_cv.pressure_control_volume).unwrap();

                    
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
            * total_volume / dt + inner_side_fluid_conductance_array[i]
            + outer_side_fluid_conductance_array[i];
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

            power_source_vector[i] = inner_side_fluid_conductance_array[i] *
                last_timestep_temperature_inner_side[i] 
                + outer_side_fluid_conductance_array[i] * 
                last_timestep_temperature_outer_side[i]
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
                    back_single_cv.pressure_control_volume).unwrap();

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
        back_single_cv.pressure_control_volume).unwrap();

    back_single_cv.current_timestep_control_volume_specific_enthalpy 
    = back_node_enthalpy_next_timestep;

    let front_node_enthalpy_next_timestep: AvailableEnergy = 
    try_get_h(
        front_single_cv.material_control_volume,
        temperature_vector[number_of_nodes-1],
        front_single_cv.pressure_control_volume).unwrap();

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
    back_single_cv.set_liquid_cv_mass_from_temperature().unwrap();
    back_single_cv.rate_enthalpy_change_vector.clear();
    back_single_cv.max_timestep_vector.clear();

    front_single_cv.set_liquid_cv_mass_from_temperature().unwrap();
    front_single_cv.rate_enthalpy_change_vector.clear();
    front_single_cv.max_timestep_vector.clear();

    return Ok(temperature_vector);

}
