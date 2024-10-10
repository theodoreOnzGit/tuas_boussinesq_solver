use std::f64::consts::PI;
use std::thread::JoinHandle;
use std::thread;

use uom::ConstZero;
use uom::si::f64::*;
use ndarray::*;
use super::SolidStructure;
use crate::pre_built_components::heat_transfer_entities::preprocessing::try_get_thermal_conductance_based_on_interaction;
use crate::boundary_conditions::BCType;
use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::{DataUserSpecifiedConvectionResistance, HeatTransferInteractionType};
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;

use crate::thermal_hydraulics_error::ThermalHydraulicsLibError;

impl SolidStructure {

    /// used to connect the arrays laterally 
    /// you'll need to set the heater power
    ///
    /// executes serially, and uses lots of cloning, so it's 
    /// heavier in resource usage,
    ///
    /// unoptimised in this regard
    /// at each timestep, you are allowed to set a heater power, where 
    /// heat is dumped into the heated tube surrounding the pipe
    /// you set it using the heater power input here.
    ///
    /// otherwise you set it to zero for an unpowered pipe
    ///
    /// the input conductance is given on a per node or per control volume 
    /// basis (not total)
    ///
    /// However, the power input is a total power input
    #[inline]
    pub fn link_mixed_boundary_condition_laterally(&mut self,
        solid_array_to_ambient_nodal_conductance: ThermalConductance,
        ambient_temp: ThermodynamicTemperature,
        total_power_input_into_column: Power,
        ) -> Result<(), ThermalHydraulicsLibError>{

        //
        // 1. we need to consider discretisation, ie how much 
        // power fraction
        let number_of_temperature_nodes = self.inner_nodes + 2;
        let q_fraction_per_node: f64 = 1.0/ number_of_temperature_nodes as f64;
        let mut q_frac_arr: Array1<f64> = Array::default(number_of_temperature_nodes);
        q_frac_arr.fill(q_fraction_per_node);

        // then get the ambient temperature 

        // lateral connections 
        {
            // first i will need to create temperature vectors 
            // for ambient temperature. This is for use in calculating 
            // heat loss from insulation to ambient air

            let mut ambient_temperature_vector: Vec<ThermodynamicTemperature> 
            = Array1::default(number_of_temperature_nodes)
                .iter().map( |&temp| {
                    temp
                }
                ).collect();

            ambient_temperature_vector.fill(ambient_temp);


            // for this process, I will make a clone of 
            // each HeatTransferEntity, modify the clone, then 
            // replace the HeatTransferEntity within the pipe using 
            // these changed entities

            let mut solid_array_clone: SolidColumn = 
                self.solid_array.clone().try_into()?;

            // second, fill them into the each array 
            
            // solid array to ambient interaction

            solid_array_clone.lateral_link_new_temperature_vector_avg_conductance(
                solid_array_to_ambient_nodal_conductance,
                ambient_temperature_vector
            )?;

            // solid array to power interaction
            solid_array_clone.lateral_link_new_power_vector(
                total_power_input_into_column,
                q_frac_arr
            )?;


            self.solid_array.set(solid_array_clone.into())?;

        }
        // axial connections (insulation by default)
        // you can of course add new ones

        self.zero_power_bc_connection()?;

        Ok(())

    }

    /// used to connect the arrays laterally 
    /// you'll need to set conductances and temperatures
    ///
    /// executes serially, and uses lots of cloning, so it's 
    /// heavier in resource usage,
    ///
    /// unoptimised in this regard
    ///
    /// the input conductance is given on a per node or per control volume 
    /// basis (not total)
    #[inline]
    pub fn link_ambient_temperature_boundary_condition_laterally(&mut self,
        solid_array_to_ambient_nodal_conductance: ThermalConductance,
        ambient_temp: ThermodynamicTemperature,
        ) -> Result<(), ThermalHydraulicsLibError>{

        //
        // 1. we need to consider discretisation, ie how much 
        // power fraction
        let number_of_temperature_nodes = self.inner_nodes + 2;
        let q_fraction_per_node: f64 = 1.0/ number_of_temperature_nodes as f64;
        let mut q_frac_arr: Array1<f64> = Array::default(number_of_temperature_nodes);
        q_frac_arr.fill(q_fraction_per_node);

        // then get the ambient temperature 

        // lateral connections 
        {
            // first i will need to create temperature vectors 
            // for ambient temperature. This is for use in calculating 
            // heat loss from insulation to ambient air

            let mut ambient_temperature_vector: Vec<ThermodynamicTemperature> 
            = Array1::default(number_of_temperature_nodes)
                .iter().map( |&temp| {
                    temp
                }
                ).collect();

            ambient_temperature_vector.fill(ambient_temp);


            // for this process, I will make a clone of 
            // each HeatTransferEntity, modify the clone, then 
            // replace the HeatTransferEntity within the pipe using 
            // these changed entities

            let mut solid_array_clone: SolidColumn = 
                self.solid_array.clone().try_into()?;

            // second, fill them into the each array 
            
            // solid array to ambient interaction

            solid_array_clone.lateral_link_new_temperature_vector_avg_conductance(
                solid_array_to_ambient_nodal_conductance,
                ambient_temperature_vector
            )?;

            // solid array to power interaction
            solid_array_clone.lateral_link_new_power_vector(
                Power::ZERO,
                q_frac_arr
            )?;


            self.solid_array.set(solid_array_clone.into())?;

        }
        // axial connections (insulation by default)
        // you can of course add new ones

        self.zero_power_bc_connection()?;

        Ok(())

    }


    /// for insulated pipes
    /// the end of each node should have a zero power boundary condition 
    /// connected to each of them at the bare minimum
    ///
    /// this function does exactly that
    ///
    /// to connect the rest of the heat transfer entities, 
    /// use the link to front or back methods within the 
    /// FluidArray or SolidColumn
    #[inline]
    fn zero_power_bc_connection(&mut self) -> Result<(),ThermalHydraulicsLibError>{

        let zero_power: Power = Power::ZERO;

        let mut zero_power_bc: HeatTransferEntity = 
        HeatTransferEntity::BoundaryConditions(
            BCType::UserSpecifiedHeatAddition(zero_power)
        );

        // constant heat addition interaction 

        let interaction: HeatTransferInteractionType = 
        HeatTransferInteractionType::UserSpecifiedHeatAddition;

        // now connect the twisted tape 



        self.solid_array.link_to_front(&mut zero_power_bc,
            interaction)?;

        self.solid_array.link_to_back(&mut zero_power_bc,
            interaction)?;


        Ok(())
    }


    /// obtains ambient (usually air) to structure conductance
    ///
    /// this conductance assumes that we transfer heat to a cylinder 
    /// and that cylinder can be modelled using lumped capacitance model
    /// 
    /// So the thermal resistance within the cylinder is negligible
    ///
    /// Conductance is given on a per node basis
    #[inline]
    pub fn get_ambient_surroundings_to_cylinder_thermal_conductance(&mut self,
    h_air_to_pipe_surf: HeatTransfer,
    cylinder_diameter: Length,
    ambient_temp: ThermodynamicTemperature) 
        -> Result<ThermalConductance,ThermalHydraulicsLibError> {
        // first, let's get a clone of the pipe_shell shell surface
        let mut structure_clone: SolidColumn = 
        self.solid_array.clone().try_into()?;

        let number_of_temperature_nodes = self.inner_nodes + 2;
        let heated_length = self.strucutre_length;

        // next is to have pipe_shell inner conductance

        let cylinder_temperature: ThermodynamicTemperature 
        = structure_clone.try_get_bulk_temperature()?;

        let node_length = heated_length / 
            number_of_temperature_nodes as f64;

        // surface area per node laterally is PI * D * L
        let surface_area_per_node: Area = PI * cylinder_diameter * node_length;

        let pipe_air_conductance_interaction: HeatTransferInteractionType
        = HeatTransferInteractionType::UserSpecifiedConvectionResistance(
            DataUserSpecifiedConvectionResistance { 
                surf_area: surface_area_per_node.into(), 
                heat_transfer_coeff: h_air_to_pipe_surf,
            });

        let pipe_air_nodal_thermal_conductance: ThermalConductance = try_get_thermal_conductance_based_on_interaction(
            ambient_temp,
            cylinder_temperature,
            structure_clone.pressure_control_volume,
            structure_clone.pressure_control_volume,
            pipe_air_conductance_interaction,
        )?;


        return Ok(pipe_air_nodal_thermal_conductance);
    }


    /// obtains ambient (usually air) to structure conductance
    ///
    /// it goes roughly to the middle of the hollow cylinder
    ///
    /// Conductance is given on a per node basis
    #[inline]
    pub fn get_ambient_surroundings_to_hollow_cylinder_thermal_conductance(&mut self,
    h_air_to_pipe_surf: HeatTransfer,
    cylinder_id: Length,
    cylinder_od: Length,
    ambient_temp: ThermodynamicTemperature) 
        -> Result<ThermalConductance,ThermalHydraulicsLibError> {
        // first, let's get a clone of the pipe_shell shell surface
        let mut structure_clone: SolidColumn = 
        self.solid_array.clone().try_into()?;

        let number_of_temperature_nodes = self.inner_nodes + 2;
        let heated_length = self.strucutre_length;

        // next is to have pipe_shell inner conductance

        let insulation_shell_temperature: ThermodynamicTemperature 
        = structure_clone.try_get_bulk_temperature()?;

        let cylinder_mid_diameter: Length = 0.5*(cylinder_id+cylinder_od);


        let node_length = heated_length / 
            number_of_temperature_nodes as f64;

        let pipe_air_conductance_interaction: HeatTransferInteractionType
        = HeatTransferInteractionType::
            CylindricalConductionConvectionLiquidOutside(
                (structure_clone.material_control_volume, 
                    (cylinder_od-cylinder_mid_diameter).into(),
                    insulation_shell_temperature,
                    structure_clone.pressure_control_volume),
                (h_air_to_pipe_surf,
                    cylinder_od.into(),
                    node_length.into())
            );

        let pipe_air_nodal_thermal_conductance: ThermalConductance = try_get_thermal_conductance_based_on_interaction(
            ambient_temp,
            insulation_shell_temperature,
            structure_clone.pressure_control_volume,
            structure_clone.pressure_control_volume,
            pipe_air_conductance_interaction,
        )?;


        return Ok(pipe_air_nodal_thermal_conductance);
    }






    /// spawns a thread and moves the clone of the entire heater object into the 
    /// thread, "locking" it for parallel computation
    ///
    /// once that is done, the join handle is returned 
    /// which when unwrapped, returns the heater object
    pub fn lateral_connection_thread_spawn(&self,
        thermal_conductance_to_ambient: ThermalConductance,
        ambient_temp: ThermodynamicTemperature,
        power_input_into_column: Power) -> JoinHandle<Self>{

        let mut heater_clone = self.clone();

        // move ptr into a new thread 

        let join_handle = thread::spawn(
            move || -> Self {

                // carry out the connection calculations
                heater_clone.
                    link_mixed_boundary_condition_laterally(
                        thermal_conductance_to_ambient,
                        ambient_temp,
                        power_input_into_column).unwrap();
                
                heater_clone

            }
        );

        return join_handle;

    }


}
