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
use uom::si::dynamic_viscosity::pascal_second;
use uom::si::thermal_conductivity::watt_per_meter_kelvin;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::available_energy::joule_per_kilogram;

// this is for the root finding algorithms
extern crate peroxide;
use peroxide::prelude::*;

use crate::boussinesq_thermophysical_properties::{range_check, LiquidMaterial, Material};
use crate::thermal_hydraulics_error::TuasLibError;

/// function to obtain nitrate salt density
/// given a temperature
///
/// Du, B. C., He, Y. L., Qiu, Y., Liang, Q., & Zhou, Y. P. (2018). 
/// Investigation on heat transfer characteristics of molten salt in 
/// a shell-and-tube heat exchanger. International Communications 
/// in Heat and Mass Transfer, 96, 61-68.
/// 
///
/// rho (kg/m3) = 2280.22  - 0.733 T(K)
pub fn get_hitec_density(
    fluid_temp: ThermodynamicTemperature) -> Result<MassDensity,TuasLibError> {


    // first we check if fluid temp is between 440-800 K (range of validity)
    // panic otherwise
    range_check_hitec_salt(fluid_temp)?;
    let fluid_temp_kelvin = fluid_temp.get::<kelvin>();
    let a = 2280.22;
    let b = -0.733;
    // generic correlation is:
    // a + bT + cT^2 + dT^3 + eT^4;

    let density_value_kg_per_m3 = 
        a 
        + b * fluid_temp_kelvin;


    return Ok(MassDensity::new::<
              kilogram_per_cubic_meter>(density_value_kg_per_m3));
}

/// function to obtain nitrate salt viscosity
/// given a temperature
///
/// Du, B. C., He, Y. L., Qiu, Y., Liang, Q., & Zhou, Y. P. (2018). 
/// Investigation on heat transfer characteristics of molten salt in 
/// a shell-and-tube heat exchanger. International Communications 
/// in Heat and Mass Transfer, 96, 61-68.
///
/// mu Pa-s (T = 440 - 500 K) 
/// = 0.93845
/// -0.54754 T(K)
/// + 1.08225e-5 T(K)^2
/// - 7.2058e-9 T(K)^3
///
/// mu Pa-s (T = 500 - 800 K) 
/// = 0.23816
/// - 1.2768e-3 T(K)
/// + 2.6275e-6 T(K)^2
/// - 2.4331e-9 T(K)^3
/// + 8.507e-13 T(K)^4
///
/// Bohlmann, E. G. (1972). HEAT TRANSFER SALT FOR HIGH TEMPERATURE 
/// STEAM GENERATION (No. ORNL-TM-3777). Oak Ridge National 
/// Lab.(ORNL), Oak Ridge, TN (United States).
///
/// given the complicated looking correlations, it's always good to 
/// against data. I'm using Bohlman's data for HITEC salt in 1972 
/// as comparison. Fig 6 on page 25 of the document shows a graph 
/// of HITEC salt viscosity in centipoises against temperature in 
/// Fahrenheit
///
/// Using graphreader, I got the following pieces of data for viscosity 
/// in cP against temp in Fahrenheit (roughly, the curve axes were 
/// tilted)
///
///
/// 315.282,15.039
/// 336.479,12.087
/// 346.338,10.984
/// 375.915,8.642
/// 399.577,7.362
/// 440.986,5.709
/// 498.169,4.272
/// 585.915,3.051
/// 653.944,2.5
/// 730.845,1.988
/// 832.394,1.555
/// 928.521,1.28
///
/// I can use a simple test to ascertain if the viscosity is close 
/// to this value
///
///
///
/// 
pub fn get_hitec_dynamic_viscosity(
    fluid_temp: ThermodynamicTemperature) -> Result<DynamicViscosity,
TuasLibError>{

    range_check_hitec_salt(fluid_temp)?;
    let fluid_temp_kelvin = fluid_temp.get::<kelvin>();
    // mu Pa-s (T = 500 - 800 K) 
    // = 0.23816
    // - 1.2768e-3 T(K)
    // + 2.6275e-6 T(K)^2
    // - 2.4331e-9 T(K)^3
    // + 8.507e-13 T(K)^4
    let mut a = 0.23816;
    let mut b = - 1.2768e-3;
    let mut c = 2.6275e-6;
    let mut d = -2.4331e-9;
    let mut e = 8.507e-13;

    if fluid_temp_kelvin < 500.0 {
        // mu Pa-s (T = 440 - 500 K) 
        // = 0.93845
        // -0.54754 T(K)
        // + 1.08225e-5 T(K)^2
        // - 7.2058e-9 T(K)^3
        a = 0.93845;
        b = - 5.4754e-3;
        c = 1.08225e-5;
        d = -7.2058e-9;
        e = 0.0;

    }

    // generic correlation is:
    // a + bT + cT^2 + dT^3 + eT^4;
    let viscosity_value_pascal_second = 
        a 
        + b * fluid_temp_kelvin
        + c * fluid_temp_kelvin.powf(2.0)
        + d * fluid_temp_kelvin.powf(3.0)
        + e * fluid_temp_kelvin.powf(4.0);

    Ok(DynamicViscosity::new::<pascal_second>(viscosity_value_pascal_second))
                                
}


#[test]
pub fn hitec_nitrate_salt_test_viscosity(){
    // going to perform 2 tests here
    //
    // From
    // Bohlmann, E. G. (1972). HEAT TRANSFER SALT FOR HIGH TEMPERATURE 
    // STEAM GENERATION (No. ORNL-TM-3777). Oak Ridge National 
    // Lab.(ORNL), Oak Ridge, TN (United States).
    //
    // figure 6 page 25, the HITEC salt temperature in Fahrenheit 
    // was given and the resulting viscosity was plotted
    //
    // T(F), mu (cP)
    // 346.338,10.984
    // 653.944,2.5
    //
    // These two temperautres were chosen because there are two 
    // correlations used by Du 
    //
    // first in the 440-500K range. This is where 
    // the 346 F or 447 K temperature is used 
    //
    // then in the 500K-800K range, where the 
    // 653 F or 618 K temperature is used
    //
    // No error bars were given, but based on Sohal's work 
    // typical error bars from Janz were as high as 16% 
    //
    // Sohal, M. S., Ebner, M. A., Sabharwall, P., & Sharpe, P. (2010). 
    // Engineering database of liquid salt thermophysical and 
    // thermochemical properties (No. INL/EXT-10-18297). 
    // Idaho National Lab.(INL), Idaho Falls, ID (United States).
    //

    use uom::si::thermodynamic_temperature::degree_fahrenheit;
    use uom::si::dynamic_viscosity::centipoise;
    extern crate approx;
    // let's try the 346 F one first 
    let temperature_346_f = 
        ThermodynamicTemperature::new::<degree_fahrenheit>(
            346.338);

    // let's get the viscosity, should be around 11 cP 
    let viscosity_346_f = 
        get_hitec_dynamic_viscosity(temperature_346_f).unwrap();

    let viscosity_value_centipoise_346_f = 
        viscosity_346_f.get::<centipoise>();

    // we expect a dynamic viscosity of around 11 cP at this temperature
    // we have +/- 16% uncertainty
    approx::assert_relative_eq!(
        10.984, 
        viscosity_value_centipoise_346_f, 
        max_relative=0.16);

    // let's try the 654 F one first 
    let temperature_654_f = 
        ThermodynamicTemperature::new::<degree_fahrenheit>(
            653.944);

    // let's get the viscosity, should be around 2.5 cP 
    let viscosity_654_f = 
        get_hitec_dynamic_viscosity(temperature_654_f).unwrap();

    let viscosity_value_centipoise_654f = 
        viscosity_654_f.get::<centipoise>();

    // we expect a dynamic viscosity of around 2.5 cP at this temperature
    // we have +/- 16% uncertainty
    approx::assert_relative_eq!(
        2.5, 
        viscosity_value_centipoise_654f, 
        max_relative=0.16);


}

/// function to obtain nitrate salt specific heat capacity
/// given a temperature
/// Du, B. C., He, Y. L., Qiu, Y., Liang, Q., & Zhou, Y. P. (2018). 
/// Investigation on heat transfer characteristics of molten salt in 
/// a shell-and-tube heat exchanger. International Communications 
/// in Heat and Mass Transfer, 96, 61-68.
///
/// cp (J/kg/K) = 1560.0 
/// T in kelvin
///
/// Now, Sohal has a different correlation for cp 
///
/// Sohal, M. S., Ebner, M. A., Sabharwall, P., & Sharpe, P. (2010). 
/// Engineering database of liquid salt thermophysical and 
/// thermochemical properties (No. INL/EXT-10-18297). 
/// Idaho National Lab.(INL), Idaho Falls, ID (United States).
///
/// but I'm not going to consider that yet
///
pub fn get_hitec_constant_pressure_specific_heat_capacity(
    fluid_temp: ThermodynamicTemperature) -> Result<SpecificHeatCapacity,
TuasLibError>{

    range_check_hitec_salt(fluid_temp)?;
    // note, specific entropy and heat capcity are the same unit...
    //
    let _temperature_degrees_c_value = fluid_temp.get::<degree_celsius>();
    let cp_value_joule_per_kg = 1560.0;

    Ok(SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(
        cp_value_joule_per_kg))
}

/// function to obtain nitrate salt thermal conductivity
/// given a temperature
///
/// Du, B. C., He, Y. L., Qiu, Y., Liang, Q., & Zhou, Y. P. (2018). 
/// Investigation on heat transfer characteristics of molten salt in 
/// a shell-and-tube heat exchanger. International Communications 
/// in Heat and Mass Transfer, 96, 61-68.
///
/// k (thermal conductivity in W/mK for T = 536-800 kelvin) = 
/// 0.7663 - 6.47e-4 T(K)
///
/// k (thermal conductivity in W/mK for T = 420-536 kelvin) = 
/// 2.2627 - 0.01176 T(K)
/// + 2.551e-5 T(K)^2 
/// - 1.863e-8 T(K)^3
///
/// T in kelvin
pub fn get_hitec_thermal_conductivity(
    fluid_temp: ThermodynamicTemperature) -> Result<ThermalConductivity,TuasLibError> {


    range_check_hitec_salt(fluid_temp)?;
    let fluid_temp_kelvin = fluid_temp.get::<kelvin>();
    // k (thermal conductivity in W/mK for T = 420-536 kelvin) = 
    // 2.2627 - 0.01176 T(K)
    // + 2.551e-5 T(K)^2 
    // - 1.863e-8 T(K)^3
    let mut a = 2.2627;
    let mut b = - 0.01176;
    let mut c = 2.551e-5;
    let mut d = -1.863e-8;
    let mut e = 0.0;

    if fluid_temp_kelvin > 536.0 {
        // k (thermal conductivity in W/mK for T = 420-536 kelvin) = 
        // 2.2627 - 0.01176 T(K)
        // + 2.551e-5 T(K)^2 
        // - 1.863e-8 T(K)^3
        a = 0.7663;
        b = - 6.47e-4;
        c = 0.0;
        d = 0.0;
        e = 0.0;

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

/// function to obtain nitrate salt specific enthalpy
/// given a temperature
/// Du, B. C., He, Y. L., Qiu, Y., Liang, Q., & Zhou, Y. P. (2018). 
/// Investigation on heat transfer characteristics of molten salt in 
/// a shell-and-tube heat exchanger. International Communications 
/// in Heat and Mass Transfer, 96, 61-68.
///
/// cp (J/kg/K) = 1560.0 
/// T in kelvin
///
/// Manual integration with temperature yields:
///
/// h (J/kg) = 1560.0 T(K) + Constant
///
/// I can just adjust the enthalpy to be 0 J/kg at 440K
///
/// 0 J/kg = 1560 * T_0 (K) + Constant
/// Constant = 0 - 1560 T_0 (K)
/// Constant = 0 - 1560 * 440
/// Constant = 0 - 686,400
/// 
/// h (J/kg) = 1560.0 T(K) - 686400
/// h (J/kg) = - 686400 + 1560.0 T(K) 
///
///
///
pub fn get_hitec_specific_enthalpy(
    fluid_temp: ThermodynamicTemperature) -> 
Result<AvailableEnergy,TuasLibError>{

    range_check_hitec_salt(fluid_temp)?;
    // note, specific entropy and heat capcity are the same unit...
    //
    // h (J/kg) = - 686400 + 1560.0 T(K) 
    let temp_kelvin_value = fluid_temp.get::<kelvin>();
    let enthalpy_value_joule_per_kg 
        = -686400_f64 
        + 1560.0 * temp_kelvin_value;

    // the closest unit available is AvailableEnergy which is
    // joule per kg 

    return Ok(AvailableEnergy::new::<joule_per_kilogram>(
        enthalpy_value_joule_per_kg));
}



/// function to obtain nitrate salt temperature from specific enthalpy
/// Du, B. C., He, Y. L., Qiu, Y., Liang, Q., & Zhou, Y. P. (2018). 
/// Investigation on heat transfer characteristics of molten salt in 
/// a shell-and-tube heat exchanger. International Communications 
/// in Heat and Mass Transfer, 96, 61-68.
///
///
/// Note that the enthalpy equation was derived from manual 
/// integration of cp assuming 0 J/kg at 440K (the minimum temperature)
///
/// 0 J/kg = 1560 * T_0 (K) + Constant
/// Constant = 0 - 1560 T_0 (K)
/// Constant = 0 - 1560 * 440
/// Constant = 0 - 686,400
/// 
/// h (J/kg) = 1560.0 T(K) - 686400
/// h (J/kg) = - 686400 + 1560.0 T(K) 
///
///
pub fn get_temperature_from_enthalpy(
    fluid_enthalpy: AvailableEnergy) -> Result<ThermodynamicTemperature,TuasLibError> {

    // if enthalpy value below zero,
    // based on me setting zero enthalpy at the lower end of the 
    // temperature validity range for enthalpy,
    // then enthalpy is technically out of range
    if fluid_enthalpy.value < 0_f64 {
        panic!("HITEC : get_temperature_from_enthalpy \n
               enthalpy < 0.0 , out of correlation range");
    }

    // first let's convert enthalpy to a double (f64)
    let enthalpy_value_joule_per_kg = 
        fluid_enthalpy.get::<joule_per_kilogram>();

    // second let's define a function 
    // or actually a closure or anonymous function that
    // is aware of the variables declared
    // enthalpy value = 1518*T +2.82/2.0 T^2 - 30924
    // LHS is actual enthalpy value

    let enthalpy_root = |temp_degrees_kelvin_value : f64| -> f64 {
        let lhs_value = enthalpy_value_joule_per_kg;
        let temp_degrees_kelvin_value_double = temp_degrees_kelvin_value;

        let fluid_temperature = 
            ThermodynamicTemperature::new::<kelvin>(
                temp_degrees_kelvin_value_double);
        let rhs = get_hitec_specific_enthalpy(fluid_temperature).unwrap();
        let rhs_value = rhs.get::<joule_per_kilogram>();

        return lhs_value-rhs_value;
    };
    
    // now solve using bisection
    // the range is from 440 K - 800 K
    
    use anyhow::Result;
    let fluid_temperature_degrees_kelvin_result 
        = bisection!(enthalpy_root,
                    (440.0,800.0),
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
/// Du, B. C., He, Y. L., Qiu, Y., Liang, Q., & Zhou, Y. P. (2018). 
/// Investigation on heat transfer characteristics of molten salt in 
/// a shell-and-tube heat exchanger. International Communications 
/// in Heat and Mass Transfer, 96, 61-68./// Jana, S. S., 
///
/// From HITEC, the applicable range is 440K - 800 K, 
///
/// In Du's paper, the viscosity correlation is applicable from 440 to 800K
/// while the rest of the properties are from 420-800K
/// 
///
pub fn range_check_hitec_salt(fluid_temp: ThermodynamicTemperature) 
    -> Result<bool,TuasLibError>{

        // first i convert the fluidTemp object into a degree 
        // celsius

        range_check(&Material::Liquid(LiquidMaterial::HITEC), 
            fluid_temp, 
            max_temp_hitec(), 
            min_temp_hitec()
            )?;

        return Ok(true);

    }


#[inline]
/// hitec max temp 
pub fn max_temp_hitec() -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(800.0)

}
#[inline]
/// hitec min temp 
pub fn min_temp_hitec() -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(440.0)
}
