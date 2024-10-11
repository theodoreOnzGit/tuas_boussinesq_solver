use crate::boussinesq_thermophysical_properties::thermal_diffusivity::try_get_alpha_thermal_diffusivity;
use crate::boussinesq_thermophysical_properties::Material;
use uom::si::f64::*;
use ndarray::*;
use uom::si::thermodynamic_temperature::kelvin;

use crate::thermal_hydraulics_error::TuasLibError;

use super::SolidColumn;

impl SolidColumn {

    /// gets the maximum timestep from the 
    /// solid array 

    pub fn get_max_timestep(&mut self,
    max_temperature_change: TemperatureInterval) 
    -> Result<Time, TuasLibError>{

        // for a solid array, there are two types of time intervals to be 
        // aware of 
        //
        // these are just the fourier numbers in the axial and 
        // lateral direction

        //
        let mut max_timestep_vector: Vec<Time> = vec![];

        // timestep values for two single cvs at both boundaries 

        let max_timestep_front_cv: Time = 
        self.front_single_cv.get_max_timestep(max_temperature_change)?;

        let max_timestep_back_cv: Time = 
        self.back_single_cv.get_max_timestep(max_temperature_change)?;

        max_timestep_vector.push(max_timestep_front_cv);
        max_timestep_vector.push(max_timestep_back_cv);

        // secondly we have fourier number based timestepping 
        // of course, fourier number is traditionally used for conduction 
        // here we have convection timescales between solid and fluid 
        //


        // let's find alpha, which is thermal diffusivity
        let control_vol_pressure: Pressure = self.pressure_control_volume;
        let control_vol_material: Material = self.material_control_volume;
        let control_vol_temperature_array: Array1<ThermodynamicTemperature> 
        = self.get_temperature_array()?;

        // now let's map the alpha 
        // this is quick and dirty cos i used unwrap

        let thermal_diffusivity_array: Array1<DiffusionCoefficient>
        = control_vol_temperature_array.map(
            |temperature_reference| {

                try_get_alpha_thermal_diffusivity(control_vol_material, 
                    *temperature_reference, 
                    control_vol_pressure).unwrap()
            }

        );
        // let's calculate the internal lengthscale
        let number_of_temperature_nodes: usize = 
        self.len();

        let solid_volume_in_one_node: Volume = 
        self.front_single_cv.volume;

        // now, delta x is based two scales, 
        // the axial length scale and the radial length scale 
        //
        // the shorter of the two will determine the appropriate 
        // time step for conduction 
        // interactions 
        let delta_x_axial: Length = self.total_length/
        number_of_temperature_nodes as f64;

        let cross_sectional_area: Area = 
        solid_volume_in_one_node/delta_x_axial;

        // characteristic lateral 
        // length scale shall be square root of 
        // cross sectional area. 
        //
        // This keeps it quite generic 
        // I'll just leave it for the time being. As long as it's stable 
        // that's fine

        let delta_x_lateral: Length = cross_sectional_area.sqrt();

        // the minimum lengthscale is the minimum of these two 

        let mut delta_x_minimum: Length = delta_x_lateral;
        
        if delta_x_lateral > delta_x_axial {
            delta_x_minimum = delta_x_axial;
        }

        // now for the array CV, implicit schemes are used. Therefore,
        // the threshold for stability is higher, at 1.0 
        // out of some caution, let me use 0.8
        let max_mesh_fourier_number: f64 = 0.8;

        // for the minimum conduction timescale, we need the 
        // maximum alpha
        //
        // I'm using this closure to find the maximum, rather than 
        // a manual for loop
        let thermal_diffusivity_coeff_opt 
        = thermal_diffusivity_array.iter().max_by(
            |alpha_1, alpha_2| {
                // a and b represent two typical items in the array 
                let a = &alpha_1.value;
                let b = &alpha_2.value;
                a.total_cmp(b)
            });

        let bulk_temp = self.try_get_bulk_temperature()?;
        let thermal_diffusivity_coeff: DiffusionCoefficient = 
        match thermal_diffusivity_coeff_opt {
            Some(alpha_reference) => *alpha_reference,
            None => {
                // the none case should NOT happen at all, I'm just 
                // otherwise it means that it's impossible to get thermal 
                // diffusivity
                // providing a fallback mechanism

                try_get_alpha_thermal_diffusivity(control_vol_material, 
                    bulk_temp, 
                    control_vol_pressure).unwrap()
            },
        };

        // timescales for conduction of this array
        let max_conduction_timescale: Time = max_mesh_fourier_number * 
        delta_x_minimum *
        delta_x_minimum / 
        thermal_diffusivity_coeff;

        max_timestep_vector.push(max_conduction_timescale);

        // lets get the maximum timestep

        let maximum_timestep: Time = 
        *max_timestep_vector.iter().min_by(
            |time_1, time_2| {
                // a and b represent two typical items in the array 
                let a = &time_1.value;
                let b = &time_2.value;
                a.partial_cmp(b).unwrap()
            }).unwrap();

        // all right done!

        return Ok(maximum_timestep);
    }
    /// gets bulk temperature of the array cv based on volume fraction 
    /// now, for solid and liquid, that would be sort of 
    /// a good approximation since the boussinesq approximation
    /// may work well for liquids
    ///
    #[inline]
    pub fn try_get_bulk_temperature(&mut self) -> 
    Result<ThermodynamicTemperature,TuasLibError>{

        // for now, doing it quick and dirty, i'm going to obtain a volume 
        // averaged temperature 

        let volume_fraction_array_reference = 
        &self.volume_fraction_array;
        let temperature_array_reference = 
        &self.temperature_array_current_timestep;

        let mut vol_averaged_temperature_array_values: Array1<f64> 
        = Array::default(temperature_array_reference.len());

        for (idx, temperature_reference) in 
            temperature_array_reference.iter().enumerate() {
                //get the vol fraction 

                let vol_fraction: f64 = 
                volume_fraction_array_reference[idx];

                let vol_avg_temperature_component: f64
                = vol_fraction * (temperature_reference.get::<kelvin>());

                vol_averaged_temperature_array_values[idx] = 
                    vol_avg_temperature_component;

            }

        // sum it all up (these are float values) 

        let vol_averaged_temperature_kelvin: f64 
        = vol_averaged_temperature_array_values.sum();

        return Ok(ThermodynamicTemperature::new
            ::<kelvin>(vol_averaged_temperature_kelvin));


    }

    /// obtains length of the array
    #[inline]
    pub fn get_component_length(&self) -> Length {
        self.total_length
    }


    /// obtains cross sectional area of the array
    #[inline]
    pub fn get_component_xs_area(&self) -> Area {
        self.xs_area
    }

}
