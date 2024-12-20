use std::thread::JoinHandle;
use std::thread;

use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
use crate::pre_built_components::heat_transfer_entities::preprocessing::try_get_thermal_conductance_based_on_interaction;
use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
use crate::heat_transfer_correlations::nusselt_number_correlations::input_structs::NusseltPrandtlReynoldsData;
use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::boundary_conditions::BCType;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;

use super::NonInsulatedPorousMediaFluidComponent;
use uom::si::area::square_inch;
use uom::si::length::inch;
use uom::si::length::meter;
use uom::ConstZero;
use uom::si::pressure::atmosphere;
use uom::si::f64::*;
use ndarray::*;

impl NonInsulatedPorousMediaFluidComponent {




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

        self.interior_solid_array_for_porous_media.link_to_back(&mut zero_power_bc,
            interaction).unwrap();


        self.interior_solid_array_for_porous_media.link_to_front(&mut zero_power_bc,
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





    /// spawns a thread and moves the clone of the entire heater object into the 
    /// thread, "locking" it for parallel computation
    ///
    /// once that is done, the join handle is returned 
    /// which when unwrapped, returns the heater object
    pub fn lateral_connection_thread_spawn(&self,
    mass_flowrate: MassRate,
    heater_steady_state_power: Power) -> JoinHandle<Self>{

        let mut heater_clone = self.clone();

        // move ptr into a new thread 

        let join_handle = thread::spawn(
            move || -> Self {

                // carry out the connection calculations
                heater_clone.
                    ciet_heater_v2_lateral_and_miscellaneous_connections(
                        mass_flowrate,
                        heater_steady_state_power);
                
                heater_clone

            }
        );

        return join_handle;

    }

}



/// contains preprocessing functions specifically for 
/// ciet heater v2
pub mod ciet_heater_v2;
pub use ciet_heater_v2::*;
