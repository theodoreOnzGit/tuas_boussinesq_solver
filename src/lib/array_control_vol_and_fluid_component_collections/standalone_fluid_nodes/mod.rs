/// deals with fluid nodes in the core region
pub mod core_fluid_node;

/// deals with fluid nodes as if they were in a shell region 
/// that means they are exposed to an inner region and an outer region
pub mod shell_fluid_node;

use ndarray_linalg::{Solve, error};
use uom::si::f64::*;
use uom::ConstZero;
use ndarray::*;
use uom::si::thermodynamic_temperature::kelvin;

/// this basically solves for a temperature vector 
/// given a conductance matrix and power vector
#[inline]
pub fn solve_conductance_matrix_power_vector(
    thermal_conductance_matrix: Array2<ThermalConductance>,
    power_vector: Array1<Power>)
-> Result<Array1<ThermodynamicTemperature>, error::LinalgError>{

    // I can of course convert it into f64 types 
    //
    //

    let get_value_conductance = |conductance: &ThermalConductance| {
        return conductance.value;
    };

    let get_value_power = |power: &Power| {
        return power.value;
    };

    // i'm allowing non snake case so that the syntax is the same as 
    // GeN-Foam
    #[allow(non_snake_case)]
    let M: Array2<f64> = 
    thermal_conductance_matrix.map(get_value_conductance);

    #[allow(non_snake_case)]
    let S: Array1<f64> = power_vector.map(get_value_power);

    // now for the raw temperature matrix 

    #[allow(non_snake_case)]
    let T: Array1<f64> = M.solve(&S)?;

    // To check for unit safety, I can just perform one calc

    let _unit_check: Power = 
    power_vector[0] + 
    thermal_conductance_matrix[[0,0]] 
    * ThermodynamicTemperature::ZERO;

    // now map T back to a ThermodynamicTemperature
    // T is already a ThermodynamicTemperature, so don't need to manually 
    // convert, do it in kelvin 
    //

    let convert_f64_to_kelvin_temperature = |float: &f64| {
        return ThermodynamicTemperature::new::<kelvin>(*float);
    };


    // this is the last step
    let temperature_vector: Array1<ThermodynamicTemperature> 
    = T.map(convert_f64_to_kelvin_temperature);

    return Ok(temperature_vector);
}
