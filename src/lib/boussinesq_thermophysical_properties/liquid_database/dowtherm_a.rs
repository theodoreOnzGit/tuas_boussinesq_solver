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
//    Copyright (C) 2022-2023  Theodore Kay Chen Ong, Singapore Nuclear
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
use uom::si::thermodynamic_temperature::degree_celsius;
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

/// function to obtain dowtherm A density
/// given a temperature

pub fn get_dowtherm_a_density(
    fluid_temp: ThermodynamicTemperature) -> Result<MassDensity,TuasLibError> {

    // first we check if fluid temp is between 20-180C (range of validity)
    // panic otherwise
    range_check_dowtherm_a(fluid_temp)?;

    //then convert the fluidTemp object into a f64
    // and plug it into the correlation
    let density_value_kg_per_m3 = 1078.0 - 0.85*fluid_temp
       .get::<degree_celsius>();

    return Ok(MassDensity::new::<
              kilogram_per_cubic_meter>(density_value_kg_per_m3));
}

/// function to obtain dowtherm A viscosity
/// given a temperature
pub fn get_dowtherm_a_viscosity(
    fluid_temp: ThermodynamicTemperature) -> Result<DynamicViscosity,
TuasLibError>{

    range_check_dowtherm_a(fluid_temp)?;
    let temperature_degrees_c_value = fluid_temp.get::<degree_celsius>();
    let viscosity_value_pascal_second = 0.130/
        temperature_degrees_c_value.powf(1.072);

    Ok(DynamicViscosity::new::<pascal_second>(viscosity_value_pascal_second))
                                
}

/// function to obtain dowtherm A specific heat capacity
/// given a temperature
pub fn get_dowtherm_a_constant_pressure_specific_heat_capacity(
    fluid_temp: ThermodynamicTemperature) -> Result<SpecificHeatCapacity,
TuasLibError>{

    range_check_dowtherm_a(fluid_temp)?;
    // note, specific entropy and heat capcity are the same unit...
    //
    let cp_value_joule_per_kg = 1518.0 + 2.82*fluid_temp.get::<degree_celsius>();

    Ok(SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(
        cp_value_joule_per_kg))
}

/// function to obtain dowtherm A thermal conductivity
/// given a temperature
pub fn get_dowtherm_a_thermal_conductivity(
    fluid_temp: ThermodynamicTemperature) -> Result<ThermalConductivity,TuasLibError> {


    range_check_dowtherm_a(fluid_temp)?;
    let thermal_conductivity_value = 0.142 - 0.00016* fluid_temp
        .get::<degree_celsius>();

    return Ok(ThermalConductivity::new::<watt_per_meter_kelvin>(
        thermal_conductivity_value));
}

/// function to obtain dowtherm A enthalpy
/// given a temperature
///
/// 
/// This is done via analytically integrating 
/// the function for specific heat capacity of 
/// dowtherm A
///
/// However,
/// the thing is that with enthalpy
/// we need a reference value
/// i take the reference value to be 0 J/kg enthalpy at 20C
/// integrating heat capacity with respect to T, we get
///
/// cp = 1518 + 2.82*T
///
/// H = 1518*T + 2.82/2.0*T^2 + C
/// at T = 20C, 
/// H = 30924 + C
/// H = 0
/// C = -30924 (i used libre office to calculate this)
///
/// Example use:
/// ```rust
///
/// use uom::si::f64::*;
/// use uom::si::thermodynamic_temperature::kelvin;
/// use tuas_boussinesq_solver::boussinesq_thermophysical_properties::
/// liquid_database::dowtherm_a::get_dowtherm_a_enthalpy;
/// 
///
/// let temp1 = ThermodynamicTemperature::new::<kelvin>(303_f64);
///
/// let specific_enthalpy_1 = 
/// get_dowtherm_a_enthalpy(temp1);
///
///
/// let expected_enthalpy: f64 = 
/// 1518_f64*30_f64 + 2.82/2.0*30_f64.powf(2_f64) - 30924_f64;
///
/// // the expected value is about 15885 J/kg
///
/// extern crate approx;
/// approx::assert_relative_eq!(expected_enthalpy, specific_enthalpy_1.unwrap().value, 
/// max_relative=0.02);
/// ```
pub fn get_dowtherm_a_enthalpy(
    fluid_temp: ThermodynamicTemperature) -> 
Result<AvailableEnergy,TuasLibError>{

    range_check_dowtherm_a(fluid_temp)?;
    // note, specific entropy and heat capcity are the same unit...
    //
    // H = 1518*T + 2.82/2.0*T^2 - 30924
    let temp_c_value = fluid_temp.get::<degree_celsius>();
    let enthalpy_value_joule_per_kg 
        = 1518.0 * temp_c_value 
        + 2.82/2.0 * temp_c_value.powf(2.0) -
        30924.0;

    // the closest unit available is AvailableEnergy which is
    // joule per kg 

    return Ok(AvailableEnergy::new::<joule_per_kilogram>(
        enthalpy_value_joule_per_kg));
}

/// function to obtain dowtherm A temperature 
/// given a enthalpy
///
/// 
/// This is done via analytically integrating 
/// the function for specific heat capacity of 
/// dowtherm A
///
/// However,
/// the thing is that with enthalpy
/// we need a reference value
/// i take the reference value to be 0 J/kg enthalpy at 20C
/// integrating heat capacity with respect to T, we get
///
/// cp = 1518 + 2.82*T
///
/// H = 1518*T + 2.82/2.0*T^2 + C
/// at T = 20C, 
/// H = 30924 + C
/// H = 0
/// C = -30924 (i used libre office to calculate this)
///
/// Once i have this correlation, i will use
/// an iterative root finding method to find the temperature
///
/// As of Oct 2022, it is bisection
///
/// Example: 
///
/// ```rust
/// use uom::si::f64::*;
/// use uom::si::thermodynamic_temperature::kelvin;
/// use uom::si::available_energy::joule_per_kilogram;
/// use tuas_boussinesq_solver::boussinesq_thermophysical_properties::
/// liquid_database::dowtherm_a::get_temperature_from_enthalpy;
///
///
/// let specific_enthalpy_1 = AvailableEnergy::new::
/// <joule_per_kilogram>(15885.0);
///
/// let temp_expected = ThermodynamicTemperature::new::
/// <kelvin>(303_f64);
/// 
/// let temp_acutal = get_temperature_from_enthalpy(
/// specific_enthalpy_1).unwrap();
///
///
/// extern crate approx;
/// approx::assert_relative_eq!(temp_expected.value, 
/// temp_acutal.value, 
/// max_relative=0.01);
///
///
/// ```
pub fn get_temperature_from_enthalpy(
    fluid_enthalpy: AvailableEnergy) -> Result<ThermodynamicTemperature,TuasLibError> {

    if fluid_enthalpy.value < 0_f64 {
        panic!("dowtherm A : get_temperature_from_enthalpy \n
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

    let enthalpy_root = |temp_degrees_c_value : f64| -> f64 {
        let lhs_value = enthalpy_value_joule_per_kg;
        // convert AD type into double
        let temp_degrees_c_value_double = temp_degrees_c_value;

        let fluid_temperature = 
            ThermodynamicTemperature::new::<degree_celsius>(
                temp_degrees_c_value_double);
        let rhs = get_dowtherm_a_enthalpy(fluid_temperature).unwrap();
        let rhs_value = rhs.get::<joule_per_kilogram>();

        return lhs_value-rhs_value;
    };
    
    // now solve using bisection
    use anyhow::Result;
    
    let fluid_temperature_degrees_cresult 
        = bisection!(enthalpy_root,
                    (20.0,180.0),
                    100,
                    1e-8);

    let fluid_temperature_degrees_c = fluid_temperature_degrees_cresult.unwrap();

    return Ok(ThermodynamicTemperature::
        new::<degree_celsius>(fluid_temperature_degrees_c));

}


/// function checks if a fluid temperature falls in a range (20-180C)
///
/// If it falls outside this range, it will panic
/// or throw an error, and the program will not run
///
/// TODO: find a dowtherm a correlation with larger temperature range
/// of validity
pub fn range_check_dowtherm_a(fluid_temp: ThermodynamicTemperature) 
    -> Result<bool,TuasLibError>{

        // first i convert the fluidTemp object into a degree 
        // celsius

        range_check(&Material::Liquid(LiquidMaterial::DowthermA), 
            fluid_temp, 
            ThermodynamicTemperature::new::<degree_celsius>(180.0), 
            ThermodynamicTemperature::new::<degree_celsius>(19.8))?;

        return Ok(true);

    }

/// dowtherm a max temp 
pub fn max_temp_dowtherm_a() -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<degree_celsius>(180.0)

}
/// dowtherm a min temp 
pub fn min_temp_dowtherm_a() -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<degree_celsius>(20.0)
}


#[test]
pub fn dynamic_viscosity_test_dowtherm_a(){

    use uom::si::pressure::atmosphere;
    use uom::si::thermodynamic_temperature::kelvin;
    use crate::prelude::beta_testing::try_get_mu_viscosity;
    let dowtherm_a = Material::Liquid(LiquidMaterial::DowthermA);
    let temperature = ThermodynamicTemperature::new::<kelvin>(350.0);
    let pressure = Pressure::new::<atmosphere>(1.0);

    let dynamic_viscosity = try_get_mu_viscosity(dowtherm_a, temperature, pressure);

    approx::assert_relative_eq!(
        0.001237,
        dynamic_viscosity.unwrap().value,
        max_relative=0.01);
}
