use std::f64::consts::PI;

use super::SimpleShellAndTubeHeatExchanger;
use uom::si::f64::*;
use uom::si::ratio::ratio;
use uom::si::pressure::atmosphere;
use crate::tuas_lib_error::TuasLibError;
use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;

impl SimpleShellAndTubeHeatExchanger {

    /// gets the temperature of the inner pipe shell array
    pub fn inner_pipe_shell_temperature(&mut self) -> 
        Result<Vec<ThermodynamicTemperature>, TuasLibError>{
            self.inner_pipe_shell_array_for_single_tube.get_temperature_vector()
    }

    /// gets the temperature of the tube side fluid array
    pub fn inner_tube_fluid_array_temperature(&mut self) ->
        Result<Vec<ThermodynamicTemperature>,TuasLibError>{
            self.tube_side_fluid_array_for_single_tube.get_temperature_vector()
    }

    /// gets the shell side fluid array temperature
    pub fn shell_side_fluid_array_temperature(&mut self,) ->
        Result<Vec<ThermodynamicTemperature>,TuasLibError>{
            self.shell_side_fluid_array.get_temperature_vector()
    }

    /// gets the shell side outer tube temperature 
    pub fn shell_side_outer_tube_array_temperature(&mut self,) -> 
        Result<Vec<ThermodynamicTemperature>,TuasLibError>{
            self.outer_shell.get_temperature_vector()
    }

    /// gets the temperature of the insulation 
    pub fn insulation_array_temperature(&mut self,) ->
        Result<Vec<ThermodynamicTemperature>,TuasLibError>{

            if self.heat_exchanger_has_insulation {
                self.insulation_array.get_temperature_vector()
            } else {
                // if the heat exchanger is not even insulated, then 
                // there should be no insulation array
                todo!("simple Shell and Tube Heat exchanger is not insulated")
            }
    }


    /// provides overall heat transfer coeff using conductance 
    /// calculations 
    /// |            |            |               |
    /// |            |            |               |
    /// |-tube fluid-|-inner tube-|- shell fluid -|
    /// |            |            |               |
    /// |            |            |               |
    ///
    /// 1/(u a_shell_node) = 1/H_tube + 1/H_shell
    pub fn overall_htc_based_on_conductance(&mut self,
        correct_for_prandtl_wall_temperatures: bool,
        tube_side_total_mass_flowrate: MassRate,
        shell_side_total_mass_flowrate: MassRate,) -> HeatTransfer {

        self.set_tube_side_total_mass_flowrate(tube_side_total_mass_flowrate);
        self.set_shell_side_total_mass_flowrate(shell_side_total_mass_flowrate);


        let single_tube_to_shell_side_fluid_conductance: ThermalConductance
            = self.get_shell_side_fluid_to_single_inner_pipe_shell_nodal_conductance(
                correct_for_prandtl_wall_temperatures).unwrap();
        let single_tube_to_tube_side_fluid_conductance: ThermalConductance
            = self.get_single_tube_side_fluid_array_node_to_inner_pipe_shell_nodal_conductance(
                correct_for_prandtl_wall_temperatures).unwrap();

        let one_over_ua_shell_node: ThermalResistance = 
            single_tube_to_shell_side_fluid_conductance.recip() +
            single_tube_to_tube_side_fluid_conductance.recip();

        let ua_shell_node: ThermalConductance = 
            one_over_ua_shell_node.recip();

        // a_shell for one node
        let number_of_temperature_nodes = self.inner_nodes + 2;
        let mut single_inner_tube_fluid_arr_clone: FluidArray = 
            self.tube_side_fluid_array_for_single_tube.clone().try_into().unwrap();
        let heated_length = single_inner_tube_fluid_arr_clone
            .get_component_length();

        let od = self.tube_side_od;


        let node_length = heated_length / 
            number_of_temperature_nodes as f64;

        // area of shell for one node is PI D l 

        let area_shell_for_one_node: Area = PI * od * node_length;

        // obtain u

        let u_from_conductance: HeatTransfer = 
            ua_shell_node/area_shell_for_one_node;

        return u_from_conductance;


    }

    /// provides the overall heat transfer coefficient based on the 
    /// shell side area
    ///
    /// note that this assumes the tubes are circular, not oddly 
    /// shaped
    ///
    /// as mentioned by Du:
    ///
    /// 1/U = 1/h_t d_o/d_i + d_o/(2 lambda_w) ln (d_o/d_i) 
    /// + 1/h_s
    pub fn overall_heat_transfer_coeff_u_shell_side(&self,
        correct_for_prandtl_wall_temperatures: bool) 
        -> Result<HeatTransfer, TuasLibError>{

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

            let tube_fluid_temperature: ThermodynamicTemperature 
                = tube_side_single_fluid_array_clone.try_get_bulk_temperature()?;

            let wall_temperature: ThermodynamicTemperature 
                = pipe_shell_clone.try_get_bulk_temperature()?;

            let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);

            let pipe_shell_surf_temperature: ThermodynamicTemperature 
                = pipe_shell_clone.try_get_bulk_temperature()?;

            let single_tube_hydraulic_diameter = 
                self.get_tube_side_hydraulic_diameter_circular_tube();
            let single_tube_flow_area: Area = 
                tube_side_single_fluid_array_clone.get_cross_sectional_area_immutable();

            // flow area and hydraulic diameter are ok


            let tube_fluid_material: LiquidMaterial
                = tube_side_single_fluid_array_clone.material_control_volume.try_into()?;

            let solid_material: SolidMaterial 
                = pipe_shell_clone.material_control_volume.try_into()?;

            let viscosity: DynamicViscosity = 
                tube_fluid_material.try_get_dynamic_viscosity(tube_fluid_temperature)?;

            // need to convert hydraulic diameter to an equivalent 
            // spherical diameter
            //
            // but for now, I'm going to use Re and Nu using hydraulic diameter 
            // and live with it for the time being
            //
            let reynolds_number_single_tube_abs_for_nusselt_estimate: Ratio = 
                (single_tube_mass_flowrate/
                single_tube_flow_area
                *single_tube_hydraulic_diameter / viscosity).abs();

            // next, bulk prandtl number 

            let bulk_prandtl_number_tube_side: Ratio 
                = tube_fluid_material.try_get_prandtl_liquid(
                    tube_fluid_temperature,
                    atmospheric_pressure
                )?;

            let wall_prandtl_number_tube_side: Ratio;

            let nusselt_estimate_tube_side: Ratio; 
            
            // darcy friction factor
            // todo: this only works for things in a pipe form
            let darcy_friction_factor_tube_side: Ratio = self.
                tube_side_custom_component_loss_correlation.
                darcy_friction_factor(reynolds_number_single_tube_abs_for_nusselt_estimate)
                .unwrap();


            if correct_for_prandtl_wall_temperatures {

                // just use the wall prandtl number 
                // if the number falls outside the range of correlations,
                // then use the prandtl number at the max or min 

                let mut wall_temperature_estimate = wall_temperature;

                if wall_temperature_estimate > tube_fluid_material.max_temperature() {

                    wall_temperature_estimate = tube_fluid_material.max_temperature();

                } else if wall_temperature_estimate < tube_fluid_material.min_temperature() {

                    wall_temperature_estimate = tube_fluid_material.min_temperature();

                }

                wall_prandtl_number_tube_side
                    = tube_fluid_material.try_get_prandtl_liquid(
                        wall_temperature_estimate,
                        atmospheric_pressure
                    )?;

                nusselt_estimate_tube_side = 
                    self.tube_side_nusselt_correlation
                    .estimate_based_on_prandtl_darcy_and_reynolds_wall_correction(
                        bulk_prandtl_number_tube_side, 
                        wall_prandtl_number_tube_side, 
                        darcy_friction_factor_tube_side,
                        reynolds_number_single_tube_abs_for_nusselt_estimate)?;
            } else {
                nusselt_estimate_tube_side = 
                    self.tube_side_nusselt_correlation
                    .estimate_based_on_prandtl_darcy_and_reynolds_wall_correction(
                        bulk_prandtl_number_tube_side, 
                        bulk_prandtl_number_tube_side, 
                        darcy_friction_factor_tube_side,
                        reynolds_number_single_tube_abs_for_nusselt_estimate)?;
            }


            let k_fluid_average_tube_side: ThermalConductivity = 
                tube_fluid_material.try_get_thermal_conductivity(
                    tube_fluid_temperature)?;

            let tube_side_h
                = nusselt_estimate_tube_side 
                * k_fluid_average_tube_side / single_tube_hydraulic_diameter;
            
            let d_o_by_d_i: Ratio = 
                self.tube_side_od/self.tube_side_id;

            // 1/h_t * d_o/d_i 
            let one_over_ht_times_do_by_di = 
                1.0 as f64 / tube_side_h 
                * d_o_by_d_i;
            
            // now for the shell conductivity part 
            let shell_conductivity_average: ThermalConductivity = 
                solid_material.try_get_thermal_conductivity(
                    pipe_shell_surf_temperature)?;

            let ln_d_o_by_d_i: f64 = 
                d_o_by_d_i.get::<ratio>().ln();

            // d_o/(2 lambda_w) * ln (d_o_by_d_i) 

            let do_by_2_lambda_w_times_ln_do_by_di 
                = self.tube_side_od * 0.5 /
                (shell_conductivity_average) * 
                ln_d_o_by_d_i;

            // now for shell side heat transfer coeff

            // the thermal conductance here should be based on the 
            // nusselt number correlation

            // before any calculations, I will first need a clone of 
            // the fluid array and twisted tape array
            let mut shell_side_fluid_array_clone: FluidArray = 
                self.shell_side_fluid_array.clone().try_into()?;

            // also need to get basic temperatures and mass flowrates 
            // only do this once because some of these methods involve 
            // cloning, which is computationally expensive

            let shell_side_mass_flowrate: MassRate = 
                shell_side_fluid_array_clone.get_mass_flowrate();

            let shell_side_fluid_temperature: ThermodynamicTemperature 
                = shell_side_fluid_array_clone.try_get_bulk_temperature()?;

            let wall_temperature: ThermodynamicTemperature 
                = pipe_shell_clone.try_get_bulk_temperature()?;

            let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);

            let shell_side_fluid_hydraulic_diameter = 
                self.get_shell_side_hydraulic_diameter();

            let shell_side_cross_sectional_flow_area: Area = 
                self.get_shell_side_cross_sectional_area();


            // flow area and hydraulic diameter are ok


            let shell_fluid_material: LiquidMaterial
                = shell_side_fluid_array_clone.material_control_volume.try_into()?;

            let viscosity: DynamicViscosity = 
                shell_fluid_material.try_get_dynamic_viscosity(shell_side_fluid_temperature)?;

            // need to convert hydraulic diameter to an equivalent 
            // spherical diameter
            //
            // but for now, I'm going to use Re and Nu using hydraulic diameter 
            // and live with it for the time being
            //
            let reynolds_number_shell_side_abs_for_nusselt_estimate: Ratio = 
                (shell_side_mass_flowrate/
                shell_side_cross_sectional_flow_area
                *shell_side_fluid_hydraulic_diameter / viscosity).abs();

            // next, bulk prandtl number 

            let bulk_prandtl_number_shell_side: Ratio 
                = shell_fluid_material.try_get_prandtl_liquid(
                    shell_side_fluid_temperature,
                    atmospheric_pressure
                )?;



            let shell_side_fluid_to_inner_tube_surf_nusselt_correlation: NusseltCorrelation
                = self.shell_side_nusselt_correlation_to_tubes;
            let fldk_shell_side: Ratio = self.
                shell_side_custom_component_loss_correlation.
                fldk_based_on_darcy_friction_factor(reynolds_number_shell_side_abs_for_nusselt_estimate)
                .unwrap();
            let pipe_length = shell_side_fluid_array_clone.get_component_length_immutable();

            // f + d/L * K
            let modifed_darcy_friction_factor_shell_side = 
                fldk_shell_side * shell_side_fluid_hydraulic_diameter/pipe_length;


            // I need to use Nusselt correlations present in this struct 
            //
            // wall correction is optionally done here
            //
            // this uses the gnielinski correlation for pipes or tubes

            let nusselt_estimate_shell_side: Ratio;

            if correct_for_prandtl_wall_temperatures {


                // just use the wall prandtl number 
                // if the number falls outside the range of correlations,
                // then use the prandtl number at the max or min 

                let mut wall_temperature_estimate = wall_temperature;

                if wall_temperature_estimate > shell_fluid_material.max_temperature() {

                    wall_temperature_estimate = shell_fluid_material.max_temperature();

                } else if wall_temperature_estimate < shell_fluid_material.min_temperature() {

                    wall_temperature_estimate = shell_fluid_material.min_temperature();

                }

                let wall_prandtl_number_estimate: Ratio 
                    = shell_fluid_material.try_get_prandtl_liquid(
                        wall_temperature_estimate,
                        atmospheric_pressure
                    )?;


                nusselt_estimate_shell_side = shell_side_fluid_to_inner_tube_surf_nusselt_correlation.
                    estimate_based_on_prandtl_darcy_and_reynolds_wall_correction(
                        bulk_prandtl_number_shell_side, 
                        wall_prandtl_number_estimate,
                        modifed_darcy_friction_factor_shell_side,
                        reynolds_number_shell_side_abs_for_nusselt_estimate)?;

            } else {
                nusselt_estimate_shell_side = shell_side_fluid_to_inner_tube_surf_nusselt_correlation.
                    estimate_based_on_prandtl_darcy_and_reynolds_no_wall_correction(
                        bulk_prandtl_number_shell_side, 
                        modifed_darcy_friction_factor_shell_side,
                        reynolds_number_shell_side_abs_for_nusselt_estimate)?;

            }



            // now we can get the heat transfer coeff, 

            let shell_side_h_to_fluid: HeatTransfer;

            let k_fluid_average_shell_side: ThermalConductivity = 
                shell_fluid_material.try_get_thermal_conductivity(
                    shell_side_fluid_temperature)?;

            shell_side_h_to_fluid = nusselt_estimate_shell_side * k_fluid_average_shell_side / shell_side_fluid_hydraulic_diameter;

            //// this was for debugging
            //dbg!(
            //    &(nusselt_estimate_shell_side,
            //        nusselt_estimate_tube_side,
            //        shell_side_fluid_temperature.get::<uom::si::thermodynamic_temperature::degree_celsius>(),
            //        reynolds_number_shell_side_abs_for_nusselt_estimate,
            //        reynolds_number_single_tube_abs_for_nusselt_estimate)
            //);

            // 1/h_s 
            let one_over_hs = 1.0 as f64  / shell_side_h_to_fluid;



            // 1/U = 1/h_t d_o/d_i + d_o/(2 lambda_w) ln (d_o/d_i) 
            // + 1/h_s


            let one_over_u = 
                one_over_ht_times_do_by_di + 
                do_by_2_lambda_w_times_ln_do_by_di +
                one_over_hs;

            // used in debugging tube side nusselt
            //dbg!(&(
            //        reynolds_number_single_tube_abs_for_nusselt_estimate,
            //        self.tube_side_custom_component_loss_correlation,
            //        darcy_friction_factor_tube_side,
            //        bulk_prandtl_number_tube_side,
            //        wall_prandtl_number_tube_side,
            //        nusselt_estimate_tube_side
            //)
            //);

            //dbg!(&(
            //        reynolds_number_shell_side_abs_for_nusselt_estimate,
            //        bulk_prandtl_number_shell_side,
            //        nusselt_estimate_shell_side,
            //        one_over_u,
            //        one_over_ht_times_do_by_di,
            //        tube_side_h,
            //        shell_conductivity_average,
            //        pipe_shell_surf_temperature,
            //)
            //);


            // overall heat transfer coeff shell side
            let u_based_on_shell_side_area: HeatTransfer = 
                1.0 as f64 / one_over_u;

            Ok(u_based_on_shell_side_area)
        }

    /// provides nusselt number for tube side 
    /// wall correction provided by default using a Prandtl number 
    /// correction
    #[inline]
    pub fn nusselt_tube_side(&self) -> Ratio {
        // the thermal conductance here should be based on the 
        // nusselt number correlation

        // before any calculations, I will first need a clone of 
        // the fluid array and inner shell array
        //
        // the fluid array represents only a single tube
        let mut tube_side_single_fluid_array_clone: FluidArray = 
            self.tube_side_fluid_array_for_single_tube.clone().try_into().unwrap();


        let mut pipe_shell_clone: SolidColumn = 
            self.inner_pipe_shell_array_for_single_tube.clone().try_into().unwrap();

        // also need to get basic temperatures and mass flowrates 
        // only do this once because some of these methods involve 
        // cloning, which is computationally expensive

        let single_tube_mass_flowrate: MassRate = 
            tube_side_single_fluid_array_clone.get_mass_flowrate();

        let tube_fluid_temperature: ThermodynamicTemperature 
            = tube_side_single_fluid_array_clone.try_get_bulk_temperature().unwrap();

        let wall_temperature: ThermodynamicTemperature 
            = pipe_shell_clone.try_get_bulk_temperature().unwrap();

        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);


        let single_tube_hydraulic_diameter = 
            self.get_tube_side_hydraulic_diameter_circular_tube();
        let single_tube_flow_area: Area = 
            tube_side_single_fluid_array_clone.get_cross_sectional_area_immutable();

        // flow area and hydraulic diameter are ok


        let tube_fluid_material: LiquidMaterial
            = tube_side_single_fluid_array_clone.material_control_volume.try_into().unwrap();


        let viscosity: DynamicViscosity = 
            tube_fluid_material.try_get_dynamic_viscosity(tube_fluid_temperature).unwrap();

        // need to convert hydraulic diameter to an equivalent 
        // spherical diameter
        //
        // but for now, I'm going to use Re and Nu using hydraulic diameter 
        // and live with it for the time being
        //
        let reynolds_number_single_tube_abs_for_nusselt_estimate: Ratio = 
            (single_tube_mass_flowrate/
             single_tube_flow_area
             *single_tube_hydraulic_diameter / viscosity).abs();

        // next, bulk prandtl number 

        let bulk_prandtl_number_tube_side: Ratio 
            = tube_fluid_material.try_get_prandtl_liquid(
                tube_fluid_temperature,
                atmospheric_pressure
            ).unwrap();

        let wall_prandtl_number_tube_side: Ratio;

        let nusselt_estimate_tube_side: Ratio; 

        // darcy friction factor
        // todo: this only works for things in a pipe form
        let darcy_friction_factor_tube_side: Ratio = self.
            tube_side_custom_component_loss_correlation.
            darcy_friction_factor(reynolds_number_single_tube_abs_for_nusselt_estimate)
            .unwrap();


        // just use the wall prandtl number 
        // if the number falls outside the range of correlations,
        // then use the prandtl number at the max or min 

        let mut wall_temperature_estimate = wall_temperature;

        if wall_temperature_estimate > tube_fluid_material.max_temperature() {

            wall_temperature_estimate = tube_fluid_material.max_temperature();

        } else if wall_temperature_estimate < tube_fluid_material.min_temperature() {

            wall_temperature_estimate = tube_fluid_material.min_temperature();

        }

        wall_prandtl_number_tube_side
            = tube_fluid_material.try_get_prandtl_liquid(
                wall_temperature_estimate,
                atmospheric_pressure
            ).unwrap();

        nusselt_estimate_tube_side = 
            self.tube_side_nusselt_correlation
            .estimate_based_on_prandtl_darcy_and_reynolds_wall_correction(
                bulk_prandtl_number_tube_side, 
                wall_prandtl_number_tube_side, 
                darcy_friction_factor_tube_side,
                reynolds_number_single_tube_abs_for_nusselt_estimate).unwrap();

        nusselt_estimate_tube_side
    }

    /// returns reynolds number for tube side for a single tube
    #[inline]
    pub fn reynolds_tube_side_single_tube(&self) -> Ratio {

        // the thermal conductance here should be based on the 
        // nusselt number correlation

        // before any calculations, I will first need a clone of 
        // the fluid array and inner shell array
        //
        // the fluid array represents only a single tube
        let mut tube_side_single_fluid_array_clone: FluidArray = 
            self.tube_side_fluid_array_for_single_tube.clone().try_into().unwrap();


        // also need to get basic temperatures and mass flowrates 
        // only do this once because some of these methods involve 
        // cloning, which is computationally expensive

        let single_tube_mass_flowrate: MassRate = 
            tube_side_single_fluid_array_clone.get_mass_flowrate();

        let tube_fluid_temperature: ThermodynamicTemperature 
            = tube_side_single_fluid_array_clone.try_get_bulk_temperature().unwrap();

        let single_tube_hydraulic_diameter = 
            self.get_tube_side_hydraulic_diameter_circular_tube();
        let single_tube_flow_area: Area = 
            tube_side_single_fluid_array_clone.get_cross_sectional_area_immutable();

        // flow area and hydraulic diameter are ok


        let tube_fluid_material: LiquidMaterial
            = tube_side_single_fluid_array_clone.material_control_volume.try_into().unwrap();


        let viscosity: DynamicViscosity = 
            tube_fluid_material.try_get_dynamic_viscosity(tube_fluid_temperature).unwrap();

        // need to convert hydraulic diameter to an equivalent 
        // spherical diameter
        //
        // but for now, I'm going to use Re and Nu using hydraulic diameter 
        // and live with it for the time being
        //
        let reynolds_number_single_tube_abs_for_nusselt_estimate: Ratio = 
            (single_tube_mass_flowrate/
             single_tube_flow_area
             *single_tube_hydraulic_diameter / viscosity).abs();

        reynolds_number_single_tube_abs_for_nusselt_estimate
    }

    /// bulk and wall prandtl number for tube side 
    #[inline]
    pub fn bulk_prandtl_number_tube_side(&self) -> (Ratio, Ratio) {
        // the thermal conductance here should be based on the 
        // nusselt number correlation

        // before any calculations, I will first need a clone of 
        // the fluid array and inner shell array
        //
        // the fluid array represents only a single tube
        let mut tube_side_single_fluid_array_clone: FluidArray = 
            self.tube_side_fluid_array_for_single_tube.clone().try_into().unwrap();


        let mut pipe_shell_clone: SolidColumn = 
            self.inner_pipe_shell_array_for_single_tube.clone().try_into().unwrap();

        // also need to get basic temperatures and mass flowrates 
        // only do this once because some of these methods involve 
        // cloning, which is computationally expensive


        let tube_fluid_temperature: ThermodynamicTemperature 
            = tube_side_single_fluid_array_clone.try_get_bulk_temperature().unwrap();

        let wall_temperature: ThermodynamicTemperature 
            = pipe_shell_clone.try_get_bulk_temperature().unwrap();

        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);



        // flow area and hydraulic diameter are ok


        let tube_fluid_material: LiquidMaterial
            = tube_side_single_fluid_array_clone.material_control_volume.try_into().unwrap();



        // next, bulk prandtl number 

        let bulk_prandtl_number_tube_side: Ratio 
            = tube_fluid_material.try_get_prandtl_liquid(
                tube_fluid_temperature,
                atmospheric_pressure
            ).unwrap();

        let wall_prandtl_number_tube_side: Ratio;



        // just use the wall prandtl number 
        // if the number falls outside the range of correlations,
        // then use the prandtl number at the max or min 

        let mut wall_temperature_estimate = wall_temperature;

        if wall_temperature_estimate > tube_fluid_material.max_temperature() {

            wall_temperature_estimate = tube_fluid_material.max_temperature();

        } else if wall_temperature_estimate < tube_fluid_material.min_temperature() {

            wall_temperature_estimate = tube_fluid_material.min_temperature();

        }

        wall_prandtl_number_tube_side
            = tube_fluid_material.try_get_prandtl_liquid(
                wall_temperature_estimate,
                atmospheric_pressure
            ).unwrap();


        return (bulk_prandtl_number_tube_side,wall_prandtl_number_tube_side);

    }

    /// provides nusselt number for shell side to tubes
    ///
    /// wall correction enabled by default
    pub fn nusselt_number_shell_side_to_tubes(&self) -> Ratio {

        // the thermal conductance here should be based on the 
        // nusselt number correlation

        // before any calculations, I will first need a clone of 
        // the fluid array and inner shell array
        //
        // the fluid array represents only a single tube


        let mut pipe_shell_clone: SolidColumn = 
            self.inner_pipe_shell_array_for_single_tube.clone().try_into().unwrap();



        // now for shell side heat transfer coeff

        // the thermal conductance here should be based on the 
        // nusselt number correlation

        // before any calculations, I will first need a clone of 
        // the fluid array and twisted tape array
        let mut shell_side_fluid_array_clone: FluidArray = 
            self.shell_side_fluid_array.clone().try_into().unwrap();

        // also need to get basic temperatures and mass flowrates 
        // only do this once because some of these methods involve 
        // cloning, which is computationally expensive

        let shell_side_mass_flowrate: MassRate = 
            shell_side_fluid_array_clone.get_mass_flowrate();

        let shell_side_fluid_temperature: ThermodynamicTemperature 
            = shell_side_fluid_array_clone.try_get_bulk_temperature().unwrap();

        let wall_temperature: ThermodynamicTemperature 
            = pipe_shell_clone.try_get_bulk_temperature().unwrap();

        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);

        let shell_side_fluid_hydraulic_diameter = 
            self.get_shell_side_hydraulic_diameter();

        let shell_side_cross_sectional_flow_area: Area = 
            self.get_shell_side_cross_sectional_area();


        // flow area and hydraulic diameter are ok


        let shell_fluid_material: LiquidMaterial
            = shell_side_fluid_array_clone.material_control_volume.try_into().unwrap();

        let viscosity: DynamicViscosity = 
            shell_fluid_material.try_get_dynamic_viscosity(shell_side_fluid_temperature).unwrap();

        // need to convert hydraulic diameter to an equivalent 
        // spherical diameter
        //
        // but for now, I'm going to use Re and Nu using hydraulic diameter 
        // and live with it for the time being
        //
        let reynolds_number_shell_side_abs_for_nusselt_estimate: Ratio = 
            (shell_side_mass_flowrate/
             shell_side_cross_sectional_flow_area
             *shell_side_fluid_hydraulic_diameter / viscosity).abs();

        // next, bulk prandtl number 

        let bulk_prandtl_number_shell_side: Ratio 
            = shell_fluid_material.try_get_prandtl_liquid(
                shell_side_fluid_temperature,
                atmospheric_pressure
            ).unwrap();



        let shell_side_fluid_to_inner_tube_surf_nusselt_correlation: NusseltCorrelation
            = self.shell_side_nusselt_correlation_to_tubes;
        let fldk_shell_side: Ratio = self.
            shell_side_custom_component_loss_correlation.
            fldk_based_on_darcy_friction_factor(reynolds_number_shell_side_abs_for_nusselt_estimate)
            .unwrap();
        let pipe_length = shell_side_fluid_array_clone.get_component_length_immutable();

        // f + d/L * K
        let modifed_darcy_friction_factor_shell_side = 
            fldk_shell_side * shell_side_fluid_hydraulic_diameter/pipe_length;



        // I need to use Nusselt correlations present in this struct 
        //
        // wall correction is optionally done here
        //
        // this uses the gnielinski correlation for pipes or tubes

        let nusselt_estimate_shell_side: Ratio;



        // just use the wall prandtl number 
        // if the number falls outside the range of correlations,
        // then use the prandtl number at the max or min 

        let mut wall_temperature_estimate = wall_temperature;

        if wall_temperature_estimate > shell_fluid_material.max_temperature() {

            wall_temperature_estimate = shell_fluid_material.max_temperature();

        } else if wall_temperature_estimate < shell_fluid_material.min_temperature() {

            wall_temperature_estimate = shell_fluid_material.min_temperature();

        }

        let wall_prandtl_number_estimate: Ratio 
            = shell_fluid_material.try_get_prandtl_liquid(
                wall_temperature_estimate,
                atmospheric_pressure
            ).unwrap();


        nusselt_estimate_shell_side = shell_side_fluid_to_inner_tube_surf_nusselt_correlation.
            estimate_based_on_prandtl_darcy_and_reynolds_wall_correction(
                bulk_prandtl_number_shell_side, 
                wall_prandtl_number_estimate,
                modifed_darcy_friction_factor_shell_side,
                reynolds_number_shell_side_abs_for_nusselt_estimate).unwrap();


        nusselt_estimate_shell_side
    }

    /// provides reynolds number for shell side (both to tubes and 
    /// for parasitic heat loss, ie to outer shell)
    pub fn reynolds_shell_side(&self) -> Ratio {

        let mut shell_side_fluid_array_clone: FluidArray = 
            self.shell_side_fluid_array.clone().try_into().unwrap();


        let shell_side_mass_flowrate: MassRate = 
            shell_side_fluid_array_clone.get_mass_flowrate();

        let shell_side_fluid_temperature: ThermodynamicTemperature 
            = shell_side_fluid_array_clone.try_get_bulk_temperature().unwrap();


        let shell_side_fluid_hydraulic_diameter = 
            self.get_shell_side_hydraulic_diameter();

        let shell_side_cross_sectional_flow_area: Area = 
            self.get_shell_side_cross_sectional_area();


        let shell_fluid_material: LiquidMaterial
            = shell_side_fluid_array_clone.material_control_volume.try_into().unwrap();

        let viscosity: DynamicViscosity = 
            shell_fluid_material.try_get_dynamic_viscosity(shell_side_fluid_temperature).unwrap();

        //
        let reynolds_number_shell_side_abs_for_nusselt_estimate: Ratio = 
            (shell_side_mass_flowrate/
             shell_side_cross_sectional_flow_area
             *shell_side_fluid_hydraulic_diameter / viscosity).abs();

        reynolds_number_shell_side_abs_for_nusselt_estimate

    }

    /// provides bulk prandtl number for shell side
    pub fn bulk_prandtl_number_shell_side(&self) -> Ratio {

        let mut shell_side_fluid_array_clone: FluidArray = 
            self.shell_side_fluid_array.clone().try_into().unwrap();


        let shell_side_fluid_temperature: ThermodynamicTemperature 
            = shell_side_fluid_array_clone.try_get_bulk_temperature().unwrap();

        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);

        let shell_fluid_material: LiquidMaterial
            = shell_side_fluid_array_clone.material_control_volume.try_into().unwrap();


        // next, bulk prandtl number 

        let bulk_prandtl_number_shell_side: Ratio 
            = shell_fluid_material.try_get_prandtl_liquid(
                shell_side_fluid_temperature,
                atmospheric_pressure
            ).unwrap();




        bulk_prandtl_number_shell_side

    }

    /// provides wall prandtl number based on inner tube temperature 
    pub fn wall_prandtl_number_shell_side_fluid_for_inner_tube(&self) -> Ratio {

        // the thermal conductance here should be based on the 
        // nusselt number correlation

        // before any calculations, I will first need a clone of 
        // the fluid array and inner shell array
        //
        // the fluid array represents only a single tube


        let mut pipe_shell_clone: SolidColumn = 
            self.inner_pipe_shell_array_for_single_tube.clone().try_into().unwrap();



        let shell_side_fluid_array_clone: FluidArray = 
            self.shell_side_fluid_array.clone().try_into().unwrap();


        let wall_temperature: ThermodynamicTemperature 
            = pipe_shell_clone.try_get_bulk_temperature().unwrap();

        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);



        // flow area and hydraulic diameter are ok


        let shell_fluid_material: LiquidMaterial
            = shell_side_fluid_array_clone.material_control_volume.try_into().unwrap();





        // just use the wall prandtl number 
        // if the number falls outside the range of correlations,
        // then use the prandtl number at the max or min 

        let mut wall_temperature_estimate = wall_temperature;

        if wall_temperature_estimate > shell_fluid_material.max_temperature() {

            wall_temperature_estimate = shell_fluid_material.max_temperature();

        } else if wall_temperature_estimate < shell_fluid_material.min_temperature() {

            wall_temperature_estimate = shell_fluid_material.min_temperature();

        }

        let wall_prandtl_number_estimate_shell_side_to_tubes: Ratio 
            = shell_fluid_material.try_get_prandtl_liquid(
                wall_temperature_estimate,
                atmospheric_pressure
            ).unwrap();

        wall_prandtl_number_estimate_shell_side_to_tubes

    }
    /// provides wall prandtl number based on outer tube temperature 
    /// mainly for parasitic heat loss
    pub fn wall_prandtl_number_shell_side_fluid_for_outer_tube(&self) -> Ratio {

        // the thermal conductance here should be based on the 
        // nusselt number correlation

        // before any calculations, I will first need a clone of 
        // the fluid array and inner shell array
        //
        // the fluid array represents only a single tube


        let mut outer_pipe_shell_clone: SolidColumn = 
            self.outer_shell.clone().try_into().unwrap();



        let shell_side_fluid_array_clone: FluidArray = 
            self.shell_side_fluid_array.clone().try_into().unwrap();


        let wall_temperature: ThermodynamicTemperature 
            = outer_pipe_shell_clone.try_get_bulk_temperature().unwrap();

        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);



        // flow area and hydraulic diameter are ok


        let shell_fluid_material: LiquidMaterial
            = shell_side_fluid_array_clone.material_control_volume.try_into().unwrap();





        // just use the wall prandtl number 
        // if the number falls outside the range of correlations,
        // then use the prandtl number at the max or min 

        let mut wall_temperature_estimate = wall_temperature;

        if wall_temperature_estimate > shell_fluid_material.max_temperature() {

            wall_temperature_estimate = shell_fluid_material.max_temperature();

        } else if wall_temperature_estimate < shell_fluid_material.min_temperature() {

            wall_temperature_estimate = shell_fluid_material.min_temperature();

        }

        let wall_prandtl_number_estimate_shell_side_fluid_to_outer_shell: Ratio 
            = shell_fluid_material.try_get_prandtl_liquid(
                wall_temperature_estimate,
                atmospheric_pressure
            ).unwrap();

        wall_prandtl_number_estimate_shell_side_fluid_to_outer_shell

    }


    /// provides nusselt number to outer shell 
    #[inline]
    pub fn nusselt_number_shell_side_parasitic(&self) -> Ratio {



        let mut outer_shell_clone: SolidColumn = 
            self.outer_shell.clone().try_into().unwrap();

        let mut shell_side_fluid_array_clone: FluidArray = 
            self.shell_side_fluid_array.clone().try_into().unwrap();

        // also need to get basic temperatures and mass flowrates 
        // only do this once because some of these methods involve 
        // cloning, which is computationally expensive

        let shell_side_mass_flowrate: MassRate = 
            shell_side_fluid_array_clone.get_mass_flowrate();

        let shell_side_fluid_temperature: ThermodynamicTemperature 
            = shell_side_fluid_array_clone.try_get_bulk_temperature().unwrap();

        let wall_temperature: ThermodynamicTemperature 
            = outer_shell_clone.try_get_bulk_temperature().unwrap();

        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);

        let shell_side_fluid_hydraulic_diameter = 
            self.get_shell_side_hydraulic_diameter();

        let shell_side_cross_sectional_flow_area: Area = 
            self.get_shell_side_cross_sectional_area();


        // flow area and hydraulic diameter are ok


        let shell_fluid_material: LiquidMaterial
            = shell_side_fluid_array_clone.material_control_volume.try_into().unwrap();

        let viscosity: DynamicViscosity = 
            shell_fluid_material.try_get_dynamic_viscosity(shell_side_fluid_temperature).unwrap();

        // need to convert hydraulic diameter to an equivalent 
        // spherical diameter
        //
        // but for now, I'm going to use Re and Nu using hydraulic diameter 
        // and live with it for the time being
        //
        let reynolds_number_shell_side_abs_for_nusselt_estimate: Ratio = 
            (shell_side_mass_flowrate/
             shell_side_cross_sectional_flow_area
             *shell_side_fluid_hydraulic_diameter / viscosity).abs();

        // next, bulk prandtl number 

        let bulk_prandtl_number_shell_side: Ratio 
            = shell_fluid_material.try_get_prandtl_liquid(
                shell_side_fluid_temperature,
                atmospheric_pressure
            ).unwrap();



        let shell_side_fluid_parasitic_nusselt_correlation: NusseltCorrelation
            = self.shell_side_nusselt_correlation_parasitic;

        let fldk_shell_side: Ratio = self.
            shell_side_custom_component_loss_correlation.
            fldk_based_on_darcy_friction_factor(reynolds_number_shell_side_abs_for_nusselt_estimate)
            .unwrap();

        let pipe_length = shell_side_fluid_array_clone.get_component_length_immutable();

        // f + d/L * K
        let modifed_darcy_friction_factor_shell_side = 
            fldk_shell_side * shell_side_fluid_hydraulic_diameter/pipe_length;



        // I need to use Nusselt correlations present in this struct 
        //
        // wall correction is optionally done here
        //
        // this uses the gnielinski correlation for pipes or tubes

        let nusselt_estimate_shell_side_to_outer_shell: Ratio;



        // just use the wall prandtl number 
        // if the number falls outside the range of correlations,
        // then use the prandtl number at the max or min 

        let mut wall_temperature_estimate = wall_temperature;

        if wall_temperature_estimate > shell_fluid_material.max_temperature() {

            wall_temperature_estimate = shell_fluid_material.max_temperature();

        } else if wall_temperature_estimate < shell_fluid_material.min_temperature() {

            wall_temperature_estimate = shell_fluid_material.min_temperature();

        }

        let wall_prandtl_number_estimate: Ratio 
            = shell_fluid_material.try_get_prandtl_liquid(
                wall_temperature_estimate,
                atmospheric_pressure
            ).unwrap();


        nusselt_estimate_shell_side_to_outer_shell = 
            shell_side_fluid_parasitic_nusselt_correlation.
            estimate_based_on_prandtl_darcy_and_reynolds_wall_correction(
                bulk_prandtl_number_shell_side, 
                wall_prandtl_number_estimate,
                modifed_darcy_friction_factor_shell_side,
                reynolds_number_shell_side_abs_for_nusselt_estimate).unwrap();


        nusselt_estimate_shell_side_to_outer_shell

    }

    /// provides the tube bundle side heat transfer area 
    /// on the shell side
    ///
    /// assuming the bundle of inner tubes is circular
    pub fn circular_tube_bundle_heat_transfer_area_shell_side(&self) -> Area {

        let n_t = self.number_of_tubes;
        let pipe_shell_clone: SolidColumn = 
            self.inner_pipe_shell_array_for_single_tube.
            clone().try_into().unwrap();

        let l = pipe_shell_clone.get_component_length();

        let d_o = self.tube_side_od;

        let shell_area: Area = 
            n_t as f64 * PI * d_o * l;

        return shell_area;


    }
    /// provides the tube bundle side heat transfer area 
    /// on the shell side
    ///
    /// assuming the bundle of inner tubes is circular
    pub fn parasitic_heat_transfer_area_shell_side(&self) -> Area {

        let pipe_shell_clone: SolidColumn = 
            self.inner_pipe_shell_array_for_single_tube.
            clone().try_into().unwrap();

        let l = pipe_shell_clone.get_component_length();

        let d_i = self.shell_side_id;

        let shell_area: Area = PI * d_i * l;

        return shell_area;


    }
    /// provides the tube bundle side heat transfer area 
    /// on the shell side
    ///
    /// assuming the bundle of inner tubes is circular
    pub fn circular_tube_bundle_heat_transfer_area_tube_side(&self) -> Area {

        let n_t = self.number_of_tubes;
        let pipe_shell_clone: SolidColumn = 
            self.inner_pipe_shell_array_for_single_tube.
            clone().try_into().unwrap();

        let l = pipe_shell_clone.get_component_length();

        let d_i = self.tube_side_id;

        let tube_side_area: Area = 
            n_t as f64 * PI * d_i * l;

        return tube_side_area;


    }



}
