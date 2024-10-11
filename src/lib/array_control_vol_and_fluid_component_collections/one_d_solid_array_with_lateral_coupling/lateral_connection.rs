use super::SolidColumn;

use ndarray::*;
use ndarray_linalg::error::LinalgError;
use uom::si::f64::*;
use crate::tuas_lib_error::TuasLibError;
/// this implementation deals with lateral connections 
///
/// the convention is to supply an average conductance 
/// as well as a temperature array
///
/// at the end of the connection phase, one can then use 
/// the advance_timestep method to calculate the new 
/// temperature array
impl SolidColumn {

    /// connects an adjacent solid or fluid node laterally 
    /// with a given average thermal conductance
    /// note that doing so with 
    pub fn lateral_link_new_temperature_vector_avg_conductance(&mut self,
    average_thermal_conductance: ThermalConductance,
    temperature_vec: Vec<ThermodynamicTemperature>) 
        -> Result<(), TuasLibError>{

        let number_of_temperature_nodes = self.len();

        // check if temperature_vec has the correct number_of_temperature_nodes

        if temperature_vec.len() !=  number_of_temperature_nodes {
            let shape_error = ShapeError::from_kind(
                ErrorKind::IncompatibleShape
            );

            let linalg_error = LinalgError::Shape(shape_error);

            return Err(TuasLibError::LinalgError
                (linalg_error));

        }

        // now let's make a new temperature array 

        let mut temperature_arr: Array1<ThermodynamicTemperature>
        = Array1::default(number_of_temperature_nodes);

        // assign the temperatures 
        
        for (idx, temperature) in temperature_vec.iter().enumerate() {

            temperature_arr[idx] = *temperature;

        }

        // push it to the lateral adjacent array temp vec 

        self.lateral_adjacent_array_temperature_vector.push(temperature_arr);

        // next, construct conductance vector

        let mut conductance_arr: Array1<ThermalConductance>
        = Array1::default(number_of_temperature_nodes);
        conductance_arr.fill(average_thermal_conductance);

        self.lateral_adjacent_array_conductance_vector.push(conductance_arr);

        Ok(())
    }
    /// connects an adjacent solid or fluid node laterally 
    /// with a given power source with an axial power distribution
    #[inline]
    pub fn lateral_link_new_power_vector(&mut self,
    power_source: Power,
    q_fraction_arr: Array1<f64>) 
        -> Result<(), TuasLibError>{

        // need to ensure that array parameters match 
        let number_of_temperature_nodes = self.len();

        if q_fraction_arr.len() !=  number_of_temperature_nodes {
            let shape_error = ShapeError::from_kind(
                ErrorKind::IncompatibleShape
            );

            let linalg_error = LinalgError::Shape(shape_error);

            return Err(TuasLibError::LinalgError
                (linalg_error));

        }
        

        self.q_vector.push(power_source);
        self.q_fraction_vector.push(q_fraction_arr);

        Ok(())
    }
}
