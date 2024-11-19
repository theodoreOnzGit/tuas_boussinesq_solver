use uom::si::f64::*;

use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component::FluidComponent;

use super::ClamshellRadiativeHeater;

impl ClamshellRadiativeHeater {

    /// clones the shell side fluid array, and converts it into a 
    /// fluid component
    pub fn get_clone_of_annular_air_array
        (&self) -> FluidComponent 
    {

        // first clone the heat transfer entity
        let annular_air_array_hte_clone: HeatTransferEntity 
            = self.annular_air_array.clone();

        // convert it into a fluid array
        let annular_air_fluid_array: FluidArray
            = annular_air_array_hte_clone.try_into().unwrap();

        return annular_air_fluid_array.into();

    }
    /// returns the annular air hydraulic diameter
    #[inline]
    pub fn get_annular_air_hydraulic_diameter(&self) -> Length {


        // or just take the hydraulic diameter from the 
        // shell side fluid array 
        // 
        let shell_side_fluid_array: FluidArray = 
            self.annular_air_array.clone().try_into().unwrap();

        let hydraulic_diameter: Length 
            = shell_side_fluid_array.get_hydraulic_diameter_immutable();

        return hydraulic_diameter;
    }

    /// returns the annular air cross sectional area 
    pub fn get_annular_air_cross_sectional_area(&self) -> Area {

        let shell_side_fluid_array: FluidArray = 
            self.annular_air_array.clone().try_into().unwrap();

        let shell_side_xs_area: Area 
            = shell_side_fluid_array.get_cross_sectional_area_immutable();

        return shell_side_xs_area;
    }
    /// returns tube side hydraulic diameter 
    /// by default, the internal diameter of the tube side
    /// assumes that tube side is circular
    #[inline]
    pub fn get_tube_side_hydraulic_diameter_circular_tube(&self) -> Length {
        return self.tube_id;
    }
    /// sets the tube side mass flowrate 
    pub fn set_tube_side_total_mass_flowrate(&mut self,
        mass_flowrate: MassRate) {

        let mut tube_side_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();


        let single_tube_mass_rate = mass_flowrate;

        tube_side_fluid_array.set_mass_flowrate(single_tube_mass_rate);
        // unfortunately, this makes setting mass flowrate quite 
        // expensive as we need to clone it everytime

        self.pipe_fluid_array.set(tube_side_fluid_array.into()).unwrap();

    }

    /// sets the tube side mass flowrate 
    pub fn set_annular_side_total_mass_flowrate(&mut self,
        mass_flowrate_through_annulus: MassRate) {

        let mut shell_side_fluid_array: FluidArray = 
        self.annular_air_array.clone().try_into().unwrap();



        shell_side_fluid_array.set_mass_flowrate(mass_flowrate_through_annulus);
        // unfortunately, this makes setting mass flowrate quite 
        // expensive as we need to clone it everytime

        self.annular_air_array.set(shell_side_fluid_array.into()).unwrap();

    }
}
