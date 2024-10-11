use uom::si::f64::*;
use uom::si::length::millimeter;
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::thermal_conductivity::watt_per_meter_kelvin;
use crate::boussinesq_thermophysical_properties::*;
use crate::thermal_hydraulics_error::TuasLibError;
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

#[test]
pub fn specific_heat_capacity_test_steel(){

    // we're going to test thermal conductivity for steel,
    // first at 500K for both the spline and the correlation 
    // cp, we expect at 350K 
    // 469.4894 J/(kg K)

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
