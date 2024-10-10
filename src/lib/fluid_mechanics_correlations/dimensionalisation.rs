#![warn(missing_docs)]
extern crate peroxide;
extern crate uom;

use uom::si::f64::*;
use uom::typenum::P2;


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



/// calculates reynolds number given a fluid average velocity
pub fn calc_reynolds_from_velocity(fluid_density: MassDensity,
    velocity: Velocity, 
    hydraulic_diameter: Length,
    fluid_viscosity: DynamicViscosity) -> Ratio {

    if fluid_viscosity.value <= 0.0 {
        panic!("fluid Viscosity <= 0.0, nonphysical");
    }

    if hydraulic_diameter.value <= 0.0 {
        panic!("hydraulic Diameter <= 0.0, nonphysical");
    }
    if fluid_density.value <= 0.0 {
        panic!("fluidDensity <= 0.0, nonphysical");
    }

    let reynolds_number = 
        fluid_density * 
        velocity * 
        hydraulic_diameter / 
        fluid_viscosity;

    reynolds_number

}


/// calculates Re = mass_flow/area * D_H/mu
pub fn calc_reynolds_from_mass_rate(fluid_mass_flowrate: MassRate,
    cross_sectional_area: Area,
    hydraulic_diameter: Length,
    fluid_viscosity: DynamicViscosity) -> Ratio {

    if fluid_viscosity.value <= 0.0 {
        panic!("fluid Viscosity <= 0.0, nonphysical");
    }

    if hydraulic_diameter.value <= 0.0 {
        panic!("hydraulic Diameter <= 0.0, nonphysical");
    }
    if cross_sectional_area.value <= 0.0 {
        panic!("pipe Area <= 0.0, nonphysical");
    }

    let reynolds_number = fluid_mass_flowrate/
        cross_sectional_area*
        hydraulic_diameter/
        fluid_viscosity;


    reynolds_number
}


/// converts Re to mass flowrate using
/// Re = mass_flow/area * D_H/mu
pub fn calc_reynolds_to_mass_rate(cross_sectional_area: Area,
    reynolds_number: Ratio,
    hydraulic_diameter: Length,
    fluid_viscosity: DynamicViscosity) -> MassRate {

    if fluid_viscosity.value <= 0.0 {
        panic!("fluid Viscosity <= 0.0, nonphysical");
    }

    if hydraulic_diameter.value <= 0.0 {
        panic!("hydraulic Diameter <= 0.0, nonphysical");
    }

    if cross_sectional_area.value <= 0.0 {
        panic!("pipe Area <= 0.0, nonphysical");
    }

    let fluid_mass_flowrate = fluid_viscosity*
        cross_sectional_area/
        hydraulic_diameter*
        reynolds_number;

    return fluid_mass_flowrate;
}


/// calculates Bejan number from pressure
///
/// Be_D = Delta P * rho * D_H^2 / mu^2
pub fn calc_bejan_from_pressure(fluid_pressure: Pressure,
    hydraulic_diameter: Length,
    fluid_density: MassDensity,
    fluid_viscosity: DynamicViscosity) -> Ratio {


    if fluid_viscosity.value <= 0.0 {
        panic!("fluid Viscosity <= 0.0, nonphysical");
    }

    if hydraulic_diameter.value <= 0.0 {
        panic!("hydraulic Diameter <= 0.0, nonphysical");
    }

    if fluid_density.value <= 0.0 {
        panic!("fluidDensity <= 0.0, nonphysical");
    }

    let bejan_number_d = fluid_pressure*
        fluid_density *
        hydraulic_diameter.powi(P2::new())/
        fluid_viscosity.powi(P2::new());

    bejan_number_d
}

/// converts Bejan number to pressure
/// using:
///
/// Be_D = Delta P * rho * D_H^2 / mu^2
pub fn calc_bejan_to_pressure(bejan_d: f64,
    hydraulic_diameter: Length,
    fluid_density: MassDensity,
    fluid_viscosity: DynamicViscosity) -> Pressure {


    if fluid_viscosity.value <= 0.0 {
        panic!("fluid Viscosity <= 0.0, nonphysical");
    }

    if hydraulic_diameter.value <= 0.0 {
        panic!("hydraulic Diameter <= 0.0, nonphysical");
    }

    if fluid_density.value <= 0.0 {
        panic!("fluidDensity <= 0.0, nonphysical");
    }

    let fluid_pressure = fluid_viscosity.powi(P2::new())*
        bejan_d/
        hydraulic_diameter.powi(P2::new())/
        fluid_density;

    return fluid_pressure;
}



