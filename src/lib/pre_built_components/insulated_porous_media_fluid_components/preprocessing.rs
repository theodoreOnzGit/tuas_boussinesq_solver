use std::f64::consts::PI;
use std::thread::{self, JoinHandle};

use super::InsulatedPorousMediaFluidComponent;
use uom::si::length::meter;
use uom::ConstZero;
use uom::si::pressure::atmosphere;
use ndarray::*;

use uom::si::f64::*;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
use crate::heat_transfer_correlations::nusselt_number_correlations::input_structs::GnielinskiData;
use crate::heat_transfer_correlations::thermal_resistance::try_get_thermal_conductance_annular_cylinder;
use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::boundary_conditions::BCType;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;

impl InsulatedPorousMediaFluidComponent {


    /// the end of each node should have a zero power boundary condition 
    /// connected to each of them at the bare minimum
    ///
    /// this function does exactly that
    ///
    /// to connect the rest of the heat transfer entities, 
    /// use the link to front or back methods within the 
    /// FluidArray or SolidColumn
    #[inline]
    fn zero_power_bc_connection(&mut self){

        let zero_power: Power = Power::ZERO;

        let mut zero_power_bc: HeatTransferEntity = 
        BCType::UserSpecifiedHeatAddition(zero_power).into();

        // constant heat addition interaction 

        let interaction: HeatTransferInteractionType = 
        HeatTransferInteractionType::UserSpecifiedHeatAddition;

        // now connect the twisted tape 

        self.insulation_array.link_to_back(&mut zero_power_bc,
            interaction).unwrap();


        self.insulation_array.link_to_front(&mut zero_power_bc,
            interaction).unwrap();

        self.pipe_fluid_array.link_to_front(&mut zero_power_bc,
            interaction).unwrap();

        self.pipe_fluid_array.link_to_back(&mut zero_power_bc,
            interaction).unwrap();

        self.pipe_shell.link_to_front(&mut zero_power_bc,
            interaction).unwrap();

        self.pipe_shell.link_to_back(&mut zero_power_bc,
            interaction).unwrap();
    }
}


/// contains preprocessing calcs specifc to mx10 and static 
/// mixers
pub mod mx10;
