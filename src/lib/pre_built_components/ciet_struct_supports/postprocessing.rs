use uom::si::f64::*;

use super::StructuralSupport;

impl StructuralSupport {

    /// obtains the temperature profile (or array) of the structural 
    /// support
    pub fn get_temperature_array(&mut self) -> Vec<ThermodynamicTemperature>{
        self.support_array.get_temperature_vector().unwrap()
    }

}
