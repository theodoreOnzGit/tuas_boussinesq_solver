use std::f64::consts::PI;
use std::thread::{self, JoinHandle};

use super::InsulatedPorousMediaFluidComponent;
use uom::si::length::meter;
use uom::ConstZero;
use uom::si::pressure::atmosphere;
use ndarray::*;

use uom::si::f64::*;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
use crate::heat_transfer_correlations::nusselt_number_correlations::input_structs::GnielinskiData;
use crate::heat_transfer_correlations::thermal_resistance::try_get_thermal_conductance_annular_cylinder;
use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::boundary_conditions::BCType;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::tuas_lib_error::TuasLibError;

impl InsulatedPorousMediaFluidComponent {


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

        self.insulation_array.link_to_back(&mut zero_power_bc,
            interaction).unwrap();


        self.insulation_array.link_to_front(&mut zero_power_bc,
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

    /// obtains the conductance from ambient to the pipe shell 
    /// nodally speaking 

    #[inline]
    pub fn get_ambient_to_insulation_nodal_conductance(&mut self,
        heat_transfer_to_ambient: HeatTransfer) 
        -> Result<ThermalConductance,TuasLibError> {

            // the solid conductance is calculated using 
            // k (A/L) where L is representative thickness 
            // A is heat transfer area,
            // k is thermal conductivity
            //
            let insulation_conductance_lengthscale: Length = 
                self.thermal_conductance_lengthscale_insulation_to_ambient;
            
            // to calculate k, we need the bulk temperature 

            let mut insulation_clone: SolidColumn = 
                self.insulation_array.clone().try_into().unwrap();
            let insulation_bulk_temp: ThermodynamicTemperature = 
                insulation_clone.try_get_bulk_temperature()?;

            // next, let's get the conductivity 

            let insulation_material_conductivity: ThermalConductivity = 
                insulation_clone.material_control_volume
                .try_get_thermal_conductivity(
                    insulation_bulk_temp)?;

            let number_of_nodes: f64 = self.inner_nodes as f64 + 2.0;
            // solid side nodalised thermal conductance

            let nodalised_insulation_side_thermal_conductance: ThermalConductance
                = insulation_conductance_lengthscale * insulation_material_conductivity
                / number_of_nodes;

            let nodalised_insulation_side_thermal_resistance: ThermalResistance 
                = nodalised_insulation_side_thermal_conductance.recip();

            // next, nodalised thermal conductance due to liquid side 
            
            let nodalised_thermal_conductance_ambient_convection: ThermalConductance 
                = (heat_transfer_to_ambient * self.convection_heat_transfer_area_insulation_to_ambient)
                / number_of_nodes;

            let nodalised_ambient_convection_thermal_resistance: ThermalResistance 
                = nodalised_thermal_conductance_ambient_convection.recip();


            // add resistances together 
            let nodalised_insulation_to_ambient_resistance: ThermalResistance 
                = nodalised_insulation_side_thermal_resistance +
                nodalised_ambient_convection_thermal_resistance;

            // get conductance, and then return 

            let nodalised_insulation_to_ambient_conductance 
                = nodalised_insulation_to_ambient_resistance.recip();

            // and we done!
            return Ok(nodalised_insulation_to_ambient_conductance);

    }


    #[inline]
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
            self.nusselt_correlation_lengthscale_fluid_to_pipe_shell;
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
            self.nusselt_correlation_fluid_to_pipe_shell
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
            = (tube_h_to_fluid * self.convection_heat_transfer_area_fluid_to_pipe_shell)
            / number_of_temperature_nodes;

        
        // now solid side nodalised conductance 
        let nodalised_solid_side_conductance: ThermalConductance 
            = (self.thermal_conductance_lengthscale_pipe_shell_to_fluid * 
                solid_thermal_conductivity) / number_of_temperature_nodes;

        let nodalised_pipe_fluid_to_shell_thermal_resistance: ThermalResistance 
            = nodalised_solid_side_conductance.recip() 
            + nodalised_fluid_side_conductance.recip();

        // return the conductance 

        return Ok(nodalised_pipe_fluid_to_shell_thermal_resistance.recip());
    }


}


/// contains preprocessing calcs specifc to mx10 and static 
/// mixers
pub mod mx10;
