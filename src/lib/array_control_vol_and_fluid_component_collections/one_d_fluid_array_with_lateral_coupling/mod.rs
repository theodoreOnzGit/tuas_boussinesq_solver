use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
use crate::single_control_vol::SingleCVNode;
use crate::boussinesq_thermophysical_properties::Material;
use crate::boussinesq_thermophysical_properties::specific_enthalpy::try_get_h;
use uom::si::f64::*;
use ndarray::*;

use crate::tuas_lib_error::TuasLibError;
use ndarray_linalg::error::LinalgError;

use self::fluid_component_calculation::DimensionlessDarcyLossCorrelations;


/// this is essentially a 1D pipe array containing two CVs 
/// and two other laterally connected arrays
/// (it's essentially a generic solid array representing heat 
/// structures with mainly axial conduction and radial conduction)
///
/// it can be used to represent rods, or cylindrical shells
/// in the latter case, the Column is hollow so to speak
///
/// Usually, these will be nested inside a heat transfer component 
/// and then be used
///
/// Within this array, the implicit Euler Scheme is used
///
/// You must supply the number of nodes for the fluid array
/// Note that the front and back cv count as one node
#[derive(Debug,Clone,PartialEq)]
pub struct FluidArray {

    /// represents the control volume at the back 
    /// imagine a car or train cruising along in a positive x direction
    ///
    /// //----------------------------------------------> x 
    ///
    /// //            (back --- train/car --- front)
    /// //            lower x                 higher x
    ///
    pub back_single_cv: SingleCVNode,

    /// represents the control volume at the front
    ///
    /// to think of which is front and back, we think of coordinates 
    /// imagine a car or train cruising along in a positive x direction
    ///
    /// //----------------------------------------------> x 
    ///
    /// //            (back --- train/car --- front)
    /// //            lower x                 higher x
    ///
    pub front_single_cv: SingleCVNode,

    /// number of inner nodes ,
    /// besides the back and front node 
    ///
    /// total number of nodes is inner_nodes + 2
    inner_nodes: usize,


    // total length for the array
    total_length: Length,

    // cross sectional area for the 1D array, assumed to be uniform 
    pub (crate) xs_area: Area,

    /// temperature array current timestep 
    /// only accessible via get and set methods
    pub (crate) temperature_array_current_timestep: Array1<ThermodynamicTemperature>,

    /// control volume material 
    pub material_control_volume: Material,

    /// control volume pressure 
    pub pressure_control_volume: Pressure,

    // volume fraction array 
    pub (crate) volume_fraction_array: Array1<f64>,

    /// mass flowrate through the fluid array, 
    mass_flowrate: MassRate,

    /// pressure loss term 
    pressure_loss: Pressure,

    /// wetted perimeter (for hydraulic diameter) 
    wetted_perimeter: Length,

    /// incline angle 
    incline_angle: Angle,

    /// internal pressure source 
    internal_pressure_source: Pressure,

    /// fluid component loss properties 
    /// be it for pipe or something else
    pub fluid_component_loss_properties: DimensionlessDarcyLossCorrelations,

    /// nusselt correlation 
    pub nusselt_correlation: NusseltCorrelation,

    /// now fluid arrays can be connected to solid arrays 
    /// or other fluid arrays adjacent to it radially
    ///
    /// There will be no advection in the radial direction,
    /// but there can be thermal conductance shared between the nodes 
    ///
    /// hence, I only want to have a copy of the temperature 
    /// arrays radially adjacent to it
    ///
    /// plus their thermal resistances
    /// N is the array size, which is known at compile time

    pub lateral_adjacent_array_temperature_vector: 
    Vec<Array1<ThermodynamicTemperature>>,

    /// now fluid arrays can be connected to solid arrays 
    /// or other fluid arrays adjacent to it radially
    ///
    /// There will be no advection in the radial direction,
    /// but there can be thermal conductance shared between the nodes 
    ///
    /// hence, I only want to have a copy of the temperature 
    /// arrays radially adjacent to it
    ///
    /// plus their thermal resistances 
    /// N is the array size, which is known at compile time
    pub lateral_adjacent_array_conductance_vector:
    Vec<Array1<ThermalConductance>>,

    /// fluid arrays can also be connected to heat sources 
    /// or have specified volumetric heat sources 
    pub q_vector: Vec<Power>,

    /// fluid arrays should have their power distributed according 
    /// to their nodes 
    pub q_fraction_vector: Vec<Array1<f64>>,

}

impl FluidArray {



    /// obtains a clone of the temperature array in Array1 ndarray 
    /// form 
    pub fn get_temperature_array(&self) -> Result< 
    Array1<ThermodynamicTemperature>, TuasLibError> {

        // converts the fixed sized temperature array (at compile time) 
        // into a dynamically sized ndarray type so we can use solve
        // methods
        let mut temperature_arr: Array1<ThermodynamicTemperature> = 
        Array1::default(self.len());

        for (idx,temperature) in 
            self.temperature_array_current_timestep.iter().enumerate() {
                temperature_arr[idx] = *temperature;
        }

        Ok(temperature_arr)


    }


    /// sets the temperature vector to a 
    pub fn set_temperature_vector(&mut self,
    temperature_vec: Vec<ThermodynamicTemperature>) -> Result<(), TuasLibError>{

        let number_of_temperature_nodes = self.len();

        // check if temperature_vec has the correct number_of_temperature_nodes

        if temperature_vec.len() !=  number_of_temperature_nodes {
            let shape_error = ShapeError::from_kind(
                ErrorKind::IncompatibleShape
            );

            let linalg_error = LinalgError::Shape(shape_error);

            return Err(TuasLibError::LinalgError
                (linalg_error));

        }

        for (index,temperature) in 
            self.temperature_array_current_timestep.iter_mut().enumerate() {
            *temperature = temperature_vec[index];
        }

        // we also need to ensure that the front and end nodes are 
        // properly synchronised in terms of temperature
        //

        let back_cv_temperature: ThermodynamicTemperature 
        = temperature_vec[0];

        let front_cv_temperature: ThermodynamicTemperature 
        = *temperature_vec.last().unwrap();


        // update enthalpies of control volumes withing

        let material = self.material_control_volume;
        let pressure = self.pressure_control_volume;

        let back_cv_enthalpy = try_get_h(
            material,
            back_cv_temperature,
            pressure
        )?;

        let front_cv_enthalpy = try_get_h(
            material,
            front_cv_temperature,
            pressure
        )?;

        self.back_single_cv.current_timestep_control_volume_specific_enthalpy
            = back_cv_enthalpy;

        self.front_single_cv.current_timestep_control_volume_specific_enthalpy
            = front_cv_enthalpy;


        
        Ok(())
    }

    /// obtains a clone of the temperature array in Array1 ndarray 
    /// form 
    pub fn set_temperature_array(&mut self,
    temperature_arr: Array1<ThermodynamicTemperature>) -> Result<(),
    TuasLibError> {

        // we'll convert the temperature array into vector form 
        // and use the existing method 
        let mut temperature_vec: Vec<ThermodynamicTemperature> 
        = vec![];

        for temperature in temperature_arr.iter() {
            temperature_vec.push(*temperature)
        }

        self.set_temperature_vector(temperature_vec)


    }

    /// length of the fluid array 
    pub fn len(&self) -> usize {
        self.inner_nodes + 2
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


/// contains code to connect control volumes laterally,
/// in a cylindrical situation, it means radially 
pub mod lateral_connection;


/// contains code to connect to other array cvs, other boundary conditions 
/// or other single cvs
pub mod axial_connection;

/// defaults 
pub mod default;


/// constructors  
pub mod constructors;

/// fluid component calculations 
/// with the DimensionlessDarcyLossCorrelations
pub mod fluid_component_calculation;

/// type conversion 
pub mod type_conversion;
