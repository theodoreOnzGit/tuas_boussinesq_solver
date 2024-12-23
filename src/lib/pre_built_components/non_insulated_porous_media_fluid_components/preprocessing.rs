use std::thread::JoinHandle;
use std::thread;

use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::boundary_conditions::BCType;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::tuas_lib_error::TuasLibError;

use super::NonInsulatedPorousMediaFluidComponent;
use uom::ConstZero;
use uom::si::pressure::atmosphere;
use uom::si::f64::*;
use ndarray::*;

impl NonInsulatedPorousMediaFluidComponent {

    /// NonInsulatedPorousMediaFluidComponent config:
    ///
    /// Firstly with insulation:
    /// |               |               |              |
    /// |               |               |              |
    /// |-porous media -|- shell fluid -|-outer shell--| ambient
    /// |               |               |              |
    /// |               |               |              |
    ///
    ///
    /// This connects the control volumes within this component 
    /// causing them to interact given a set mass flowrate
    #[inline]
    pub fn lateral_and_miscellaneous_connections(&mut self,
        prandtl_wall_correction_setting: bool,
        mass_flowrate: MassRate,
        shell_side_steady_state_power: Power,
        porous_media_side_steady_state_power: Power,
    ) -> Result<(), TuasLibError>{

        // first set the mass flowrate
        self.set_mass_flowrate(mass_flowrate);

        // then get conductances
        let heat_transfer_to_ambient = self.heat_transfer_to_ambient;

        let ambient_to_pipe_shell_nodal_conductance: ThermalConductance = 
            self.get_ambient_to_pipe_shell_nodal_conductance(
                heat_transfer_to_ambient)?;

        let pipe_shell_to_fluid_nodal_conductance: ThermalConductance
            = self.get_pipe_shell_to_fluid_nodal_conductance(
                prandtl_wall_correction_setting)?;

        let interior_to_fluid_nodal_conductance: ThermalConductance 
            = self.get_interior_to_fluid_nodal_conductance(
                prandtl_wall_correction_setting)?;

        // now that we have obtained the conductances, we then need to 
        // obtain temperature vectors and conductance vectors for  
        // each pipe array for the lateral connections

        let ambient_temp: ThermodynamicTemperature = self.ambient_temperature;
        let number_of_temperature_nodes = self.inner_nodes + 2;

        // now for the lateral linkages
        {
            // let's do the temperature vectors first 
            let mut ambient_temperature_vector: Vec<ThermodynamicTemperature>
                = Array1::default(number_of_temperature_nodes)
                .iter().map( |&temp| {
                    temp
                }
                ).collect();

            ambient_temperature_vector.fill(ambient_temp);
            let mut pipe_fluid_array_clone: FluidArray = 
                self.pipe_fluid_array.clone().try_into()?;

            let mut interior_solid_array_clone: SolidColumn = 
                self.interior_solid_array_for_porous_media.clone().try_into()?;
            let mut pipe_shell_clone: SolidColumn = 
                self.pipe_shell.clone().try_into()?;

            // let's get the temperature vectors

            let pipe_fluid_arr_temp_vec: Vec<ThermodynamicTemperature>
                = pipe_fluid_array_clone.get_temperature_vector()?;

            let pipe_shell_arr_temp_vec: Vec<ThermodynamicTemperature> 
                = pipe_shell_clone.get_temperature_vector()?;

            let interior_arr_temp_vec: Vec<ThermodynamicTemperature> 
                = interior_solid_array_clone.get_temperature_vector()?;
            // perform the inner connections 
            // for tube fluid to shell arr 
            //
            pipe_fluid_array_clone. 
                lateral_link_new_temperature_vector_avg_conductance(
                    pipe_shell_to_fluid_nodal_conductance, 
                    pipe_shell_arr_temp_vec)?;

            pipe_shell_clone.
                lateral_link_new_temperature_vector_avg_conductance(
                    pipe_shell_to_fluid_nodal_conductance, 
                    pipe_fluid_arr_temp_vec.clone())?;

            // next fluid array to interior 
            pipe_fluid_array_clone.
                lateral_link_new_temperature_vector_avg_conductance(
                    interior_to_fluid_nodal_conductance, 
                    interior_arr_temp_vec)?;

            interior_solid_array_clone.
                lateral_link_new_temperature_vector_avg_conductance(
                    interior_to_fluid_nodal_conductance, 
                    pipe_fluid_arr_temp_vec)?;

            // finally, pipe shell to ambient 

            pipe_shell_clone.
                lateral_link_new_temperature_vector_avg_conductance(
                    ambient_to_pipe_shell_nodal_conductance, 
                    ambient_temperature_vector)?;
            // after this, we are done for the internal connections

            // now, add power arrays
            // assume even power distribution (this can be changed 
            // in future)
            let number_of_temperature_nodes = self.inner_nodes + 2;
            let q_fraction_per_node: f64 = 1.0/ number_of_temperature_nodes as f64;
            let mut q_frac_arr: Array1<f64> = Array::default(number_of_temperature_nodes);
            q_frac_arr.fill(q_fraction_per_node);

            pipe_shell_clone.lateral_link_new_power_vector(
                shell_side_steady_state_power,
                q_frac_arr.clone()
            ).unwrap();

            interior_solid_array_clone.lateral_link_new_power_vector(
                porous_media_side_steady_state_power, 
                q_frac_arr
            ).unwrap();

            // now that lateral connections are done, 
            // for the outer shell, inner shell and 
            // both fluid arrays
            // modify the heat transfer entities

            self.pipe_shell.set(pipe_shell_clone.into())?;

            self.interior_solid_array_for_porous_media.set(
                interior_solid_array_clone.into())?;

            self.pipe_fluid_array
                .set(pipe_fluid_array_clone.into())?;


        }
        // axial connections (adiabatic by default, otherwise 
        // you'll have to add your own through the heat
        // transfer entitites eg. advection)
        self.zero_power_bc_connection();
        

        Ok(())

    }

    /// obtains the conductance from ambient to the pipe shell 
    /// nodally speaking 

    #[inline]
    pub fn get_ambient_to_pipe_shell_nodal_conductance(&mut self,
        heat_transfer_to_ambient: HeatTransfer) 
        -> Result<ThermalConductance,TuasLibError> {

            // the solid conductance is calculated using 
            // k (A/L) where L is representative thickness 
            // A is heat transfer area,
            // k is thermal conductivity
            //
            let solid_conductance_lengthscale: Length = 
                self.solid_side_thermal_conductance_lengthscale_pipe_to_ambient;
            
            // to calculate k, we need the bulk temperature 

            let mut pipe_shell_clone: SolidColumn = 
                self.pipe_shell.clone().try_into().unwrap();
            let pipe_bulk_temp: ThermodynamicTemperature = 
                pipe_shell_clone.try_get_bulk_temperature()?;

            // next, let's get the conductivity 

            let pipe_shell_material_conductivity: ThermalConductivity = 
                pipe_shell_clone.material_control_volume
                .try_get_thermal_conductivity(
                    pipe_bulk_temp)?;

            let number_of_nodes: f64 = self.inner_nodes as f64 + 2.0;
            // solid side nodalised thermal conductance

            let nodalised_solid_side_thermal_conductance: ThermalConductance
                = solid_conductance_lengthscale * pipe_shell_material_conductivity
                / number_of_nodes;

            let nodalised_solid_side_thermal_resistance: ThermalResistance 
                = nodalised_solid_side_thermal_conductance.recip();

            // next, nodalised thermal conductance due to liquid side 
            
            let nodalised_thermal_conductance_ambient_convection: ThermalConductance 
                = (heat_transfer_to_ambient * self.convection_heat_transfer_area_to_ambient)
                / number_of_nodes;

            let nodalised_ambient_convection_thermal_resistance: ThermalResistance 
                = nodalised_thermal_conductance_ambient_convection.recip();


            // add resistances together 
            let nodalised_pipe_shell_to_ambient_resistance: ThermalResistance 
                = nodalised_solid_side_thermal_resistance +
                nodalised_ambient_convection_thermal_resistance;

            // get conductance, and then return 

            let nodalised_pipe_shell_to_ambient_conductance 
                = nodalised_pipe_shell_to_ambient_resistance.recip();

            // and we done!
            return Ok(nodalised_pipe_shell_to_ambient_conductance);

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


    /// still needs to be tested
    pub fn get_interior_to_fluid_nodal_conductance(
        &self, prandtl_wall_correction_setting: bool) -> Result<ThermalConductance, TuasLibError>
    {
        // the thermal conductance here should be based on the 
        // nusselt number correlation

        // before any calculations, I will first need a clone of 
        // the fluid array and inner shell array
        //
        // the fluid array represents only a single tube
        let mut pipe_fluid_arr_clone: FluidArray = 
            self.pipe_fluid_array.clone().try_into()?;


        let mut interior_solid_array_clone: SolidColumn = 
            self.interior_solid_array_for_porous_media.clone().try_into()?;

        // also need to get basic temperatures and mass flowrates 
        // only do this once because some of these methods involve 
        // cloning, which is computationally expensive

        let single_tube_mass_flowrate: MassRate = 
            pipe_fluid_arr_clone.get_mass_flowrate();

        let fluid_temperature: ThermodynamicTemperature 
            = pipe_fluid_arr_clone.try_get_bulk_temperature()?;

        let solid_array_temp: ThermodynamicTemperature 
            = interior_solid_array_clone.try_get_bulk_temperature()?;

        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);


        let single_tube_hydraulic_diameter = 
            self.get_hydraulic_diameter_immutable();
        let single_tube_flow_area: Area = 
            pipe_fluid_arr_clone.get_cross_sectional_area_immutable();

        // flow area and hydraulic diameter are ok


        let fluid_material: LiquidMaterial
            = pipe_fluid_arr_clone.material_control_volume.try_into()?;

        let solid_material: SolidMaterial 
            = interior_solid_array_clone.material_control_volume.try_into()?;

        let viscosity: DynamicViscosity = 
            fluid_material.try_get_dynamic_viscosity(fluid_temperature)?;

        let solid_thermal_conductivity: ThermalConductivity = 
            solid_material.try_get_thermal_conductivity(
                solid_array_temp)?;

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

        // the reynolds number here is used for nusselt number estimates 
        // so I'm going to have an aboslute value of reynolds number 
        // for nusselt estimates
        
        let reynolds_number_abs_for_nusselt: Ratio = 
            reynolds_number_single_tube.abs();

        // next, bulk prandtl number 

        let bulk_prandtl_number: Ratio 
            = fluid_material.try_get_prandtl_liquid(
                fluid_temperature,
                atmospheric_pressure
            )?;

        // here's where we account for wall prandtl setting 
        // if ever an issue
        let correct_prandtl_for_wall_temperatures = 
            prandtl_wall_correction_setting;

        let mut wall_prandtl_number = bulk_prandtl_number;

        if correct_prandtl_for_wall_temperatures {

            // then wall prandtl number
            //
            // wall prandtl number will likely be out of range as the 
            // wall temperature is quite different from bulk fluid 
            // temperature. May be  out of correlation range
            // if that is the case, then just go for a partial correction
            // temperature of the range or go for the lowest temperature 
            // possible

            // The method I use is to just use the wall prandtl number 
            // if the number falls outside the range of correlations,
            // then use the prandtl number at the max or min 

            let mut wall_temperature_estimate = solid_array_temp;

            if wall_temperature_estimate > fluid_material.max_temperature() {

                wall_temperature_estimate = fluid_material.max_temperature();

            } else if wall_temperature_estimate < fluid_material.min_temperature() {

                wall_temperature_estimate = fluid_material.min_temperature();

            }

            wall_prandtl_number = fluid_material.try_get_prandtl_liquid(
                wall_temperature_estimate,
                atmospheric_pressure
            )?;

        }


        // I need to use Nusselt correlations present in this struct 
        //
        // wall correction is optionally done here
        //
        // for tubes,
        // the gnielinski correlation should be used as it 
        // is for tubes and pipes.
        //
        // but I allow the user to set the nusselt correlation 

        // now, for gnielinski type correlations, we require the 
        // darcy friction factor
        //
        // However, the darcy friction factor for other components 
        // will come in the form:
        //
        // (f_darcy L/D + K)
        //
        // the next best thing we can get is:
        //
        // (f_darcy + D/L  K)

        // (f_darcy L/D + K)
        let fldk: Ratio = 
            pipe_fluid_arr_clone
            .fluid_component_loss_properties
            .fldk_based_on_darcy_friction_factor(reynolds_number_abs_for_nusselt)
            .unwrap();

        let length_to_diameter: Ratio = 
            pipe_fluid_arr_clone.get_component_length_immutable()/
            pipe_fluid_arr_clone.get_hydraulic_diameter_immutable();
        // (f_darcy + D/L  K)
        // then let's scale it by length to diameter 
        let modified_darcy_friction_factor: Ratio = 
            fldk/length_to_diameter;

        let nusselt_estimate_tube_side = 
            self.nusselt_correlation_to_pipe_shell
            .estimate_based_on_prandtl_darcy_and_reynolds_wall_correction(
                bulk_prandtl_number, 
                wall_prandtl_number, 
                modified_darcy_friction_factor,
                reynolds_number_abs_for_nusselt)?;

        // now we can get the heat transfer coeff, 

        let tube_h_to_fluid: HeatTransfer;

        let k_fluid_average: ThermalConductivity = 
            fluid_material.try_get_thermal_conductivity(
                fluid_temperature)?;

        tube_h_to_fluid = nusselt_estimate_tube_side * k_fluid_average / single_tube_hydraulic_diameter;

        // and then get the convective resistance
        let number_of_temperature_nodes = self.inner_nodes as f64 + 2.0;
        // now we can get the nodalised conductance 

        let nodalised_fluid_side_conductance: ThermalConductance 
            = (tube_h_to_fluid * self.convection_heat_transfer_area_to_pipe)
            / number_of_temperature_nodes;

        
        // now solid side nodalised conductance 
        let nodalised_solid_side_conductance: ThermalConductance 
            = (self.solid_side_thermal_conductance_lengthscale_fluid_to_porous_media_internal * 
                solid_thermal_conductivity) / number_of_temperature_nodes;

        let nodalised_pipe_fluid_to_shell_thermal_resistance: ThermalResistance 
            = nodalised_solid_side_conductance.recip() 
            + nodalised_fluid_side_conductance.recip();

        // return the conductance 

        return Ok(nodalised_pipe_fluid_to_shell_thermal_resistance.recip());
    }

    /// still work in progress... yet to be tested
    pub fn get_pipe_shell_to_fluid_nodal_conductance(
        &self, prandtl_wall_correction_setting: bool) -> Result<ThermalConductance, TuasLibError>
    {
        // the thermal conductance here should be based on the 
        // nusselt number correlation

        // before any calculations, I will first need a clone of 
        // the fluid array and inner shell array
        //
        // the fluid array represents only a single tube
        let mut pipe_fluid_arr_clone: FluidArray = 
            self.pipe_fluid_array.clone().try_into()?;


        let mut pipe_shell_clone: SolidColumn = 
            self.pipe_shell.clone().try_into()?;

        // also need to get basic temperatures and mass flowrates 
        // only do this once because some of these methods involve 
        // cloning, which is computationally expensive

        let mass_flowrate: MassRate = 
            pipe_fluid_arr_clone.get_mass_flowrate();

        let fluid_temperature: ThermodynamicTemperature 
            = pipe_fluid_arr_clone.try_get_bulk_temperature()?;

        let wall_temperature: ThermodynamicTemperature 
            = pipe_shell_clone.try_get_bulk_temperature()?;

        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);


        let nusselt_hydraulic_diameter = 
            self.nusselt_correlation_lengthscale_to_ambient;
        let flow_area: Area = 
            pipe_fluid_arr_clone.get_cross_sectional_area_immutable();

        // flow area and hydraulic diameter are ok


        let fluid_material: LiquidMaterial
            = pipe_fluid_arr_clone.material_control_volume.try_into()?;

        let solid_material: SolidMaterial 
            = pipe_shell_clone.material_control_volume.try_into()?;

        let viscosity: DynamicViscosity = 
            fluid_material.try_get_dynamic_viscosity(fluid_temperature)?;

        let solid_thermal_conductivity: ThermalConductivity = 
            solid_material.try_get_thermal_conductivity(
                wall_temperature)?;

        // need to convert hydraulic diameter to an equivalent 
        // spherical diameter
        //
        // but for now, I'm going to use Re and Nu using hydraulic diameter 
        // and live with it for the time being
        //
        let reynolds_number_single_tube: Ratio = 
            mass_flowrate/
            flow_area
            *nusselt_hydraulic_diameter / viscosity;

        // the reynolds number here is used for nusselt number estimates 
        // so I'm going to have an aboslute value of reynolds number 
        // for nusselt estimates
        
        let reynolds_number_abs_for_nusselt: Ratio = 
            reynolds_number_single_tube.abs();

        // next, bulk prandtl number 

        let bulk_prandtl_number: Ratio 
            = fluid_material.try_get_prandtl_liquid(
                fluid_temperature,
                atmospheric_pressure
            )?;

        // here's where we account for wall prandtl setting 
        // if ever an issue
        let correct_prandtl_for_wall_temperatures = 
            prandtl_wall_correction_setting;

        let mut wall_prandtl_number = bulk_prandtl_number;

        if correct_prandtl_for_wall_temperatures {

            // then wall prandtl number
            //
            // wall prandtl number will likely be out of range as the 
            // wall temperature is quite different from bulk fluid 
            // temperature. May be  out of correlation range
            // if that is the case, then just go for a partial correction
            // temperature of the range or go for the lowest temperature 
            // possible

            // The method I use is to just use the wall prandtl number 
            // if the number falls outside the range of correlations,
            // then use the prandtl number at the max or min 

            let mut wall_temperature_estimate = wall_temperature;

            if wall_temperature_estimate > fluid_material.max_temperature() {

                wall_temperature_estimate = fluid_material.max_temperature();

            } else if wall_temperature_estimate < fluid_material.min_temperature() {

                wall_temperature_estimate = fluid_material.min_temperature();

            }

            wall_prandtl_number = fluid_material.try_get_prandtl_liquid(
                wall_temperature_estimate,
                atmospheric_pressure
            )?;

        }


        // I need to use Nusselt correlations present in this struct 
        //
        // wall correction is optionally done here
        //
        // for tubes,
        // the gnielinski correlation should be used as it 
        // is for tubes and pipes.
        //
        // but I allow the user to set the nusselt correlation 

        // now, for gnielinski type correlations, we require the 
        // darcy friction factor
        //
        // However, the darcy friction factor for other components 
        // will come in the form:
        //
        // (f_darcy L/D + K)
        //
        // the next best thing we can get is:
        //
        // (f_darcy + D/L  K)

        // (f_darcy L/D + K)
        let fldk: Ratio = 
            pipe_fluid_arr_clone
            .fluid_component_loss_properties
            .fldk_based_on_darcy_friction_factor(reynolds_number_abs_for_nusselt)
            .unwrap();

        let length_to_diameter: Ratio = 
            pipe_fluid_arr_clone.get_component_length_immutable()/
            pipe_fluid_arr_clone.get_hydraulic_diameter_immutable();
        // (f_darcy + D/L  K)
        // then let's scale it by length to diameter 
        let modified_darcy_friction_factor: Ratio = 
            fldk/length_to_diameter;

        let nusselt_estimate_tube_side = 
            self.nusselt_correlation_to_pipe_shell
            .estimate_based_on_prandtl_darcy_and_reynolds_wall_correction(
                bulk_prandtl_number, 
                wall_prandtl_number, 
                modified_darcy_friction_factor,
                reynolds_number_abs_for_nusselt)?;

        // now we can get the heat transfer coeff, 

        let tube_h_to_fluid: HeatTransfer;

        let k_fluid_average: ThermalConductivity = 
            fluid_material.try_get_thermal_conductivity(
                fluid_temperature)?;

        tube_h_to_fluid = nusselt_estimate_tube_side * k_fluid_average / nusselt_hydraulic_diameter;

        // and then get the convective resistance
        let number_of_temperature_nodes = self.inner_nodes as f64 + 2.0;
        // now we can get the nodalised conductance 

        let nodalised_fluid_side_conductance: ThermalConductance 
            = (tube_h_to_fluid * self.convection_heat_transfer_area_to_pipe)
            / number_of_temperature_nodes;

        
        // now solid side nodalised conductance 
        let nodalised_solid_side_conductance: ThermalConductance 
            = (self.solid_side_thermal_conductance_lengthscale_pipe_to_fluid * 
                solid_thermal_conductivity) / number_of_temperature_nodes;

        let nodalised_pipe_fluid_to_shell_thermal_resistance: ThermalResistance 
            = nodalised_solid_side_conductance.recip() 
            + nodalised_fluid_side_conductance.recip();

        // return the conductance 

        return Ok(nodalised_pipe_fluid_to_shell_thermal_resistance.recip());
    }


    /// spawns a thread and moves the clone of the entire heater object into the 
    /// thread, "locking" it for parallel computation
    ///
    /// once that is done, the join handle is returned 
    /// which when unwrapped, returns the heater object
    pub fn lateral_connection_thread_spawn(&self,
        prandtl_wall_correction_setting: bool,
        mass_flowrate: MassRate,
        shell_side_steady_state_power: Power,
        porous_media_side_steady_state_power: Power) -> JoinHandle<Self>{

        let mut heater_clone = self.clone();

        // move ptr into a new thread 

        let join_handle = thread::spawn(
            move || -> Self {

                // carry out the connection calculations
                heater_clone.
                    lateral_and_miscellaneous_connections(
                        prandtl_wall_correction_setting,
                        mass_flowrate,
                        shell_side_steady_state_power,
                        porous_media_side_steady_state_power
                    ).unwrap();
                
                heater_clone

            }
        );

        return join_handle;

    }


}



/// contains preprocessing functions specifically for 
/// ciet heater v2
pub mod ciet_heater_v2;
