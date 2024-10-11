use ndarray::*;
use uom::si::f64::*;
use uom::si::thermodynamic_temperature::kelvin;


use crate::boussinesq_thermophysical_properties::prandtl::try_get_prandtl;
use crate::thermal_hydraulics_error::TuasLibError;
use crate::boussinesq_thermophysical_properties::Material;
use crate::boussinesq_thermophysical_properties::thermal_diffusivity::try_get_alpha_thermal_diffusivity;


use super::FluidArray;

impl FluidArray {

    /// gets the maximum timestep from the 
    /// fluid array 

    pub fn get_max_timestep(&mut self,
    max_temperature_change: TemperatureInterval,
    mass_flowrate: MassRate) -> Result<Time, TuasLibError>{

        // for a fluid node, there are two types of time intervals to be 
        // aware of 
        //
        // First is the Courant Number for advection, and second is 
        // the fourier number for conduction 

        // now, the thing is, courant number timestepping should already 
        // be taken into account when getting timestep from single control 
        // volumes at both ends of this vector, so I won't do anything
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


        // let's find alpha, 
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

        let fluid_volume_in_one_node: Volume = 
        self.front_single_cv.volume;

        // now, delta x is based on the cross sectional area 
        // of the control volume, not the node length 
        // 

        let node_length: Length = self.total_length/
        number_of_temperature_nodes as f64;

        let cross_sectional_area: Area = 
        fluid_volume_in_one_node/node_length;

        // characteristic length scale shall be square root of 
        // cross sectional area. 
        //
        // This keeps it quite generic 
        // I'll just leave it for the time being. As long as it's stable 
        // that's fine

        let delta_x: Length = cross_sectional_area.sqrt();

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
        delta_x *
        delta_x / 
        thermal_diffusivity_coeff;

        max_timestep_vector.push(max_conduction_timescale);

        // now, we should technically also get a nusselt number  to help 
        // determine the timestepping 
        //
        // for this, we need to get a reynold's number and prandtl number 
        // bulk fluid prandtl number will be used

        let fluid_prandtl_number = try_get_prandtl(
            control_vol_material,
            bulk_temp,
            control_vol_pressure
        )?;

        let fluid_reynolds_number = self.get_reynolds(mass_flowrate)?;

        // obtain a nusselt number estimate ignoring wall prandtl 
        // number
        let nusselt_estimate_ignoring_wall_prandtl 
        = self.nusselt_correlation.estimate_based_on_prandtl_and_reynolds_no_wall_correction(
            fluid_prandtl_number,
            fluid_reynolds_number)?;

        // now we need a radial conduction timescale 
        // for this, the length estimate at the denominator will be 
        // the cross sectional area 

        let max_radial_condition_timescale: Time = 
        max_mesh_fourier_number * self.xs_area 
        /thermal_diffusivity_coeff;

        let max_solid_fluid_convection_timescale: Time = 
        max_radial_condition_timescale / 
        nusselt_estimate_ignoring_wall_prandtl;

        max_timestep_vector.push(max_solid_fluid_convection_timescale);
        max_timestep_vector.push(max_radial_condition_timescale);

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

    /// gets the reynolds number for this fluid array
    #[inline]
    pub fn get_reynolds(&mut self, 
    mass_flowrate: MassRate,) -> Result<Ratio,
    TuasLibError>{

        let xs_area = self.xs_area;
        let hydraulic_diameter = self.get_hydraulic_diameter();
        let fluid_viscosity = self.get_fluid_viscosity();

        let reynolds = mass_flowrate / xs_area * hydraulic_diameter 
            / fluid_viscosity;
        
        Ok(reynolds)
    }

    /// gets the nusselt number based on reynolds number 
    /// and prandtl number
    ///
    /// also the darcy friction factor in case gnielinski correlation 
    /// requires 
    ///
    /// However, if there is a form loss, note that 
    /// the darcy friction factor used is:
    ///
    /// f_darcy_modified = f + d/L K
    ///
    /// Where d/L is diameter to length ratio and K is form loss
    ///
    #[inline]
    pub fn get_nusselt(&mut self,
        reynolds: Ratio, 
        prandtl_bulk: Ratio,
        prandtl_wall: Ratio) -> Result<Ratio, TuasLibError>{

        // first we need the darcy friction factor in case the Nusselt 
        // correlation is the Gnielinski type 
        //
        // but sometimes, we don't have f_darcy alone, we have:
        //
        // (f L/D + K)
        //
        // So i'm getting that first
        let fldk: Ratio = self
            .fluid_component_loss_properties
            .fldk_based_on_darcy_friction_factor(reynolds)?
            ;

        // I don't really have f_darcy by itself in this case,
        // but I can get the diamter to length and multiply:
        //
        // f + D/L K = (f L/D + K) * D/L

        let length_to_diameter: Ratio = 
            self.get_component_length()/
            self.get_hydraulic_diameter();

        // f + D/L K
        // this is the darcy plus a bit a little bit of form loss

        let darcy_plus_normalised_form_loss: Ratio = 
            fldk/length_to_diameter;

        // for reverse flow, if Re < 0, take absolute 
        let reynolds_abs = reynolds.abs();

        self.nusselt_correlation.
            estimate_based_on_prandtl_darcy_and_reynolds_wall_correction(
                prandtl_bulk,
                prandtl_wall,
                darcy_plus_normalised_form_loss,
                reynolds_abs)
    }
}
