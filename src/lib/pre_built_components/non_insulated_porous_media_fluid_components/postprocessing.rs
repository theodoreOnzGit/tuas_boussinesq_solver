use uom::si::f64::*;

use super::NonInsulatedPorousMediaFluidComponent;

impl NonInsulatedPorousMediaFluidComponent {
    /// provides an array of temperatures representing 
    /// the steel piping within the heater top or bottom head
    pub fn steel_shell_temperature(&mut self) -> Vec<ThermodynamicTemperature>{
        self.pipe_shell.get_temperature_vector().unwrap()
    }

    /// provides an array of temperatures representing 
    /// the therminol fluid within the heater top or bottom head
    pub fn therminol_array_temperature(&mut self) -> Vec<ThermodynamicTemperature>{
        self.pipe_fluid_array.get_temperature_vector().unwrap()
    }

    /// provides an array of temperatures representing 
    /// the twisted tape within the heater top or bottom head
    pub fn twisted_tape_temperature(&mut self) -> Vec<ThermodynamicTemperature>{
        self.interior_solid_array_for_porous_media.get_temperature_vector().unwrap()
    }
}
