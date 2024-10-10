
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






/// Here is a test which is meant to test a simple struct made
/// to hold and calculate fluid component collections
///
/// First i make a typical fluid component, a set of air pipes
/// perhaps 10 air pipes and i want to put them in series
#[test]
pub fn simple_fluid_collection_example_5 () {

    // this tests the calc pressure loss for fluid component 
    use uom::si::f64::*;
    use uom::si::length::inch;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::pressure::pascal;
    use uom::si::length::meter;
    use uom::si::angle::degree;

    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;

    use uom::si::{pressure::atmosphere, thermodynamic_temperature::kelvin};

    use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component::FluidComponent;
    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_collection::FluidComponentCollectionOreintation;
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_collection::FluidComponentCollection;
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_collection::FluidComponentCollectionMethods;


    // honestly, I am quite unsure as to how to structure the 
    //
    // fluid mechanics and thermal hydraulics calculations.
    // 
    //
    // there are several ways we can do this... 
    //
    //
    // We could first manage one component at a time, and manually 
    // sum up the pressure drop contributions of each component as well 
    // as manually update each of the component temperatures
    //
    // This, however, is a very cumbersome way to coding. It is extremely 
    // error prone.
    //
    // To abstract away some of this, we could have individual heat 
    // transfer entities, and at each timestep, clone their relevant fluid 
    // arrays into a FluidComponentCollection. These FluidComponentCollections 
    // can then be arranged into a FluidComponentSuperCollection
    // and then the relevant fluid mechanics calculations could be 
    // performed.
    //
    // The heat transfer entities would then have their 
    // mass flowrates sorted out manually at each timestep.
    //
    // The most abstract way would be to couple all components into a 
    // collection. However, given that each component has multiple 
    // heat transfer entities in it, this may not be the best way to do 
    // things.
    //
    // For now, let's opt for way two, where we have HeatTransferEntity 
    // objects converted into FluidComponent objects, and then arranged 
    // into arrays.
    //
    // HeatTransferEntity objects are usually in the pre_built_components 
    // section. So I won't test them here.
    // However, these can be converted into FluidComponent objects.
    // So we'll just work with fluid component objects for now.

    let adjacent_solid_material = SolidMaterial::Copper;
    let liquid_material = LiquidMaterial::TherminolVP1;
    let initial_pressure = Pressure::new::<atmosphere>(1.0);
    let initial_temperature = ThermodynamicTemperature::new::<kelvin>(298.0);
    let hydraulic_diameter = Length::new::<inch>(2.0);
    let pipe_incline_angle = Angle::new::<degree>(0.0);
    let pipe_form_loss = 5.0;
    let user_specified_inner_nodes = 0;
    let length = Length::new::<meter>(1.0);

    let therminol_pipe_1: FluidComponent = FluidComponent::FluidArray(
        FluidArray::new_cylinder(
            length, 
            hydraulic_diameter, 
            initial_temperature, 
            initial_pressure, 
            adjacent_solid_material, 
            liquid_material, 
            pipe_form_loss.into(), 
            user_specified_inner_nodes, 
            pipe_incline_angle)
        );

    let therminol_pipe_2 = therminol_pipe_1.clone();
    let therminol_pipe_3 = therminol_pipe_1.clone();
    let therminol_pipe_4 = therminol_pipe_1.clone();
    let therminol_pipe_5 = therminol_pipe_1.clone();
    let therminol_pipe_6 = therminol_pipe_1.clone();
    let therminol_pipe_7 = therminol_pipe_1.clone();
    let therminol_pipe_8 = therminol_pipe_1.clone();
    let mut therminol_pipe_9 = therminol_pipe_1.clone();
    let therminol_pipe_10 = therminol_pipe_1.clone();

    // let's now put all of them in series.
    
    let series_collection_of_therminol_pipes = 
        FluidComponentCollection{
            components: vec![therminol_pipe_1.clone(),
            therminol_pipe_2.clone(),
            therminol_pipe_3.clone(),
            therminol_pipe_4.clone(),
            therminol_pipe_5.clone(),
            therminol_pipe_6.clone(),
            therminol_pipe_7.clone(),
            therminol_pipe_8.clone(),
            therminol_pipe_9.clone(),
            therminol_pipe_10.clone(),],
            orientation: FluidComponentCollectionOreintation::Series,
        };


    // now let's push a 0.1kg/s fluid flow through this pipe series
    //
    let pipe_fluid_flow = MassRate::new::<kilogram_per_second>(0.1);

    // and then let's get the pressure change

    let series_pipe_pressure_change = series_collection_of_therminol_pipes.
        get_pressure_change(pipe_fluid_flow);

    // the pressure losses are about -81.61 Pa
    approx::assert_relative_eq!(
        series_pipe_pressure_change.get::<pascal>(),
        -81.61,
        max_relative=0.001);
    // the pressure drop across 1 pipe should be about 1/10th of this value
    
    therminol_pipe_9.set_mass_flowrate(pipe_fluid_flow);
    let single_pipe_pressure_change = therminol_pipe_9.get_pressure_change();

    // the pressure losses are about -81.61 Pa over the series of pipes,
    // the pressure change across 1 pipe should be 1/10th of that if 
    // in series
    approx::assert_relative_eq!(
        series_pipe_pressure_change.get::<pascal>()/10.0,
        single_pipe_pressure_change.get::<pascal>(),
        max_relative=0.001);

    // since there is no elevation, or pressure source internally
    // the pressure change and pressure loss are the same (in magnitude)

    let series_pipe_pressure_loss = series_collection_of_therminol_pipes.
        get_pressure_loss(pipe_fluid_flow);

    assert_eq!(-series_pipe_pressure_change,
        series_pipe_pressure_loss);


    // all right, so now we want to check if the same pressure loss
    // will yield us 0.001 kg/s

    let series_pipe_mass_flowrate = series_collection_of_therminol_pipes.
        get_mass_flowrate_from_pressure_loss(series_pipe_pressure_loss);


    // we should get back our 0.1 kg/s if all else works fine
    approx::assert_relative_eq!(
        series_pipe_mass_flowrate.get::<kilogram_per_second>(),
        0.1,
        max_relative=0.001);

    // test complete!

}
