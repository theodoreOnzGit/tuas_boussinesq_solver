#![warn(missing_docs)]
extern crate peroxide;
use peroxide::prelude::*;

use crate::thermal_hydraulics_error::ThermalHydraulicsLibError;

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
// Btw, I have no affiliation with the Rust Foundation.

// here are the functions used for friction factor, rather messy but
// for fast prototyping and sandboxing don't really care too much
//

fn churchill_friction_captial_b(reynolds_number: f64) -> f64 {
    let numerator = 37530.0_f64.powf(16.0);
    let denominator = reynolds_number.powf(16.0);
    return numerator/denominator;
}

fn churchill_friction_captial_a(reynolds_number: f64, roughness_ratio: f64) -> f64 {
    let seven_over_reynolds_number = 7.0/reynolds_number;
    let reynolds_term = seven_over_reynolds_number.powf(0.9);

    let roughness_term = 0.27 * roughness_ratio;

    let log_fraction = 1.0/(reynolds_term + roughness_term);
    // we will need natural logarithms:
    let inner_bracket_term = 2.457*log_fraction.ln();

    let capital_a = inner_bracket_term.powf(16.0);

    return capital_a;


}

fn churchill_inner_term(reynolds: f64, roughness_ratio: f64) -> f64 {

    let eight_over_reynolds_number = 8.0/reynolds;
    let laminar_term = eight_over_reynolds_number.powf(12.0);

    let captial_a_term = churchill_friction_captial_a(reynolds,roughness_ratio);
    let captial_b_term = churchill_friction_captial_b(reynolds);

    let captial_a_plus_captial_b_all_inverse = 1.0/(captial_a_term+captial_b_term);
    let turbulent_term = captial_a_plus_captial_b_all_inverse.powf(3.0/2.0);

    return laminar_term + turbulent_term;
}

// this particular implementation uses the churchill correlation
#[inline]
fn fanning(reynolds_number: f64, roughness_ratio: f64) -> Result<f64,ThermalHydraulicsLibError>{

    if reynolds_number == 0.0 {
        panic!("Re = 0.0");
    }

    if reynolds_number < 0.0 {
        panic!("Re<0.0");
    }

    if roughness_ratio < 0.0 {
        panic!("roughness_ratio<0.0");
    }

    let inner_term = churchill_inner_term(reynolds_number, roughness_ratio);
    let power_term = inner_term.powf(1.0/12.0);
    let fanning_friction_factor = 2.0 * power_term;
    return Ok(fanning_friction_factor);
}

#[inline]
/// calculates darcy friction factor using churchill correlation
pub fn darcy(reynolds_number: f64, roughness_ratio: f64) -> 
Result<f64,ThermalHydraulicsLibError> {
    return Ok(4.0*fanning(reynolds_number, roughness_ratio)?);
}

/// calculates moody friction factor using churchill correlation
/// basically same as darcy
pub fn moody(reynolds_number: f64, roughness_ratio: f64) -> 
Result<f64,ThermalHydraulicsLibError> {
    return Ok(4.0*fanning(reynolds_number, roughness_ratio)?);
}



/// calculates fLDK using churchill correlation
/// and a user defined form loss K value
pub fn f_ldk(reynolds_number: f64,
    roughness_ratio: f64,
    length_to_diameter_ratio: f64,
    k: f64) -> Result<f64,ThermalHydraulicsLibError>{
    if reynolds_number == 0.0 {
        panic!("Re = 0");
    }

    if reynolds_number < 0.0 {
        panic!("Re < 0");
    }

    if roughness_ratio < 0.0 {
        panic!("roughnessRatio<0.0");
    }

    if length_to_diameter_ratio <= 0.0 {
        panic!("lengthToDiameterRatio<=0.0");
    }

    if k < 0.0 {
        panic!("For m loss coefficient K < 0.0");
    }

    let f = darcy(reynolds_number, roughness_ratio)?;
    let f_ldk = f*length_to_diameter_ratio + k;

    return Ok(f_ldk);
}


/// calculates a nondimensional pressure loss (Be_D)
/// from the nondimensionalised flowrate (Re_D)
pub fn get_bejan_number_d(
    mut reynolds_number: f64,
    roughness_ratio: f64,
    length_to_diameter_ratio: f64,
    form_loss_k: f64) -> Result<f64,ThermalHydraulicsLibError>{

    if reynolds_number == 0.0 {
        return Ok(0.0);
    }

    let mut is_negative = false;

    if reynolds_number < 0.0 {
        is_negative = true;
        reynolds_number = reynolds_number * -1.0;
    }

    if roughness_ratio < 0.0 {
        panic!("roughnessRatio<0.0");
    }

    if length_to_diameter_ratio <= 0.0 {
        panic!("lengthToDiameterRatio<=0.0");
    }

    if form_loss_k < 0.0 {
        panic!("Form loss coefficient K < 0.0");
    }

    let f = darcy(reynolds_number, roughness_ratio)?;

    let f_ldk = f*length_to_diameter_ratio + form_loss_k;

    let mut bejan_number = 0.5*f_ldk*reynolds_number.powf(2.0);

    if is_negative {
        bejan_number = bejan_number * -1.0;
        return Ok(bejan_number);
    }

    return Ok(bejan_number);
}

/// calculates Re given a Be_D 
///
/// it is basically calculating nondimensionalised
/// flowrate from nondimensionalised pressure loss
pub fn get_reynolds_from_bejan(
    mut bejan_number_d: f64,
    roughness_ratio: f64,
    length_to_diameter: f64,
    form_loss_k: f64) -> Result<f64,ThermalHydraulicsLibError> {

    if length_to_diameter <= 0.0 {
        panic!("lengthToDiameterRatio<=0.0");
    }

    if roughness_ratio < 0.0 {
        panic!("roughnessRatio<0.0");
    }

    if form_loss_k < 0.0 {
        panic!("formLossK<0.0");
    }

    // this part deals with negative Be_L values
    // invalid Be_L values
    let mut is_negative = false;
    if bejan_number_d < 0.0 {
        bejan_number_d = bejan_number_d * -1.0;
        is_negative = true;
    }

    let max_reynolds = 1.0e12;
    let min_reynolds = 0.0;

    // i calculate the Be_D corresponding to 
    // Re = 1e12
    let max_bejan_d: f64 = get_bejan_number_d(max_reynolds,roughness_ratio, 
        length_to_diameter,form_loss_k)?;

    if bejan_number_d >= max_bejan_d {
        panic!("Be too large");
    }



    let bisect = BisectionMethod { max_iter: 100, tol: 1e-8 };
    let problem = ReynoldsFromBejanD {
        max_reynolds,
        min_reynolds,
        bejan_number_d,
        roughness_ratio,
        length_to_diameter,
        form_loss_k,
    };
    let reynolds_number_result = bisect.find(&problem).unwrap();



    // the unwrap turns the result into f64
    let mut reynolds_number = reynolds_number_result[0];


    if is_negative
    {
        reynolds_number = reynolds_number * -1.0;
        return Ok(reynolds_number);
    }

    return Ok(reynolds_number);
}

use anyhow::Result;

struct ReynoldsFromBejanD {
    pub max_reynolds: f64,
    pub min_reynolds: f64,
    pub bejan_number_d: f64,
    pub roughness_ratio: f64,
    pub length_to_diameter: f64,
    pub form_loss_k: f64,
}

impl ReynoldsFromBejanD {
    // i'm solving for
    // Be - 0.5*fLDK*Re^2 = 0 
    // the fLDK term can be calculated using
    // getBe
    //
    // now i don't really need the interpolation
    // term in here because when Re = 0,
    // Be = 0 in the getBe code.
    // so really, no need for fancy interpolation.
    //
    // Now in peroxide, the type taken in and out
    // is not a f64 double
    // but rather AD which stands for automatic 
    // differentiation
    // https://docs.rs/peroxide/latest/peroxide/structure/ad/index.html
    fn pressure_drop_root(&self, reynolds_number: Pt<1>) -> Result<Pt<1>> {
        let lhs_value: f64 = self.bejan_number_d;

        let reynolds_double: f64 = reynolds_number[0];
        let f_ldk_term = get_bejan_number_d(reynolds_double, 
            self.roughness_ratio,
            self.length_to_diameter,
            self.form_loss_k).unwrap();

        let rhs_value = f_ldk_term;

        return Ok([lhs_value - rhs_value]);
    }
}
impl RootFindingProblem<1, 1, (f64, f64)> for ReynoldsFromBejanD {
    fn function(&self, reynolds_number: Pt<1>) -> Result<Pt<1>> {
        self.pressure_drop_root(reynolds_number)
    }
    fn initial_guess(&self) -> (f64, f64) {
        let upper_bound = 
            self.max_reynolds;
        let lower_bound = 
            self.min_reynolds;
        (lower_bound, upper_bound)
    }
}
