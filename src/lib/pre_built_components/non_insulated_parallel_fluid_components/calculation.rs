use super::NonInsulatedParallelFluidComponent;
use uom::si::f64::*;
use uom::si::power::watt;
use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::array_control_vol_and_fluid_component_collections::standalone_fluid_nodes::solve_conductance_matrix_power_vector;
use crate::boussinesq_thermophysical_properties::prandtl::try_get_prandtl;
use crate::boussinesq_thermophysical_properties::specific_enthalpy::try_get_h;
use crate::boussinesq_thermophysical_properties::thermal_conductivity::try_get_kappa_thermal_conductivity;
use crate::boussinesq_thermophysical_properties::volumetric_heat_capacity::try_get_rho_cp;
use crate::thermal_hydraulics_error::TuasLibError;
use std::thread::JoinHandle;
use std::thread;
use ndarray::*;
use ndarray_linalg::error::LinalgError;
use uom::num_traits::Zero;

impl NonInsulatedParallelFluidComponent {

    /// advances timestep for each HeatTransferEntity within the 
    /// NonInsulatedPipe
    /// treats the pipe as a single tube
    #[inline]
    fn _advance_timestep_single_tube(&mut self, 
    timestep: Time) -> Result<(),TuasLibError> {

        self.pipe_fluid_array.advance_timestep_mut_self(timestep)?;
        self.pipe_shell.advance_timestep_mut_self(timestep)?;
        Ok(())
        
    }

    /// advances timestep for each HeatTransferEntity within the 
    /// NonInsulatedPipe
    ///
    /// gives each pipe the parallel tube treatment
    #[inline]
    pub fn advance_timestep(&mut self, 
    timestep: Time) -> Result<(),TuasLibError> {
        self.advance_timestep_for_parallel_fluid_array_bundle(timestep)?;
        self.advance_timestep_for_parallel_solid_column_bundle(timestep)?;
        Ok(())
        
    }

    #[inline]
    fn advance_timestep_for_parallel_fluid_array_bundle(&mut self,
        timestep: Time,) -> Result<(), TuasLibError>{

        // first, we need to perform timestep advancement 
        // like for: 
        // self.pipe_fluid_array.advance_timestep_mut_self(timestep)?;
        //
        // however, we must factor in the 1/number of tubes for each tube
        
        let one_over_number_of_tubes: f64 = 1.0/(self.number_of_tubes as f64);

        // we shall need to clone and convert the pipe_fluid_array into 
        // an actual fluid array 

        let mut fluid_array_clone: FluidArray = 
            self.pipe_fluid_array.clone().try_into()?;

        // for mass flowrate, getting the mass flow property from the 
        // fluid array clone is the single tube mass flowrate
        let mass_flowrate_for_single_tube = 
            fluid_array_clone.get_mass_flowrate();


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

        let number_of_nodes = fluid_array_clone.len();
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
        fluid_array_clone.back_single_cv.rate_enthalpy_change_vector.clone();

        let front_cv_rate_enthalpy_change_vector: Vec<Power> = 
        fluid_array_clone.front_single_cv.rate_enthalpy_change_vector.clone();


        // compute power source for back node,
        // then multiply by one_over_number_of_tubes

        let mut total_enthalpy_rate_change_back_node = 
        Power::new::<watt>(0.0);

        for enthalpy_chg_rate in 
            back_cv_rate_enthalpy_change_vector.iter() {

                total_enthalpy_rate_change_back_node += *enthalpy_chg_rate;
            }


        let mut total_enthalpy_rate_change_front_node = 
        Power::new::<watt>(0.0);

        for enthalpy_chg_rate in 
            front_cv_rate_enthalpy_change_vector.iter() {

                total_enthalpy_rate_change_front_node += *enthalpy_chg_rate;
            }

        // this is the parallel tube correction bit 

        total_enthalpy_rate_change_back_node *= one_over_number_of_tubes;
        total_enthalpy_rate_change_front_node *= one_over_number_of_tubes;

        //minimal differences between parallel implementation and original
        //dbg!(&total_enthalpy_rate_change_back_node);

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

        let forward_flow: bool = mass_flowrate_for_single_tube
            .ge(&MassRate::zero());

        // obtain some important parameters for calculation
        let material = fluid_array_clone.material_control_volume;
        let pressure = fluid_array_clone.pressure_control_volume;
        let bulk_temperature = fluid_array_clone.try_get_bulk_temperature()?;
        let total_volume = fluid_array_clone.get_component_length() 
            *  fluid_array_clone.get_cross_sectional_area();
        let dt = timestep;
        let node_length = fluid_array_clone.get_component_length() 
            / number_of_nodes as f64;
        // for the fluid array, we do not keep track of node enthalpies,
        // instead, we keep track of temperatures and then calculate 
        // enthalpy changes using rho_cp calculated at the current 
        // timestep
        let volume_fraction_array: Array1<f64> = 
        fluid_array_clone.volume_fraction_array.iter().map(
            |&vol_frac| {
                vol_frac
            }
        ).collect();
        // rho_cp is determined by the temperature array 
        let rho_cp: Array1<VolumetricHeatCapacity> = 
        fluid_array_clone.temperature_array_current_timestep.iter().map(
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
        = fluid_array_clone.lateral_adjacent_array_conductance_vector.len() > 0;

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
                fluid_array_clone.lateral_adjacent_array_conductance_vector.iter(){


                    // now I'm inside the each conductance array,
                    // i can now sum the conductance array
                    // using array arithmetic without the hassle of indexing
                    sum_of_lateral_conductances += conductance_array;
                }
            // end sum of conductances for loop
            
            
            // we also need the HT sum for the lateral temperatures 
            // this is power due to heat transfer (other than advection)

            for (lateral_idx, conductance_arr) in 
                fluid_array_clone.lateral_adjacent_array_conductance_vector.iter().enumerate() {
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
                    = fluid_array_clone
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
        // we need to do the same for the q and q fractions
        //once the power array is built, I can add it to 
        // the htsum array


        let lateral_power_sources_connected: bool 
        = fluid_array_clone.q_vector.len() > 0; 

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

            for (lateral_idx, q_reference) in fluid_array_clone.q_vector.iter().enumerate() {

                // multiply q by qfraction


                let power_frac_array: Array1<f64>
                = fluid_array_clone.q_fraction_vector[lateral_idx].clone();

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


            // the first part of the source term deals with 
            // the flow direction independent terms

            let h_fluid_last_timestep: AvailableEnergy = 
            fluid_array_clone.back_single_cv.current_timestep_control_volume_specific_enthalpy;

            // now this makes the scheme semi implicit, and we should then 
            // treat the scheme as explicit
            
            // we need to consider heat source from lateral conductances 
            // for that, we need to find the temperature at each node 
            // and multiply that by the conductance 




            power_source_vector[0] = 
                sum_of_lateral_conductance_times_lateral_temperatures[0]
                - mass_flowrate_for_single_tube * h_fluid_last_timestep 
                + fluid_array_clone.temperature_array_current_timestep[0] 
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

            if !forward_flow {
                // first, get enthalpy of the node in front 

                let enthalpy_of_adjacent_node_to_the_front: AvailableEnergy = 
                try_get_h(
                    fluid_array_clone.back_single_cv.material_control_volume,
                    fluid_array_clone.temperature_array_current_timestep[1],
                    fluid_array_clone.back_single_cv.pressure_control_volume)?;

                // now if mass flowrate is less than zero, then 
                // we receive enthalpy from the front cv 
                //
                // But we need to subtract the negative mass flow if 
                // that makes sense, or at least make it absolute
                // - (-m) * h = m * h
                //

                power_source_vector[0] += 
                mass_flowrate_for_single_tube.abs() * enthalpy_of_adjacent_node_to_the_front;

                // additionally, in backflow situations, the mass 
                // flow out of this cv is already accounted for 
                // so don't double count 

                power_source_vector[0] += 
                mass_flowrate_for_single_tube * h_fluid_last_timestep;
            }


        }
        //dbg!(&power_source_vector[0]);

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
                    fluid_array_clone.back_single_cv.material_control_volume,
                    fluid_array_clone.temperature_array_current_timestep[i],
                    fluid_array_clone.back_single_cv.pressure_control_volume)?;

                // basically, all the power terms remain 
                power_source_vector[i] = 
                    sum_of_lateral_conductance_times_lateral_temperatures[i]
                    - mass_flowrate_for_single_tube.abs() * h_fluid_last_timestep 
                    + fluid_array_clone.temperature_array_current_timestep[i] * total_volume * 
                    volume_fraction_array[i] * rho_cp[i] / dt 
                    + sum_of_lateral_power_sources[i];

                // account for enthalpy inflow

                if forward_flow {

                    // enthalpy must be based on the the cv at i-1

                    let h_fluid_adjacent_node: AvailableEnergy = 
                    try_get_h(
                        fluid_array_clone.back_single_cv.material_control_volume,
                        fluid_array_clone.temperature_array_current_timestep[i-1],
                        fluid_array_clone.back_single_cv.pressure_control_volume)?;


                    power_source_vector[i] += 
                    h_fluid_adjacent_node * mass_flowrate_for_single_tube.abs();

                } else {

                    // enthalpy must be based on cv at i+1
                    let h_fluid_adjacent_node: AvailableEnergy = 
                    try_get_h(
                        fluid_array_clone.back_single_cv.material_control_volume,
                        fluid_array_clone.temperature_array_current_timestep[i+1],
                        fluid_array_clone.back_single_cv.pressure_control_volume)?;


                    power_source_vector[i] += 
                    h_fluid_adjacent_node * mass_flowrate_for_single_tube.abs();

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
                + fluid_array_clone.temperature_array_current_timestep[i] * total_volume * 
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
                    fluid_array_clone.back_single_cv.material_control_volume,
                    fluid_array_clone.temperature_array_current_timestep[i-1],
                    fluid_array_clone.back_single_cv.pressure_control_volume)?;

                // now if mass flowrate is less than zero, then 
                // we receive enthalpy from the front cv 
                //
                // But we need to subtract the negative mass flow if 
                // that makes sense, or at least make it absolute
                // - (-m) * h = m * h
                //

                power_source_vector[i] += 
                mass_flowrate_for_single_tube.abs() * enthalpy_of_adjacent_node_to_the_rear;


            } else {
                // if there's backflow, 
                // the front cv (last node) will receive enthalpy from 
                // outside based on the enthalpy rate change vector in the 
                // front node, 
                // however it must also lose enthalpy, this is no 
                // longer accounted for in the 
                let h_fluid_last_timestep: AvailableEnergy = 
                fluid_array_clone.front_single_cv.current_timestep_control_volume_specific_enthalpy;

                power_source_vector[i] -= 
                mass_flowrate_for_single_tube.abs() * h_fluid_last_timestep;
            }



        }
        //// note that this works for high peclet number flows
        //// peclet number is Re * Pr
        //// if peclet number is low, then we must consider conduction 
        ////
        //// I'm also not interested in directionality,
        //// rather, the magnitude is more important

        let reynolds: Ratio = fluid_array_clone.get_reynolds(
            mass_flowrate_for_single_tube)?.abs();

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
                fluid_array_clone.xs_area / node_length;

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
        //dbg!(&sum_of_lateral_conductance_times_lateral_temperatures[0]);
        new_temperature_array = 
            solve_conductance_matrix_power_vector(
                coefficient_matrix,power_source_vector)?;
        // update the single cvs at the front and back with new enthalpies 

        // Todo: probably need to synchronise error types in future
        let back_node_enthalpy_next_timestep: AvailableEnergy = 
        try_get_h(
            fluid_array_clone.back_single_cv.material_control_volume,
            new_temperature_array[0],
            fluid_array_clone.back_single_cv.pressure_control_volume)?;

        fluid_array_clone.back_single_cv.current_timestep_control_volume_specific_enthalpy 
            = back_node_enthalpy_next_timestep;

        let front_node_enthalpy_next_timestep: AvailableEnergy = 
        try_get_h(
            fluid_array_clone.front_single_cv.material_control_volume,
            new_temperature_array[number_of_nodes-1],
            fluid_array_clone.front_single_cv.pressure_control_volume)?;

        fluid_array_clone.front_single_cv.current_timestep_control_volume_specific_enthalpy 
            = front_node_enthalpy_next_timestep;
        // let's also update the previous temperature vector 

        fluid_array_clone.set_temperature_array(new_temperature_array.clone())?;


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

        fluid_array_clone.back_single_cv.temperature = back_cv_temperature;
        fluid_array_clone.front_single_cv.temperature = front_cv_temperature;

        // set liquid cv mass after the temperature
        fluid_array_clone.back_single_cv.set_liquid_cv_mass_from_temperature()?;
        fluid_array_clone.front_single_cv.set_liquid_cv_mass_from_temperature()?;

        fluid_array_clone.clear_vectors()?;

        // change the pipe fluid array now
        self.pipe_fluid_array = fluid_array_clone.into();

        Ok(())
    }

    #[inline]
    fn advance_timestep_for_parallel_solid_column_bundle(
        &mut self,
        timestep: Time,) -> Result<(), TuasLibError>{

        // like for: 
        // self.pipe_fluid_array.advance_timestep_mut_self(timestep)?;
        //
        // however, we must factor in the 1/number of tubes for each tube
        
        let one_over_number_of_tubes: f64 = 1.0/(self.number_of_tubes as f64);

        // we shall need to clone and convert the pipe_fluid_array into 
        // an actual fluid array 

        let mut pipe_shell_clone: SolidColumn = 
            self.pipe_shell.clone().try_into()?;

        // there must always be at least 2 nodes

        let number_of_nodes = pipe_shell_clone.len();

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
        pipe_shell_clone.back_single_cv.rate_enthalpy_change_vector.clone();

        let front_cv_rate_enthalpy_change_vector: Vec<Power> = 
        pipe_shell_clone.front_single_cv.rate_enthalpy_change_vector.clone();

        // compute power source for back node
        //
        // I also need to factor this by 1/number_of_tubes

        let mut total_enthalpy_rate_change_back_node = 
        Power::new::<watt>(0.0);

        for enthalpy_chg_rate in 
            back_cv_rate_enthalpy_change_vector.iter() {

                total_enthalpy_rate_change_back_node += *enthalpy_chg_rate;
            }

        total_enthalpy_rate_change_back_node *= one_over_number_of_tubes;

        // then the front node,
        //
        // I also need to factor this by 1/number_of_tubes

        let mut total_enthalpy_rate_change_front_node = 
        Power::new::<watt>(0.0);

        for enthalpy_chg_rate in 
            front_cv_rate_enthalpy_change_vector.iter() {

                total_enthalpy_rate_change_front_node += *enthalpy_chg_rate;
            }

        total_enthalpy_rate_change_front_node *= one_over_number_of_tubes;


        // this front and back nodes will be an extra term added to the 
        // heat source vector S
        //
        // The old solid temperature will need to be used to calculate 
        // new specific enthalpy for the system
        //
        // We need a blank temperature array to start the iteration 
        // process, so we just copy the old temperature array over 
        // as the initial guess (i think?)

        let new_temperature_array: Array1<ThermodynamicTemperature>;
        // now let's start calculation 
        //
        let mut coefficient_matrix: Array2<ThermalConductance> = 
        Array::zeros((number_of_nodes, number_of_nodes));

        let mut power_source_vector: 
        Array1<Power> = Array::zeros(number_of_nodes);

        // obtain some important parameters for calculation
        let material = pipe_shell_clone.material_control_volume;
        let pressure = pipe_shell_clone.pressure_control_volume;
        let bulk_temperature = pipe_shell_clone.try_get_bulk_temperature()?;
        let single_shell_volume = pipe_shell_clone.total_length *  
            pipe_shell_clone.get_component_xs_area();

        let dt = timestep;
        let node_length = pipe_shell_clone.total_length / number_of_nodes as f64;



        // for the fluid array, we do not keep track of node enthalpies,
        // instead, we keep track of temperatures and then calculate 
        // enthalpy changes using rho_cp calculated at the current 
        // timestep
        let volume_fraction_array: Array1<f64> = 
        pipe_shell_clone.volume_fraction_array.iter().map(
            |&vol_frac| {
                vol_frac
            }
        ).collect();
        // rho_cp is determined by the temperature array 
        let rho_cp: Array1<VolumetricHeatCapacity> = 
        pipe_shell_clone.temperature_array_current_timestep.iter().map(
            |&temperature| {
                try_get_rho_cp(material, temperature, pressure).unwrap()
            }
        ).collect();


        // need to determine the sort of connections 
        // we assume axial conduction always occurs unless it can be 
        // neglected
        let lateral_power_sources_connected: bool 
        = pipe_shell_clone.q_vector.len() > 0; 
        let lateral_temperature_arary_connected: bool 
        = pipe_shell_clone.lateral_adjacent_array_conductance_vector.len() > 0;

        let _axial_conduction_only: bool = !lateral_power_sources_connected
        && !lateral_temperature_arary_connected;

        // if there is lateral conduction, then construct matrices 
        // which take that into account 
        let mut sum_of_lateral_conductances: Array1<ThermalConductance>
        = Array1::zeros(number_of_nodes);

        let mut sum_of_lateral_conductance_times_lateral_temperatures:
        Array1<Power> = Array1::zeros(number_of_nodes);
        // conductances will need to be summed over each node 
        //
        // i will also need to make sure that there are actually 
        // lateral connections to this array in the first place 
        // though!
        //
        // moreover, I need to account for the number of tubes in 
        // each case I calculate HT


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
                pipe_shell_clone.lateral_adjacent_array_conductance_vector.iter(){


                    // now I'm inside the each conductance array,
                    // i can now sum the conductance array
                    // using array arithmetic without the hassle of indexing
                    sum_of_lateral_conductances += conductance_array ;
                }
            // end sum of conductances for loop
            
            // we also need the HT sum for the lateral temperatures 
            // this is power due to heat transfer (other than advection)

            for (lateral_idx, conductance_arr) in 
                pipe_shell_clone.lateral_adjacent_array_conductance_vector.iter().enumerate() {
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
                    //
                    // to account for parallel tubes, I multiply the power 
                    // by one_over_number_of_tubes
                    //
                    // but this is done in the 
                    // lateral_and_miscellaneous_connections,
                    // so don't double correct here

                    let temperature_arr: Array1<ThermodynamicTemperature> 
                    = pipe_shell_clone.lateral_adjacent_array_temperature_vector[lateral_idx].clone();

                    let mut power_arr: Array1<Power> = Array::zeros(
                        number_of_nodes);

                    for (node_idx, power) in power_arr.iter_mut().enumerate() {
                        
                        // this part deals with the HT
                        // since we are taking power values from the shell,
                        // don't double correct for number of tubes

                        *power = conductance_arr[node_idx] 
                            * temperature_arr[node_idx];
                    }

                    // once the power array is built, I can add it to 
                    // the htsum array

                    sum_of_lateral_conductance_times_lateral_temperatures 
                    += &power_arr;

                }


        }

        // we need to do the same for the q and q fractions
        let mut sum_of_lateral_power_sources: Array1<Power>
        = Array::zeros(number_of_nodes);

        if lateral_power_sources_connected {
            // again index into each node, multiply q by the 
            // q fraction 
            //
            // for a single shell, must multiply by one_over_number_of_tubes
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

            for (lateral_idx, q_reference) in pipe_shell_clone.q_vector.iter().enumerate() {

                // multiply q by qfraction


                let power_frac_array: Array1<f64>
                = pipe_shell_clone.q_fraction_vector[lateral_idx].clone();

                let power_ndarray: Array1<Power>
                = power_frac_array.map(
                    |&power_frac| {
                        // for parallel tube treatment,
                        // we already accounted for this in the 
                        // lateral_and_miscellaneous_connections 
                        // so don't double correct here
                        //
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
            // I don't need to correct for number_of_tubes again,
            // already did it previously
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
        // end if for lateral_power_sources_connected

        // now that we've gotten all the important properties, we can 
        // start matrix construction


        // back node calculation (first node)
        {

            // for the first node, also called the back node
            // energy balance is: 
            // m c_p dT/dt = -\sum H (T - T_lateral) + q
            // of all these terms, only the m cp dT/dt term and HT 
            // 
            // We separate this out to get:
            //
            // m cp T / dt + \sum HT = 
            // \sum HT_lateral + m cp / dt (Told)
            // + q
            //
            // so we will need to determine sum H and sum HT_lateral
            // as sum H is the relevant coefficient in the coefficient_matrix
            //
            // of course, for correction, I will only use the 
            // single_shell_volume rather than the total volume
            // of all shells in the bundle


            // Now I'm ready to construct the M matrix
            // belong in the M matrix, the rest belong in S
            coefficient_matrix[[0,0]] = 
                volume_fraction_array[0] * rho_cp[0] 
                * single_shell_volume / dt + sum_of_lateral_conductances[0];



            // now this makes the scheme semi implicit, and we should then 
            // treat the scheme as explicit

            power_source_vector[0] = 
                sum_of_lateral_conductance_times_lateral_temperatures[0] 
                + pipe_shell_clone.temperature_array_current_timestep[0] 
                * single_shell_volume 
                * volume_fraction_array[0] * rho_cp[0] / dt 
                + sum_of_lateral_power_sources[0]
                + total_enthalpy_rate_change_back_node ;

        }
        // end back node calculation code block

        
        // bulk node calculations 
        // only takes into account lateral conductances
        if number_of_nodes > 2 {
            // loop over all nodes from 1 to n-2 (n-1 is not included)
            for i in 1..number_of_nodes-1 {
                // the coefficient matrix is pretty much the same, 
                // we only consider solid fluid conduction, and the 
                // thermal inertia terms

                coefficient_matrix[[i,i]] = volume_fraction_array[i] * rho_cp[i] 
                    * single_shell_volume / dt + sum_of_lateral_conductances[i];

                // we also consider outflow using previous timestep 
                // temperature, 
                // assume back cv and front cv material are the same

                // basically, all the power terms remain 
                power_source_vector[i] = sum_of_lateral_conductance_times_lateral_temperatures[i] 
                    + pipe_shell_clone.temperature_array_current_timestep[i] 
                    * single_shell_volume 
                    * volume_fraction_array[i] * rho_cp[i] / dt 
                    + sum_of_lateral_power_sources[i];


            }
            // end for loop for bulk nodes
        }
        // end bulk node calculation block

        // front node (last node) calculation 
        {
            // we still follow the same guideline
            // m cp T / dt + HT = 
            //
            // HT_solid + m cp / dt (Told)
            // + q
            let i = number_of_nodes-1;

            coefficient_matrix[[i,i]] = volume_fraction_array[i] * rho_cp[i] 
                * single_shell_volume / dt + sum_of_lateral_conductances[i];
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

            power_source_vector[i] = sum_of_lateral_conductance_times_lateral_temperatures[i] 
                + pipe_shell_clone.temperature_array_current_timestep[i] 
                * single_shell_volume 
                * volume_fraction_array[i] * rho_cp[i] / dt 
                + sum_of_lateral_power_sources[i] 
                + total_enthalpy_rate_change_front_node ;


        }

        // the above takes care of lateral conduction and thermal 
        // inertia, if there was no need to worry about axial conduction 
        // then we can just follow through
        //
        // We can neglect axial conduction only if the lateral 
        // conduction is much greater than axial conduction 

        // an important parameter for all these calculations 
        // is the average axial thermal 
        // conductance
        // first calculate axial conduction thermal 
        // resistance

        let average_thermal_conductivity = 
        try_get_kappa_thermal_conductivity(
            material,
            bulk_temperature,
            pressure,
        )?;
        // i need to base the axial conductance on the 
        // single shell surface area rather than the surface area 
        // of the whole bundle of shells

        let average_axial_conductance: ThermalConductance 
        = average_thermal_conductivity 
        * pipe_shell_clone.get_component_xs_area()
        / node_length;

        // neglecting axial conduction may be good,
        // but checking for it is computationally expensive.
        // so just don't neglect_axial_conduction
        let neglect_axial_conduction = false;
        
        // if we dont neglect axial conduction 

        if !neglect_axial_conduction {

            // construct matrices for axial conduction
            for node_idx in 0..number_of_nodes {
                // check if first or last node 
                let first_node: bool = node_idx == 0;
                let last_node: bool = node_idx == number_of_nodes - 1;
                // bulk node means 
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
        // done axial conduction code and ready to solve matrix

        new_temperature_array = 
            solve_conductance_matrix_power_vector(
                coefficient_matrix,power_source_vector)?;

        // update the single cvs at the front and back with new enthalpies 

        // Todo: probably need to synchronise error types in future
        let back_node_enthalpy_next_timestep: AvailableEnergy = 
        try_get_h(
            pipe_shell_clone.back_single_cv.material_control_volume,
            new_temperature_array[0],
            pipe_shell_clone.back_single_cv.pressure_control_volume)?;

        pipe_shell_clone.back_single_cv.current_timestep_control_volume_specific_enthalpy 
            = back_node_enthalpy_next_timestep;

        let front_node_enthalpy_next_timestep: AvailableEnergy = 
        try_get_h(
            pipe_shell_clone.front_single_cv.material_control_volume,
            new_temperature_array[number_of_nodes-1],
            pipe_shell_clone.front_single_cv.pressure_control_volume)?;

        pipe_shell_clone.front_single_cv.current_timestep_control_volume_specific_enthalpy 
            = front_node_enthalpy_next_timestep;
        // let's also update the previous temperature vector 

        pipe_shell_clone.set_temperature_array(new_temperature_array.clone())?;
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

        pipe_shell_clone.back_single_cv.temperature = back_cv_temperature;
        pipe_shell_clone.front_single_cv.temperature = front_cv_temperature;

        // set liquid cv mass 
        // probably also need to update error types in future
        pipe_shell_clone.back_single_cv.set_liquid_cv_mass_from_temperature()?;
        pipe_shell_clone.front_single_cv.set_liquid_cv_mass_from_temperature()?;
        pipe_shell_clone.clear_vectors()?;

        self.pipe_shell = pipe_shell_clone.into();

        // all done
        Ok(())
    }


    /// advances timestep by spawning a thread 
    /// 
    pub fn advance_timestep_thread_spawn(&self,
        timestep: Time,) -> JoinHandle<Self> {

        // make a clone
        let mut heater_clone = self.clone();

        // move ptr into a new thread 

        let join_handle = thread::spawn(
            move || -> Self {


                // carry out the connection calculations
                heater_clone.advance_timestep(timestep).unwrap();
                
                heater_clone

            }
        );

        return join_handle;

    }
}
