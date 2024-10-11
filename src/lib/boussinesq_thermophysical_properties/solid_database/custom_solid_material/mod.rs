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
//
//
// For solids not in this database, we can use these functions 
// to code in correlations for your own solids
//
// I copied most of the code from the fluid side ad verbatim for solids 
// so I'm not testing out code here
//

use uom::si::f64::*;
use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};
use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
use uom::si::available_energy::joule_per_kilogram;

// this is for the root finding algorithms
extern crate peroxide;
use peroxide::fuga::*;

use crate::thermal_hydraulics_error::TuasLibError;

/// function to obtain custom solid density
/// given a temperature
/// and temperature bounds
pub fn get_custom_solid_density(
    solid_temp: ThermodynamicTemperature,
    density_function: fn(ThermodynamicTemperature) -> MassDensity,
    upper_bound_temperature: ThermodynamicTemperature,
    lower_bound_temperature: ThermodynamicTemperature,
    ) -> Result<MassDensity,TuasLibError> {

    // first we check if solid temp is between the specified 
    // upper and lower bound
    // panic otherwise
    range_check_custom_solid(solid_temp,
        upper_bound_temperature,lower_bound_temperature)?;

    return Ok(density_function(solid_temp));
}

/// function to obtain custom solid specific heat capacity
/// given a temperature
pub fn get_custom_solid_constant_pressure_specific_heat_capacity(
    solid_temp: ThermodynamicTemperature,
    cp_function: fn(ThermodynamicTemperature) -> SpecificHeatCapacity,
    upper_bound_temperature: ThermodynamicTemperature,
    lower_bound_temperature: ThermodynamicTemperature) -> Result<SpecificHeatCapacity,
TuasLibError>{

    // first we check if solid temp is between the specified 
    // upper and lower bound
    // panic otherwise
    range_check_custom_solid(solid_temp,
        upper_bound_temperature,
        lower_bound_temperature)?;
    return Ok(cp_function(solid_temp));
}


/// function to obtain custom solid thermal conductivity
/// given a temperature
pub fn get_custom_solid_thermal_conductivity(
    solid_temp: ThermodynamicTemperature,
    conductivity_function: fn(ThermodynamicTemperature) -> ThermalConductivity,
    upper_bound_temperature: ThermodynamicTemperature,
    lower_bound_temperature: ThermodynamicTemperature
    ) -> Result<ThermalConductivity,TuasLibError> {


    range_check_custom_solid(solid_temp,
        upper_bound_temperature,
        lower_bound_temperature)?;

    return Ok(conductivity_function(solid_temp));
}


/// function to obtain custom solid enthalpy
/// given a temperature
///
/// Now, there are two ways of doing this,
/// firstly, allow the user to specify the enthalpy correlation
/// doing so saves calculation speed 
///
/// the second way is to numerically integrate the cp value on behalf of 
/// the user. It is slower on the calculation times but faster with 
/// implementation
///
/// I suppose for the user end, I don't assume runtime
/// speed to be of the essence in this case in comparison to coding 
/// speed and ease of use.
///
/// Therefore I will just use numerical integrals, so that the user need not 
/// perform extra coding
pub fn get_custom_solid_enthalpy(
    solid_temp: ThermodynamicTemperature,
    cp_function: fn(ThermodynamicTemperature) -> SpecificHeatCapacity,
    upper_bound_temperature: ThermodynamicTemperature,
    lower_bound_temperature: ThermodynamicTemperature) -> 
Result<AvailableEnergy,TuasLibError>{

    // first we check if solid temp is between the specified 
    // upper and lower bound
    // panic otherwise
    range_check_custom_solid(solid_temp,
        upper_bound_temperature,
        lower_bound_temperature)?;


    // next we need to convert the cp function into something 
    // we can use numerical integration on
    // for now, i select Gauss-Kronrod quadrature as it is convenient 
    // with tolerance of 1e-9 
    // and 100 max iterations 

    let abs_tolerance = 1e-9;
    let max_iterations = 100;
    let integration_method = Integral::G20K41(abs_tolerance, max_iterations);

    // next, my cp function needs to be converted into a f64 type function
    // first let me get my solid temperature and lower bound temperature 
    // in kelvin 

    let solid_temp_kelvin: f64 = solid_temp.get::<kelvin>();
    let lower_bound_temp_kelvin: f64 = lower_bound_temperature.get::<kelvin>();

    // my cp function will take a solid temperature value in kelvin 
    // and return a cp value in joules/(kg Kelvin)

    let cp_fn_float = |temp_kelvin: f64| {
        // it takes in the temperature in kelvin, 

        let temperature = ThermodynamicTemperature::new::<kelvin>(
            temp_kelvin);
        let cp = cp_function(temperature);

        // then return cp in joule_per_kilogram_kelvin
        cp.get::<joule_per_kilogram_kelvin>()

    };

    // now we can numerically integrate, and we get specific enthalpy 
    // numerically in joule per kilogram
    //
    // note that I define zero enthalpy at the lower bound temperautre 
    // provided

    let specific_enthalpy_joule_per_kilogram 
        = integrate(
            // cp function float
            cp_fn_float, 
            // bounds from lower bound to upper bound
            (lower_bound_temp_kelvin,solid_temp_kelvin),
            // integration method described earlier
            integration_method,
            );
    

    let enthalpy = AvailableEnergy::new::<
        joule_per_kilogram>(specific_enthalpy_joule_per_kilogram);

    return Ok(enthalpy);

}


/// function to obtain custom solid temperature 
/// given a enthalpy 
/// 
/// note that this is quite intensive calculation load 
/// wise due to its iterative nature, use sparingly and with caution
/// 
pub fn get_custom_solid_temperature_from_enthalpy(
    solid_enthalpy: AvailableEnergy,
    cp_function: fn(ThermodynamicTemperature) -> SpecificHeatCapacity,
    upper_bound_temperature: ThermodynamicTemperature,
    lower_bound_temperature: ThermodynamicTemperature) -> Result<ThermodynamicTemperature,TuasLibError> {

    if solid_enthalpy.value < 0_f64 {
        panic!("user supplied solid: get_temperature_from_enthalpy \n
               enthalpy < 0.0 , out of correlation range");
    }

    // now solve using bisection
    
    let bisect = BisectionMethod { max_iter: 100, tol: 1e-8 };
    let problem = CustomSolidTemperatureFromEnthalpy {
        lower_bound_temperature,
        upper_bound_temperature,
        solid_enthalpy,
        cp_function,
    };
    let solid_temperature_degrees_cresult 
        = bisect.find(&problem).unwrap();

    let solid_temperature_degrees_c = solid_temperature_degrees_cresult[0];

    return Ok(ThermodynamicTemperature::
        new::<degree_celsius>(solid_temperature_degrees_c));

}
use anyhow::Result;
struct CustomSolidTemperatureFromEnthalpy {
    pub lower_bound_temperature: ThermodynamicTemperature,
    pub upper_bound_temperature: ThermodynamicTemperature,
    pub solid_enthalpy: AvailableEnergy,
    pub cp_function: fn(ThermodynamicTemperature) -> SpecificHeatCapacity,
}

impl CustomSolidTemperatureFromEnthalpy {
    // let's define a function for solving the enthalpy root
    // LHS is actual enthalpy value we compare against
    fn enthalpy_root(&self, temp_degrees_c: Pt<1>) -> Result<Pt<1>> {

        // first let's convert enthalpy to a double (f64)
        let enthalpy_value_joule_per_kg = 
            self.solid_enthalpy.get::<joule_per_kilogram>();
        let lhs_value = enthalpy_value_joule_per_kg;
        let temp_degrees_c_value_double: f64 = temp_degrees_c[0];

        let fluid_temperature = 
            ThermodynamicTemperature::new::<degree_celsius>(
                temp_degrees_c_value_double);
        let rhs = get_custom_solid_enthalpy(fluid_temperature,
            self.cp_function,
            self.upper_bound_temperature,
            self.lower_bound_temperature).unwrap();
        let rhs_value = rhs.get::<joule_per_kilogram>();

        return Ok([lhs_value-rhs_value]);
    }
}

impl RootFindingProblem<1, 1, (f64, f64)> for CustomSolidTemperatureFromEnthalpy {
    fn function(&self, temp_degrees_c: Pt<1>) -> Result<Pt<1>> {
        self.enthalpy_root(temp_degrees_c)
    }
    fn initial_guess(&self) -> (f64, f64) {
        // i use degrees c for conversion here because of legacy reasons.
        // as long as the conversions are consistent, it doesn't matter whether 
        // I use degrees c or degrees kelvin
        let upper_bound_temp_degc = 
            self.upper_bound_temperature.get::<degree_celsius>();
        let lower_bound_temp_degc = 
            self.lower_bound_temperature.get::<degree_celsius>();
        (lower_bound_temp_degc, upper_bound_temp_degc)
    }
}

/// function checks if a solid temperature falls in a range (20-180C)
///
/// If it falls outside this range, it will panic
/// or throw an error, and the program will not run
///
pub fn range_check_custom_solid(solid_temp: ThermodynamicTemperature,
    upper_bound_temperature: ThermodynamicTemperature,
    lower_bound_temperature: ThermodynamicTemperature,
) 
    -> Result<bool,TuasLibError>{

        // first i convert the solidTemp object into a degree 
        // celsius
        let temp_value_celsius = 
            solid_temp.get::<degree_celsius>();
        let low_temp_value_celsius = 
            lower_bound_temperature.get::<degree_celsius>();
        let high_temp_value_celsius = 
            upper_bound_temperature.get::<degree_celsius>();

        if temp_value_celsius < low_temp_value_celsius {
            let error_msg = "Your custom material temperature \n";
            let error_msg1 = "is too low :";
            let error_msg3 = "C \n";
            let error_msg4 = "\n the minimum is ".to_owned() + &low_temp_value_celsius.to_string() + "C";


            println!("{}{}{:?}{}{}",
                error_msg,
                error_msg1,
                temp_value_celsius,
                error_msg3,
                error_msg4);
            return Err(TuasLibError::ThermophysicalPropertyTemperatureRangeError);
        }


        if temp_value_celsius > high_temp_value_celsius {
            let error_msg = "Your custom material temperature \n";
            let error_msg1 = "is too high :";
            let error_msg3 = "C \n";
            let error_msg4 = "\n the max is".to_owned()+ &high_temp_value_celsius.to_string()+"C";

            println!("{}{}{:?}{}{}",
                error_msg,
                error_msg1,
                temp_value_celsius,
                error_msg3,
                error_msg4);
            return Err(TuasLibError::ThermophysicalPropertyTemperatureRangeError);
        }

        return Ok(true);

    }


