use crate::single_control_vol::SingleCVNode;
use crate::tuas_lib_error::TuasLibError;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::{DataAdvection, HeatTransferInteractionType};

/// which calls other functions depending on whether the 
/// heat transfer interaction is conductance based on advection based
#[inline]
pub fn calculate_between_two_singular_cv_nodes(
    single_cv_1: &mut SingleCVNode,
    single_cv_2: &mut SingleCVNode,
    interaction: HeatTransferInteractionType)-> Result<(), TuasLibError>{


    match interaction {
        HeatTransferInteractionType::UserSpecifiedThermalConductance(_) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::SingleCartesianThermalConductanceOneDimension(_, _) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::DualCartesianThermalConductanceThreeDimension(_) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::DualCartesianThermalConductance(_, _) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::DualCylindricalThermalConductance(_, _, _) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::CylindricalConductionConvectionLiquidOutside(_, _) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::CylindricalConductionConvectionLiquidInside(_, _) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::UserSpecifiedHeatAddition => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::UserSpecifiedHeatFluxCustomArea(_) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::UserSpecifiedHeatFluxCylindricalOuterArea(_, _) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::UserSpecifiedHeatFluxCylindricalInnerArea(_, _) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::UserSpecifiedConvectionResistance(_) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },

        HeatTransferInteractionType::Advection(advection_data) => {
            calculate_advection_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                advection_data)
        },
        HeatTransferInteractionType::SimpleRadiation
            (_area_coeff) => 
            {

                calculate_conductance_interaction_between_two_singular_cv_nodes(
                    single_cv_1,
                    single_cv_2,
                    interaction)
            }
        ,
    }

}

/// for advection flows between two SingleCVNode objects,
/// and specified advection information,
///
/// this updates the heat transfer vector in both singlecv nodes
#[inline]
pub fn calculate_advection_interaction_between_two_singular_cv_nodes(
    single_cv_1: &mut SingleCVNode,
    single_cv_2: &mut SingleCVNode,
    advection_data: DataAdvection)-> Result<(), TuasLibError>{

    single_cv_1.calculate_advection_interaction_to_front_singular_cv_node(
        single_cv_2, advection_data)
}

/// if two singleCV nodes have a conductance or thermal resistance between 
/// them, their temperature differentials and conductance are used to 
/// calculate the heat flow between them.
#[inline]
pub fn calculate_conductance_interaction_between_two_singular_cv_nodes(
    single_cv_1: &mut SingleCVNode,
    single_cv_2: &mut SingleCVNode,
    interaction: HeatTransferInteractionType)-> Result<(), TuasLibError>{

    single_cv_1.calculate_conductance_interaction_to_front_singular_cv_node(
        single_cv_2, interaction)
}
