use crate::thermal_hydraulics_error::ThermalHydraulicsLibError;
use crate::boussinesq_thermophysical_properties::
density::try_get_rho;
use crate::boussinesq_thermophysical_properties::
specific_enthalpy::try_get_temperature_from_h;
use crate::boussinesq_thermophysical_properties::specific_heat_capacity
::try_get_cp;
use crate::boussinesq_thermophysical_properties::thermal_diffusivity::
try_get_alpha_thermal_diffusivity;

use super::SingleCVNode;
use uom::si::f64::*;
use uom::si::length::meter;
use uom::si::power::watt;
use uom::si::time::second;
use uom::si::ratio::ratio;
use uom::si::volume_rate::cubic_meter_per_second;

impl SingleCVNode {

    /// this is a function to determine the relevant time scales 
    /// this one is based on conduction, which calculates timescales 
    /// based on mesh fourier number
    /// for stable conduction
    ///
    /// for an uneven volume, it will just take the shortest of 
    /// these lengthscales to determine a proper time scale
    #[inline]
    pub fn calculate_conduction_timestep(&self) -> Result<Time,ThermalHydraulicsLibError>{

        // first let us get relevant length scales 
        // for shortest time step, we will get the shortest length 
        // scale

        let lengthscale_stability_vec = 
        self.mesh_stability_lengthscale_vector.clone();


        // initiate a simple loop to find the shortest length scale
        let mut min_lengthscale = Length::new::<meter>(0.0);

        for length in lengthscale_stability_vec.iter() {
            if *length > min_lengthscale {
                min_lengthscale = *length;
            }
        }

        // now we can determine the conduction time scale regardless 
        // of whether it's solid or liquid (no gas implementations yet, 
        // this is alpha code, probably needs a few rewrites)
        //

        let max_mesh_fourier_number: f64 = 0.25;

        let control_vol_material = self.material_control_volume.clone();
        let control_vol_pressure = self.pressure_control_volume.clone();
        let cv_temperature = self.temperature;


        let thermal_diffusivity_coeff: DiffusionCoefficient = 
        try_get_alpha_thermal_diffusivity(control_vol_material, 
            cv_temperature, 
            control_vol_pressure)?;

        // now let's obtain the minimum timescale for conduction

        let min_conduction_timescale = max_mesh_fourier_number * 
        min_lengthscale *
        min_lengthscale / 
        thermal_diffusivity_coeff;


        return Ok(min_conduction_timescale);
    }


    
    /// calculates timestep based on courant number 
    #[inline]
    pub fn calculate_courant_number_timestep(&mut self,
        max_courant_number: Ratio) 
        -> Result<Time, ThermalHydraulicsLibError>{


        // then let's calculate the dot product
        // it has units of volumetric flowrate
        let mut absolute_sum_of_volumetric_flowrate: VolumeRate
        = VolumeRate::new::<cubic_meter_per_second>(0.0);

        let volume_flowrate_vector = &self.volumetric_flowrate_vector;

        // now if the vector is empty, end the calculation or return 
        // some obscenely high time value

        if volume_flowrate_vector.is_empty() {
            return Err(ThermalHydraulicsLibError::CourantMassFlowVectorEmpty);
        }


        for vol_flowrate_ptr in volume_flowrate_vector.iter() {

            absolute_sum_of_volumetric_flowrate += vol_flowrate_ptr.abs();

        }

        // let's get the volume of this control volume 

        let cv_material_reference = &self.material_control_volume;
        let cv_temperature = self.temperature;
        let cv_pressure_reference = &self.pressure_control_volume;

        let cv_density: MassDensity = 
        try_get_rho(*cv_material_reference,
            cv_temperature,
            *cv_pressure_reference,
        )?;

        let cv_mass_reference = &self.mass_control_volume;

        let cv_volume: Volume = *cv_mass_reference/cv_density;

        // using the OpenFOAM formulation:
        //
        // Co = 0.5 * timestep * vol_flow_sum / cv_volume
        //
        
        // timestep = Co/0.5 * cv_vol / vol_flow_sum

        let timestep: Time = max_courant_number * 2.0 * cv_volume / 
        absolute_sum_of_volumetric_flowrate;

        Ok(timestep)

    }

    


    /// appends timestep constrained to fourier number stability
    #[inline]
    pub fn append_conduction_mesh_stability_timestep(&mut self, 
        lengthscale: Length) -> Result<Time, ThermalHydraulicsLibError> {

        // now we can determine the conduction time scale regardless 
        // of whether it's solid or liquid (no gas implementations yet, 
        // this is alpha code, probably needs a few rewrites)
        //

        let max_mesh_fourier_number: f64 = 0.25;

        let control_vol_material = self.material_control_volume.clone();
        let control_vol_pressure = self.pressure_control_volume.clone();
        let cv_temperature = try_get_temperature_from_h(
            self.material_control_volume, 
            self.current_timestep_control_volume_specific_enthalpy, 
            self.pressure_control_volume)?;


        let thermal_diffusivity_coeff: DiffusionCoefficient = 
        try_get_alpha_thermal_diffusivity(control_vol_material, 
            cv_temperature, 
            control_vol_pressure)?;

        // now let's obtain the minimum timescale for conduction

        let min_conduction_timescale = max_mesh_fourier_number * 
        lengthscale *
        lengthscale / 
        thermal_diffusivity_coeff;


        return Ok(min_conduction_timescale);
    }
    

    /// calculates a time scale based on maximum temperature change 
    /// within one time step, 
    #[inline]
    fn get_max_timestep_based_on_max_temperature_change(
        &self,
        max_temperature_change: TemperatureInterval) 
    -> Result<Time,ThermalHydraulicsLibError> {

        // let's get the net power change first 

        // first thing is to sum up all enthalpy rate of change 
        let mut total_enthalpy_rate_change = 
        Power::new::<watt>(0.0);

        for enthalpy_chg_rate in 
            self.rate_enthalpy_change_vector.clone().iter() {

                total_enthalpy_rate_change += *enthalpy_chg_rate;
            }

        // we have Q = m c_p Delta T
        // Q = power * time 
        //
        // power * time = m c_p  Delta T
        //
        // time = m c_p Delta T / power 
        // for small temperature changes, assume cp constant
        //
        // algorithm is okay if the enthalpy change is positive ,
        // but if negative, then we need to take the absolute value 

        total_enthalpy_rate_change = total_enthalpy_rate_change.abs();

        let cv_mass_clone = self.mass_control_volume.clone();
        let cv_material = self.material_control_volume.clone();
        let cv_temperature = self.temperature;
        let cv_pressure = self.pressure_control_volume.clone();

        let cv_heat_capacity = try_get_cp(
            cv_material,
            cv_temperature,
            cv_pressure,
        )?;

        // now we can calculate time step

        let time_step_based_on_max_temperature_change = 
        cv_mass_clone * cv_heat_capacity * max_temperature_change / 
        total_enthalpy_rate_change;

        // now return it to the environment 

        return Ok(time_step_based_on_max_temperature_change);

    }

    /// compiles a list of time steps based on various criteria, 
    /// such as temperature change, courant number, fourier number 
    /// and so on
    ///
    #[inline]
    pub fn get_max_timestep(&mut self, 
        max_temperature_change: TemperatureInterval) -> Result<Time, ThermalHydraulicsLibError> {

        // let's calculate conduction time step first, based on 
        // thermal diffusivity and fourier number
        let conduction_timestep = self.calculate_conduction_timestep()?;

        // next I want to push it to the vector 

        self.max_timestep_vector.push(conduction_timestep);


        // last but not least, check if temperature changes exceed a 
        // certain mount, recommend 0.5 C or 1 C for max temperature 
        // change in one time step
        //
        // inspired by GeN-Foam and other papers.
        
        let max_temperature_change_timestep: Time = 
        self.get_max_timestep_based_on_max_temperature_change(
            max_temperature_change)?;

        self.max_timestep_vector.push(max_temperature_change_timestep);

        // we still need the timestep changes from the control volumes 
        // linked to this cv.

        // now, we can also get the courant number based timestep
        // our max Co is 1 

        let max_courant_number: Ratio = Ratio::new::<ratio>(1.0);

        let courant_number_timestep_result = 
        self.calculate_courant_number_timestep(max_courant_number);

        let courant_number_timescale: Time = match courant_number_timestep_result {
            Ok(timescale) => timescale,
            Err(error) => {
                match error {
                    ThermalHydraulicsLibError::CourantMassFlowVectorEmpty => {

                        // just return a large timestep for an empty
                        // vector
                        Time::new::<second>(80_f64)
                    },
                    _ => {
                        return Err(error.into());
                    }
                }

            }
        };

        self.max_timestep_vector.push(courant_number_timescale);
        

        // initiate a simple loop to find the shortest time scale
        // that is "safe" out of all time steps
        let mut min_timescale = Time::new::<second>(75_f64);

        for time in self.max_timestep_vector.iter() {
            if *time < min_timescale {
                min_timescale = *time;
            }
        }
        return Ok(min_timescale);

    }
}
