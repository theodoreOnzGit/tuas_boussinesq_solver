use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::boussinesq_thermophysical_properties::SolidMaterial;

use super::heat_transfer_entities::cv_types::CVType;
use super::heat_transfer_entities::HeatTransferEntity;
use uom::si::f64::*;

/// The simplest component is an insulated pipe
///
/// This is a simple pipe with a set hydraulic diameter and length
///
/// the standard assumption is that at each boundary of this pipe,
/// there is no conduction heat transfer in the axial direction
#[derive(Clone,Debug,PartialEq)]
pub struct SolidStructure {

    inner_nodes: usize,

    /// this HeatTransferEntity represents the pipe shell 
    /// only one radial layer of control volumes is used to simulate 
    /// the pipe shell
    ///
    /// it is thermally coupled to insulation and to the fluid 
    /// in the pipe_fluid_array
    pub solid_array: HeatTransferEntity,

    /// length 
    pub strucutre_length: Length,

    /// cross section area 
    pub cross_sectional_area: Area,

}

impl SolidStructure {

    /// constructs a solid structure as a hollow cylinder
    pub fn new_hollow_cylinder(initial_temperature: ThermodynamicTemperature,
        solid_pressure: Pressure,
        cross_sectional_area: Area,
        shell_id: Length,
        shell_od: Length,
        cylinder_length: Length,
        pipe_shell_material: SolidMaterial,
        user_specified_inner_nodes: usize,) -> SolidStructure {

        // now the outer pipe array
        let pipe_shell = 
        SolidColumn::new_cylindrical_shell(
            cylinder_length,
            shell_id,
            shell_od,
            initial_temperature,
            solid_pressure,
            pipe_shell_material,
            user_specified_inner_nodes 
        );


        return Self { inner_nodes: user_specified_inner_nodes,
            solid_array: CVType::SolidArrayCV(pipe_shell).into(),
            strucutre_length: cylinder_length,
            cross_sectional_area,
        };
    }
}


/// stuff such as conductances are calculated here
pub mod preprocessing;



/// stuff for calculation is done here, ie, advancing timestep
pub mod calculation;

/// postprocessing stuff, ie, get the temperature vectors 
/// of both arrays of control volumes 
pub mod postprocessing;
