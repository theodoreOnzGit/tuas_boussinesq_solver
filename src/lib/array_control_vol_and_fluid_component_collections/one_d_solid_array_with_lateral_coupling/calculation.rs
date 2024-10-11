use crate::array_control_vol_and_fluid_component_collections::standalone_fluid_nodes::solve_conductance_matrix_power_vector;
use crate::boussinesq_thermophysical_properties::thermal_conductivity::try_get_kappa_thermal_conductivity;

use crate::boussinesq_thermophysical_properties::volumetric_heat_capacity::try_get_rho_cp;


use crate::boussinesq_thermophysical_properties::specific_enthalpy::try_get_h;
use uom::si::f64::*;
use ndarray::*;
use uom::si::power::watt;


use crate::thermal_hydraulics_error::TuasLibError;
use ndarray_linalg::error::LinalgError;

use super::SolidColumn;
/// This deals with the calculations of the solid column array
/// at the end of the connection phase, one can then use 
/// the advance_timestep method to calculate the new 
/// temperature array
impl SolidColumn {

    /// advances timestep for the solid array column
    /// given a fixed timestep
    pub fn advance_timestep(&mut self,
        timestep: Time,) 
        -> Result<(), TuasLibError>{

        // there must always be at least 2 nodes

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

        let mut total_enthalpy_rate_change_front_node = 
        Power::new::<watt>(0.0);

        for enthalpy_chg_rate in 
            front_cv_rate_enthalpy_change_vector.iter() {

                total_enthalpy_rate_change_front_node += *enthalpy_chg_rate;
            }


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


        // need to determine the sort of connections 
        // we assume axial conduction always occurs unless it can be 
        // neglected
        let lateral_power_sources_connected: bool 
        = self.q_vector.len() > 0; 
        let lateral_temperature_arary_connected: bool 
        = self.lateral_adjacent_array_conductance_vector.len() > 0;

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
                    = self.lateral_adjacent_array_temperature_vector[lateral_idx].clone();

                    let mut power_arr: Array1<Power> = Array::zeros(
                        number_of_nodes);

                    for (node_idx, power) in power_arr.iter_mut().enumerate() {

                        *power = conductance_arr[node_idx] * temperature_arr[node_idx];
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

            // this part construts
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




            // Now I'm ready to construct the M matrix
            // belong in the M matrix, the rest belong in S
            coefficient_matrix[[0,0]] = 
                volume_fraction_array[0] * rho_cp[0] 
                * total_volume / dt + sum_of_lateral_conductances[0];

            //dbg!(&coefficient_matrix[[0,0]]);

            // now this makes the scheme semi implicit, and we should then 
            // treat the scheme as explicit

            power_source_vector[0] = 
                sum_of_lateral_conductance_times_lateral_temperatures[0] 
                + self.temperature_array_current_timestep[0] 
                * total_volume 
                * volume_fraction_array[0] * rho_cp[0] / dt 
                + sum_of_lateral_power_sources[0]
                + total_enthalpy_rate_change_back_node ;

            //dbg!(&power_source_vector[0]);
            //dbg!(&total_enthalpy_rate_change_back_node);
            //dbg!(&sum_of_lateral_power_sources[0]);
            //dbg!(&sum_of_lateral_conductance_times_lateral_temperatures[0]);
            //dbg!(&(self.temperature_array_current_timestep[0] 
            //    * total_volume 
            //    * volume_fraction_array[0] * rho_cp[0] / dt));


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
                    * total_volume / dt + sum_of_lateral_conductances[i];

                // we also consider outflow using previous timestep 
                // temperature, 
                // assume back cv and front cv material are the same

                // basically, all the power terms remain 
                power_source_vector[i] = sum_of_lateral_conductance_times_lateral_temperatures[i] 
                    + self.temperature_array_current_timestep[i] * total_volume * 
                    volume_fraction_array[i] * rho_cp[i] / dt 
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

            power_source_vector[i] = sum_of_lateral_conductance_times_lateral_temperatures[i] 
                + self.temperature_array_current_timestep[i] * total_volume * 
                volume_fraction_array[i] * rho_cp[i] / dt 
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

        let average_axial_conductance: ThermalConductance 
        = average_thermal_conductivity * self.xs_area 
        / node_length;

        // neglecting axial conduction may be good,
        // but checking for it is computationally expensive.
        let neglect_axial_conduction = false;
        
        //let neglect_axial_conduction: bool = {

        //    // how shall we know if the axial conduction is to be 
        //    // neglected?


        //    let axial_power_scale_insignificant = || -> bool {

        //        // we can calculate a typical power scale for axial 
        //        // conduction and compare it to the radial conduction



        //        // the max temperature gradient is the max temperature 
        //        // minus the min temperature 

        //        let temp_value_kelvin_integer_vector: Array1<u32> = 
        //        self.temperature_array_current_timestep.map(
        //            |&temperature|{
        //                let temp_value_kelvin = temperature.get::<kelvin>();

        //                temp_value_kelvin.ceil() as u32
        //            }
        //        );

        //        let max_temp_val_kelvin: u32 = 
        //        *temp_value_kelvin_integer_vector.iter().max().unwrap();

        //        let min_temp_val_kelvin: u32 = 
        //        *temp_value_kelvin_integer_vector.iter().min().unwrap();

        //        let approx_axial_temp_diff_val_kelvin: f64 = 
        //        max_temp_val_kelvin as f64 
        //        - min_temp_val_kelvin as f64;

        //        let axial_power_scale: Power = 
        //        TemperatureInterval::new::<degree_celsius>(
        //            approx_axial_temp_diff_val_kelvin)
        //        * average_axial_conductance;
        //        

        //        // we need to compare this against the radial temperature 
        //        // differences
        //        //
        //        // as well as power inputs 
        //        //
        //        // unfortunately, the temperatures are nested in a 
        //        // vector of arrays or in essence a 2D array

        //        let mut lateral_power_sum: Power = Power::zero();

        //        for &power in &self.q_vector {
        //            lateral_power_sum += power.abs();
        //        }


        //        // now an estimate for the lateral power by conduction
        //        // thankfully, we already calculated this beforehand
        //        

        //        // the HT here comes from 
        //        //
        //        // Q = -H(T_node - T_lateral) 
        //        //
        //        // Q = -HT_node + HT_lateral 

        //        let mut sum_of_conductance_times_node_temp_array:
        //        Array1<Power> = Array::zeros(number_of_nodes);

        //        for (node_idx, ht_node) in 
        //        sum_of_conductance_times_node_temp_array.iter_mut().enumerate(){

        //                // get the node temperature and sum of conductance 

        //                let node_temperature: ThermodynamicTemperature = 
        //                self.temperature_array_current_timestep[node_idx];

        //                let node_conductance_sum: ThermalConductance = 
        //                sum_of_lateral_conductances[node_idx];

        //                *ht_node = node_temperature * node_conductance_sum;

        //            }

        //        let lateral_ht_power_vector: Array1<Power> = 
        //        sum_of_lateral_conductance_times_lateral_temperatures 
        //        - sum_of_conductance_times_node_temp_array;

        //        // add all the power changes to the lateral power 
        //        // vector
        //        
        //        for &power in &lateral_ht_power_vector {
        //            lateral_power_sum += power.abs();
        //        }
        //        
        //        // for lateral power flow, we have the lateral 
        //        // power sum to measure the power flows 
        //        // 

        //        let axial_power_to_lateral_power_ratio: Ratio = 
        //        axial_power_scale.abs()/lateral_power_sum;

        //        // if this ratio is less than 1%, then we neglect 
        //        // axial power otherwise, don't neglect

        //        let neglect_axial_power = 
        //        axial_power_to_lateral_power_ratio <= Ratio::new::<percent>(1.0);

        //        neglect_axial_power
        //    };



        //    // first let's deal with the case that it's axial conduction 
        //    // only 

        //    if axial_conduction_only {
        //        // if axial conduction only, we cannot neglect 
        //        // axial conduction
        //        false
        //    } else if axial_power_scale_insignificant() {
        //        // if axial power scales are insignificant
        //        // neglect
        //        // axial conduction
        //        true
        //    } else {
        //        // the safest is to not neglect
        //        false
        //    }

        //};
        // end code block for checking if we can neglect axial conduction

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

        // set liquid cv mass 
        // probably also need to update error types in future
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
