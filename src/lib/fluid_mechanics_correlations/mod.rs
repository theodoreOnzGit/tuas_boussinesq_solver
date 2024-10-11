//! License
//!    This file is part of a thermal hydraulics library written 
//!    in rust meant to help with the
//!    fluid mechanics and heat transfer aspects of the calculations
//!    for the Compact Integral Effects Tests (CIET) and hopefully 
//!    Gen IV Reactors such as the Fluoride Salt cooled High Temperature 
//!    Reactor (FHR)
//!     
//!    Copyright (C) 2022-2023  Theodore Kay Chen Ong, Singapore Nuclear
//!    Research and Safety Initiative, Per F. Peterson, University of 
//!    California, Berkeley Thermal Hydraulics Laboratory
//!
//!    tuas_boussinesq_solver is free software; you can 
//!    redistribute it and/or modify it
//!    under the terms of the GNU General Public License as published by the
//!    Free Software Foundation; either version 2 of the License, or (at your
//!    option) any later version.
//!
//!    tuas_boussinesq_solver is distributed in the hope 
//!    that it will be useful, but WITHOUT
//!    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
//!    FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
//!    for more details.
//!
//!    This thermal hydraulics library 
//!    contains some code copied from GeN-Foam, and OpenFOAM derivative.
//!    This offering is not approved or endorsed by the OpenFOAM Foundation nor
//!    OpenCFD Limited, producer and distributor of the OpenFOAM(R)software via
//!    www.openfoam.com, and owner of the OPENFOAM(R) and OpenCFD(R) trademarks.
//!    Nor is it endorsed by the authors and owners of GeN-Foam.
//!
//!    You should have received a copy of the GNU General Public License
//!    along with this program.  If not, see <http://www.gnu.org/licenses/>.
//!
//! © All rights reserved. Theodore Kay Chen Ong,
//! Singapore Nuclear Research and Safety Initiative,
//! Per F. Peterson,
//! University of California, Berkeley Thermal Hydraulics Laboratory
//!
//! Main author of the code: Theodore Kay Chen Ong, supervised by
//! Professor Per F. Peterson
//!
//! Btw, I have no affiliation with the Rust foundation.

/// calculate darcy, fanning friction factor
/// using churchill correlation
pub mod churchill_friction_factor;


/// contains functions and/or structs
/// which help you calcualte a custom fLDK factor
///
/// ie 
///
/// (f L/D +K ) 
///
/// f is the darcy firction factor
/// L/D is length to diameter ratio
/// K is the form loss
pub mod custom_fldk;


/// contains functions and/or structs
/// which help you dimensionalise and nondimensionalise variables
/// eg Reynold's number
pub mod dimensionalisation;



///// Contains structs or classes which
///// help you calculate pressure loss from mass 
///// flowrate and vice versa for pipes and custom components
//pub mod fluid_component_calculation;
//
//
///// Contains structs or classes which
///// help you calculate pressure loss from mass 
///// flowrate and vice versa for therminol VP 1 or
///// dowtherm A components
//pub mod therminol_component;
//
//
///// Contains traits which allow you to calculate 
///// mass flowrate, pressure drop and pressure change
///// for fluid components in series or parallel
//pub mod fluid_component_collection;


/// Courant Number Modules 
pub mod courant_number;




use crate::tuas_lib_error::TuasLibError;
/// This function calculates darcy friction factor
/// It takes in a Reynold's number and roughness ratio
///
/// and gives the darcy friction factor for laminar 
/// turbulent, and transition regimes. 
///
/// However, Re = 0 will not work!
/// ```rust
/// let darcy_friction_factor = 
///     tuas_boussinesq_solver::
///     fluid_mechanics_correlations::darcy(1800.0,0.0015).unwrap();
///
/// println!("{}", darcy_friction_factor);
/// ```
pub fn darcy(reynolds_number: f64, roughness_ratio: f64) -> Result<f64,
TuasLibError>{
    return churchill_friction_factor:: 
        darcy(reynolds_number, roughness_ratio);
}

/// This function calculates moody friction factor
/// It takes in a Reynold's number and roughness ratio
///
/// and gives the darcy friction factor for laminar 
/// turbulent, and transition regimes. 
///
/// It's basically the same as darcy friction factor
///
/// However, Re = 0 will not work!
/// ```rust
/// let moody_friction_factor = 
///     tuas_boussinesq_solver::
///     fluid_mechanics_correlations::moody(1800.0,0.0015).unwrap();
///
/// println!("{}", moody_friction_factor);
/// ```
pub fn moody(reynolds_number: f64, roughness_ratio: f64) -> Result<f64,
TuasLibError>{
    return churchill_friction_factor:: 
        moody(reynolds_number, roughness_ratio);
}

/// This function calculates the fldk
///
/// this is the
///
/// Be = 0.5 * Re^2 * (f * (L/D) + K)
///
/// the f is darcy friction factor
///
/// and the term in the brackets is fldk
///
/// you are to give a K value, L/D value, Re
/// and roughness ratio
///
/// However, Re = 0 will not work!
/// ```rust
///    let fldk = 
///        tuas_boussinesq_solver:: 
///        fluid_mechanics_correlations::fldk(
///            15000.0,0.00014,10.0,5.0).unwrap();
///
///    println!("{}", fldk);
/// ```
pub fn fldk(reynolds_number: f64,
                   roughness_ratio: f64,
                   length_to_diameter_ratio: f64,
                   k: f64) -> Result<f64, TuasLibError>{
    return churchill_friction_factor::
        f_ldk(reynolds_number,
             roughness_ratio,
             length_to_diameter_ratio,
             k);
}


/// This function calculates the bejan number
///
/// this is the
///
///
/// Be = (P * D^2)/(mu * nu)
/// 
/// P is pressure loss
/// D is hydraulic diameter
/// mu is dynamic viscosity
/// nu is kinematic viscosity
///
/// Be is the bejan number which is dimensionless
///
/// It is calculated using:
/// Be = 0.5 * Re^2 * (f * (L/D) + K)
///
/// the f is darcy friction factor
///
/// and the term in the brackets is fldk
///
/// you are to give a K value, L/D value, Re
/// and roughness ratio
///
/// Re = 0  and Re < 0 is supported,
/// this assumes that the component is symmetrical
/// in terms of pressure loss, which may usually
/// be the case for pipes anyhow
///
/// 
///
/// ```rust
/// let bejan_d = 
///     tuas_boussinesq_solver::
///     fluid_mechanics_correlations::get_bejan_d(
///         0.00000000000001,0.00014,10.0,5.0).unwrap();
///
/// println!("{}", bejan_d);
///
/// let bejan_d = 
///     tuas_boussinesq_solver::
///     fluid_mechanics_correlations::get_bejan_d(
///         -5000.0,0.00014,10.0,5.0).unwrap();
///
/// println!("{}", bejan_d);
///
/// let bejan_d = 
///     tuas_boussinesq_solver::
///     fluid_mechanics_correlations::get_bejan_d(
///         0.0,0.00014,10.0,5.0).unwrap();
///
/// println!("{}", bejan_d);
/// ```
pub fn get_bejan_d(reynolds_number: f64,
                   roughness_ratio: f64,
                   length_to_diameter_ratio: f64,
                   k: f64) -> Result<f64, TuasLibError> {
    return churchill_friction_factor::
        get_bejan_number_d(reynolds_number, roughness_ratio,
              length_to_diameter_ratio, k);
}


/// This function calculates the Reynolds number given
/// a Bejan number.
///
/// Remember Bejan number is dimensionless pressure 
/// drop
///
/// Be = (P * D^2)/(mu * nu)
/// 
/// P is pressure loss
/// D is hydraulic diameter
/// mu is dynamic viscosity
/// nu is kinematic viscosity
///
/// We implicitly solve for Re using:
/// Be = 0.5 * Re^2 * (f * (L/D) + K)
///
/// the f is darcy friction factor
///
/// and the term in the brackets is fldk
///
/// you are to give a K value, L/D value, Be
/// and roughness ratio
///
/// Re = 0  and Re < 0 is supported,
/// this assumes that the component is symmetrical
/// in terms of pressure loss, which may usually
/// be the case for pipes anyhow
///
/// 
/// In the following example, we get a bejan number calculated
/// first with Re = 5000.0
/// and then using that bejan number, we try and find the Re again
/// which should be about 5000.0
///
/// we use the approx package and ensure that the numbers are similar
/// to within 0.001 or 0.1% of each other
///
/// ```rust
///
/// extern crate approx;
/// let bejan_d = 
/// tuas_boussinesq_solver::fluid_mechanics_correlations::
///     get_bejan_d(
///         5000.0,0.00014,10.0,5.0).unwrap();
///
/// println!("{}", bejan_d);
///
/// let reynolds_number = 
/// tuas_boussinesq_solver::fluid_mechanics_correlations::
///     get_reynolds_number(
///         bejan_d,0.00014,10.0,5.0).unwrap();
///
/// approx::assert_relative_eq!(reynolds_number, 5000.0,
/// max_relative = 0.001);
/// ```
///
///
/// Note: why can't we just find Reynold's number from friction factor?
///
/// Note that in the laminar and turbulent region, a single Reynold's
/// number can have two different friction factor values.
/// Even in the transition region, there's probably a range of friction
/// factors where Re can have a third or fourth value
/// That's not good
///
/// Hence Reynold's number is not a function of friction factor unless
/// you restrict Re to a certain range
///
/// To get around this, we assume that pressure losses are a function
/// of Re and vice versa, 
///
/// meaning to say each pressure loss value maps to a single Re
/// and therefore dimensionless pressure losses (Be) should also
/// map to a single Re.
///
/// Therefore, we must supply a Bejan number to get an Re value.
///
pub fn get_reynolds_number(bejan_d: f64,
             roughness_ratio: f64,
             length_to_diameter: f64,
             form_loss_k: f64) -> Result<f64,TuasLibError> {
    return churchill_friction_factor::
        get_reynolds_from_bejan(bejan_d, roughness_ratio,
              length_to_diameter, form_loss_k);

}



/// pipe calculations 
///
/// these are pre-built functions which make calculating mass flowrate 
/// and pressure drop across pipes easier
pub mod pipe_calculations;
