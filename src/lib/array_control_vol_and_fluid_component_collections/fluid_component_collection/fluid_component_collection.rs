use std::fmt::Debug;

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
use uom::si::f64::{Pressure, MassRate};
use uom::si::mass_rate::kilogram_per_second;

use super::collection_series_and_parallel_functions::FluidComponentCollectionSeriesAssociatedFunctions;
use super::collection_series_and_parallel_functions::FluidComponentCollectionParallelAssociatedFunctions;
use super::fluid_component::FluidComponent;
use crate::thermal_hydraulics_error::ThermalHydraulicsLibError;


/// a fluid component collection,
/// which contains fluid components stored into a vector
/// and should contain some methods for CRUD operations
///
/// Create
/// Read
/// Update
/// Delete
///
#[derive(Debug,Clone,PartialEq)]
pub struct FluidComponentCollection {
    /// this vector is the collection of fluid components 
    pub components: Vec<FluidComponent>,
    /// this decides if the components are connected in series 
    /// or parallel
    pub orientation: FluidComponentCollectionOreintation
}

/// tells you whether the components in FluidComponentCollection
/// or FluidComponentSuperCollection are connected in series or parallel
#[derive(Debug,Clone,PartialEq)]
pub enum FluidComponentCollectionOreintation {
    /// fluid components are connected in series
    Parallel,
    /// fluid components are connected in parallel
    Series
}


impl FluidComponentCollection{


    /// returns a copy of the fluid component vector
    /// containing immutable elements
    ///
    /// you'll probably need some legwork to create a fresh
    /// object
    pub fn get_immutable_fluid_component_vector(&self) 
        -> Vec<FluidComponent>{
            self.components.clone()
    }

    /// sets the fluid component vector to a specific value
    pub fn set_fluid_component_vector(
        &mut self,
        fluid_component_vector: Vec<FluidComponent>){
        self.components = fluid_component_vector;
    }


    /// adds a fluid component to the collection

    pub fn add_fluid_component(
        &mut self,
        fluid_component: FluidComponent){

        // then i push the pointer to this mutable copy
        self.components.push(fluid_component);

    }


    /// removes a fluid component by index from the collection

    pub fn remove_fluid_component(&mut self,
                              component_index: usize)-> 
        Result<(),ThermalHydraulicsLibError>{

        // i remove the index from the vector 
        // (note that there may be a case where the vector is smaller than
        // the given index),
        // however, the remove method already has a panic if the 
        // vector is shorter than the given index

        self.components.remove(component_index);
        Ok(())

    }

    /// returns read only a pointer of the fluid component 
    /// given an index

    pub fn get_fluid_component(
        &self,
        component_index: usize) -> Result<FluidComponent,ThermalHydraulicsLibError> {

        // first let's access the fluid component


        let fluid_component = 
            self.get_immutable_fluid_component_vector()[component_index].clone();

        return Ok(fluid_component);

    }

    /// updates the fluid component at the specified
    /// index with a fluid component supplied by the user

    pub fn update_fluid_component(
        &mut self,
        component_index: usize,
        fluid_component: FluidComponent){

        // then i change the pointer in this mutable copy
        self.components[component_index] = fluid_component;

    }

    /// new empty series component collection 
    pub fn new_series_component_collection()-> Self {

        Self { 
            components: vec![], 
            orientation: FluidComponentCollectionOreintation::Series 
        }
    }

    /// new empty parallel component collection 
    pub fn new_parallel_component_collection()-> Self {

        Self { 
            components: vec![], 
            orientation: FluidComponentCollectionOreintation::Parallel
        }
    }

    /// clones anything that can be converted (try)into a FluidComponent 
    /// and adds it to the component list
    pub fn try_clone_and_add_component
        <T:TryInto<FluidComponent> + Clone + Debug >(
        &mut self,
        component: &T,
    ) -> Result<(), ThermalHydraulicsLibError>
        where <T as TryInto<FluidComponent>>::Error: Debug
    {

        // first, we clone the component, and convert it into a 
        // fluid component

        let component_clone: FluidComponent = 
            component.clone().try_into().unwrap();

        self.components.push(component_clone);

        Ok(())
    }

    /// clones anything that can be converted into a FluidComponent 
    /// and adds it to the component list
    pub fn clone_and_add_component
        <T:Into<FluidComponent> + Clone >(
        &mut self,
        component: &T,
    ) -> ()
    {

        // first, we clone the component, and convert it into a 
        // fluid component

        let component_clone: FluidComponent = 
            component.clone().into();

        self.components.push(component_clone);

    }

    /// empties the vector 
    pub fn empty_vector(&mut self,) -> 
        Result<(),ThermalHydraulicsLibError>{
            self.components.clear();
            Ok(())
    }


}

impl FluidComponentCollectionMethods for FluidComponentCollection {
    fn get_pressure_change(
        &self, 
        fluid_mass_flowrate: MassRate) -> Pressure {
        
        let orientation = &self.orientation;

        match orientation {
            FluidComponentCollectionOreintation::Parallel => {
                let fluid_component_vector = &self.components;
                <Self as FluidComponentCollectionParallelAssociatedFunctions>::
                    calculate_pressure_change_from_mass_flowrate(
                        fluid_mass_flowrate, fluid_component_vector)
            },
            FluidComponentCollectionOreintation::Series => {
                let fluid_component_vector = &self.components;
                <Self as FluidComponentCollectionSeriesAssociatedFunctions>::
                    calculate_pressure_change_from_mass_flowrate(
                        fluid_mass_flowrate, fluid_component_vector)
            },
        }


    }

    fn get_mass_flowrate_from_pressure_change(
        &self,
        pressure_change: Pressure) -> MassRate {
        let orientation = &self.orientation;

        match orientation {
            FluidComponentCollectionOreintation::Parallel => {
                let fluid_component_vector = &self.components;
                <Self as FluidComponentCollectionParallelAssociatedFunctions>::
                    calculate_mass_flowrate_from_pressure_change(
                        pressure_change, fluid_component_vector)
            },
            FluidComponentCollectionOreintation::Series => {
                let fluid_component_vector = &self.components;
                <Self as FluidComponentCollectionSeriesAssociatedFunctions>::
                    calculate_mass_flowrate_from_pressure_change(
                        pressure_change, fluid_component_vector)
            },
        }
    }
}

impl FluidComponentCollectionSeriesAssociatedFunctions for FluidComponentCollection {

}

impl FluidComponentCollectionParallelAssociatedFunctions for FluidComponentCollection {

}


/// contains methods to get pressure loss 
/// and pressure change and mass flowrate based on 
/// current state of the fluid component collection
pub trait FluidComponentCollectionMethods{

    /// calculates pressure loss when given a mass flowrate
    fn get_pressure_loss(
        &self, 
        fluid_mass_flowrate: MassRate) -> Pressure {

        // for pressure losses, we compare the pressure change at
        // zero mass flowrate to pressure change at the desired
        // mass flowrate
        // noting that 
        //
        // pressure_change = - pressure_loss + hydrostatic pressure +
        // internal pressure


        let zero_mass_flow = MassRate::new::<kilogram_per_second>(0.0);

        let reference_pressure_change = 
            self.get_pressure_change(zero_mass_flow);

        let current_pressure_change = 
            self.get_pressure_change(fluid_mass_flowrate);

        let pressure_change_due_to_losses = 
            current_pressure_change - reference_pressure_change;

        let pressure_loss = -pressure_change_due_to_losses;

        return pressure_loss;

    }

    /// calculates pressure change when given a mass flowrate
    fn get_pressure_change(
        &self, 
        fluid_mass_flowrate: MassRate) -> Pressure;

    /// calculates mass flowrate from pressure change

    fn get_mass_flowrate_from_pressure_change(
        &self,
        pressure_change: Pressure) -> MassRate;

    /// calculates mass flowrate from pressure loss
    
    fn get_mass_flowrate_from_pressure_loss(
        &self,
        pressure_loss: Pressure) -> MassRate {

        // for this, the default implementation is
        // to obtain pressure change
        //
        // pressure_change = -pressure_loss +
        // hydrostatic pressure
        // + internal pressure
        //
        // to get the latter two terms, i can obtain
        // pressure change when mass flowrate is zero
        let zero_mass_flow = MassRate::new::<kilogram_per_second>(0.0);

        let reference_pressure_change = 
            self.get_pressure_change(zero_mass_flow);

        let pressure_change = 
            -pressure_loss + reference_pressure_change;

        // now let's calculate the mass flowrate

        return self.get_mass_flowrate_from_pressure_change(pressure_change);
    }


}

