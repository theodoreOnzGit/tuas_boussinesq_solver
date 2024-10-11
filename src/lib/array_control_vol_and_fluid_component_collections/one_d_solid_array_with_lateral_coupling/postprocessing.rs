use super::SolidColumn;
use uom::si::f64::*;
use crate::tuas_lib_error::TuasLibError;

impl SolidColumn { 
    /// obtains a clone of the temperature vector within the CV 
    /// thus obtaining the temperature profile
    pub fn get_temperature_vector(&self) -> Result<
    Vec<ThermodynamicTemperature>,TuasLibError>{
        let mut temperature_vec: Vec<ThermodynamicTemperature> = vec![];

        for temperature in self.temperature_array_current_timestep.iter() {
            temperature_vec.push(*temperature);
        }

        return Ok(temperature_vec);
    }
    /// obtains a clone of temperature vector, but in reverse format 
    /// this is useful for counter flow heat exchangers 
    pub fn get_reverse_temperature_vector(&self) -> 
    Result<Vec<ThermodynamicTemperature>,TuasLibError>{
        let vec = self.get_temperature_vector()?;

        let reversed_vec = vec.iter().copied().rev().collect();

        Ok(reversed_vec)
    }


}
