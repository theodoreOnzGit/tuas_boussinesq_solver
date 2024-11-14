use std::thread::JoinHandle;
use std::thread;

use uom::si::thermodynamic_temperature::kelvin;
use uom::ConstZero;
use uom::si::pressure::atmosphere;
use uom::si::f64::*;
use ndarray::*;
use super::ClamshellRadiativeHeater;
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
//

impl ClamshellRadiativeHeater {

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
        self.pipe_fluid_array.link_to_front(&mut zero_power_bc,
            interaction)?;

        self.pipe_fluid_array.link_to_back(&mut zero_power_bc,
            interaction)?;

        self.pipe_shell_array.link_to_front(&mut zero_power_bc,
            interaction)?;

        self.pipe_shell_array.link_to_back(&mut zero_power_bc,
            interaction)?;

        // annular air array
        self.annular_air_array.link_to_front(&mut zero_power_bc,
            interaction)?;

        self.annular_air_array.link_to_back(&mut zero_power_bc,
            interaction)?;

        // heating element and insulation
        self.heating_element_shell.link_to_front(&mut zero_power_bc,
            interaction)?;

        self.heating_element_shell.link_to_back(&mut zero_power_bc,
            interaction)?;


        self.insulation_array.link_to_front(&mut zero_power_bc,
            interaction)?;

        self.insulation_array.link_to_back(&mut zero_power_bc,
            interaction)?;



        Ok(())
    }

    /// obtains ambient air to radiative heater insulation layer
    /// conductance 
    ///
    ///
    /// excludes radiative heat transfer
    #[inline]
    pub fn get_ambient_air_to_insulation_nodal_conductance(&mut self,
        h_air_to_pipe_surf: HeatTransfer) 
        -> Result<ThermalConductance,TuasLibError> 
    {

        // for conductance calculations (no radiation), 
        // it is important to get the temperatures of the ambient 
        // surroundings and the dimensions of the outer shell or insulation

        let heated_length: Length;
        let insulation_id: Length;
        let insulation_od: Length;
        let insulation_node_temperature: ThermodynamicTemperature;
        // shell and tube heat excanger (STHE) to air interaction
        let number_of_temperature_nodes = self.inner_nodes + 2;
        let mut insulation_array_clone: SolidColumn;

        // if there's insulation, the id is the outer diameter of 
        // the shell. 

        insulation_id = self.heating_element_od;
        insulation_od = self.heating_element_od + 2.0*self.insulation_thickness;

        // heated length is the shell side length 
        // first I need the fluid array as a fluid component

        let annular_air_fluid_component_clone: FluidComponent 
            = self.get_clone_of_annular_air_array();

        // then i need to get the component length 
        heated_length = annular_air_fluid_component_clone
            .get_component_length_immutable();

        // surface temperature is the insulation bulk temperature 
        // (estimated)

        insulation_array_clone = 
            self.insulation_array.clone().try_into()?;

        insulation_node_temperature = insulation_array_clone
            .try_get_bulk_temperature()?;

        // the outer node clone is insulation if it is switched on


        let cylinder_mid_diameter: Length = 0.5*(insulation_id+insulation_od);


        let node_length = heated_length / 
            number_of_temperature_nodes as f64;

        let outer_node_air_conductance_interaction: HeatTransferInteractionType
        = HeatTransferInteractionType::
            CylindricalConductionConvectionLiquidOutside(
                (insulation_array_clone.material_control_volume, 
                    (insulation_od-cylinder_mid_diameter).into(),
                    insulation_node_temperature,
                    insulation_array_clone.pressure_control_volume),
                (h_air_to_pipe_surf,
                    insulation_od.into(),
                    node_length.into())
            );

        let outer_node_air_nodal_thermal_conductance: ThermalConductance = try_get_thermal_conductance_based_on_interaction(
            self.ambient_temperature,
            insulation_node_temperature,
            insulation_array_clone.pressure_control_volume,
            insulation_array_clone.pressure_control_volume,
            outer_node_air_conductance_interaction,
        )?;


        return Ok(outer_node_air_nodal_thermal_conductance);
    }

    /// obtains heating element to insulation conductance
    #[inline]
    pub fn get_heating_element_to_insulation_conductance(
    &self) -> Result<ThermalConductance,TuasLibError> {
        // first, make a clone of outer pipe shell and insulation

        let mut insulation_array_clone: SolidColumn = 
        self.insulation_array.clone().try_into()?;

        let mut pipe_shell_clone: SolidColumn = 
        self.heating_element_shell.clone().try_into()?;

        // find the length of the array and node length

        let array_length =  pipe_shell_clone.get_component_length();

        let number_of_temperature_nodes = self.inner_nodes + 2;

        let node_length = array_length / 
        number_of_temperature_nodes as f64;

        // then we need to find the surface area of each node 
        // for steel to insulation_material, it will be 
        // the steel outer diameter or insulation inner_diameter
        
        let heating_element_mid_section_diameter = 0.5 * (self.heating_element_od 
        + self.heating_element_id);

        let insulation_material_mid_section_diameter = 
            self.insulation_thickness + 
            self.heating_element_od;

        let heating_element_od = self.heating_element_od;

        // next, thermal conductivities of both solid_pipe_material and insulation_material 

        let heating_element_material_temperature = pipe_shell_clone.try_get_bulk_temperature() 
            ?;

        let heating_element_material: SolidMaterial = pipe_shell_clone.material_control_volume
            .try_into()?;

        let heating_element_material_conductivity: ThermalConductivity 
        = heating_element_material.try_get_thermal_conductivity(
            heating_element_material_temperature
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
            heating_element_od,
            insulation_material_mid_section_diameter,
            node_length,
            insulation_material_conductivity
        )?;
        
        let heating_element_material_layer_conductance: ThermalConductance = 
        try_get_thermal_conductance_annular_cylinder(
            heating_element_mid_section_diameter,
            heating_element_od,
            node_length,
            heating_element_material_conductivity
        )?;
        // now that we have the conductances, we get the resistances 

        let insulation_material_resistance = 1.0/insulation_material_layer_conductance;
        let heating_element_material_resistance = 1.0/heating_element_material_layer_conductance;

        let total_resistance = insulation_material_resistance + heating_element_material_resistance;


        return Ok(1.0/total_resistance);
    }



    /// this calculates the conductance on a per node basis 
    /// from shell side fluid to the outer shell.
    ///
    /// See diagram below:
    /// |            |            |               |             |            |
    /// |            |            |               |             |            |
    /// |-tube fluid-|-inner tube-|- annular air -|-heater elem-|-insulation-| ambient
    /// |            |            |               |             |            |
    /// |            |            |               |             |            |
    ///
    /// radiation not taken into account, assumed to be non-participating 
    /// media
    #[inline]
    pub fn get_heating_element_to_annular_air_nodal_conductance(
        &mut self,
        correct_prandtl_for_wall_temperatures: bool) 
        -> Result<ThermalConductance,TuasLibError> 
    {
        // the thermal conductance here should be based on the 
        // nusselt number correlation

        // before any calculations, I will first need a clone of 
        // the fluid array and outer shell array
        let mut annular_air_array_clone: FluidArray = 
            self.annular_air_array.clone().try_into()?;

        let mut heating_element_clone: SolidColumn = 
            self.heating_element_shell.clone().try_into()?;

        // also need to get basic temperatures and mass flowrates 
        // only do this once because some of these methods involve 
        // cloning, which is computationally expensive

        let annular_air_mass_flowrate: MassRate = 
            annular_air_array_clone.get_mass_flowrate();

        let annular_air_temperature: ThermodynamicTemperature 
            = annular_air_array_clone.try_get_bulk_temperature()?;
            
        let heating_element_temperature: ThermodynamicTemperature 
            = heating_element_clone.try_get_bulk_temperature()?;

        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);

        let shell_side_fluid_hydraulic_diameter = 
            self.get_annular_air_hydraulic_diameter();

        let shell_side_cross_sectional_flow_area: Area = 
            self.get_annular_air_cross_sectional_area();

        // flow area and hydraulic diameter are ok


        let fluid_material: LiquidMaterial
            = annular_air_array_clone.material_control_volume.try_into()?;

        let solid_material: SolidMaterial 
            = heating_element_clone.material_control_volume.try_into()?;

        let viscosity: DynamicViscosity = 
            fluid_material.try_get_dynamic_viscosity(annular_air_temperature)?;


        // need to convert hydraulic diameter to an equivalent 
        // spherical diameter
        //
        // but for now, I'm going to use Re and Nu using hydraulic diameter 
        // and live with it for the time being
        //
        let reynolds_number_shell_side: Ratio = 
            annular_air_mass_flowrate/
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
                annular_air_temperature,
                atmospheric_pressure
            )?;

        let heating_elem_to_annular_air_nusselt_correlation: NusseltCorrelation
            = self.heating_element_to_annular_air_nusselt_correlation;



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
            &self.annular_air_loss_correlation;

        let fldk: Ratio = pipe_darcy_correlation
            .fldk_based_on_darcy_friction_factor(reynolds_number_abs_for_nusselt_estimate)
            .unwrap();

        let length_to_diameter: Ratio = 
            annular_air_array_clone.get_component_length_immutable()/
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

            let mut wall_temperature_estimate = heating_element_temperature;

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

            nusselt_estimate = heating_elem_to_annular_air_nusselt_correlation.
            estimate_based_on_prandtl_darcy_and_reynolds_wall_correction(
                bulk_prandtl_number, 
                wall_prandtl_number,
                modified_darcy_friction_factor,
                reynolds_number_abs_for_nusselt_estimate)?;

        } else {
            nusselt_estimate = heating_elem_to_annular_air_nusselt_correlation.
            estimate_based_on_prandtl_darcy_and_reynolds_no_wall_correction(
                bulk_prandtl_number, 
                modified_darcy_friction_factor,
                reynolds_number_abs_for_nusselt_estimate)?;

        }


        // now we can get the heat transfer coeff, 

        let h_to_fluid: HeatTransfer;

        let k_fluid_average: ThermalConductivity = 
            fluid_material.try_get_thermal_conductivity(
                annular_air_temperature)?;

        h_to_fluid = nusselt_estimate * k_fluid_average / shell_side_fluid_hydraulic_diameter;


        // and then get the convective resistance from shell side fluid 
        // to the tubes
        let number_of_temperature_nodes = self.inner_nodes + 2;
        let heated_length = annular_air_array_clone.get_component_length();
        let id = self.tube_id;
        let od = self.tube_od;


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
                heating_element_temperature,
                 atmospheric_pressure), 
                (h_to_fluid,
                 id.into(),
                 node_length.into()
                 )

            );

        let shell_fluid_to_outer_tube_surf_nodal_thermal_conductance:
            ThermalConductance = 
            try_get_thermal_conductance_based_on_interaction(
                annular_air_temperature,
                heating_element_temperature,
                atmospheric_pressure,
                atmospheric_pressure,
                shell_fluid_to_outer_tube_conductance_interaction)?;

        return Ok(shell_fluid_to_outer_tube_surf_nodal_thermal_conductance);

    }

    /// obtains annular air to pipe shell (inner tube) conductance
    ///
    /// See diagram below:
    /// |            |            |               |             |            |
    /// |            |            |               |             |            |
    /// |-tube fluid-|-inner tube-|- annular air -|-heater elem-|-insulation-| ambient
    /// |            |            |               |             |            |
    /// |            |            |               |             |            |
    ///
    /// radiation not taken into account, assumed to be non-participating 
    /// media
    #[inline]
    pub fn get_annular_air_inner_tube_shell_nodal_conductance(
        &mut self,
        correct_prandtl_for_wall_temperatures: bool) 
        -> Result<ThermalConductance,TuasLibError> 
    {

        // the thermal conductance here should be based on the 
        // nusselt number correlation

        // before any calculations, I will first need a clone of 
        // the fluid array and twisted tape array
        let mut shell_side_fluid_array_clone: FluidArray = 
            self.annular_air_array.clone().try_into()?;

        let mut pipe_shell_clone: SolidColumn = 
            self.pipe_shell_array.clone().try_into()?;

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
            self.get_annular_air_hydraulic_diameter();

        let shell_side_cross_sectional_flow_area: Area = 
            self.get_annular_air_cross_sectional_area();


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
            = self.annular_air_nusselt_correlation_to_tube;


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
            .annular_air_loss_correlation
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
        let id = self.tube_id;
        let od = self.tube_od;


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
            self.pipe_fluid_array.clone().try_into()?;


        let mut pipe_shell_clone: SolidColumn = 
            self.pipe_shell_array.clone().try_into()?;

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
            .tube_loss_correlation
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
        let id = self.tube_id;
        let od = self.tube_od;


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
}
