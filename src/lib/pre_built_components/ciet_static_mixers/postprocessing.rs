use uom::si::f64::*;

use super::StaticMixers;

impl StaticMixers {
    /// gets the steel piping temperature of MX-10 in an array
    pub fn steel_shell_temperature(&mut self) -> Vec<ThermodynamicTemperature>{
        self.steel_shell.get_temperature_vector().unwrap()
    }

    /// gets the fluid temperature of MX-10 in an array
    pub fn therminol_array_temperature(&mut self) -> Vec<ThermodynamicTemperature>{
        self.therminol_array.get_temperature_vector().unwrap()
    }

    /// gets the insulation temperature in an array
    pub fn insulation_array_temperature(&mut self) -> Vec<ThermodynamicTemperature>{
        self.insulation_array.get_temperature_vector().unwrap()
    }
}
