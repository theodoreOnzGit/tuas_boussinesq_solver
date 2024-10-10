use std::thread::{self};
use std::thread::JoinHandle;

use super::StructuralSupport;
use ndarray::*;
use uom::si::f64::*;
use uom::ConstZero;

use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::boundary_conditions::BCType;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;

impl StructuralSupport {


    /// used to connect the arrays laterally 
    /// you'll need to set the mass flowrate and heater power
    ///
    /// executes serially, and uses lots of cloning, so it's 
    /// heavier in resource usage,
    ///
    /// unoptimised in this regard
    #[inline]
    pub fn lateral_and_miscellaneous_connections(&mut self){

        let h_air_to_steel_surf = self.heat_transfer_to_air;
        let heater_steady_state_power: Power = Power::ZERO;

        // clone each array and set them later

        let mut steel_shell_clone: SolidColumn = 
        self.support_array.clone().try_into() .unwrap();


        // first let's get all the conductances 

        let support_to_air_conductance: ThermalConductance 
        = self.get_air_to_steel_array_conductance(
            h_air_to_steel_surf
        );



        // power fraction array
        let number_of_temperature_nodes = self.inner_nodes + 2;
        let q_fraction_per_node: f64 = 
        1.0/ number_of_temperature_nodes as f64;
        let mut q_frac_arr: Array1<f64> = 
        Array::default(number_of_temperature_nodes);
        q_frac_arr.fill(q_fraction_per_node);

        // then get the ambient temperature 

        let ambient_air_temp = self.ambient_temperature;

        // ambient air temperature to insulation

        let mut ambient_temperature_vector: Vec<ThermodynamicTemperature> 
        = Array1::default(number_of_temperature_nodes)
            .iter().map( |&temp| {
                temp
            }
            ).collect();

        ambient_temperature_vector.fill(ambient_air_temp);




        // second, fill them into the each array 


        // insulation to steel shell interaction 

        steel_shell_clone.lateral_link_new_temperature_vector_avg_conductance(
            support_to_air_conductance,
            ambient_temperature_vector.clone()
        ).unwrap();


        // we also want to add a heat source to steel shell
        //
        // technically no need though

        steel_shell_clone.lateral_link_new_power_vector(
            heater_steady_state_power,
            q_frac_arr
        ).unwrap();

        // note, must set mass flowrate first 
        // otherwise there is by default zero flow through 
        // the array


        // now that lateral connections are done, 
        // modify the heat transfer entity 


        self.support_array.set(steel_shell_clone.into()).unwrap();

        // adiabatic bc connections to make things finished 

        self.zero_power_bc_connection();
    }


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

        self.support_array.link_to_front(&mut zero_power_bc,
            interaction).unwrap();

        self.support_array.link_to_back(&mut zero_power_bc,
            interaction).unwrap();
    }




    /// obtains air to steel shell conductance
    #[inline]
    pub fn get_air_to_steel_array_conductance(&mut self,
    h_air_to_insulation_surf: HeatTransfer) 
        -> ThermalConductance {

        // find parameters for air to support surface conductance

        let number_of_temperature_nodes = self.inner_nodes + 2;

        // treat this as a lumped heat capacitance model, 
        // ie no conductive resistance

        let total_lateral_surface_area = self.total_lateral_surface_area;

        let node_area: Area = 
        total_lateral_surface_area / number_of_temperature_nodes as f64;

        let air_convection_conductance: ThermalConductance =
        node_area * h_air_to_insulation_surf;


        return air_convection_conductance;

    }

    /// obtains node to bc conductance 
    #[inline]
    pub fn get_axial_node_to_bc_conductance(&mut self) -> ThermalConductance {

        let mut steel_support_clone: SolidColumn = 
        self.support_array.clone().try_into().unwrap();
        let number_of_temperature_nodes = self.inner_nodes + 2;
        let component_length: Length = steel_support_clone.get_component_length();
        let steel_bulk_temperature = steel_support_clone.try_get_bulk_temperature() 
            .unwrap();

        let steel: SolidMaterial = steel_support_clone.material_control_volume
            .try_into().unwrap();

        let steel_conductivity: ThermalConductivity 
        = steel.try_get_thermal_conductivity(
            steel_bulk_temperature
        ).unwrap();

        let node_length: Length  = component_length / 
        number_of_temperature_nodes as f64;

        let node_xs_area = steel_support_clone.get_component_xs_area();

        let conductance: ThermalConductance 
        = steel_conductivity * node_xs_area / node_length *0.5;

        return conductance;
    }



    /// spawns a thread and moves the clone of the entire heater object into the 
    /// thread, "locking" it for parallel computation
    ///
    /// once that is done, the join handle is returned 
    /// which when unwrapped, returns the heater object
    pub fn lateral_connection_thread_spawn(&self) -> JoinHandle<Self>{

        let mut component_clone = self.clone();

        // move ptr into a new thread 

        let join_handle = thread::spawn(
            move || -> Self {

                // carry out the connection calculations
                component_clone.lateral_and_miscellaneous_connections();
                
                component_clone

            }
        );

        return join_handle;

    }

}


