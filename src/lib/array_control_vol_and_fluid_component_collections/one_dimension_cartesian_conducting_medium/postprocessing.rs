use super::CartesianConduction1DArray;
use crate::thermal_hydraulics_error::ThermalHydraulicsLibError;
use uom::si::f64::*;

impl CartesianConduction1DArray {

    /// returns a clone of the temperature_array_current_timestep
    #[inline]
    pub fn get_temperature_vector(&mut self) -> 
    Result<Vec<ThermodynamicTemperature>, ThermalHydraulicsLibError> {
        let temp_array = self.temperature_array_current_timestep.clone();

        let mut temp_vector: Vec<ThermodynamicTemperature> = vec![];

        for temp_reference in temp_array.iter() {
            temp_vector.push(*temp_reference)
        }

        Ok(temp_vector)
    }
}
