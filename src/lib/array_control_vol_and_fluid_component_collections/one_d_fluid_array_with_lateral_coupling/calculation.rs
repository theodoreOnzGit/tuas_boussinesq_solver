use ndarray::*;
use ndarray_linalg::error::LinalgError;
use uom::num_traits::Zero;
use uom::si::f64::*;
use uom::si::power::watt;

use crate::array_control_vol_and_fluid_component_collections::standalone_fluid_nodes::solve_conductance_matrix_power_vector;
use crate::boussinesq_thermophysical_properties::prandtl::try_get_prandtl;
use crate::boussinesq_thermophysical_properties::specific_enthalpy::try_get_h;
use crate::boussinesq_thermophysical_properties::thermal_conductivity::try_get_kappa_thermal_conductivity;
use crate::boussinesq_thermophysical_properties::volumetric_heat_capacity::try_get_rho_cp;
use crate::thermal_hydraulics_error::TuasLibError;

use super::FluidArray;

/// This deals with the calculations of the fluid array 
/// at the end of the connection phase, one can then use 
/// the advance_timestep method to calculate the new 
/// temperature array
impl FluidArray{

    /// advance_timestep in the array, using the mass flowrate set 
    /// within the fluid array
    pub fn advance_timestep(&mut self,
        timestep: Time,) 
        -> Result<(), TuasLibError>{


        self.advance_timestep_with_mass_flowrate(
            timestep,
            self.mass_flowrate)
    }
    
    /// advances timestep for the fluid array 
    /// given a fixed timestep
    #[inline]
    pub fn advance_timestep_with_mass_flowrate(&mut self,
        timestep: Time,
        mass_flowrate: MassRate) 
        -> Result<(), TuasLibError>{
        // here's the function I copied from initially and modified
        //advance_timestep_fluid_node_array_pipe_high_peclet_number(
        //    back_cv_ptr_in_loop.deref_mut(),
        //    front_cv_ptr_in_loop.deref_mut(),
        //    number_of_nodes,
        //    timestep,
        //    total_volume,
        //    heater_steady_state_power,
        //    steel_temp_at_present_timestep_ptr_in_loop.deref_mut(),
        //    &mut conductance_vector,
        //    fluid_temp_vec_ptr_in_loop.deref_mut(),
        //    therminol_mass_flowrate,
        //    fluid_vol_fraction_ptr_in_loop.deref_mut(),
        //    &mut fluid_rho_cp_array,
        //    q_fraction_ptr_in_loop.deref_mut(),
        //)?;

        // there will always be at least 2 nodes

        let number_of_nodes = self.len();
        if number_of_nodes <= 1 {
            return Err(LinalgError::Shape(
                ShapeError::from_kind(
                    ErrorKind::OutOfBounds
                )).into());
        }

        // First things first, we need to set up 
        // how the CV interacts with the internal array
        // here is heat added to CV

        let back_cv_rate_enthalpy_change_vector: Vec<Power> = 
        self.back_single_cv.rate_enthalpy_change_vector.clone();

        let front_cv_rate_enthalpy_change_vector: Vec<Power> = 
        self.front_single_cv.rate_enthalpy_change_vector.clone();

        // compute power source for back node

        let mut total_enthalpy_rate_change_back_node = 
        Power::new::<watt>(0.0);

        for enthalpy_chg_rate in 
            back_cv_rate_enthalpy_change_vector.iter() {

                total_enthalpy_rate_change_back_node += *enthalpy_chg_rate;
            }

        // then the front node,
        //dbg!(&total_enthalpy_rate_change_back_node);

        let mut total_enthalpy_rate_change_front_node = 
        Power::new::<watt>(0.0);

        for enthalpy_chg_rate in 
            front_cv_rate_enthalpy_change_vector.iter() {

                total_enthalpy_rate_change_front_node += *enthalpy_chg_rate;
            }

        // this front and back nodes will be an extra term added to the 
        // heat source vector S
        //
        // We need a blank temperature array to start the iteration 
        // process, so we just copy the old temperature array over 
        // as the initial guess (i think?)
        //

        let new_temperature_array: Array1<ThermodynamicTemperature>;
        // now let's start calculation 
        //
        let mut coefficient_matrix: Array2<ThermalConductance> = 
        Array::zeros((number_of_nodes, number_of_nodes));

        let mut power_source_vector: 
        Array1<Power> = Array::zeros(number_of_nodes);

        // ascertain if we have forward flow 

        let forward_flow: bool = mass_flowrate.ge(&MassRate::zero());

        // obtain some important parameters for calculation
        let material = self.material_control_volume;
        let pressure = self.pressure_control_volume;
        let bulk_temperature = self.try_get_bulk_temperature()?;
        let total_volume = self.total_length *  self.xs_area;
        let dt = timestep;
        let node_length = self.total_length / number_of_nodes as f64;
        // for the fluid array, we do not keep track of node enthalpies,
        // instead, we keep track of temperatures and then calculate 
        // enthalpy changes using rho_cp calculated at the current 
        // timestep
        let volume_fraction_array: Array1<f64> = 
        self.volume_fraction_array.iter().map(
            |&vol_frac| {
                vol_frac
            }
        ).collect();
        // rho_cp is determined by the temperature array 
        let rho_cp: Array1<VolumetricHeatCapacity> = 
        self.temperature_array_current_timestep.iter().map(
            |&temperature| {
                try_get_rho_cp(material, temperature, pressure).unwrap()
            }
        ).collect();
        // energy balance is: 
        // m c_p dT/dt = -\sum H (T - T_lateral) - m_flow h_fluid(T) 
        // + m_flow h_fluid(adjacent T) + q
        // of all these terms, only the m cp dT/dt term and HT 
        // 
        // We separate this out to get:
        //
        // m cp T / dt + \sum HT = 
        //
        // \sum HT_lateral - m_flow h_fluid(T_old) 
        // + m_flow h_fluid(adjacent T_old) + m cp / dt (Told)
        // + q
        //
        // so we will need to determine sum H and sum HT_lateral
        // as sum H is the relevant coefficient in the coefficient_matrix

        let mut sum_of_lateral_conductances: Array1<ThermalConductance>
        = Array1::zeros(number_of_nodes);

        let mut sum_of_lateral_conductance_times_lateral_temperatures:
        Array1<Power> = Array1::zeros(number_of_nodes);

        // conductances will need to be summed over each node 
        //
        // i will also need to make sure that there are actually 
        // lateral connections to this array in the first place 
        // though!

        let lateral_temperature_arary_connected: bool 
        = self.lateral_adjacent_array_conductance_vector.len() > 0;

        if lateral_temperature_arary_connected {

            // the HT here comes from 
            //
            // Q = -H(T_node - T_lateral) 
            //
            // Q = -HT_node + HT_lateral 
            //
            // HT_node goes into the coefficient matrix 
            // where H * T[i] is calculated 
            // We sum over all H, and therefore we need to 
            // sum over all H for each node
            // we need sum of conductances, which becomes the coefficient 
            // for the node temperature

            for conductance_array in 
                self.lateral_adjacent_array_conductance_vector.iter(){


                    // now I'm inside the each conductance array,
                    // i can now sum the conductance array
                    // using array arithmetic without the hassle of indexing
                    sum_of_lateral_conductances += conductance_array;
                }
            // end sum of conductances for loop
            
            
            // we also need the HT sum for the lateral temperatures 
            // this is power due to heat transfer (other than advection)

            for (lateral_idx, conductance_arr) in 
                self.lateral_adjacent_array_conductance_vector.iter().enumerate() {
                    // the HT here comes from 
                    //
                    // Q = -H(T_node - T_lateral) 
                    //
                    // Q = -HT_node + HT_lateral 
                    //
                    // HT_node goes into the coefficient matrix 
                    // where H * T[i] is calculated 
                    // We sum over all H, and therefore we need to 
                    // sum over all H for each node
                    // 
                    // However, HT_lateral needs to be summed manually 
                    // over all the nodes and over all adjacent temperatures

                    // I will need to index into the conductance array
                    // and sum the conductances times temperature
                    //
                    // so the node_ht_lateral_sum represents the sum 
                    // of this, which we must sequentially add to

                    // for each node, at node index, we need to sum 
                    // all HT_lateral for all laterally linked 
                    // temperature arrays

                    // once we cycle through the lateral index, 
                    // we need to calculate the vector of power contributions 
                    // for this temperature array

                    let temperature_arr: Array1<ThermodynamicTemperature> 
                    = self
                    .lateral_adjacent_array_temperature_vector[lateral_idx]
                    .clone();

                    let mut power_arr: Array1<Power> = Array::zeros(
                        number_of_nodes);

                    for (node_idx, power) in power_arr.iter_mut().enumerate() {

                        *power = conductance_arr[node_idx] 
                            * temperature_arr[node_idx];

                    }


                    sum_of_lateral_conductance_times_lateral_temperatures 
                        += &power_arr;

                }


        }
        //dbg!(&sum_of_lateral_conductance_times_lateral_temperatures);
        //dbg!(&sum_of_lateral_conductances);
        // we need to do the same for the q and q fractions
        //once the power array is built, I can add it to 
        // the htsum array


        let lateral_power_sources_connected: bool 
        = self.q_vector.len() > 0; 

        let mut sum_of_lateral_power_sources: Array1<Power>
        = Array::zeros(number_of_nodes);

        if lateral_power_sources_connected {
            // again index into each node, multiply q by the 
            // q fraction 
            //
            // suppose there are two power sources in a three node 
            // system 
            //
            // 
            // [P1a, P1b, P1c] 
            // [P2a, P2b, P2c] 
            //
            // --> these are power ndarray
            //
            // The total power contributed to each node is:
            //
            //
            // [Pa, Pb, Pc]
            //
            // --> this is the sum_of_lateral_power_sources
            //
            // The simplest way would be to multiply the power 
            // q by its respective fractions to obtain a vector
            // of power arrays

            let mut power_ndarray_vector: Vec<Array1<Power>>
            = vec![];

            for (lateral_idx, q_reference) in self.q_vector.iter().enumerate() {

                // multiply q by qfraction


                let power_frac_array: Array1<f64>
                = self.q_fraction_vector[lateral_idx].clone();

                let power_ndarray: Array1<Power>
                = power_frac_array.map(
                    |&power_frac| {
                        power_frac * (*q_reference)
                    }

                );


                power_ndarray_vector.push(power_ndarray);

            }

            // this part constructs
            // the sum_of_lateral_power_sources
            // vector
            //
            // [Pa, Pb, Pc]
            //
            // at first, it's just a vector of zero power 
            // 
            // [0, 0, 0]
            //
            // The outer loop gets the node index, 
            // for example, node 1, 
            //
            // and obtains a reference to the node power source,
            // 0 watts
            //

            for (node_idx, node_power_source) in 
                sum_of_lateral_power_sources.iter_mut().enumerate() {

                    // at the node index i, and 
                    // with a reference to the node power source,
                    // I want to sum all the power sources
                    //
                    // So I start indexing into each power vector

                    for power_vector in power_ndarray_vector.iter(){

                        // from each power vector, I pull out the 
                        // relevant node power at the correct 
                        // node

                        let node_power_contribution = 
                            power_vector[node_idx];

                        // then I add it to the power source

                        *node_power_source 
                            += node_power_contribution;


                    }

                }

        }



        // back node calculation (first node)
        {

            // for the first node, also called the back node
            // energy balance is: 
            // m c_p dT/dt = -\sum H (T - T_lateral) - m_flow h_fluid(T) 
            // + m_flow h_fluid(adjacent T) + q
            // of all these terms, only the m cp dT/dt term and HT 
            // 
            // We separate this out to get:
            //
            // m cp T / dt + \sum HT = 
            //
            // \sum HT_lateral - m_flow h_fluid(T_old) 
            // + m_flow h_fluid(adjacent T_old) + m cp / dt (Told)
            // + q
            //
            // so we will need to determine sum H and sum HT_lateral
            // as sum H is the relevant coefficient in the coefficient_matrix




            // Now I'm ready to construct the M matrix
            // belong in the M matrix, the rest belong in S
            coefficient_matrix[[0,0]] = 
                volume_fraction_array[0] * rho_cp[0] 
                * total_volume / dt 
                + sum_of_lateral_conductances[0];

            //dbg!(&coefficient_matrix[[0,0]]);

            // the first part of the source term deals with 
            // the flow direction independent terms

            let h_fluid_last_timestep: AvailableEnergy = 
            self.back_single_cv.current_timestep_control_volume_specific_enthalpy;

            // now this makes the scheme semi implicit, and we should then 
            // treat the scheme as explicit
            
            // we need to consider heat source from lateral conductances 
            // for that, we need to find the temperature at each node 
            // and multiply that by the conductance 




            power_source_vector[0] = 
                sum_of_lateral_conductance_times_lateral_temperatures[0]
                - mass_flowrate * h_fluid_last_timestep 
                + self.temperature_array_current_timestep[0] 
                * total_volume 
                * volume_fraction_array[0] * rho_cp[0] / dt 
                + sum_of_lateral_power_sources[0]
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
            //minimal differences between parallel implementation and original
            //dbg!(&total_enthalpy_rate_change_back_node);
            //dbg!(&(self.temperature_array_current_timestep[0] 
            //    * total_volume * 
            //    volume_fraction_array[0] * rho_cp[0] / dt));

            if !forward_flow {
                // first, get enthalpy of the node in front 

                let enthalpy_of_adjacent_node_to_the_front: AvailableEnergy = 
                try_get_h(
                    self.back_single_cv.material_control_volume,
                    self.temperature_array_current_timestep[1],
                    self.back_single_cv.pressure_control_volume).unwrap();

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
                    * total_volume / dt + sum_of_lateral_conductances[i];

                // we also consider outflow using previous timestep 
                // temperature, 
                // assume back cv and front cv material are the same

                let h_fluid_last_timestep: AvailableEnergy = 
                try_get_h(
                    self.back_single_cv.material_control_volume,
                    self.temperature_array_current_timestep[i],
                    self.back_single_cv.pressure_control_volume).unwrap();

                // basically, all the power terms remain 
                power_source_vector[i] = 
                    sum_of_lateral_conductance_times_lateral_temperatures[i]
                    - mass_flowrate.abs() * h_fluid_last_timestep 
                    + self.temperature_array_current_timestep[i] * total_volume * 
                    volume_fraction_array[i] * rho_cp[i] / dt 
                    + sum_of_lateral_power_sources[i];

                // account for enthalpy inflow

                if forward_flow {

                    // enthalpy must be based on the the cv at i-1

                    let h_fluid_adjacent_node: AvailableEnergy = 
                    try_get_h(
                        self.back_single_cv.material_control_volume,
                        self.temperature_array_current_timestep[i-1],
                        self.back_single_cv.pressure_control_volume).unwrap();


                    power_source_vector[i] += 
                    h_fluid_adjacent_node * mass_flowrate.abs();

                } else {

                    // enthalpy must be based on cv at i+1
                    let h_fluid_adjacent_node: AvailableEnergy = 
                    try_get_h(
                        self.back_single_cv.material_control_volume,
                        self.temperature_array_current_timestep[i+1],
                        self.back_single_cv.pressure_control_volume).unwrap();


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
                * total_volume / dt + sum_of_lateral_conductances[i];
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

            power_source_vector[i] = 
                sum_of_lateral_conductance_times_lateral_temperatures[i]
                + self.temperature_array_current_timestep[i] * total_volume * 
                volume_fraction_array[i] * rho_cp[i] / dt 
                + sum_of_lateral_power_sources[i] 
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
                    self.back_single_cv.material_control_volume,
                    self.temperature_array_current_timestep[i-1],
                    self.back_single_cv.pressure_control_volume).unwrap();

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
                self.front_single_cv.current_timestep_control_volume_specific_enthalpy;

                power_source_vector[i] -= 
                mass_flowrate.abs() * h_fluid_last_timestep;
            }



        }

        //// note that this works for high peclet number flows
        //// peclet number is Re * Pr
        //// if peclet number is low, then we must consider conduction 
        ////
        //// I'm also not interested in directionality,
        //// rather, the magnitude is more important

        let reynolds: Ratio = self.get_reynolds(self.mass_flowrate)?.abs();

        let prandtl_number: Ratio = try_get_prandtl(
            material,
            bulk_temperature,
            pressure
        )?;

        let peclet_number = reynolds * prandtl_number;
        let average_axial_conductance: ThermalConductance;

        // note: this part is quite buggy as in the peclet number correction 
        // bit
        //
        // let peclet_number = Ratio::zero();
        //
        // I ascertained manually setting peclet number to zero does not 
        // visibly change the results, hence, 
        // it seems okay for now

        let low_peclet_number_flow = peclet_number.value < 100.0;
        
        if low_peclet_number_flow {
            // for low peclet number flows, consider conduction
            // which means we need to get axial conductance 
            // between nodes 

            let average_fluid_conductivity = try_get_kappa_thermal_conductivity(
                material,
                bulk_temperature,
                pressure
            )?;

            // note that conductance axially is done only ONCE 
            // per timestep to expedite the speed of calculation

            average_axial_conductance = 
                average_fluid_conductivity * 
                self.xs_area / node_length;

            for node_idx in 0..number_of_nodes {
                // check if first or last node 
                let first_node: bool = node_idx == 0;
                let last_node: bool = node_idx == number_of_nodes - 1;
                // bulk node means it's not the first node and not the 
                // last node, not OR, 
                // otherwise every node is a bulk node
                //
                // debug note: major bug was solved here 
                // with boolean operators, i used an OR operator 
                // rather than the AND operator
                let bulk_node: bool = !first_node  && !last_node;
                // bulk nodes
                if bulk_node {

                    // The extra terms from conduction are: 
                    // q = -H_avg(T_i - T_{i+1}) 
                    // -H_avg (T_i - T_{i-1})
                    //
                    // of course, by convention, we move all 
                    // temperature dependent terms to the right hand 
                    // side so that 
                    //
                    // m cp dT_i/dt + 2 H_avg T_i
                    // - Havg T_{i+1} - Havg T_{i-1}
                    // + other terms (see above)
                    // = heat sources
                    //
                    // So i'll have to add conductances to the 
                    // coefficient matrix
                    coefficient_matrix[[node_idx,node_idx]] 
                    += 2.0 *average_axial_conductance;

                    // we index using row, column convention 
                    // as per the ndarray crate
                    // "In a 2D array the index of each element is 
                    // [row, column] as seen in this 4 × 3 example:"
                    // https://docs.rs/ndarray/latest/ndarray/struct.ArrayBase.html
                    //
                    // The row is i, because that deals with node i 
                    //
                    // the column is i, i-1 and i+1 because that deals 
                    // with the node temperatures affecting node i 
                    //
                    // In this case, I want to involve T_{i+1} 
                    // and T_i and both terms are multipled by an 
                    // average conductance
                    // for speed. 
                    //
                    // Of course, one could use the node conductance 
                    // for each node to estimate the conductance 
                    // but that would be computationally expensive

                    coefficient_matrix[[node_idx, node_idx+1]] 
                    -= average_axial_conductance;

                    coefficient_matrix[[node_idx, node_idx-1]] 
                    -= average_axial_conductance;

                }

                // first node 
                if first_node {

                    // it's easier to do bulk nodes first because 
                    // it is the general case 
                    // first node is a fringe case where 
                    // it only conducts heat from the node in front 
                    // m cp dT_0/dt +  H_avg T_0
                    // - Havg T_{1} 
                    // + other terms (see above)
                    // = heat sources

                    coefficient_matrix[[node_idx,node_idx]] 
                    += average_axial_conductance;
                    coefficient_matrix[[node_idx, node_idx+1]] 
                    -= average_axial_conductance;

                }
                // last node 
                if last_node {

                    // likewise, the last node is also a fringe case

                    coefficient_matrix[[node_idx,node_idx]] 
                    += average_axial_conductance;
                    coefficient_matrix[[node_idx, node_idx-1]] 
                    -= average_axial_conductance;
                }


                // done modification for axial conduction
            }
            // done for loop
        }
        // done peclet number check (fluid array)

        
        //dbg!(&power_source_vector);
        // parallel same as normal implementation
        //dbg!(&coefficient_matrix);
        new_temperature_array = 
            solve_conductance_matrix_power_vector(
                coefficient_matrix,power_source_vector)?;
        // update the single cvs at the front and back with new enthalpies 

        // Todo: probably need to synchronise error types in future
        let back_node_enthalpy_next_timestep: AvailableEnergy = 
        try_get_h(
            self.back_single_cv.material_control_volume,
            new_temperature_array[0],
            self.back_single_cv.pressure_control_volume).unwrap();

        self.back_single_cv.current_timestep_control_volume_specific_enthalpy 
            = back_node_enthalpy_next_timestep;

        let front_node_enthalpy_next_timestep: AvailableEnergy = 
        try_get_h(
            self.front_single_cv.material_control_volume,
            new_temperature_array[number_of_nodes-1],
            self.front_single_cv.pressure_control_volume).unwrap();

        self.front_single_cv.current_timestep_control_volume_specific_enthalpy 
            = front_node_enthalpy_next_timestep;
        // let's also update the previous temperature vector 

        self.set_temperature_array(new_temperature_array.clone())?;


        // need to also set the front and back single cv temperature 
        // [back ------ front]
        //
        // [T0, T1, T2, ... Tn]
        //
        // the back is the first temperature in the array, 
        // and the front is the last
        //
        let back_cv_temperature: ThermodynamicTemperature 
            = *new_temperature_array.first().unwrap();

        let front_cv_temperature: ThermodynamicTemperature 
            = *new_temperature_array.last().unwrap();

        self.back_single_cv.temperature = back_cv_temperature;
        self.front_single_cv.temperature = front_cv_temperature;

        // set liquid cv mass after the temperature
        self.back_single_cv.set_liquid_cv_mass_from_temperature()?;
        self.front_single_cv.set_liquid_cv_mass_from_temperature()?;

        self.clear_vectors()?;

        // all done
        Ok(())
    }

    /// clears all vectors for next timestep
    /// This is important for the advance timestep method
    pub fn clear_vectors(&mut self) 
    -> Result<(), TuasLibError>{

        self.lateral_adjacent_array_conductance_vector.clear();
        self.lateral_adjacent_array_temperature_vector.clear();

        self.q_vector.clear();
        self.q_fraction_vector.clear();

        self.back_single_cv.clear_vectors()?;
        self.front_single_cv.clear_vectors()?;

        Ok(())
    }
}
