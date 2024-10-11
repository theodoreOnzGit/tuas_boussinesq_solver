use uom::si::f64::*;
use crate::tuas_lib_error::TuasLibError;

use super::SolidStructure;

impl SolidStructure {

    /// gets the temperature of the pipe shell array
    pub fn array_temperature(&mut self) -> 
        Result<Vec<ThermodynamicTemperature>, TuasLibError>{
        self.solid_array.get_temperature_vector()
    }


}
