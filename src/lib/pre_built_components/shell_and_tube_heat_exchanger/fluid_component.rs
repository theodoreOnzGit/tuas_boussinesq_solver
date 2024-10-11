use uom::si::f64::*;

use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component::FluidComponent;

use super::SimpleShellAndTubeHeatExchanger;

impl SimpleShellAndTubeHeatExchanger {


    /// clones the shell side fluid array, and converts it into a 
    /// fluid component
    pub fn get_clone_of_shell_side_fluid_component
        (&self) -> FluidComponent 
    {

        // first clone the heat transfer entity
        let shell_side_fluid_hte_clone: HeatTransferEntity 
            = self.shell_side_fluid_array.clone();

        // convert it into a fluid array
        let shell_side_fluid_array: FluidArray
            = shell_side_fluid_hte_clone.try_into().unwrap();

        return shell_side_fluid_array.into();

    }

    /// clones the tube side fluid array and converts it into 
    /// a fluid component 
    pub fn get_clone_of_tube_side_parallel_tube_fluid_component
        (&self) -> FluidComponent {

            let fluid_array_heat_transfer_entity = self.tube_side_fluid_array_for_single_tube.clone();
            let fluid_array: FluidArray = fluid_array_heat_transfer_entity.try_into().unwrap();

            let number_of_parallel_tubes: u32 = self.number_of_tubes;

            FluidComponent::ParallelUniformFluidArray(
                fluid_array,number_of_parallel_tubes)
    }

    /// sets the tube side mass flowrate 
    pub fn set_tube_side_total_mass_flowrate(&mut self,
        mass_flowrate_over_all_tubes: MassRate) {

        let mut tube_side_fluid_array: FluidArray = 
        self.tube_side_fluid_array_for_single_tube.clone().try_into().unwrap();


        let single_tube_mass_rate = 
            mass_flowrate_over_all_tubes / (self.number_of_tubes as f64);

        tube_side_fluid_array.set_mass_flowrate(single_tube_mass_rate);
        // unfortunately, this makes setting mass flowrate quite 
        // expensive as we need to clone it everytime

        self.tube_side_fluid_array_for_single_tube.set(tube_side_fluid_array.into()).unwrap();

    }

    /// sets the tube side mass flowrate 
    pub fn set_shell_side_total_mass_flowrate(&mut self,
        mass_flowrate_through_shell: MassRate) {

        let mut shell_side_fluid_array: FluidArray = 
        self.shell_side_fluid_array.clone().try_into().unwrap();



        shell_side_fluid_array.set_mass_flowrate(mass_flowrate_through_shell);
        // unfortunately, this makes setting mass flowrate quite 
        // expensive as we need to clone it everytime

        self.shell_side_fluid_array.set(shell_side_fluid_array.into()).unwrap();

    }

    /// returns tube side hydraulic diameter 
    /// by default, the internal diameter of the tube side
    /// assumes that tube side is circular
    #[inline]
    pub fn get_tube_side_hydraulic_diameter_circular_tube(&self) -> Length {
        return self.tube_side_id;
    }

    /// returns the shell side hydraulic diameter 
    /// this assumes that the shell side is a big tube with 
    /// a number of uniform circular tubes inside the big tube 
    ///
    /// from Du's paper, the formula here is:
    /// D_e = (D_i^2 - N_t d_o^2) / (D_i + N_t d_i)
    ///
    /// where 
    /// D_i  is shell_side_id
    /// d_i  is tube_side_id
    /// d_o is tube_side_od
    /// N_t is number_of_tubes
    #[inline]
    pub fn get_shell_side_hydraulic_diameter(&self) -> Length {


        // or just take the hydraulic diameter from the 
        // shell side fluid array 
        // 
        let shell_side_fluid_array: FluidArray = 
            self.shell_side_fluid_array.clone().try_into().unwrap();

        let hydraulic_diameter: Length 
            = shell_side_fluid_array.get_hydraulic_diameter_immutable();

        return hydraulic_diameter;
    }

    /// returns the effective length of the single pass sthe
    #[inline]
    pub fn get_effective_length(&self) -> Length {


        // or just take the hydraulic diameter from the 
        // shell side fluid array 
        // 
        let shell_side_fluid_array: FluidArray = 
            self.shell_side_fluid_array.clone().try_into().unwrap();

        let effective_length: Length 
            = shell_side_fluid_array.get_component_length_immutable();

        return effective_length;
    }


    /// returns the shell side cross sectional area 
    /// this assumes that the shell side is a big tube with 
    /// a number of uniform circular tubes inside the big tube 
    ///
    /// from Du's paper, the formula here is:
    ///
    /// pi/4 * (D_i^2 - N_t d_o^2)
    pub fn get_shell_side_cross_sectional_area(&self) -> Area {

        let shell_side_fluid_array: FluidArray = 
            self.shell_side_fluid_array.clone().try_into().unwrap();

        let shell_side_xs_area: Area 
            = shell_side_fluid_array.get_cross_sectional_area_immutable();

        return shell_side_xs_area;
    }

    /// returns shell side thermal conductivity 
    pub fn get_shell_side_fluid_thermal_conductivity(&self) -> ThermalConductivity {

        let mut shell_side_fluid_arr_clone: FluidArray = 
            self.shell_side_fluid_array.clone().
            try_into().unwrap();

        // get h_parasitic
        // Nu_parasitic = h_parasitic * D_e/lambda_s
        // lambda_s = thermal conductivity
        let shell_side_temperature: ThermodynamicTemperature = 
            shell_side_fluid_arr_clone.
            try_get_bulk_temperature().unwrap();


        let shell_fluid_material: LiquidMaterial
            = shell_side_fluid_arr_clone.material_control_volume.try_into().unwrap();

        let thermal_conductivity_shell_side_fluid: ThermalConductivity = 
            shell_fluid_material.try_get_thermal_conductivity(
                shell_side_temperature).unwrap();

        thermal_conductivity_shell_side_fluid
    }

}
