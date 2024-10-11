use approx::assert_relative_eq;
use ndarray::*;
use uom::si::f64::*;
use uom::si::thermodynamic_temperature::kelvin;

use crate::single_control_vol::SingleCVNode;
use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
use crate::boussinesq_thermophysical_properties::Material;

use crate::tuas_lib_error::TuasLibError;


/// for 1D Cartesian Conduction array,
/// it is essentially an array control volume of one homogeneous 
/// material
///
/// it is in Cartesian coordinates, basically, x direction only conduction 
///
/// the structure is segregated into several smaller nodes using finite 
/// difference methods 
///
/// I'll use lumped_nuclear_structure_inspired_functions to calculate 
/// new temperatures for this structure 
///
/// the scheme used is the implicit Euler scheme to calculate new 
/// temperatures. However, material properties are calculated using 
/// current timestep temperatures rather than next timestep temperatures 
/// therefore, it is more of a hybrid between the implicit and explicit 
/// schemes. 
///
/// the important methods are to advance timestep, and to update 
/// material properties at every timestep
#[derive(Debug,Clone,PartialEq,Default)]
pub struct CartesianConduction1DArray {


    /// represents the inner (lower r) control volume end 
    /// or back (lower x) control volume
    /// to think of which is front and back, we think of coordinates 
    /// imagine a car or train cruising along in a positive x direction
    ///
    /// //----------------------------------------------> x 
    ///
    /// //            (back --- train/car --- front)
    /// //            lower x                 higher x
    ///
    pub inner_single_cv: SingleCVNode,
    /// represents the outer (higher r) control volume end 
    /// or front (higher x) control volume
    ///
    /// to think of which is front and back, we think of coordinates 
    /// imagine a car or train cruising along in a positive x direction
    ///
    /// //----------------------------------------------> x 
    ///
    /// //            (back --- train/car --- front)
    /// //            lower x                 higher x
    ///
    pub outer_single_cv: SingleCVNode,

    // number of ADDITIONAL nodes within the array 
    // in addition to the inner and outer single cvs
    inner_nodes: usize,
    
    // total length for the 1D array
    total_length: Length,

    // volume fraction array 
    volume_fraction_array: Array1<f64>,

    /// temperature array current timestep 
    pub temperature_array_current_timestep: Array1<ThermodynamicTemperature>,

    // temperature_array_next timestep 
    temperature_array_next_timestep: Array1<ThermodynamicTemperature>,

    /// control volume material 
    pub material_control_volume: Material,

    /// control volume pressure 
    pub pressure_control_volume: Pressure,

    /// nusselt correlation type 
    nusselt_correlation_type: NusseltCorrelation
}

/// here, i mostly do constructors
impl CartesianConduction1DArray {

    /// constructs a new instance of the CartesianConduction1DArray
    pub fn new(material: Material,
    initial_uniform_temperature: ThermodynamicTemperature,
    uniform_pressure: Pressure,
    inner_nodes: usize,
    total_length: Length) -> Result<Self,TuasLibError> {
        // we start building the 1Darray object by a default first
        let mut array_to_return = Self::default();

        // set the scalars first, they are the easiest
        array_to_return.material_control_volume = material;
        array_to_return.inner_nodes = inner_nodes;
        array_to_return.pressure_control_volume = uniform_pressure;
        array_to_return.total_length = total_length;

        // by now, we should be able to set the volume fraction array 
        let vol_frac_array = 
            array_to_return.construct_volume_fraction_array()?;

        array_to_return.volume_fraction_array = vol_frac_array.clone();

        // set the temperature arrays
        let number_of_temperature_nodes: usize = inner_nodes + 2;

        let mut initial_temperature_array: 
        Array1<ThermodynamicTemperature> = 
            Array::default(number_of_temperature_nodes);

        initial_temperature_array.fill(initial_uniform_temperature);

        array_to_return.temperature_array_current_timestep = 
            initial_temperature_array.clone();

        array_to_return.temperature_array_next_timestep = 
            initial_temperature_array;

        // lets set the remaining control volumes
        // 
        // for a 1D volume, the volume fraction is the same 
        // as the length fraction
        let boundary_length: Length = vol_frac_array[0] * total_length;

        let boundary_cv: SingleCVNode = 
        SingleCVNode::new_one_dimension_volume(
            boundary_length,
            material,
            initial_uniform_temperature,
            uniform_pressure)?;


        array_to_return.inner_single_cv = boundary_cv.clone();
        array_to_return.outer_single_cv = boundary_cv.clone();

        // now package as an array cv 


        return Ok(array_to_return);
    }

    /// constructor method which helps set the volume fraction array
    /// This returns a volume fraction array
    /// based on the following diagram:
    ///
    ///
    ///
    /// Tmax            T[0]           T[1]          T[n-1]         T_ambient
    /// [ignored]       T_innersingleCV             T_outersingleCV [ignored]
    ///   | --- H[0] --- | --- H[1] --- | --- H[2] --- |  --- H[n] --- |  
    ///        [ignored]                                    [ignore]
    /// 
    ///
    /// Between each temperature node, there is a thermal resistor 
    /// each resistor has resistance equivalent to delta x 
    /// long 
    ///
    /// where delta x = total length / number of resistors

    /// the volume fraction rray helps to determine the thermal inertia 
    /// of each node 
    /// now each node would have a thermal inertia corresponding to 
    /// a thickness of delta_x 
    ///
    ///
    /// However, the nodes at both boundaries would have a length 
    /// equivalent to half of delta_x because they are at the boundaries
    /// This would only apply to the thermal resistor
    ///
    /// In terms of thermal inertia, this would not make a difference 
    /// because the last node would have the same thermal inertia as 
    /// any other node. Therefore, length is evenly divided among all 
    /// nodes
    ///
    /// Consider again a four node system with the thermal resistors 
    /// of length L within the bulk and 0.5L at the boundary
    ///
    /// surf1 [0]    [1]     [2]     [3]    surf2
    /// | --- * ------- x ------ x ------ * --- |
    ///  0.5L     L         L        L      0.5L
    /// |<----------- total_length ------------>|
    ///
    /// We must account for the full thermal inertia of the system,
    /// and therefore, the whole length is taken into account. 
    /// The thermal inertia of each array is represented by length L
    ///
    ///
    pub (in crate) fn construct_volume_fraction_array(&mut self) 
    -> Result<Array1<f64>, TuasLibError> {

        let number_of_temperature_nodes: usize = 
        self.inner_nodes + 2;

        let delta_x: Length = self.total_length/
        number_of_temperature_nodes as f64;

        // now we can compute the volume fraction based on length 
        // because the basis cross sectional area is the same 
        // throughout

        let volume_fraction_per_inner_node: f64 = 
        (delta_x/self.total_length).value;

        let mut volume_fraction_array: Array1<f64> = 
        Array::zeros(number_of_temperature_nodes);

        // set all volume fractions to the inner node volume fractions
        volume_fraction_array.fill(volume_fraction_per_inner_node);

        // assert if they add up to 1.0

        let vol_fraction_sum: f64 = volume_fraction_array.sum();
        assert_relative_eq!(
            1.0,
            vol_fraction_sum,
            epsilon = 0.001);

        return Ok(volume_fraction_array);
    }

    /// gets bulk temperature of the array cv based on volume fraction 
    /// now, for solid and liquid, that would be sort of 
    /// a good approximation since the boussinesq approximation
    /// may work well for liquids
    ///
    #[inline]
    pub fn get_bulk_temperature(&mut self) -> 
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

}


/// Functions or methods to retrieve temperature and other such 
/// data from the array_cv
pub mod postprocessing;


/// Functions or methods to get timestep and other such quantiies 
/// for calculations 
///
/// helps to set up quantities used in calculation step
pub mod preprocessing;
 

/// Contains functions which advance the timestep
/// it's the bulk of calculation
pub mod calculation;

