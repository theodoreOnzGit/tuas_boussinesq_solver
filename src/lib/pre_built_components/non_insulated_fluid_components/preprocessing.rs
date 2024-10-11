use std::thread::JoinHandle;
use std::thread;

use uom::ConstZero;
use uom::si::pressure::atmosphere;
use uom::si::f64::*;
use ndarray::*;
use super::NonInsulatedFluidComponent;
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

impl NonInsulatedFluidComponent {


    /// used to connect the arrays laterally 
    /// you'll need to set the mass flowrate and heater power
    ///
    /// note that this is a wrapper function, here because of 
    /// coding legacy reasons
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
    /// 
    #[inline]
    pub fn lateral_and_miscellaneous_connections_no_wall_correction(&mut self,
        mass_flowrate: MassRate,
        heater_power: Power) -> Result<(), TuasLibError>{

        let correct_prandtl_for_wall_temperatures = false;

        self.lateral_and_miscellaneous_connections(
            mass_flowrate, 
            heater_power, 
            correct_prandtl_for_wall_temperatures)
    }

    /// used to connect the arrays laterally 
    /// you'll need to set the mass flowrate and heater power
    ///
    /// note that this is a wrapper function, here partly because of 
    /// coding legacy reasons. Because if i have a no_wall_correction 
    /// code, there should be one for wall correction also
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
    /// 
    #[inline]
    pub fn lateral_and_miscellaneous_connections_wall_correction(&mut self,
        mass_flowrate: MassRate,
        heater_power: Power) -> Result<(), TuasLibError>{

        let correct_prandtl_for_wall_temperatures = true;

        self.lateral_and_miscellaneous_connections(
            mass_flowrate, 
            heater_power, 
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
    /// you will also need to manually set the wall correction setting 
    #[inline]
    pub fn lateral_and_miscellaneous_connections(&mut self,
        mass_flowrate: MassRate,
        heater_power: Power,
        correct_prandtl_for_wall_temperatures: bool) -> Result<(), TuasLibError>{


        // first let's get all the conductances 
        let heat_transfer_to_ambient = self.heat_transfer_to_ambient;

        let pipe_shell_to_air_nodal_conductance: ThermalConductance 
        = self.get_air_shell_nodal_shell_conductance(
            heat_transfer_to_ambient
        )?;

        self.set_mass_flowrate(mass_flowrate);


        let pipe_shell_surf_to_fluid_conductance: ThermalConductance 
        = self.get_fluid_array_node_pipe_shell_conductance(
            correct_prandtl_for_wall_temperatures)?;


        // other stuff 
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


            // note, must set mass flowrate first 
            // otherwise there is by default zero flow through 
            // the array

            fluid_array_clone.set_mass_flowrate(
                mass_flowrate);

            // temperature vectors

            let pipe_temp_vector: Vec<ThermodynamicTemperature> 
            = pipe_shell_clone.get_temperature_vector()?;

            let fluid_temp_vector: Vec<ThermodynamicTemperature> 
            = fluid_array_clone.get_temperature_vector()?;

            // second, fill them into the each array 
            
            // pipe to air interaction

            pipe_shell_clone.lateral_link_new_temperature_vector_avg_conductance(
                pipe_shell_to_air_nodal_conductance,
                ambient_temperature_vector
            )?;

            // pipe shell to fluid interaction

            pipe_shell_clone.lateral_link_new_temperature_vector_avg_conductance(
                pipe_shell_surf_to_fluid_conductance,
                fluid_temp_vector.clone()
            )?;

            fluid_array_clone.lateral_link_new_temperature_vector_avg_conductance(
                pipe_shell_surf_to_fluid_conductance,
                pipe_temp_vector
            )?;

            // we also want to add a heat source to steel shell

            pipe_shell_clone.lateral_link_new_power_vector(
                heater_power,
                q_frac_arr
            )?;

            // now fluid to twisted tape interaction
            

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

        // now connect the twisted tape 


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
    pub fn get_air_shell_nodal_shell_conductance(&mut self,
    h_air_to_pipe_surf: HeatTransfer) 
        -> Result<ThermalConductance,TuasLibError> {
        // first, let's get a clone of the pipe shell surface
        let mut pipe_shell_clone: SolidColumn = 
        self.pipe_shell.clone().try_into()?;

        let number_of_temperature_nodes = self.inner_nodes + 2;
        let heated_length = self.get_component_length();
        let id = self.id;
        let od = self.od;

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


    /// obtains fluid to pipe  shell conductance
    #[inline]
    pub fn get_fluid_array_node_pipe_shell_conductance(
        &mut self, 
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

        let mass_flowrate: MassRate = 
        fluid_array_clone.get_mass_flowrate();

        let fluid_temperature: ThermodynamicTemperature 
        = fluid_array_clone.try_get_bulk_temperature()?;

        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);

        let pipe_shell_surf_temperature: ThermodynamicTemperature 
        = pipe_shell_clone.try_get_bulk_temperature()?;

        let hydraulic_diameter = self.get_hydraulic_diameter();
        let flow_area: Area = self.get_cross_sectional_area_immutable();

        // firstly, reynolds 


        // flow area and hydraulic diameter are ok

        let mut fluid_array_clone: FluidArray = 
            self.pipe_fluid_array.clone().try_into()?;

        let fluid_material: LiquidMaterial
            = fluid_array_clone.material_control_volume.try_into()?;

        let viscosity: DynamicViscosity = 
            fluid_material.try_get_dynamic_viscosity(fluid_temperature)?;

        // need to convert hydraulic diameter to an equivalent 
        // spherical diameter
        //
        // but for now, I'm going to use Re and Nu using hydraulic diameter 
        // and live with it for the time being
        //
        let reynolds_number: Ratio = 
            mass_flowrate/flow_area*hydraulic_diameter / viscosity;

        // next, bulk prandtl number 

        let bulk_prandtl_number: Ratio 
        = fluid_material.try_get_prandtl_liquid(
            fluid_temperature,
            atmospheric_pressure
        )?;





        // I need to use Nusselt correlations present in this struct 
        let mut fluid_array: FluidArray 
            = self.pipe_fluid_array.clone().try_into()?;


        // now let's estimate the nusselt number, depending on whether 
        // there is wall correction

        let nusselt_estimate: Ratio;

        if !correct_prandtl_for_wall_temperatures {
            nusselt_estimate = fluid_array.get_nusselt(
                reynolds_number, 
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
                reynolds_number, 
                bulk_prandtl_number, 
                wall_prandtl_number)?;

        }



        // now we can get the heat transfer coeff, 

        let h_to_fluid: HeatTransfer;

        let k_fluid_average: ThermalConductivity = 
        fluid_material.try_get_thermal_conductivity(
            fluid_temperature)?;

        h_to_fluid = nusselt_estimate * k_fluid_average / hydraulic_diameter;


        // and then get the convective resistance
        let number_of_temperature_nodes = self.inner_nodes + 2;
        let heated_length = fluid_array_clone.get_component_length();
        let id = self.id;
        let od = self.od;


        let node_length = heated_length / 
            number_of_temperature_nodes as f64;


        // now I need to calculate resistance of the half length of the 
        // pipe shell, which is an annular cylinder

        let cylinder_mid_diameter: Length = 0.5*(id+od);



        let fluid_pipe_shell_conductance_interaction: HeatTransferInteractionType
        = HeatTransferInteractionType::
            CylindricalConductionConvectionLiquidInside(
                (SolidMaterial::SteelSS304L.into(), 
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
