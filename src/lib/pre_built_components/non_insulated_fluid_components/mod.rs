use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;

use super::heat_transfer_entities::cv_types::CVType;
use super::heat_transfer_entities::HeatTransferEntity;
use uom::si::f64::*;

/// The simplest component is a non insulated pipe
///
/// This is a simple pipe with a set hydraulic diameter and length
///
/// the standard assumption is that at each boundary of this pipe,
/// there is no conduction heat transfer in the axial direction
/// TODO: the nusselt number correlations for the shell and tube side 
/// are not yet capable/tested of handling nusselt number correlations other 
/// than Gnielinski type correlations
///
#[derive(Clone,Debug,PartialEq)]
pub struct NonInsulatedFluidComponent {

    inner_nodes: usize,

    /// this HeatTransferEntity represents the pipe shell which is 
    /// exposed to an ambient constant temperature boundary condition
    /// This is because constant heat flux BCs are not common for pipes
    ///
    /// only one radial layer of control volumes is used to simulate 
    /// the pipe shell
    pub pipe_shell: HeatTransferEntity,


    /// this HeatTransferEntity represents the pipe fluid
    /// which is coupled to the pipe shell via a Nusselt Number based
    /// thermal resistance (usually Gnielinski correlation)
    pub pipe_fluid_array: HeatTransferEntity,

    /// pipe ambient temperature
    pub ambient_temperature: ThermodynamicTemperature,

    /// pipe heat transfer coefficient to ambient
    pub heat_transfer_to_ambient: HeatTransfer,

    /// pipe  outer diameter 
    pub od: Length,

    /// pipe inner diameter 
    pub id: Length,

    /// flow area 
    pub flow_area: Area,

    /// loss correlation 
    pub custom_component_loss_correlation: DimensionlessDarcyLossCorrelations

}

impl NonInsulatedFluidComponent {

    /// constructs a new pipe
    ///
    /// you need to supply the initial temperature, ambient temperature
    /// as well as all the pipe parameters 
    ///
    /// such as:
    ///
    /// 1. flow area 
    /// 2. hydraulic diameter 
    /// 3. incline angle
    /// 4. any form losses beyond the Gnielinski correlation
    /// 5. inner diameter (id)
    /// 6. outer diameter (od)
    /// 7. pipe shell material 
    /// 8. pipe fluid 
    /// 9. fluid pressure (if in doubt, 1 atmosphere will do)
    /// 10. solid pressure (if in doubt, 1 atmosphere will do)
    /// 11. heat transfer coeffficient to ambient
    /// 12. how many inner axial nodes for both solid and fluid arrays
    ///
    /// The number of total axial nodes is the number of inner nodes plus 2
    ///
    /// this is because there are two nodes at the periphery of the pipe 
    /// and there
    ///
    /// at each timestep, you are allowed to set a heater power, where 
    /// heat is dumped into the heated tube surrounding the pipe
    ///
    /// so the pipe shell becomes the heating element so to speak
    pub fn new_bare_pipe(initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature,
        fluid_pressure: Pressure,
        solid_pressure: Pressure,
        flow_area: Area,
        incline_angle: Angle,
        form_loss: Ratio,
        id: Length,
        od: Length,
        pipe_length: Length,
        hydraulic_diameter: Length,
        surface_roughness: Length,
        pipe_shell_material: SolidMaterial,
        pipe_fluid: LiquidMaterial,
        htc_to_ambient: HeatTransfer,
        user_specified_inner_nodes: usize) -> NonInsulatedFluidComponent {

        // inner fluid_array
        let mut fluid_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            pipe_length,
            hydraulic_diameter,
            flow_area,
            initial_temperature,
            fluid_pressure,
            pipe_shell_material,
            pipe_fluid,
            form_loss,
            user_specified_inner_nodes,
            incline_angle
        );
        let custom_component_loss_correlation = DimensionlessDarcyLossCorrelations::
                new_pipe(pipe_length, 
                    surface_roughness, 
                    hydraulic_diameter, 
                    form_loss);

        fluid_array.fluid_component_loss_properties = custom_component_loss_correlation;

        // now the outer steel array
        let pipe_shell = 
        SolidColumn::new_cylindrical_shell(
            pipe_length,
            id,
            od,
            initial_temperature,
            solid_pressure,
            pipe_shell_material,
            user_specified_inner_nodes 
        );

        return Self { inner_nodes: user_specified_inner_nodes,
            pipe_shell: CVType::SolidArrayCV(pipe_shell).into(),
            pipe_fluid_array: CVType::FluidArrayCV(fluid_array).into(),
            ambient_temperature,
            heat_transfer_to_ambient: htc_to_ambient,
            od,
            id,
            flow_area,
            custom_component_loss_correlation,
        };
    }
    /// constructs a new insulated pipe
    ///
    /// you need to supply the initial temperature, ambient temperature
    /// as well as all the pipe parameters 
    ///
    /// The loss coefficient is calculated as:
    ///
    /// f_darcy = form_loss + b Re^(c)
    ///
    /// b is the reynolds_coefficient
    /// c is reynolds power
    pub fn new_custom_component(initial_temperature: ThermodynamicTemperature,
        ambient_temperature: ThermodynamicTemperature,
        fluid_pressure: Pressure,
        solid_pressure: Pressure,
        flow_area: Area,
        incline_angle: Angle,
        form_loss: Ratio,
        reynolds_coefficient: Ratio,
        reynolds_power: f64,
        shell_id: Length,
        shell_od: Length,
        component_length: Length,
        hydraulic_diameter: Length,
        pipe_shell_material: SolidMaterial,
        pipe_fluid: LiquidMaterial,
        htc_to_ambient: HeatTransfer,
        user_specified_inner_nodes: usize,) -> NonInsulatedFluidComponent {

        // inner fluid_array

        let a = form_loss;
        let b = reynolds_coefficient;
        let c = reynolds_power;

        let fluid_array: FluidArray = 
            FluidArray::new_custom_component(
                component_length, 
                hydraulic_diameter, 
                flow_area, 
                initial_temperature, 
                fluid_pressure, 
                pipe_fluid, 
                form_loss, 
                b, 
                c, 
                user_specified_inner_nodes, 
                incline_angle);

        // now the outer pipe array
        let pipe_shell = 
        SolidColumn::new_cylindrical_shell(
            component_length,
            shell_id,
            shell_od,
            initial_temperature,
            solid_pressure,
            pipe_shell_material,
            user_specified_inner_nodes 
        );


        // custom component loss correlation
        //


        let custom_component_loss_correlation = DimensionlessDarcyLossCorrelations::
            new_simple_reynolds_power_component(a, b, c);

        return Self { inner_nodes: user_specified_inner_nodes,
            pipe_shell: CVType::SolidArrayCV(pipe_shell).into(),
            pipe_fluid_array: CVType::FluidArrayCV(fluid_array).into(),
            ambient_temperature,
            heat_transfer_to_ambient: htc_to_ambient,
            od: shell_od,
            id: shell_id,
            flow_area,
            custom_component_loss_correlation,
        };
    }
}


/// stuff such as conductances are calculated here
pub mod preprocessing;

/// implementations for the FluidComponent trait
/// are done here
pub mod fluid_component;


/// stuff for calculation is done here, ie, advancing timestep
pub mod calculation;

/// postprocessing stuff, ie, get the temperature vectors 
/// of both arrays of control volumes 
pub mod postprocessing;

/// type conversion, such as into fluid component and such
pub mod type_conversion;


/// calibration, for calibrating thickness or nusselt correlation 
/// (incomplete)
pub mod calibration;
