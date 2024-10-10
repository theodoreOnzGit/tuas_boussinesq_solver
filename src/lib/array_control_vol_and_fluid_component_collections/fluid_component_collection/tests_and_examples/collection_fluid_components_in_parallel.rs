
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
/// perhaps 10 air pipes and i want to put them in parallel
#[test]
pub fn simple_fluid_collection_example_6 () {

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
    
    let parallel_collection_of_therminol_pipes = 
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
            orientation: FluidComponentCollectionOreintation::Parallel,
        };


    // now let's have a pressure change of 1000 Pa across the parallel 
    // branches of pipes and compare it to just one pipe 
    // for 10 pipes in parallel, we should expect 10 times the mass flowrate
    
    let parallel_pipe_pressure_change = Pressure::new::<pascal>(1000.0);

    therminol_pipe_9.set_pressure_change(parallel_pipe_pressure_change);
    let single_pipe_mass_flowrate = therminol_pipe_9.get_mass_flowrate();

    let parallel_pipe_mass_flowrate = parallel_collection_of_therminol_pipes.
        get_mass_flowrate_from_pressure_change(parallel_pipe_pressure_change);

    approx::assert_relative_eq!(
        single_pipe_mass_flowrate.get::<kilogram_per_second>()*10.0,
        parallel_pipe_mass_flowrate.get::<kilogram_per_second>(),
        max_relative=0.001);


    // the expected mass flowrate for one pipe is about -1.238 kg/s
    approx::assert_relative_eq!(
        single_pipe_mass_flowrate.get::<kilogram_per_second>(),
        -1.238,
        max_relative=0.001);


    // now let's get pressure change from mass flowrate 
    // for 10 pipes, we should get back about 1000 Pa as before

    let parallel_pipe_pressure_change = parallel_collection_of_therminol_pipes.
        get_pressure_change(MassRate::new::<kilogram_per_second>(-12.38));

    approx::assert_relative_eq!(
        parallel_pipe_pressure_change.get::<pascal>(),
        1000.0,
        max_relative=0.001);

    // now we don't have elevation, and no pressure source, so pressure 
    // change should equal minus pressure loss 

    let parallel_pipe_pressure_loss = parallel_collection_of_therminol_pipes.
        get_pressure_loss(MassRate::new::<kilogram_per_second>(-12.38));

    assert_eq!(-parallel_pipe_pressure_loss,
        parallel_pipe_pressure_change);

    // we also want to try tests for reverse flow, should be 12.38 kg/s

    let parallel_pipe_reverse_mass_flowrate = parallel_collection_of_therminol_pipes.
        get_mass_flowrate_from_pressure_change(-parallel_pipe_pressure_change);

    approx::assert_relative_eq!(
        parallel_pipe_reverse_mass_flowrate.get::<kilogram_per_second>(),
        12.38,
        max_relative=0.001);

    // and when we subject the parallel collection to 12.38 kg/s of flow, 
    // we should get -1000Pa of pressure change

    let parallel_pipe_pressure_change_reverse = parallel_collection_of_therminol_pipes.
        get_pressure_change(MassRate::new::<kilogram_per_second>(12.38));


    approx::assert_relative_eq!(
        parallel_pipe_pressure_change_reverse.get::<pascal>(),
        -1000.0,
        max_relative=0.001);

}
