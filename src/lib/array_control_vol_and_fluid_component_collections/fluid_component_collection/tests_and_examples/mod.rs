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
//    This file is part of fluid_mechanics_rust, a partial library of the
//    thermal hydraulics library written in rust meant to help with the
//    fluid mechanics aspects of the calculations
//     
//    Copyright (C) 2022-2023  Theodore Kay Chen Ong, Singapore Nuclear
//    Research and Safety Initiative, Per F. Peterson, University of 
//    California, Berkeley Thermal Hydraulics Laboratory
//
//    fluid_mechanics_rust is free software; you can redistribute it and/or modify it
//    under the terms of the GNU General Public License as published by the
//    Free Software Foundation; either version 2 of the License, or (at your
//    option) any later version.
//
//    fluid_mechanics_rust is distributed in the hope that it will be useful, but WITHOUT
//    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
//    FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
//    for more details.
//
//    This library is part of a thermal hydraulics library in rust
//    and contains some code copied from GeN-Foam, and OpenFOAM derivative.
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



/// Example 1: 
///
/// This example shows how to create a simple pipe
/// using the FluidComponent and FluidPipeCalcPressureLoss,
/// traits
///
/// this is by no means the best way to do it, but its a start
/// remember to use the relevant imports in the fluid component
/// tests
///
/// it is made of copper, 1m long, 2 in in diameter
///
/// This does not take inclined angles into consideration yet
pub mod air_pipe_example;


/// Example 2:
///
/// We saw previously how to create an air pipe
/// now we shall make a slanted water pipe
/// with some internal pressure source (as if it had a pump attached
/// to it)
///
/// we shall improve on how we can create the pipes
/// to do so, we shall use the FluidComponent trait and the 
/// FluidPipeCalcPressureChange trait
///
pub mod water_pipe_example;

/// Example 3,
/// 
/// suppose now we have a coriolis flowmeter
/// with a custom friction factor correlation
///
/// (f_darcy L/D + K) = 18 + 93000/Re^1.35
///
/// we shall use water to push flow through this coriolis flowmeter
///
/// also, the programming is rather tedious
/// because of lifetimes, but this is one example of how it can be done
pub mod coriolis_flowmeter_example;

/// Example 4 
///
///
/// Testing if fluid component structs can be put into threads with move closures
pub mod concurrency_and_multithreading_example;

/// Example 5 
///
/// fluid components in series 
pub mod collection_fluid_components_in_series;

/// Example 6
///
/// fluid components in parallel 
pub mod collection_fluid_components_in_parallel;

/// Example 7
///
/// a colletion of fluid component collections is known 
/// as a super collection. 
///
/// for example, we have three branches of fluid components connected 
/// in series 
/// They are in turn connected in parallel for CIET. 
/// To facilitate calculations here, we have super collections
pub mod super_collection_fluid_components_in_parallel;

