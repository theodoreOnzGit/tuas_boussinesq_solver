use peroxide::fuga::{CubicSpline, Spline};
use roots::{find_root_brent, SimpleConvergency};
use specific_enthalpy::try_get_h;
use uom::si::available_energy::joule_per_kilogram;
use uom::si::f64::*;
use uom::si::length::nanometer;
use uom::si::mass_density::gram_per_cubic_centimeter;
use uom::si::pressure::atmosphere;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::specific_power::kilowatt_per_kilogram;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::thermal_conductivity::milliwatt_per_meter_kelvin;
use crate::boussinesq_thermophysical_properties::*;
use crate::tuas_lib_error::TuasLibError;
use uom::si::thermodynamic_temperature::kelvin;

#[inline]
pub fn tungsten_density() -> Result<MassDensity,TuasLibError> {
    todo!()
}


pub fn tungsten_surf_roughness() -> Length {
    todo!()
}

#[inline]
pub fn tungsten_specific_heat_capacity_rough_estimate(
    temperature: ThermodynamicTemperature) -> SpecificHeatCapacity {

    range_check(
        &Material::Solid(SolidMaterial::PyrogelHPS),
        temperature, 
        ThermodynamicTemperature::new::<degree_celsius>(650.0), 
        ThermodynamicTemperature::new::<degree_celsius>(10.0)).unwrap();

    todo!()

}


#[inline]
pub fn tungsten_specific_enthalpy(
    temperature: ThermodynamicTemperature) -> AvailableEnergy {

    todo!()
}

#[inline]
pub fn tungsten_hps_specific_heat_capacity_spline_low_temp(
    temperature: ThermodynamicTemperature) -> 
Result<SpecificHeatCapacity,TuasLibError> {

    range_check(
        &Material::Solid(SolidMaterial::PyrogelHPS),
        temperature, 
        ThermodynamicTemperature::new::<degree_celsius>(39.819), 
        ThermodynamicTemperature::new::<degree_celsius>(9.88))?;
    let pyrogel_cp_temperature_values_degc = c!(
        9.883, 24.019, 39.819);
    let pyrogel_cp_values_joule_per_kg_kelvin = c!(
        883.261, 898.311, 983.814);
    let s = CubicSpline::from_nodes(&pyrogel_cp_temperature_values_degc, 
        &pyrogel_cp_values_joule_per_kg_kelvin);

    let temperature_value_degc: f64 = temperature.get::<degree_celsius>();
    let pyrogel_generic_cp_joule_per_kg_kelvin = 
        s.unwrap().eval(temperature_value_degc);
    todo!()
}

#[inline]
pub fn tungsten_spline_data(temperature: ThermodynamicTemperature,) 
    -> Result<SpecificPower, TuasLibError> {

    range_check(
        &Material::Solid(SolidMaterial::PyrogelHPS),
        temperature, 
        ThermodynamicTemperature::new::<degree_celsius>(327.0), 
        ThermodynamicTemperature::new::<degree_celsius>(35.0))?;
    let specific_power_temperature_values_degc = c!(35.426,
        42.085,
        50.609,
        59.132,
        74.581,
        88.432,
        99.619,
        123.592,
        136.91,
        151.294,
        166.21,
        178.463,
        190.183,
        200.837,
        215.221,
        225.875,
        234.399,
        239.726,
        243.455,
        248.25,
        251.446,
        253.577,
        257.839,
        259.97,
        262.633,
        265.83,
        267.428,
        271.157,
        279.148,
        304.186,
        326.294
        );
    let specific_power_values_milliwatts_per_milligram = c!(
        0.261,
        0.274,
        0.285,
        0.292,
        0.293,
        0.291,
        0.287,
        0.28,
        0.278,
        0.277,
        0.282,
        0.285,
        0.285,
        0.285,
        0.291,
        0.302,
        0.32,
        0.338,
        0.351,
        0.365,
        0.373,
        0.374,
        0.369,
        0.361,
        0.345,
        0.322,
        0.303,
        0.296,
        0.29,
        0.286,
        0.283
            );

    let s = CubicSpline::from_nodes(&specific_power_temperature_values_degc, 
        &specific_power_values_milliwatts_per_milligram);

    let temperature_value_degc: f64 = temperature.get::<degree_celsius>();
    let pyrogel_generic_dsc_milliwatt_per_milligram = 
        s.unwrap().eval(temperature_value_degc);

    todo!()
}

#[inline]
pub fn tungsten_thermal_conductivity(
    temperature: ThermodynamicTemperature) -> Result<ThermalConductivity,TuasLibError> {

    range_check(
        &Material::Solid(SolidMaterial::PyrogelHPS),
        temperature, 
        ThermodynamicTemperature::new::<degree_celsius>(650.0), 
        ThermodynamicTemperature::new::<degree_celsius>(0.0))?;

    let temperature_value_degc: f64 = temperature.get::<degree_celsius>();
    // here we use a cubic spline to interpolate the values
    // it's a little calculation heavy, but don't really care now
    let thermal_cond_temperature_values_degc = c!(
        0.0, 100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 650.0
        );
    let thermal_conductivity_values_milliwatt_per_meter_kelvin = c!(
        20.0, 24.0, 28.0, 33.0, 40.0, 49.0, 62.0, 69.0
        );

    let s = CubicSpline::from_nodes(&thermal_cond_temperature_values_degc, 
        &thermal_conductivity_values_milliwatt_per_meter_kelvin);

    let pyrogel_hps_thermal_conductivity_value_milliwatt_per_meter_kelvin = s.unwrap().eval(
        temperature_value_degc);

    todo!()
}

#[inline]
pub (crate) fn tungsten_spline_temp_attempt_1_from_specific_enthalpy(
    h_fiberglass: AvailableEnergy) -> ThermodynamicTemperature {

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

        //next let's evaluate the specific enthalpy of fiberglass 
        let pyrogel_hps = Material::Solid(SolidMaterial::PyrogelHPS);
        let pyrogel_hps_temp = ThermodynamicTemperature::new::<kelvin>(
            temperature_value);
        let pressure = Pressure::new::<atmosphere>(1.0);

        let pyrogel_enthalpy_result = try_get_h(pyrogel_hps, 
            pyrogel_hps_temp, pressure);

        let pyrogel_enthalpy_value = match pyrogel_enthalpy_result {
            Ok(fiberglass_enthalpy) => fiberglass_enthalpy.value,
            // i can of course unwrap the result,
            // but i want to leave it more explicit in case 
            // i wish to manually handle the error
            Err(error_msg) => panic!("{}",error_msg),
        };

        // once i evalute the enthalpy value, pass it on to the vector

        enthalpy_vector[index_i] = pyrogel_enthalpy_value;

    }


    // now I have my enthalpy vector, i can do an inverted spline 
    // to have enthalpy given in as an input, and temperature received
    // as an output

    let enthalpy_to_temperature_spline = 
    CubicSpline::from_nodes(&enthalpy_vector,
    &temperature_values_kelvin);

    // now let's get our enthalpy in joules_per_kg
    let h_pyrogel_joules_per_kg = h_fiberglass.get::<joule_per_kilogram>();

    let temperature_from_enthalpy_kelvin = 
    enthalpy_to_temperature_spline.unwrap().eval(h_pyrogel_joules_per_kg);

    // now, the pyrogel enthalpy will not be quite near 
    // enough, but it is very close. We can bracket 
    // the root 


    let enthalpy_root = |temp_degrees_c_value : f64| -> f64 {
        let lhs_value = h_fiberglass.get::<joule_per_kilogram>();


        let fiberglass = Material::Solid(SolidMaterial::PyrogelHPS);
        let fiberglass_temp = ThermodynamicTemperature::new::
            <kelvin>(temp_degrees_c_value) ;
        let pressure = Pressure::new::<atmosphere>(1.0);

        let rhs = try_get_h(fiberglass, 
            fiberglass_temp, pressure);

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
    todo!()

}

#[inline]
/// tungsten_hps max temp 
pub fn max_temp_tungsten_hps() -> ThermodynamicTemperature {
    todo!()

}
#[inline]
/// tungsten_hps min temp 
pub fn min_temp_tungsten_hps() -> ThermodynamicTemperature {
    todo!()
}
