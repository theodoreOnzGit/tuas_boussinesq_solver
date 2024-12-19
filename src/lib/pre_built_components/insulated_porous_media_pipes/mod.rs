use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;

use super::heat_transfer_entities::HeatTransferEntity;
use uom::si::f64::*;
use uom::si::area::square_meter;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::length::meter;
use uom::si::ratio::ratio;
use uom::si::pressure::atmosphere;
/// Fluid Components with Internals 
/// 
/// This could be an insulated pipe with some twisted tape inside 
/// For example, a static mixer
/// 
/// StaticMixer MX-10 is a classic example of what this class is meant for 
/// 
/// However, it could also be used for CIET Heater v1.0 where it was insulated
/// and had an annular pipe inside it
/// 
#[derive(Debug,Clone,PartialEq)]
pub struct InsulatedPorousMediaFluidComponent {

    inner_nodes: usize,

    /// heat transfer entity representing control volumes 
    /// for the insulation around the Insulated Porous media component 
    /// such as MX-10
    pub insulation_array: HeatTransferEntity,

    /// heat transfer entity representing control volumes 
    /// for the steel piping in MX-10
    pub pipe_shell: HeatTransferEntity,

    /// heat transfer entity representing control volumes 
    /// for the therminol fluid in MX-10
    pub pipe_fluid_array: HeatTransferEntity,

    /// ambient temperature of air used to calculate heat loss
    pub ambient_temperature: ThermodynamicTemperature,

    /// heat transfer coefficient used to calculate heat loss 
    /// to air
    pub heat_transfer_to_air: HeatTransfer,

    tube_inner_diameter: Length,

    tube_outer_diameter: Length, 

    insulation_inner_diameter: Length,

    insulation_outer_diameter: Length,

    flow_area: Area,

    darcy_loss_correlation: DimensionlessDarcyLossCorrelations,
}

impl InsulatedPorousMediaFluidComponent {
    /// constructs the static mixer using the RELAP/SAM model 
    /// as a basis 
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
    /// Nusselt Number Correlation: same as heater (assumed)
    /// because there is quite a lot of mixing going on
    /// within the mixer
    ///
    /// Reynolds Number Correlation: 21 + 4000/Re
    ///
    ///
    /// Unheated Structure Thermal Inertia: ignored
    pub fn new_static_mixer_23_mx20(initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature) -> Self {

        let user_specified_inner_nodes: usize = 0;
        let flow_area = Area::new::<square_meter>(6.11e-4);
        let component_length = Length::new::<meter>(0.33);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);


        // heater is inclined 90 degrees upwards, not that this is 
        // particularly important for this scenario

        let pipe_incline_angle = Angle::new::<uom::si::angle::degree>(90.0-180.0);

        // theoretically it's 6 W/(m^2 K) but then we'll have to manually 
        // input wall structures for additional heat loss
        //
        let h_to_air: HeatTransfer = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(6.0);

        let fiberglass_thickness = Length::new::<meter>(0.0508);

        let steel_id = Length::new::<meter>(0.0381);
        let steel_od = Length::new::<meter>(0.04);
        let fiberglass_id = steel_od;
        let fiberglass_od = fiberglass_id + 
        fiberglass_thickness + fiberglass_thickness;

        // correlation 

        let correlation_constant_a = Ratio::new::<ratio>(21.0);
        let correlation_coeff_b = Ratio::new::<ratio>(4000.0);
        let reynolds_power_c: f64 = -1.0;



        // inner therminol array
        let therminol_array: FluidArray = 
        FluidArray::new_custom_component(
            component_length,
            hydraulic_diameter,
            flow_area,
            initial_temperature,
            atmospheric_pressure,
            LiquidMaterial::TherminolVP1,
            correlation_constant_a,
            correlation_coeff_b,
            reynolds_power_c,
            user_specified_inner_nodes,
            pipe_incline_angle
        );
        // now the outer steel array
        let steel_shell_array = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            steel_id,
            steel_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
        // insulation
        let insulation = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            fiberglass_id,
            fiberglass_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::Fiberglass,
            user_specified_inner_nodes 
        );

        // f + L/D K = 21 + 4000/Re
        let darcy_loss_correlation = 
        DimensionlessDarcyLossCorrelations::
            new_simple_reynolds_power_component(
                Ratio::new::<ratio>(21.0),
                Ratio::new::<ratio>(4000.0),
                -1.0
            );

        return Self { inner_nodes: user_specified_inner_nodes,
            insulation_array: insulation.into(),
            pipe_shell: steel_shell_array.into(),
            pipe_fluid_array: therminol_array.into(),
            ambient_temperature,
            heat_transfer_to_air: h_to_air,
            tube_inner_diameter: steel_id,
            tube_outer_diameter: steel_od,
            insulation_inner_diameter: fiberglass_id,
            insulation_outer_diameter: fiberglass_od,
            flow_area,
            darcy_loss_correlation,
        };
    }

    /// constructs the static mixer using the RELAP/SAM model 
    /// as a basis 
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
    /// Nusselt Number Correlation: same as heater (assumed)
    /// because there is quite a lot of mixing going on
    /// within the mixer
    ///
    /// Reynolds Number Correlation: 21 + 4000/Re
    ///
    ///
    /// Unheated Structure Thermal Inertia: ignored
    pub fn new_static_mixer_25_mx21(initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature) -> Self {

        let user_specified_inner_nodes: usize = 0;
        let flow_area = Area::new::<square_meter>(6.11e-4);
        let component_length = Length::new::<meter>(0.33);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);


        // heater is inclined 90 degrees upwards, not that this is 
        // particularly important for this scenario

        let pipe_incline_angle = Angle::new::<uom::si::angle::degree>(90.0-180.0);

        // theoretically it's 6 W/(m^2 K) but then we'll have to manually 
        // input wall structures for additional heat loss
        //
        let h_to_air: HeatTransfer = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(6.0);

        let fiberglass_thickness = Length::new::<meter>(0.0508);

        let steel_id = Length::new::<meter>(0.0381);
        let steel_od = Length::new::<meter>(0.04);
        let fiberglass_id = steel_od;
        let fiberglass_od = fiberglass_id + 
        fiberglass_thickness + fiberglass_thickness;

        // correlation 

        let correlation_constant_a = Ratio::new::<ratio>(21.0);
        let correlation_coeff_b = Ratio::new::<ratio>(4000.0);
        let reynolds_power_c: f64 = -1.0;



        // inner therminol array
        let therminol_array: FluidArray = 
        FluidArray::new_custom_component(
            component_length,
            hydraulic_diameter,
            flow_area,
            initial_temperature,
            atmospheric_pressure,
            LiquidMaterial::TherminolVP1,
            correlation_constant_a,
            correlation_coeff_b,
            reynolds_power_c,
            user_specified_inner_nodes,
            pipe_incline_angle
        );
        // now the outer steel array
        let steel_shell_array = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            steel_id,
            steel_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
        // insulation
        let insulation = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            fiberglass_id,
            fiberglass_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::Fiberglass,
            user_specified_inner_nodes 
        );

        // f + L/D K = 21 + 4000/Re
        let darcy_loss_correlation = 
        DimensionlessDarcyLossCorrelations::
            new_simple_reynolds_power_component(
                Ratio::new::<ratio>(21.0),
                Ratio::new::<ratio>(4000.0),
                -1.0
            );

        return Self { inner_nodes: user_specified_inner_nodes,
            insulation_array: insulation.into(),
            pipe_shell: steel_shell_array.into(),
            pipe_fluid_array: therminol_array.into(),
            ambient_temperature,
            heat_transfer_to_air: h_to_air,
            tube_inner_diameter: steel_id,
            tube_outer_diameter: steel_od,
            insulation_inner_diameter: fiberglass_id,
            insulation_outer_diameter: fiberglass_od,
            flow_area,
            darcy_loss_correlation,
        };
    }

    /// constructs the static mixer using the RELAP/SAM model 
    /// as a basis 
    ///
    /// length = 0.33 m 
    /// d_h = 2.79e-2
    /// Insulation thickness: 5.08 cm
    /// (fiberglass)
    /// number of nodes (including two ends): 2
    ///
    /// Nusselt Number Correlation: same as heater (assumed)
    /// because there is quite a lot of mixing going on
    /// within the mixer
    ///
    /// Reynolds Number Correlation: 21 + 4000/Re
    ///
    ///
    /// Unheated Structure Thermal Inertia: ignored
    pub fn new_static_mixer_2_mx10(initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature) -> Self {

        let user_specified_inner_nodes: usize = 0;
        let flow_area = Area::new::<square_meter>(6.11e-4);
        let component_length = Length::new::<meter>(0.33);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);


        // heater is inclined 90 degrees upwards, not that this is 
        // particularly important for this scenario

        let pipe_incline_angle = Angle::new::<uom::si::angle::degree>(90.0);

        // theoretically it's 6 W/(m^2 K) but then we'll have to manually 
        // input wall structures for additional heat loss
        //
        let h_to_air: HeatTransfer = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(6.0);

        let fiberglass_thickness = Length::new::<meter>(0.0508);

        let steel_id = Length::new::<meter>(0.0381);
        let steel_od = Length::new::<meter>(0.04);
        let fiberglass_id = steel_od;
        let fiberglass_od = fiberglass_id + 
        fiberglass_thickness + fiberglass_thickness;

        // correlation 

        let correlation_constant_a = Ratio::new::<ratio>(21.0);
        let correlation_coeff_b = Ratio::new::<ratio>(4000.0);
        let reynolds_power_c: f64 = -1.0;



        // inner therminol array
        let therminol_array: FluidArray = 
        FluidArray::new_custom_component(
            component_length,
            hydraulic_diameter,
            flow_area,
            initial_temperature,
            atmospheric_pressure,
            LiquidMaterial::TherminolVP1,
            correlation_constant_a,
            correlation_coeff_b,
            reynolds_power_c,
            user_specified_inner_nodes,
            pipe_incline_angle
        );
        // now the outer steel array
        let steel_shell_array = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            steel_id,
            steel_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
        // insulation
        let insulation = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            fiberglass_id,
            fiberglass_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::Fiberglass,
            user_specified_inner_nodes 
        );

        // f + L/D K = 21 + 4000/Re
        let darcy_loss_correlation = 
        DimensionlessDarcyLossCorrelations::
            new_simple_reynolds_power_component(
                Ratio::new::<ratio>(21.0),
                Ratio::new::<ratio>(4000.0),
                -1.0
            );

        return Self { inner_nodes: user_specified_inner_nodes,
            insulation_array: insulation.into(),
            pipe_shell: steel_shell_array.into(),
            pipe_fluid_array: therminol_array.into(),
            ambient_temperature,
            heat_transfer_to_air: h_to_air,
            tube_inner_diameter: steel_id,
            tube_outer_diameter: steel_od,
            insulation_inner_diameter: fiberglass_id,
            insulation_outer_diameter: fiberglass_od,
            flow_area,
            darcy_loss_correlation,
        };
    }

    /// constructs the static mixer pipe using the RELAP/SAM model 
    /// as a basis 
    ///
    /// length = 0.149425 m 
    /// d_h = 2.79e-2
    /// Insulation thickness: 5.08 cm
    /// (fiberglass)
    /// number of nodes (including two ends): 2
    ///
    /// form loss: 1.8
    ///
    /// Nusselt Number Correlation: same as heater (assumed)
    /// because there is quite a lot of mixing going on
    /// within the mixer
    ///
    ///
    ///
    /// Unheated Structure Thermal Inertia: ignored
    pub fn new_static_mixer_pipe_2a_mx10(initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature) -> Self {

        let user_specified_inner_nodes: usize = 0;
        let flow_area = Area::new::<square_meter>(6.11e-4);
        let component_length = Length::new::<meter>(0.149425);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);


        // heater is inclined 90 degrees upwards, not that this is 
        // particularly important for this scenario

        let pipe_incline_angle = Angle::new::<uom::si::angle::degree>(90.0);

        // theoretically it's 6 W/(m^2 K) but then we'll have to manually 
        // input wall structures for additional heat loss
        //
        let h_to_air: HeatTransfer = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(6.0);

        let fiberglass_thickness = Length::new::<meter>(0.0508);

        let steel_id = Length::new::<meter>(0.0381);
        let steel_od = Length::new::<meter>(0.04);
        let fiberglass_id = steel_od;
        let fiberglass_od = fiberglass_id + 
        fiberglass_thickness + fiberglass_thickness;

        // correlation 

        let form_loss = Ratio::new::<ratio>(1.8);



        // inner therminol array
        let therminol_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            component_length,
            hydraulic_diameter,
            flow_area,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            LiquidMaterial::TherminolVP1,
            form_loss,
            user_specified_inner_nodes,
            pipe_incline_angle
        );
        // now the outer steel array
        let steel_shell_array = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            steel_id,
            steel_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
        // insulation
        let insulation = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            fiberglass_id,
            fiberglass_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::Fiberglass,
            user_specified_inner_nodes 
        );

        // K = 1.8 in a pipe
        let darcy_loss_correlation = 
        DimensionlessDarcyLossCorrelations::
            new_pipe(
                component_length,
                SolidMaterial::SteelSS304L.surface_roughness().unwrap(),
                hydraulic_diameter,
                form_loss
            );

        return Self { inner_nodes: user_specified_inner_nodes,
            insulation_array: insulation.into(),
            pipe_shell: steel_shell_array.into(),
            pipe_fluid_array: therminol_array.into(),
            ambient_temperature,
            heat_transfer_to_air: h_to_air,
            tube_inner_diameter: steel_id,
            tube_outer_diameter: steel_od,
            insulation_inner_diameter: fiberglass_id,
            insulation_outer_diameter: fiberglass_od,
            flow_area,
            darcy_loss_correlation,
        };
    }

    /// constructs the static mixer pipe using the RELAP/SAM model 
    /// as a basis 
    ///
    /// Static mixer pipe 25a adjacent to MX-21
    /// in DHX branch
    /// pipe 25a
    /// otherwise known as the static mixer pipe 25a
    ///
    /// Nusselt Number Correlation: same as heater (assumed)
    /// because there is quite a lot of mixing going on
    /// within the mixer
    ///
    ///
    ///
    /// Unheated Structure Thermal Inertia: ignored
    pub fn new_static_mixer_pipe_25a_mx21(initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature) -> Self {

        let user_specified_inner_nodes: usize = 0;
        let flow_area = Area::new::<square_meter>(6.11e-4);
        let component_length = Length::new::<meter>(0.22245);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);


        // heater is inclined 90 degrees upwards, not that this is 
        // particularly important for this scenario

        let pipe_incline_angle = Angle::new::<uom::si::angle::degree>(90.0-180.0);

        // theoretically it's 6 W/(m^2 K) but then we'll have to manually 
        // input wall structures for additional heat loss
        //
        let h_to_air: HeatTransfer = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(6.0);

        let fiberglass_thickness = Length::new::<meter>(0.0508);

        let steel_id = Length::new::<meter>(0.0381);
        let steel_od = Length::new::<meter>(0.04);
        let fiberglass_id = steel_od;
        let fiberglass_od = fiberglass_id + 
        fiberglass_thickness + fiberglass_thickness;

        // correlation 

        let form_loss = Ratio::new::<ratio>(1.35);



        // inner therminol array
        let therminol_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            component_length,
            hydraulic_diameter,
            flow_area,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            LiquidMaterial::TherminolVP1,
            form_loss,
            user_specified_inner_nodes,
            pipe_incline_angle
        );
        // now the outer steel array
        let steel_shell_array = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            steel_id,
            steel_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
        // insulation
        let insulation = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            fiberglass_id,
            fiberglass_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::Fiberglass,
            user_specified_inner_nodes 
        );

        // K = 1.8 in a pipe
        let darcy_loss_correlation = 
        DimensionlessDarcyLossCorrelations::
            new_pipe(
                component_length,
                SolidMaterial::SteelSS304L.surface_roughness().unwrap(),
                hydraulic_diameter,
                form_loss
            );

        return Self { inner_nodes: user_specified_inner_nodes,
            insulation_array: insulation.into(),
            pipe_shell: steel_shell_array.into(),
            pipe_fluid_array: therminol_array.into(),
            ambient_temperature,
            heat_transfer_to_air: h_to_air,
            tube_inner_diameter: steel_id,
            tube_outer_diameter: steel_od,
            insulation_inner_diameter: fiberglass_id,
            insulation_outer_diameter: fiberglass_od,
            flow_area,
            darcy_loss_correlation,
        };
    }

    /// constructs the static mixer pipe using the RELAP/SAM model 
    /// as a basis 
    ///
    /// static mixer pipe 23a in DHX branch in CIET
    ///
    /// otherwise known as the static mixer pipe 
    /// to MX-20
    ///
    /// Nusselt Number Correlation: same as heater (assumed)
    /// because there is quite a lot of mixing going on
    /// within the mixer
    ///
    ///
    ///
    /// Unheated Structure Thermal Inertia: ignored
    pub fn new_static_mixer_pipe_23a_mx20(initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature) -> Self {

        let user_specified_inner_nodes: usize = 0;
        let flow_area = Area::new::<square_meter>(6.11e-4);
        let component_length = Length::new::<meter>(0.0891);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
        let hydraulic_diameter = Length::new::<meter>(2.79e-2);


        // heater is inclined 90 degrees upwards, not that this is 
        // particularly important for this scenario

        let pipe_incline_angle = Angle::new::<uom::si::angle::degree>(90.0-180.0);

        // theoretically it's 6 W/(m^2 K) but then we'll have to manually 
        // input wall structures for additional heat loss
        //
        let h_to_air: HeatTransfer = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(6.0);

        let fiberglass_thickness = Length::new::<meter>(0.0508);

        let steel_id = Length::new::<meter>(0.0381);
        let steel_od = Length::new::<meter>(0.04);
        let fiberglass_id = steel_od;
        let fiberglass_od = fiberglass_id + 
        fiberglass_thickness + fiberglass_thickness;

        // correlation 

        let form_loss = Ratio::new::<ratio>(1.35);



        // inner therminol array
        let therminol_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            component_length,
            hydraulic_diameter,
            flow_area,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            LiquidMaterial::TherminolVP1,
            form_loss,
            user_specified_inner_nodes,
            pipe_incline_angle
        );
        // now the outer steel array
        let steel_shell_array = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            steel_id,
            steel_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::SteelSS304L,
            user_specified_inner_nodes 
        );
        // insulation
        let insulation = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            fiberglass_id,
            fiberglass_od,
            initial_temperature,
            atmospheric_pressure,
            SolidMaterial::Fiberglass,
            user_specified_inner_nodes 
        );

        // K = 1.8 in a pipe
        let darcy_loss_correlation = 
        DimensionlessDarcyLossCorrelations::
            new_pipe(
                component_length,
                SolidMaterial::SteelSS304L.surface_roughness().unwrap(),
                hydraulic_diameter,
                form_loss
            );

        return Self { inner_nodes: user_specified_inner_nodes,
            insulation_array: insulation.into(),
            pipe_shell: steel_shell_array.into(),
            pipe_fluid_array: therminol_array.into(),
            ambient_temperature,
            heat_transfer_to_air: h_to_air,
            tube_inner_diameter: steel_id,
            tube_outer_diameter: steel_od,
            insulation_inner_diameter: fiberglass_id,
            insulation_outer_diameter: fiberglass_od,
            flow_area,
            darcy_loss_correlation,
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
