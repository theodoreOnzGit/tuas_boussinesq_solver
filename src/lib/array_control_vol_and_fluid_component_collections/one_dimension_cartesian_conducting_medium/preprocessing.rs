use super::CartesianConduction1DArray;
use ndarray::*;
use uom::si::area::square_meter;
use uom::si::f64::*;
use crate::boussinesq_thermophysical_properties::thermal_conductivity::try_get_kappa_thermal_conductivity;
use crate::boussinesq_thermophysical_properties::volumetric_heat_capacity::try_get_rho_cp;
use crate::boussinesq_thermophysical_properties::Material;
use crate::control_volume_dimensions::UNIT_AREA_SQ_METER_FOR_ONE_DIMENSIONAL_CALCS;
use crate::thermal_hydraulics_error::TuasLibError;
use crate::boussinesq_thermophysical_properties::thermal_diffusivity::try_get_alpha_thermal_diffusivity;


impl CartesianConduction1DArray {

    /// gets the maximum timestep from the one dimensional 
    /// control volume for cartesian conduction
    pub fn get_max_timestep(&mut self, 
    max_temperature_change: TemperatureInterval) -> Result<Time,TuasLibError>{
        
        // start with an empty timestep vector
        let mut max_timestep_vector: Vec<Time> = vec![];

        // let's find alpha, 
        let control_vol_pressure: Pressure = self.pressure_control_volume;
        let control_vol_material: Material = self.material_control_volume;
        let control_vol_temperature_array: Array1<ThermodynamicTemperature> 
        = self.temperature_array_current_timestep.clone();

        // now let's map the alpha 
        // this is quick and dirty cos i used unwrap

        let thermal_diffusivity_array: Array1<DiffusionCoefficient>
        = control_vol_temperature_array.map(
            |temperature_reference| {

                let alpha = try_get_alpha_thermal_diffusivity(control_vol_material, 
                    *temperature_reference, 
                    control_vol_pressure).unwrap();

                alpha
            }

        );

        // let's calculate the internal lengthscale
        let number_of_temperature_nodes: usize = 
        self.inner_nodes + 2;

        let delta_x: Length = self.total_length/
        number_of_temperature_nodes as f64;

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

        let thermal_diffusivity_coeff: DiffusionCoefficient = 
        match thermal_diffusivity_coeff_opt {
            Some(alpha_reference) => *alpha_reference,
            None => {
                // the none case should NOT happen at all, I'm just 
                // otherwise it means that it's impossible to get thermal 
                // diffusivity
                // providing a fallback mechanism
                let bulk_temp =self.get_bulk_temperature().unwrap();

                try_get_alpha_thermal_diffusivity(control_vol_material, 
                    bulk_temp, 
                    control_vol_pressure)?
            },
        };

        // timescales for conduction of this array
        let max_conduction_timescale: Time = max_mesh_fourier_number * 
        delta_x *
        delta_x / 
        thermal_diffusivity_coeff;

        max_timestep_vector.push(max_conduction_timescale);

        // we also need to take into account the timescales of each of 
        // the control volume at the boundaries

        let max_timestep_front_cv: Time = 
        self.outer_single_cv.get_max_timestep(max_temperature_change)?;

        let max_timestep_back_cv: Time = 
        self.inner_single_cv.get_max_timestep(max_temperature_change)?;

        max_timestep_vector.push(max_timestep_front_cv);
        max_timestep_vector.push(max_timestep_back_cv);

        // now we just find the minimum in this max_timestep_vector 
        //
        // this would help to find the timestep vector without using 
        // a manual for loop, though of course, the for loop is more 
        // readable for rust beginners

        //let mut maximum_timestep = Time::new::<second>(75_f64);

        //for time in max_timestep_vector.iter() {
        //    if *time < maximum_timestep {
        //        maximum_timestep = *time;
        //    }
        //} 
        //
        // will probably want to watch the edge BC timesteps as well

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

    pub (in crate)
    fn get_current_timestep_rho_cp(
        material: Material,
        temperature_array_current_timestep_reference: &Array1<ThermodynamicTemperature>,
        pressure: Pressure,
    ) -> 
    Result<Array1<VolumetricHeatCapacity>,TuasLibError>{

        let rho_cp_array:Array1<VolumetricHeatCapacity> 
        = temperature_array_current_timestep_reference.map(
            // here is the closure (function) i shall use
            |temperature_reference: &ThermodynamicTemperature|{
                let temperature = *temperature_reference;

                let rho_cp: VolumetricHeatCapacity 
                = try_get_rho_cp( material, temperature, pressure).unwrap();

                return rho_cp;
            }
        );

        return Ok(rho_cp_array);

    }

    /// This returns a conductance array
    /// based on the following diagram:
    ///
    ///
    ///
    /// Tmax            T[0]           T[1]          T[n-1]         T_ambient
    /// [ignored]       T_innersingleCV             T_outersingleCV [ignored]
    ///   | --- H[0] --- | --- H[1] --- | --- H[2] --- |  --- H[n] --- |  
    ///        [ignored]                                    [ignore]
    ///
    /// Basically, H[0] and H[n] are first and last elements of the 
    /// conductance arrays, these values are ignored because I don't 
    /// want to rewrite code for the advance_timestep function as  
    /// far as possible
    ///
    /// to construct the array, we consider that we have n nodes
    /// connected via thermal resistors 
    ///
    /// for thermal conductivity in 1D cartesian coordinates ,
    /// the thermal conductance is kA/L 
    ///
    /// Where A is the cross sectional area, and L is the distance 
    /// from node to node
    ///
    /// A is determined for 1D cross sectional area using a global constant
    ///
    /// For our case, we assume that the conductance is contributed to 
    /// by two thermal resistances.
    ///
    /// For example H[1] is contributed to by kA/(0.5L) for T[0] and
    /// kA/(0.5L) for T[1]
    ///
    /// The temperatures of T[0] and T[n-1] correspond to surface 
    /// temperatures
    ///
    /// The trick is in calculating L, the node to node distance 
    /// for purposes of calculating thermal resistance 
    ///
    /// consider a four node system:
    ///
    /// surf1 [0]    [1]     [2]     [3]    surf2
    /// | --- * ------- x ------ x ------ * --- |
    ///
    /// |<----------- total_length ------------>|
    ///
    /// Now L would be the full length, however 
    /// the total length of the thermal resistors 
    /// would be slightly less than total_length because we are not 
    /// measuring the full thermal resistance from surface 1 to 
    /// surface 2 
    ///
    /// the full length of the thermal resistors would only account 
    /// for node [0] to node [3]
    ///
    /// so we can find the node to node distance assuming the nodes 
    /// are spaced equally apart and the distance from the boundary 
    /// nodes to surfaces are about 0.5L
    ///
    /// surf1 [0]    [1]     [2]     [3]    surf2
    /// | --- * ------- x ------ x ------ * --- |
    ///  0.5L     L         L        L      0.5L
    /// |<----------- total_length ------------>|
    ///
    /// So the total length is 4L, and 4 is the number of temperature 
    /// nodes
    ///
    /// however, each thermal resistor is connected by a resistance of L
    ///
    ///
    pub (in crate)
    fn get_current_timestep_conductance_array(
        material: Material,
        temperature_array_curnent_timestep_reference: &Array1<ThermodynamicTemperature>,
        pressure: Pressure,
        total_length: Length,
    ) -> 
    Result<Array1<ThermalConductance>,TuasLibError> {

        // for 1D calcs, we use a basis area
        let basis_area: Area = Area::new::<square_meter>( 
            UNIT_AREA_SQ_METER_FOR_ONE_DIMENSIONAL_CALCS);

        // we can obtain a map of thermal conductivity first 

        let thermal_conductivity_array: Array1<ThermalConductivity> 
        = temperature_array_curnent_timestep_reference.map(
            |temperature_reference: &ThermodynamicTemperature| {

                let temperature = *temperature_reference;

                let k: ThermalConductivity 
                = try_get_kappa_thermal_conductivity( 
                    material, temperature, pressure).unwrap();

                return k;
            }
        );
        // now we are dealing with a number of thermal resistors 

        let number_of_temperature_nodes: usize = 
        thermal_conductivity_array.len();

        let number_of_thermal_resistors: usize = 
        number_of_temperature_nodes - 1;

        let delta_x: Length = total_length/
        number_of_temperature_nodes as f64;

        let mut thermal_conductance_vector: Vec<ThermalConductance> 
        = vec![];

        for index in 0..number_of_thermal_resistors {

            // get first thermal resistance at the index
            let thermal_conductance_idx: ThermalConductance
            = thermal_conductivity_array[index] * basis_area/
            (0.5*delta_x);

            let thermal_conductance_idx_plus_one: ThermalConductance 
            = thermal_conductivity_array[index + 1] * basis_area/
            (0.5*delta_x);

            // sum both thermal resistance 

            let total_thermal_resistance = 
            1.0/thermal_conductance_idx + 
            1.0/thermal_conductance_idx_plus_one;

            // final thermal conductance 

            let thermal_conductance_idx_and_idx_plus_one = 
            1.0/total_thermal_resistance;

            // push to conductance vector
            thermal_conductance_vector.push(
            thermal_conductance_idx_and_idx_plus_one);
        }
        
        // now let's make the array with number of thermal 
        // resistors plus 2, the first and last elements 
        // are zero because I want to save effort rewriting the 
        // finite difference code

        let mut thermal_conductance_array: Array1<ThermalConductance> 
        = Array::zeros(number_of_thermal_resistors + 2);

        for (idx, conductance_reference) in 
            thermal_conductance_vector.iter().enumerate() {

                // load conductance array at index plus 1 

                thermal_conductance_array[idx + 1] 
                = *conductance_reference;

        }
        // ok done!
        Ok(thermal_conductance_array)
    }

}
