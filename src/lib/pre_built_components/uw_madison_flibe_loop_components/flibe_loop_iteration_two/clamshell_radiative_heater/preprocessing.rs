use std::thread::JoinHandle;
use std::thread;

use uom::si::thermodynamic_temperature::kelvin;
use uom::ConstZero;
use uom::si::pressure::atmosphere;
use uom::si::f64::*;
use ndarray::*;
use super::ClamshellRadiativeHeater;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component::FluidComponent;
use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
use crate::heat_transfer_correlations::thermal_resistance::try_get_thermal_conductance_annular_cylinder;
use crate::{heat_transfer_correlations::nusselt_number_correlations::input_structs::GnielinskiData, pre_built_components::heat_transfer_entities::preprocessing::try_get_thermal_conductance_based_on_interaction};
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::boundary_conditions::BCType;
use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;

use crate::tuas_lib_error::TuasLibError;

// preprocessing is where heat transfer entities 
// are connected to each other whether axially or laterally
//
//

impl ClamshellRadiativeHeater {

    /// the end of each node should have a zero power boundary condition 
    /// connected to each of them at the bare minimum
    ///
    /// this function does exactly that
    ///
    /// to connect the rest of the heat transfer entities, 
    /// use the link to front or back methods within the 
    /// FluidArrays or SolidColumns
    ///
    /// note that for the STHE, the link to front and back 
    /// functions are exactly the same as for non parallel components,
    /// the parallel treatment is given in the advance timestep 
    /// portion of the code
    #[inline]
    fn zero_power_bc_axial_connection(&mut self) -> Result<(),TuasLibError>{

        let zero_power: Power = Power::ZERO;

        let mut zero_power_bc: HeatTransferEntity = 
        HeatTransferEntity::BoundaryConditions(
            BCType::UserSpecifiedHeatAddition(zero_power)
        );

        // constant heat addition interaction 

        let interaction: HeatTransferInteractionType = 
        HeatTransferInteractionType::UserSpecifiedHeatAddition;

        // now connect the four or five arrays 
        // two solid arrays (or three if insulation is switched on) 
        // and two fluid arrays


        // tube side
        self.pipe_fluid_array.link_to_front(&mut zero_power_bc,
            interaction)?;

        self.pipe_fluid_array.link_to_back(&mut zero_power_bc,
            interaction)?;

        self.pipe_shell_array.link_to_front(&mut zero_power_bc,
            interaction)?;

        self.pipe_shell_array.link_to_back(&mut zero_power_bc,
            interaction)?;

        // annular air array
        self.annular_air_array.link_to_front(&mut zero_power_bc,
            interaction)?;

        self.annular_air_array.link_to_back(&mut zero_power_bc,
            interaction)?;

        // heating element and insulation
        self.outer_shell.link_to_front(&mut zero_power_bc,
            interaction)?;

        self.outer_shell.link_to_back(&mut zero_power_bc,
            interaction)?;


        self.insulation_array.link_to_front(&mut zero_power_bc,
            interaction)?;

        self.insulation_array.link_to_back(&mut zero_power_bc,
            interaction)?;



        Ok(())
    }
}
