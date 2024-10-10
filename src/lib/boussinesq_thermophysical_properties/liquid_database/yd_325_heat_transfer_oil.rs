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
// Additions to it were made in Singapore Nuclear Research and Safety 
// Institute (SNRSI) in National University of Singapore (NUS)
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
use uom::si::thermodynamic_temperature::kelvin;
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::dynamic_viscosity::pascal_second;
use uom::si::thermal_conductivity::watt_per_meter_kelvin;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::available_energy::joule_per_kilogram;

// this is for the root finding algorithms
extern crate peroxide;
use peroxide::prelude::*;

use crate::boussinesq_thermophysical_properties::{range_check, LiquidMaterial, Material};
use crate::thermal_hydraulics_error::ThermalHydraulicsLibError;

/// function to obtain yd_325_heat_transfer_oil density
/// given a temperature
///
/// Du, B. C., He, Y. L., Qiu, Y., Liang, Q., & Zhou, Y. P. (2018). 
/// Investigation on heat transfer characteristics of molten salt in 
/// a shell-and-tube heat exchanger. International Communications 
/// in Heat and Mass Transfer, 96, 61-68.
/// 
/// Qiu, Y., Li, M. J., Wang, W. Q., Du, B. C., & Wang, K. (2018). 
/// An experimental study on the heat transfer performance of a prototype 
/// molten-salt rod baffle heat exchanger for concentrated solar power. 
/// Energy, 156, 63-72.
///
/// rho (kg/m3) = 1199.13  - 0.6311 T(K)
pub fn get_yd325_density(
    fluid_temp: ThermodynamicTemperature) -> Result<MassDensity,ThermalHydraulicsLibError> {


    // first we check if fluid temp is between 440-800 K (range of validity)
    // panic otherwise
    range_check_yd325_oil(fluid_temp)?;
    let fluid_temp_kelvin = fluid_temp.get::<kelvin>();
    let a = 1199.13;
    let b = -0.6311;
    // generic correlation is:
    // a + bT + cT^2 + dT^3 + eT^4;

    let density_value_kg_per_m3 = 
        a 
        + b * fluid_temp_kelvin;


    return Ok(MassDensity::new::<
              kilogram_per_cubic_meter>(density_value_kg_per_m3));
}

/// function to obtain yd_325_heat_transfer_oil viscosity
/// given a temperature
///
/// Du, B. C., He, Y. L., Qiu, Y., Liang, Q., & Zhou, Y. P. (2018). 
/// Investigation on heat transfer characteristics of molten salt in 
/// a shell-and-tube heat exchanger. International Communications 
/// in Heat and Mass Transfer, 96, 61-68.
///
/// mu Pa-s (T = 323-423 K) 
/// = 0.33065
/// - 2.283e-3 T(K)
/// + 5.2746e-6 T(K)^2
/// - 4.066e-9 T(K)^3
///
/// mu Pa-s (T = 423-523 K) 
/// = 0.05989
/// - 3.452e-4 T(K)
/// + 6.735e-7 T(K)^2
/// - 4.413e-10 T(K)^3
/// 
///
///
///
/// 
pub fn get_yd325_dynamic_viscosity(
    fluid_temp: ThermodynamicTemperature) -> Result<DynamicViscosity,
ThermalHydraulicsLibError>{

    range_check_yd325_oil(fluid_temp)?;
    let fluid_temp_kelvin = fluid_temp.get::<kelvin>();
    // mu Pa-s (T = 423-523 K) 
    // = 0.05989
    // - 3.452e-4 T(K)
    // + 6.735e-7 T(K)^2
    // - 4.413e-10 T(K)^3
    let mut a = 0.05989;
    let mut b = - 3.452e-4;
    let mut c = 6.735e-7;
    let mut d = -4.413e-10;
    let mut e = 0.0;

    if fluid_temp_kelvin < 423.0 {
        // mu Pa-s (T = 323-423 K) 
        // = 0.33065
        // - 2.283e-3 T(K)
        // + 5.2746e-6 T(K)^2
        // - 4.066e-9 T(K)^3
        a = 0.33065;
        b = - 2.283e-3;
        c = 5.2746e-6;
        d = -4.066e-9;
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



/// function to obtain yd_325_heat_transfer_oil specific heat capacity
/// given a temperature
/// Du, B. C., He, Y. L., Qiu, Y., Liang, Q., & Zhou, Y. P. (2018). 
/// Investigation on heat transfer characteristics of molten salt in 
/// a shell-and-tube heat exchanger. International Communications 
/// in Heat and Mass Transfer, 96, 61-68.
///
/// Qiu, Y., Li, M. J., Wang, W. Q., Du, B. C., & Wang, K. (2018). 
/// An experimental study on the heat transfer performance of a prototype 
/// molten-salt rod baffle heat exchanger for concentrated solar power. 
/// Energy, 156, 63-72.
///
/// cp (J/kg/K) = 776.0 + 3.40 T(K)
/// T in kelvin
///
///
#[inline]
pub fn get_yd325_constant_pressure_specific_heat_capacity(
    fluid_temp: ThermodynamicTemperature) -> Result<SpecificHeatCapacity,
ThermalHydraulicsLibError>{

    range_check_yd325_oil(fluid_temp)?;
    // note, specific entropy and heat capcity are the same unit...
    //
    let fluid_temp_kelvin = fluid_temp.get::<kelvin>();
    let a = 776.0;
    let b = 3.40;
    // generic correlation is:
    // a + bT + cT^2 + dT^3 + eT^4;

    let cp_value_joule_per_kg = 
        a 
        + b * fluid_temp_kelvin;

    Ok(SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(
        cp_value_joule_per_kg))
}

/// function to obtain yd_325_heat_transfer_oil thermal conductivity
/// given a temperature
///
/// Du, B. C., He, Y. L., Qiu, Y., Liang, Q., & Zhou, Y. P. (2018). 
/// Investigation on heat transfer characteristics of molten salt in 
/// a shell-and-tube heat exchanger. International Communications 
/// in Heat and Mass Transfer, 96, 61-68.
///
/// Qiu, Y., Li, M. J., Wang, W. Q., Du, B. C., & Wang, K. (2018). 
/// An experimental study on the heat transfer performance of a prototype 
/// molten-salt rod baffle heat exchanger for concentrated solar power. 
/// Energy, 156, 63-72.
///
/// lambda = 0.1416 - 6.68e-5 T(K)
///
/// T in kelvin
pub fn get_yd325_thermal_conductivity(
    fluid_temp: ThermodynamicTemperature) -> Result<ThermalConductivity,ThermalHydraulicsLibError> {


    range_check_yd325_oil(fluid_temp)?;
    let fluid_temp_kelvin = fluid_temp.get::<kelvin>();
    // k (thermal conductivity in W/mK for T = 300-573 kelvin) = 
    // lambda = 0.1416 - 6.68e-5 T(K)
    let a = 0.1416;
    let b = - 6.68e-5;
    let c = 0.0;
    let d = 0.0;
    let e = 0.0;


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

/// function to obtain yd_325_heat_transfer_oil specific enthalpy
/// given a temperature
///
/// Du, B. C., He, Y. L., Qiu, Y., Liang, Q., & Zhou, Y. P. (2018). 
/// Investigation on heat transfer characteristics of molten salt in 
/// a shell-and-tube heat exchanger. International Communications 
/// in Heat and Mass Transfer, 96, 61-68.
///
/// Qiu, Y., Li, M. J., Wang, W. Q., Du, B. C., & Wang, K. (2018). 
/// An experimental study on the heat transfer performance of a prototype 
/// molten-salt rod baffle heat exchanger for concentrated solar power. 
/// Energy, 156, 63-72.
///
/// cp (J/kg/K) = 776.0 + 3.40 T(K)
/// T in kelvin
///
/// Manual integration with temperature yields:
///
/// h (J/kg) = 776.0 T(K) + 3.40 * 0.5 T(K)^2 + Constant
///
/// Now, I can just "cheat" and perform a definite integral 
/// The reference temperature can be the lower bound temperature of 
/// 323K
///
///
///
///
///
pub fn get_yd325_specific_enthalpy(
    fluid_temp: ThermodynamicTemperature) -> 
Result<AvailableEnergy,ThermalHydraulicsLibError>{

    range_check_yd325_oil(fluid_temp)?;
    let reference_temperature_kelvin = 323.0;

    let temp_kelvin_value = fluid_temp.get::<kelvin>();

    // generic correlation is:
    // a + bT + cT^2 + dT^3 + eT^4;
    // I define those parameters here:
    let a = 0.0;
    let b = 776.0;
    let c = 3.40 * 0.5;
    let d = 0.0;
    let e = 0.0;

    let enthalpy_value_joule_per_kg_calc =  |fluid_temp_kelvin: f64|{
        a + b * fluid_temp_kelvin
            + c * fluid_temp_kelvin.powf(2.0)
            + d * fluid_temp_kelvin.powf(3.0)
            + e * fluid_temp_kelvin.powf(4.0)
    };

    // this is slightly more computationally expensive than other calcs,
    // but perhaps less error prone during development.
    let enthalpy_value_joule_per_kg 
        = enthalpy_value_joule_per_kg_calc(temp_kelvin_value) 
        - enthalpy_value_joule_per_kg_calc(reference_temperature_kelvin);

    // the closest unit available is AvailableEnergy which is
    // joule per kg 

    // note, specific entropy and heat capcity are the same unit...
    return Ok(AvailableEnergy::new::<joule_per_kilogram>(
        enthalpy_value_joule_per_kg));
}



/// function to obtain nitrate salt temperature from specific enthalpy
/// Du, B. C., He, Y. L., Qiu, Y., Liang, Q., & Zhou, Y. P. (2018). 
/// Investigation on heat transfer characteristics of molten salt in 
/// a shell-and-tube heat exchanger. International Communications 
/// in Heat and Mass Transfer, 96, 61-68.
///
/// Qiu, Y., Li, M. J., Wang, W. Q., Du, B. C., & Wang, K. (2018). 
/// An experimental study on the heat transfer performance of a prototype 
/// molten-salt rod baffle heat exchanger for concentrated solar power. 
/// Energy, 156, 63-72.
///
///
///
pub fn get_temperature_from_enthalpy(
    fluid_enthalpy: AvailableEnergy) -> Result<ThermodynamicTemperature,ThermalHydraulicsLibError> {

    // if enthalpy value below zero,
    // based on me setting zero enthalpy at the lower end of the 
    // temperature validity range for enthalpy,
    // then enthalpy is technically out of range
    if fluid_enthalpy.value < 0_f64 {
        panic!("yd_325_heat_transfer_oil : get_temperature_from_enthalpy \n
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
        let rhs = get_yd325_specific_enthalpy(fluid_temperature).unwrap();
        let rhs_value = rhs.get::<joule_per_kilogram>();

        return lhs_value-rhs_value;
    };
    
    // now solve using bisection
    // the range is from 323 K - 523 K
    
    use anyhow::Result;
    let fluid_temperature_degrees_kelvin_result 
        = bisection!(enthalpy_root,
                    (323.0,523.0),
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
/// Qiu, Y., Li, M. J., Wang, W. Q., Du, B. C., & Wang, K. (2018). 
/// An experimental study on the heat transfer performance of a prototype 
/// molten-salt rod baffle heat exchanger for concentrated solar power. 
/// Energy, 156, 63-72.
///
/// From YD-325, the applicable range is 323K - 523 K, 
///
/// In Qiu's paper, the viscosity correlation is applicable from 323-523 K
/// while the rest of the properties are from 300-573 K
/// 
///
pub fn range_check_yd325_oil(fluid_temp: ThermodynamicTemperature) 
    -> Result<bool,ThermalHydraulicsLibError>{

        // first i convert the fluidTemp object into a degree 
        // celsius

        // TBD with range checking
        range_check(&Material::Liquid(LiquidMaterial::YD325), 
            fluid_temp, 
            max_temp_yd325_oil(), 
            min_temp_yd325_oil()
            )?;

        return Ok(true);

    }


#[inline]
/// yd325_oil max temp 
pub fn max_temp_yd325_oil() -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(523.0)

}
#[inline]
/// yd325_oil min temp 
pub fn min_temp_yd325_oil() -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(323.0)
}
