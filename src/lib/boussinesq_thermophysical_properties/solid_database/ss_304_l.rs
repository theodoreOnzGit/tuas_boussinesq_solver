use roots::{find_root_brent, SimpleConvergency};
use specific_enthalpy::try_get_h;
use uom::si::available_energy::{joule_per_gram, joule_per_kilogram};
use uom::si::f64::*;
use uom::si::length::millimeter;
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::pressure::atmosphere;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::thermal_conductivity::watt_per_meter_kelvin;
use crate::boussinesq_thermophysical_properties::*;
use crate::tuas_lib_error::TuasLibError;
use uom::si::thermodynamic_temperature::kelvin;


use peroxide::prelude::*;
/// returns thermal conductivity of stainless steel 304L
/// cited from:
/// Zou, L., Hu, R., & Charpentier, A. (2019). SAM code 
/// validation using the compact integral effects test (CIET) experimental 
/// data (No. ANL/NSE-19/11). Argonne National 
/// Lab.(ANL), Argonne, IL (United States).
#[inline]
pub fn steel_304_l_spline_specific_heat_capacity_ciet_zweibaum(
    temperature: ThermodynamicTemperature) -> Result<SpecificHeatCapacity,TuasLibError> {

    range_check(
        &Material::Solid(SolidMaterial::SteelSS304L),
        temperature, 
        ThermodynamicTemperature::new::<kelvin>(1000.0), 
        ThermodynamicTemperature::new::<kelvin>(250.0))?;

    let temperature_value_kelvin: f64 = temperature.get::<kelvin>();
    // here we use a cubic spline to interpolate the values
    // it's a little calculation heavy, but don't really care now
    let thermal_cond_temperature_values_kelvin = c!(250.0, 300.0, 350.0, 
        400.0, 450.0, 500.0, 700.0, 1000.0);
    let specific_heat_capacity_values_joule_per_kilogram_kelvin = c!(443.3375,
        457.0361, 469.4894, 480.6974, 490.66, 500.6227, 526.7746,
        551.6812);

    let s = CubicSpline::from_nodes(&thermal_cond_temperature_values_kelvin, 
        &specific_heat_capacity_values_joule_per_kilogram_kelvin);

    let steel_specific_heat_capacity_value = s.unwrap().eval(
        temperature_value_kelvin);

    return Ok(SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(
        steel_specific_heat_capacity_value));
}

/// returns thermal conductivity of stainless steel 304L
/// cited from:
/// Zou, L., Hu, R., & Charpentier, A. (2019). SAM code 
/// validation using the compact integral effects test (CIET) experimental 
/// data (No. ANL/NSE-19/11). Argonne National 
/// Lab.(ANL), Argonne, IL (United States).
///
/// Instead of constructing a spline object on the spot and then deleting 
/// it, I used Libreoffice Calc to construct a spline manually instead
#[inline]
pub fn steel_304_l_libreoffice_spline_specific_heat_capacity_ciet_zweibaum(
    temperature: ThermodynamicTemperature) -> Result<SpecificHeatCapacity,TuasLibError> {

    range_check(
        &Material::Solid(SolidMaterial::SteelSS304L),
        temperature, 
        ThermodynamicTemperature::new::<kelvin>(1000.0), 
        ThermodynamicTemperature::new::<kelvin>(250.0))?;

    let temperature_value_kelvin: f64 = temperature.get::<kelvin>();
    // here we use a cubic spline to interpolate the values
    // this correlation was done in libreoffice
    //
    // in joule per kilogram kelvin
    let steel_specific_heat_capacity_value = 
        3.494005840e2 
        + 4.655602117e-1 * temperature_value_kelvin
        - 3.976680063e-4 * temperature_value_kelvin.powf(2.0)
        + 1.313656168e-7 * temperature_value_kelvin.powf(3.0);

    return Ok(SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(
        steel_specific_heat_capacity_value));
}


///
/// Graves, R. S., Kollie, T. G., 
/// McElroy, D. L., & Gilchrist, K. E. (1991). The 
/// thermal conductivity of AISI 304L stainless steel. 
/// International journal of thermophysics, 12, 409-415. 
///
/// data taken from ORNL
///
/// It's only good for range of 300K to 700K
#[inline]
pub fn steel_ss_304_l_ornl_specific_heat_capacity(
    temperature: ThermodynamicTemperature) -> Result<SpecificHeatCapacity,TuasLibError> {

    range_check(
        &Material::Solid(SolidMaterial::SteelSS304L),
        temperature, 
        ThermodynamicTemperature::new::<kelvin>(700.0), 
        ThermodynamicTemperature::new::<kelvin>(300.0))?;

    let temperature_value_kelvin: f64 = temperature.get::<kelvin>();
    let specific_heat_capacity_val = 1000.0 * (0.4267
    + 1.700 * f64::powf(10.0,-4.0) * temperature_value_kelvin
    - 5.200 * f64::powf(10.0, -8.0));

    Ok(SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(
        specific_heat_capacity_val))
}

/// we're going to test thermal conductivity for steel,
/// first at 500K for both the spline and the correlation 
/// cp, we expect at 350K 
/// 469.4894 J/(kg K)
#[test]
pub fn specific_heat_capacity_test_steel(){


    let thermal_cond_spline = steel_304_l_spline_specific_heat_capacity_ciet_zweibaum(
        ThermodynamicTemperature::new::<kelvin>(350.0));

    approx::assert_relative_eq!(
        469.4894,
        thermal_cond_spline.unwrap().value,
        max_relative=0.001);

    // now for the Graves et al. 1991 version, from ORNL
    //

    let specific_heat_graves_et_al_1991 = 
    steel_ss_304_l_ornl_specific_heat_capacity(
        ThermodynamicTemperature::new::<kelvin>(350.0));

    // between graves and the Zou/Zweibaum version,
    // there is abut 3.5\% difference
    //
    approx::assert_relative_eq!(
        469.4894,
        specific_heat_graves_et_al_1991.unwrap().value,
        max_relative=0.035);

    // let's try now at 1000K 
    // we expect thermal specific_heat_capacity to be at 23.83

    let thermal_cond_spline = 
    steel_304_l_spline_specific_heat_capacity_ciet_zweibaum(
        ThermodynamicTemperature::new::<kelvin>(1000.0));

    approx::assert_relative_eq!(
        551.6812,
        thermal_cond_spline.unwrap().value,
        max_relative=0.0001);


}


///
/// Graves, R. S., Kollie, T. G., 
/// McElroy, D. L., & Gilchrist, K. E. (1991). The 
/// thermal conductivity of AISI 304L stainless steel. 
/// International journal of thermophysics, 12, 409-415. 
///
/// data taken from ORNL
///
/// It's only good for range of 300K to 700K
#[inline]
pub fn steel_ss_304_l_ornl_thermal_conductivity(
    temperature: ThermodynamicTemperature) -> Result<ThermalConductivity,TuasLibError> {

    range_check(
        &Material::Solid(SolidMaterial::SteelSS304L),
        temperature, 
        ThermodynamicTemperature::new::<kelvin>(700.0), 
        ThermodynamicTemperature::new::<kelvin>(300.0))?;

    let temperature_value_kelvin: f64 = temperature.get::<kelvin>();
    let thermal_conductivity_val = 7.9318 
    + 0.023051 * temperature_value_kelvin
    - 6.4166 * f64::powf(10.0, -6.0);

    Ok(ThermalConductivity::new::<watt_per_meter_kelvin>(
        thermal_conductivity_val))
}


/// returns thermal conductivity of stainless steel 304L
/// cited from:
/// Zou, L., Hu, R., & Charpentier, A. (2019). SAM code 
/// validation using the compact integral effects test (CIET) experimental 
/// data (No. ANL/NSE-19/11). Argonne National 
/// Lab.(ANL), Argonne, IL (United States).
#[inline]
pub fn steel_304_l_spline_thermal_conductivity(
    temperature: ThermodynamicTemperature) -> Result<ThermalConductivity,TuasLibError> {

    range_check(
        &Material::Solid(SolidMaterial::SteelSS304L),
        temperature, 
        ThermodynamicTemperature::new::<kelvin>(1000.0), 
        ThermodynamicTemperature::new::<kelvin>(250.0))?;

    let temperature_value_kelvin: f64 = temperature.get::<kelvin>();
    // here we use a cubic spline to interpolate the values
    // it's a little calculation heavy, but don't really care now
    let thermal_cond_temperature_values_kelvin = c!(250.0, 300.0, 350.0, 
        400.0, 450.0, 500.0, 700.0, 1000.0);
    let thermal_conductivity_values_watt_per_meter_kelin = c!(14.31,
        14.94, 15.58, 16.21, 16.85, 17.48, 20.02, 23.83);
    //let cp_values_watt_per_meter_kelin = c!(443.3375,
    //    457.0361, 469.4894, 480.6974, 490.66, 500.6227, 526.7746,
    //    551.6812);

    let s = CubicSpline::from_nodes(&thermal_cond_temperature_values_kelvin, 
        &thermal_conductivity_values_watt_per_meter_kelin);

    let steel_thermal_conductivity_value = s.unwrap().eval(
        temperature_value_kelvin);

    return Ok(ThermalConductivity::new::<watt_per_meter_kelvin>(
        steel_thermal_conductivity_value));
}


/// returns thermal conductivity of stainless steel 304L
/// cited from:
/// Zou, L., Hu, R., & Charpentier, A. (2019). SAM code 
/// validation using the compact integral effects test (CIET) experimental 
/// data (No. ANL/NSE-19/11). Argonne National 
/// Lab.(ANL), Argonne, IL (United States).
///
/// I used libreoffice to construct the spline rather than use Rust's 
/// inbuilt function, which is more computationally expensive
#[inline]
pub fn steel_304_l_libreoffice_spline_thermal_conductivity_zweibaum(
    temperature: ThermodynamicTemperature) -> Result<ThermalConductivity,TuasLibError> {

    range_check(
        &Material::Solid(SolidMaterial::SteelSS304L),
        temperature, 
        ThermodynamicTemperature::new::<kelvin>(1000.0), 
        ThermodynamicTemperature::new::<kelvin>(250.0))?;

    let temperature_value_kelvin: f64 = temperature.get::<kelvin>();

    let steel_thermal_conductivity_value = 1.113e1 + 
        1.269e-2 * temperature_value_kelvin;

    return Ok(ThermalConductivity::new::<watt_per_meter_kelvin>(
        steel_thermal_conductivity_value));
}

/// this test checks if the libreoffice splines match up to 
/// the cubic splines that are constructed using the CubicSpline 
/// function in Rust
#[test] 
pub fn verify_libreoffice_splines_work(){


    fn test_thermal_conductivity(temperature: ThermodynamicTemperature){
        // for thermal conductivity 
        
        let standard_spline_value_si_units: f64 = 
            steel_304_l_spline_thermal_conductivity(temperature)
            .unwrap()
            .get::<watt_per_meter_kelvin>();

        let libreoffice_spline_value_si_units: f64 = 
            steel_304_l_libreoffice_spline_thermal_conductivity_zweibaum(temperature)
            .unwrap()
            .get::<watt_per_meter_kelvin>();

        // correlation agrees to within 0.1%
        approx::assert_relative_eq!(
            standard_spline_value_si_units,
            libreoffice_spline_value_si_units,
            max_relative=0.001);

    }

    fn test_specific_heat_capacity(temperature: ThermodynamicTemperature){
        // for thermal conductivity 
        
        let standard_spline_value_si_units: f64 = 
            steel_304_l_spline_specific_heat_capacity_ciet_zweibaum(temperature)
            .unwrap()
            .get::<joule_per_kilogram_kelvin>();

        let libreoffice_spline_value_si_units: f64 = 
            steel_304_l_libreoffice_spline_specific_heat_capacity_ciet_zweibaum(temperature)
            .unwrap()
            .get::<joule_per_kilogram_kelvin>();

        // max deviation is 0.55% from the standard spline value
        approx::assert_relative_eq!(
            standard_spline_value_si_units,
            libreoffice_spline_value_si_units,
            max_relative=0.0055);

    }

    let thermal_cond_temperature_values_kelvin = c!(250.0, 300.0, 350.0, 
        400.0, 450.0, 500.0, 700.0, 1000.0);

    for temperature_value_kelvin in thermal_cond_temperature_values_kelvin.iter(){

        let temperature = ThermodynamicTemperature::new::<kelvin>(
            *temperature_value_kelvin);


        test_thermal_conductivity(temperature);
        test_specific_heat_capacity(temperature);

    }


}

/// density ranges not quite given in original text 
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code validation 
/// using the compact integral effects test (CIET) experimental data. 
/// No. ANL/NSE-19/11. 
/// Argonne National Lab.(ANL), Argonne, IL (United States), 2019.
#[inline]
pub fn steel_ss_304_l_density() -> Result<MassDensity,TuasLibError> {
    return Ok(MassDensity::new::<kilogram_per_cubic_meter>(8030.0));
}


/// Value from: Perry's chemical Engineering handbook 
/// 8th edition Table 6-1 
/// commercial steel or wrought iron 
/// Perry, R. H., & DW, G. (2007). 
/// Perry’s chemical engineers’ handbook, 
/// 8th illustrated ed. New York: McGraw-Hill.
pub fn steel_surf_roughness() -> Length{
    Length::new::<millimeter>(0.0457)
}

/// From:
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code validation 
/// using the compact integral effects test (CIET) experimental data. 
/// No. ANL/NSE-19/11. 
/// Argonne National Lab.(ANL), Argonne, IL (United States), 2019.
///
/// Basically, the density is a fixed value
#[test]
pub fn density_test_steel(){

    use uom::si::thermodynamic_temperature::kelvin;
    use uom::si::pressure::atmosphere;
    use density::try_get_rho;
    let steel = Material::Solid(SolidMaterial::SteelSS304L);
    let temperature = ThermodynamicTemperature::new::<kelvin>(396.0);
    let pressure = Pressure::new::<atmosphere>(1.0);

    let density = try_get_rho(steel, temperature, pressure);

    approx::assert_relative_eq!(
        8030_f64,
        density.unwrap().value,
        max_relative=0.01);
}

/// returns specific enthalpy of stainless steel 304L
/// cited from:
/// Zou, L., Hu, R., & Charpentier, A. (2019). SAM code 
/// validation using the compact integral effects test (CIET) experimental 
/// data (No. ANL/NSE-19/11). Argonne National 
/// Lab.(ANL), Argonne, IL (United States).
#[inline]
pub fn steel_304_l_spline_specific_enthalpy_ciet_zweibaum(
    temperature: ThermodynamicTemperature) -> AvailableEnergy {


    let temperature_value_kelvin: f64 = temperature.get::<kelvin>();
    // here we use a cubic spline to interpolate the values
    // it's a little calculation heavy, but don't really care now
    let specific_enthalpy_temperature_values_kelvin = c!(250.0, 300.0, 350.0, 
        400.0, 450.0, 500.0, 700.0, 1000.0);
    let specific_heat_capacity_values_joule_per_kilogram_kelvin = c!(443.3375,
        457.0361, 469.4894, 480.6974, 490.66, 500.6227, 526.7746,
        551.6812);

    let s = CubicSpline::from_nodes(&specific_enthalpy_temperature_values_kelvin, 
        &specific_heat_capacity_values_joule_per_kilogram_kelvin);

    let steel_specific_enthalpy_value = s.unwrap().integrate(
        (273.15,temperature_value_kelvin));

    return AvailableEnergy::new::<joule_per_kilogram>(
        steel_specific_enthalpy_value);
}

///
/// Graves, R. S., Kollie, T. G., 
/// McElroy, D. L., & Gilchrist, K. E. (1991). The 
/// specific enthalpy of AISI 304L stainless steel. 
/// International journal of thermophysics, 12, 409-415. 
///
/// data taken from ORNL
///
/// It's only good for range of 300K to 700K
///
/// However, I analytically integrated it with wolfram alpha
#[inline]
pub fn steel_ss_304_l_ornl_specific_enthalpy_graves(
    temperature: ThermodynamicTemperature) -> AvailableEnergy {

    // first I define a function for specific enthalpy between two 
    // temperatures in kelvin
    fn definite_integral_specific_enthalpy(
        temp_1: ThermodynamicTemperature,
        temp_2: ThermodynamicTemperature) -> AvailableEnergy {

        // the integration constant is assumed to be zero 

        let temp_1_value_kelvin = temp_1.get::<kelvin>();
        let temp_2_value_kelvin = temp_2.get::<kelvin>();

        let enthalpy_value_joule_per_gram_per_kelvin_temp_1 = 
        1.73333e-8 * f64::powf(temp_1_value_kelvin,3.0) 
        + 0.000085 * f64::powf(temp_1_value_kelvin, 2.0)
        + 0.4267 * temp_1_value_kelvin;

        let enthalpy_value_joule_per_gram_per_kelvin_temp_2 = 
        1.73333e-8 * f64::powf(temp_2_value_kelvin,3.0) 
        + 0.000085 * f64::powf(temp_2_value_kelvin, 2.0)
        + 0.4267 * temp_2_value_kelvin;

        let enthalpy_difference_joule_per_gram = 
        enthalpy_value_joule_per_gram_per_kelvin_temp_2 
        - enthalpy_value_joule_per_gram_per_kelvin_temp_1;

        AvailableEnergy::new::<joule_per_gram>(
            enthalpy_difference_joule_per_gram)
    }

    // reference temperature is zero degrees c, 
    // enthalpy is zero j/kg at that point
    let refernce_temperature = ThermodynamicTemperature::new::
    <kelvin>(273.15);

    let steel_enthalpy = definite_integral_specific_enthalpy(
        refernce_temperature, temperature);

    steel_enthalpy
}

/// specific enthalpys test using oak ridge national lab publication
/// wolfram gives an enthalpy (assuming enthalpy is zero at zero 
/// degrees C, 273.15 K)
/// this is done using the Graves et al. 1991 version for cp
/// 37.2524 j/g
#[test]
pub fn specific_enthalpy_test_steel_ornl(){
    // let's test specifc enthalpy at 350K 

    let test_temperature = ThermodynamicTemperature::new::
    <kelvin>(350.0);

    // wolfram gives an enthalpy (assuming enthalpy is zero at zero 
    // degrees C, 273.15 K)
    // this is done using the Graves et al. 1991 version for cp
    //  37.2524 j/g
    let wolfram_enthalpy_value_joule_per_kg = 37.2524*1000.0;

    let enthalpy_analytical_ornl = 
    steel_ss_304_l_ornl_specific_enthalpy_graves(test_temperature);

    approx::assert_relative_eq!(
        wolfram_enthalpy_value_joule_per_kg,
        enthalpy_analytical_ornl.value,
        max_relative=0.0001);

    
}

/// here is a test for comparing ornl and nico zweibaum's value 
/// at 375 to 425 kelvin
///
/// the cp correlation was from 300 to 700 Kelvin, so using 273.15 
/// as zero enthalpy is technically outside the range.
///
/// Despite this, it should still be able to calculate enthalpy 
/// change from 375 K to 425K
#[test]
pub fn specific_enthalpy_test_steel_ornl_and_zweibaum_spline(){
    // let's test specifc enthalpy at 350K 

    let test_temperature_1 = ThermodynamicTemperature::new::
    <kelvin>(375.0);
    let test_temperature_2 = ThermodynamicTemperature::new::
    <kelvin>(425.0);

    // wolfram gives an enthalpy (assuming enthalpy is zero at zero 
    // degrees C, 273.15 K)
    // this is done using the Graves et al. 1991 version for cp
    // 25.1515 j/g for 375 to 425 K
    let wolfram_enthalpy_value_joule_per_kg = 25.1515*1000.0;

    let enthalpy_analytical_ornl = 
    steel_ss_304_l_ornl_specific_enthalpy_graves(test_temperature_2)
    - steel_ss_304_l_ornl_specific_enthalpy_graves(test_temperature_1);

    approx::assert_relative_eq!(
        wolfram_enthalpy_value_joule_per_kg,
        enthalpy_analytical_ornl.value,
        max_relative=0.0001);

    // now let's test the spline version 
    //
    let enthalpy_spline_zweibaum = 
    steel_304_l_spline_specific_enthalpy_ciet_zweibaum(test_temperature_2)
    - steel_304_l_spline_specific_enthalpy_ciet_zweibaum(test_temperature_1);

    // there is about a 4.5% difference between the ornl value 
    // and the spline value
    // doesn't seem too bad honestly.
    //
    // Of course, one can do uncertainty propagation in order to 
    // find out the degree of change, but I won't do that for now.
    //
    // otherwise, spline should work quite okay
    approx::assert_relative_eq!(
        enthalpy_analytical_ornl.value,
        enthalpy_spline_zweibaum.value,
        max_relative=0.045);
}
/// returns temperature of stainless steel 304L 
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
///
/// This uses the spline methodology as an initial guess
/// but then uses brent dekker to finish it off
#[inline]
pub (crate)fn steel_304_l_spline_temp_attempt_3_from_specific_enthalpy_ciet_zweibaum(
    h_steel: AvailableEnergy) -> ThermodynamicTemperature {

    // the idea is basically to evaluate enthalpy at the 
    // following temperatures
    let temperature_values_kelvin: Vec<f64>
    = c!(250.0, 300.0, 350.0, 
        400.0, 450.0, 500.0, 700.0, 1000.0);

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

        //next let's evaluate the specific enthalpy of steel 
        let steel = Material::Solid(SolidMaterial::SteelSS304L);
        let steel_temp = ThermodynamicTemperature::new::<kelvin>(
            temperature_value);
        let pressure = Pressure::new::<atmosphere>(1.0);

        let steel_enthalpy_result = try_get_h(steel, 
            steel_temp, pressure);

        let steel_enthalpy_value = match steel_enthalpy_result {
            Ok(steel_enthalpy) => steel_enthalpy.value,
            // i can of course unwrap the result,
            // but i want to leave it more explicit in case 
            // i wish to manually handle the error
            Err(error_msg) => panic!("{}",error_msg),
        };

        // once i evalute the enthalpy value, pass it on to the vector

        enthalpy_vector[index_i] = steel_enthalpy_value;

    }


    // now I have my enthalpy vector, i can do an inverted spline 
    // to have enthalpy given in as an input, and temperature received
    // as an output

    let enthalpy_to_temperature_spline = 
    CubicSpline::from_nodes(&enthalpy_vector,
    &temperature_values_kelvin);

    // now let's get our enthalpy in joules_per_kg
    let h_steel_joules_per_kg = h_steel.get::<joule_per_kilogram>();

    let temperature_from_enthalpy_kelvin = 
    enthalpy_to_temperature_spline.unwrap().eval(h_steel_joules_per_kg);

    let enthalpy_root = |temp_degrees_c_value : f64| -> f64 {
        let lhs_value = h_steel.get::<joule_per_kilogram>();


        let steel = Material::Solid(SolidMaterial::SteelSS304L);
        let steel_temp = ThermodynamicTemperature::new::
            <kelvin>(temp_degrees_c_value) ;
        let pressure = Pressure::new::<atmosphere>(1.0);

        let rhs = try_get_h(steel, 
            steel_temp, pressure);

        let rhs_value = match rhs {
            Ok(enthalpy_val) => enthalpy_val.get::<joule_per_kilogram>(),
                // fall back to guess value, 
            Err(error_msg) => panic!("{}",error_msg),
        };

        return lhs_value-rhs_value;
    };

    // brent error bounds can be smaller for this steel spline, 
    // it's a lot more accurate than for steel
    let brent_error_bound: f64 = 1.0;

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

    // we can extract the temperature 
    // but if this doesn't work, bracket the entire range
    // 250K to 1000K 

    //let upper_limit_full_range: f64 = 1000.0;
    //let lower_limit_full_range: f64 = 250.0;
    //
    // or better yet, just panic the value for steel enthalpy

    let temperature_from_enthalpy_kelvin: f64 = 
    match fluid_temperature_degrees_c_result {
        Ok(temperature_val) => temperature_val,
        Err(_) => panic!("{:?}",h_steel),
            //find_root_brent(upper_limit_full_range, 
            //lower_limit_full_range, 
            //enthalpy_root, 
            //&mut convergency).unwrap(),

    };

    // return temperature
    ThermodynamicTemperature::new::<kelvin>(
        temperature_from_enthalpy_kelvin)

}

/// this is my third, and final iteration of getting enthalpy as a function 
/// of temperature
#[test]
pub fn steel_temperature_from_enthalpy_test_spline_3(){
    // we'll test temperature at 375K 
    // we should get an enthalpy from the spline 
    // for zweibaum's paper 

    let steel = Material::Solid(SolidMaterial::SteelSS304L);
    let steel_temp = ThermodynamicTemperature::new::<kelvin>(375.0);
    let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);

    let enthalpy_spline_zweibaum_375k = try_get_h(
        steel,steel_temp,atmospheric_pressure).unwrap();

    // now we have an enthalpy, let's check the temperature 

    let temperature_from_enthalpy_test = 
    steel_304_l_spline_temp_attempt_3_from_specific_enthalpy_ciet_zweibaum(
        enthalpy_spline_zweibaum_375k);

    // we are basically off by less than 0.05K, which is 
    // within measurement error!
    approx::assert_abs_diff_eq!(
    temperature_from_enthalpy_test.get::<kelvin>(),
    375.0,
    epsilon=0.00005);

    // let's test from 325, 425, 525, 625, 725, 825, 925
    let temperature_vec_kelvin: Vec<f64> = 
    vec![325.0, 425.0, 525.0, 625.0, 725.0, 825.0, 925.0];

    for temperature_val_kelvin in temperature_vec_kelvin.iter() {

        let steel_temp = ThermodynamicTemperature::new::<kelvin>(
            *temperature_val_kelvin);

        let enthalpy_spline_zweibaum = try_get_h(steel, 
            steel_temp, atmospheric_pressure).unwrap();

        let temperature_from_enthalpy_test = 
        steel_304_l_spline_temp_attempt_3_from_specific_enthalpy_ciet_zweibaum(
            enthalpy_spline_zweibaum);

        // we are basically off by less than 0.5K, which is 
        // within measurement error!
        approx::assert_abs_diff_eq!(
        temperature_from_enthalpy_test.get::<kelvin>(),
        *temperature_val_kelvin,
        epsilon=0.00005);
    };


}

#[inline]
/// ss_304l max temp 
pub fn max_temp_ss_304l_zou_zweibaum_spline() -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(1000.0)

}
#[inline]
/// ss_304l min temp 
pub fn min_temp_ss_304l_zou_zweibaum_spline() -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(250.0)
}
