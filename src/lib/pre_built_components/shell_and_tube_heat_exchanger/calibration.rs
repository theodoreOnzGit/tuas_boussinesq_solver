use std::f64::consts::PI;

use uom::si::{f64::*, ratio::ratio, thermal_resistance::kelvin_per_watt, thermodynamic_temperature::kelvin};

use crate::{prelude::beta_testing::{FluidArray, LiquidMaterial, SolidColumn, SolidMaterial}, thermal_hydraulics_error::TuasLibError};

use super::SimpleShellAndTubeHeatExchanger;

impl SimpleShellAndTubeHeatExchanger {

    /// assuming sthe insulation is cylindrical,
    /// get the thermal resistance
    ///
    /// ln (d_o/d_i) * 1/(2 pi L lambda_insulation)
    pub fn try_get_insulation_cylindrical_thermal_resistance(&self) -> 
        Result<ThermalResistance, TuasLibError> {

        if !self.heat_exchanger_has_insulation {

            return Ok(ThermalResistance::new::<kelvin_per_watt>(0.0));
        }
        let insulation_id = self.shell_side_od;
        let insulation_od = insulation_id + 2.0*self.insulation_thickness;

        let l = self.get_effective_length();

        let lambda_insulation: ThermalConductivity;

        let mut insulation_array_clone: SolidColumn = self.insulation_array
            .clone()
            .try_into()
            .unwrap();

        let insulation_array_temp: ThermodynamicTemperature = 
            insulation_array_clone
            .try_get_bulk_temperature()
            .unwrap();

        let insulation_array_material: SolidMaterial = 
            insulation_array_clone.material_control_volume
            .clone()
            .try_into()
            .unwrap();

        lambda_insulation = 
            insulation_array_material
            .try_get_thermal_conductivity(
                insulation_array_temp
            ).unwrap();


        let log_numerator_term: f64 = 
            (insulation_od/insulation_id)
            .get::<ratio>()
            .ln();

            let denominator_term: ThermalConductance = 
                2.0 * PI * lambda_insulation * l;


            return Ok(log_numerator_term/denominator_term);

        }


    /// assuming sthe outer shell is cylindrical,
    /// get the thermal resistance
    ///
    /// ln (d_o/d_i) * 1/(2 pi L lambda_insulation)
    pub fn get_outer_shell_cylindrical_thermal_resistance(&self) -> 
        ThermalResistance {



            let outer_shell_id = self.shell_side_id;
            let outer_shell_od = self.shell_side_od;

            let l = self.get_effective_length();

            let lambda_outer_shell: ThermalConductivity;

            let mut outer_shell_array_clone: SolidColumn = self.outer_shell
                .clone()
                .try_into()
                .unwrap();

            let outer_shell_array_temp: ThermodynamicTemperature = 
                outer_shell_array_clone
                .try_get_bulk_temperature()
                .unwrap();

            let outer_shell_array_material: SolidMaterial = 
                outer_shell_array_clone.material_control_volume
                .clone()
                .try_into()
                .unwrap();

            lambda_outer_shell = 
                outer_shell_array_material
                .try_get_thermal_conductivity(
                    outer_shell_array_temp
                ).unwrap();


            let log_numerator_term: f64 = 
                (outer_shell_od/outer_shell_id)
                .get::<ratio>()
                .ln();

            let denominator_term: ThermalConductance = 
                2.0 * PI * lambda_outer_shell * l;


            log_numerator_term/denominator_term

    }

    /// assuming sthe inner tubes are cylindrical parallel tubes,
    /// get the thermal resistance
    ///
    /// ln (d_o/d_i) * 1/(2 pi L lambda_insulation N_t)
    pub fn get_inner_tubes_cylindrical_thermal_resistance(&self) -> 
        ThermalResistance {

            let inner_tube_id = self.tube_side_id;
            let inner_tube_od = self.tube_side_od;

            let l = self.get_effective_length();

            let lambda_inner_tubes: ThermalConductivity;

            let mut inner_single_tube_array_clone: SolidColumn = self.inner_pipe_shell_array_for_single_tube
                .clone()
                .try_into()
                .unwrap();

            let inner_tube_array_temp: ThermodynamicTemperature = 
                inner_single_tube_array_clone
                .try_get_bulk_temperature()
                .unwrap();

            let inner_tube_array_material: SolidMaterial = 
                inner_single_tube_array_clone.material_control_volume
                .clone()
                .try_into()
                .unwrap();

            lambda_inner_tubes = 
                inner_tube_array_material
                .try_get_thermal_conductivity(
                    inner_tube_array_temp
                ).unwrap();


            let log_numerator_term: f64 = 
                (inner_tube_od/inner_tube_id)
                .get::<ratio>()
                .ln();

            let denominator_term: ThermalConductance = 
                2.0 * PI * lambda_inner_tubes * l * self.number_of_tubes as f64;


            log_numerator_term/denominator_term

    }


    /// assuming the outer shell or insulation is cylindrical,
    /// get the convective thermal resistance to ambient
    #[inline]
    pub fn get_convective_thermal_resistance_to_ambient(&self) -> ThermalResistance {

        let mut od = self.shell_side_od;

        if self.heat_exchanger_has_insulation {

            let id = self.shell_side_od;
            od = id + 2.0*self.insulation_thickness;
        }

        // now let's get the area 
        // pi * od * L 

        let outermost_heat_transfer_area: Area
            = PI * od * self.get_effective_length();

        let conductance_to_ambient: ThermalConductance 
            = self.heat_transfer_to_ambient * outermost_heat_transfer_area;
        conductance_to_ambient.recip()

    }

    /// get inner tube thermal resistance using wetted perimeter 
    /// d_h = 4A_xs/P_w 
    /// P_w = 4A_xs/d_h
    /// 
    /// for circle, P_w = pi * D
    ///
    /// for single_tube, heat transfer area = P_w * l
    #[inline] 
    pub fn get_inner_tubes_convective_thermal_resistance_based_on_wetted_perimeter(
        &self) -> ThermalResistance {

        let mut fluid_arr_clone_for_single_inner_tube: FluidArray = 
            self.tube_side_fluid_array_for_single_tube.clone().
            try_into().unwrap();

        let hydraulic_diameter: Length = 
            fluid_arr_clone_for_single_inner_tube.
            get_hydraulic_diameter_immutable();

        let flow_area: Area = 
            fluid_arr_clone_for_single_inner_tube.
            get_cross_sectional_area_immutable();

        let tube_length: Length = 
            fluid_arr_clone_for_single_inner_tube.
            get_component_length();

        // d_h = 4A/P_w
        let wetted_perimeter: Length = (4.0 * flow_area)/hydraulic_diameter;

        let single_tube_heat_trf_area: Area = 
            wetted_perimeter * tube_length;

        let area_tube_side_total: Area = 
            single_tube_heat_trf_area * self.number_of_tubes as f64;

        let nusselt_tube_side: Ratio = 
            self.nusselt_tube_side();

        // get h_t
        // Nu_t = h_t * d_t/lambda_t
        // lambda_t = thermal conductivity
        let tube_side_temperature: ThermodynamicTemperature = 
            fluid_arr_clone_for_single_inner_tube.
            try_get_bulk_temperature().unwrap();


        let tube_fluid_material: LiquidMaterial
            = fluid_arr_clone_for_single_inner_tube.material_control_volume.try_into().unwrap();

        let thermal_conductivity_tube_side_fluid: ThermalConductivity = 
            tube_fluid_material.try_get_thermal_conductivity(
                tube_side_temperature).unwrap();

        let h_t: HeatTransfer = 
            nusselt_tube_side * thermal_conductivity_tube_side_fluid / 
            hydraulic_diameter;


        // 1/(h_t A_t)
        (h_t * area_tube_side_total).recip()

    }
    /// get shell side thermal resistance assuming cylindrical tubing
    /// for circle, P_w = pi * d_o
    ///
    /// for single_tube, heat transfer area = P_w * l
    /// for all tubes, heat_transfer_area = P_w * l * N_t
    #[inline] 
    pub fn get_shell_side_convective_thermal_resistance_cylindrical(
        &self) -> ThermalResistance {

        let mut shell_side_fluid_arr_clone: FluidArray = 
            self.shell_side_fluid_array.clone().
            try_into().unwrap();

        let shell_length: Length = 
            shell_side_fluid_arr_clone.
            get_component_length();

        let wetted_perimeter: Length = PI * self.tube_side_od;

        let single_tube_heat_trf_area: Area = 
            wetted_perimeter * shell_length;

        let area_shell_side_total: Area = 
            single_tube_heat_trf_area * self.number_of_tubes as f64;

        let nusselt_shell_side: Ratio = 
            self.nusselt_number_shell_side_to_tubes();

        // get h_s
        // Nu_s = h_s * D_e/lambda_s
        // lambda_s = thermal conductivity
        let shell_side_temperature: ThermodynamicTemperature = 
            shell_side_fluid_arr_clone.
            try_get_bulk_temperature().unwrap();


        let shell_fluid_material: LiquidMaterial
            = shell_side_fluid_arr_clone.material_control_volume.try_into().unwrap();

        let thermal_conductivity_shell_side_fluid: ThermalConductivity = 
            shell_fluid_material.try_get_thermal_conductivity(
                shell_side_temperature).unwrap();

        let hydraulic_diameter_shell_side: Length 
            = shell_side_fluid_arr_clone.
            get_hydraulic_diameter_immutable();

        let h_s: HeatTransfer = 
            nusselt_shell_side * thermal_conductivity_shell_side_fluid / 
            hydraulic_diameter_shell_side;


        // 1/(h_s A_s)
        (h_s * area_shell_side_total).recip()

    }
    /// get shell side thermal resistance assuming cylindrical tubing
    /// for circle, P_w = pi * d_o
    ///
    /// for single_tube, heat transfer area = P_w * l
    /// assumes cylindrical shell
    #[inline] 
    pub fn get_shell_side_parasitic_convective_thermal_resistance_cylindrical(
        &self) -> ThermalResistance {

        let mut shell_side_fluid_arr_clone: FluidArray = 
            self.shell_side_fluid_array.clone().
            try_into().unwrap();

        let shell_length: Length = 
            shell_side_fluid_arr_clone.
            get_component_length();

        let wetted_perimeter: Length = PI * self.shell_side_id;


        let area_shell_side_total: Area = wetted_perimeter * shell_length;

        let parasitic_nusselt_number: Ratio = 
            self.nusselt_number_shell_side_parasitic();

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

        let hydraulic_diameter_shell_side: Length 
            = shell_side_fluid_arr_clone.
            get_hydraulic_diameter_immutable();

        let h_parasitic: HeatTransfer = 
            parasitic_nusselt_number * thermal_conductivity_shell_side_fluid / 
            hydraulic_diameter_shell_side;


        // 1/(h_parasitic A_outer)
        (h_parasitic * area_shell_side_total).recip()

    }

    /// obtains shell side heat gain or loss based on 
    /// temperature difference and mass flowrate
    ///
    /// Q = m cp delta T
    ///
    /// mass flows from inlet to outlet by convention
    #[inline]
    pub fn get_shell_side_heat_rate_based_on_mass_flowrate(
        &self,
        shell_inlet_temperature: ThermodynamicTemperature,
        shell_outlet_temeprature: ThermodynamicTemperature,
        shell_mass_flowrate: MassRate) -> Power {

        let mut shell_side_fluid_arr_clone: FluidArray = 
            self.shell_side_fluid_array.clone().
            try_into().unwrap();

        let shell_fluid_material: LiquidMaterial
            = shell_side_fluid_arr_clone.material_control_volume.try_into().unwrap();

        let shell_side_temperature: ThermodynamicTemperature = 
            shell_side_fluid_arr_clone.
            try_get_bulk_temperature().unwrap();

        let cp_shell_side_fluid: SpecificHeatCapacity = 
            shell_fluid_material.try_get_cp(
                shell_side_temperature).unwrap();

        let delta_t: TemperatureInterval = 
            TemperatureInterval::new::<uom::si::temperature_interval::kelvin>(
                shell_outlet_temeprature.get::<kelvin>()
                - shell_inlet_temperature.get::<kelvin>()
            );

        shell_mass_flowrate * cp_shell_side_fluid * delta_t
    }
    /// obtains shell side heat gain or loss based on 
    /// temperature difference and vol flowrate
    ///
    /// Q = V  rho cp delta T
    ///
    /// mass flows from inlet to outlet by convention
    #[inline]
    pub fn get_shell_side_heat_rate_based_on_vol_flowrate(
        &self,
        shell_inlet_temperature: ThermodynamicTemperature,
        shell_outlet_temeprature: ThermodynamicTemperature,
        vol_flowrate: VolumeRate) -> Power {

        let mut shell_side_fluid_arr_clone: FluidArray = 
            self.shell_side_fluid_array.clone().
            try_into().unwrap();

        let shell_fluid_material: LiquidMaterial
            = shell_side_fluid_arr_clone.material_control_volume.try_into().unwrap();

        let shell_side_temperature: ThermodynamicTemperature = 
            shell_side_fluid_arr_clone.
            try_get_bulk_temperature().unwrap();

        let cp_shell_side_fluid: SpecificHeatCapacity = 
            shell_fluid_material.try_get_cp(
                shell_side_temperature).unwrap();

        let rho_shell_side_fluid: MassDensity = 
            shell_fluid_material.try_get_density(
                shell_side_temperature).unwrap();


        let delta_t: TemperatureInterval = 
            TemperatureInterval::new::<uom::si::temperature_interval::kelvin>(
                shell_outlet_temeprature.get::<kelvin>()
                - shell_inlet_temperature.get::<kelvin>()
            );

        vol_flowrate * rho_shell_side_fluid * cp_shell_side_fluid * delta_t
    }

    /// obtains tube side heat gain or loss based on 
    /// temperature difference and mass flowrate
    ///
    /// Q = m cp delta T
    ///
    /// mass flows from inlet to outlet by convention
    #[inline]
    pub fn get_tube_side_heat_rate_based_on_mass_flowrate(
        &self,
        tube_inlet_temperature: ThermodynamicTemperature,
        tube_outlet_temeprature: ThermodynamicTemperature,
        tube_mass_flowrate: MassRate) -> Power {

        let mut tube_side_fluid_arr_clone: FluidArray = 
            self.tube_side_fluid_array_for_single_tube.clone().
            try_into().unwrap();

        let tube_fluid_material: LiquidMaterial
            = tube_side_fluid_arr_clone.material_control_volume.try_into().unwrap();

        let tube_side_temperature: ThermodynamicTemperature = 
            tube_side_fluid_arr_clone.
            try_get_bulk_temperature().unwrap();

        let cp_tube_side_fluid: SpecificHeatCapacity = 
            tube_fluid_material.try_get_cp(
                tube_side_temperature).unwrap();

        let delta_t: TemperatureInterval = 
            TemperatureInterval::new::<uom::si::temperature_interval::kelvin>(
                tube_outlet_temeprature.get::<kelvin>()
                - tube_inlet_temperature.get::<kelvin>()
            );

        tube_mass_flowrate * cp_tube_side_fluid * delta_t
    }
    /// obtains tube side heat gain or loss based on 
    /// temperature difference and vol flowrate
    ///
    /// Q = V  rho cp delta T
    ///
    /// mass flows from inlet to outlet by convention
    #[inline]
    pub fn get_tube_side_heat_rate_based_on_vol_flowrate(
        &self,
        tube_inlet_temperature: ThermodynamicTemperature,
        tube_outlet_temeprature: ThermodynamicTemperature,
        vol_flowrate: VolumeRate) -> Power {

        let mut tube_side_fluid_arr_clone: FluidArray = 
            self.tube_side_fluid_array_for_single_tube.clone().
            try_into().unwrap();

        let tube_fluid_material: LiquidMaterial
            = tube_side_fluid_arr_clone.material_control_volume.try_into().unwrap();

        let tube_side_temperature: ThermodynamicTemperature = 
            tube_side_fluid_arr_clone.
            try_get_bulk_temperature().unwrap();

        let cp_tube_side_fluid: SpecificHeatCapacity = 
            tube_fluid_material.try_get_cp(
                tube_side_temperature).unwrap();

        let rho_tube_side_fluid: MassDensity = 
            tube_fluid_material.try_get_density(
                tube_side_temperature).unwrap();


        let delta_t: TemperatureInterval = 
            TemperatureInterval::new::<uom::si::temperature_interval::kelvin>(
                tube_outlet_temeprature.get::<kelvin>()
                - tube_inlet_temperature.get::<kelvin>()
            );

        vol_flowrate * rho_tube_side_fluid * cp_tube_side_fluid * delta_t
    }

    /// gets the overall thermal resistance for heat 
    /// exchanger based on Q = UA (LMTD) 
    /// Q is based on shell or tube side, whichever is smaller 
    /// because we don't want to include parasitic heat losses 
    ///
    /// also allows you to specify if the sthe is counter current 
    /// manually
    #[inline]
    pub fn get_ua_based_on_mass_flowrates_and_temperature_differences(&self,
        tube_inlet_temperature: ThermodynamicTemperature,
        tube_outlet_temeprature: ThermodynamicTemperature,
        tube_mass_flowrate: MassRate,
        shell_inlet_temperature: ThermodynamicTemperature,
        shell_outlet_temeprature: ThermodynamicTemperature,
        shell_mass_flowrate: MassRate,
        is_counter_current: bool) -> ThermalConductance {

        let tube_side_heat_rate: Power = 
            self.get_tube_side_heat_rate_based_on_mass_flowrate(
                tube_inlet_temperature, 
                tube_outlet_temeprature, 
                tube_mass_flowrate);

        let shell_side_heat_rate: Power = 
            self.get_shell_side_heat_rate_based_on_mass_flowrate(
                shell_inlet_temperature, 
                shell_outlet_temeprature, 
                shell_mass_flowrate);

        // get the smaller of the two, because there is likely to 
        // be parasitic heat loss 
        // note that it must be absolute, heat loss, because 
        // they are likely to be negative numbers
        let heat_transfer_rate_through_sthe_no_parasitic_losses: Power;

        if tube_side_heat_rate.abs() > shell_side_heat_rate.abs() {
            heat_transfer_rate_through_sthe_no_parasitic_losses = 
                shell_side_heat_rate;
        } else {

            heat_transfer_rate_through_sthe_no_parasitic_losses = 
                tube_side_heat_rate;
        }

        // then get ua 
        Self::get_ua_based_on_heat_transfer_and_temperature_differences(
            heat_transfer_rate_through_sthe_no_parasitic_losses, 
            tube_inlet_temperature, 
            tube_outlet_temeprature, 
            shell_inlet_temperature, 
            shell_outlet_temeprature, 
            is_counter_current)


    }

    /// gets the overall thermal resistance for heat 
    /// exchanger based on Q = UA (LMTD) 
    ///
    /// also allows you to specify if the sthe is counter current 
    /// manually
    /// 
    #[inline]
    pub fn get_ua_based_on_heat_transfer_and_temperature_differences(
        heat_transfer_rate: Power,
        tube_inlet_temperature: ThermodynamicTemperature,
        tube_outlet_temeprature: ThermodynamicTemperature,
        shell_inlet_temperature: ThermodynamicTemperature,
        shell_outlet_temeprature: ThermodynamicTemperature,
        is_counter_current: bool,) -> ThermalConductance {

        let delta_t_min: TemperatureInterval;
        let delta_t_max: TemperatureInterval;

        if is_counter_current {
            delta_t_min = TemperatureInterval::new::<uom::si::temperature_interval::kelvin>(
                shell_inlet_temperature.get::<kelvin>() 
                - tube_outlet_temeprature.get::<kelvin>()
            );
            delta_t_max = TemperatureInterval::new::<uom::si::temperature_interval::kelvin>(
                shell_outlet_temeprature.get::<kelvin>() 
                - tube_inlet_temperature.get::<kelvin>()
            );
        } else {
            delta_t_min = TemperatureInterval::new::<uom::si::temperature_interval::kelvin>(
                shell_outlet_temeprature.get::<kelvin>() 
                - tube_outlet_temeprature.get::<kelvin>()
            );
            delta_t_max = TemperatureInterval::new::<uom::si::temperature_interval::kelvin>(
                shell_inlet_temperature.get::<kelvin>() 
                - tube_inlet_temperature.get::<kelvin>()
            );

        }

        let log_mean_temperature_difference: TemperatureInterval;

        let numerator_term_lmtd: TemperatureInterval = 
            delta_t_max - delta_t_min;

        let denominator_term_lmtd: Ratio = 
            (delta_t_max/delta_t_min).get::<ratio>().ln().into();

        log_mean_temperature_difference = 
            numerator_term_lmtd/denominator_term_lmtd;


        return (heat_transfer_rate/log_mean_temperature_difference).abs();

    }

    /// obtain shell side Nusselt number based on prevailing flowrates 
    #[inline]
    pub fn obtain_shell_side_nusselt_number_based_on_expt_data(
        &self,
        tube_inlet_temperature: ThermodynamicTemperature,
        tube_outlet_temeprature: ThermodynamicTemperature,
        tube_mass_flowrate: MassRate,
        shell_inlet_temperature: ThermodynamicTemperature,
        shell_outlet_temeprature: ThermodynamicTemperature,
        shell_mass_flowrate: MassRate,
        is_counter_current: bool) -> Ratio {

        let overall_ua: ThermalConductance = 
            self.get_ua_based_on_mass_flowrates_and_temperature_differences(
                tube_inlet_temperature, 
                tube_outlet_temeprature, 
                tube_mass_flowrate, 
                shell_inlet_temperature, 
                shell_outlet_temeprature, 
                shell_mass_flowrate, 
                is_counter_current);

        let overall_thermal_resistance = overall_ua.recip();

        // 1/(U A)_overall = 1/(h_s A_s) + 1/(h_t A_t) + inner_tubes_thermal_resistance
        //
        // 1/(h_s A_s)
        let shell_side_thermal_resistance_expt_data = 
            overall_thermal_resistance
            - self.get_inner_tubes_convective_thermal_resistance_based_on_wetted_perimeter()
            - self.get_inner_tubes_cylindrical_thermal_resistance();

        // A_s 
        let total_area_shell_side = 
            self.circular_tube_bundle_heat_transfer_area_shell_side();

        // 1/(h_s) = A_s * 1/(hs_A_s)
        // take reciprocal then
        // h_s
        let h_s: HeatTransfer = 
            (shell_side_thermal_resistance_expt_data*total_area_shell_side).recip();

        let shell_side_fluid_hydraulic_diameter =
            self.get_shell_side_hydraulic_diameter();

        let expt_nusselt_number_shell_side: Ratio = 
            h_s * shell_side_fluid_hydraulic_diameter/
            self.get_shell_side_fluid_thermal_conductivity();

        expt_nusselt_number_shell_side


    }
    /// obtain shell side Nusselt number based on prevailing flowrates 
    /// and heat rate
    #[inline]
    pub fn obtain_shell_side_nusselt_number_based_on_expt_data_and_heat_rate(
        &self,
        sthe_heat_transfer_rate: Power,
        tube_inlet_temperature: ThermodynamicTemperature,
        tube_outlet_temeprature: ThermodynamicTemperature,
        shell_inlet_temperature: ThermodynamicTemperature,
        shell_outlet_temeprature: ThermodynamicTemperature,
        is_counter_current: bool) -> Ratio {

        let overall_ua: ThermalConductance = 
            Self::get_ua_based_on_heat_transfer_and_temperature_differences(
                sthe_heat_transfer_rate,
                tube_inlet_temperature, 
                tube_outlet_temeprature, 
                shell_inlet_temperature, 
                shell_outlet_temeprature, 
                is_counter_current);

        let overall_thermal_resistance = overall_ua.recip();

        // 1/(U A)_overall = 1/(h_s A_s) + 1/(h_t A_t) + inner_tubes_thermal_resistance
        //
        // 1/(h_s A_s)
        let shell_side_thermal_resistance_expt_data = 
            overall_thermal_resistance
            - self.get_inner_tubes_convective_thermal_resistance_based_on_wetted_perimeter()
            - self.get_inner_tubes_cylindrical_thermal_resistance();

        // A_s 
        let total_area_shell_side = 
            self.circular_tube_bundle_heat_transfer_area_shell_side();

        // 1/(h_s) = A_s * 1/(hs_A_s)
        // take reciprocal then
        // h_s
        let h_s: HeatTransfer = 
            (shell_side_thermal_resistance_expt_data*total_area_shell_side).recip();

        let shell_side_fluid_hydraulic_diameter =
            self.get_shell_side_hydraulic_diameter();

        let expt_nusselt_number_shell_side: Ratio = 
            h_s * shell_side_fluid_hydraulic_diameter/
            self.get_shell_side_fluid_thermal_conductivity();

        expt_nusselt_number_shell_side


    }

    /// obtain tube side nusselt number based on prevailing flowrates
    /// existing nusselt number correlation at the shell side
    /// and assuming tubes in the bundle are cylindrical
    /// outside and inside
    #[inline]
    pub fn obtain_tube_side_nusselt_number_based_on_expt_data(
        &self,
        tube_inlet_temperature: ThermodynamicTemperature,
        tube_outlet_temeprature: ThermodynamicTemperature,
        tube_mass_flowrate: MassRate,
        shell_inlet_temperature: ThermodynamicTemperature,
        shell_outlet_temeprature: ThermodynamicTemperature,
        shell_mass_flowrate: MassRate,
        is_counter_current: bool) -> Ratio {

        let overall_ua: ThermalConductance = 
            self.get_ua_based_on_mass_flowrates_and_temperature_differences(
                tube_inlet_temperature, 
                tube_outlet_temeprature, 
                tube_mass_flowrate, 
                shell_inlet_temperature, 
                shell_outlet_temeprature, 
                shell_mass_flowrate, 
                is_counter_current);

        let overall_thermal_resistance = overall_ua.recip();

        // 1/(h_t A_t)
        let tube_side_thermal_resistance_expt_data = 
            overall_thermal_resistance
            - self.get_shell_side_convective_thermal_resistance_cylindrical()
            - self.get_inner_tubes_cylindrical_thermal_resistance();

        // A_s 
        let total_area_tube_side = 
            self.circular_tube_bundle_heat_transfer_area_shell_side();

        // 1/(h_t) = A_t * 1/(ht_A_t)
        // take reciprocal then
        // h_t
        let h_t: HeatTransfer = 
            (tube_side_thermal_resistance_expt_data*total_area_tube_side).recip();

        let tube_side_fluid_hydraulic_diameter =
            self.tube_side_id;

        let expt_nusselt_number_tube_side: Ratio = 
            h_t * tube_side_fluid_hydraulic_diameter/
            self.get_shell_side_fluid_thermal_conductivity();

        expt_nusselt_number_tube_side


    }

    /// obtain parasitic heat loss nusselt number 
    /// based on experimental data
    ///
    /// 1/(U A) = R_conv_parasitic + 1/(h_ambient A_ambient) + R_insulation + R_outer_shell
    ///
    /// ambient temperature is based on existing ambient temperature of 
    /// the STHE
    #[inline]
    pub fn obtain_parasitic_nusselt_number_based_on_expt_data(
        &self,
        tube_inlet_temperature: ThermodynamicTemperature,
        tube_outlet_temeprature: ThermodynamicTemperature,
        tube_mass_flowrate: MassRate,
        shell_inlet_temperature: ThermodynamicTemperature,
        shell_outlet_temeprature: ThermodynamicTemperature,
        shell_mass_flowrate: MassRate) -> Ratio {

        let overall_thermal_resistance = 
            self.obtain_parasitic_thermal_resistance_based_on_expt_data(
                tube_inlet_temperature, 
                tube_outlet_temeprature, 
                tube_mass_flowrate, 
                shell_inlet_temperature, 
                shell_outlet_temeprature, 
                shell_mass_flowrate);

        // 1/(h_parasitic A_parasitic)
        let mut shell_side_parasitic_thermal_resistance_expt_data = 
            overall_thermal_resistance
            - self.get_convective_thermal_resistance_to_ambient()
            - self.get_outer_shell_cylindrical_thermal_resistance();

        if self.heat_exchanger_has_insulation {
            shell_side_parasitic_thermal_resistance_expt_data 
                -= self.try_get_insulation_cylindrical_thermal_resistance().unwrap();
        }

        // A_parasitic
        let total_area_shell_side_parasitic = 
            self.parasitic_heat_transfer_area_shell_side();

        // 1/(h_parasitic) = A_parasitic * 1/(h_parasitic_A_parasitic)
        // take reciprocal then
        // h_parasitic
        let h_parasitic: HeatTransfer = 
            (shell_side_parasitic_thermal_resistance_expt_data *
             total_area_shell_side_parasitic
            ).recip();

        let shell_side_fluid_hydraulic_diameter =
            self.get_shell_side_hydraulic_diameter();

        let expt_nusselt_number_parasitic_shell_side: Ratio = 
            h_parasitic * shell_side_fluid_hydraulic_diameter/
            self.get_shell_side_fluid_thermal_conductivity();

        expt_nusselt_number_parasitic_shell_side

    }

    /// gets thermal resistance for parasitic heat losses 
    /// based on expt
    #[inline]
    pub fn obtain_parasitic_thermal_resistance_based_on_expt_data(
        &self,
        tube_inlet_temperature: ThermodynamicTemperature,
        tube_outlet_temeprature: ThermodynamicTemperature,
        tube_mass_flowrate: MassRate,
        shell_inlet_temperature: ThermodynamicTemperature,
        shell_outlet_temeprature: ThermodynamicTemperature,
        shell_mass_flowrate: MassRate) -> ThermalResistance {

        let parasitic_heat_transfer_rate = 
            self.obtain_parasitic_heat_loss_rate_based_on_expt_data(
                tube_inlet_temperature, 
                tube_outlet_temeprature, 
                tube_mass_flowrate, 
                shell_inlet_temperature, 
                shell_outlet_temeprature, 
                shell_mass_flowrate);
        
        // then get UA and thermal resistance based on LMTD
        let is_counter_current = true;
        let ambient_inlet_temperature = self.ambient_temperature;
        let ambient_outlet_temeprature = self.ambient_temperature;

        let overall_ua: ThermalConductance = 
            Self::get_ua_based_on_heat_transfer_and_temperature_differences(
                parasitic_heat_transfer_rate, 
                ambient_inlet_temperature, 
                ambient_outlet_temeprature, 
                shell_inlet_temperature, 
                shell_outlet_temeprature, 
                is_counter_current);

        let overall_thermal_resistance = overall_ua.recip();

        overall_thermal_resistance

    }

    /// obtain parasitic heat losses based on expt data 
    #[inline]
    pub fn obtain_parasitic_heat_loss_rate_based_on_expt_data(
        &self,
        tube_inlet_temperature: ThermodynamicTemperature,
        tube_outlet_temeprature: ThermodynamicTemperature,
        tube_mass_flowrate: MassRate,
        shell_inlet_temperature: ThermodynamicTemperature,
        shell_outlet_temeprature: ThermodynamicTemperature,
        shell_mass_flowrate: MassRate) -> Power {

        let shell_side_heat_rate: Power = 
            self.get_shell_side_heat_rate_based_on_mass_flowrate(
                shell_inlet_temperature, 
                shell_outlet_temeprature, 
                shell_mass_flowrate);

        let tube_side_heat_rate: Power = 
            self.get_tube_side_heat_rate_based_on_mass_flowrate(
                tube_inlet_temperature, 
                tube_outlet_temeprature, 
                tube_mass_flowrate);

        // parasitic heat losses are that of the tube side minus shell 
        // side, anything outside is heat supplied
        //
        // but I'll just do that anyway
        let parasitic_heat_transfer_rate: Power = 
            (shell_side_heat_rate - tube_side_heat_rate).abs();

        parasitic_heat_transfer_rate
    }



}
