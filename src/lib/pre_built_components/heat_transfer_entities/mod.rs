use self::cv_types::CVType;
use crate::tuas_lib_error::TuasLibError;
use crate::boundary_conditions::BCType;
/// Contains entities which transfer heat and interact with each 
/// other
///
/// for example, control volumes and boundary conditions
#[derive(Debug,Clone,PartialEq)]
pub enum HeatTransferEntity {
    /// Contains a list of ControlVolumeTypes
    ControlVolume(CVType),
    /// Contains a list of Boundary conditions
    BoundaryConditions(BCType)
}

impl HeatTransferEntity {

    /// allows the user to override the heat transfer entity 
    pub fn set(&mut self, 
        user_input_hte: HeatTransferEntity) -> Result<(), TuasLibError>{
        *self = user_input_hte;

        Ok(())
    }

}

/// all the types of Control volumes are represented in an enum 
/// to abstract away the complications of connecting different types 
/// of control volumes. 
pub mod cv_types;


/// converts to and from boundary conditions 
pub mod bc_types;

/// preprocessing 
///
/// this module contains abstraction pertaining 
/// to how to set up a heat transfer problem 
///
/// This means setting up the timestep, mass flowrates and how 
/// heat transfer entities are linked to each other via heat 
/// transfer interactions
pub mod preprocessing;

/// postprocessing contains functions to obtain temperature profiles 
/// of the HeatTransferEntity
pub mod  postprocessing;

/// calculation modules deal mainly with advancing timestep
pub mod calculation;

/// type conversion 
/// converts underlying nested enums into HeatTransferEntity objects
pub mod type_conversion;


/// convert to data_advection 
/// that is to say, you can construct a DataAdvection struct from 
/// a HeatTransferEntity
pub mod conversion_to_data_advection;


/// tests 
#[cfg(test)]
pub mod tests;
