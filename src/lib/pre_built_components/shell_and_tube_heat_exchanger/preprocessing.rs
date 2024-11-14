use std::thread::JoinHandle;
use std::thread;

use uom::si::thermodynamic_temperature::kelvin;
use uom::ConstZero;
use uom::si::pressure::atmosphere;
use uom::si::f64::*;
use ndarray::*;
use super::SimpleShellAndTubeHeatExchanger;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component::FluidComponent;
use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
use crate::heat_transfer_correlations::thermal_resistance::try_get_thermal_conductance_annular_cylinder;
use crate::{heat_transfer_correlations::nusselt_number_correlations::input_structs::GnielinskiData, pre_built_components::heat_transfer_entities::preprocessing::try_get_thermal_conductance_based_on_interaction};
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::boundary_conditions::BCType;
use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;

use crate::tuas_lib_error::TuasLibError;

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
//
// If you want to set this in countercurrent mode, ensure that 
// the mass flowrates are going in opposite sides, otherwise,
// it will be in co-current mode
//
impl SimpleShellAndTubeHeatExchanger {

    /// The shell and tube heat exchanger has two configurations,
    ///
    /// Firstly with insulation:
    /// |            |            |               |             |            |
    /// |            |            |               |             |            |
    /// |-tube fluid-|-inner tube-|- shell fluid -|-outer shell-|-insulation-| ambient
    /// |            |            |               |             |            |
    /// |            |            |               |             |            |
    ///
    /// Secondly, without insulation
    ///
    /// |            |            |               |             |
    /// |            |            |               |             |
    /// |-tube fluid-|-inner tube-|- shell fluid -|-outer shell-| ambient
    /// |            |            |               |             |
    /// |            |            |               |             |
    ///
    /// This setting is toggled on or off depending on the 
    /// self.heat_exchanger_has_insulation variable, which should be set 
    /// when you construct this struct
    ///
    ///
    ///
    #[inline]
    pub fn lateral_and_miscellaneous_connections(&mut self,
        prandtl_wall_correction_setting: bool,
        tube_side_total_mass_flowrate: MassRate,
        shell_side_total_mass_flowrate: MassRate,
    ) -> Result<(), TuasLibError>
    {
        // set the mass flowrates first on shell and tube side
        self.set_tube_side_total_mass_flowrate(tube_side_total_mass_flowrate);
        self.set_shell_side_total_mass_flowrate(shell_side_total_mass_flowrate);

        // first let's get all the conductances 
        let heat_transfer_to_ambient = self.heat_transfer_to_ambient;

        // note that this outer node layer depends on whether 
        // insulation is toggled on by the user 
        //
        // if it is toggled on, then the outer layer is the insulation 
        // if it is toggled off, then the outer layer is the outer 
        // metallic shell
        let outer_node_layer_to_air_conductance = 
            self.get_air_to_outer_sthe_layer_nodal_conductance(
                heat_transfer_to_ambient)?;

        // we will only calculate the insulation to outer shell 
        // conductance if we toggled that this heat exchanger has 
        // insulation
        let insulation_to_outer_shell_conductance: ThermalConductance;
        
        
        let outer_shell_to_shell_side_fluid_conductance: ThermalConductance = 
            self.get_shell_side_fluid_to_outer_pipe_shell_nodal_conductance(
                prandtl_wall_correction_setting)?;


        // for the parallel tube bundle, we have to be extra careful 
        // because we are accounting for multiple tubes 
        //
        // for the tube, it is one single pipe so to speak, 
        // Therefore, the tube fluid to tube metallic side should be 
        // for a single tube only 
        //
        // However, for shell side fluid to tube, we could use the 
        // conductance to a single tube or to the tube bundle
        // For clarity, I just named the conductance by the tube bundle.
        //
        // However, I also define a single tube to shell side fluid conductance
        // This avoids ambiguity when dealing with the conductance arrays
        //
        let single_tube_to_shell_side_fluid_conductance: ThermalConductance
            = self.get_shell_side_fluid_to_single_inner_pipe_shell_nodal_conductance(
                prandtl_wall_correction_setting)?;
        let single_tube_to_tube_side_fluid_conductance: ThermalConductance
            = self.get_single_tube_side_fluid_array_node_to_inner_pipe_shell_nodal_conductance(
                prandtl_wall_correction_setting)?;

        let tube_bundle_to_shell_side_fluid_conductance: ThermalConductance 
            = single_tube_to_shell_side_fluid_conductance * 
            self.number_of_tubes as f64;

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


            // for this process, I will make a clone of 
            // each HeatTransferEntity, modify the clone, then 
            // replace the HeatTransferEntity within the pipe using 
            // these changed entities
            let mut single_inner_tube_fluid_arr_clone: FluidArray = 
                self.tube_side_fluid_array_for_single_tube.clone().try_into()?;

            let mut single_inner_pipe_shell_clone: SolidColumn = 
                self.inner_pipe_shell_array_for_single_tube.clone().try_into()?;

            let mut shell_side_fluid_arr_clone: FluidArray = 
                self.shell_side_fluid_array.clone().try_into()?;

            let mut outer_shell_clone: SolidColumn = 
                self.outer_shell.clone().try_into()?;

            // let's get the temperature vectors

            let single_inner_tube_fluid_arr_temp_vec: Vec<ThermodynamicTemperature>
                = single_inner_tube_fluid_arr_clone.get_temperature_vector()?;

            let single_inner_pipe_shell_arr_temp_vec: Vec<ThermodynamicTemperature> 
                = single_inner_pipe_shell_clone.get_temperature_vector()?;

            let shell_side_fluid_arry_temp_vec: Vec<ThermodynamicTemperature> 
                = shell_side_fluid_arr_clone.get_temperature_vector()?;

            let outer_shell_arr_temp_vec: Vec<ThermodynamicTemperature> 
                = outer_shell_clone.get_temperature_vector()?;

            // perform the inner connections 
            // for single inner tube fluid to single pipe shell arr 
            //
            // so the single inner fluid array must be linked to the 
            // temperature of the shell via a single tube to single 
            // tube side fluid conductance

            single_inner_tube_fluid_arr_clone.
                lateral_link_new_temperature_vector_avg_conductance(
                    single_tube_to_tube_side_fluid_conductance, 
                    single_inner_pipe_shell_arr_temp_vec.clone())?;

            single_inner_pipe_shell_clone.
                lateral_link_new_temperature_vector_avg_conductance(
                    single_tube_to_tube_side_fluid_conductance, 
                    single_inner_tube_fluid_arr_temp_vec)?;

            // next the single inner tube needs to be connected 
            // laterally to the shell side fluid
            // no reversals are given here, as in to reverse the 
            // temperature vector
            //
            // the only thing is that to account for parallel tube effects,
            //
            // the conductance to the single 
            // inner tube is based on one tube only,
            //
            // while the conductance to shell side fluid is based on all 
            // the parallel tubes

            single_inner_pipe_shell_clone.
                lateral_link_new_temperature_vector_avg_conductance(
                    single_tube_to_shell_side_fluid_conductance, 
                    shell_side_fluid_arry_temp_vec.clone())?;

            shell_side_fluid_arr_clone. 
                lateral_link_new_temperature_vector_avg_conductance(
                    tube_bundle_to_shell_side_fluid_conductance, 
                    single_inner_pipe_shell_arr_temp_vec)?;

            // next, we need to link the shell side fluid 
            // to the outer shell 

            shell_side_fluid_arr_clone. 
                lateral_link_new_temperature_vector_avg_conductance(
                    outer_shell_to_shell_side_fluid_conductance, 
                    outer_shell_arr_temp_vec.clone())?;

            outer_shell_clone. 
                lateral_link_new_temperature_vector_avg_conductance(
                    outer_shell_to_shell_side_fluid_conductance, 
                    shell_side_fluid_arry_temp_vec)?;

            // for the last part, it depends whether we turned 
            // insulation on or off 

            if self.heat_exchanger_has_insulation {
                // if insulation is on, then use the insulation to outer 
                // shell thermal conductance
                //

                insulation_to_outer_shell_conductance = 
                    self.get_outer_pipe_shell_to_insulation_conductance()?;

                // we shall need to clone the insulation array 
                let mut insulation_array_clone: SolidColumn = 
                    self.insulation_array.clone().try_into()?;

                // get its temperature vector
                let insulation_arr_arr_temp_vec: Vec<ThermodynamicTemperature> 
                    = insulation_array_clone.get_temperature_vector()?;

                // then laterally link it to the outer shell array 


                insulation_array_clone. 
                    lateral_link_new_temperature_vector_avg_conductance(
                        insulation_to_outer_shell_conductance, 
                        outer_shell_arr_temp_vec)?;

                outer_shell_clone 
                    .lateral_link_new_temperature_vector_avg_conductance(
                        insulation_to_outer_shell_conductance, 
                        insulation_arr_arr_temp_vec)?;

                // then the ambient air

                insulation_array_clone
                    .lateral_link_new_temperature_vector_avg_conductance(
                        outer_node_layer_to_air_conductance, 
                        ambient_temperature_vector)?;

                // for the insulation array,
                // lateral connections are done, 
                // so now, modify the heat transfer entity 
                self.insulation_array.set(
                    insulation_array_clone.into())?;

                // pretty much done here, now for testing..

            } else {
                // the outer shell connects directly to the 
                // air 

                outer_shell_clone 
                    .lateral_link_new_temperature_vector_avg_conductance(
                        outer_shell_to_shell_side_fluid_conductance, 
                        ambient_temperature_vector)?;
            }

            // after this, we are done for the internal connections

            // by default, we don't expect shell and 
            // heat exchangers to have heat added to them 
            // so I'm not going to add heat addition vectors to 
            // any of these arrays 


            // now that lateral connections are done, 
            // for the outer shell, inner shell and 
            // both fluid arrays
            // modify the heat transfer entities

            self.outer_shell.set(outer_shell_clone.into())?;

            self.shell_side_fluid_array.set(shell_side_fluid_arr_clone.into())?;

            self.tube_side_fluid_array_for_single_tube
                .set(single_inner_tube_fluid_arr_clone.into())?;

            self.inner_pipe_shell_array_for_single_tube
                .set(single_inner_pipe_shell_clone.into())?;

            

        }

        // axial connections  (adiabatic by default)
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
    /// FluidArrays or SolidColumns
    ///
    /// note that for the STHE, the link to front and back 
    /// functions are exactly the same as for non parallel components,
    /// the parallel treatment is given in the advance timestep 
    /// portion of the code
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

        // now connect the four or five arrays 
        // two solid arrays (or three if insulation is switched on) 
        // and two fluid arrays


        // tube side
        self.tube_side_fluid_array_for_single_tube.link_to_front(&mut zero_power_bc,
            interaction)?;

        self.tube_side_fluid_array_for_single_tube.link_to_back(&mut zero_power_bc,
            interaction)?;

        self.inner_pipe_shell_array_for_single_tube.link_to_front(&mut zero_power_bc,
            interaction)?;

        self.inner_pipe_shell_array_for_single_tube.link_to_back(&mut zero_power_bc,
            interaction)?;

        // shell side
        self.shell_side_fluid_array.link_to_front(&mut zero_power_bc,
            interaction)?;

        self.shell_side_fluid_array.link_to_back(&mut zero_power_bc,
            interaction)?;

        self.outer_shell.link_to_front(&mut zero_power_bc,
            interaction)?;

        self.outer_shell.link_to_back(&mut zero_power_bc,
            interaction)?;

        // insulation 

        if self.heat_exchanger_has_insulation {

            self.insulation_array.link_to_front(&mut zero_power_bc,
                interaction)?;

            self.insulation_array.link_to_back(&mut zero_power_bc,
                interaction)?;

        }


        Ok(())
    }


    /// obtains air to shell and tube heat exchanger (sthe)
    /// outer array conductance 
    ///
    /// The outer array will be insulation if insulation is switched on,
    /// or the outer shell if insulation is switched off
    #[inline]
    pub fn get_air_to_outer_sthe_layer_nodal_conductance(&mut self,
        h_air_to_pipe_surf: HeatTransfer) 
        -> Result<ThermalConductance,TuasLibError> 
    {

        // for conductance calculations (no radiation), 
        // it is important to get the temperatures of the ambient 
        // surroundings and the dimensions of the outer shell or insulation

        let heated_length: Length;
        let id: Length;
        let od: Length;
        let outer_node_temperature: ThermodynamicTemperature;
        // shell and tube heat excanger (STHE) to air interaction
        let number_of_temperature_nodes = self.inner_nodes + 2;
        let mut outer_solid_array_clone: SolidColumn;

        if self.heat_exchanger_has_insulation {
            // if there's insulation, the id is the outer diameter of 
            // the shell. 

            id = self.shell_side_od;
            od = self.shell_side_od + 2.0*self.insulation_thickness;

            // heated length is the shell side length 
            // first I need the fluid array as a fluid component

            let shell_side_fluid_component_clone: FluidComponent 
                = self.get_clone_of_shell_side_fluid_component();

            // then i need to get the component length 
            heated_length = shell_side_fluid_component_clone
                .get_component_length_immutable();

            // surface temperature is the insulation bulk temperature 
            // (estimated)

            // the outer node clone is insulation if it is switched on
            outer_solid_array_clone = 
                self.insulation_array.clone().try_into()?;

            outer_node_temperature = outer_solid_array_clone
                .try_get_bulk_temperature()?;

        } else {
            // if there's no insulation, the id is the inner diameter of 
            // the shell
            // od is outer diameter of the shell

            id = self.shell_side_id;
            od = self.shell_side_od;

            // heated length is the shell side length 
            // first I need the fluid array as a fluid component

            let shell_side_fluid_component_clone: FluidComponent 
                = self.get_clone_of_shell_side_fluid_component();

            // then i need to get the component length 
            heated_length = shell_side_fluid_component_clone
                .get_component_length_immutable();

            // surface temperature is the outer shell bulk temperature 
            // (estimated)

            // the outer node clone is outer shell array if it is switched off
            outer_solid_array_clone = 
                self.outer_shell.clone().try_into()?;

            outer_node_temperature = outer_solid_array_clone
                .try_get_bulk_temperature()?;

        }

        let cylinder_mid_diameter: Length = 0.5*(id+od);


        let node_length = heated_length / 
            number_of_temperature_nodes as f64;

        let outer_node_air_conductance_interaction: HeatTransferInteractionType
        = HeatTransferInteractionType::
            CylindricalConductionConvectionLiquidOutside(
                (outer_solid_array_clone.material_control_volume, 
                    (od-cylinder_mid_diameter).into(),
                    outer_node_temperature,
                    outer_solid_array_clone.pressure_control_volume),
                (h_air_to_pipe_surf,
                    od.into(),
                    node_length.into())
            );

        let outer_node_air_nodal_thermal_conductance: ThermalConductance = try_get_thermal_conductance_based_on_interaction(
            self.ambient_temperature,
            outer_node_temperature,
            outer_solid_array_clone.pressure_control_volume,
            outer_solid_array_clone.pressure_control_volume,
            outer_node_air_conductance_interaction,
        )?;


        return Ok(outer_node_air_nodal_thermal_conductance);
    }


    /// obtains tube side fluid to pipe shell conductance
    #[inline]
    pub fn get_single_tube_side_fluid_array_node_to_inner_pipe_shell_nodal_conductance(
        &mut self,
        correct_prandtl_for_wall_temperatures: bool) 
        -> Result<ThermalConductance,TuasLibError> 
    {

        // the thermal conductance here should be based on the 
        // nusselt number correlation

        // before any calculations, I will first need a clone of 
        // the fluid array and inner shell array
        //
        // the fluid array represents only a single tube
        let mut tube_side_single_fluid_array_clone: FluidArray = 
            self.tube_side_fluid_array_for_single_tube.clone().try_into()?;


        let mut pipe_shell_clone: SolidColumn = 
            self.inner_pipe_shell_array_for_single_tube.clone().try_into()?;

        // also need to get basic temperatures and mass flowrates 
        // only do this once because some of these methods involve 
        // cloning, which is computationally expensive

        let single_tube_mass_flowrate: MassRate = 
            tube_side_single_fluid_array_clone.get_mass_flowrate();

        let fluid_temperature: ThermodynamicTemperature 
            = tube_side_single_fluid_array_clone.try_get_bulk_temperature()?;

        let wall_temperature: ThermodynamicTemperature 
            = pipe_shell_clone.try_get_bulk_temperature()?;

        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);

        let pipe_shell_surf_temperature: ThermodynamicTemperature 
            = wall_temperature;

        let single_tube_hydraulic_diameter = 
            self.get_tube_side_hydraulic_diameter_circular_tube();
        let single_tube_flow_area: Area = 
            tube_side_single_fluid_array_clone.get_cross_sectional_area_immutable();

        // flow area and hydraulic diameter are ok


        let fluid_material: LiquidMaterial
            = tube_side_single_fluid_array_clone.material_control_volume.try_into()?;

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


        // for tube side, gnielinski correlation is expected
        // however, if we want to change this, 
        // we need to rely on the nusselt correlation set in 
        // the struct

        let mut pipe_prandtl_reynolds_data: GnielinskiData 
            = GnielinskiData::default();

        // wall correction is optionally turned on based on whether 
        // wall correction is true or false
        pipe_prandtl_reynolds_data.reynolds = reynolds_number_abs_for_nusselt;
        pipe_prandtl_reynolds_data.prandtl_bulk = bulk_prandtl_number;
        pipe_prandtl_reynolds_data.prandtl_wall = bulk_prandtl_number;
        pipe_prandtl_reynolds_data.length_to_diameter = 
            tube_side_single_fluid_array_clone.get_component_length_immutable()/
            tube_side_single_fluid_array_clone.get_hydraulic_diameter_immutable();

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

            let wall_prandtl_number: Ratio 
                = fluid_material.try_get_prandtl_liquid(
                    wall_temperature_estimate,
                    atmospheric_pressure
                )?;

            pipe_prandtl_reynolds_data.prandtl_wall = wall_prandtl_number;




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
        let fldk: Ratio = self
            .tube_side_custom_component_loss_correlation
            .fldk_based_on_darcy_friction_factor(reynolds_number_abs_for_nusselt)
            .unwrap();

        // (f_darcy + D/L  K)
        // then let's scale it by length to diameter 
        let modified_darcy_friction_factor: Ratio = 
            fldk/pipe_prandtl_reynolds_data.length_to_diameter;




        let nusselt_estimate_tube_side = 
            self.tube_side_nusselt_correlation
            .estimate_based_on_prandtl_darcy_and_reynolds_wall_correction(
                pipe_prandtl_reynolds_data.prandtl_bulk, 
                pipe_prandtl_reynolds_data.prandtl_wall, 
                modified_darcy_friction_factor,
                reynolds_number_abs_for_nusselt)?;

        // for debugging
        //dbg!(&nusselt_estimate_tube_side);


        // now we can get the heat transfer coeff, 

        let tube_h_to_fluid: HeatTransfer;

        let k_fluid_average: ThermalConductivity = 
            fluid_material.try_get_thermal_conductivity(
                fluid_temperature)?;

        tube_h_to_fluid = nusselt_estimate_tube_side * k_fluid_average / single_tube_hydraulic_diameter;


        // and then get the convective resistance
        let number_of_temperature_nodes = self.inner_nodes + 2;
        let heated_length = tube_side_single_fluid_array_clone.get_component_length();
        let id = self.tube_side_id;
        let od = self.tube_side_od;


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
                 (tube_h_to_fluid,
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


    /// obtains shell side fluid to *single* pipe shell conductance
    /// you'll have to multiply by the number of tubes to obtain 
    /// the whole conductance bit
    ///
    ///
    /// See diagram below:
    /// |            |            |               |             |            |
    /// |            |            |               |             |            |
    /// |-tube fluid-|-inner tube-|- shell fluid -|-outer shell-|-insulation-| ambient
    /// |            |            |               |             |            |
    /// |            |            |               |             |            |
    #[inline]
    pub fn get_shell_side_fluid_to_single_inner_pipe_shell_nodal_conductance(
        &mut self,
        correct_prandtl_for_wall_temperatures: bool) 
        -> Result<ThermalConductance,TuasLibError> 
    {

        // the thermal conductance here should be based on the 
        // nusselt number correlation

        // before any calculations, I will first need a clone of 
        // the fluid array and twisted tape array
        let mut shell_side_fluid_array_clone: FluidArray = 
            self.shell_side_fluid_array.clone().try_into()?;

        let mut pipe_shell_clone: SolidColumn = 
            self.inner_pipe_shell_array_for_single_tube.clone().try_into()?;

        // also need to get basic temperatures and mass flowrates 
        // only do this once because some of these methods involve 
        // cloning, which is computationally expensive

        let shell_side_mass_flowrate: MassRate = 
            shell_side_fluid_array_clone.get_mass_flowrate();

        let fluid_temperature: ThermodynamicTemperature 
            = shell_side_fluid_array_clone.try_get_bulk_temperature()?;

        let wall_temperature: ThermodynamicTemperature 
            = pipe_shell_clone.try_get_bulk_temperature()?;

        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);

        let pipe_shell_surf_temperature: ThermodynamicTemperature 
            = pipe_shell_clone.try_get_bulk_temperature()?;

        let shell_side_fluid_hydraulic_diameter = 
            self.get_shell_side_hydraulic_diameter();

        let shell_side_cross_sectional_flow_area: Area = 
            self.get_shell_side_cross_sectional_area();


        // flow area and hydraulic diameter are ok


        let fluid_material: LiquidMaterial
            = shell_side_fluid_array_clone.material_control_volume.try_into()?;

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
        let reynolds_number_shell_side: Ratio = 
            shell_side_mass_flowrate/
            shell_side_cross_sectional_flow_area
            *shell_side_fluid_hydraulic_diameter / viscosity;

        // the reynolds number here is used for nusselt number estimates 
        // so I'm going to have an aboslute value of reynolds number 
        // for nusselt estimates

        let reynolds_number_abs_for_nusselt_estimate: Ratio 
            = reynolds_number_shell_side.abs();
        

        // next, bulk prandtl number 

        let bulk_prandtl_number: Ratio 
            = fluid_material.try_get_prandtl_liquid(
                fluid_temperature,
                atmospheric_pressure
            )?;



        let shell_side_fluid_to_inner_tube_surf_nusselt_correlation: NusseltCorrelation
            = self.shell_side_nusselt_correlation_to_tubes;


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
        let fldk: Ratio = self
            .shell_side_custom_component_loss_correlation
            .fldk_based_on_darcy_friction_factor(reynolds_number_abs_for_nusselt_estimate)
            .unwrap();

        let length_to_diameter: Ratio = 
            shell_side_fluid_array_clone.get_component_length_immutable()/
            shell_side_fluid_hydraulic_diameter;

        // (f_darcy + D/L  K)
        // then let's scale it by length to diameter 
        let modified_darcy_friction_factor: Ratio = 
            fldk/length_to_diameter;

        // I need to use Nusselt correlations present in this struct 
        //
        // wall correction is optionally done here
        //
        // this uses the gnielinski correlation for pipes or tubes

        let nusselt_estimate_shell: Ratio;

        if correct_prandtl_for_wall_temperatures {

            // then wall prandtl number
            // if the number falls outside the range of correlations,
            // then use the prandtl number at the max or min 

            let mut wall_temperature_estimate = wall_temperature;

            if wall_temperature_estimate > fluid_material.max_temperature() {

                wall_temperature_estimate = fluid_material.max_temperature();

            } else if wall_temperature_estimate < fluid_material.min_temperature() {

                wall_temperature_estimate = fluid_material.min_temperature();

            }


            let wall_prandtl_number: Ratio 
                = fluid_material.try_get_prandtl_liquid(
                    wall_temperature_estimate,
                    atmospheric_pressure
                )?;

            nusselt_estimate_shell = shell_side_fluid_to_inner_tube_surf_nusselt_correlation.
            estimate_based_on_prandtl_darcy_and_reynolds_wall_correction(
                bulk_prandtl_number, 
                wall_prandtl_number,
                modified_darcy_friction_factor,
                reynolds_number_abs_for_nusselt_estimate)?;

        } else {
            nusselt_estimate_shell = shell_side_fluid_to_inner_tube_surf_nusselt_correlation.
            estimate_based_on_prandtl_darcy_and_reynolds_no_wall_correction(
                bulk_prandtl_number, 
                modified_darcy_friction_factor,
                reynolds_number_abs_for_nusselt_estimate)?;

        }

        // for debugging
        //dbg!(&nusselt_estimate_shell);



        // now we can get the heat transfer coeff, 

        let shell_h_to_fluid: HeatTransfer;

        let k_fluid_average: ThermalConductivity = 
            fluid_material.try_get_thermal_conductivity(
                fluid_temperature)?;

        shell_h_to_fluid = nusselt_estimate_shell * k_fluid_average / shell_side_fluid_hydraulic_diameter;


        // and then get the convective resistance from shell side fluid 
        // to the tubes
        let number_of_temperature_nodes = self.inner_nodes + 2;
        let heated_length = shell_side_fluid_array_clone.get_component_length();
        let id = self.tube_side_id;
        let od = self.tube_side_od;


        let node_length = heated_length / 
            number_of_temperature_nodes as f64;


        // now I need to calculate resistance of the half length of the 
        // pipe shell, which is an annular cylinder

        let cylinder_mid_diameter: Length = 0.5*(id+od);



        // conductance calculations assumes a cylinder with 
        // liquid on the outside, solid on the inside
        let shell_fluid_to_inner_tube_surf_conductance_interaction: HeatTransferInteractionType
            = HeatTransferInteractionType::
            CylindricalConductionConvectionLiquidOutside(
                (solid_material.into(), 
                 (od - cylinder_mid_diameter).into(),
                 pipe_shell_surf_temperature,
                 atmospheric_pressure),
                 (shell_h_to_fluid,
                  od.into(),
                  node_length.into())
            );

        // now based on conductance interaction, 
        // we can obtain thermal conductance, the temperatures 
        // and pressures don't really matter
        //
        // this is because all the thermal conductance data 
        // has already been loaded into the thermal conductance 
        // interaction object

        let shell_fluid_to_inner_tube_surf_nodal_thermal_conductance: ThermalConductance = 
            try_get_thermal_conductance_based_on_interaction(
                fluid_temperature,
                pipe_shell_surf_temperature,
                atmospheric_pressure,
                atmospheric_pressure,
                shell_fluid_to_inner_tube_surf_conductance_interaction)?;


        return Ok(shell_fluid_to_inner_tube_surf_nodal_thermal_conductance);
    }


    /// this calculates the conductance on a per node basis 
    /// from shell side fluid to the outer shell.
    ///
    /// See diagram below:
    /// |            |            |               |             |            |
    /// |            |            |               |             |            |
    /// |-tube fluid-|-inner tube-|- shell fluid -|-outer shell-|-insulation-| ambient
    /// |            |            |               |             |            |
    /// |            |            |               |             |            |
    #[inline]
    pub fn get_shell_side_fluid_to_outer_pipe_shell_nodal_conductance(
        &mut self,
        correct_prandtl_for_wall_temperatures: bool) 
        -> Result<ThermalConductance,TuasLibError> 
    {
        // the thermal conductance here should be based on the 
        // nusselt number correlation

        // before any calculations, I will first need a clone of 
        // the fluid array and outer shell array
        let mut shell_side_fluid_array_clone: FluidArray = 
            self.shell_side_fluid_array.clone().try_into()?;

        let mut outer_shell_clone: SolidColumn = 
            self.outer_shell.clone().try_into()?;

        // also need to get basic temperatures and mass flowrates 
        // only do this once because some of these methods involve 
        // cloning, which is computationally expensive

        let shell_side_mass_flowrate: MassRate = 
            shell_side_fluid_array_clone.get_mass_flowrate();

        let fluid_temperature: ThermodynamicTemperature 
            = shell_side_fluid_array_clone.try_get_bulk_temperature()?;
            
        let wall_temperature: ThermodynamicTemperature 
            = outer_shell_clone.try_get_bulk_temperature()?;

        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);

        let shell_side_fluid_hydraulic_diameter = 
            self.get_shell_side_hydraulic_diameter();

        let shell_side_cross_sectional_flow_area: Area = 
            self.get_shell_side_cross_sectional_area();

        // flow area and hydraulic diameter are ok


        let fluid_material: LiquidMaterial
            = shell_side_fluid_array_clone.material_control_volume.try_into()?;

        let solid_material: SolidMaterial 
            = outer_shell_clone.material_control_volume.try_into()?;

        let viscosity: DynamicViscosity = 
            fluid_material.try_get_dynamic_viscosity(fluid_temperature)?;


        // need to convert hydraulic diameter to an equivalent 
        // spherical diameter
        //
        // but for now, I'm going to use Re and Nu using hydraulic diameter 
        // and live with it for the time being
        //
        let reynolds_number_shell_side: Ratio = 
            shell_side_mass_flowrate/
            shell_side_cross_sectional_flow_area
            *shell_side_fluid_hydraulic_diameter / viscosity;

        // the reynolds number here is used for nusselt number estimates 
        // so I'm going to have an aboslute value of reynolds number 
        // for nusselt estimates

        let reynolds_number_abs_for_nusselt_estimate: Ratio 
            = reynolds_number_shell_side.abs();
        // next, bulk prandtl number 

        let bulk_prandtl_number: Ratio 
            = fluid_material.try_get_prandtl_liquid(
                fluid_temperature,
                atmospheric_pressure
            )?;

        let shell_side_fluid_to_outer_tube_surf_nusselt_correlation: NusseltCorrelation
            = self.shell_side_nusselt_correlation_parasitic;



        // I need to use Nusselt correlations present in this struct 
        //
        // wall correction is optionally done here
        //
        // this uses the gnielinski correlation for pipes or tubes
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

        let pipe_darcy_correlation = 
            &self.shell_side_custom_component_loss_correlation;

        let fldk: Ratio = pipe_darcy_correlation
            .fldk_based_on_darcy_friction_factor(reynolds_number_abs_for_nusselt_estimate)
            .unwrap();

        let length_to_diameter: Ratio = 
            shell_side_fluid_array_clone.get_component_length_immutable()/
            shell_side_fluid_hydraulic_diameter;

        // (f_darcy + D/L  K)
        // then let's scale it by length to diameter 
        let modified_darcy_friction_factor: Ratio = 
            fldk/length_to_diameter;

        let nusselt_estimate: Ratio;

        if correct_prandtl_for_wall_temperatures {

            // then wall prandtl number
            //

            // method I use is to just use the wall prandtl number 
            // if the number falls outside the range of correlations,
            // then use the prandtl number at the max or min 

            let mut wall_temperature_estimate = wall_temperature;

            if wall_temperature_estimate > fluid_material.max_temperature() {

                wall_temperature_estimate = fluid_material.max_temperature();

            } else if wall_temperature_estimate < fluid_material.min_temperature() {

                wall_temperature_estimate = fluid_material.min_temperature();

            }

            let wall_prandtl_number: Ratio 
                = fluid_material.try_get_prandtl_liquid(
                    wall_temperature_estimate,
                    atmospheric_pressure
                )?;

            nusselt_estimate = shell_side_fluid_to_outer_tube_surf_nusselt_correlation.
            estimate_based_on_prandtl_darcy_and_reynolds_wall_correction(
                bulk_prandtl_number, 
                wall_prandtl_number,
                modified_darcy_friction_factor,
                reynolds_number_abs_for_nusselt_estimate)?;

        } else {
            nusselt_estimate = shell_side_fluid_to_outer_tube_surf_nusselt_correlation.
            estimate_based_on_prandtl_darcy_and_reynolds_no_wall_correction(
                bulk_prandtl_number, 
                modified_darcy_friction_factor,
                reynolds_number_abs_for_nusselt_estimate)?;

        }


        // now we can get the heat transfer coeff, 

        let h_to_fluid: HeatTransfer;

        let k_fluid_average: ThermalConductivity = 
            fluid_material.try_get_thermal_conductivity(
                fluid_temperature)?;

        h_to_fluid = nusselt_estimate * k_fluid_average / shell_side_fluid_hydraulic_diameter;


        // and then get the convective resistance from shell side fluid 
        // to the tubes
        let number_of_temperature_nodes = self.inner_nodes + 2;
        let heated_length = shell_side_fluid_array_clone.get_component_length();
        let id = self.tube_side_id;
        let od = self.tube_side_od;


        let node_length = heated_length / 
            number_of_temperature_nodes as f64;

        // now I need to calculate resistance of the half length of the 
        // pipe shell, which is an annular cylinder

        let cylinder_mid_diameter: Length = 0.5*(id+od);

        // conductance calculations assumes a cylinder with 
        // liquid on the inside, solid on the outside 
        

        let shell_fluid_to_outer_tube_conductance_interaction: HeatTransferInteractionType 
            = HeatTransferInteractionType::
            CylindricalConductionConvectionLiquidInside(
                (solid_material.into(),
                (cylinder_mid_diameter - id).into(),
                wall_temperature,
                 atmospheric_pressure), 
                (h_to_fluid,
                 id.into(),
                 node_length.into()
                 )

            );

        let shell_fluid_to_outer_tube_surf_nodal_thermal_conductance:
            ThermalConductance = 
            try_get_thermal_conductance_based_on_interaction(
                fluid_temperature,
                wall_temperature,
                atmospheric_pressure,
                atmospheric_pressure,
                shell_fluid_to_outer_tube_conductance_interaction)?;

        return Ok(shell_fluid_to_outer_tube_surf_nodal_thermal_conductance);

    }

    /// obtains outer pipe shell to insulation conductance
    #[inline]
    pub fn get_outer_pipe_shell_to_insulation_conductance(
    &self) -> Result<ThermalConductance,TuasLibError> {
        // first, make a clone of outer pipe shell and insulation

        let mut insulation_array_clone: SolidColumn = 
        self.insulation_array.clone().try_into()?;

        let mut pipe_shell_clone: SolidColumn = 
        self.outer_shell.clone().try_into()?;

        // find the length of the array and node length

        let array_length =  pipe_shell_clone.get_component_length();

        let number_of_temperature_nodes = self.inner_nodes + 2;

        let node_length = array_length / 
        number_of_temperature_nodes as f64;

        // then we need to find the surface area of each node 
        // for steel to insulation_material, it will be 
        // the steel outer diameter or insulation inner_diameter
        
        let pipe_shell_mid_section_diameter = 0.5 * (self.shell_side_od 
        + self.shell_side_id);

        let insulation_material_mid_section_diameter = 
            self.insulation_thickness + 
            self.shell_side_od;

        let shell_od = self.shell_side_od;

        // next, thermal conductivities of both solid_pipe_material and insulation_material 

        let solid_pipe_material_shell_temperature = pipe_shell_clone.try_get_bulk_temperature() 
            ?;

        let solid_pipe_material: SolidMaterial = pipe_shell_clone.material_control_volume
            .try_into()?;

        let solid_pipe_material_conductivity: ThermalConductivity 
        = solid_pipe_material.try_get_thermal_conductivity(
            solid_pipe_material_shell_temperature
        )?;


        let insulation_material_shell_temperature = insulation_array_clone.try_get_bulk_temperature() 
            ?;

        let insulation_material: SolidMaterial = insulation_array_clone.material_control_volume
            .try_into()?;

        let insulation_material_conductivity: ThermalConductivity 
        = insulation_material.try_get_thermal_conductivity(
            insulation_material_shell_temperature
        )?;

        // we should be able to get the conductance now

        let insulation_material_layer_conductance: ThermalConductance = 
        try_get_thermal_conductance_annular_cylinder(
            shell_od,
            insulation_material_mid_section_diameter,
            node_length,
            insulation_material_conductivity
        )?;
        
        let solid_pipe_material_layer_conductance: ThermalConductance = 
        try_get_thermal_conductance_annular_cylinder(
            pipe_shell_mid_section_diameter,
            shell_od,
            node_length,
            solid_pipe_material_conductivity
        )?;
        // now that we have the conductances, we get the resistances 

        let insulation_material_resistance = 1.0/insulation_material_layer_conductance;
        let solid_pipe_material_resistance = 1.0/solid_pipe_material_layer_conductance;

        let total_resistance = insulation_material_resistance + solid_pipe_material_resistance;


        return Ok(1.0/total_resistance);
    }

    /// spawns a thread and moves the clone of the entire heater object into the 
    /// thread, "locking" it for parallel computation
    ///
    /// once that is done, the join handle is returned 
    /// which when unwrapped, returns the heater object
    pub fn lateral_connection_thread_spawn(&self,
        prandtl_wall_correction_setting: bool,
        tube_side_total_mass_flowrate: MassRate,
        shell_side_total_mass_flowrate: MassRate,) -> JoinHandle<Self>{

        let mut heater_clone = self.clone();

        // move ptr into a new thread 

        let join_handle = thread::spawn(
            move || -> Self {

                // carry out the connection calculations
                heater_clone.
                    lateral_and_miscellaneous_connections(
                        prandtl_wall_correction_setting,
                        tube_side_total_mass_flowrate,
                        shell_side_total_mass_flowrate,).unwrap();

                heater_clone

            }
        );

        return join_handle;

    }
    /// calibrates the insulation thickness of this pipe or component, 
    /// to increase or decrease parasitic heat loss
    /// however, will not change thermal inertia
    /// 
    pub fn calibrate_insulation_thickness(&mut self, 
        insulation_thickness: Length){

        self.insulation_thickness = insulation_thickness;

    }

    /// gets the insulation thickness based on 
    /// (insulation_od - insulation_id)/2
    pub fn get_insulation_thickness(&self) -> Length {

        return self.insulation_thickness;

    }
    
}
