
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

/// 
pub mod fluid_component;
