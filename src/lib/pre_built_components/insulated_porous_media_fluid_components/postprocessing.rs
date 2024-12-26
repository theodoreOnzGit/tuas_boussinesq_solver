use uom::si::f64::*;

use super::InsulatedPorousMediaFluidComponent;

impl InsulatedPorousMediaFluidComponent {
    /// gets the steel piping temperature of MX-10 in an array
    pub fn steel_shell_temperature(&mut self) -> Vec<ThermodynamicTemperature>{
        self.pipe_shell.get_temperature_vector().unwrap()
    }

    /// gets the fluid temperature of MX-10 in an array
    pub fn therminol_array_temperature(&mut self) -> Vec<ThermodynamicTemperature>{
        self.pipe_fluid_array.get_temperature_vector().unwrap()
    }

    /// gets the insulation temperature in an array
    pub fn insulation_array_temperature(&mut self) -> Vec<ThermodynamicTemperature>{
        self.insulation_array.get_temperature_vector().unwrap()
    }

    /// returns the number of nodes in this InsulatedPorousMediaFluidComponent
    ///
    /// I made this to check if the constructor was working correctly
    pub fn number_of_nodes(&self) -> usize {
        return self.inner_nodes + 2;
    }
}

