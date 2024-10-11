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
use uom::si::thermodynamic_temperature::kelvin;
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::dynamic_viscosity::centipoise;
use uom::si::thermal_conductivity::watt_per_meter_kelvin;
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::available_energy::joule_per_kilogram;

// this is for the root finding algorithms
extern crate peroxide;
use peroxide::prelude::*;

use crate::boussinesq_thermophysical_properties::{range_check, LiquidMaterial, Material};
use crate::tuas_lib_error::TuasLibError;

/// Romatoski, R. R., & Hu, L. W. (2017). Fluoride salt coolant properties 
/// for nuclear reactor applications: A review. Annals 
/// of Nuclear Energy, 109, 635-647.
///
/// using recommendation by Romatoski to use Janz and Tompkins correlation 
///
/// rho (kg/m3) = 2579 - 0.624 T[K]
///
/// uncertainty is 2%
/// applicable from 940 - 1170 K 
/// This is a major factor limiting the temperature range of the 
/// correlations for FLiNaK as a whole
///
pub fn get_flinak_density(
    fluid_temp: ThermodynamicTemperature) -> Result<MassDensity,TuasLibError> {

    range_check_flinak_salt(fluid_temp)?;

    let fluid_temp_kelvin = fluid_temp.get::<kelvin>();
    let a = 2579.6;
    let b = -0.624;
    // generic correlation is:
    // a + bT + cT^2 + dT^3 + eT^4;

    let density_value_kg_per_m3 = 
        a 
        + b * fluid_temp_kelvin;


    return Ok(MassDensity::new::<
              kilogram_per_cubic_meter>(density_value_kg_per_m3));
}

/// Romatoski, R. R., & Hu, L. W. (2017). Fluoride salt coolant properties 
/// for nuclear reactor applications: A review. Annals 
/// of Nuclear Energy, 109, 635-647.
///
/// using recommendation by Romatoski to use Cohen correlation
/// as he had experimental data points
///
/// mu = 0.04 exp(4170/T[K])
pub fn get_flinak_dynamic_viscosity(
    fluid_temp: ThermodynamicTemperature) -> Result<DynamicViscosity,
TuasLibError>{
    range_check_flinak_salt(fluid_temp)?;


    let fluid_temp_kelvin = fluid_temp.get::<kelvin>();
    // generic form:  
    // mu = a * exp (b/T[K])

    let a = 0.04;
    let b = 4170_f64;
    let viscosity_value_centipoise = a * (b/fluid_temp_kelvin).exp();

    Ok(DynamicViscosity::new::<centipoise>(viscosity_value_centipoise))
}

/// Romatoski, R. R., & Hu, L. W. (2017). Fluoride salt coolant properties 
/// for nuclear reactor applications: A review. Annals 
/// of Nuclear Energy, 109, 635-647.
///
/// we are using Romatoski's recommended value of 1884 J/(kg K)
/// uncertainty (error bars) are 10%
pub fn get_flinak_constant_pressure_specific_heat_capacity(
    fluid_temp: ThermodynamicTemperature) -> Result<SpecificHeatCapacity,
TuasLibError>{
    range_check_flinak_salt(fluid_temp)?;

    let cp_value_joule_per_kg = 1884.0;

    Ok(SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(
        cp_value_joule_per_kg))
}

/// Romatoski, R. R., & Hu, L. W. (2017). Fluoride salt coolant properties 
/// for nuclear reactor applications: A review. Annals 
/// of Nuclear Energy, 109, 635-647.
///
/// we are using Smirnov correlation as recommended by Romatoski
pub fn get_flinak_thermal_conductivity(
    fluid_temp: ThermodynamicTemperature) -> Result<ThermalConductivity,TuasLibError> {

    // first check if correlation is in range of validity
    range_check_flinak_salt(fluid_temp)?;

    // get fluid temp in kelvin
    let fluid_temp_kelvin = fluid_temp.get::<kelvin>();

    // apply correlation by smirnov
    //
    // I was unsure as to whether the correlation units were in kelvin or 
    // degrees c, hence I coded a test for it
    let thermal_conductivity_value_watt_per_meter_kelvin 
        = 0.36 + 0.00056 * fluid_temp_kelvin;


    return Ok(ThermalConductivity::new::<watt_per_meter_kelvin>(
        thermal_conductivity_value_watt_per_meter_kelvin));

}

#[test]
pub fn test_flinak_thermal_conductivity_correlation_unit_in_kelvin(){
    // at 973K, we shld expect 0.90 W/(m K)
    // this test checks if the value is properly obtained in terms of units

    let thermal_cond_value_973_k_watt_per_meter_kelvin: f64;

    let temperature_973_kelvin = 
        ThermodynamicTemperature::new::<kelvin>(973.0);

    let thermal_cond_973_kelvin = 
        get_flinak_thermal_conductivity(temperature_973_kelvin).unwrap();

    thermal_cond_value_973_k_watt_per_meter_kelvin = 
        thermal_cond_973_kelvin.get::<watt_per_meter_kelvin>();

    approx::assert_relative_eq!(
        0.90, 
        thermal_cond_value_973_k_watt_per_meter_kelvin, 
        max_relative=0.02);
}


/// returns flinak specific enthalpy 
///
/// based on reference temperature at the minimum correlation temperature 
/// of flinak (h = 0 J/kg at that point)
///
///
pub fn get_flinak_specific_enthalpy(
    fluid_temp: ThermodynamicTemperature) -> 
Result<AvailableEnergy,TuasLibError>{
    range_check_flinak_salt(fluid_temp)?;

    // find cp at this temperature first
    //
    // delta h = cp (delta T)
    let cp = get_flinak_constant_pressure_specific_heat_capacity(fluid_temp)?;

    // we'll have a reference temperature:
    let reference_temperature_kelvin = min_temp_flinak().get::<kelvin>();

    // calculate delta T 
    let delta_t_from_ref_temperature: TemperatureInterval = 
        TemperatureInterval::new::<uom::si::temperature_interval::kelvin>
        (
            fluid_temp.get::<kelvin>()
            -reference_temperature_kelvin
        );

    let delta_h: AvailableEnergy = 
        cp * delta_t_from_ref_temperature;

    return Ok(delta_h);

}

/// returns flinak temperature from specific enthalpy 
///
/// the specific enthalpy is 
/// based on reference temperature at the minimum correlation temperature 
/// of flinak (h = 0 J/kg at that point)
///
///
pub fn get_temperature_from_enthalpy(
    fluid_enthalpy: AvailableEnergy) -> Result<ThermodynamicTemperature,TuasLibError> {

    // if enthalpy value below zero,
    // based on me setting zero enthalpy at the lower end of the 
    // temperature validity range for enthalpy,
    // then enthalpy is technically out of range
    if fluid_enthalpy.value < 0_f64 {
        panic!("FLiNaK : get_temperature_from_enthalpy \n
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
        let rhs = get_flinak_specific_enthalpy(fluid_temperature).unwrap();
        let rhs_value = rhs.get::<joule_per_kilogram>();

        return lhs_value-rhs_value;
    };
    
    // now solve using bisection
    // the range is from 940.0 K - 1073.0 K
    
    use anyhow::Result;
    let fluid_temperature_degrees_kelvin_result 
        = bisection!(enthalpy_root,
                    (940.0,1073.0),
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
/// Romatoski, R. R., & Hu, L. W. (2017). Fluoride salt coolant properties 
/// for nuclear reactor applications: A review. Annals 
/// of Nuclear Energy, 109, 635-647.
///
/// For FLiNaK, the absolute lower bound is 462C, which is a melting point 
/// estimate 
///
/// The density correlation is in range 940 - 1170 K 
/// about 666.85 C to 896.85 C
///
/// cp is across all temperature range 1884 J/(kg K)
///
/// the thermal conductivity is from about 773 to 1073 K
/// 
///
/// viscosity is over from 773-1173 K
///
/// From these, it seems that density and thermal conductivity correlations 
/// limit the range of applicability
///
/// I'm not going to make effort to increase this range for the time being,
/// can be patched in future
///
/// most conservative range is density (940 - 1073 K)
/// 666.85- 800C
///
/// 
pub fn range_check_flinak_salt(fluid_temp: ThermodynamicTemperature) 
    -> Result<bool,TuasLibError>{

        // first i convert the fluidTemp object into a degree 
        // celsius

        range_check(&Material::Liquid(LiquidMaterial::FLiNaK), 
            fluid_temp, 
            max_temp_flinak(), 
            min_temp_flinak()
            )?;

        return Ok(true);

    }


#[inline]
/// flinak max temp 
pub fn max_temp_flinak() -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(1073.0)

}
#[inline]
/// flinak min temp 
pub fn min_temp_flinak() -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<kelvin>(940.0)
}
