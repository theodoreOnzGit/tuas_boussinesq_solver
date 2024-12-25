use std::f64::consts::PI;

use uom::si::angle::degree;
use uom::si::area::square_meter;

/// builds ciet branches for the isothermal test
pub mod ciet_branch_builders_isothermal;

/// tests for the heater branch and ctah branch only
pub mod isothermal_ctah_heater_branch_test;
/// tests for the heater branch, ctah branch and dhx branch
pub mod isothermal_ctah_heater_dhx_branch_test;

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
//    This file is part of tuas_boussinesq_solver, a partial library of the
//    thermal hydraulics library written in rust meant to help with the
//    fluid mechanics aspects of the calculations
//     
//    Copyright (C) 2022-2023  Theodore Kay Chen Ong, Singapore Nuclear
//    Research and Safety Initiative, Per F. Peterson, University of 
//    California, Berkeley Thermal Hydraulics Laboratory
//
//    tuas_boussinesq_solver is free software; you can redistribute it and/or modify it
//    under the terms of the GNU General Public License as published by the
//    Free Software Foundation; either version 2 of the License, or (at your
//    option) any later version.
//
//    tuas_boussinesq_solver is distributed in the hope that it will be useful, but WITHOUT
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
use uom::si::f64::*;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::length::{meter, millimeter};
use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::pressure::atmosphere;

use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;

use super::insulated_pipes_and_fluid_components::InsulatedFluidComponent;
use super::non_insulated_fluid_components::NonInsulatedFluidComponent;

/// creates a new pipe6a for CIET using the RELAP5-3D and SAM parameters 
/// Pipe6a in Compact Integral Effects Test (CIET)
/// CTAH branch 
///
/// It is a static mixer pipe
/// otherwise known as the static mixer pipe 6a
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_6a(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.1526);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(51.526384);
    let form_loss = Ratio::new::<ratio>(5.05);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    let user_specified_inner_nodes = 0; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// creates a new pipe6a for CIET using the RELAP5-3D and SAM parameters 
/// Component 6 in Compact Integral Effects Test (CIET)
/// CTAH branch  (also known as static mixer 41)
///
/// static mixer 41
/// label component 6 
/// in Compact Integral Effects Test (CIET)
/// CTAH branch 
/// static mixer 41 (MX-41) on CIET diagram
/// in the pump and CTAH branch
/// just before CTAH (AKA IHX)
/// from top to bottom
///
/// label 6 on diagram
///
/// It is a static mixer pipe
/// otherwise known as the static mixer pipe 6a
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_static_mixer_41_label_6(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(0.33);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(51.526384);
    let form_loss = Ratio::new::<ratio>(21.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(4000.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    let user_specified_inner_nodes = 0; 

    let insulated_component = InsulatedFluidComponent::new_custom_component(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        reynolds_coefficient, 
        reynolds_power, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        component_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes);

    insulated_component
}

/// creates a new ctah vertical for CIET using the RELAP5-3D and SAM parameters 
/// in Compact Integral Effects Test (CIET)
///
/// this is inactive, so it behaves more like a pipe rather than a 
/// heat exchanger
///
/// Vertical part of Coiled Tube Air Heater (CTAH)
/// label component 7a
/// in Compact Integral Effects Test (CIET)
/// CTAH branch 
///
/// It is NOT insulated by the way
///
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
pub fn new_inactive_ctah_vertical(initial_temperature: ThermodynamicTemperature) -> 
NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(1.19e-2);
    let pipe_length = Length::new::<meter>(0.3302);
    let flow_area = Area::new::<square_meter>(1.33E-03);
    let incline_angle = Angle::new::<degree>(-90.0);
    let form_loss = Ratio::new::<ratio>(3.9);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.000406);
    let od = id + pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 3 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    let user_specified_inner_nodes = 3-2; 

    let non_insulated_component = NonInsulatedFluidComponent::new_bare_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        id, 
        od, 
        pipe_length, 
        hydraulic_diameter, 
        surface_roughness, 
        pipe_shell_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes);

    non_insulated_component
}



/// creates a new ctah vertical for CIET using the RELAP5-3D and SAM parameters 
/// in Compact Integral Effects Test (CIET)
///
/// this is inactive, so it behaves more like a pipe rather than a 
/// heat exchanger
///
/// Horizontal part of Coiled Tube Air Heater (CTAH)
/// label component 7b
/// in Compact Integral Effects Test (CIET)
/// CTAH branch 
/// coiled tube air heater
/// has fldk = 400 + 52,000/Re
///
/// label is 7b
/// empirical data in page 48 on pdf viewer in Dr
/// Zweibaum thesis shows reverse flow has same
/// pressure drop characteristics as forward flow
///
/// It is NOT insulated by the way
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
///
pub fn new_inactive_ctah_horizontal(initial_temperature: ThermodynamicTemperature) -> NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(1.19e-2);
    let component_length = Length::new::<meter>(1.2342);
    let flow_area = Area::new::<square_meter>(1.33E-03);
    let incline_angle = Angle::new::<degree>(0.0);
    let form_loss = Ratio::new::<ratio>(400.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(52000_f64);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.000406);
    let shell_od = shell_id + pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 11 nodes only, 
    // now because there are two outer nodes, 
    // we subtract 2 
    let user_specified_inner_nodes = 11-2; 

    let non_insulated_component = NonInsulatedFluidComponent::
        new_custom_component(
            initial_temperature, 
            ambient_temperature, 
            fluid_pressure, 
            solid_pressure, 
            flow_area, 
            incline_angle, 
            form_loss, 
            reynolds_coefficient, 
            reynolds_power, 
            shell_id, 
            shell_od, 
            component_length, 
            hydraulic_diameter, 
            pipe_shell_material, 
            pipe_fluid, 
            htc_to_ambient, 
            user_specified_inner_nodes);

    non_insulated_component
}


/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// It is a static mixer pipe
/// Static mixer pipe 8a
/// adjacent to MX-40 in the CTAH branch
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_8a(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.22245);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(-90.0);
    let form_loss = Ratio::new::<ratio>(3.75);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0381);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    let user_specified_inner_nodes = 0; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// static mixer 40 (MX-40) on CIET diagram
/// just after CTAH (AKA IHX)
/// from top to bottom
/// label 8 on diagram
///
/// forced convection flow direction is same as top to bottom
/// has a fldk of 21+4000/Re
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_static_mixer_40_label_8(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(0.33);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(-90.0);
    let form_loss = Ratio::new::<ratio>(21.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(4000_f64);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, we subtract 2
    let user_specified_inner_nodes = 2-2; 

    // insulation 
    let insulation_thickness = Length::new::<meter>(0.0381);
    let insulation_material = SolidMaterial::Fiberglass;


    let insulated_component = InsulatedFluidComponent::
        new_custom_component(
            initial_temperature, 
            ambient_temperature, 
            fluid_pressure, 
            solid_pressure, 
            flow_area, 
            incline_angle, 
            form_loss, 
            reynolds_coefficient, 
            reynolds_power, 
            shell_id, 
            shell_od, 
            insulation_thickness, 
            component_length, 
            hydraulic_diameter, 
            pipe_shell_material, 
            insulation_material, 
            pipe_fluid, 
            htc_to_ambient, 
            user_specified_inner_nodes);

    insulated_component

}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// pipe number 9 in CIET's CTAH branch
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_9(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.7112);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(-42.73211);
    let form_loss = Ratio::new::<ratio>(0.8);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0381);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 7 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 7-2
    let user_specified_inner_nodes = 7-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// pipe number 10 in CIET's CTAH branch
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_10(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(2.4511);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(-90.0);
    let form_loss = Ratio::new::<ratio>(0.45);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0381);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 22 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 20
    let user_specified_inner_nodes = 22-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// pipe number 11 in CIET's CTAH branch
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_11(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.4826);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(-63.47465);
    let form_loss = Ratio::new::<ratio>(2.4);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0381);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 4 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 4-2
    let user_specified_inner_nodes = 4-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}


/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// pipe number 12 in CIET's CTAH branch
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_12(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.333375);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(0.0);
    let form_loss = Ratio::new::<ratio>(21.65);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 3 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 3-2
    let user_specified_inner_nodes = 3-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// ctah pump is a custom therminol component with
/// ie no friction factor losses
/// but it provides a source pressure
///
/// it is located between pipe 12 and 13
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_ctah_pump(initial_temperature: ThermodynamicTemperature) -> NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(0.36);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(0.0);
    let form_loss = Ratio::new::<ratio>(0.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, we subtract 2
    let user_specified_inner_nodes = 2-2; 



    let non_insulated_component = NonInsulatedFluidComponent::
        new_custom_component(
            initial_temperature, 
            ambient_temperature, 
            fluid_pressure, 
            solid_pressure, 
            flow_area, 
            incline_angle, 
            form_loss, 
            reynolds_coefficient, 
            reynolds_power, 
            shell_id, 
            shell_od, 
            component_length, 
            hydraulic_diameter, 
            pipe_shell_material, 
            pipe_fluid, 
            htc_to_ambient, 
            user_specified_inner_nodes);

    non_insulated_component

}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// pipe number 13 in CIET's CTAH branch
/// just after the pump
/// pipe 13 on the diagram in Nico Zweibaum nodalisation
/// probably some combination of V-42,
/// F-40 and F-41 on CIET diagram
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_13(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(1.273175);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(0.0);
    let form_loss = Ratio::new::<ratio>(12.95);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 12 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 12-2
    let user_specified_inner_nodes = 12-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}


/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// pipe number 14 in CIET's CTAH branch
/// pipe 14 on the diagram in Nico Zweibaum nodalisation
/// probably some combination of V-42,
/// F-40 and F-41 on CIET diagram
/// it is inclined 90 degrees upwards in direction
/// of flow
///
/// and from a top to bottom direction from pipe 5
/// to pipe 17, the incline angle is also 90 degrees
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_14(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.6687);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(90.0);
    let form_loss = Ratio::new::<ratio>(2.4);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 6 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 6-2
    let user_specified_inner_nodes = 6-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}


/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// FM-40 Coriolis Flowmeter in CIET's CTAH branch
/// labelled 14a in simulation schmeatic
///
/// ctah line flowmeter 40
/// label 14a on simulation diagram
/// fldk = 18.0+93000/(Re^1.35)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_flowmeter_40_14a(initial_temperature: ThermodynamicTemperature) -> NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(0.36);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(90.0);
    let form_loss = Ratio::new::<ratio>(18.0);
    let reynolds_power = -1.35_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(93000_f64);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, we subtract 2
    let user_specified_inner_nodes = 2-2; 



    let non_insulated_component = NonInsulatedFluidComponent::
        new_custom_component(
            initial_temperature, 
            ambient_temperature, 
            fluid_pressure, 
            solid_pressure, 
            flow_area, 
            incline_angle, 
            form_loss, 
            reynolds_coefficient, 
            reynolds_power, 
            shell_id, 
            shell_od, 
            component_length, 
            hydraulic_diameter, 
            pipe_shell_material, 
            pipe_fluid, 
            htc_to_ambient, 
            user_specified_inner_nodes);

    non_insulated_component

}


/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// pipe number 15 in CIET's CTAH branch
///
/// pipe 15 on the diagram in Nico Zweibaum nodalisation
/// probably corresponds of F30 on CIET's P&ID
///
/// and from a top to bottom direction from pipe 5
/// to pipe 17, the incline angle is also
/// -49.36983 degrees
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_15(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.3556);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(-49.36983);
    let form_loss = Ratio::new::<ratio>(0.8);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 3 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 3-2
    let user_specified_inner_nodes = 3-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}


/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// pipe number 16 in CIET's CTAH branch
///
/// pipe 16 on the diagram in Nico Zweibaum nodalisation
/// probably corresponds of F30 on CIET's P&ID
///
/// and from a top to bottom direction from pipe 5
/// to pipe 17, the incline angle is also
/// -49.36983 degrees
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_16(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.644525);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(-90.0);
    let form_loss = Ratio::new::<ratio>(1.9);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 6 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 6-2
    let user_specified_inner_nodes = 6-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}


/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// Branch (or pipe 17) in CIET's CTAH branch
///
/// Approximations were made for this branch though,
/// technically branch 17a is part of CTAH branch
/// while 17b is part of the DHX branch,
/// I combined both for convenience
/// pipe 17 on the diagram in Nico Zweibaum nodalisation
/// probably corresponds of F30 on CIET's P&ID
///
/// and from a top to bottom direction from pipe 5
/// to pipe 17, the incline angle is 0 degrees
///
///
/// This is treated as a single pipe though
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_branch_17(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.473075);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(0.0);
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 6 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 6-2
    let user_specified_inner_nodes = 6-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters,
/// this is branch 17b within the DHX branch in the SAM model
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_branch_17b(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.094615);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(0.0);
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 2-2
    let user_specified_inner_nodes = 2-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters,
/// this is branch 17a within the CTAH branch in the SAM model
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_branch_17a(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.37846);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(0.0);
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 2-2
    let user_specified_inner_nodes = 2-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
/// Branch 5a in the SAM model, using RELAP models for pipe length 
/// and hydrualic diameter 
///
/// located in the DHX branch
///
/// this is reverse order compared to table A-1 in
/// the Zweibaum nodalised relap model
/// pipe 5a on the diagram in SAM nodalisation
/// and from a top to bottom direction from pipe 5
/// to pipe 5, the incline angle is also
/// 0 degrees
/// i add 180 degrees so that it is
/// properly reversed in
/// inclination angle from top to bottom
///
/// This is treated as a single pipe though
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_branch_5a(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.37465);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(0.0 + 180.0);
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 2-2
    let user_specified_inner_nodes = 2-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
/// Branch 5b in the SAM model, using RELAP models for pipe length 
/// and hydrualic diameter 
///
/// located in the CTAH branch. 
/// But for the purposes of hydrodynamic modelling and heat transfer 
/// modelling, it is the same as the branch 5a
///
/// this is reverse order compared to table A-1 in
/// the Zweibaum nodalised relap model
/// pipe 5a on the diagram in SAM nodalisation
/// and from a top to bottom direction from pipe 5
/// to pipe 5, the incline angle is also
/// 0 degrees
/// i add 180 degrees so that it is
/// properly reversed in
/// inclination angle from top to bottom
///
/// This is treated as a single pipe though
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_branch_5b(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.37465);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(0.0 + 180.0);
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 2-2
    let user_specified_inner_nodes = 2-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}


/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// Branch 5 in the Heater Branch (top to bottom perspective)
/// 
/// Approximations were made for this branch though,
/// technically branch 5a is part of DHX branch
/// while 5b is part of the DHX branch,
/// I combined both for convenience
///
/// This is treated as a single pipe though
///
/// Now I'd probably made a mistake putting branch 5 in
/// the heater branch, it's probably better put inside the
/// CTAH branch, (as of Oct 2022)
/// I'll probably put this in the CTAH branch in future
///
/// But for forced isothermal circulation tests with only
/// the heater branch and CTAH branch, it doesn't really matter
/// since there are only two branches
///
/// So no matter which branch you put branch or pipe 5 in,
/// it is still the same set of pipes in series
/// calculations will still be the same numerically
///
/// 
// this is reverse order compared to table A-1 in
// the Zweibaum nodalised relap model
/// pipe 5 on the diagram in Nico Zweibaum nodalisation
/// and from a top to bottom direction from pipe 5
/// to pipe 5, the incline angle is also
/// 0 degrees
/// i add 180 degrees so that it is
/// properly reversed in
/// inclination angle from top to bottom
///
/// This is treated as a single pipe though
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_branch_5(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.7493);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(0.0 + 180.0);
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 9 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 9-2
    let user_specified_inner_nodes = 9-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}


/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// pipe 4 within the heater branch
/// pipe 4 on the diagram in Nico Zweibaum nodalisation
/// probably corresponds of V11 and F12
///
/// and from a top to bottom direction from pipe 5
/// to pipe 17, the incline angle is also
/// 49.743387 +180.0 degrees
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_4(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.2413);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(49.743387 + 180.0);
    let form_loss = Ratio::new::<ratio>(2.4);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 2-2
    let user_specified_inner_nodes = 2-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}


/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// pipe3 within the heater branch
/// pipe 3 on the diagram in Nico Zweibaum nodalisation
/// probably corresponds of V11 and F12
///
/// and from a top to bottom direction from pipe 5
/// to pipe 17, the incline angle is also
/// 90.0 +180.0 degrees
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_3_relap_model(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(1.2827);
    let flow_area = hydraulic_diameter * hydraulic_diameter * PI/4.0;
    let incline_angle = Angle::new::<degree>(90.0 + 180.0);
    let form_loss = Ratio::new::<ratio>(3.15);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 12 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 12-2
    let user_specified_inner_nodes = 12-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// pipe3 within the heater branch
/// pipe 3 on the diagram in Nico Zweibaum nodalisation
/// probably corresponds of V11 and F12
///
/// and from a top to bottom direction from pipe 5
/// to pipe 17, the incline angle is also
/// 90.0 +180.0 degrees
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
/// This uses the SAM parameters rather than the RELAP parameters 
/// which uses 17.15 K value for pipe 3 rather than 3.15
pub fn new_pipe_3_sam_model(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(1.2827);
    let flow_area = hydraulic_diameter * hydraulic_diameter * PI/4.0;
    let incline_angle = Angle::new::<degree>(90.0 + 180.0);
    let form_loss = Ratio::new::<ratio>(17.15);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 12 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 12-2
    let user_specified_inner_nodes = 12-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// MX-10 within the heater branch
/// labelled as component 2
///
///
/// static mixer 10 (MX-10) on CIET diagram
/// just before the heater in the heater branch
/// from top to bottom
/// label 2 on diagram (fig A-1 on Nico Zweibaum thesis)
/// pg 125 on pdf viewer, pg 110 on printed page number on bottom right
///
/// though in reality flow goes from bottom to
/// top in forced convection
/// so from a flow perspective it is before the
/// heater
///
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_static_mixer_10_label_2(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(0.33);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(21.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(4000_f64);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, we subtract 2
    let user_specified_inner_nodes = 2-2; 

    // insulation 
    let insulation_thickness = Length::new::<meter>(0.0381);
    let insulation_material = SolidMaterial::Fiberglass;


    let insulated_component = InsulatedFluidComponent::
        new_custom_component(
            initial_temperature, 
            ambient_temperature, 
            fluid_pressure, 
            solid_pressure, 
            flow_area, 
            incline_angle, 
            form_loss, 
            reynolds_coefficient, 
            reynolds_power, 
            shell_id, 
            shell_od, 
            insulation_thickness, 
            component_length, 
            hydraulic_diameter, 
            pipe_shell_material, 
            insulation_material, 
            pipe_fluid, 
            htc_to_ambient, 
            user_specified_inner_nodes);

    insulated_component

}
/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// static mixer pipe2a in heater branch
///
/// adjacent to MX-10
/// pipe 2a on the diagram in Nico Zweibaum nodalisation
/// probably corresponds of V11 and F12
///
/// and from a top to bottom direction from pipe 5
/// to pipe 17, the incline angle is also
/// 90.0 +180.0 degrees
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_2a(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.149425);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(90.0 + 180.0);
    let form_loss = Ratio::new::<ratio>(1.8);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 2-2
    let user_specified_inner_nodes = 2-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// heater top head 1a of heater branch in CIET
///
/// heater top head
/// diagram label 1a
///
/// inclined at 90 degrees bottom to top
/// or 90 degrees + 180 top to bottom orientation
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_heater_top_head_1a(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(6.60e-3);
    let pipe_length = Length::new::<meter>(0.0889);
    let flow_area = Area::new::<square_meter>(3.64e-4);
    let incline_angle = Angle::new::<degree>(90.0 + 180.0);
    let form_loss = Ratio::new::<ratio>(3.75);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.001905);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 2-2
    let user_specified_inner_nodes = 2-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// This is the first version of CIET's heater
/// 
/// It is found in CIET's heater branch;
/// It has hydrodynamic losses similar to a pipe
///
///
/// this is the first version of the ciet heater
/// without any insert within the heater
/// the heater behaves like a pipe
///
/// inclined at 90 degrees bottom to top
/// or 90 degrees + 180 top to bottom orientation
///
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_heated_section_version_1_label_1_without_inner_annular_pipe(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(6.60e-3);
    let pipe_length = Length::new::<meter>(1.6383);
    let flow_area = Area::new::<square_meter>(3.64e-4);
    let incline_angle = Angle::new::<degree>(90.0 + 180.0);
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.001905);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 15 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 15-2
    let user_specified_inner_nodes = 15-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// heater bottom head 1b within CIET's heater branch
///
/// heater top head
/// diagram label 1b
///
/// inclined at 90 degrees bottom to top
/// or 90 degrees + 180 top to bottom orientation
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_heater_bottom_head_1b(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(6.60e-3);
    let pipe_length = Length::new::<meter>(0.19685);
    let flow_area = Area::new::<square_meter>(3.64e-4);
    let incline_angle = Angle::new::<degree>(90.0 + 180.0);
    let form_loss = Ratio::new::<ratio>(3.95);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.001905);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 2-2
    let user_specified_inner_nodes = 2-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// pipe 18 within CIET's heater branch
/// pipe 18 on the diagram in Nico Zweibaum nodalisation
///
/// and from a top to bottom direction from pipe 5
/// to pipe 17, the incline angle is also
/// -40.00520 +180.0 degrees
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_18(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.1778);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(-40.00520 + 180.0);
    let form_loss = Ratio::new::<ratio>(5.15);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 2-2
    let user_specified_inner_nodes = 2-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}


/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// pipe 26 in DHX Branch from Top to Bottom orientation
///
/// pipe 26 on the diagram in Nico Zweibaum nodalisation
///
/// and from a top to bottom direction from pipe 5
/// to pipe 17, the incline angle is also
/// -40.00520 +180.0 degrees
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_26(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.2159);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(52.571994 + 180.0);
    let form_loss = Ratio::new::<ratio>(1.75);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 2-2
    let user_specified_inner_nodes = 2-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// Static mixer pipe 25a adjacent to MX-21
/// in DHX branch
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_25a(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.22245);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(1.35);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0381);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    let user_specified_inner_nodes = 0; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// static mixer 21 (MX-21) on CIET diagram
/// in the DHX branch in primary loop
/// just before the DRACS heat exchanger
/// from top to bottom
/// label 25
///
/// in reality flow goes from bottom to
/// top in natural convection
/// also in the DRACS
/// loop there are flow diodes to make
/// it such that flow going from bottom to top
/// encounters more resistance
///
/// forced convection flow direction is same as top to bottom
/// has a fldk of 21+4000/Re
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_static_mixer_21_label_25(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(0.33);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(21.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(4000_f64);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, we subtract 2
    let user_specified_inner_nodes = 2-2; 

    // insulation 
    let insulation_thickness = Length::new::<meter>(0.0381);
    let insulation_material = SolidMaterial::Fiberglass;


    let insulated_component = InsulatedFluidComponent::
        new_custom_component(
            initial_temperature, 
            ambient_temperature, 
            fluid_pressure, 
            solid_pressure, 
            flow_area, 
            incline_angle, 
            form_loss, 
            reynolds_coefficient, 
            reynolds_power, 
            shell_id, 
            shell_od, 
            insulation_thickness, 
            component_length, 
            hydraulic_diameter, 
            pipe_shell_material, 
            insulation_material, 
            pipe_fluid, 
            htc_to_ambient, 
            user_specified_inner_nodes);

    insulated_component

}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// this is the heat exchanger
/// in the DHX branch, labelled 24
///
/// It is shell side heat exchanger which allows
/// for heat to be transferred to natural circulation loop
/// or DRACS Loop
/// inclined at 90 degrees bottom to top
/// or 90 degrees + 180 top to bottom orientation
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_inactive_dhx_shell_side_heat_exchanger(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(5.65e-3);
    let pipe_length = Length::new::<meter>(1.18745);
    let flow_area = Area::new::<square_meter>(9.43e-4);
    let incline_angle = Angle::new::<degree>(90.0 + 180.0);
    let form_loss = Ratio::new::<ratio>(23.9);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0016);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 11 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 11-2
    let user_specified_inner_nodes = 11-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);
    insulated_component
}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// static mixer pipe 23a in DHX branch in CIET
///
/// otherwise known as the static mixer pipe 
/// to MX-20
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_23a(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.0891);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(1.35);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0381);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is zero
    let user_specified_inner_nodes = 0; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// static mixer 20 (MX-20) on CIET diagram
/// in the DRACS branch in primary loop
/// just after the DRACS heat exchanger
/// from top to bottom
/// label 23
///
/// in reality flow goes from bottom to
/// top in natural convection
/// also in the DRACS
/// loop there are flow diodes to make
/// it such that flow going from bottom to top
/// encounters more resistance
///
/// original angle is is 90 degrees
/// but i orientate from top to bottom
///
/// forced convection flow direction is same as top to bottom
/// has a fldk of 21+4000/Re
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_static_mixer_20_label_23(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(0.33);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(21.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(4000_f64);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, we subtract 2
    let user_specified_inner_nodes = 2-2; 

    // insulation 
    let insulation_thickness = Length::new::<meter>(0.0381);
    let insulation_material = SolidMaterial::Fiberglass;


    let insulated_component = InsulatedFluidComponent::
        new_custom_component(
            initial_temperature, 
            ambient_temperature, 
            fluid_pressure, 
            solid_pressure, 
            flow_area, 
            incline_angle, 
            form_loss, 
            reynolds_coefficient, 
            reynolds_power, 
            shell_id, 
            shell_od, 
            insulation_thickness, 
            component_length, 
            hydraulic_diameter, 
            pipe_shell_material, 
            insulation_material, 
            pipe_fluid, 
            htc_to_ambient, 
            user_specified_inner_nodes);

    insulated_component

}
/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// pipe 22 within DHX branch in CIEt
///
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_22_sam_model(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.69215);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    // for RELAP model there may be a K value of 9.95 here. 
    // check again
    let form_loss = Ratio::new::<ratio>(45.95);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 6 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 6-2
    let user_specified_inner_nodes = 6-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}

/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// pipe 22 within DHX branch in CIEt
///
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_22_relap_model(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.69215);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    // for SAM model there may be a K value of 45.95 here. 
    // check again
    let form_loss = Ratio::new::<ratio>(9.95);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 6 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 6-2
    let user_specified_inner_nodes = 6-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}


/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// FM-20 DHX branch flow coriolis flowmeter 20
///
/// natural convection heat exchanger in primary loop
/// diagram label is 21a
///
/// we use the convention of top of bypass branch to bottom
/// hence degree is -90
///
/// However in DHX, i also expect there to be
/// a check valve which only allows flow from top to bottom
///
/// That is the forward direction of flow for FM20,
///
/// ctah line flowmeter 40
/// label 14a on simulation diagram
/// fldk = 18.0+93000/(Re^1.35)
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_flowmeter_20_label_21a(initial_temperature: ThermodynamicTemperature) -> NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let component_length = Length::new::<meter>(0.36);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(90.0 - 180.0);
    let form_loss = Ratio::new::<ratio>(18.0);
    let reynolds_power = -1.35_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(93000_f64);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, we subtract 2
    let user_specified_inner_nodes = 2-2; 



    let non_insulated_component = NonInsulatedFluidComponent::
        new_custom_component(
            initial_temperature, 
            ambient_temperature, 
            fluid_pressure, 
            solid_pressure, 
            flow_area, 
            incline_angle, 
            form_loss, 
            reynolds_coefficient, 
            reynolds_power, 
            shell_id, 
            shell_od, 
            component_length, 
            hydraulic_diameter, 
            pipe_shell_material, 
            pipe_fluid, 
            htc_to_ambient, 
            user_specified_inner_nodes);

    non_insulated_component

}


/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// pipe 21 within CIET DHX loop
///
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_21(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.487725);
    let flow_area = hydraulic_diameter * hydraulic_diameter * PI/4.0;
    let incline_angle = Angle::new::<degree>(90.0-180.0);
    let form_loss = Ratio::new::<ratio>(4.4);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 5 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 5-2
    let user_specified_inner_nodes = 5-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}


/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// pipe 20 within CIET DHX loop
///
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_20(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.33655);
    let flow_area = Area::new::<square_meter>(6.11e-4);
    let incline_angle = Angle::new::<degree>(0.0 - 180.0);
    let form_loss = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 3 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 3-2
    let user_specified_inner_nodes = 3-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}


/// creates a new component for CIET using the RELAP5-3D and SAM parameters 
///
/// pipe 19 within CIET DHX loop
///
///
/// Zou, Ling, Rui Hu, and Anne Charpentier. SAM code 
/// validation using the compact integral effects test (CIET) 
/// experimental data. No. ANL/NSE-19/11. Argonne National Lab.(ANL), 
///
///
/// Zweibaum, Nicolas. Experimental validation of passive safety 
/// system models: Application to design and optimization of 
/// fluoride-salt-cooled, high-temperature reactors. University of 
/// California, Berkeley, 2015.
/// Argonne, IL (United States), 2019.
pub fn new_pipe_19(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<meter>(2.79e-2);
    let pipe_length = Length::new::<meter>(0.219075);
    let flow_area = hydraulic_diameter * hydraulic_diameter * PI/4.0;
    let incline_angle = Angle::new::<degree>(-31.44898 - 180.0);
    let form_loss = Ratio::new::<ratio>(7.5);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, the 
    // number of inner nodes is 2-2
    let user_specified_inner_nodes = 2-2; 

    let insulated_component = InsulatedFluidComponent::new_insulated_pipe(
        initial_temperature, 
        ambient_temperature, 
        fluid_pressure, 
        solid_pressure, 
        flow_area, 
        incline_angle, 
        form_loss, 
        shell_id, 
        shell_od, 
        insulation_thickness, 
        pipe_length, 
        hydraulic_diameter, 
        pipe_shell_material, 
        insulation_material, 
        pipe_fluid, 
        htc_to_ambient, 
        user_specified_inner_nodes, 
        surface_roughness);

    insulated_component
}
