use uom::si::f64::*;

use crate::boussinesq_thermophysical_properties::Material;
use crate::single_control_vol::SingleCVNode;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;

use crate::thermal_hydraulics_error::ThermalHydraulicsLibError;

/// calculates the interaction between a heat addition BC and 
/// a control volume 
///
/// (single cv) ------------------ (heat addition bc)
///
/// the heat addition is at the front, the cv is at the back
#[inline]
pub fn calculate_constant_heat_addition_front_single_cv_back(
    control_vol: &mut SingleCVNode,
    heat_added_to_control_vol: Power,
    interaction: HeatTransferInteractionType
    ) -> Result<(), ThermalHydraulicsLibError> {

    // ensure that the interaction is UserSpecifiedHeatAddition
    // or advection
    // otherwise, return error 

    match interaction {

        HeatTransferInteractionType::UserSpecifiedHeatAddition => {
            // return a void value, that would be dropped 
            // instantly
            //
            // it pretty much has the same meaning as break
            ()
        },
        HeatTransferInteractionType::Advection(advection_data) => {
            control_vol.calculate_bc_front_cv_back_advection_non_set_temperature(
                advection_data)?;

            return Ok(());
            
        },


        _ => {
            println!("you need to specify that the interaction type \n 
            is UserSpecifiedHeatAddition");

            return Err(ThermalHydraulicsLibError::WrongHeatTransferInteractionType);
        },
    };


    control_vol.rate_enthalpy_change_vector.
        push(heat_added_to_control_vol);

    // auto time stepping doesn't work for constant heat flux 
    // or specified power as well. 
    // it is best to see at the end of all power calculations what 
    // is the temperature change
    //
    // For liquid CV, still need to calculate time scale based 
    // on convection flow
    // match statement is meant to tell that liquid CVs are not quite 
    // ready for use
    // Actually, for solid CV, I will also need to recalculate time scale 
    // based on the material thermal thermal_diffusivity
    


    let cv_material = control_vol.material_control_volume;
    match cv_material {
        Material::Solid(_) => {
            // in this case, we just have one cv and one bc 
            // so we only consider thermal inertia of this cv 

            control_vol.calculate_mesh_stability_conduction_timestep_for_single_node_and_bc(
                interaction)?;

            ()
        },
        Material::Liquid(_) => {
            // liquid time scales should be calculated using courant 
            // number at the end of each timestep after volumetric flows 
            // in and out of the cv are calculated
            ()
        },
    }

    return Ok(());
}

/// calculates the interaction between a heat addition BC and 
/// a control volume 
///
/// (heat addition) ------------------ (single cv)
///
/// the heat addition is at the front, the cv is at the back
#[inline]
pub fn calculate_single_cv_front_constant_heat_addition_back(
    heat_added_to_control_vol: Power,
    control_vol: &mut SingleCVNode,
    interaction: HeatTransferInteractionType
    ) -> Result<(), ThermalHydraulicsLibError> {

    // ensure that the interaction is UserSpecifiedHeatAddition
    // or advection
    // otherwise, return error 

    match interaction {

        HeatTransferInteractionType::UserSpecifiedHeatAddition => {
            // return a void value, that would be dropped 
            // instantly
            //
            // it pretty much has the same meaning as break
            ()
        },
        HeatTransferInteractionType::Advection(advection_data) => {

            control_vol.
                calculate_cv_front_bc_back_advection_non_set_temperature(
                advection_data)?;

            return Ok(());

        },

        _ => {
            println!("you need to specify that the interaction type \n 
            is UserSpecifiedHeatAddition");

            return Err(ThermalHydraulicsLibError::WrongHeatTransferInteractionType);
        },
    };


    control_vol.rate_enthalpy_change_vector.
        push(heat_added_to_control_vol);

    // auto time stepping doesn't work for constant heat flux 
    // or specified power as well. 
    // it is best to see at the end of all power calculations what 
    // is the temperature change
    //
    // For liquid CV, still need to calculate time scale based 
    // on convection flow
    // match statement is meant to tell that liquid CVs are not quite 
    // ready for use
    // Actually, for solid CV, I will also need to recalculate time scale 
    // based on the material thermal thermal_diffusivity
    


    let cv_material = control_vol.material_control_volume;
    match cv_material {
        Material::Solid(_) => {
            // in this case, we just have one cv and one bc 
            // so we only consider thermal inertia of this cv 
            control_vol.calculate_mesh_stability_conduction_timestep_for_single_node_and_bc(
                interaction)?;

            ()
        },
        Material::Liquid(_) => {
            // liquid time scales should be calculated using courant 
            // number at the end of each timestep after volumetric flows 
            // in and out of the cv are calculated
            ()
        },
    }

    return Ok(());
}


