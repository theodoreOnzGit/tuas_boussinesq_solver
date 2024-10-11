#[warn(missing_docs)]

// This library was developed for use in my PhD thesis under supervision 
// of Professor Per F. Peterson. It is part of a thermal hydraulics
// library in Rust that is released under the GNU General Public License
// v 3.0. This is partly due to the fact that some of the libraries 
// inherit from GeN-Foam and OpenFOAM, both licensed under GNU General
// Public License v3.0.
//
// As such, the entire library is released under GNU GPL v3.0. It is a strong 
// copyleft license which means you cannot use it in proprietary software.
//
//
// License
//    This is file is part of a thermal hydraulics library written 
//    in rust meant to help with the
//    fluid mechanics and heat transfer aspects of the calculations
//    for the Compact Integral Effects Tests (CIET) and hopefully 
//    Gen IV Reactors such as the Fluoride Salt cooled High Temperature 
//    Reactor (FHR)
//     
//    Copyright (C) 2022-2024  Theodore Kay Chen Ong, Singapore Nuclear
//    Research and Safety Initiative, Per F. Peterson, University of 
//    California, Berkeley Thermal Hydraulics Laboratory
//
//    thermal_hydrualics_rs is free software; you can 
//    redistribute it and/or modify it
//    under the terms of the GNU General Public License as published by the
//    Free Software Foundation; either version 2 of the License, or (at your
//    option) any later version.
//
//    thermal_hydrualics_rs is distributed in the hope 
//    that it will be useful, but WITHOUT
//    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
//    FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
//    for more details.
//
//    This thermal hydraulics library 
//    contains some code copied from GeN-Foam, and OpenFOAM derivative.
//    This offering is not approved or endorsed by the OpenFOAM Foundation nor
//    OpenCFD Limited, producer and distributor of the OpenFOAM(R)software via
//    www.openfoam.com, and owner of the OPENFOAM(R) and OpenCFD(R) trademarks.
//    Nor is it endorsed by the authors and owners of GeN-Foam.
//
//    You should have received a copy of the GNU General Public License
//    along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// © All rights reserved. Theodore Kay Chen Ong,
// Singapore Nuclear Research and Safety Initiative,
// Per F. Peterson,
// University of California, Berkeley Thermal Hydraulics Laboratory
//
// Main author of the code: Theodore Kay Chen Ong, supervised by
// Professor Per F. Peterson
//
// Btw, I have no affiliation with the Rust foundation.
use uom::si::f64::*;
use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::dynamic_viscosity::centipoise;
use uom::si::thermal_conductivity::watt_per_meter_kelvin;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::available_energy::joule_per_kilogram;

// this is for the root finding algorithms
extern crate peroxide;
use peroxide::prelude::*;

use crate::boussinesq_thermophysical_properties::{range_check, LiquidMaterial, Material};
use crate::thermal_hydraulics_error::TuasLibError;

/// function to obtain flibe salt density
/// given a temperature
///
/// Romatoski, R. R., & Hu, L. W. (2017). Fluoride salt coolant properties 
/// for nuclear reactor applications: A review. Annals 
/// of Nuclear Energy, 109, 635-647.
/// properties for a custom liquid material 
/// not covered in the database
///
/// Sohal, M. S., Ebner, M. A., Sabharwall, P., & Sharpe, P. (2010). 
/// Engineering database of liquid salt thermophysical and thermochemical 
/// properties (No. INL/EXT-10-18297). Idaho National Lab.(INL), 
/// Idaho Falls, ID (United States).
/// 
///
/// rho (kg/m3) = 2415.6 - 0.49072 T(K)
/// Density correlation applies from melting point to critical point
/// 732.2 - 4498.8 K 
///
/// There is slight non-linearity for flibe density
/// but I'm ignoring that for now
pub fn get_flibe_density(
    fluid_temp: ThermodynamicTemperature) -> Result<MassDensity,TuasLibError> {


    // first we check if fluid temp is between 732.2-1573 K (range of validity)
    // panic otherwise
    range_check_flibe_salt(fluid_temp)?;
    let fluid_temp_kelvin = fluid_temp.get::<kelvin>();
    let a = 2415.6;
    let b = -0.49072;
    // generic correlation is:
    // a + bT + cT^2 + dT^3 + eT^4;

    let density_value_kg_per_m3 = 
        a 
        + b * fluid_temp_kelvin;


    return Ok(MassDensity::new::<
              kilogram_per_cubic_meter>(density_value_kg_per_m3));
}

/// function to obtain flibe salt viscosity
/// given a temperature
///
/// Romatoski, R. R., & Hu, L. W. (2017). Fluoride salt coolant properties 
/// for nuclear reactor applications: A review. Annals 
/// of Nuclear Energy, 109, 635-647.
/// properties for a custom liquid material 
/// not covered in the database
///
/// Sohal, M. S., Ebner, M. A., Sabharwall, P., & Sharpe, P. (2010). 
/// Engineering database of liquid salt thermophysical and thermochemical 
/// properties (No. INL/EXT-10-18297). Idaho National Lab.(INL), 
/// Idaho Falls, ID (United States).
///
/// Romatoski writes that Gierszewski et al. had a correlation for 
/// 66 mol% LiF, 34 mol% BeF2 for dynamic_viscosity in cP, temperature 
/// in kelvin
///
/// mu (cP) = 0.116 exp(3760/T(K))
/// Applicable from 600-1200 K
///
/// Beyond this range, there is no viscosity data for this same composition,
/// but Romatoski writes that Abe et al, had data for
/// 66 mol% LiF, 34 mol% BeF2 for dynamic_viscosity in cP, temperature 
/// in kelvin
///
/// mu (cP) = 0.07803 exp(4022/T(K))
/// Applicable from 812.5 - 1573 K 
///
/// 
/// There is some discrepancy within the literature data, 
/// but I suppose for this code, 
/// Abe's correlation can work from 1200 - 1573 K 
///
///
/// There will be obvious discontinuity at 1200K, but I'll leave it 
/// for future patches
///
/// in totality, 600-1573 K is reasonable, but 
/// freezing point is 732.2
/// 
/// 
pub fn get_flibe_dynamic_viscosity(
    fluid_temp: ThermodynamicTemperature) -> Result<DynamicViscosity,
TuasLibError>{

    range_check_flibe_salt(fluid_temp)?;
    let fluid_temp_kelvin = fluid_temp.get::<kelvin>();
    // mu centipoise (T = 600 - 1200 K) 
    // mu (cP) = 0.116 exp(3760/T(K))
    // 
    // 
    // generic form:  
    // mu = a * exp (b/T[K])
    let mut a = 0.116;
    let mut b = 3760_f64;
    // c, d and e variables need not be here,
    // but I'm leaving it as a legacy thing
    let mut _c = 0.0;
    let mut _d = 0.0;
    let mut _e = 0.0;

    if fluid_temp_kelvin > 1200.0 {
        // Abe's correlation can work from 1200 - 1573 K 
        // mu (cP) = 0.07803 exp(4022/T(K))
        a = 0.07803;
        b = 4022_f64;
        _c = 0.0;
        _d = 0.0;
        _e = 0.0;

    }

    let viscosity_value_centipoise = a * (b/fluid_temp_kelvin).exp();

    Ok(DynamicViscosity::new::<centipoise>(viscosity_value_centipoise))
                                
}


#[test]
pub fn flibe_salt_test_viscosity(){
    // going to perform 1 test
    //
    // From
    // Romatoski, R. R., & Hu, L. W. (2017). Fluoride salt coolant properties 
    // for nuclear reactor applications: A review. Annals 
    // of Nuclear Energy, 109, 635-647.
    // properties for a custom liquid material 
    // not covered in the database
    //
    // Figure 3 page 7, there is a graph providing some values of viscosity
    //
    // at roughly 900K viscosity was 7.5 cP from the graph 
    // (didn't use graphreader)
    //
    // I'm going to use this to test 
    //
    //

    use uom::si::thermodynamic_temperature::kelvin;
    use uom::si::dynamic_viscosity::centipoise;
    extern crate approx;
    // let's try the 900 F one first 
    let temperature_900_k = 
        ThermodynamicTemperature::new::<kelvin>(900.0);

    // let's get the viscosity, should be around 7.5 cP 
    let viscosity_900_k = 
        get_flibe_dynamic_viscosity(temperature_900_k).unwrap();

    let viscosity_value_centipoise_900_k = 
        viscosity_900_k.get::<centipoise>();

    // we expect a dynamic viscosity of around 7.5 cP at this temperature
    // we have +/- 2% uncertainty
    approx::assert_relative_eq!(
        7.5, 
        viscosity_value_centipoise_900_k, 
        max_relative=0.02);



}

/// function to obtain flibe salt specific heat capacity
/// given a temperature
/// Romatoski, R. R., & Hu, L. W. (2017). Fluoride salt coolant properties 
/// for nuclear reactor applications: A review. Annals 
/// of Nuclear Energy, 109, 635-647.
/// properties for a custom liquid material 
/// not covered in the database
///
/// Sohal, M. S., Ebner, M. A., Sabharwall, P., & Sharpe, P. (2010). 
/// Engineering database of liquid salt thermophysical and thermochemical 
/// properties (No. INL/EXT-10-18297). Idaho National Lab.(INL), 
/// Idaho Falls, ID (United States).
///
/// It is quite invariant with temperature
/// 
/// values range from 2415.8 J/(kg K) to 2386 J/(kg K)
///
/// Lichtenstein had a cp value of 1860 J/(kg K), but this 
/// lowered value was attributed to BeO impurities within the FLiBe
///
/// Lichtenstein, T., Rose, M. A., Krueger, J., Wu, E., & 
/// Williamson, M. A. (2022). Thermochemical Property Measurements of 
/// FLiNaK and FLiBe in FY 2020 (No. ANL/CFCT-20/37 Rev. 1). 
/// Argonne National Lab.(ANL), Argonne, IL (United States).
///
/// It is more reasonable to take the 2386 J/(kg K) value as this had an 
/// uncertainty of +/- 3% 
///
/// the 2415.6 value had an uncertainty of about +/- 20%
///
/// The temperature range is for all fluid temperatures, from 
/// 732.2K all the way up to 4498.8K (ish), the triple point
///
///
pub fn get_flibe_constant_pressure_specific_heat_capacity(
    fluid_temp: ThermodynamicTemperature) -> Result<SpecificHeatCapacity,
TuasLibError>{

    range_check_flibe_salt(fluid_temp)?;
    // note, specific entropy and heat capcity are the same unit...
    //
    let _temperature_degrees_c_value = fluid_temp.get::<degree_celsius>();
    let cp_value_joule_per_kg = 2386.0;

    Ok(SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(
        cp_value_joule_per_kg))
}

/// function to obtain flibe salt thermal conductivity
/// given a temperature.
/// Data was obtained from the following publications
///
/// Sohal, M. S., Ebner, M. A., Sabharwall, P., & Sharpe, P. (2010). 
/// Engineering database of liquid salt thermophysical and thermochemical 
/// properties (No. INL/EXT-10-18297). Idaho National Lab.(INL), 
/// Idaho Falls, ID (United States).
///
/// Romatoski, R. R., & Hu, L. W. (2017). Fluoride salt coolant properties 
/// for nuclear reactor applications: A review. Annals 
/// of Nuclear Energy, 109, 635-647.
///
/// Thermal conductivity is in the range of 1.1 W/(m K) in 873K to 1073K,
/// and Sohal's correlation was originally for 500-650 K
/// Which is strangely below the melting 
/// point of flibe
/// but based on Romatoski's data, I found that Romatoski's data 
/// could fit Sohal's correlation to within 10% error
/// even up to 1123 K in my PhD Thesis
///
/// Therefore I could use it in the whole temperature range from 
/// 500 - 1123K .
///
///
/// Ong, T. K. C. (2024). Digital Twins as Testbeds for 
/// Iterative Simulated Neutronics Feedback Controller 
/// Development (Doctoral dissertation, UC Berkeley).
///
/// k (thermal conductivity in W/mK for T = 500-1123 kelvin) = 
/// 0.629697 + 0.0005 T[K]
///
/// For more than 1123 K, I'll just let the conductivity be 
/// the value obtained at 1123 K, seeing how in Sohal, the value is 
/// does not seem to vary greatly with temperature 
///
/// T in kelvin
pub fn get_flibe_thermal_conductivity(
    fluid_temp: ThermodynamicTemperature) -> Result<ThermalConductivity,TuasLibError> {


    range_check_flibe_salt(fluid_temp)?;
    let mut fluid_temp_kelvin = fluid_temp.get::<kelvin>();
    // k (thermal conductivity in W/mK for T = 500-1123 kelvin) = 
    // 0.629697 + 0.0005 T[K]
    let a = 0.629697;
    let b = 0.0005;
    let c = 0.0;
    let d = 0.0;
    let e = 0.0;

    if fluid_temp_kelvin > 1123.0 {
        // For more than 1123 K, I'll just let the conductivity be 
        // the value obtained at 1123 K, seeing how in Sohal, the value is 
        // does not seem to vary greatly with temperature 
        fluid_temp_kelvin = 1123.0;
    }

    // generic correlation is:
    // a + bT + cT^2 + dT^3 + eT^4;
    let thermal_conductivity_value = 
        a 
        + b * fluid_temp_kelvin
        + c * fluid_temp_kelvin.powf(2.0)
        + d * fluid_temp_kelvin.powf(3.0)
        + e * fluid_temp_kelvin.powf(4.0);

    return Ok(ThermalConductivity::new::<watt_per_meter_kelvin>(
        thermal_conductivity_value));
}

/// FLiBe prandtl number is widely known in literature by people in 
/// Th Lab, UC Berkeley around 2015 
/// Eg. Zweibaum
///
/// I replotted some of these plots in my PhD thesis
/// 
/// Ong, T. K. C. (2024). Digital Twins as Testbeds for 
/// Iterative Simulated Neutronics Feedback Controller 
/// Development (Doctoral dissertation, UC Berkeley).
///
/// A simple test to check if the thermophysical property 
/// values are reasonable is to calculate prandtl number at various 
/// temperatures
///
/// at 560 C, Pr is around 24.0
/// at 730 C, Pr is around 10.5
///
/// These are the values I'm using to test the FLiBe correlations
#[test]
pub fn flibe_prandtl_number(){

    let prandtl_fn= |flibe_temp: ThermodynamicTemperature|{

        let mu = get_flibe_dynamic_viscosity(flibe_temp).unwrap();
        let cp = get_flibe_constant_pressure_specific_heat_capacity(flibe_temp).unwrap();
        let k = get_flibe_thermal_conductivity(flibe_temp).unwrap();

        // return prandtl number
        return mu*cp/k;

    };

    use uom::si::ratio::ratio;

    let temp_1_560_c = ThermodynamicTemperature::new::<degree_celsius>(560.0);
    let temp_2_730_c = ThermodynamicTemperature::new::<degree_celsius>(730.0);

    // at 560 C, Pr is around 24.0
    // accurate to within 2%
    let prandtl_560c = prandtl_fn(temp_1_560_c);

    approx::assert_relative_eq!(
        prandtl_560c.get::<ratio>(), 
        24.0, 
        max_relative=0.02);

    // at 730c, Pr is around 10.5
    // accurate to within 2%
    let prandtl_730c = prandtl_fn(temp_2_730_c);

    approx::assert_relative_eq!(
        prandtl_730c.get::<ratio>(), 
        10.5, 
        max_relative=0.02);
}

/// function to obtain flibe salt specific enthalpy
/// given a temperature
///
/// Sohal, M. S., Ebner, M. A., Sabharwall, P., & Sharpe, P. (2010). 
/// Engineering database of liquid salt thermophysical and thermochemical 
/// properties (No. INL/EXT-10-18297). Idaho National Lab.(INL), 
/// Idaho Falls, ID (United States).
///
/// Romatoski, R. R., & Hu, L. W. (2017). Fluoride salt coolant properties 
/// for nuclear reactor applications: A review. Annals 
/// of Nuclear Energy, 109, 635-647.
///
/// cp (J/kg/K) = 2389.0, T in kelvin
///
/// Manual integration with temperature yields:
///
/// h (J/kg) = 2389.0 T(K) + Constant
///
/// I can just adjust the enthalpy to be 0 J/kg at 732.2 K, which is 
/// the low bound temperature for FLiBe
///
/// 0.0 = 2389.0 * 732.2 + Constant
///
pub fn get_flibe_specific_enthalpy(
    fluid_temp: ThermodynamicTemperature) -> 
Result<AvailableEnergy,TuasLibError>{

    range_check_flibe_salt(fluid_temp)?;
    // note, specific entropy and heat capcity are the same unit...
    //
    // h (J/kg) = 2389.0 T(K) + Constant
    // Where:
    // 0.0 = 2389.0 * 732.2 + Constant
    let low_bound_temp_kelvin = 732.2;
    let cp_val_constant_joule_per_kilogram_kelvin = 2389.0;
    let temp_kelvin_value = fluid_temp.get::<kelvin>();
    let enthalpy_value_joule_per_kg 
        = -low_bound_temp_kelvin * cp_val_constant_joule_per_kilogram_kelvin
        + cp_val_constant_joule_per_kilogram_kelvin * temp_kelvin_value;

    // the closest unit available is AvailableEnergy which is
    // joule per kg 

    return Ok(AvailableEnergy::new::<joule_per_kilogram>(
        enthalpy_value_joule_per_kg));
}



/// function to obtain flibe salt temperature from specific enthalpy
///
/// Sohal, M. S., Ebner, M. A., Sabharwall, P., & Sharpe, P. (2010). 
/// Engineering database of liquid salt thermophysical and thermochemical 
/// properties (No. INL/EXT-10-18297). Idaho National Lab.(INL), 
/// Idaho Falls, ID (United States).
///
///
/// Note that the enthalpy equation was derived from manual 
/// integration of cp assuming 0 J/kg at 732.2K (the minimum temperature)
///
/// h (J/kg) = 2389.0 T(K) + Constant
///
/// I can just adjust the enthalpy to be 0 J/kg at 732.2 K, which is 
/// the low bound temperature for FLiBe
///
/// 0.0 = 2389.0 * 732.2 + Constant
///
///
pub fn get_temperature_from_enthalpy(
    fluid_enthalpy: AvailableEnergy) -> Result<ThermodynamicTemperature,TuasLibError> {

    // if enthalpy value below zero,
    // based on me setting zero enthalpy at the lower end of the 
    // temperature validity range for enthalpy,
    // then enthalpy is technically out of range
    if fluid_enthalpy.value < 0_f64 {
        panic!("FLiBe : get_temperature_from_enthalpy \n
               enthalpy < 0.0 , out of correlation range");
    }

    // first let's convert enthalpy to a double (f64)
    let enthalpy_value_joule_per_kg = 
        fluid_enthalpy.get::<joule_per_kilogram>();

    // second let's define a function 
    // or actually a closure or anonymous function that
    // is aware of the variables declared
    // LHS is actual enthalpy value

    let enthalpy_root = |temp_degrees_kelvin_value : f64| -> f64 {
        let lhs_value = enthalpy_value_joule_per_kg;
        let temp_degrees_kelvin_value_double = temp_degrees_kelvin_value;

        let fluid_temperature = 
            ThermodynamicTemperature::new::<kelvin>(
                temp_degrees_kelvin_value_double);
        let rhs = get_flibe_specific_enthalpy(fluid_temperature).unwrap();
        let rhs_value = rhs.get::<joule_per_kilogram>();

        return lhs_value-rhs_value;
    };
    
    // now solve using bisection
    // the range is from 732.2 K - 1573 K
    
    use anyhow::Result;
    let fluid_temperature_degrees_kelvin_result 
        = bisection!(enthalpy_root,
                    (732.2,1573.0),
                    100,
                    1e-8);

    let fluid_temperature_degrees_kelvin = fluid_temperature_degrees_kelvin_result.unwrap();

    return Ok(ThermodynamicTemperature::
        new::<kelvin>(fluid_temperature_degrees_kelvin));

}

/// function checks if a fluid temperature falls in a range 
///
/// If it falls outside this range, it will panic
/// or throw an error, and the program will not run
///
/// Sohal, M. S., Ebner, M. A., Sabharwall, P., & Sharpe, P. (2010). 
/// Engineering database of liquid salt thermophysical and thermochemical 
/// properties (No. INL/EXT-10-18297). Idaho National Lab.(INL), 
/// Idaho Falls, ID (United States).
///
/// For FLiBe, the applicable range is 732.2K (melting point) - 1573 K.
/// I try to make the range as wide as possible because Gnielinski's correlation 
/// requires corrections using wall temperature. These may be outside 
/// the usual bulk temperatures of FLiBe.
///
/// 
/// thermal conductivity is extrapolated (constant till 1573 K, no data 
/// exists there)
/// viscosity is all the way up to 732.2 K - 1573 K (Abe's correlation
/// forms the upper bound limit)
/// 
///
///
pub fn range_check_flibe_salt(fluid_temp: ThermodynamicTemperature) 
    -> Result<bool,TuasLibError>{

        // first i convert the fluidTemp object into a degree 
        // celsius

        range_check(&Material::Liquid(LiquidMaterial::FLiBe), 
            fluid_temp, 
            ThermodynamicTemperature::new::<kelvin>(1573.0), 
            ThermodynamicTemperature::new::<kelvin>(732.2))?;

        return Ok(true);

    }



/// flibe max temp 
pub fn max_temp_flibe() -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(1573.0)

}
/// flibe min temp 
pub fn min_temp_flibe() -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(732.2)
}
