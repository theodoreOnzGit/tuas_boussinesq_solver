use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::boundary_conditions::BCType;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::pre_built_components::non_insulated_fluid_components::NonInsulatedFluidComponent;
use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
use uom::si::{f64::*, specific_heat_capacity::joule_per_kilogram_kelvin};
use std::time::SystemTime;
use std::thread::JoinHandle;

use uom::{si::{time::second, power::kilowatt}, ConstZero};

use uom::si::thermodynamic_temperature::degree_celsius;

use uom::si::mass_rate::kilogram_per_second;
#[test]
pub fn cp_for_therminol_vp_1(){

    let initial_temperature: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(95.0);
    let cp_therminol: SpecificHeatCapacity = 
        LiquidMaterial::TherminolVP1.try_get_cp(
            initial_temperature).unwrap();

    approx::assert_relative_eq!(
        cp_therminol.get::<joule_per_kilogram_kelvin>(),
        1785.9,
        max_relative=1e-5
        );


}
