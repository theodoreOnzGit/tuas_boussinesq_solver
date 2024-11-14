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

    /// obtains air to radiative heater insulation layer
    /// conductance 
    ///
    ///
    /// excludes radiative heat transfer
    #[inline]
    pub fn get_air_to_insulation_nodal_conductance(&mut self,
        h_air_to_pipe_surf: HeatTransfer) 
        -> Result<ThermalConductance,TuasLibError> 
    {

        // for conductance calculations (no radiation), 
        // it is important to get the temperatures of the ambient 
        // surroundings and the dimensions of the outer shell or insulation

        let heated_length: Length;
        let insulation_id: Length;
        let insulation_od: Length;
        let outer_node_temperature: ThermodynamicTemperature;
        // shell and tube heat excanger (STHE) to air interaction
        let number_of_temperature_nodes = self.inner_nodes + 2;
        let outer_solid_array_clone: SolidColumn;

        // if there's insulation, the id is the outer diameter of 
        // the shell. 

        insulation_id = self.heating_element_od;
        insulation_od = self.heating_element_od + 2.0*self.insulation_thickness;

        // heated length is the shell side length 
        // first I need the fluid array as a fluid component

        let shell_side_fluid_component_clone: FluidComponent 
            = self.get_clone_of_annular_air_array();

        // then i need to get the component length 
        heated_length = shell_side_fluid_component_clone
            .get_component_length_immutable();

        // surface temperature is the insulation bulk temperature 
        // (estimated)

        let mut shell_side_fluid_array: FluidArray = 
            shell_side_fluid_component_clone.try_into().unwrap();

        outer_node_temperature = shell_side_fluid_array
            .try_get_bulk_temperature()?;

        // the outer node clone is insulation if it is switched on
        outer_solid_array_clone = 
            self.insulation_array.clone().try_into()?;


        let cylinder_mid_diameter: Length = 0.5*(insulation_id+insulation_od);


        let node_length = heated_length / 
            number_of_temperature_nodes as f64;

        let outer_node_air_conductance_interaction: HeatTransferInteractionType
        = HeatTransferInteractionType::
            CylindricalConductionConvectionLiquidOutside(
                (outer_solid_array_clone.material_control_volume, 
                    (insulation_od-cylinder_mid_diameter).into(),
                    outer_node_temperature,
                    outer_solid_array_clone.pressure_control_volume),
                (h_air_to_pipe_surf,
                    insulation_od.into(),
                    node_length.into())
            );

        let outer_node_air_nodal_thermal_conductance: ThermalConductance = try_get_thermal_conductance_based_on_interaction(
            self.ambient_temperature,
            outer_node_temperature,
            outer_solid_array_clone.pressure_control_volume,
            outer_solid_array_clone.pressure_control_volume,
            outer_node_air_conductance_interaction,
        )?;


        return Ok(outer_node_air_nodal_thermal_conductance);
    }
}
