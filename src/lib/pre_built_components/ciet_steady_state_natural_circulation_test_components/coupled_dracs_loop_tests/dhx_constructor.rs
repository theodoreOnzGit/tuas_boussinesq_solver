use uom::si::angle::degree;
use uom::si::area::square_meter;
use uom::si::f64::*;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::length::meter;
use uom::si::pressure::atmosphere;
use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::degree_celsius;

use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
use crate::heat_transfer_correlations::nusselt_number_correlations::input_structs::GnielinskiData;
use crate::pre_built_components::shell_and_tube_heat_exchanger::SimpleShellAndTubeHeatExchanger;
use crate::prelude::beta_testing::{LiquidMaterial, SolidMaterial};

/// constructs a new instance of the shell and tube 
/// heat exchanger for the DHX based on Zou's specifications
/// for the flow area, hydraulic diameter and number of tubes
/// but Zweibaum's specifications for insulation thickness 
/// 
///
/// the heat transfer coefficients are based on Gnielinski 
/// correlation 
///
/// Whereas hydrodynamically, the DHX shell and tube 
/// sides are modelled as pipes with K values of 23.9 on 
/// the tube side and 3.3 on the tube side 
/// insulation thickness for DHX is 0.0508 m of fiberglass
/// DHX is made of copper tubing on the inside
/// and assumed to be copper on shell side as well
pub fn new_dhx_sthe_version_1(initial_temperature: ThermodynamicTemperature
    ) -> SimpleShellAndTubeHeatExchanger {

    let insulation_thickness: Length = Length::new::<meter>(0.0508);
    let copper = SolidMaterial::Copper;
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let sthe_length = Length::new::<meter>(1.18745);
    // tube side
    //
    // this is labelled 30 and within DRACS loop
    let number_of_tubes = 19;
    //from Zou's publication, DHX tube side and shell side have 11 nodes 
    let number_of_inner_nodes = 11 - 2;

    let tube_side_id = Length::new::<meter>(0.00635);
    let tube_side_od = Length::new::<meter>(0.00794);
    // wall thickness is 7.95e-4 meters 
    // this is (OD-ID)/2 which i verified in Libreoffice Calc 
    let tube_side_hydraulic_diameter = tube_side_id;
    let tube_side_flow_area_single_tube = 
        Area::new::<square_meter>(6.0172e-4) / 
        number_of_tubes as f64;

    let tube_side_form_loss = Ratio::new::<ratio>(3.3);
    let tube_side_incline_angle = Angle::new::<degree>(-90.0);
    let tube_side_liquid = LiquidMaterial::TherminolVP1;
    let inner_tube_material = copper;
    let tube_side_initial_temperature = initial_temperature;
    let tube_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            sthe_length, 
            SolidMaterial::Copper.surface_roughness().unwrap(), 
            tube_side_id, 
            tube_side_form_loss

        );
    // note that the dummy ratio for the gnielinski_data will be 
    // overwritten. So no need to have this.
    let dummy_ratio = Ratio::new::<ratio>(0.1);
    let tube_side_length_to_diameter: Ratio = 
        sthe_length/tube_side_hydraulic_diameter;
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

    // shell side 
    //
    // this is labelled 24 and within Primary loop
    let shell_side_id = Length::new::<meter>(0.0508);
    let shell_side_wall_thickness = Length::new::<meter>(0.0016);
    let shell_side_od = shell_side_id + 2.0 * shell_side_wall_thickness;
    let shell_side_hydraulic_diameter = Length::new::<meter>(6.857144e-3);
    let shell_side_flow_area = Area::new::<square_meter>(1.086058e-3);
    let shell_side_form_loss = Ratio::new::<ratio>(23.9);
    let shell_side_incline_angle = Angle::new::<degree>(-90.0);
    let shell_side_liquid = LiquidMaterial::TherminolVP1;
    let outer_tube_material = copper;
    let shell_side_initial_temperature = initial_temperature;
    let shell_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            sthe_length, 
            SolidMaterial::SteelSS304L.surface_roughness().unwrap(), 
            shell_side_hydraulic_diameter, 
            shell_side_form_loss
        );

    let shell_side_length_to_diameter: Ratio = 
        sthe_length/shell_side_hydraulic_diameter;
    let shell_side_gnielinski_data: GnielinskiData = 
        GnielinskiData {
            reynolds: dummy_ratio,
            prandtl_bulk: dummy_ratio,
            prandtl_wall: dummy_ratio,
            darcy_friction_factor: dummy_ratio,
            length_to_diameter: shell_side_length_to_diameter,
        };
    let shell_side_nusselt_correlation_to_tubes = 
        NusseltCorrelation::PipeGnielinskiGeneric(
            shell_side_gnielinski_data);

    // insulation side, accounts for parasitic heat loss
    let insulation_material = SolidMaterial::Fiberglass;
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(21.67);
    let heat_transfer_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // for heat losses, I use the same Gnielinksi correlation 
    // for estimation
    let shell_side_nusselt_correlation_to_outer_shell = 
        NusseltCorrelation::PipeGnielinskiGeneric(
            shell_side_gnielinski_data);


    

    let dhx_sthe_ver_1 = SimpleShellAndTubeHeatExchanger::new_custom_circular_single_pass_sthe_with_insulation(
        number_of_tubes, 
        number_of_inner_nodes, 
        fluid_pressure,  // for the sake of fluid property calculations, not hydrostatic pressure
                         // and such
        solid_pressure, 
        tube_side_od, 
        tube_side_id, 
        tube_side_hydraulic_diameter, 
        tube_side_flow_area_single_tube, 
        shell_side_od, 
        shell_side_id, 
        shell_side_hydraulic_diameter, 
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
        shell_side_nusselt_correlation_to_outer_shell);

    dhx_sthe_ver_1
}


/// constructs a new instance of the shell and tube 
/// heat exchanger for the DHX based on Zou's specifications
/// for the flow area, hydraulic diameter and number of tubes
/// but Zweibaum's specifications for insulation thickness 
/// 
///
/// the heat transfer coefficients are based on Gnielinski 
/// correlation 
///
/// Whereas hydrodynamically, the DHX shell and tube 
/// sides are modelled as pipes with K values of 23.9 on 
/// the tube side and 3.3 on the tube side 
/// insulation thickness for DHX is 0.0508 m of fiberglass
/// DHX is made of copper tubing on the inside
/// and assumed to be copper on shell side as well
pub fn new_dhx_sthe_version_1_mesh_refined(initial_temperature: ThermodynamicTemperature
    ) -> SimpleShellAndTubeHeatExchanger {

    let insulation_thickness: Length = Length::new::<meter>(0.0508);
    let copper = SolidMaterial::Copper;
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let sthe_length = Length::new::<meter>(1.18745);
    // tube side
    //
    // this is labelled 30 and within DRACS loop
    let number_of_tubes = 19;
    //from Zou's publication, DHX tube side and shell side have 11 nodes 
    let number_of_inner_nodes = 19 - 2;

    let tube_side_id = Length::new::<meter>(0.00635);
    let tube_side_od = Length::new::<meter>(0.00794);
    // wall thickness is 7.95e-4 meters 
    // this is (OD-ID)/2 which i verified in Libreoffice Calc 
    let tube_side_hydraulic_diameter = tube_side_id;
    let tube_side_flow_area_single_tube = 
        Area::new::<square_meter>(6.0172e-4) / 
        number_of_tubes as f64;

    let tube_side_form_loss = Ratio::new::<ratio>(3.3);
    let tube_side_incline_angle = Angle::new::<degree>(-90.0);
    let tube_side_liquid = LiquidMaterial::TherminolVP1;
    let inner_tube_material = copper;
    let tube_side_initial_temperature = initial_temperature;
    let tube_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            sthe_length, 
            SolidMaterial::Copper.surface_roughness().unwrap(), 
            tube_side_id, 
            tube_side_form_loss

        );
    // note that the dummy ratio for the gnielinski_data will be 
    // overwritten. So no need to have this.
    let dummy_ratio = Ratio::new::<ratio>(0.1);
    let tube_side_length_to_diameter: Ratio = 
        sthe_length/tube_side_hydraulic_diameter;
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

    // shell side 
    //
    // this is labelled 24 and within Primary loop
    let shell_side_id = Length::new::<meter>(0.0508);
    let shell_side_wall_thickness = Length::new::<meter>(0.0016);
    let shell_side_od = shell_side_id + 2.0 * shell_side_wall_thickness;
    let shell_side_hydraulic_diameter = Length::new::<meter>(6.857144e-3);
    let shell_side_flow_area = Area::new::<square_meter>(1.086058e-3);
    let shell_side_form_loss = Ratio::new::<ratio>(23.9);
    let shell_side_incline_angle = Angle::new::<degree>(-90.0);
    let shell_side_liquid = LiquidMaterial::TherminolVP1;
    let outer_tube_material = copper;
    let shell_side_initial_temperature = initial_temperature;
    let shell_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            sthe_length, 
            SolidMaterial::SteelSS304L.surface_roughness().unwrap(), 
            shell_side_hydraulic_diameter, 
            shell_side_form_loss
        );

    let shell_side_length_to_diameter: Ratio = 
        sthe_length/shell_side_hydraulic_diameter;
    let shell_side_gnielinski_data: GnielinskiData = 
        GnielinskiData {
            reynolds: dummy_ratio,
            prandtl_bulk: dummy_ratio,
            prandtl_wall: dummy_ratio,
            darcy_friction_factor: dummy_ratio,
            length_to_diameter: shell_side_length_to_diameter,
        };
    let shell_side_nusselt_correlation_to_tubes = 
        NusseltCorrelation::PipeGnielinskiGeneric(
            shell_side_gnielinski_data);

    // insulation side, accounts for parasitic heat loss
    let insulation_material = SolidMaterial::Fiberglass;
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(21.67);
    let heat_transfer_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // for heat losses, I use the same Gnielinksi correlation 
    // for estimation
    let shell_side_nusselt_correlation_to_outer_shell = 
        NusseltCorrelation::PipeGnielinskiGeneric(
            shell_side_gnielinski_data);


    

    let dhx_sthe_ver_1 = SimpleShellAndTubeHeatExchanger::new_custom_circular_single_pass_sthe_with_insulation(
        number_of_tubes, 
        number_of_inner_nodes, 
        fluid_pressure,  // for the sake of fluid property calculations, not hydrostatic pressure
                         // and such
        solid_pressure, 
        tube_side_od, 
        tube_side_id, 
        tube_side_hydraulic_diameter, 
        tube_side_flow_area_single_tube, 
        shell_side_od, 
        shell_side_id, 
        shell_side_hydraulic_diameter, 
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
        shell_side_nusselt_correlation_to_outer_shell);

    dhx_sthe_ver_1
}
