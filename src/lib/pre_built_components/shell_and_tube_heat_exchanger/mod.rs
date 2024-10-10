// I'm taking data from:
// Du, Bao-Cun, et al. "Investigation on heat transfer 
// characteristics of molten salt in a shell-and-tube heat 
// exchanger." International Communications in Heat and 
// Mass Transfer 96 (2018): 61-68.
//
// to validate the models here. Hopefully it works!
//
// Basically, I'm not given data straight up, nor am I given the conditions 
// used to produce the data. However, I am given some data points of 
// the external tube nusselt number, which correspond to experimentally 
// determined nusselt numbers for the salt.
//
// I'm also given a Nusselt correlation for the shell side 
// (outside the tube) which contains the salt 
//
// Nu = 0.04318 * (Re^0.7797 - 280) Pr ^(0.4) ( 1 + (D_e/l)^(2/3) ) * (Pr_f/Pr_w)^0.25
//
// If i use this correlation for the outside of the tube, I should technically 
// be able to reproduce the results.
//
// The Re and Nu_shell numbers (Fig 9, obtained using graph reader) are:
//
// Re, Nu_shell
// 3510.033,42.582
// 3571.349,43.32
// 3691.75,43.852
// 3751.951,44.672
// 3794.314,44.795
// 3847.826,45.574
// 3959.309,47.09
// 4019.509,47.459
// 4267.001,53.238
// 4356.187,54.836
// 4550.167,58.238
// 4630.435,59.303
// 4730.769,60.451
// 4942.586,62.582
// 5230.212,63.77
// 5388.517,64.344
// 5481.048,65.861
//
// So basically, we need to reproduce this data. Use the correlations 
// and obtain the Nu_shell using some of the experiments. This is a 
// verification study.
//
//
// I'll still need to program in the Nusselt correlations, as well 
// as the as the thermophysical properties 
//
// Also, what do I use as the input parameters?
//
// I'm only given a flow rate of molten salt:
// 12.63 - 16.23 m3/h
//
// Flow rate of oil (almost constant):
// 15.48 - 15.79 m3/h
// 
// inlet temperature of molten salt (HITEC):
// 214.93 - 236.91 (deg C)
//
// outlet temperature of molten salt (HITEC):
// 204.06 - 225.35 (deg C)
//
// Inlet Temperature of oil:
// 74.49 - 90.41 (deg C)
//
// Outlet Temperature of oil:
// 88.24 - 110.74 (deg C) 
//
// And the Reynold's number on the shell side is meant to be:
// Re_s = rho_s u_s D_e / manual_tests
// 
// The superifical velocity is:
//
// u_s = q_(m,s) / (rho_s A_c)
//
// The cross sectional area is the total shell inner area
// minus the area of the area of the tubes:
//
// A_c = pi/4 * D_i^2 - N_t * pi/4 * d_o^2
//
// N_t = number of tubes
//
// The effective hydraulic diameter for the tube is:
//
// D_e = 4 * A_c / P_w
//
// where P_w  is the wetted perimeter
//
// The wetted (heated) perimeter is: 
// Pi * D_i + N_t * Pi * d_o
//
// Substitution results in a final expression:
//
// D_e = (D_i^2 - N_t d_o^2) / (D_i + N_t d_o)
//
// The Re for the oil is from 3514 - 5482, and Pr is from 19.82 to 24.03
//
// For programming, we need two sets of fluid entities within this 
// heat exchanger.
//
// For the reference simulation work, the tube is assumed to be well 
// insulated. (adiabatic BC)
//
// There are a few steps to complete to try and replicate the experiment
//


use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
use std::f64::consts::PI;

use uom::si::angle::degree;
//use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::length::meter;
use uom::si::pressure::atmosphere;
use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::ConstZero;

use crate::heat_transfer_correlations::nusselt_number_correlations::input_structs::GnielinskiData;

use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;

use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};

use uom::si::f64::*;


/// Single pass, no baffle, parrallel flow shell and tube 
/// heat exchanger 
///
/// Nusselt correlations can be customised to empirically fit other 
/// correlations
///
/// The axial sides are adiabatic unless otherwise stated
///
#[derive(Clone,Debug,PartialEq)]
pub struct SimpleShellAndTubeHeatExchanger {

    
    inner_nodes: usize,

    /// this HeatTransferEntity represents the pipe shell which 
    /// contains the tube side fluid
    /// it is exposed to the shell side fluid
    pub inner_pipe_shell_array_for_single_tube: HeatTransferEntity,


    /// this HeatTransferEntity represents the pipe fluid
    /// which is coupled to the pipe shell via a Nusselt Number based
    /// thermal resistance (usually Gnielinski correlation)
    pub tube_side_fluid_array_for_single_tube: HeatTransferEntity,

    /// this HeatTransferEntity represents the pipe fluid
    /// which is coupled to the pipe shell via a Nusselt Number based
    /// thermal resistance, this must be specified by the user
    pub shell_side_fluid_array: HeatTransferEntity,

    /// this HeatTransferEntity represents the pipe shell which 
    /// contains the shell side fluid and tube bundle
    /// it is exposed to the insulation, or ambient temperature 
    /// depending on whether the insulation is toggled on or off
    pub outer_shell: HeatTransferEntity,

    /// ambient temperature that the shell and tube heat 
    /// exchanger is exposed to
    pub ambient_temperature: ThermodynamicTemperature,

    /// heat transfer coefficient to ambient
    /// This provides thermal resistance between the surface of 
    /// the shell and tube heat exchanger 
    /// This could be the outer shell or insulation, depending on whether 
    /// insulation is toggled on or off
    /// 
    ///
    pub heat_transfer_to_ambient: HeatTransfer,

    /// insulation array covering the 
    /// outer_shell array if insulation is toggled on
    pub insulation_array: HeatTransferEntity,

    /// this option allows the user to toggle on or off insulation 
    pub heat_exchanger_has_insulation: bool,

    /// representative 
    /// tube outer diameter on a per tube bases
    pub tube_side_od: Length,

    /// representative 
    /// tube inner diameter one a per tube basis
    pub tube_side_id: Length,

    /// representative tube flow area on a per tube basis
    pub tube_side_flow_area: Area,

    /// loss correlation on a per tube basis
    pub tube_side_custom_component_loss_correlation: DimensionlessDarcyLossCorrelations,

    /// loss correlation for shell side
    pub shell_side_custom_component_loss_correlation: DimensionlessDarcyLossCorrelations,

    /// number of tubes in parallel 
    /// each pipe fluid array represents one tube only
    pub number_of_tubes: u32,

    /// assuming the outer shell is circular, provide the internal diameter 
    pub shell_side_id: Length,

    /// assuming the outer shell is circular, provide the outer diameter 
    pub shell_side_od: Length,

    /// allows for a custom flow area for the shell side
    pub shell_side_flow_area: Area,


    /// allows user to set custom nusselt correlation for shell side 
    /// fluid to tubes
    pub shell_side_nusselt_correlation_to_tubes: NusseltCorrelation,

    /// allows user to set custom nusselt correlation for shell side 
    /// fluid to shell
    pub shell_side_nusselt_correlation_parasitic: NusseltCorrelation,

    /// allows the user to set custom nusselt correlation 
    /// for tube side fluid to tube 
    pub tube_side_nusselt_correlation: NusseltCorrelation,

    /// specifies an thickness for the insulation covering 
    /// the shell side
    pub insulation_thickness: Length,


}


/// stuff such as conductances are calculated here
pub mod preprocessing;

/// implementations for the FluidComponent trait
/// are done here
///
/// unfortunately, we cannot treat this as a fluid component because 
/// this is not a simple pipe 
///
/// each fluid array must be treated as a fluid component in itself
pub mod fluid_component;


/// stuff for calculation is done here, ie, advancing timestep
pub mod calculation;

/// postprocessing stuff, ie, get the temperature vectors 
/// of both arrays of control volumes 
pub mod postprocessing;

/// type conversion, such as into fluid component and such
pub mod type_conversion;

/// functions to help calibrate the shell and tube heat exchanger 
pub mod calibration;


/// verification and validation tests for parallel tubing
/// as well as constructors
pub mod tests;


/// constructors here mostly
impl SimpleShellAndTubeHeatExchanger {

    /// heat exchanger constructor
    /// for a generic shell and tube heat exchanger 
    /// near atmospheric pressure
    ///
    /// hydraulic diameter and flow area on the shell side and 
    /// tube side is up to you to decide for the purposes of fluid 
    /// mechanics calculations.
    /// However, shell and tube is meant to be circular for the sake 
    /// of calculating thermal resistance/conductance through 
    /// shell and tube, so an inner and outer diameter needs to 
    /// be provided for both shell and tube
    ///
    /// for fluid calculations, you will also define 
    /// the shell side incline angle and tube side incline 
    /// angle separately (for maximum flexibility)
    ///
    /// sthe will have insulation
    ///
    #[inline]
    pub fn new_custom_circular_single_pass_sthe_with_insulation(
        number_of_tubes: u32,
        number_of_inner_nodes: usize,
        fluid_pressure: Pressure,
        solid_pressure: Pressure,
        tube_side_od: Length,
        tube_side_id: Length,
        tube_side_hydraulic_diameter: Length,
        tube_side_flow_area_single_tube: Area,
        shell_side_od: Length,
        shell_side_id: Length,
        shell_side_hydraulic_diameter: Length,
        shell_side_flow_area: Area,
        sthe_length: Length,
        tube_side_form_loss: Ratio,
        shell_side_form_loss: Ratio,
        insulation_thickness: Length,
        tube_side_incline_angle: Angle,
        shell_side_incline_angle: Angle,
        shell_side_liquid: LiquidMaterial,
        tube_side_liquid: LiquidMaterial,
        inner_tube_material: SolidMaterial,
        outer_tube_material: SolidMaterial,
        insulation_material: SolidMaterial,
        ambient_temperature: ThermodynamicTemperature,
        heat_transfer_to_ambient: HeatTransfer,
        tube_side_initial_temperature: ThermodynamicTemperature,
        shell_side_initial_temperature: ThermodynamicTemperature,
        shell_loss_correlations: DimensionlessDarcyLossCorrelations,
        tube_loss_correlations: DimensionlessDarcyLossCorrelations,
        tube_side_nusselt_correlation: NusseltCorrelation,
        shell_side_nusselt_correlation_to_tubes: NusseltCorrelation,
        shell_side_nusselt_correlation_to_outer_shell: NusseltCorrelation,
        ) -> SimpleShellAndTubeHeatExchanger {



        // inner fluid_array
        // the nusselt correlation here is a standard pipe correlation 
        // but I don't use that
        let tube_side_fluid_array: FluidArray = 
            FluidArray::new_odd_shaped_pipe(
                sthe_length,
                tube_side_hydraulic_diameter,
                tube_side_flow_area_single_tube,
                tube_side_initial_temperature,
                fluid_pressure,
                inner_tube_material, // meant for surface roughness calcs
                tube_side_liquid,
                tube_side_form_loss,
                number_of_inner_nodes,
                tube_side_incline_angle
            );
        let shell_side_fluid_array: FluidArray = 
            FluidArray::new_odd_shaped_pipe(
                sthe_length,
                shell_side_hydraulic_diameter,
                shell_side_flow_area,
                shell_side_initial_temperature,
                fluid_pressure,
                inner_tube_material, // meant for surface roughness calcs
                shell_side_liquid,
                shell_side_form_loss,
                number_of_inner_nodes,
                shell_side_incline_angle
            );
        let outer_shell: SolidColumn 
            = SolidColumn::new_cylindrical_shell(
                sthe_length, 
                shell_side_id, 
                shell_side_od, 
                shell_side_initial_temperature, 
                solid_pressure, 
                outer_tube_material, 
                number_of_inner_nodes
            );

        let inner_shell: SolidColumn 
            = SolidColumn::new_cylindrical_shell(
                sthe_length, 
                tube_side_id, 
                tube_side_od, 
                tube_side_initial_temperature, 
                solid_pressure, 
                inner_tube_material, 
                number_of_inner_nodes
            );

        let insulation_id = shell_side_od;
        let insulation_od = shell_side_od + 2.0*insulation_thickness;

        let insulation_array: SolidColumn 
            = SolidColumn::new_cylindrical_shell(
                sthe_length, 
                insulation_id, 
                insulation_od, 
                shell_side_initial_temperature, 
                solid_pressure, 
                insulation_material, 
                number_of_inner_nodes
            );

        let sthe
            = SimpleShellAndTubeHeatExchanger{ 
                inner_nodes: number_of_inner_nodes, 
                inner_pipe_shell_array_for_single_tube: inner_shell.into(), 
                tube_side_fluid_array_for_single_tube: tube_side_fluid_array.into(), 
                shell_side_fluid_array: shell_side_fluid_array.into(), 
                outer_shell: outer_shell.into(), 
                ambient_temperature: ambient_temperature.into(), 
                heat_transfer_to_ambient, 
                insulation_array: insulation_array.into(),
                heat_exchanger_has_insulation: true, 
                tube_side_od, 
                tube_side_id, 
                tube_side_flow_area: tube_side_flow_area_single_tube, 
                tube_side_custom_component_loss_correlation: tube_loss_correlations, 
                shell_side_custom_component_loss_correlation: shell_loss_correlations, 
                number_of_tubes, 
                shell_side_id, 
                shell_side_od, 
                shell_side_flow_area, 
                shell_side_nusselt_correlation_to_tubes: shell_side_nusselt_correlation_to_tubes.clone(), 
                shell_side_nusselt_correlation_parasitic: shell_side_nusselt_correlation_to_outer_shell, 
                tube_side_nusselt_correlation: tube_side_nusselt_correlation.clone(), 
                insulation_thickness,
            };

        sthe
    }
    /// heat exchanger constructor
    /// for a generic shell and tube heat exchanger 
    /// without baffles at near atmospheric pressure
    ///
    /// This is such that the hydraulic diameter on the 
    /// shell side is 
    /// D_e = (D_i^2 - N_t d_o^2)/(D_i + N_t d_o)
    ///
    /// N_t is number of tubes,
    /// D_i is shell side id 
    /// d_o is tube side od
    ///
    /// for fluid calculations, you will also define 
    /// the shell side incline angle and tube side incline 
    /// angle separately (for maximum flexibility)
    ///
    /// sthe will have insulation
    ///
    /// not properly tested and verified yet
    fn _new_sthe_with_insulation(
        number_of_tubes: u32,
        number_of_inner_nodes: usize,
        tube_side_od: Length,
        tube_side_id: Length,
        shell_side_od: Length,
        shell_side_id: Length,
        sthe_length: Length,
        tube_side_form_loss: Ratio,
        shell_side_form_loss: Ratio,
        insulation_thickness: Length,
        tube_side_incline_angle: Angle,
        shell_side_incline_angle: Angle,
        shell_side_liquid: LiquidMaterial,
        tube_side_liquid: LiquidMaterial,
        inner_tube_material: SolidMaterial,
        outer_tube_material: SolidMaterial,
        insulation_material: SolidMaterial,
        ambient_temperature: ThermodynamicTemperature,
        heat_transfer_to_ambient: HeatTransfer,
        tube_side_initial_temperature: ThermodynamicTemperature,
        shell_side_initial_temperature: ThermodynamicTemperature,
        shell_loss_correlations: DimensionlessDarcyLossCorrelations,
        tube_loss_correlations: DimensionlessDarcyLossCorrelations,
        tube_side_nusselt_correlation: NusseltCorrelation,
        shell_side_nusselt_correlation_to_tubes: NusseltCorrelation,
        shell_side_nusselt_correlation_to_outer_shell: NusseltCorrelation,
        ) -> SimpleShellAndTubeHeatExchanger {

        let fluid_pressure = Pressure::new::<atmosphere>(1.0);
        let solid_pressure = Pressure::new::<atmosphere>(1.0);
        let tube_side_flow_area_single_tube: Area 
            = PI * 0.25 * tube_side_id * tube_side_id;

        let shell_side_flow_area: Area 
            = PI * 0.25 * shell_side_id * shell_side_id 
            - number_of_tubes as f64 *
            PI * 0.25 * tube_side_od * tube_side_od;

        let shell_side_fluid_hydraulic_diameter: Length = 
            (shell_side_id * shell_side_id - number_of_tubes as f64 *
             tube_side_od * tube_side_od)/
            (shell_side_id + number_of_tubes as f64 * tube_side_od);

        let tube_side_hydraulic_diameter = tube_side_id;

        Self::new_custom_circular_single_pass_sthe_with_insulation(
            number_of_tubes, 
            number_of_inner_nodes, 
            fluid_pressure,
            solid_pressure,
            tube_side_od, tube_side_id, 
            tube_side_hydraulic_diameter, 
            tube_side_flow_area_single_tube, 
            shell_side_od, 
            shell_side_id, shell_side_fluid_hydraulic_diameter, 
            shell_side_flow_area, 
            sthe_length, 
            tube_side_form_loss, 
            shell_side_form_loss, 
            insulation_thickness, 
            tube_side_incline_angle, 
            shell_side_incline_angle, 
            shell_side_liquid, 
            tube_side_liquid, 
            inner_tube_material, 
            outer_tube_material, 
            insulation_material, 
            ambient_temperature, 
            heat_transfer_to_ambient, 
            tube_side_initial_temperature, 
            shell_side_initial_temperature, 
            shell_loss_correlations, 
            tube_loss_correlations, 
            tube_side_nusselt_correlation, 
            shell_side_nusselt_correlation_to_tubes, 
            shell_side_nusselt_correlation_to_outer_shell)


    }


    /// heat exchanger constructor
    /// using Du's paper 
    /// Du, B. C., He, Y. L., Qiu, Y., Liang, Q., & Zhou, Y. P. 
    /// (2018). Investigation on heat transfer characteristics of 
    /// molten salt in a shell-and-tube heat exchanger. International 
    /// Communications in Heat and Mass Transfer, 96, 61-68.
    pub fn new_du_et_al_sthe() -> SimpleShellAndTubeHeatExchanger {

        let number_of_tubes = 19_u32;
        let fluid_pressure = Pressure::new::<atmosphere>(1.0);
        let solid_pressure = Pressure::new::<atmosphere>(1.0);

        // from Du's heat exchanger type, except we use one inner tube
        let tube_side_od = Length::new::<meter>(0.014);
        let tube_side_id = Length::new::<meter>(0.01);
        let shell_side_od = Length::new::<meter>(0.108);
        let shell_side_id = Length::new::<meter>(0.1);
        let pipe_length = Length::new::<meter>(1.95);

        let tube_side_flow_area: Area 
            = PI * 0.25 * tube_side_id * tube_side_id;

        let shell_side_flow_area: Area 
            = PI * 0.25 * shell_side_id * shell_side_id 
            - number_of_tubes as f64 *
            PI * 0.25 * tube_side_od * tube_side_od;

        let shell_side_fluid_hydraulic_diameter: Length = 
            (shell_side_id * shell_side_id - number_of_tubes as f64 *
             tube_side_od * tube_side_od)/
            (shell_side_id + number_of_tubes as f64 * tube_side_od);

        let hitec: LiquidMaterial = LiquidMaterial::HITEC;
        let yd325: LiquidMaterial = LiquidMaterial::YD325;
        let steel: SolidMaterial = SolidMaterial::SteelSS304L;

        let form_loss = Ratio::new::<ratio>(0.0);

        let inlet_temp_salt = 
            ThermodynamicTemperature::new::<degree_celsius>(214.93);
        let inlet_temp_oil = 
            ThermodynamicTemperature::new::<degree_celsius>(74.49);
        // ambient temperature is 25C 
        let ambient_temperature = ThermodynamicTemperature::
            new::<degree_celsius>(25.0);

        // adiabatic, heat transfer for ambient is zero 
        let heat_transfer_to_ambient = HeatTransfer::ZERO;

        // perfomed mesh refinement up to 25 inner nodes, 
        // 12 is sufficient
        let number_of_inner_nodes = 12;

        let incline_angle = Angle::new::<degree>(0.0);

        // inner fluid_array
        // the nusselt correlation here is a standard pipe correlation 
        // but I don't use that
        let tube_side_fluid_array: FluidArray = 
            FluidArray::new_odd_shaped_pipe(
                pipe_length,
                tube_side_id,
                tube_side_flow_area,
                inlet_temp_oil,
                fluid_pressure,
                steel,
                yd325,
                form_loss,
                number_of_inner_nodes,
                incline_angle
            );

        // shell side fluid_array
        // the nusselt correlation here is a standard pipe correlation 
        // but I don't use that
        //
        // the reason is because for the shell side, the heat transfer 
        // to the outer shell and heat transfer to inner tubes will 
        // be different. 
        //
        // The fluid array, unfortunately, only has one nusselt correlation 
        // by default.
        let shell_side_fluid_array: FluidArray = 
            FluidArray::new_odd_shaped_pipe(
                pipe_length,
                shell_side_fluid_hydraulic_diameter,
                shell_side_flow_area,
                inlet_temp_salt,
                fluid_pressure,
                steel,
                hitec,
                form_loss,
                number_of_inner_nodes,
                incline_angle
            );

        let outer_shell: SolidColumn 
            = SolidColumn::new_cylindrical_shell(
                pipe_length, 
                shell_side_id, 
                shell_side_od, 
                inlet_temp_salt, 
                solid_pressure, 
                steel, 
                number_of_inner_nodes
            );

        let inner_shell: SolidColumn 
            = SolidColumn::new_cylindrical_shell(
                pipe_length, 
                tube_side_id, 
                tube_side_od, 
                inlet_temp_salt, 
                solid_pressure, 
                steel, 
                number_of_inner_nodes
            );

        // dummy insulation array, will not be used,
        // just clone the inner shell 
        let dummy_insulation_array 
            = inner_shell.clone();

        // loss correlations, use pipe by default 
        // but none are used in calculations depending on 
        // nusselt correlations
        let shell_loss_correlations: DimensionlessDarcyLossCorrelations
            = DimensionlessDarcyLossCorrelations::new_pipe(
                pipe_length, 
                SolidMaterial::SteelSS304L.surface_roughness().unwrap(), 
                shell_side_fluid_hydraulic_diameter, 
                form_loss
            );


        // for tube loss correlations, we need to use the 
        // darcy_friction_factor
        let tube_loss_correlations: DimensionlessDarcyLossCorrelations
            = DimensionlessDarcyLossCorrelations::new_pipe(
                pipe_length, 
                SolidMaterial::SteelSS304L.surface_roughness().unwrap(), 
                tube_side_id, 
                form_loss
            );

        // nusselt correlations, 4.36 by default

        let shell_side_length_to_diameter: Ratio = 
            pipe_length/shell_side_fluid_hydraulic_diameter;

        // note that prandtl, reynolds and darcy friction factor for shell 
        // side are all arbitrary, will get overwritten later
        let dummy_ratio = Ratio::new::<ratio>(0.1);
        let shell_side_gnielinski_data: GnielinskiData = 
            GnielinskiData {
                reynolds: dummy_ratio,
                prandtl_bulk: dummy_ratio,
                prandtl_wall: dummy_ratio,
                darcy_friction_factor: dummy_ratio,
                length_to_diameter: shell_side_length_to_diameter,
            };

        let c: Ratio = Ratio::new::<ratio>(0.04318);
        let m: f64 = 0.7797;
        let shell_side_nusselt_correlation_to_tubes = 
            NusseltCorrelation::CustomGnielinskiGenericPrandtlBulk(
                shell_side_gnielinski_data, c, m);

        let tube_side_length_to_diameter: Ratio = 
            pipe_length/tube_side_id;

        // note that prandtl, reynolds and darcy friction factor for shell 
        // side are all arbitrary, will get overwritten later
        let tube_side_gnielinski_data: GnielinskiData = 
            GnielinskiData {
                reynolds: dummy_ratio,
                prandtl_bulk: dummy_ratio,
                prandtl_wall: dummy_ratio,
                darcy_friction_factor: dummy_ratio,
                length_to_diameter: tube_side_length_to_diameter,
            };

        let tube_side_nusselt_correlation = 
            NusseltCorrelation::PipeGnielinskiGeneric(tube_side_gnielinski_data);

        let shell_side_nusselt_correlation_to_outer_shell = 
            NusseltCorrelation::FixedNusselt(Ratio::ZERO);

        // we are not going to use this anyway
        let dummy_insulation_thickness =
            Length::new::<meter>(1.0);


        let du_heat_exchanger 
            = SimpleShellAndTubeHeatExchanger{ 
                inner_nodes: number_of_inner_nodes, 
                inner_pipe_shell_array_for_single_tube: inner_shell.into(), 
                tube_side_fluid_array_for_single_tube: tube_side_fluid_array.into(), 
                shell_side_fluid_array: shell_side_fluid_array.into(), 
                outer_shell: outer_shell.into(), 
                ambient_temperature: ambient_temperature.into(), 
                heat_transfer_to_ambient, 
                insulation_array: dummy_insulation_array.into(), 
                heat_exchanger_has_insulation: false, 
                tube_side_od, 
                tube_side_id, 
                tube_side_flow_area, 
                tube_side_custom_component_loss_correlation: tube_loss_correlations, 
                shell_side_custom_component_loss_correlation: shell_loss_correlations, 
                number_of_tubes, 
                shell_side_id, 
                shell_side_od, 
                shell_side_flow_area, 
                shell_side_nusselt_correlation_to_tubes: shell_side_nusselt_correlation_to_tubes.clone(), 
                shell_side_nusselt_correlation_parasitic: shell_side_nusselt_correlation_to_outer_shell, 
                tube_side_nusselt_correlation: tube_side_nusselt_correlation.clone(), 
                insulation_thickness: dummy_insulation_thickness,
            };

        du_heat_exchanger
    }
}
