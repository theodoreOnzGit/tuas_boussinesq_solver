
use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
//use std::f64::consts::PI;
//
//use uom::si::angle::degree;
//use uom::si::heat_transfer::watt_per_square_meter_kelvin;
//use uom::si::length::meter;
//use uom::si::pressure::atmosphere;
//use uom::si::ratio::ratio;
//use uom::si::thermodynamic_temperature::degree_celsius;
//use uom::ConstZero;
//
//use crate::heat_transfer_correlations::nusselt_number_correlations::input_structs::GnielinskiData;

use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;

//use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
//use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
//use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};

use uom::si::f64::*;
/// clamshell_radiative_heater for UW madison flibe loop 
/// NOTE: not done yet 
/// TODO: should complete calibration and validation in future
#[derive(Clone,Debug,PartialEq)]
pub struct ClamshellRadiativeHeater {

    
    inner_nodes: usize,

    /// this HeatTransferEntity represents the pipe shell which 
    /// contains the tube side fluid
    /// it is exposed to the radiative heater and some ambient air
    pub pipe_shell_array: HeatTransferEntity,


    /// this HeatTransferEntity represents the pipe fluid
    /// which is coupled to the pipe shell via a Nusselt Number based
    /// thermal resistance (usually Gnielinski correlation)
    pub pipe_fluid_array: HeatTransferEntity,

    /// this HeatTransferEntity represents the 
    /// fluid (air) between the radiative heating element and the pipe.
    /// That is, in the annulus
    /// it is coupled to the pipe shell and radiative heating element
    /// via a Nusselt Number based
    /// thermal resistance, this must be specified by the user 
    /// this is usually air
    pub annular_air_array: HeatTransferEntity,

    /// this HeatTransferEntity represents the 
    ///
    /// heating element which is radiatively coupled 
    /// to the pipe shell
    pub heating_element_shell: HeatTransferEntity,

    /// ambient temperature that the shell and tube heat 
    /// exchanger is exposed to.
    pub ambient_temperature: ThermodynamicTemperature,

    /// heat transfer coefficient to ambient
    /// This provides thermal resistance between the surface of 
    /// the shell and tube heat exchanger 
    pub heat_transfer_to_ambient: HeatTransfer,

    /// insulation array covering the 
    /// heating element
    pub insulation_array: HeatTransferEntity,

    /// this option allows the user to toggle on or off 
    /// annular airflow
    pub is_annular_airflow_on: bool,

    /// tube outer diameter 
    pub tube_od: Length,

    /// tube inner diameter 
    pub tube_id: Length,

    /// specifies an thickness for the insulation covering 
    /// the heating element
    pub insulation_thickness: Length,

    /// representative tube flow area on a per tube basis
    pub tube_flow_area: Area,

    /// loss correlation for tube
    pub tube_loss_correlation: DimensionlessDarcyLossCorrelations,

    /// loss correlation for annular region
    pub annular_air_loss_correlation: DimensionlessDarcyLossCorrelations,

    /// assuming the heating element 
    /// is circular, provide the internal diameter 
    pub heating_element_id: Length,

    /// assuming the heating element 
    /// is circular, provide the outer diameter 
    pub heating_element_od: Length,

    /// allows for a custom flow area for the annular region 
    /// shell side
    pub annular_region_flow_area: Area,


    /// annular air nusselt correlation to tubes
    pub annular_air_nusselt_correlation_to_tube: NusseltCorrelation,

    /// allows user to set custom nusselt correlation for heating 
    /// element to annular air
    pub heating_element_to_annular_air_nusselt_correlation: NusseltCorrelation,

    /// allows the user to set custom nusselt correlation 
    /// for tube 
    pub tube_side_nusselt_correlation: NusseltCorrelation,




}


/// stuff such as conductances are calculated here
pub mod preprocessing;

/// fluid component traits
/// and other things helping make fluid calculations easier
pub mod fluid_component;

///  timestep advancing and other such steps
pub mod calculation;

/// postprocessing, which also includes how much radiant heat exits 
/// the heater 
pub mod postprocessing;

/// convenient functions for type_conversion 
pub mod type_conversion;


/// there some unit tests I will need to conduct for the clamshell
/// radiative heater in order to test if the heater is functioning 
/// correctly. 
/// 
/// For a radial nodalisation of the heater
/// See diagram below:
/// |            |            |               |             |            |
/// |            |            |               |             |            |
/// |-tube fluid-|-inner tube-|- annular air -|-heater elem-|-insulation-| ambient
/// |            |            |               |             |            |
/// |            |            |               |             |            |
///
/// 
/// The convection bits themselves should work since they are essentially 
/// the same as the shell and tube heat exchanger. Provided I copied 
/// them correctly. The radiation bits are new and will need testing. 
///
/// I unit tested radiation heat transfer view factors to see that 
/// the coaxial cylinder view factors sum up to one. But the energy input 
/// into the heater should equal the energy loss through convection 
/// and radiation at steady state.
///
///
/// Basically, in the UW madison FLiBe loop, heaters are 1.7 kW each of power.
/// Which means the heating element should have a 1.7 kW input.
///
/// At steady state, the heating element should lose 
///
/// (1) heat through radiant heat to the inner tube 
/// (2) heat through radiant heat to the exterior 
/// (3) heat through radiant heat to the axial annular openings between 
/// the inner tube and heating element 
/// (4) heat loss to insulation.
///
/// At steady state, heat loss from heating element to 
/// the insulation should be the same as heat loss from insulation 
/// to the environment.
///
/// So a steady state energy balance can be performed over both 
/// heating element and insulaiton together. If the total heat loss 
/// is the same as total heat input, then the test passes.
///
///
/// Likewise, the tube fluid and inner tube also need to have 
/// proper energy balance. The net heat received from the radiant heater 
/// must be equal to:
///
/// (1) convective heat loss through annular air
/// (2) radiative heat loss to annular axis
/// (3) convective heat loss through tube fluid
///
///
/// In all these, axial radiation and conduction is neglected
///
///
///
///
pub mod tests;

