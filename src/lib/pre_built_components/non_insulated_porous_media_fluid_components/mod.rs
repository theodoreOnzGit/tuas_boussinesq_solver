
use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;

use super::heat_transfer_entities::HeatTransferEntity;
use uom::si::f64::*;
use uom::si::length::{inch, meter};
use uom::ConstZero;
use uom::si::area::square_meter;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::ratio::ratio;
use uom::si::pressure::atmosphere;
/// represents heater version 2 without insulation 
/// This is because during 2018-ish, the heater insulation 
/// got burnt off and a lot of frequency response tests were done 
/// with insulation removed
///
/// Heater version 2 bare has no insulation
/// but it has a twisted tape interior
///
///
/// note that it only contains the heated section, not the top nor 
/// bottom heads
///
/// note: the pressure drop correlations are not yet properly implemented 
/// so it behaves like a pipe in terms of pressure drop
/// For now, I did not do anything special with it
#[derive(Debug,Clone,PartialEq)]
pub struct NonInsulatedPorousMediaFluidComponent {

    inner_nodes: usize,


    /// heat transfer entity representing control volumes 
    /// for the twisted tape in the heated section of CIET's Heater
    pub twisted_tape_interior: HeatTransferEntity,

    /// heat transfer entity representing control volumes 
    /// for the steel piping in the heated section of CIET's Heater
    pub steel_shell: HeatTransferEntity,

    /// heat transfer entity representing control volumes 
    /// for the therminol fluid in the heated section of CIET's Heater
    pub therminol_array: HeatTransferEntity,

    /// ambient temperature of air used to calculate heat loss
    pub ambient_temperature: ThermodynamicTemperature,

    /// heat transfer coefficient used to calculate heat loss 
    /// to air
    pub heat_transfer_to_air: HeatTransfer,


}


impl NonInsulatedPorousMediaFluidComponent {

    /// traditional callibrated heater constructor 
    /// with 20 W/(m^2 K) of heat loss  to air
    ///
    /// uses RELAP and SAM model rather than DeWet's Transform 
    /// model as reference
    ///
    ///
    pub fn new_dewet_model(initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature,
        user_specified_inner_nodes: usize) -> Self {

        let flow_area = Area::new::<square_meter>(0.00105);
        let heated_length = Length::new::<meter>(1.6383);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
        let hydraulic_diameter = Length::new::<meter>(0.01467);

        // heater is inclined 90 degrees upwards, not that this is 
        // particularly important for this scenario

        let pipe_incline_angle = Angle::new::<uom::si::angle::degree>(90.0);

        // default is a 20 W/(m^2 K) callibrated heat transfer coeff 
        // theoretically it's 6 W/(m^2 K) but then we'll have to manually 
        // input wall structures for additional heat loss
        //
        let h_to_air: HeatTransfer = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
        let id = Length::new::<meter>(0.0381);
        let od = Length::new::<meter>(0.04);


        // inner therminol array 
        //
        // the darcy loss correlation is f = 17.9 *Re^{-0.34}
        // accurate to within 4% (Lukas et al)
        // Improved Heat Transfer and Volume Scaling through 
        // Novel Heater Design
        // 

        let a = Ratio::ZERO;
        let b = Ratio::new::<ratio>(17.9);
        let c: f64  = -0.34;
        let therminol_array: FluidArray = 
        FluidArray::new_custom_component(
            heated_length,
            hydraulic_diameter,
            flow_area,
            initial_temperature,
            atmospheric_pressure,
            LiquidMaterial::TherminolVP1,
            a,
            b,
            c,
            user_specified_inner_nodes,
            pipe_incline_angle
        );
        // now the outer steel array
        let steel_shell_array = 
        SolidColumn::new_cylindrical_shell(
            heated_length,
            id,
            od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
        // now twisted_tape 
        let twisted_tape_width: Length = Length::new::<inch>(1.0);
        let twisted_tape_thickness = Length::new::<inch>(0.048);
        let twisted_tape_height = heated_length;

        let twisted_tape = 
        SolidColumn::new_block(
            twisted_tape_height,
            twisted_tape_thickness,
            twisted_tape_width,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );

        return Self { inner_nodes: user_specified_inner_nodes,
            twisted_tape_interior: twisted_tape.into(),
            steel_shell: steel_shell_array.into(),
            therminol_array: therminol_array.into(),
            ambient_temperature,
            heat_transfer_to_air: h_to_air,
        };
    }
    /// traditional uncallibrated heater constructor 
    /// with 6 W/(m^2 K) of heat loss  to air
    ///
    /// uses RELAP and SAM model rather than DeWet's Transform 
    /// model as reference
    ///
    /// 6 W/(m^2 K) is the heat transfer coefficeint assuming natural 
    /// convection only 
    ///
    /// it was increased to 20 W/(m^2 K) because of the support structures 
    /// and other such losses
    pub fn _new_six_watts_per_m2_kelvin_model(initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature,
        user_specified_inner_nodes: usize) -> Self {

        let flow_area = Area::new::<square_meter>(0.00105);
        let heated_length = Length::new::<meter>(1.6383);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
        let dummy_pipe_form_loss = Ratio::new::<ratio>(0.1);
        let hydraulic_diameter = Length::new::<meter>(0.01467);

        // heater is inclined 90 degrees upwards, not that this is 
        // particularly important for this scenario

        let pipe_incline_angle = Angle::new::<uom::si::angle::degree>(90.0);

        // default is a 20 W/(m^2 K) callibrated heat transfer coeff 
        // theoretically it's 6 W/(m^2 K) but then we'll have to manually 
        // input wall structures for additional heat loss
        //
        let h_to_air: HeatTransfer = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(6.0);
        let id = Length::new::<meter>(0.0381);
        let od = Length::new::<meter>(0.04);


        // inner therminol array
        let therminol_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            heated_length,
            hydraulic_diameter,
            flow_area,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            LiquidMaterial::TherminolVP1,
            dummy_pipe_form_loss,
            user_specified_inner_nodes,
            pipe_incline_angle
        );
        // now the outer steel array
        let steel_shell_array = 
        SolidColumn::new_cylindrical_shell(
            heated_length,
            id,
            od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
        // the twisted tape width is assumed to be the twisted 
        // tape diameter in De Wet's dissertation
        let twisted_tape_width: Length = Length::new::<inch>(1.0);
        let twisted_tape_thickness = Length::new::<inch>(0.048);
        let twisted_tape_height = heated_length;

        let twisted_tape = 
        SolidColumn::new_block(
            twisted_tape_height,
            twisted_tape_thickness,
            twisted_tape_width,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );

        return Self { inner_nodes: user_specified_inner_nodes,
            twisted_tape_interior: twisted_tape.into(),
            steel_shell: steel_shell_array.into(),
            therminol_array: therminol_array.into(),
            ambient_temperature,
            heat_transfer_to_air: h_to_air,
        };
    }

    /// user uncallibrated heater constructor 
    /// with 6 W/(m^2 K) of heat loss  to air
    ///
    /// uses RELAP and SAM model rather than DeWet's Transform 
    /// model as reference
    ///
    /// 6 W/(m^2 K) is the heat transfer coefficeint assuming natural 
    /// convection only 
    ///
    /// it was increased to 20 W/(m^2 K) because of the support structures 
    /// and other such losses
    pub fn _user_callibrated_htc_to_air_model(initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature,
        user_specified_inner_nodes: usize,
        h_to_air: HeatTransfer) -> Self {

        let flow_area = Area::new::<square_meter>(0.00105);
        let heated_length = Length::new::<meter>(1.6383);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
        let dummy_pipe_form_loss = Ratio::new::<ratio>(0.1);
        let hydraulic_diameter = Length::new::<meter>(0.01467);

        // heater is inclined 90 degrees upwards, not that this is 
        // particularly important for this scenario

        let pipe_incline_angle = Angle::new::<uom::si::angle::degree>(90.0);

        // default is a 20 W/(m^2 K) callibrated heat transfer coeff 
        // theoretically it's 6 W/(m^2 K) but then we'll have to manually 
        // input wall structures for additional heat loss
        //
        let id = Length::new::<meter>(0.0381);
        let od = Length::new::<meter>(0.04);


        // inner therminol array
        let therminol_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            heated_length,
            hydraulic_diameter,
            flow_area,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            LiquidMaterial::TherminolVP1,
            dummy_pipe_form_loss,
            user_specified_inner_nodes,
            pipe_incline_angle
        );
        // now the outer steel array
        let steel_shell_array = 
        SolidColumn::new_cylindrical_shell(
            heated_length,
            id,
            od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
        // the twisted tape width is assumed to be the twisted 
        // tape diameter in De Wet's dissertation
        let twisted_tape_width: Length = Length::new::<inch>(1.0);
        let twisted_tape_thickness = Length::new::<inch>(0.048);
        let twisted_tape_height = heated_length;

        let twisted_tape = 
        SolidColumn::new_block(
            twisted_tape_height,
            twisted_tape_thickness,
            twisted_tape_width,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );

        return Self { inner_nodes: user_specified_inner_nodes,
            twisted_tape_interior: twisted_tape.into(),
            steel_shell: steel_shell_array.into(),
            therminol_array: therminol_array.into(),
            ambient_temperature,
            heat_transfer_to_air: h_to_air,
        };
    }
}




/// contains method implementations for obtaining conductances 
/// between the different arrays, and also laterally coupling 
/// the arrays to one another using a radial thermal resistance
pub mod preprocessing;

/// contains method implementations for FluidComponentTrait
/// This means all the stuff about getting mass flowrate from pressure 
/// and vice versa
pub mod fluid_entity;


/// contains methods to help advance timesteps (ie update the 
/// state of the control volumes after each timestep)
pub mod calculation;

/// for postprocessing, one can obtain temperature profiles 
/// of the component using the postprocessing modules
pub mod postprocessing;


/// tests for all ciet's heaters in 
///
/// Ong, T. K. C. (2024). Digital Twins as Testbeds for 
/// Iterative Simulated Neutronics Feedback Controller Development 
/// (Doctoral dissertation, UC Berkeley).
///
/// The tuas_boussinesq_solver library was constructed with CIET 
/// in mind
///
/// This is the compact integral effects test from the UC Berkeley 
/// Thermal Hydraulics Lab
/// A Library which contains useful traits and methods for thermal 
/// hydraulics calculations.
///
///
/// This crate has heavy reliance on units of measure (uom) released under 
/// Apache 2.0 license. So you'll need to get used to unit safe calculations
/// with uom as well.
///
///
/// This library was initially developed for 
/// use in my PhD thesis under supervision 
/// of Professor Per F. Peterson. It a thermal hydraulics
/// library in Rust that is released under the GNU General Public License
/// v 3.0. This is partly due to the fact that some of the libraries 
/// inherit from GeN-Foam and OpenFOAM, both licensed under GNU General
/// Public License v3.0.
///
/// As such, the entire library is released under GNU GPL v3.0. It is a strong 
/// copyleft license which means you cannot use it in proprietary software.
///
///
/// License
///    This is a thermal hydraulics library written 
///    in rust meant to help with the
///    fluid mechanics and heat transfer aspects of the calculations
///    for the Compact Integral Effects Tests (CIET) and hopefully 
///    Gen IV Reactors such as the Fluoride Salt cooled High Temperature 
///    Reactor (FHR)
///     
///    Copyright (C) 2022-2023  Theodore Kay Chen Ong, Singapore Nuclear
///    Research and Safety Initiative, Per F. Peterson, University of 
///    California, Berkeley Thermal Hydraulics Laboratory
///
///    tuas_boussinesq_solver is free software; you can 
///    redistribute it and/or modify it
///    under the terms of the GNU General Public License as published by the
///    Free Software Foundation; either version 2 of the License, or (at your
///    option) any later version.
///
///    tuas_boussinesq_solver is distributed in the hope 
///    that it will be useful, but WITHOUT
///    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
///    FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
///    for more details.
///
///    This thermal hydraulics library 
///    contains some code copied from GeN-Foam, and OpenFOAM derivative.
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
///
/// Btw, I no affiliation with the Rust Foundation. 
pub mod tests;
