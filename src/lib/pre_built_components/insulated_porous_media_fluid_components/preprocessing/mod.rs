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
use crate::tuas_lib_error::TuasLibError;

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

    /// obtains the conductance from ambient to the pipe shell 
    /// nodally speaking 

    #[inline]
    pub fn get_ambient_to_insulation_nodal_conductance(&mut self,
        heat_transfer_to_ambient: HeatTransfer) 
        -> Result<ThermalConductance,TuasLibError> {

            // the solid conductance is calculated using 
            // k (A/L) where L is representative thickness 
            // A is heat transfer area,
            // k is thermal conductivity
            //
            let insulation_conductance_lengthscale: Length = 
                self.thermal_conductance_lengthscale_insulation_to_ambient;
            
            // to calculate k, we need the bulk temperature 

            let mut insulation_clone: SolidColumn = 
                self.insulation_array.clone().try_into().unwrap();
            let insulation_bulk_temp: ThermodynamicTemperature = 
                insulation_clone.try_get_bulk_temperature()?;

            // next, let's get the conductivity 

            let insulation_material_conductivity: ThermalConductivity = 
                insulation_clone.material_control_volume
                .try_get_thermal_conductivity(
                    insulation_bulk_temp)?;

            let number_of_nodes: f64 = self.inner_nodes as f64 + 2.0;
            // solid side nodalised thermal conductance

            let nodalised_insulation_side_thermal_conductance: ThermalConductance
                = insulation_conductance_lengthscale * insulation_material_conductivity
                / number_of_nodes;

            let nodalised_insulation_side_thermal_resistance: ThermalResistance 
                = nodalised_insulation_side_thermal_conductance.recip();

            // next, nodalised thermal conductance due to liquid side 
            
            let nodalised_thermal_conductance_ambient_convection: ThermalConductance 
                = (heat_transfer_to_ambient * self.convection_heat_transfer_area_insulation_to_ambient)
                / number_of_nodes;

            let nodalised_ambient_convection_thermal_resistance: ThermalResistance 
                = nodalised_thermal_conductance_ambient_convection.recip();


            // add resistances together 
            let nodalised_insulation_to_ambient_resistance: ThermalResistance 
                = nodalised_insulation_side_thermal_resistance +
                nodalised_ambient_convection_thermal_resistance;

            // get conductance, and then return 

            let nodalised_insulation_to_ambient_conductance 
                = nodalised_insulation_to_ambient_resistance.recip();

            // and we done!
            return Ok(nodalised_insulation_to_ambient_conductance);

    }
}


/// contains preprocessing calcs specifc to mx10 and static 
/// mixers
pub mod mx10;
