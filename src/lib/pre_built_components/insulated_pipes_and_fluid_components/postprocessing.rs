use uom::si::f64::*;
use crate::thermal_hydraulics_error::ThermalHydraulicsLibError;

use super::InsulatedFluidComponent;

impl InsulatedFluidComponent {

    /// gets the temperature of the pipe shell array
    pub fn pipe_shell_temperature(&mut self) -> 
        Result<Vec<ThermodynamicTemperature>, ThermalHydraulicsLibError>{
        self.pipe_shell.get_temperature_vector()
    }

    /// gets the temperature of the pipe fluid array
    pub fn pipe_fluid_array_temperature(&mut self) ->
        Result<Vec<ThermodynamicTemperature>,ThermalHydraulicsLibError>{
        self.pipe_fluid_array.get_temperature_vector()
    }

    /// insulation temperature array 
    pub fn insulation_array_temperature(&mut self) ->
        Result<Vec<ThermodynamicTemperature>,ThermalHydraulicsLibError>{
        self.insulation.get_temperature_vector()
    }

}
