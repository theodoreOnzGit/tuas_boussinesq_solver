use std::thread::JoinHandle;
use std::thread;

use uom::ConstZero;
use uom::si::pressure::atmosphere;
use uom::si::f64::*;
use ndarray::*;
use super::NonInsulatedParallelFluidComponent;
use crate::pre_built_components::heat_transfer_entities::preprocessing::try_get_thermal_conductance_based_on_interaction;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::boundary_conditions::BCType;
use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;

use crate::thermal_hydraulics_error::TuasLibError;

// preprocessing is where heat transfer entities 
// are connected to each other whether axially or laterally
//
// it is important to pay attention to the code here because it contains 
// logic of how heat transfer entites interact with the arrayCVs within 
// this parallel fluid component 
//
// first, we want a function to connect the components via heat transfer 
// interactions axially
//
// then we need to connect the parallel fluid arrays laterally to some 
// boundary condition
impl NonInsulatedParallelFluidComponent {


    /// wrapper function
    /// used to connect the arrays laterally 
    /// you'll need to set the mass flowrate and heater power
    ///
    /// heater power is the heat input into the solid part of the 
    /// pipe. Set to zero if the pipe is unheated
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
    /// no wall correction here
    #[inline]
    pub fn lateral_and_miscellaneous_connections_no_wall_correction(&mut self,
        mass_flowrate_over_all_tubes: MassRate,
        heater_power_over_all_tubes: Power) -> Result<(), TuasLibError>{

        let correct_prandtl_for_wall_temperatures = false;
        self.lateral_and_miscellaneous_connections(
            mass_flowrate_over_all_tubes, 
            heater_power_over_all_tubes, 
            correct_prandtl_for_wall_temperatures)
    }

    /// wrapper function
    /// used to connect the arrays laterally 
    /// you'll need to set the mass flowrate and heater power
    ///
    /// heater power is the heat input into the solid part of the 
    /// pipe. Set to zero if the pipe is unheated
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
    /// wall correction here
    #[inline]
    pub fn lateral_and_miscellaneous_connections_wall_correction(&mut self,
        mass_flowrate_over_all_tubes: MassRate,
        heater_power_over_all_tubes: Power) -> Result<(), TuasLibError>{

        let correct_prandtl_for_wall_temperatures = true;
        self.lateral_and_miscellaneous_connections(
            mass_flowrate_over_all_tubes, 
            heater_power_over_all_tubes, 
            correct_prandtl_for_wall_temperatures)
    }

    /// used to connect the arrays laterally 
    /// you'll need to set the mass flowrate and heater power
    ///
    /// heater power is the heat input into the solid part of the 
    /// pipe. Set to zero if the pipe is unheated
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
    #[inline]
    pub fn lateral_and_miscellaneous_connections(&mut self,
        mass_flowrate_over_all_tubes: MassRate,
        heater_power_over_all_tubes: Power,
        correct_prandtl_for_wall_temperatures: bool
        ) -> Result<(), TuasLibError>{


        // first let's get all the conductances 
        let heat_transfer_to_ambient = self.heat_transfer_to_ambient;

        let single_pipe_shell_to_air_nodal_conductance: ThermalConductance 
        = self.get_air_to_single_shell_nodal_shell_conductance(
            heat_transfer_to_ambient
        )?;

        self.set_mass_flowrate(mass_flowrate_over_all_tubes);

        let single_pipe_shell_surf_to_fluid_conductance: ThermalConductance 
        = self.get_single_tube_fluid_array_node_pipe_shell_conductance(
            correct_prandtl_for_wall_temperatures)?;


        // temperature array and fraction of 
        // heat transfer array for each of the nodes
        let number_of_temperature_nodes = self.inner_nodes + 2;
        let q_fraction_per_node: f64 = 1.0/ number_of_temperature_nodes as f64;
        let mut q_frac_arr: Array1<f64> = Array::default(number_of_temperature_nodes);
        q_frac_arr.fill(q_fraction_per_node);

        // then get the ambient temperature 

        let ambient_air_temp = self.ambient_temperature;

        // lateral connections 
        {
            // first i will need to create temperature vectors 

            let mut ambient_temperature_vector: Vec<ThermodynamicTemperature> 
            = Array1::default(number_of_temperature_nodes)
                .iter().map( |&temp| {
                    temp
                }
                ).collect();

            ambient_temperature_vector.fill(ambient_air_temp);


            // clone each array and set them later

            let mut pipe_shell_clone: SolidColumn = 
            self.pipe_shell.clone().try_into()?;

            let mut fluid_array_clone: FluidArray = 
            self.pipe_fluid_array.clone().try_into()?;


            // temperature vectors

            let pipe_temp_vector: Vec<ThermodynamicTemperature> 
            = pipe_shell_clone.get_temperature_vector()?;

            let fluid_temp_vector: Vec<ThermodynamicTemperature> 
            = fluid_array_clone.get_temperature_vector()?;

            // second, fill them into the each array 
            
            // pipe to air interaction

            pipe_shell_clone.lateral_link_new_temperature_vector_avg_conductance(
                single_pipe_shell_to_air_nodal_conductance,
                ambient_temperature_vector
            )?;

            // pipe shell to fluid interaction

            pipe_shell_clone.lateral_link_new_temperature_vector_avg_conductance(
                single_pipe_shell_surf_to_fluid_conductance,
                fluid_temp_vector.clone()
            )?;

            fluid_array_clone.lateral_link_new_temperature_vector_avg_conductance(
                single_pipe_shell_surf_to_fluid_conductance,
                pipe_temp_vector
            )?;

            // we also want to add a heat source to steel shell
            // for parallel treatment I divide heater power over all tubes 
            // by number of tubes 

            let heater_power_over_single_tube = 
                heater_power_over_all_tubes 
                / (self.number_of_tubes as f64);

            pipe_shell_clone.lateral_link_new_power_vector(
                heater_power_over_single_tube,
                q_frac_arr
            )?;


            // now that lateral connections are done, 
            // modify the heat transfer entity 

            self.pipe_fluid_array.set(fluid_array_clone.into())?;

            self.pipe_shell.set(pipe_shell_clone.into())?;



        }
        // axial connections 

        self.zero_power_bc_axial_connection()?;

        Ok(())

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
    fn zero_power_bc_axial_connection(&mut self) -> Result<(),TuasLibError>{

        let zero_power: Power = Power::ZERO;

        let mut zero_power_bc: HeatTransferEntity = 
        HeatTransferEntity::BoundaryConditions(
            BCType::UserSpecifiedHeatAddition(zero_power)
        );

        // constant heat addition interaction 

        let interaction: HeatTransferInteractionType = 
        HeatTransferInteractionType::UserSpecifiedHeatAddition;

        // now connect the fluid arrays and pipe shells


        self.pipe_fluid_array.link_to_front(&mut zero_power_bc,
            interaction)?;

        self.pipe_fluid_array.link_to_back(&mut zero_power_bc,
            interaction)?;

        self.pipe_shell.link_to_front(&mut zero_power_bc,
            interaction)?;

        self.pipe_shell.link_to_back(&mut zero_power_bc,
            interaction)?;


        Ok(())
    }




    /// obtains air to pipe shell conductance
    #[inline]
    pub fn get_air_to_single_shell_nodal_shell_conductance(&mut self,
    h_air_to_pipe_surf: HeatTransfer) 
        -> Result<ThermalConductance,TuasLibError> {
        // first, let's get a clone of the pipe shell surface
        let mut pipe_shell_clone: SolidColumn = 
        self.pipe_shell.clone().try_into()?;

        let number_of_temperature_nodes = self.inner_nodes + 2;
        let heated_length = self.get_component_length();
        let id = self.single_tube_id;
        let od = self.single_tube_od;

        // next is to have pipe shell inner conductance

        let pipe_surf_temperature: ThermodynamicTemperature 
        = pipe_shell_clone.try_get_bulk_temperature()?;

        let cylinder_mid_diameter: Length = 0.5*(id+od);


        let node_length = heated_length / 
            number_of_temperature_nodes as f64;

        let pipe_air_conductance_interaction: HeatTransferInteractionType
        = HeatTransferInteractionType::
            CylindricalConductionConvectionLiquidOutside(
                (pipe_shell_clone.material_control_volume, 
                    (od-cylinder_mid_diameter).into(),
                    pipe_surf_temperature,
                    pipe_shell_clone.pressure_control_volume),
                (h_air_to_pipe_surf,
                    od.into(),
                    node_length.into())
            );

        let pipe_air_nodal_thermal_conductance: ThermalConductance = try_get_thermal_conductance_based_on_interaction(
            self.ambient_temperature,
            pipe_surf_temperature,
            pipe_shell_clone.pressure_control_volume,
            pipe_shell_clone.pressure_control_volume,
            pipe_air_conductance_interaction,
        )?;


        return Ok(pipe_air_nodal_thermal_conductance);
    }


    /// obtains fluid to pipe shell conductance
    /// gives user an option on whether or not to turn on  
    /// the wall correction factor
    #[inline]
    pub fn get_single_tube_fluid_array_node_pipe_shell_conductance(&mut self,
        correct_prandtl_for_wall_temperatures: bool) 
        -> Result<ThermalConductance,TuasLibError> {

        // the thermal conductance here should be based on the 
        // nusselt number correlation

        // before any calculations, I will first need a clone of 
        // the fluid array and twisted tape array
        let mut fluid_array_clone: FluidArray = 
        self.pipe_fluid_array.clone().try_into()?;

        let mut pipe_shell_clone: SolidColumn = 
        self.pipe_shell.clone().try_into()?;

        // also need to get basic tmeperatures and mass flowrates 
        // only do this once because some of these methods involve 
        // cloning, which is computationally expensive

        let single_tube_mass_flowrate: MassRate = 
        fluid_array_clone.get_mass_flowrate();

        let fluid_temperature: ThermodynamicTemperature 
        = fluid_array_clone.try_get_bulk_temperature()?;

        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);

        let pipe_shell_surf_temperature: ThermodynamicTemperature 
        = pipe_shell_clone.try_get_bulk_temperature()?;

        let single_tube_hydraulic_diameter = 
            self.get_hydraulic_diameter();
        let bundled_tubes_flow_area: Area = 
            self.get_cross_sectional_area_immutable();

        let single_tube_flow_area: Area = 
            bundled_tubes_flow_area / (self.number_of_tubes as f64);

        // firstly, reynolds 


        // flow area and hydraulic diameter are ok

        let fluid_material: LiquidMaterial
            = fluid_array_clone.material_control_volume.try_into()?;

        let solid_material: SolidMaterial 
            = pipe_shell_clone.material_control_volume.try_into()?;

        let viscosity: DynamicViscosity = 
            fluid_material.try_get_dynamic_viscosity(fluid_temperature)?;

        // need to convert hydraulic diameter to an equivalent 
        // spherical diameter
        //
        // but for now, I'm going to use Re and Nu using hydraulic diameter 
        // and live with it for the time being
        //
        let reynolds_number_single_tube: Ratio = 
            single_tube_mass_flowrate/
            single_tube_flow_area
            *single_tube_hydraulic_diameter / viscosity;

        // next, bulk prandtl number 

        let bulk_prandtl_number: Ratio 
        = fluid_material.try_get_prandtl_liquid(
            fluid_temperature,
            atmospheric_pressure
        )?;



        // no wall correction is done here
        let mut fluid_array: FluidArray 
            = self.pipe_fluid_array.clone().try_into()?;

        let nusselt_estimate: Ratio;

        if !correct_prandtl_for_wall_temperatures {
            nusselt_estimate = fluid_array.get_nusselt(
                reynolds_number_single_tube, 
                bulk_prandtl_number, 
                bulk_prandtl_number)?;
        } else {

            // in this case, we do wall correction, but must have a 
            // bounded Prandtl number approach for the wall
            // Firstly, get the shell temperature 

            let mut wall_temperature_estimate = pipe_shell_clone
                .try_get_bulk_temperature()?;

            // then ensure that the wall temperature is within 
            // the temperature bounds of the fluid properties
            if wall_temperature_estimate > fluid_material.max_temperature() {

                wall_temperature_estimate = fluid_material.max_temperature();

            } else if wall_temperature_estimate < fluid_material.min_temperature() {

                wall_temperature_estimate = fluid_material.min_temperature();

            }
            // then get the wall prandtl number

            let wall_prandtl_number: Ratio 
                = fluid_material.try_get_prandtl_liquid(
                    wall_temperature_estimate,
                    atmospheric_pressure
                )?;


            nusselt_estimate = fluid_array.get_nusselt(
                reynolds_number_single_tube, 
                bulk_prandtl_number, 
                wall_prandtl_number)?;

        }

        // I need to use Nusselt correlations present in this struct 
        //
        // no wall correction is done here
        //
        // it's hard coded to a tube 

        //let nusselt_estimate = 
        //    pipe_prandtl_reynolds_data.
        //    get_nusselt_for_developing_flow_bulk_fluid_prandtl()?;



        // now we can get the heat transfer coeff, 

        let h_to_fluid: HeatTransfer;

        let k_fluid_average: ThermalConductivity = 
        fluid_material.try_get_thermal_conductivity(
            fluid_temperature)?;

        h_to_fluid = nusselt_estimate * k_fluid_average / single_tube_hydraulic_diameter;


        // and then get the convective resistance
        let number_of_temperature_nodes = self.inner_nodes + 2;
        let heated_length = fluid_array_clone.get_component_length();
        let id = self.single_tube_id;
        let od = self.single_tube_od;


        let node_length = heated_length / 
            number_of_temperature_nodes as f64;


        // now I need to calculate resistance of the half length of the 
        // pipe shell, which is an annular cylinder

        let cylinder_mid_diameter: Length = 0.5*(id+od);

        



        let fluid_pipe_shell_conductance_interaction: HeatTransferInteractionType
        = HeatTransferInteractionType::
            CylindricalConductionConvectionLiquidInside(
                (solid_material.into(), 
                    (cylinder_mid_diameter - id).into(),
                    pipe_shell_surf_temperature,
                    atmospheric_pressure),
                (h_to_fluid,
                    id.into(),
                    node_length.into())
            );

        // now based on conductance interaction, 
        // we can obtain thermal conductance, the temperatures 
        // and pressures don't really matter
        //
        // this is because all the thermal conductance data 
        // has already been loaded into the thermal conductance 
        // interaction object

        let fluid_pipe_shell_nodal_thermal_conductance: ThermalConductance = 
            try_get_thermal_conductance_based_on_interaction(
                fluid_temperature,
                pipe_shell_surf_temperature,
                atmospheric_pressure,
                atmospheric_pressure,
                fluid_pipe_shell_conductance_interaction)?;


        return Ok(fluid_pipe_shell_nodal_thermal_conductance);
    }

    /// gets the reynolds number based on mass flworate and 
    /// temperature
    #[inline]
    pub fn get_reynolds_based_on_hydraulic_diameter_and_flow_area(
        &self,
        mass_flowrate: MassRate,
        temperature: ThermodynamicTemperature) -> Result<Ratio,TuasLibError> {

        // flow area and hydraulic diameter are ok
        let flow_area: Area = self.get_cross_sectional_area_immutable();
        let hydraulic_diameter = self.get_hydraulic_diameter_immutable();

        let fluid_array_clone: FluidArray = 
            self.pipe_fluid_array.clone().try_into()?;

        let fluid_material: LiquidMaterial
            = fluid_array_clone.material_control_volume.try_into()?;

        let viscosity: DynamicViscosity = 
            fluid_material.try_get_dynamic_viscosity(temperature)?;

        // need to convert hydraulic diameter to an equivalent 
        // spherical diameter
        //
        // but for now, I'm going to use Re and Nu using hydraulic diameter 
        // and live with it for the time being
        //
        let reynolds: Ratio = 
        mass_flowrate/flow_area*hydraulic_diameter / viscosity;

        Ok(reynolds)

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
                    lateral_and_miscellaneous_connections_no_wall_correction(
                        mass_flowrate,
                        heater_steady_state_power).unwrap();
                
                heater_clone

            }
        );

        return join_handle;

    }



    

}
