use uom::si::f64::*;
use crate::tuas_lib_error::TuasLibError;

use super::InsulatedFluidComponent;

impl InsulatedFluidComponent {

    /// gets the temperature of the pipe shell array
    pub fn pipe_shell_temperature(&mut self) -> 
        Result<Vec<ThermodynamicTemperature>, TuasLibError>{
        self.pipe_shell.get_temperature_vector()
    }

    /// gets the temperature of the pipe fluid array
    pub fn pipe_fluid_array_temperature(&mut self) ->
        Result<Vec<ThermodynamicTemperature>,TuasLibError>{
        self.pipe_fluid_array.get_temperature_vector()
    }

    /// insulation temperature array 
    pub fn insulation_array_temperature(&mut self) ->
        Result<Vec<ThermodynamicTemperature>,TuasLibError>{
        self.insulation.get_temperature_vector()
    }

}