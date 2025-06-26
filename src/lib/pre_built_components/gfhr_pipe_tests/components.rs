use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
use crate::heat_transfer_correlations::nusselt_number_correlations::input_structs::GnielinskiData;
use crate::pre_built_components::insulated_pipes_and_fluid_components::InsulatedFluidComponent;
use crate::pre_built_components::non_insulated_fluid_components::NonInsulatedFluidComponent;
use crate::pre_built_components::shell_and_tube_heat_exchanger::SimpleShellAndTubeHeatExchanger;
use crate::prelude::beta_testing::{FluidArray, HeatTransferEntity};
use crate::single_control_vol::SingleCVNode;
use uom::si::angle::degree;
use uom::si::area::{square_centimeter, square_inch};
use uom::si::f64::*;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::length::{centimeter, inch, meter, millimeter};
use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::pressure::atmosphere;


/// creates a new pipe 1 for the fhr simulator, this goes from bottom 
/// to top of the pebble bed
///
/// it has 5 nodes, and the middle node is used to cool the reactor
///
/// it is then joined to two mixing nodes at the top and bottom of the 
/// reactor
///
/// we make it roughly 
/// 310 cm in height 
/// 120 cm in radius
///
/// core barrel thickness is 2 cm 
/// vessel thickness is 4 cm
/// downcomer width is 5cm
///
/// expected mass flowrate of FLiBe is about 1173 kg/s for a 280 MWth reactor
/// 
/// https://kairospower.com/generic-fhr-core-model/
///
/// we can scale it down
/// forward flow direction going upwards 
pub fn new_reactor_vessel_pipe_1(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    // hydraulic diameter = 4 * void vol/surface area of pebbles
    // = 4 * void frac/(1-void frac)
    // = 2.67 cm
    let hydraulic_diameter = Length::new::<centimeter>(2.67);
    let pipe_length = Length::new::<meter>(3.10);
    // area of a 120 cm radius circle is about 11310 cm^2 
    // assume 60% filled by pebbles 
    // we get about 4523 cm2
    let flow_area = Area::new::<square_centimeter>(4500.0);
    let incline_angle = Angle::new::<degree>(90.0);
    // not putting in ergun equation yet
    let form_loss = Ratio::new::<ratio>(5.05);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    // we get reactor vessel around 120 cm in diameter
    let shell_id = Length::new::<centimeter>(120.0*2.0);
    // the pipe at this point just functions as thermal inertia 
    // it isn't meant to conduct heat to graphite and so on, even though it 
    // can. 
    // It is a quick an dirty way to gprogram this
    let pipe_thickness = Length::new::<centimeter>(5.0);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // insulate the pipe totally from environment
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(0.0);
    // we want 5 total nodes,
    // so two outer nodes on each end, plus 3 inner nodes
    let user_specified_inner_nodes = 3; 

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
/// creates a new pipe 2 for the fhr simulator, this goes from bottom 
/// to parallel to pebble bed
///
/// it is then joined to two mixing nodes at the top and bottom of the 
/// reactor
///
/// we make it roughly 
/// 310 cm in height 
/// 5 cm in radius
///
/// this is based on 
///
/// core barrel thickness of 2 cm 
/// vessel thickness of 4 cm
/// downcomer width of 5cm
///
/// expected mass flowrate of FLiBe is about 1173 kg/s for a 280 MWth reactor
/// 
/// https://kairospower.com/generic-fhr-core-model/
///
/// we can scale it down
/// forward flow direction going upwards 
pub fn new_downcomer_pipe_2(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let pipe_length = Length::new::<meter>(3.10);
    let hydraulic_diameter = Length::new::<inch>(14.0);
    let flow_area = Area::new::<square_inch>(154.0);
    let incline_angle = Angle::new::<degree>(90.0);
    // not putting in ergun equation yet
    let form_loss = Ratio::new::<ratio>(550.05);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    // the pipe at this point just functions as thermal inertia 
    // it isn't meant to conduct heat to graphite and so on, even though it 
    // can. 
    // It is a quick an dirty way to program this
    let pipe_thickness = Length::new::<centimeter>(4.0);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::FLiBe;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // we want 5 total nodes,
    // so two outer nodes on each end, plus 3 inner nodes
    let user_specified_inner_nodes = 3; 

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


/// creates a new pipe 3 for the fhr simulator, this goes from bottom 
/// to top of the pebble bed
///
/// pretty much identical to pipe 2
/// except it cools more efficiently to the environment 
/// just for effect
///
/// it is then joined to two mixing nodes at the top and bottom of the 
/// reactor
///
/// we make it roughly 
/// 310 cm in height 
/// 5 cm in radius
///
/// this is based on 
///
/// core barrel thickness of 2 cm 
/// vessel thickness of 4 cm
/// downcomer width of 5cm
///
/// expected mass flowrate of FLiBe is about 1173 kg/s for a 280 MWth reactor
/// 
/// https://kairospower.com/generic-fhr-core-model/
///
/// we can scale it down
///
/// forward flow direction going upwards 
pub fn new_downcomer_pipe_3(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let pipe_length = Length::new::<meter>(3.10);
    let hydraulic_diameter = Length::new::<inch>(14.0);
    let flow_area = Area::new::<square_inch>(154.0);
    let incline_angle = Angle::new::<degree>(90.0);
    // not putting in ergun equation yet
    let form_loss = Ratio::new::<ratio>(55.05);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    // the pipe at this point just functions as thermal inertia 
    // it isn't meant to conduct heat to graphite and so on, even though it 
    // can. 
    // It is a quick an dirty way to gprogram this
    let pipe_thickness = Length::new::<centimeter>(4.0);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // I just made this side more conductive to environment
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(200.0);
    // we want 5 total nodes,
    // so two outer nodes on each end, plus 3 inner nodes
    let user_specified_inner_nodes = 3; 

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


/// fhr pipe 11,
/// flow direction going downwards by 1m
/// note that for fhr pipes, the diamter is 14 inches 
///
/// from KP-FHR Mechanistic Source Term Methodology Topical Report, Revision 3
/// https://www.nrc.gov/docs/ML2208/ML22088A231.pdf
/// page 151 of 195
/// this makes for 14 inch diameter and 154 inch square flow area
pub fn new_fhr_pipe_11(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let pipe_length = Length::new::<meter>(1.0);
    let hydraulic_diameter = Length::new::<inch>(14.0);
    let flow_area = Area::new::<square_inch>(154.0);
    let incline_angle = Angle::new::<degree>(-90.0);
    // not putting in ergun equation yet
    let form_loss = Ratio::new::<ratio>(1.05);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    // the pipe at this point just functions as thermal inertia 
    // it isn't meant to conduct heat to graphite and so on, even though it 
    // can. 
    // It is a quick an dirty way to gprogram this
    let pipe_thickness = Length::new::<centimeter>(4.0);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // I just made this side more conductive to environment
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // 2 total nodes
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
/// fhr pipe 10,
/// flow direction going right by 10.0m - 0.36m (which is the pump length)
/// well after sorting stability, it became
/// flow direction going right by 10.0m - 2.0m (which is the pump length)
///
/// this is 9.64m
/// 10 total nodes
pub fn new_fhr_pipe_10(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let pipe_length = Length::new::<meter>(10.0 - 2.0);
    let hydraulic_diameter = Length::new::<inch>(14.0);
    let flow_area = Area::new::<square_inch>(154.0);
    let incline_angle = Angle::new::<degree>(0.0);
    // not putting in ergun equation yet
    let form_loss = Ratio::new::<ratio>(1.05);
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<centimeter>(4.0);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // I just made this side more conductive to environment
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // we want 10 total nodes,
    // so two outer nodes on each end, plus 8 inner nodes
    // one node per meter 
    let user_specified_inner_nodes = 6; 

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

/// creates a new pump component for the primary loop
/// (was originally 0.36m, but i think there was numerical instability 
/// at least for the FLiBe pump as temperatures fluctuated too much,
/// As the Courant 
/// number was too high due to high flowrates).
///
/// Hence, it is now 2.0m
pub fn new_fhr_pri_loop_pump_9(initial_temperature: ThermodynamicTemperature) -> NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let component_length = Length::new::<meter>(2.0);
    let hydraulic_diameter = Length::new::<inch>(14.0);
    let flow_area = Area::new::<square_inch>(154.0);
    let incline_angle = Angle::new::<degree>(0.0);
    let form_loss = Ratio::new::<ratio>(0.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::FLiBe;
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

/// fhr pipe 8,
/// flow direction going up by 1m
pub fn new_fhr_pipe_8(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let pipe_length = Length::new::<meter>(1.0);
    let hydraulic_diameter = Length::new::<inch>(14.0);
    let flow_area = Area::new::<square_inch>(154.0);
    let incline_angle = Angle::new::<degree>(90.0);
    // not putting in ergun equation yet
    let form_loss = Ratio::new::<ratio>(1.05);
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;

    let pipe_thickness = Length::new::<centimeter>(4.0);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // I just made this side more conductive to environment
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // 2 total nodes
    // that is two outer nodes plus 1.0
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

/// fhr pipe 7,
/// flow direction going up by 3.1m
pub fn new_fhr_pipe_7(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let pipe_length = Length::new::<meter>(3.1);
    let hydraulic_diameter = Length::new::<inch>(14.0);
    let flow_area = Area::new::<square_inch>(154.0);
    let incline_angle = Angle::new::<degree>(90.0);
    // not putting in ergun equation yet
    let form_loss = Ratio::new::<ratio>(1.05);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;
    // the pipe at this point just functions as thermal inertia 
    // it isn't meant to conduct heat to graphite and so on, even though it 
    // can. 
    // It is a quick an dirty way to gprogram this
    let pipe_thickness = Length::new::<centimeter>(4.0);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // I just made this side more conductive to environment
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // we want 5 total nodes,
    // so two outer nodes on each end, plus 3 inner nodes
    let user_specified_inner_nodes = 3; 

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



/// constructs a new instance of the shell and tube 
/// heat exchanger for the IHX 
/// FLiBe on shell side (prevents freezing)
/// and HITEC on tube side
///
/// flow direction going up by 1m on both shell and tube side
///
/// TODO: likely need to increase heat transfer efficiency here
pub fn new_ihx_sthe_6_version_1(initial_temperature: ThermodynamicTemperature
    ) -> SimpleShellAndTubeHeatExchanger {

    let insulation_thickness: Length = Length::new::<meter>(0.0508);
    let steel = SolidMaterial::SteelSS304L;
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let sthe_length = Length::new::<meter>(1.0);
    // tube side
    //
    // this is labelled 6 within the pri loop and intermediate loop diagram
    let number_of_tubes = 32;
    // just have 0 inner nodes
    let number_of_inner_nodes = 0;

    let tube_side_id = Length::new::<meter>(0.0635);
    let tube_side_od = Length::new::<meter>(0.0794);
    // wall thickness is 7.95e-4 meters 
    // this is (OD-ID)/2 which i verified in Libreoffice Calc 
    let flow_area = Area::new::<square_inch>(154.0);
    let tube_side_hydraulic_diameter = tube_side_id;
    let tube_side_flow_area_single_tube = 
        flow_area / 
        number_of_tubes as f64;

    let tube_side_form_loss = Ratio::new::<ratio>(3.3);
    let tube_side_incline_angle = Angle::new::<degree>(90.0);
    let tube_side_liquid = LiquidMaterial::HITEC;
    let inner_tube_material = steel;
    let tube_side_initial_temperature = initial_temperature;
    let tube_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            sthe_length, 
            SolidMaterial::SteelSS304L.surface_roughness().unwrap(), 
            tube_side_id, 
            tube_side_form_loss

        );
    // note that the dummy ratio for the gnielinski_data will be 
    // overwritten. So no need to have this.
    let dummy_ratio = Ratio::new::<ratio>(0.1);
    let tube_side_length_to_diameter: Ratio = 
        sthe_length/tube_side_hydraulic_diameter;
    let _tube_side_gnielinski_data: GnielinskiData = 
        GnielinskiData {
            reynolds: dummy_ratio,
            prandtl_bulk: dummy_ratio,
            prandtl_wall: dummy_ratio,
            darcy_friction_factor: dummy_ratio,
            length_to_diameter: tube_side_length_to_diameter,
        };
    let tube_side_nusselt_correlation = 
        NusseltCorrelation::IdealNusseltOneBillion;

    // shell side 
    //
    // labelled 6 within primary loop 
    // note: 
    let hydraulic_diameter = Length::new::<inch>(14.0);
    let flow_area = Area::new::<square_inch>(154.0);
    let shell_side_id = hydraulic_diameter;
    let shell_side_wall_thickness = Length::new::<meter>(0.0016);
    let shell_side_od = shell_side_id + 2.0 * shell_side_wall_thickness;
    let shell_side_hydraulic_diameter = hydraulic_diameter;
    let shell_side_flow_area = flow_area;
    let shell_side_form_loss = Ratio::new::<ratio>(3.9);
    let shell_side_incline_angle = Angle::new::<degree>(90.0);
    let shell_side_liquid = LiquidMaterial::FLiBe;
    let outer_tube_material = steel;
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
        NusseltCorrelation::IdealNusseltOneBillion;

    // insulation side, accounts for parasitic heat loss
    let insulation_material = SolidMaterial::SteelSS304L;
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(21.67);
    let heat_transfer_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // for heat losses, I use the same Gnielinksi correlation 
    // for estimation
    let shell_side_nusselt_correlation_to_outer_shell = 
        NusseltCorrelation::PipeGnielinskiGeneric(
            shell_side_gnielinski_data);


    

    let gfhr_ihx_sthe_ver_1 = 
        SimpleShellAndTubeHeatExchanger::new_custom_circular_single_pass_sthe_with_insulation(
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

    gfhr_ihx_sthe_ver_1
}



/// fhr pipe 5,
/// flow direction going left by 10m
pub fn new_fhr_pipe_5(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let pipe_length = Length::new::<meter>(10.0);
    let hydraulic_diameter = Length::new::<inch>(14.0);
    let flow_area = Area::new::<square_inch>(154.0);
    let incline_angle = Angle::new::<degree>(0.0);
    // not putting in ergun equation yet
    let form_loss = Ratio::new::<ratio>(1.05);
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;

    let pipe_thickness = Length::new::<centimeter>(4.0);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // I just made this side more conductive to environment
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // 2 total nodes
    // that is two outer nodes plus 1.0
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


/// fhr pipe 4,
/// flow direction going downwards by 1m
/// arbitrary parameters used for regression testing
pub fn new_fhr_pipe_4_ver_1(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let hydraulic_diameter = Length::new::<centimeter>(5.0);
    let pipe_length = Length::new::<meter>(1.0);
    let flow_area = Area::new::<square_centimeter>(19.0);
    let incline_angle = Angle::new::<degree>(-90.0);
    // not putting in ergun equation yet
    let form_loss = Ratio::new::<ratio>(1.05);
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;

    let pipe_thickness = Length::new::<centimeter>(4.0);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // I just made this side more conductive to environment
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // 2 total nodes
    // that is two outer nodes plus 1.0
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
/// fhr pipe 4,
/// flow direction going downwards by 1m
pub fn new_fhr_pipe_4_ver_2(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let pipe_length = Length::new::<meter>(1.0);
    let hydraulic_diameter = Length::new::<inch>(14.0);
    let flow_area = Area::new::<square_inch>(154.0);
    let incline_angle = Angle::new::<degree>(-90.0);
    // not putting in ergun equation yet
    let form_loss = Ratio::new::<ratio>(1.05);
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;

    let pipe_thickness = Length::new::<centimeter>(4.0);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::FLiBe;
    // I just made this side more conductive to environment
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // 2 total nodes
    // that is two outer nodes plus 1.0
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

/// fhr pipe 17,
/// flow direction going up by 4.1 m
/// this is in the intermediate loop
/// so it contains hitec
pub fn new_fhr_pipe_17(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let pipe_length = Length::new::<meter>(4.1);
    let hydraulic_diameter = Length::new::<inch>(14.0);
    let flow_area = Area::new::<square_inch>(154.0);
    let incline_angle = Angle::new::<degree>(90.0);
    // not putting in ergun equation yet
    let form_loss = Ratio::new::<ratio>(1.05);
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;

    let pipe_thickness = Length::new::<centimeter>(4.0);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::HITEC;
    // I just made this side more conductive to environment
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // 2 total nodes
    // that is two outer nodes plus 1.0
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

/// fhr pipe 12,
/// flow direction going right by 4m
/// this is in the intermediate loop
/// so it contains hitec
pub fn new_fhr_pipe_12(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let pipe_length = Length::new::<meter>(4.0);
    let hydraulic_diameter = Length::new::<inch>(14.0);
    let flow_area = Area::new::<square_inch>(154.0);
    let incline_angle = Angle::new::<degree>(0.0);
    // not putting in ergun equation yet
    let form_loss = Ratio::new::<ratio>(1.05);
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;

    let pipe_thickness = Length::new::<centimeter>(4.0);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::HITEC;
    // I just made this side more conductive to environment
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // 2 total nodes
    // that is two outer nodes plus 1.0
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



/// creates a new pump component for the intermediate loop
/// goes right 2.0m
///
/// (was originally 0.36m, but i think there was numerical instability 
/// at least for the FLiBe pump as temperatures fluctuated too much,
/// suspect it is the same for intermediate loop pump too. As the Courant 
/// number was too high due to high flowrates).
///
/// note that the reference point for the intermediate loop is 
/// between pipe 17 and pump 16
pub fn new_fhr_intermediate_loop_pump_16(initial_temperature: ThermodynamicTemperature) -> NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let component_length = Length::new::<meter>(2.00);
    let hydraulic_diameter = Length::new::<inch>(14.0);
    let flow_area = Area::new::<square_inch>(154.0);
    let incline_angle = Angle::new::<degree>(0.0);
    let form_loss = Ratio::new::<ratio>(0.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::HITEC;
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


/// fhr pipe 15,
/// flow direction going right by 4m (less the pump distance)
/// this is in the intermediate loop
/// so it contains hitec
pub fn new_fhr_pipe_15(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let pipe_length = Length::new::<meter>(2.0);
    let hydraulic_diameter = Length::new::<inch>(14.0);
    let flow_area = Area::new::<square_inch>(154.0);
    let incline_angle = Angle::new::<degree>(0.0);
    // not putting in ergun equation yet
    let form_loss = Ratio::new::<ratio>(1.05);
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;

    let pipe_thickness = Length::new::<centimeter>(4.0);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::HITEC;
    // I just made this side more conductive to environment
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // 2 total nodes
    // that is two outer nodes plus 1.0
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


/// creates a new steam_generator_shell_side component for the intermediate loop
///
/// it will be externally coupled to a steam loop solver where 
/// heat loss is manually computed
/// goes up by 1.0 m
pub fn new_fhr_intermediate_loop_steam_generator_shell_side_14(initial_temperature: ThermodynamicTemperature) -> NonInsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let component_length = Length::new::<meter>(1.0);
    let hydraulic_diameter = Length::new::<inch>(14.0);
    let flow_area = Area::new::<square_inch>(154.0);
    let incline_angle = Angle::new::<degree>(90.0);
    let form_loss = Ratio::new::<ratio>(0.0);
    let reynolds_power = -1_f64;
    let reynolds_coefficient = Ratio::new::<ratio>(0.0);
    //estimated component wall roughness (doesn't matter here,
    //but i need to fill in)
    let shell_id = hydraulic_diameter;
    let pipe_thickness = Length::new::<meter>(0.0027686);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::HITEC;
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(200.0);
    // from SAM nodalisation, we have 2 nodes only, 
    // now because there are two outer nodes, we subtract 2
    let user_specified_inner_nodes = 2-2; 



    let mut non_insulated_component = NonInsulatedFluidComponent::
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

    // I'm going to make the nusselt correlation in here ideal
    // as it should be for heat exchanger 
    // or at least 10000
    //
    // basically, im cloning the entire fluid array out,
    // changing the nusselt correlation,
    // and chugging it back into the non_insulated_component

    let ideal_nusselt = NusseltCorrelation::IdealNusseltOneBillion;
    let mut fluid_array_nusselt_adjust: FluidArray = 
        non_insulated_component
        .pipe_fluid_array
        .clone()
        .try_into()
        .unwrap();

    fluid_array_nusselt_adjust.nusselt_correlation = 
        ideal_nusselt;

    non_insulated_component.pipe_fluid_array = 
        fluid_array_nusselt_adjust.into();



    non_insulated_component

}


/// fhr pipe 13,
/// flow direction going up by 4.1m
/// this is in the intermediate loop
/// so it contains hitec
///
/// and it is angled 90 degrees upwards
pub fn new_fhr_pipe_13(initial_temperature: ThermodynamicTemperature) -> InsulatedFluidComponent {
    let ambient_temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0);
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);
    let pipe_length = Length::new::<meter>(4.1);
    let hydraulic_diameter = Length::new::<inch>(14.0);
    let flow_area = Area::new::<square_inch>(154.0);
    let incline_angle = Angle::new::<degree>(90.0);
    // not putting in ergun equation yet
    let form_loss = Ratio::new::<ratio>(1.05);
    let surface_roughness = Length::new::<millimeter>(0.015);
    let shell_id = hydraulic_diameter;

    let pipe_thickness = Length::new::<centimeter>(4.0);
    let shell_od = shell_id + 2.0 * pipe_thickness;
    let insulation_thickness = Length::new::<meter>(0.0508);
    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::SteelSS304L;
    let pipe_fluid = LiquidMaterial::HITEC;
    // I just made this side more conductive to environment
    let htc_to_ambient = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
    // 2 total nodes
    // that is two outer nodes plus 1.0
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

/// creates a mixing node for the bottom of the reactor (start) 
pub fn gfhr_bottom_mixing_node_pri_loop(initial_temperature: ThermodynamicTemperature)
    -> HeatTransferEntity {

        let mixing_node_diameter = Length::new::<inch>(14.0);
        let mixing_node_material = LiquidMaterial::FLiBe;
        let mixing_node_pressure = Pressure::new::<atmosphere>(1.0);
        let mixing_node = SingleCVNode::new_sphere(
            mixing_node_diameter, 
            mixing_node_material.into(), 
            initial_temperature, 
            mixing_node_pressure).
            unwrap();
        return mixing_node.into();
}
/// creates a mixing node for the top of the reactor (end)
pub fn gfhr_top_mixing_node_pri_loop(initial_temperature: ThermodynamicTemperature)
    -> HeatTransferEntity {

        let mixing_node_diameter = Length::new::<inch>(14.0);
        let mixing_node_material = LiquidMaterial::FLiBe;
        let mixing_node_pressure = Pressure::new::<atmosphere>(1.0);
        let mixing_node = SingleCVNode::new_sphere(
            mixing_node_diameter, 
            mixing_node_material.into(), 
            initial_temperature, 
            mixing_node_pressure).
            unwrap();
        return mixing_node.into();
}


/// creates a mixing node for the bottom of the intermediate loop (start) 
pub fn gfhr_bottom_mixing_node_intrmd_loop(initial_temperature: ThermodynamicTemperature)
    -> HeatTransferEntity {

        let mixing_node_diameter = Length::new::<inch>(14.0);
        let mixing_node_material = LiquidMaterial::HITEC;
        let mixing_node_pressure = Pressure::new::<atmosphere>(1.0);
        let mixing_node = SingleCVNode::new_sphere(
            mixing_node_diameter, 
            mixing_node_material.into(), 
            initial_temperature, 
            mixing_node_pressure).
            unwrap();
        return mixing_node.into();
}
/// creates a mixing node for the top of the intermediate loop (end)
pub fn gfhr_top_mixing_node_intrmd_loop(initial_temperature: ThermodynamicTemperature)
    -> HeatTransferEntity {

        let mixing_node_diameter = Length::new::<inch>(14.0);
        let mixing_node_material = LiquidMaterial::HITEC;
        let mixing_node_pressure = Pressure::new::<atmosphere>(1.0);
        let mixing_node = SingleCVNode::new_sphere(
            mixing_node_diameter, 
            mixing_node_material.into(), 
            initial_temperature, 
            mixing_node_pressure).
            unwrap();
        return mixing_node.into();
}
