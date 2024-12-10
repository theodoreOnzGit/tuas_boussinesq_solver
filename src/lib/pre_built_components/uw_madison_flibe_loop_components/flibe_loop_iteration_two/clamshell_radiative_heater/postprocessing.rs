use uom::si::f64::*;
use crate::tuas_lib_error::TuasLibError;

use super::ClamshellRadiativeHeater;

impl ClamshellRadiativeHeater {

    /// gets the temperature of the inner pipe shell array
    pub fn pipe_shell_temperature(&mut self) -> 
        Result<Vec<ThermodynamicTemperature>, TuasLibError>{
            self.pipe_shell_array.get_temperature_vector()
    }

    /// gets the temperature of the tube side fluid array
    pub fn pipe_fluid_array_temperature(&mut self) ->
        Result<Vec<ThermodynamicTemperature>,TuasLibError>{
            self.pipe_fluid_array.get_temperature_vector()
    }

    /// gets the shell side fluid array temperature
    pub fn annular_air_array_temperature(&mut self,) ->
        Result<Vec<ThermodynamicTemperature>,TuasLibError>{
            self.annular_air_array.get_temperature_vector()
    }

    /// gets the shell side outer tube temperature 
    pub fn heating_element_array_temperature(&mut self,) -> 
        Result<Vec<ThermodynamicTemperature>,TuasLibError>{
            self.heating_element_shell.get_temperature_vector()
    }

    /// gets the temperature of the insulation 
    pub fn insulation_array_temperature(&mut self,) ->
        Result<Vec<ThermodynamicTemperature>,TuasLibError>{

            self.insulation_array.get_temperature_vector()
    }


    /// function to help obtain total
    /// heat transfer rate from both annular axial ends based 
    /// on view factor and such
    pub fn get_total_radiant_heat_rate_from_annular_axial_ends(&mut self,) ->
        Result<Power,TuasLibError>{

            todo!()
    }

    /// function to help obtain front side
    /// heat transfer rate from based 
    /// on view factor and such
    pub fn get_front_side_radiant_heat_rate_from_annular_axial_ends(&mut self,) ->
        Result<Power,TuasLibError>{

            todo!()
    }
    /// function to help obtain back side
    /// heat transfer rate from based 
    /// on view factor and such
    pub fn get_back_side_radiant_heat_rate_from_annular_axial_ends(&mut self,) ->
        Result<Power,TuasLibError>{

            todo!()
    }
}
