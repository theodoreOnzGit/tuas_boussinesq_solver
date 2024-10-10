
use std::f64::consts::PI;

use uom::si::f64::*;
use crate::boundary_conditions::BCType;
use crate::boussinesq_thermophysical_properties::Material;
use crate::control_volume_dimensions::InnerDiameterThermalConduction;
use crate::control_volume_dimensions::OuterDiameterThermalConduction;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_geometry::CylindricalAndSphericalSolidFluidArrangement;
use crate::heat_transfer_correlations::heat_transfer_interactions::*;
use crate::single_control_vol::boundary_condition_interactions::constant_heat_addition_to_bcs::calculate_single_cv_front_constant_heat_addition_back;
use crate::single_control_vol::SingleCVNode;
use crate::thermal_hydraulics_error::ThermalHydraulicsLibError;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;

use self::single_cv_and_bc_interactions::calculate_bc_front_cv_back_advection_for_heat_flux_or_heat_addition;
use self::single_cv_and_bc_interactions::calculate_constant_heat_addition_front_single_cv_back;
use self::single_cv_and_bc_interactions::calculate_mesh_stability_conduction_timestep_for_single_node_and_bc;
use self::single_cv_and_bc_interactions::calculate_single_cv_front_heat_flux_back;
use self::single_cv_and_bc_interactions::calculate_single_cv_node_front_constant_temperature_back;
use self::single_cv_single_cv_interactions::calculate_between_two_singular_cv_nodes;

use super::cv_types::CVType;
use super::HeatTransferEntity;
///// contains scripts to select enums 
///// for how heat transfer entities 
///// ie control volumes and boundary conditions 
///// interact with each other
///// and also control volume mutation
/////
///// multithread calculation only
/////
///// note that the enum_selection_alpha module 
///// here are UNSTABLE, I may not preserve the API
/////
///// alpha here means unstable
////mod enum_selection_parallel_alpha;
////use enum_selection_parallel_alpha::*;

impl HeatTransferEntity {


    /// wrapper for linking, makes it easier to link 
    #[inline]
    pub fn link_to_front(&mut self,
    other_hte: &mut HeatTransferEntity,
    interaction: HeatTransferInteractionType) -> Result<(), ThermalHydraulicsLibError>{

        link_heat_transfer_entity(
            self,
            other_hte,
            interaction)
    }

    /// wrapper for linking, makes it easier to link 
    #[inline]
    pub fn link_to_back(&mut self,
    other_hte: &mut HeatTransferEntity,
    interaction: HeatTransferInteractionType) -> Result<(), ThermalHydraulicsLibError>{

        link_heat_transfer_entity(
            other_hte,
            self,
            interaction)
    }

    /// wrapper for linking, makes it easier to link 
    #[inline]
    pub fn link(entity: &mut HeatTransferEntity,
    other_hte: &mut HeatTransferEntity,
    interaction: HeatTransferInteractionType) -> Result<(), ThermalHydraulicsLibError>{

        link_heat_transfer_entity(
            entity,
            other_hte,
            interaction)

    }
}

/// For this part, we determine how heat_transfer_entities interact 
/// with each other by using a function 
/// The function will take in a HeatTransferInteractionType enum
/// which you must first initiate
///
/// Then you need to supply two control volumes or more generally 
/// heat_transfer_entities, which can consist of mix of control volumes
/// and boundary conditions
///
/// The function will then calculate the heat transfer between the two 
/// control volumes, and either return a value or mutate the CV objects 
/// using mutable borrows
#[inline]
pub fn link_heat_transfer_entity(entity_1: &mut HeatTransferEntity,
    entity_2: &mut HeatTransferEntity,
    interaction: HeatTransferInteractionType)-> Result<(), ThermalHydraulicsLibError>{

    // first thing first, probably want to unpack the enums to obtain 
    // the underlying control volume and BCs
    // 
    // Basically there are four permutations of what the user may choose 
    // either link two control volumes,
    // link a control vol and BC  (or BC first then control vol)
    // lastly, two BCs 
    // (which is kind of invalid though, 
    // but i suppose it may make sense for steady state)
    //

    let heat_transfer_entity_result = match (entity_1,entity_2) {
        (
            HeatTransferEntity::ControlVolume(cv_type_1),
            HeatTransferEntity::ControlVolume(cv_type_2)
        ) => {
            calculate_control_volume_serial(
                cv_type_1, 
                cv_type_2, 
                interaction)
        },
        (
            HeatTransferEntity::BoundaryConditions(bc_type),
            HeatTransferEntity::ControlVolume(cv_type)
        ) =>
        {
            attach_boundary_condition_to_control_volume_back_serial(
                bc_type, cv_type, interaction)
        },
        (
            HeatTransferEntity::ControlVolume(cv_type),
            HeatTransferEntity::BoundaryConditions(bc_type)
        ) =>
        {
            attach_boundary_condition_to_control_volume_front_serial(
                cv_type, bc_type, interaction)
        },
        (
            HeatTransferEntity::BoundaryConditions(_bc_type_1),
            HeatTransferEntity::BoundaryConditions(_bc_type_2)
        ) =>
        {
            return Err( 
                ThermalHydraulicsLibError::NotImplementedForBoundaryConditions
                ( "interactions between two boundary conditions \n not implemented".to_string()));
        },
    };


    return heat_transfer_entity_result;
   
}


// the job of this function is to take in a control volume 
// and then mutate it by calculating its interaction
#[inline]
pub (crate) fn calculate_control_volume_serial(
    control_vol_1: &mut CVType,
    control_vol_2: &mut CVType,
    interaction: HeatTransferInteractionType) -> Result<(),ThermalHydraulicsLibError>{

    // let me first match my control volumes to their various types
    // at each matching arm, use those as inputs

    let cv_result = match (control_vol_1, control_vol_2) {
        (
            CVType::SingleCV(single_cv_1), CVType::SingleCV(single_cv_2)
        ) =>
        {
            calculate_between_two_singular_cv_nodes(
                single_cv_1, 
                single_cv_2, 
                interaction)
        },
        // basically like this 
        //
        // (array_cv_back -----array_cv --- array_cv_front) --- (single_cv) 
        (CVType::SolidArrayCV(solid_array_cv), CVType::SingleCV(single_cv)) =>
        {
            solid_array_cv.link_single_cv_to_higher_side(
                single_cv,
                interaction)
        },
        // basically like this 
        //
        // (array_cv_back -----array_cv --- array_cv_front) --- (single_cv) 
        (CVType::FluidArrayCV(fluid_array_cv), CVType::SingleCV(single_cv)) =>
        {
            fluid_array_cv.link_single_cv_to_higher_side(
                single_cv,
                interaction)
        },
        // basically like this 
        //
        // (single_cv) --- (array_cv_back -----array_cv --- array_cv_front)
        (CVType::SingleCV(single_cv), CVType::SolidArrayCV(solid_array_cv)) =>
        {
            solid_array_cv.link_single_cv_to_lower_side(
                single_cv,
                interaction)
        },
        // basically like this 
        //
        // (single_cv) --- (array_cv_back -----array_cv --- array_cv_front)
        (CVType::SingleCV(single_cv), CVType::FluidArrayCV(fluid_array_cv)) =>
        {
            fluid_array_cv.link_single_cv_to_lower_side(
                single_cv,
                interaction)
        },
        // basically like this 
        //
        // (back --- cv_1 --- front) ---- (back --- cv_2 --- front)
        (CVType::SolidArrayCV(solid_array_cv_1), CVType::SolidArrayCV(solid_array_cv_2)) =>
        {
            solid_array_cv_1.link_solid_column_to_the_front_of_this_solid_column(
                solid_array_cv_2,
                interaction)
        },
        // basically like this 
        //
        // (back --- cv_1 --- front) ---- (back --- cv_2 --- front)
        (CVType::SolidArrayCV(solid_array_cv_1), CVType::FluidArrayCV(fluid_array_cv_2)) =>
        {
            solid_array_cv_1.link_fluid_array_to_the_front_of_this_solid_column(
                fluid_array_cv_2,
                interaction)
        },
        // basically like this 
        //
        // (back --- cv_1 --- front) ---- (back --- cv_2 --- front)
        (CVType::FluidArrayCV(fluid_array_cv_1), CVType::SolidArrayCV(solid_array_cv_2)) =>
        {
            fluid_array_cv_1.link_solid_column_to_the_front_of_this_fluid_array(
                solid_array_cv_2,
                interaction)
        },
        // basically like this 
        //
        // (back --- cv_1 --- front) ---- (back --- cv_2 --- front)
        (CVType::FluidArrayCV(fluid_array_cv_1), CVType::FluidArrayCV(fluid_array_cv_2)) =>
        {
            fluid_array_cv_1.link_fluid_array_to_the_front_of_this_fluid_array(
                fluid_array_cv_2,
                interaction)
        },
    };


    return cv_result;

}


/// this function calculates relevant timescales when linking 
/// two heat transfer entities
pub fn calculate_timescales_for_heat_transfer_entity(
    entity_1: &mut HeatTransferEntity,
    entity_2: &mut HeatTransferEntity,
    interaction: HeatTransferInteractionType)-> Result<Time, ThermalHydraulicsLibError>{

    // first thing first, probably want to unpack the enums to obtain 
    // the underlying control volume and BCs
    // 
    // Basically there are four permutations of what the user may choose 
    // either link two control volumes,
    // link a control vol and BC  (or BC first then control vol)
    // lastly, two BCs 
    // (which is kind of invalid though, 
    // but i suppose it may make sense for steady state)
    //

    let heat_transfer_entity_timestep_result = match (entity_1,entity_2) {
        (HeatTransferEntity::ControlVolume(cv_type_1),
         HeatTransferEntity::ControlVolume(cv_type_2)) =>
        {
            calculate_timestep_control_volume_serial(
                cv_type_1, 
                cv_type_2, 
                interaction)
        },
        (HeatTransferEntity::BoundaryConditions(bc_type),
         HeatTransferEntity::ControlVolume(cv_type)) =>
        {
            calculate_timestep_control_volume_back_to_boundary_condition_serial(
                bc_type, cv_type, interaction)
        },
        (HeatTransferEntity::ControlVolume(cv_type),
         HeatTransferEntity::BoundaryConditions(bc_type)) =>
        {
            calculate_timestep_control_volume_front_to_boundary_condition_serial(
                cv_type, bc_type, interaction)
        },
        (HeatTransferEntity::BoundaryConditions(_bc_type_1),
         HeatTransferEntity::BoundaryConditions(_bc_type_2)) =>
        {
            return Err( 
                ThermalHydraulicsLibError::NotImplementedForBoundaryConditions
                ( "interactions between two boundary conditions \n not implemented".to_string()));
        }
    };


    return heat_transfer_entity_timestep_result;
   
}

/// calculates timestep for control voluem and boundary condition 
/// in serial
/// for arrayCVs, the boundary condition is attached to the front 
/// of the control volume
///
/// (back --- cv_1 --- front) ---- (boundary condition)
pub (crate) fn calculate_timestep_control_volume_front_to_boundary_condition_serial(
    control_vol: &mut CVType,
    boundary_condition: &mut BCType,
    interaction: HeatTransferInteractionType) -> Result<Time,ThermalHydraulicsLibError> {


    let cv_bc_result = match (control_vol, boundary_condition) {
        (CVType::SingleCV(single_cv), BCType::UserSpecifiedHeatAddition(_heat_rate)) 
            => {
                calculate_mesh_stability_conduction_timestep_for_single_node_and_bc
                    (single_cv, interaction)
            }
        ,
        (CVType::SingleCV(single_cv), BCType::UserSpecifiedHeatFlux(_heat_flux))
            => {
                calculate_mesh_stability_conduction_timestep_for_single_node_and_bc
                    (single_cv, interaction)
            }
        ,
        (CVType::SingleCV(single_cv), BCType::UserSpecifiedTemperature(_bc_temperature))
            => {
                calculate_mesh_stability_conduction_timestep_for_single_node_and_bc
                    (single_cv, interaction)
            }
        ,
        (CVType::SolidArrayCV(solid_array_cv),BCType::UserSpecifiedHeatFlux(_heat_flux)) => {
            // match the cv to the cv type, extract the back boundary 
            // condition 
                calculate_mesh_stability_conduction_timestep_for_single_node_and_bc
                    (&mut solid_array_cv.front_single_cv, interaction)
        }
        ,
        (CVType::SolidArrayCV(solid_array_cv),BCType::UserSpecifiedHeatAddition(_heat_rate)) => {
            // match the cv to the cv type, extract the back boundary 
            // condition 
            calculate_mesh_stability_conduction_timestep_for_single_node_and_bc
                (&mut solid_array_cv.front_single_cv, interaction)
        }
        ,
        (CVType::SolidArrayCV(solid_array_cv),BCType::UserSpecifiedTemperature(_bc_temperature)) => {
            // match the cv to the cv type, extract the back boundary 
            // condition 
            calculate_mesh_stability_conduction_timestep_for_single_node_and_bc
                (&mut solid_array_cv.front_single_cv, interaction)
        }
        ,
        (CVType::FluidArrayCV(fluid_array_cv),BCType::UserSpecifiedHeatFlux(_heat_flux)) => {
            // match the cv to the cv type, extract the back boundary 
            // condition 
                calculate_mesh_stability_conduction_timestep_for_single_node_and_bc
                    (&mut fluid_array_cv.front_single_cv, interaction)
        }
        ,
        (CVType::FluidArrayCV(fluid_array_cv),BCType::UserSpecifiedHeatAddition(_heat_rate)) => {
            // match the cv to the cv type, extract the back boundary 
            // condition 
            calculate_mesh_stability_conduction_timestep_for_single_node_and_bc
                (&mut fluid_array_cv.front_single_cv, interaction)
        }
        ,
        (CVType::FluidArrayCV(fluid_array_cv),BCType::UserSpecifiedTemperature(_bc_temperature)) => {
            // match the cv to the cv type, extract the back boundary 
            // condition 
            calculate_mesh_stability_conduction_timestep_for_single_node_and_bc
                (&mut fluid_array_cv.front_single_cv, interaction)
        }
        ,
    };

    return cv_bc_result;
}

/// the job of this function is to calculate time step 
/// when two control volumes interact, the control volumes can be 
/// single control volumes or array control volumes
#[inline]
pub (crate) fn calculate_timestep_control_volume_serial(
    control_vol_1: &mut CVType,
    control_vol_2: &mut CVType,
    interaction: HeatTransferInteractionType) -> Result<Time,ThermalHydraulicsLibError>{

    // let me first match my control volumes to their various types
    // at each matching arm, use those as inputs

    let cv_result = match (control_vol_1, control_vol_2) {
        (CVType::SingleCV(single_cv_1), CVType::SingleCV(single_cv_2)) =>
        {
            single_cv_1.calculate_mesh_stability_timestep_for_two_single_cv_nodes(
                single_cv_2, interaction)
        }
        ,
        (CVType::FluidArrayCV(fluid_array_cv), CVType::SingleCV(single_cv_node)) =>
        {
            fluid_array_cv.calculate_timestep_for_single_cv_to_front_of_array_cv(
                    single_cv_node,
                    interaction)
        }
        ,
        (CVType::SolidArrayCV(solid_array_cv), CVType::SingleCV(single_cv_node)) =>
        {
            solid_array_cv.calculate_timestep_for_single_cv_to_front_of_array_cv(
                    single_cv_node,
                    interaction)
        }
        ,
        (CVType::SingleCV(single_cv_node), CVType::SolidArrayCV(solid_array_cv)) =>
        {
            solid_array_cv.
                calculate_timestep_for_single_cv_to_back_of_array_cv(
                    single_cv_node,
                    interaction)
        },
        (CVType::SingleCV(single_cv_node), CVType::FluidArrayCV(fluid_array_cv)) =>
        {
            fluid_array_cv.
                calculate_timestep_for_single_cv_to_back_of_array_cv(
                    single_cv_node,
                    interaction)
        },
        (CVType::FluidArrayCV(fluid_array_cv_1), CVType::FluidArrayCV(fluid_array_cv_2)) =>
        {
            fluid_array_cv_2.
                calculate_timestep_for_single_cv_to_back_of_array_cv(
                    &mut fluid_array_cv_1.front_single_cv,
                    interaction)
        },
        (CVType::SolidArrayCV(solid_array_cv_1), CVType::SolidArrayCV(solid_array_cv_2)) =>
        {
            solid_array_cv_2.
                calculate_timestep_for_single_cv_to_back_of_array_cv(
                    &mut solid_array_cv_1.front_single_cv,
                    interaction)
        },
        (CVType::FluidArrayCV(fluid_array_cv_1), CVType::SolidArrayCV(solid_array_cv_2)) =>
        {
            solid_array_cv_2.
                calculate_timestep_for_single_cv_to_back_of_array_cv(
                    &mut fluid_array_cv_1.front_single_cv,
                    interaction)
        },
        (CVType::SolidArrayCV(solid_array_cv_1), CVType::FluidArrayCV(fluid_array_cv_2)) =>
        {
            fluid_array_cv_2.
                calculate_timestep_for_single_cv_to_back_of_array_cv(
                    &mut solid_array_cv_1.front_single_cv,
                    interaction)
        },
    };


    return cv_result;

}

/// calculates timestep for control voluem and boundary condition 
/// in serial
/// for arrayCVs, the boundary condition is attached to the back
/// of the control volume
///
/// (boundary condition) ----- (back --- cv_1 --- front) 
///
pub (crate) fn calculate_timestep_control_volume_back_to_boundary_condition_serial(
    boundary_condition: &mut BCType,
    control_vol: &mut CVType,
    interaction: HeatTransferInteractionType) -> Result<Time,ThermalHydraulicsLibError> {


    let cv_bc_result = match (control_vol, boundary_condition) {
        (CVType::SingleCV(single_cv), BCType::UserSpecifiedHeatAddition(_heat_rate)) 
            => {
                calculate_mesh_stability_conduction_timestep_for_single_node_and_bc
                    (single_cv, interaction)
            }
        ,
        (CVType::SingleCV(single_cv), BCType::UserSpecifiedHeatFlux(_heat_flux))
            => {
                calculate_mesh_stability_conduction_timestep_for_single_node_and_bc
                    (single_cv, interaction)
            }
        ,
        (CVType::SingleCV(single_cv), BCType::UserSpecifiedTemperature(_bc_temperature))
            => {
                calculate_mesh_stability_conduction_timestep_for_single_node_and_bc
                    (single_cv, interaction)
            }
        ,
        (CVType::SolidArrayCV(solid_array_cv),BCType::UserSpecifiedHeatFlux(_heat_flux)) => {
            // match the cv to the cv type, extract the front boundary 
            // condition 
                calculate_mesh_stability_conduction_timestep_for_single_node_and_bc
                    (&mut solid_array_cv.back_single_cv, interaction)
        }
        ,
        (CVType::SolidArrayCV(solid_array_cv),BCType::UserSpecifiedHeatAddition(_heat_rate)) => {
            // match the cv to the cv type, extract the front boundary 
            // condition 
            calculate_mesh_stability_conduction_timestep_for_single_node_and_bc
                (&mut solid_array_cv.back_single_cv, interaction)
        }
        ,
        (CVType::SolidArrayCV(solid_array_cv),BCType::UserSpecifiedTemperature(_bc_temperature)) => {
            // match the cv to the cv type, extract the front boundary 
            // condition 
            calculate_mesh_stability_conduction_timestep_for_single_node_and_bc
                (&mut solid_array_cv.back_single_cv, interaction)
        }
        ,
        (CVType::FluidArrayCV(fluid_array_cv),BCType::UserSpecifiedHeatFlux(_heat_flux)) => {
            // match the cv to the cv type, extract the front boundary 
            // condition 
                calculate_mesh_stability_conduction_timestep_for_single_node_and_bc
                    (&mut fluid_array_cv.back_single_cv, interaction)
        }
        ,
        (CVType::FluidArrayCV(fluid_array_cv),BCType::UserSpecifiedHeatAddition(_heat_rate)) => {
            // match the cv to the cv type, extract the front boundary 
            // condition 
            calculate_mesh_stability_conduction_timestep_for_single_node_and_bc
                (&mut fluid_array_cv.back_single_cv, interaction)
        }
        ,
        (CVType::FluidArrayCV(fluid_array_cv),BCType::UserSpecifiedTemperature(_bc_temperature)) => {
            // match the cv to the cv type, extract the front boundary 
            // condition 
            calculate_mesh_stability_conduction_timestep_for_single_node_and_bc
                (&mut fluid_array_cv.back_single_cv, interaction)
        }
        ,
    };

    return cv_bc_result;
}

/// this is thermal conductance function based on interaction type 
/// may want to move the calculation bits to the calculation module in 
/// future
/// it calculates thermal conductance based on the supplied enum
///
/// TODO: probably want to test this function out
/// 
pub (in crate) 
fn try_get_thermal_conductance_based_on_interaction(
    temperature_1: ThermodynamicTemperature,
    temperature_2: ThermodynamicTemperature,
    pressure_1: Pressure,
    pressure_2: Pressure,
    interaction: HeatTransferInteractionType) 
-> Result<ThermalConductance, ThermalHydraulicsLibError> 
{

    let conductance: ThermalConductance = match 
        interaction {
            HeatTransferInteractionType::UserSpecifiedThermalConductance(
                user_specified_conductance) => user_specified_conductance,
            HeatTransferInteractionType
                ::SingleCartesianThermalConductanceOneDimension(
                material,thickness) => get_conductance_single_cartesian_one_dimension(
                    material,
                    temperature_1, 
                    temperature_2, 
                    pressure_1, 
                    pressure_2, 
                    thickness)?,
            HeatTransferInteractionType::
                CylindricalConductionConvectionLiquidInside(
                (solid_material, shell_thickness,
                solid_temperature, solid_pressure), 
                (h, inner_diameter, cylinder_length)) => {

                    let id: Length = inner_diameter.clone().into();
                    let thicnkess: Length = shell_thickness.clone().into();

                    let od: Length = id+thicnkess;

                    let outer_diameter: OuterDiameterThermalConduction = 
                    OuterDiameterThermalConduction::from(od);

                    // after all the typing conversion, we can 
                    // get our conductance
                    get_conductance_single_cylindrical_radial_solid_liquid(
                        solid_material,
                        solid_temperature,
                        solid_pressure,
                        h,
                        inner_diameter,
                        outer_diameter,
                        cylinder_length,
                        CylindricalAndSphericalSolidFluidArrangement::
                        FluidOnInnerSurfaceOfSolidShell ,
                    )?
                },

            HeatTransferInteractionType::
                CylindricalConductionConvectionLiquidOutside(
                (solid_material, shell_thickness,
                solid_temperature, solid_pressure), 
                (h, outer_diameter, cylinder_length)) => {

                    let od: Length = outer_diameter.clone().into();
                    let thicnkess: Length = shell_thickness.clone().into();

                    let id: Length = od - thicnkess;

                    let inner_diameter: InnerDiameterThermalConduction = 
                    InnerDiameterThermalConduction::from(id);

                    // after all the typing conversion, we can 
                    // get our conductance
                    get_conductance_single_cylindrical_radial_solid_liquid(
                        solid_material,
                        solid_temperature,
                        solid_pressure,
                        h,
                        inner_diameter,
                        outer_diameter,
                        cylinder_length,
                        CylindricalAndSphericalSolidFluidArrangement::
                        FluidOnOuterSurfaceOfSolidShell ,
                    )?
                },
            // note: actually function signatures are a little more 
            // friendly to use than packing enums with lots of stuff 
            // so may change stuffing enums with tuples to stuffing 
            // enums with a single struct
            HeatTransferInteractionType
                ::DualCylindricalThermalConductance(
                (inner_material,inner_shell_thickness),
                (outer_material,outer_shell_thickness),
                (inner_diameter,
                outer_diameter,
                cylinder_length)
            ) => {
                    // first, want to check if inner_diameter + 
                    // shell thicknesses is outer diameter 

                    let expected_outer_diameter: Length;
                    let id: Length = inner_diameter.into();
                    let inner_thickness: Length =  inner_shell_thickness.into();
                    let outer_thickness: Length =  outer_shell_thickness.into();

                    expected_outer_diameter = 
                        id + inner_thickness + outer_thickness;

                    let od: Length = outer_diameter.into();

                    // inner diameter and outer diameter values must be 
                    // equal to within 1 nanometer 1e-9 m
                    if (od.value - expected_outer_diameter.value).abs() > 1e-9
                    {

                        let mut error_str: String = "the inner diameter 
                            plus shell thicknesses do not equate 
                            to outer diameter".to_string();

                        error_str += "supplied outer diameter (m):";
                        error_str += &od.value.to_string();
                        error_str += "expected outer diameter (m):";
                        error_str += &expected_outer_diameter.value.to_string();


                        return Err(ThermalHydraulicsLibError::GenericStringError(error_str));
                    }

                    get_conductance_cylindrical_radial_two_materials(
                        inner_material,
                        outer_material,
                        temperature_1, //convention, 1 is inner shell
                        temperature_2, // convention 2, is outer shell
                        pressure_1,
                        pressure_2,
                        inner_diameter,
                        inner_shell_thickness,
                        outer_shell_thickness,
                        cylinder_length,
                    )?
                },
            HeatTransferInteractionType::UserSpecifiedHeatAddition  
                => {
                    println!("interaction type needs to be \n 
                        thermal conductance");
                    return Err(ThermalHydraulicsLibError::WrongHeatTransferInteractionType);
                },

            HeatTransferInteractionType::
                UserSpecifiedHeatFluxCustomArea(_) => {

                    println!("interaction type needs to be \n 
                        thermal conductance");
                    return Err(ThermalHydraulicsLibError::WrongHeatTransferInteractionType);

                },
            HeatTransferInteractionType::
                UserSpecifiedHeatFluxCylindricalOuterArea(_,_) => {

                    println!("interaction type needs to be \n 
                        thermal conductance");
                    return Err(ThermalHydraulicsLibError::WrongHeatTransferInteractionType);

                },
            HeatTransferInteractionType::
                UserSpecifiedHeatFluxCylindricalInnerArea(_,_) => {

                    println!("interaction type needs to be \n 
                        thermal conductance");
                    return Err(ThermalHydraulicsLibError::WrongHeatTransferInteractionType);

                },
            HeatTransferInteractionType::
                DualCartesianThermalConductance(
                (material_1, thickness_1),
                (material_2,thickness_2)) => { 
                    
                    let conductnace_layer_1: ThermalConductance 
                    = get_conductance_single_cartesian_one_dimension(
                        material_1,
                        temperature_1, 
                        temperature_1, 
                        pressure_1, 
                        pressure_1, 
                        thickness_1)?;

                    let conductnace_layer_2: ThermalConductance 
                    = get_conductance_single_cartesian_one_dimension(
                        material_2,
                        temperature_2, 
                        temperature_2, 
                        pressure_2, 
                        pressure_2, 
                        thickness_2)?;

                    let overall_resistance = 
                    1.0/conductnace_layer_2 
                    + 1.0/conductnace_layer_1;

                    // return the conductance or resistnace inverse

                    1.0/overall_resistance
            },
            HeatTransferInteractionType::
                DualCartesianThermalConductanceThreeDimension(
                data_dual_cartesian_conduction) 
                => {

                    let material_1 = 
                    data_dual_cartesian_conduction .material_1;

                    let material_2 = 
                    data_dual_cartesian_conduction .material_2;

                    let thickness_1 = 
                    data_dual_cartesian_conduction .thickness_1;

                    let thickness_2 = 
                    data_dual_cartesian_conduction .thickness_2;

                    let xs_area = 
                    data_dual_cartesian_conduction .xs_area;

                    get_conductance_dual_cartesian_three_dimensions(
                        material_1, 
                        material_2, 
                        temperature_1, 
                        temperature_2, 
                        pressure_1, 
                        pressure_2, 
                        xs_area, 
                        thickness_1,
                        thickness_2)?
                },

            HeatTransferInteractionType::
                UserSpecifiedConvectionResistance(
                data_convection_resistance) 
                => {

                    let heat_transfer_coeff: HeatTransfer = 
                    data_convection_resistance.heat_transfer_coeff;
                    let surf_area: Area = 
                    data_convection_resistance.surf_area.into();

                    heat_transfer_coeff * surf_area
                },

            HeatTransferInteractionType::Advection(_) => {
                println!("advection interaction types \n 
                do not correspond to conductance");
                return Err(ThermalHydraulicsLibError::WrongHeatTransferInteractionType);
            },
            HeatTransferInteractionType::SimpleRadiation
                (area_coeff, hot_temperature, cold_temperature) => 
                {
                    simple_radiation_conductance(
                        area_coeff, 
                        hot_temperature, 
                        cold_temperature)
                }
            ,

        };

    return Ok(conductance);
}

/// the job of this function is to handle interactions between 
/// a control_volume and a boundary condition
///
/// here we have to handle some BC types, 
/// these are the most basic:
/// 1. constant heat flux
/// 2. constant temperature
/// 3. constant heat addition
///
/// For each case, there should be a function
/// to handle each case
/// 
/// for arrayCVs, the boundary condition is attached to the front 
/// of the control volume
///
/// (back --- cv_1 --- front) ---- (boundary condition)
#[inline]
pub (crate) fn attach_boundary_condition_to_control_volume_front_serial(
    control_vol: &mut CVType,
    boundary_condition: &mut BCType,
    interaction: HeatTransferInteractionType) -> Result<(),ThermalHydraulicsLibError> {

    let cv_bc_result = match (control_vol, boundary_condition) {
        (CVType::SingleCV(single_cv), BCType::UserSpecifiedHeatAddition(heat_rate)) 
            => calculate_constant_heat_addition_front_single_cv_back(
                single_cv, *heat_rate, interaction),
        (CVType::SingleCV(single_cv), BCType::UserSpecifiedHeatFlux(heat_flux))
            => calculate_constant_heat_flux_front_single_cv_back(
                single_cv, *heat_flux, interaction),
        (CVType::SingleCV(single_cv), BCType::UserSpecifiedTemperature(bc_temperature))
            => calculate_constant_temperature_front_single_cv_back(
                single_cv, *bc_temperature, interaction),
        (CVType::SolidArrayCV(solid_array_cv),BCType::UserSpecifiedHeatFlux(heat_flux)) => {
            solid_array_cv.link_heat_flux_bc_to_front_of_this_cv(
                *heat_flux,
                interaction)
        },
        (CVType::SolidArrayCV(solid_array_cv),BCType::UserSpecifiedHeatAddition(heat_rate)) => {
            solid_array_cv.link_heat_addition_to_front_of_this_cv(
                *heat_rate,
                interaction)
        },
        (CVType::SolidArrayCV(solid_array_cv),BCType::UserSpecifiedTemperature(bc_temperature)) => {
            solid_array_cv.link_constant_temperature_to_front_of_this_cv(
                *bc_temperature,
                interaction)
        },
        (CVType::FluidArrayCV(fluid_array_cv),BCType::UserSpecifiedHeatFlux(heat_flux)) => {
            fluid_array_cv.link_heat_flux_bc_to_front_of_this_cv(
                *heat_flux,
                interaction)
        },
        (CVType::FluidArrayCV(fluid_array_cv),BCType::UserSpecifiedHeatAddition(heat_rate)) => {
            fluid_array_cv.link_heat_addition_to_front_of_this_cv(
                *heat_rate,
                interaction)
        },
        (CVType::FluidArrayCV(fluid_array_cv),BCType::UserSpecifiedTemperature(bc_temperature)) => {
            fluid_array_cv.link_constant_temperature_to_front_of_this_cv(
                *bc_temperature,
                interaction)
        },
    };

    return cv_bc_result;
}
/// the job of this function is to handle interactions between 
/// a control_volume and a boundary condition
///
/// here we have to handle some BC types, 
/// these are the most basic:
/// 1. constant heat flux
/// 2. constant temperature
/// 3. constant heat addition
///
/// For each case, there should be a function
/// to handle each case,
/// for single CVs, it does pretty much the same job
/// 
/// for arrayCVs, the boundary condition is attached to the front 
/// of the control volume
///
/// (boundary condition) ---- (back --- cv_1 --- front) 
pub (crate) fn attach_boundary_condition_to_control_volume_back_serial(
    boundary_condition: &mut BCType,
    control_vol: &mut CVType,
    interaction: HeatTransferInteractionType) -> Result<(),ThermalHydraulicsLibError> {

    let cv_bc_result = match (control_vol, boundary_condition) {
        (CVType::SingleCV(single_cv), BCType::UserSpecifiedHeatAddition(heat_rate)) 
            => calculate_single_cv_front_constant_heat_addition_back(
                *heat_rate, single_cv, interaction),
        (CVType::SingleCV(single_cv), BCType::UserSpecifiedHeatFlux(heat_flux))
            => calculate_single_cv_front_heat_flux_back(
                *heat_flux,single_cv, interaction),
        (CVType::SingleCV(single_cv), BCType::UserSpecifiedTemperature(bc_temperature))
            => calculate_single_cv_node_front_constant_temperature_back(
                *bc_temperature, single_cv, interaction),
        (CVType::FluidArrayCV(cv),BCType::UserSpecifiedHeatFlux(heat_flux)) => {
            cv.link_heat_flux_bc_to_back_of_this_cv(
                *heat_flux,
                interaction)

        },
        (CVType::FluidArrayCV(cv),BCType::UserSpecifiedHeatAddition(heat_rate)) => {
            cv.link_heat_addition_to_back_of_this_cv(
                *heat_rate,
                interaction)
        },
        (CVType::FluidArrayCV(cv),BCType::UserSpecifiedTemperature(bc_temperature)) => {
            cv.link_constant_temperature_to_back_of_this_cv(
                *bc_temperature,
                interaction)
        },
        (CVType::SolidArrayCV(cv),BCType::UserSpecifiedHeatFlux(heat_flux)) => {
            cv.link_heat_flux_bc_to_back_of_this_cv(
                *heat_flux,
                interaction)

        },
        (CVType::SolidArrayCV(cv),BCType::UserSpecifiedHeatAddition(heat_rate)) => {
            cv.link_heat_addition_to_back_of_this_cv(
                *heat_rate,
                interaction)
        },
        (CVType::SolidArrayCV(cv),BCType::UserSpecifiedTemperature(bc_temperature)) => {
            cv.link_constant_temperature_to_back_of_this_cv(
                *bc_temperature,
                interaction)
        },
    };

    return cv_bc_result;
}

/// for connecting a bc to cv where 
///
/// (cv) ---------- (constant temperature bc)
#[inline]
pub fn calculate_constant_temperature_front_single_cv_back(
    control_vol: &mut SingleCVNode,
    boundary_condition_temperature: ThermodynamicTemperature,
    interaction: HeatTransferInteractionType) -> Result<(), ThermalHydraulicsLibError> {

    match interaction {
        HeatTransferInteractionType::Advection(
            advection_dataset) => {

            // 
            control_vol.calculate_bc_front_cv_back_advection_set_temperature(
                boundary_condition_temperature, advection_dataset)?;
            return Ok(());
        },
        _ => (),
    }
    // if anything else, use conductance

    control_vol.calculate_single_cv_node_constant_temperature_conductance(
        boundary_condition_temperature,
        interaction)?;
    return Ok(());
}


/// calculates the interaction between a heat flux BC and 
/// a control volume 
///
/// (single cv) ------------------ (heat flux bc)
///
/// the heat addition is at the front, the cv is at the back
pub fn calculate_constant_heat_flux_front_single_cv_back(
    control_vol: &mut SingleCVNode,
    heat_flux_into_control_vol: HeatFluxDensity,
    interaction: HeatTransferInteractionType) -> Result<(), ThermalHydraulicsLibError> {

    // first, obtain a heat transfer area from the constant heat flux 
    // BC
    let heat_transfer_area: Area = match interaction{
        HeatTransferInteractionType::
            UserSpecifiedThermalConductance(_) => 
            {
                println!("please specify interaction type as \n 
                UserSpecifiedHeatFluxCustomArea or Similar");
                return Err(ThermalHydraulicsLibError::WrongHeatTransferInteractionType);
            }
        ,

        HeatTransferInteractionType::
            SingleCartesianThermalConductanceOneDimension(_, _) => 
            {
                println!("please specify interaction type as \n 
                UserSpecifiedHeatFluxCustomArea or Similar");
                return Err(ThermalHydraulicsLibError::WrongHeatTransferInteractionType);
            }
        ,

        HeatTransferInteractionType::
            DualCartesianThermalConductance(_, _) => 
            {
                println!("please specify interaction type as \n 
                UserSpecifiedHeatFluxCustomArea or Similar");
                return Err(ThermalHydraulicsLibError::WrongHeatTransferInteractionType);
            }
        ,

        HeatTransferInteractionType::
            DualCylindricalThermalConductance(_, _, _) => 
            {
                println!("please specify interaction type as \n 
                UserSpecifiedHeatFluxCustomArea or Similar");
                return Err(ThermalHydraulicsLibError::WrongHeatTransferInteractionType);
            }
        ,

        HeatTransferInteractionType::
            CylindricalConductionConvectionLiquidOutside(_, _) => 
            {
                println!("please specify interaction type as \n 
                UserSpecifiedHeatFluxCustomArea or Similar");
                return Err(ThermalHydraulicsLibError::WrongHeatTransferInteractionType);
            }
        ,

        HeatTransferInteractionType::
            CylindricalConductionConvectionLiquidInside(_, _) => 
            {
                println!("please specify interaction type as \n 
                UserSpecifiedHeatFluxCustomArea or Similar");
                return Err(ThermalHydraulicsLibError::WrongHeatTransferInteractionType);
            }
        ,

        HeatTransferInteractionType::
            UserSpecifiedHeatAddition => 
            {
                println!("please specify interaction type as \n 
                UserSpecifiedHeatFluxCustomArea or Similar");
                return Err(ThermalHydraulicsLibError::WrongHeatTransferInteractionType);
            }
        ,

        HeatTransferInteractionType::
            DualCartesianThermalConductanceThreeDimension(_) => 
            {
                println!("please specify interaction type as \n 
                UserSpecifiedHeatFluxCustomArea or Similar");
                return Err(ThermalHydraulicsLibError::WrongHeatTransferInteractionType);
            }
        ,
        // these interaction types are acceptable
        HeatTransferInteractionType::
            UserSpecifiedHeatFluxCustomArea(area) => area,

            HeatTransferInteractionType::
                UserSpecifiedHeatFluxCylindricalOuterArea(
                    cylinder_length, od) => {
                    let od: Length = od.into();
                    let cylinder_length: Length  = cylinder_length.into();

                    let area = PI * od * cylinder_length;
                    area
                }
        ,

        HeatTransferInteractionType::
            UserSpecifiedHeatFluxCylindricalInnerArea(
                cylinder_length, id) => {
                let id: Length = id.into();
                let cylinder_length: Length  = cylinder_length.into();

                let area = PI * id * cylinder_length;
                area

            }
        ,

        HeatTransferInteractionType::
            UserSpecifiedConvectionResistance(_) => 
            {
                println!("please specify interaction type as \n 
                UserSpecifiedHeatFluxCustomArea or Similar");
                return Err(ThermalHydraulicsLibError::WrongHeatTransferInteractionType);
            }
        ,
        HeatTransferInteractionType::
            Advection(advection_data) => {

                calculate_bc_front_cv_back_advection_for_heat_flux_or_heat_addition(
                    control_vol,
                    advection_data)?;

                return Ok(());
            }
        ,
        HeatTransferInteractionType::SimpleRadiation
            (_area_coeff, _hot_temperature, _cold_temperature) => 
            {
                println!("please specify interaction type as \n 
                UserSpecifiedHeatFluxCustomArea or Similar");
                return Err(ThermalHydraulicsLibError::WrongHeatTransferInteractionType);
            }
        ,
    };

    let heat_flowrate_into_control_vol: Power = 
        heat_flux_into_control_vol * heat_transfer_area;

    control_vol.rate_enthalpy_change_vector.
        push(heat_flowrate_into_control_vol);

    // auto time stepping doesn't work for constant heat flux 
    // or specified power as well. 
    // it is best to see at the end of all power calculations what 
    // is the temperature change

    // For liquid CV, still need to calculate time scale based 
    // on convection flow
    // match statement is meant to tell that liquid CVs are not quite 
    // ready for use
    //
    // Actually, for solid CV, I will also need to recalculate time scale 
    // based on the material thermal thermal_diffusivity
    let cv_material = control_vol.material_control_volume;
    match cv_material {
        Material::Solid(_) => {
            // in this case, we just have one cv and one bc 
            // so we only consider thermal inertia of this cv 
            calculate_mesh_stability_conduction_timestep_for_single_node_and_bc(
                control_vol,
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


/// contains code for heat transfer interactions between SingleCV 
/// and boundary conditions
pub mod single_cv_and_bc_interactions;

/// contains code for heat transfer interactions between SingleCV and 
/// other SingleCVs
pub mod single_cv_single_cv_interactions;
