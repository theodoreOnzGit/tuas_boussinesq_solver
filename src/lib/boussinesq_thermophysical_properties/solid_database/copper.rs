use peroxide::fuga::{Calculus, CubicSpline, Spline};
use roots::{find_root_brent, SimpleConvergency};
use specific_enthalpy::try_get_h;
use uom::si::available_energy::joule_per_kilogram;
use uom::si::f64::*;
use uom::si::length::micrometer;
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::pressure::atmosphere;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::thermal_conductivity::watt_per_meter_kelvin;
use crate::boussinesq_thermophysical_properties::*;
use crate::tuas_lib_error::TuasLibError;
use uom::si::thermodynamic_temperature::kelvin;

/// density ranges not quite given in original text 
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code validation 
/// using the compact integral effects test (CIET) experimental data. 
/// No. ANL/NSE-19/11. 
/// Argonne National Lab.(ANL), Argonne, IL (United States), 2019.
#[inline]
pub fn copper_density() -> Result<MassDensity,TuasLibError> {
    return Ok(MassDensity::new::<kilogram_per_cubic_meter>(8940.0));
}

/// Arenales, M. R. M., Kumar, S., 
/// Kuo, L. S., & Chen, P. H. (2020). 
/// Surface roughness variation effects on copper tubes in 
/// pool boiling of water. International Journal of 
/// Heat and Mass Transfer, 151, 119399.
pub fn copper_surf_roughness() -> Length {
    Length::new::<micrometer>(0.544)
}
/// returns thermal conductivity of copper
/// cited from:
/// Zou, L., Hu, R., & Charpentier, A. (2019). SAM code 
/// validation using the compact integral effects test (CIET) experimental 
/// data (No. ANL/NSE-19/11). Argonne National 
/// Lab.(ANL), Argonne, IL (United States).
#[inline]
pub fn copper_specific_heat_capacity_zou_zweibaum_spline(
    temperature: ThermodynamicTemperature) -> Result<SpecificHeatCapacity,TuasLibError> {

    range_check(
        &Material::Solid(SolidMaterial::Copper),
        temperature, 
        ThermodynamicTemperature::new::<kelvin>(1000.0), 
        ThermodynamicTemperature::new::<kelvin>(200.0))?;

    let temperature_value_kelvin: f64 = temperature.get::<kelvin>();
    // here we use a cubic spline to interpolate the values
    // it's a little calculation heavy
    //
    // and actually, rebuilding the spline is quite problematic
    // we need to build it ONCE and read from it
    //
    let thermal_cond_temperature_values_kelvin = c!(200.0, 
        250.0, 300.0, 350.0, 
        400.0, 500.0, 1000.0);
    let specific_heat_capacity_values_joule_per_kilogram_kelvin = c!(355.7047,
        373.6018, 384.7875, 392.6174,
        398.2103, 407.1588, 417.2260);

    let s = CubicSpline::from_nodes(&thermal_cond_temperature_values_kelvin, 
        &specific_heat_capacity_values_joule_per_kilogram_kelvin);

    let copper_specific_heat_capacity_value = s.unwrap().
        eval(temperature_value_kelvin);

    Ok(SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(
        copper_specific_heat_capacity_value))

}
/// returns thermal conductivity of copper
/// cited from:
/// Zou, L., Hu, R., & Charpentier, A. (2019). SAM code 
/// validation using the compact integral effects test (CIET) experimental 
/// data (No. ANL/NSE-19/11). Argonne National 
/// Lab.(ANL), Argonne, IL (United States).
#[inline]
pub fn copper_thermal_conductivity_zou_zweibaum_spline(
    temperature: ThermodynamicTemperature) -> Result<ThermalConductivity,TuasLibError> {

    range_check(
        &Material::Solid(SolidMaterial::Copper),
        temperature, 
        ThermodynamicTemperature::new::<kelvin>(1000.0), 
        ThermodynamicTemperature::new::<kelvin>(250.0))?;

    let temperature_value_kelvin: f64 = temperature.get::<kelvin>();
    // here we use a cubic spline to interpolate the values
    // it's a little calculation heavy, but don't really care now
    let thermal_cond_temperature_values_kelvin = c!(250.0, 300.0, 350.0, 
        400.0, 500.0, 1000.0);
    let thermal_conductivity_values_watt_per_meter_kelin = c!(406.0,
        401.0, 369.0, 393.0, 386.0, 352.0);

    let s = CubicSpline::from_nodes(&thermal_cond_temperature_values_kelvin, 
        &thermal_conductivity_values_watt_per_meter_kelin);

    let copper_thermal_conductivity_value = s.unwrap().
        eval(temperature_value_kelvin);

    Ok(ThermalConductivity::new::<watt_per_meter_kelvin>(
        copper_thermal_conductivity_value))

}

/// returns specific enthalpy of copper
/// cited from:
/// Zou, L., Hu, R., & Charpentier, A. (2019). SAM code 
/// validation using the compact integral effects test (CIET) experimental 
/// data (No. ANL/NSE-19/11). Argonne National 
/// Lab.(ANL), Argonne, IL (United States).
#[inline]
pub fn copper_specific_enthalpy(
    temperature: ThermodynamicTemperature) -> AvailableEnergy {

    let temperature_value_kelvin: f64 = temperature.get::<kelvin>();
    // here we use a cubic spline to interpolate the values
    // it's a little calculation heavy, but don't really care now
    let specific_enthalpy_temperature_values_kelvin = c!(200.0, 
        250.0, 300.0, 350.0, 
        400.0, 500.0, 1000.0);
    let specific_heat_capacity_values_joule_per_kilogram_kelvin = c!(355.7047,
        373.6018, 384.7875, 392.6174,
        398.2103, 407.1588, 417.2260);

    let s = CubicSpline::from_nodes(&specific_enthalpy_temperature_values_kelvin, 
        &specific_heat_capacity_values_joule_per_kilogram_kelvin);

    let copper_specific_enthalpy_value = s.unwrap().
        integrate((273.15,temperature_value_kelvin));

    return AvailableEnergy::new::<joule_per_kilogram>(
        copper_specific_enthalpy_value);

}
/// returns temperature of copper
/// cited from:
/// Zou, L., Hu, R., & Charpentier, A. (2019). SAM code 
/// validation using the compact integral effects test (CIET) experimental 
/// data (No. ANL/NSE-19/11). Argonne National 
/// Lab.(ANL), Argonne, IL (United States).
///
/// The algorithm is to make a spline of enthalpy and temperature
/// and then use the enthalpy to obtain a temperature
///
/// Note: h in this function represents specific enthalpy
#[inline]
pub(crate) fn copper_spline_temp_attempt_2_from_specific_enthalpy(
    h_copper: AvailableEnergy) -> ThermodynamicTemperature {

    // the idea is basically to evaluate enthalpy at the 
    // following temperatures
    let temperature_values_kelvin: Vec<f64>
    = c!(200.0 ,250.0, 300.0, 350.0, 
        400.0, 500.0, 1000.0);

    // and then use that to formulate a spline,
    // with the spline, i'll evaluate enthalpy from temperature
    // within pretty much one iteration. However, it is spline 
    // construction which may take a little long. 
    //
    // However, the number of iterations per calculation is fixed
    //
    // I won't optimise it now just yet

    let temperature_vec_len = 
    temperature_values_kelvin.len();

    let mut enthalpy_vector = vec![0.0; temperature_vec_len];

    for index_i in 0..temperature_vec_len {

        // first, evaluate the enthalpy at temperature values 
        let temperature_value = temperature_values_kelvin[index_i];

        //next let's evaluate the specific enthalpy of copper 
        let copper = Material::Solid(SolidMaterial::Copper);
        let copper_temp = ThermodynamicTemperature::new::<kelvin>(
            temperature_value);
        let pressure = Pressure::new::<atmosphere>(1.0);

        let copper_enthalpy_result = try_get_h(copper, 
            copper_temp, pressure);

        let copper_enthalpy_value = match copper_enthalpy_result {
            Ok(copper_enthalpy) => copper_enthalpy.value,
            // i can of course unwrap the result,
            // but i want to leave it more explicit in case 
            // i wish to manually handle the error
            Err(error_msg) => panic!("{}",error_msg),
        };

        // once i evalute the enthalpy value, pass it on to the vector

        enthalpy_vector[index_i] = copper_enthalpy_value;

    }


    // now I have my enthalpy vector, i can do an inverted spline 
    // to have enthalpy given in as an input, and temperature received
    // as an output

    let enthalpy_to_temperature_spline = 
    CubicSpline::from_nodes(&enthalpy_vector,
    &temperature_values_kelvin);

    // now let's get our enthalpy in joules_per_kg
    let h_copper_joules_per_kg = h_copper.get::<joule_per_kilogram>();

    let temperature_from_enthalpy_kelvin = 
    enthalpy_to_temperature_spline.unwrap().eval(h_copper_joules_per_kg);

    // now, the copper enthalpy will not be quite near 
    // enough, but it is very close. We can bracket 
    // the root 


    let enthalpy_root = |temp_degrees_c_value : f64| -> f64 {
        let lhs_value = h_copper.get::<joule_per_kilogram>();


        let copper = Material::Solid(SolidMaterial::Copper);
        let copper_temp = ThermodynamicTemperature::new::
            <kelvin>(temp_degrees_c_value) ;
        let pressure = Pressure::new::<atmosphere>(1.0);

        let rhs = try_get_h(copper, 
            copper_temp, pressure);

        let rhs_value = match rhs {
            Ok(enthalpy_val) => enthalpy_val.get::<joule_per_kilogram>(),
                // fall back to guess value, 
            Err(error_msg) => panic!("{}",error_msg),
        };

        return lhs_value-rhs_value;
    };

    let brent_error_bound: f64 = 30.0;

    let upper_limit: f64 = temperature_from_enthalpy_kelvin +
        brent_error_bound;

    let lower_limit : f64 = temperature_from_enthalpy_kelvin -
        brent_error_bound;

    let mut convergency = SimpleConvergency { eps:1e-8f64, max_iter:30 };
    let fluid_temperature_degrees_c_result
    = find_root_brent(upper_limit,
        lower_limit,
        enthalpy_root,
        &mut convergency
    );

    let temperature_from_enthalpy_kelvin = 
    fluid_temperature_degrees_c_result.unwrap();

    // return temperature
    ThermodynamicTemperature::new::<kelvin>(
        temperature_from_enthalpy_kelvin)

}


#[test]
pub fn copper_temperature_from_enthalpy_test_spline_2(){
    // we'll test temperature at 375K 
    // we should get an enthalpy from the spline 
    // for zweibaum's paper 

    let copper = Material::Solid(SolidMaterial::Copper);
    let copper_temp = ThermodynamicTemperature::new::<kelvin>(375.0);
    let pressure = Pressure::new::<atmosphere>(1.0);

    let enthalpy_spline_zweibaum_375k = try_get_h(
        copper,copper_temp,pressure).unwrap();

    // now we have an enthalpy, let's check the temperature 

    let temperature_from_enthalpy_test = 
    copper_spline_temp_attempt_2_from_specific_enthalpy(
        enthalpy_spline_zweibaum_375k);

    // we are basically by about 5K, which is 
    // not within measurement error, probably have to do more work
    // what this means is that accuracy is sacrificed
    // for speed, sometimes too much accuracy
    //
    // for enthalpy, we probably want to have it as accurate 
    // as possible so that energy doesn't appear from nowhere 
    // and disappear from calculation
    //
    // I would note though, that the spline method does 
    // give a pretty good initial guess of where the temperature 
    // ought to be, so perhaps the iterative method can be used 
    // for the last few iterations to convergence
    // we could use brent dekker method
    approx::assert_abs_diff_eq!(
        temperature_from_enthalpy_test.get::<kelvin>(),
        375.0,
        epsilon=0.005);


}

#[inline]
/// copper max temp 
pub fn max_temp_copper_zou_zweibaum_spline() -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(1000.0)

}
#[inline]
/// copper min temp 
pub fn min_temp_copper_zou_zweibaum_spline() -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(250.0)
}
