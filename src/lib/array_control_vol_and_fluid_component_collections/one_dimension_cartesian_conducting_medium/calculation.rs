use super::CartesianConduction1DArray;
use uom::si::f64::*;
use uom::si::area::square_meter;
use ndarray::*;
use uom::si::power::watt;

use crate::array_control_vol_and_fluid_component_collections::conductance_array_functions::advance_timestep_for_specified_conductance_array_cv;
use crate::boussinesq_thermophysical_properties::specific_enthalpy::try_get_h;
use crate::tuas_lib_error::TuasLibError;
use crate::control_volume_dimensions::UNIT_AREA_SQ_METER_FOR_ONE_DIMENSIONAL_CALCS;

impl CartesianConduction1DArray {
    /// calculates the temperature array for the next timestep 
    /// and updates the temperatures and enthalpies of current timestep 
    /// to be that of the next timestep
    pub fn advance_timestep(
        &mut self, timestep: Time) -> Result<(), TuasLibError>{
        
        let inner_single_cv_mut_reference = &mut self.inner_single_cv;
        let outer_single_cv_mut_reference = &mut self.outer_single_cv;
        let inner_nodes = self.inner_nodes;

        // for 1D calcs, we use a basis area
        let basis_area: Area = Area::new::<square_meter>( 
            UNIT_AREA_SQ_METER_FOR_ONE_DIMENSIONAL_CALCS);
        let total_volume = self.total_length*basis_area;

        // this is the same as nodesNumber in the  
        // genfoam code, but I'm reformatting it to make it snake case
        let total_number_of_nodes = inner_nodes + 2;

        // heat supplied to the array is always zero
        let q: Power = Power::new::<watt>(0.0);
        // and the power fraction is always zero too 
        // I used snake case here to rename qFraction as q_fraction
        let q_fraction:Array1<f64> = Array::zeros(total_number_of_nodes);

        // probably want to use a function to obtain the conductances 
        // here
        // 
        let material = self.material_control_volume;
        let pressure = self.pressure_control_volume;
        // temperature array 
        let temperature_array_current_timestep_reference = 
        &self.temperature_array_current_timestep;


        let mut conductance_array: Array1<ThermalConductance> = 
        Self::get_current_timestep_conductance_array(
            material,
            temperature_array_current_timestep_reference,
            pressure,
            self.total_length
        )?;
        let conductance_array_mut_reference = &mut conductance_array;
        let volume_fraction_array_reference = &self.volume_fraction_array;


        // rhoCp array 
        // probably get rid of unwrap later

        let volumetric_heat_capacity_array = 
        Self::get_current_timestep_rho_cp(
            material,
            temperature_array_current_timestep_reference,
            pressure
        )?;

        let new_temperature_array: Array1<ThermodynamicTemperature> = 
        advance_timestep_for_specified_conductance_array_cv(
            inner_single_cv_mut_reference,
            outer_single_cv_mut_reference,
            inner_nodes,
            timestep,
            total_volume,
            q,
            &q_fraction,
            temperature_array_current_timestep_reference,
            conductance_array_mut_reference,
            volume_fraction_array_reference,
            &volumetric_heat_capacity_array
        )?;

        self.temperature_array_next_timestep = 
            new_temperature_array.clone();

        // Todo: probably need to synchronise error types in future
        //

        // I'm calculating the inner and outer CV's new enthalpy
        // at the current timestep 
        //
        // This code block deals with setting temperature and 
        // enthalpies for the two nested control volumes
        {
            let inner_node_enthalpy_next_timestep: AvailableEnergy = 
            try_get_h(
                self.inner_single_cv.material_control_volume,
                new_temperature_array[0],
                self.inner_single_cv.pressure_control_volume)?;

            self.inner_single_cv.current_timestep_control_volume_specific_enthalpy 
                = inner_node_enthalpy_next_timestep;

            // do the same for the outer node
            let outer_node_enthalpy_next_timestep: AvailableEnergy = 
            try_get_h(
                self.outer_single_cv.material_control_volume,
                new_temperature_array[total_number_of_nodes-1],
                self.outer_single_cv.pressure_control_volume)?;

            self.outer_single_cv.current_timestep_control_volume_specific_enthalpy 
                = outer_node_enthalpy_next_timestep;

            // I also need to update the TOld vector 
            // This will ensure that the current temperature of the single 
            // cv node is equal to that of the matrix

            // technically speaking, this isn't necessary here, as I'm already 
            // returning the new temperature vector
            // I'll set it explicitly in the advance_timestep function
            //
            //*TOld = T.mapv(
            //    |temperature_value| {
            //        return temperature_value;
            //    }
            //);
            // set liquid cv mass for both cvs,
            // and clear out both the rate_enthalpy_change_vector  
            // and the max_timestep_vector
            // probably also need to update error types in future
            // so I don't keep using unwrap
            self.inner_single_cv.set_liquid_cv_mass_from_temperature()?;

            self.outer_single_cv.set_liquid_cv_mass_from_temperature()?;

            self.inner_single_cv.clear_vectors()?;
            self.outer_single_cv.clear_vectors()?;
            
        }

        self.temperature_array_current_timestep = new_temperature_array;
        Ok(())
    }


}
