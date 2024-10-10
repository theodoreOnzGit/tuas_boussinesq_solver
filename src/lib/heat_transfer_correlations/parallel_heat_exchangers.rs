use uom::si::f64::*;
use crate::thermal_hydraulics_error::ThermalHydraulicsLibError;
use crate::heat_transfer_correlations::
thermal_resistance::subtract_two_thermodynamic_temperatures;
/// LMTD = (delta T in - delta T out) / (ln delta T in - ln delta T out)
///
/// note that reversing the order of delta T in and out doesn't really
/// matter, as long as both numerator and denominator are reversed
/// correctly
///
/// However, hot fluid temperatures and cold fluid temperature CANNOT
/// be mixed up, otherwise the logarithms will return an error
///
/// ```rust
/// extern crate approx;
/// use tuas_boussinesq_solver::heat_transfer_correlations::
/// parallel_heat_exchangers::log_mean_temperature_difference;
/// 
///
/// use uom::si::{temperature_interval, thermodynamic_temperature};
/// use uom::si::f64::*;
///
/// let cold_fluid_temp_A = ThermodynamicTemperature::new::
/// <thermodynamic_temperature::degree_celsius>(21.0);
///
/// let cold_fluid_temp_B = ThermodynamicTemperature::new::
/// <thermodynamic_temperature::degree_celsius>(20.0);
///
/// let hot_fluid_temp_A = ThermodynamicTemperature::new::
/// <thermodynamic_temperature::degree_celsius>(48.0);
///
/// let hot_fluid_temp_B = ThermodynamicTemperature::new::
/// <thermodynamic_temperature::degree_celsius>(50.0);
///
/// let A_temperature_interval_value : f64 = hot_fluid_temp_A.value - 
/// cold_fluid_temp_A.value;
///
/// let B_temperature_interval_value : f64 = hot_fluid_temp_B.value - 
/// cold_fluid_temp_B.value;
///
/// let mut LMTD_value_expected = 
/// (A_temperature_interval_value - B_temperature_interval_value)/
/// (A_temperature_interval_value.ln() - 
/// B_temperature_interval_value.ln());
///
/// let LMTD_test = log_mean_temperature_difference(
/// cold_fluid_temp_A,
/// cold_fluid_temp_B,
/// hot_fluid_temp_A,
/// hot_fluid_temp_B).unwrap();
///
///
/// approx::assert_relative_eq!(LMTD_value_expected, LMTD_test.value, 
/// max_relative=0.001);
///
/// // test 2 makes it more obvious
///
/// let mut LMTD_value_expected = 
/// ((48_f64 - 21_f64) - (50_f64-20.0))/
/// ((48_f64-21_f64).ln() - 
/// (50_f64-20.0).ln());
///
/// approx::assert_relative_eq!(LMTD_value_expected, LMTD_test.value, 
/// max_relative=0.001);
///
/// ```
pub fn log_mean_temperature_difference(
    temp_cold_fluid_a: ThermodynamicTemperature,
    temp_cold_fluid_b: ThermodynamicTemperature,
    temp_hot_fluid_a: ThermodynamicTemperature,
    temp_hot_fluid_b: ThermodynamicTemperature) -> 
Result<TemperatureInterval,ThermalHydraulicsLibError> {

    if temp_hot_fluid_a.value < temp_cold_fluid_a.value {
        panic!("LMTD: hot fluid temperature input colder than \n
               cold fluid temperature input")
    }

    if temp_hot_fluid_b.value < temp_cold_fluid_b.value {
        panic!("LMTD: hot fluid temperature input colder than \n
               cold fluid temperature input")
    }
    
    let a_temperature_interval = 
        subtract_two_thermodynamic_temperatures(
            temp_hot_fluid_a, temp_cold_fluid_a);

    let b_temperature_interval = 
        subtract_two_thermodynamic_temperatures(
            temp_hot_fluid_b, temp_cold_fluid_b);

    let numerator = a_temperature_interval - b_temperature_interval;

    let denominator = a_temperature_interval.value.ln() - 
        b_temperature_interval.value.ln();


    return Ok(numerator/denominator);

}

/// calculate overall heat flux power input based on lmtd
///
/// assuming a fixed surrounding temperature
///
/// calculates heat INPUT into fluid based on surrounding temperature
/// estimated fluid inlet and fluid outlet temperature
///
/// Q = U * A * LMTD
///
/// LMTD = (delta T in - delta T out) / (ln delta T in - ln delta T out)
///
/// U is overall_heat_transfer_coeff
///
/// A is the surface_area 
/// The surface_area you use can be the surface area of the inner 
/// or outer region of the pipe. BUT, the overall_heat_transfer_coeff 
/// must be adjusted accordingly
///
///
pub fn calculate_lmtd_heat_flux_based_on_ambient_temp(
    overall_heat_transfer_coeff : HeatTransfer,
    ambient_temperature : ThermodynamicTemperature,
    fluid_temperature_in : ThermodynamicTemperature,
    fluid_temperature_out: ThermodynamicTemperature,
    surface_area : Area) -> Result<Power,ThermalHydraulicsLibError> {

    if fluid_temperature_in.value == fluid_temperature_out.value {
        return Ok(overall_heat_transfer_coeff * surface_area * 
            subtract_two_thermodynamic_temperatures(
            ambient_temperature , fluid_temperature_in));
    }
    // note, i do this to calculate
    // delta T = ambient_temperature - fluid_temperature
    let log_mean_temp_diff = 
        log_mean_temperature_difference(
            ambient_temperature,
            ambient_temperature, 
            fluid_temperature_in,
            fluid_temperature_out)?;

    return Ok(
        overall_heat_transfer_coeff * (log_mean_temp_diff) * surface_area); 

}
