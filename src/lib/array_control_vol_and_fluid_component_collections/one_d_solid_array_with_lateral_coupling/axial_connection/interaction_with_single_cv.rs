use crate::single_control_vol::SingleCVNode;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::tuas_lib_error::TuasLibError;
use uom::si::f64::*;

// todo: need to make an error if advection is given
impl SolidColumn {

    /// attaches a single cv to the front,entrance,
    /// lower or inner side of the 
    /// array cv
    /// 
    ///
    /// basically in whatever coordinate system, it is the lowest 
    /// value 
    ///
    /// for spheres, the lowest r (inner side)
    /// for cylinders, the lowest r or z (inner or lower)
    /// for cartesian, the lowest x, y or z (back)
    ///
    /// We use this convention because heat can flow either way 
    /// and fluid can flow either way. 
    ///
    /// So it's better to define higher and lower based upon a coordinate 
    /// axis
    pub fn link_single_cv_to_lower_side(&mut self,
        single_cv_node_other: &mut SingleCVNode,
        interaction: HeatTransferInteractionType) -> Result<(), TuasLibError>{

        // check if interaction is advection
        
        if let HeatTransferInteractionType::Advection(_) = interaction {
            println!("You cannot have advection interactions for Solid Columns");
            return Err(TuasLibError::WrongHeatTransferInteractionType);
        }

        // we need to obtain the single cv from the array cv first 
        // and this will be the back cv or inner cv 
        //

        let single_cv_node_self: &mut SingleCVNode = &mut self.back_single_cv;

        // now link both cvs or calculate between them

        SingleCVNode::calculate_between_two_singular_cv_nodes(
            single_cv_node_other,
            single_cv_node_self,
            interaction)
    }

    /// attaches a single cv to the exit,back,
    /// higher or outer side of the 
    /// array cv
    /// 
    ///
    /// basically in whatever coordinate system, it is the lowest 
    /// value 
    ///
    /// for spheres, the highest r (outer side)
    /// for cylinders, the highest r or z (outer or higher)
    /// for cartesian, the highest x, y or z (front)
    ///
    /// We use this convention because heat can flow either way 
    /// and fluid can flow either way. 
    ///
    /// So it's better to define higher and lower based upon a coordinate 
    /// axis
    pub fn link_single_cv_to_higher_side(&mut self,
        single_cv_node_other: &mut SingleCVNode,
        interaction: HeatTransferInteractionType) -> Result<(), TuasLibError>{
        // check if interaction is advection
        
        if let HeatTransferInteractionType::Advection(_) = interaction {
            println!("You cannot have advection interactions for Solid Columns");
            return Err(TuasLibError::WrongHeatTransferInteractionType);
        }

        // we need to obtain the single cv from the array cv first 
        // and this will be the front cv or outer cv 
        //

        let single_cv_node_self: &mut SingleCVNode = &mut self.front_single_cv;

        // now link both cvs or calculate between them

        SingleCVNode::calculate_between_two_singular_cv_nodes(
            single_cv_node_other,
            single_cv_node_self,
            interaction)
    }
    /// calculates timestep for a single cv attached to the front of the 
    /// array cv
    /// (back --- cv_self --- front) ---- (single cv)
    pub fn calculate_timestep_for_single_cv_to_front_of_array_cv(
        &mut self,
        single_cv_node_other: &mut SingleCVNode,
        interaction: HeatTransferInteractionType) -> Result<Time,TuasLibError> {
        // check if interaction is advection
        
        if let HeatTransferInteractionType::Advection(_) = interaction {
            println!("You cannot have advection interactions for Solid Columns");
            return Err(TuasLibError::WrongHeatTransferInteractionType);
        }

        // we need to obtain the single cv from the array cv first 
        // and this will be the front cv or outer cv 
        //

        let single_cv_node_self: &mut SingleCVNode = 
        &mut self.front_single_cv;

        single_cv_node_self.calculate_mesh_stability_timestep_for_two_single_cv_nodes(
            single_cv_node_other,
            interaction)

    }

    /// calculates timestep for a single cv attached to the back of the 
    /// array cv
    /// (single cv) --- (back --- cv_self --- front)
    pub fn calculate_timestep_for_single_cv_to_back_of_array_cv(
        &mut self,
        single_cv_node_other: &mut SingleCVNode,
        interaction: HeatTransferInteractionType) -> Result<Time,TuasLibError> {
        // check if interaction is advection
        
        if let HeatTransferInteractionType::Advection(_) = interaction {
            println!("You cannot have advection interactions for Solid Columns");
            return Err(TuasLibError::WrongHeatTransferInteractionType);
        }

        // we need to obtain the single cv from the array cv first 
        // and this will be the back cv or inner cv 
        //

        let single_cv_node_self: &mut SingleCVNode = 
        &mut self.back_single_cv;

        single_cv_node_self.calculate_mesh_stability_timestep_for_two_single_cv_nodes(
            single_cv_node_other,
            interaction)
    }
}
