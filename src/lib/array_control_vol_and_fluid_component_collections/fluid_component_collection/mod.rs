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
/// License
///    This file is part of tuas_boussinesq_solver, a partial library of the
///    thermal hydraulics library written in rust meant to help with the
///    fluid mechanics and heat transfer aspects of the calculations
///     
///    Copyright (C) 2022-2023  Theodore Kay Chen Ong, Singapore Nuclear
///    Research and Safety Initiative, Per F. Peterson, University of 
///    California, Berkeley Thermal Hydraulics Laboratory
///
///    tuas_boussinesq_solver is free software; you can redistribute it and/or modify it
///    under the terms of the GNU General Public License as published by the
///    Free Software Foundation; either version 2 of the License, or (at your
///    option) any later version.
///
///    tuas_boussinesq_solver is distributed in the hope that it will be useful, but WITHOUT
///    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
///    FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
///    for more details.
///
///    This library is part of a thermal hydraulics library in rust
///    and contains some code copied from GeN-Foam, and OpenFOAM derivative.
///    This offering is not approved or endorsed by the OpenFOAM Foundation nor
///    OpenCFD Limited, producer and distributor of the OpenFOAM(R)software via
///    www.openfoam.com, and owner of the OPENFOAM(R) and OpenCFD(R) trademarks.
///    Nor is it endorsed by the authors and owners of GeN-Foam.
///
///    You should have received a copy of the GNU General Public License
///    along with this program.  If not, see <http://www.gnu.org/licenses/>.
///
/// © All rights reserved. Theodore Kay Chen Ong,
/// Singapore Nuclear Research and Safety Initiative,
/// Per F. Peterson,
/// University of California, Berkeley Thermal Hydraulics Laboratory
///
/// Main author of the code: Theodore Kay Chen Ong, supervised by
/// Professor Per F. Peterson


// the peroxide crate for root finders

// another crate for root finders, in fact this package specialises in root
// finding


// contains associated functions which take a fluid component
// vector and calculate mass flowrates and pressure changes
// and losses from it
//
// this assumes that all the components in the vector
// are connected in series
//
//
// note that the iterative methods of finding mass flowrate from pressure change
// for this is EXPERIMENTAL,
// convergence is not guaranteed. 
// Use at your own risk


/// FluidComponents are pipes and fittings you can connect in parallel
/// such that you can calculate mass flowrate and pressure drop from them
///
/// These are usually array control volumes, but could be other components 
/// as well
pub mod fluid_component;


/// contains a trait for use in fluid components and 
/// collections of fluid components
/// This is because mass flowrate and pressure drop also need to be 
/// calculated from collections of fluid components
pub mod fluid_component_traits;

/// contains functions which calculate mass flowrate and pressure drop 
/// for components connected in series or parallel 
pub mod collection_series_and_parallel_functions;

/// contains functions which calculate mass flowrate and pressure drop 
/// for branches or fluid component collections connected in series or parallel 
pub mod super_collection_series_and_parallel_functions;

/// fluid component collections 
/// these are vectors of fluid components 
pub mod fluid_component_collection;

/// fluid component super collections
/// these are vectors of fluid component collections 
/// usually used for calculating multiple branches in parallel 
pub mod fluid_component_super_collection;

/// some examples which show how to use the functionality of the fluid 
/// mechanics correlation libraries
pub mod tests_and_examples;















