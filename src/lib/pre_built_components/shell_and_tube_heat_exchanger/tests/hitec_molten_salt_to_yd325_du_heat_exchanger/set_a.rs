
/// shell and tube heat exchanger test set A,
///
/// This is where 
/// salt volumetric flowrate is 12.63 m3/s 
/// oil volumetic flowrate is 15.635 m3/s 
/// temperatures of oil and salt are varied
/// from 74.49  - 90.41 C  (YD325 oil)
/// and 214.93 - 236.91 C (HITEC salt)
/// respectively
/// this is a test for shell and tube heat exchanger in Du's paper 
/// HITEC flows through the shell side while heat transfer oil 
/// (YD 325) flows through the tube side
///
/// I'm assuming an adiabatic bc to the outside
/// and switching off the insulation boolean
///
#[test]
//#[ignore = "debugging"]
pub fn du_test_shell_and_tube_heat_exchanger_set_a(){

    use std::f64::consts::PI;

    use std::thread;

    use uom::si::time::second;
    use uom::si::thermodynamic_temperature::kelvin;

    use crate::heat_transfer_correlations::
        heat_transfer_interactions::heat_transfer_interaction_enums
        ::HeatTransferInteractionType;
    use crate::boundary_conditions::BCType;

    use crate::fluid_mechanics_correlations::
        churchill_friction_factor::darcy;

    use approx::assert_relative_eq;
    use uom::si::angle::degree;
    //use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::length::meter;
    use uom::si::pressure::atmosphere;
    use uom::si::ratio::ratio;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::ConstZero;
    use uom::si::temperature_interval;
    use uom::si::thermal_conductivity::watt_per_meter_kelvin;
    //use uom::si::area::square_meter;
    use uom::si::volume_rate::cubic_meter_per_hour;
    use uom::si::quantities::VolumeRate;

    use crate::heat_transfer_correlations::nusselt_number_correlations::input_structs::GnielinskiData;

    use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;

    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
    use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
    use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
    use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::pre_built_components::shell_and_tube_heat_exchanger::SimpleShellAndTubeHeatExchanger;
    use crate::heat_transfer_correlations::nusselt_number_correlations::pipe_correlations::custom_gnielinski_turbulent_nusselt_correlation;

    use uom::si::f64::*;
    let number_of_tubes = 19_u32;
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);

    // from Du's heat exchanger type, except we use one inner tube
    let tube_side_od = Length::new::<meter>(0.014);
    let tube_side_id = Length::new::<meter>(0.01);
    let shell_side_od = Length::new::<meter>(0.108);
    let shell_side_id = Length::new::<meter>(0.1);
    let pipe_length = Length::new::<meter>(1.95);

    let tube_side_flow_area: Area 
        = PI * 0.25 * tube_side_id * tube_side_id;

    let shell_side_flow_area: Area 
        = PI * 0.25 * shell_side_id * shell_side_id 
        - number_of_tubes as f64 *
        PI * 0.25 * tube_side_od * tube_side_od;

    let shell_side_fluid_hydraulic_diameter: Length = 
        (shell_side_id * shell_side_id - number_of_tubes as f64 *
         tube_side_od * tube_side_od)/
        (shell_side_id + number_of_tubes as f64 * tube_side_od);

    let hitec: LiquidMaterial = LiquidMaterial::HITEC;
    let yd325: LiquidMaterial = LiquidMaterial::YD325;
    let steel: SolidMaterial = SolidMaterial::SteelSS304L;

    let form_loss = Ratio::new::<ratio>(0.0);

    let inlet_temp_salt = 
        ThermodynamicTemperature::new::<degree_celsius>(214.93);
    let inlet_temp_oil = 
        ThermodynamicTemperature::new::<degree_celsius>(74.49);
    // ambient temperature is 25C 
    let ambient_temperature = ThermodynamicTemperature::
        new::<degree_celsius>(25.0);

    // adiabatic, heat transfer for ambient is zero 
    let heat_transfer_to_ambient = HeatTransfer::ZERO;

    // perfomed mesh refinement up to 25 inner nodes, 
    // 12 is sufficient
    let number_of_inner_nodes = 12;

    let incline_angle = Angle::new::<degree>(0.0);

    // inner fluid_array
    // the nusselt correlation here is a standard pipe correlation 
    // but I don't use that
    let tube_side_fluid_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            pipe_length,
            tube_side_id,
            tube_side_flow_area,
            inlet_temp_oil,
            fluid_pressure,
            steel,
            yd325,
            form_loss,
            number_of_inner_nodes,
            incline_angle
        );

    // shell side fluid_array
    // the nusselt correlation here is a standard pipe correlation 
    // but I don't use that
    //
    // the reason is because for the shell side, the heat transfer 
    // to the outer shell and heat transfer to inner tubes will 
    // be different. 
    //
    // The fluid array, unfortunately, only has one nusselt correlation 
    // by default.
    let shell_side_fluid_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            pipe_length,
            shell_side_fluid_hydraulic_diameter,
            shell_side_flow_area,
            inlet_temp_salt,
            fluid_pressure,
            steel,
            hitec,
            form_loss,
            number_of_inner_nodes,
            incline_angle
        );

    let outer_shell: SolidColumn 
        = SolidColumn::new_cylindrical_shell(
            pipe_length, 
            shell_side_id, 
            shell_side_od, 
            inlet_temp_salt, 
            solid_pressure, 
            steel, 
            number_of_inner_nodes
        );

    let inner_shell: SolidColumn 
        = SolidColumn::new_cylindrical_shell(
            pipe_length, 
            tube_side_id, 
            tube_side_od, 
            inlet_temp_salt, 
            solid_pressure, 
            steel, 
            number_of_inner_nodes
        );

    // dummy insulation array, will not be used,
    // just clone the inner shell 
    let dummy_insulation_array 
        = inner_shell.clone();

    // loss correlations, use pipe by default 
    // but none are used in calculations depending on 
    // nusselt correlations
    let shell_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            pipe_length, 
            SolidMaterial::SteelSS304L.surface_roughness().unwrap(), 
            shell_side_fluid_hydraulic_diameter, 
            form_loss
        );


    // for tube loss correlations, we need to use the 
    // darcy_friction_factor
    let tube_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            pipe_length, 
            SolidMaterial::SteelSS304L.surface_roughness().unwrap(), 
            tube_side_id, 
            form_loss
        );

    // nusselt correlations, 4.36 by default

    let shell_side_length_to_diameter: Ratio = 
        pipe_length/shell_side_fluid_hydraulic_diameter;

    // note that prandtl, reynolds and darcy friction factor for shell 
    // side are all arbitrary, will get overwritten later
    let dummy_ratio = Ratio::new::<ratio>(0.1);
    let shell_side_gnielinski_data: GnielinskiData = 
        GnielinskiData {
            reynolds: dummy_ratio,
            prandtl_bulk: dummy_ratio,
            prandtl_wall: dummy_ratio,
            darcy_friction_factor: dummy_ratio,
            length_to_diameter: shell_side_length_to_diameter,
        };

    let c: Ratio = Ratio::new::<ratio>(0.04318);
    let m: f64 = 0.7797;
    let shell_side_nusselt_correlation_to_tubes = 
        NusseltCorrelation::CustomGnielinskiGenericPrandtlBulk(
            shell_side_gnielinski_data, c, m);

    let tube_side_length_to_diameter: Ratio = 
        pipe_length/tube_side_id;

    // note that prandtl, reynolds and darcy friction factor for shell 
    // side are all arbitrary, will get overwritten later
    let tube_side_gnielinski_data: GnielinskiData = 
        GnielinskiData {
            reynolds: dummy_ratio,
            prandtl_bulk: dummy_ratio,
            prandtl_wall: dummy_ratio,
            darcy_friction_factor: dummy_ratio,
            length_to_diameter: tube_side_length_to_diameter,
        };

    let tube_side_nusselt_correlation = 
        NusseltCorrelation::PipeGnielinskiGeneric(tube_side_gnielinski_data);

    let shell_side_nusselt_correlation_to_outer_shell = 
        NusseltCorrelation::FixedNusselt(Ratio::ZERO);

    // we are not going to use this anyway
    let dummy_insulation_thickness =
        Length::new::<meter>(1.0);


    let sthe_one_shell_one_tube 
        = SimpleShellAndTubeHeatExchanger{ 
            inner_nodes: number_of_inner_nodes, 
            inner_pipe_shell_array_for_single_tube: inner_shell.into(), 
            tube_side_fluid_array_for_single_tube: tube_side_fluid_array.into(), 
            shell_side_fluid_array: shell_side_fluid_array.into(), 
            outer_shell: outer_shell.into(), 
            ambient_temperature: ambient_temperature.into(), 
            heat_transfer_to_ambient, 
            insulation_array: dummy_insulation_array.into(), 
            heat_exchanger_has_insulation: false, 
            tube_side_od, 
            tube_side_id, 
            tube_side_flow_area, 
            tube_side_custom_component_loss_correlation: tube_loss_correlations, 
            shell_side_custom_component_loss_correlation: shell_loss_correlations, 
            number_of_tubes, 
            shell_side_id, 
            shell_side_od, 
            shell_side_flow_area, 
            shell_side_nusselt_correlation_to_tubes: shell_side_nusselt_correlation_to_tubes.clone(), 
            shell_side_nusselt_correlation_parasitic: shell_side_nusselt_correlation_to_outer_shell, 
            tube_side_nusselt_correlation: tube_side_nusselt_correlation.clone(), 
            insulation_thickness: dummy_insulation_thickness,
        };

    // from data, is set A1 
    // salt flow rate is 12.63 m3/h 
    // oil flow rate is 15.635 m3/h
    // T in salt is 214.93
    // T in oil is 74.49

    let vol_flowrate_salt = 
        VolumeRate::new::<cubic_meter_per_hour>(12.63);
    let vol_flowrate_oil = 
        VolumeRate::new::<cubic_meter_per_hour>(15.635);


    let inlet_rho_salt = 
        LiquidMaterial::HITEC.try_get_density(inlet_temp_salt).unwrap();
    let inlet_rho_oil = 
        LiquidMaterial::YD325.try_get_density(inlet_temp_oil).unwrap();

    //
    // but the direction differs in a counter current setup, 
    // so I'll have to add a negative sign
    // 
    // once again, 
    // HITEC flows through the shell side while heat transfer oil 
    // (YD 325) flows through the tube side
    let m_t: MassRate = inlet_rho_oil * vol_flowrate_oil;
    let m_s: MassRate = -inlet_rho_salt * vol_flowrate_salt;

    let tube_inlet_temperature = 
        inlet_temp_oil;
    let shell_inlet_temperature = 
        inlet_temp_salt;

    // code a function to obtain outlet temperature 

    fn obtain_outlet_steady_state_temp(
        sthe: &mut SimpleShellAndTubeHeatExchanger,
        tube_inlet_temperature: ThermodynamicTemperature,
        shell_inlet_temperature: ThermodynamicTemperature,
        tube_mass_flowrate: MassRate,
        shell_mass_flowrate: MassRate) -> ThermodynamicTemperature {

        // first, connect shell and tube fluid arrays 
        // to their boundary conditions axially (advection bcs)

        let mut tube_inlet_bc: HeatTransferEntity = 
            BCType::new_const_temperature(tube_inlet_temperature).into();
        let mut shell_inlet_bc: HeatTransferEntity = 
            BCType::new_const_temperature(shell_inlet_temperature).into();

        let mut outlet_bc: HeatTransferEntity = 
            BCType::new_adiabatic_bc().into();

        let max_time = Time::new::<second>(1500_f64);

        let number_of_nodes = sthe.inner_nodes + 2;
        let timestep = Time::new::<second>(0.05 * 10.0 / number_of_nodes as f64);
        let mut simulation_time = Time::ZERO;

        // for simplicity, I'm going to use an average HITEC 
        // density at 250C (or just the shell and tube inlet temp)
        let average_temp_kelvin_hitec = shell_inlet_temperature.value;
        let average_temp_hitec = ThermodynamicTemperature::new::<kelvin>(
            average_temp_kelvin_hitec);
        let average_hitec_density = 
            LiquidMaterial::HITEC.try_get_density(average_temp_hitec).unwrap();

        let average_temp_kelvin_oil = tube_inlet_temperature.value;
        let average_temp_oil = ThermodynamicTemperature::new::<kelvin>(
            average_temp_kelvin_oil);
        let average_oil_density = 
            LiquidMaterial::YD325.try_get_density(average_temp_oil).unwrap();

        let tube_inlet_density = average_oil_density;
        let tube_back_cv_density = average_oil_density;
        let tube_front_cv_density = average_oil_density;
        let shell_inlet_density = average_hitec_density;
        let shell_back_cv_density = average_hitec_density;
        let shell_front_cv_density = average_hitec_density;



        while max_time > simulation_time {
            // probably want to make this bit a little more user friendly
            let tube_inlet_interaction: HeatTransferInteractionType = 
                HeatTransferInteractionType::new_advection_interaction(
                    tube_mass_flowrate,
                    tube_inlet_density,
                    tube_back_cv_density);

            let tube_outlet_interaction = 
                HeatTransferInteractionType::new_advection_interaction(
                    tube_mass_flowrate,
                    tube_front_cv_density,
                    tube_front_cv_density,
                );
            let shell_inlet_interaction: HeatTransferInteractionType = 
                HeatTransferInteractionType::new_advection_interaction(
                    shell_mass_flowrate,
                    shell_inlet_density,
                    shell_back_cv_density);

            let shell_outlet_interaction = 
                HeatTransferInteractionType::new_advection_interaction(
                    shell_mass_flowrate,
                    shell_front_cv_density,
                    shell_front_cv_density,
                );
            // remember this is counter flow
            // shell flows in opposite direction by default
            //
            // tube side goes back -> front
            sthe.tube_side_fluid_array_for_single_tube. 
                link_to_back(&mut tube_inlet_bc, 
                    tube_inlet_interaction).unwrap();

            sthe.tube_side_fluid_array_for_single_tube. 
                link_to_front(&mut outlet_bc, 
                    tube_outlet_interaction).unwrap();

            // shell side goes front -> back
            sthe.shell_side_fluid_array.
                link_to_front(&mut shell_inlet_bc, 
                    shell_inlet_interaction).unwrap();

            sthe.shell_side_fluid_array. 
                link_to_back(&mut outlet_bc, 
                    shell_outlet_interaction).unwrap();

            // prandtl wall correction setting can be problematic as the 
            // wall is either too cold or too hot
            // probably need result matching to get the wall prandtl number 
            // even partially corrected
            let prandtl_wall_correction_setting = true;

            // connect the arrays with thermal conductances 
            // interally

            sthe.lateral_and_miscellaneous_connections(
                prandtl_wall_correction_setting, 
                tube_mass_flowrate, 
                shell_mass_flowrate).unwrap();

            // advance timestep
            sthe.advance_timestep(timestep).unwrap();



            simulation_time += timestep;
        }

        // collect information
        let temperature_vec_tube_side = 
            sthe.tube_side_fluid_array_for_single_tube.
            get_temperature_vector().unwrap();

        //dbg!(&temperature_vec_tube_side);

        //let temperature_vec_shell_side = 
        //    sthe.shell_side_fluid_array. 
        //    get_temperature_vector().unwrap();

        //dbg!(&temperature_vec_shell_side);

        // get the last item
        //let tube_inlet_temperature_steady_state: 
        //    ThermodynamicTemperature = 
        //    *temperature_vec_tube_side.first().unwrap();

        let tube_outlet_temperature: ThermodynamicTemperature = 
            *temperature_vec_tube_side.last().unwrap();





        tube_outlet_temperature

    }

    // now I want to speed up this process using parallel threads

    // this will return Re_shell 
    // Pr_shell_fluid 
    // Pr_shell_wall 
    // Nu_shell
    let test_thread = move |mut sthe: SimpleShellAndTubeHeatExchanger,
    tube_inlet_temperature: ThermodynamicTemperature,
    shell_inlet_temperature: ThermodynamicTemperature,
    m_t: MassRate,
    m_s: MassRate,| -> (Ratio,Ratio, Ratio, Ratio) {


        let tube_side_outlet_temperature: ThermodynamicTemperature 
            = obtain_outlet_steady_state_temp(
                &mut sthe, 
                tube_inlet_temperature, 
                shell_inlet_temperature, 
                m_t, 
                m_s);
        let correct_for_prandtl_wall_temperatures_u_and_ua = true;

        let _ua: ThermalConductance 
            = sthe.overall_heat_transfer_coeff_u_shell_side(
                correct_for_prandtl_wall_temperatures_u_and_ua).unwrap() * 
            sthe.circular_tube_bundle_heat_transfer_area_shell_side();

        let u_calc_from_postprocess: HeatTransfer = 
            sthe.overall_heat_transfer_coeff_u_shell_side(
                correct_for_prandtl_wall_temperatures_u_and_ua).unwrap();


        // shell side outlet temperature and inlet cv temperature
        let (_shell_side_inlet_cv_temperature, 
            shell_side_outlet_temperature): 
            (ThermodynamicTemperature, ThermodynamicTemperature) = {
                let temperature_vec_shell_side = 
                    sthe.shell_side_fluid_array. 
                    get_temperature_vector().unwrap();

                let shell_side_outlet_temperature = 
                    *temperature_vec_shell_side.first().unwrap();

                let shell_side_inlet_cv_temperature = 
                    *temperature_vec_shell_side.last().unwrap();

                (shell_side_inlet_cv_temperature,
                 shell_side_outlet_temperature)

            };

        // tube side inlet cv temperature
        let _tube_side_inlet_cv_temperature: ThermodynamicTemperature = {

            let temperature_vec_tube_side = 
                sthe.tube_side_fluid_array_for_single_tube. 
                get_temperature_vector().unwrap();

            let tube_side_inlet_cv_temperature = 
                *temperature_vec_tube_side.first().unwrap();

            tube_side_inlet_cv_temperature

        };

        // now I need to recreate the nusselt numbers and reynolds 
        // numbers, 
        //
        // First, find T_avg on shell and tube side to get cp
        // note: tube side is oil, shell side is HITEC salt

        let average_shell_side_temp_kelvin: f64 = 
            0.5 * (shell_inlet_temperature.get::<kelvin>() 
                + shell_side_outlet_temperature.get::<kelvin>()
            );

        let average_shell_side_temp: ThermodynamicTemperature = 
            ThermodynamicTemperature::new::<kelvin>(
                average_shell_side_temp_kelvin);

        let average_cp_shell_side = 
            LiquidMaterial::HITEC.try_get_cp(
                average_shell_side_temp).unwrap();

        let shell_side_delta_t: TemperatureInterval = 
            TemperatureInterval::new::<temperature_interval::kelvin>(
                shell_side_outlet_temperature.get::<kelvin>() - 
                shell_inlet_temperature.get::<kelvin>()
            );

        let q_shell: Power = 
            m_s * average_cp_shell_side * shell_side_delta_t;


        let average_tube_side_temp_kelvin: f64 = 
            0.5 * (tube_inlet_temperature.get::<kelvin>() 
                + tube_side_outlet_temperature.get::<kelvin>()
            );

        let average_tube_side_temp: ThermodynamicTemperature = 
            ThermodynamicTemperature::new::<kelvin>(
                average_tube_side_temp_kelvin);

        let average_cp_tube_side = 
            LiquidMaterial::YD325.try_get_cp(
                average_tube_side_temp).unwrap();

        let tube_side_delta_t: TemperatureInterval = 
            TemperatureInterval::new::<temperature_interval::kelvin>(
                tube_side_outlet_temperature.get::<kelvin>() - 
                tube_inlet_temperature.get::<kelvin>()
            );

        let q_tube: Power = 
            m_t * average_cp_tube_side * tube_side_delta_t;

        let q_avg: Power = 0.5 * (q_shell + q_tube);

        let heat_balance_error: Ratio = 
            (q_tube - q_shell).abs()/q_avg;

        let max_heat_balance_error = Ratio::new::<ratio>(0.05);

        let heat_balance_valid: bool = 
            heat_balance_error < max_heat_balance_error;

        // checks that heat balance is valid, needs to be within 5% as per 
        // experimental setup
        assert!(heat_balance_valid);

        // now we go onto calculating U via the log mean temp diff 
        // U = Q_av/(A_shell LMTD)
        let delta_t_max: TemperatureInterval = 
            TemperatureInterval::new::<temperature_interval::kelvin>(
                shell_side_outlet_temperature.get::<kelvin>() - 
                tube_inlet_temperature.get::<kelvin>()
            );

        let delta_t_min: TemperatureInterval = 
            TemperatureInterval::new::<temperature_interval::kelvin>(
                shell_inlet_temperature.get::<kelvin>() - 
                tube_side_outlet_temperature.get::<kelvin>()
            );

        let delta_t_max_to_delta_t_min: Ratio = 
            delta_t_max/delta_t_min;

        let lmtd: TemperatureInterval =  
            (delta_t_max - delta_t_min)/(
                delta_t_max_to_delta_t_min.get::<ratio>().ln()
            );

        let shell_side_area: Area = 
            number_of_tubes as f64 * PI * tube_side_od * pipe_length;

        //dbg!(&
        //    (
        //        q_avg,
        //        q_tube,
        //        q_shell,
        //        lmtd,
        //        shell_side_area
        //    ));

        let u_calc_using_lmtd: HeatTransfer = q_avg / shell_side_area / lmtd ;

        // next, I want the nusselt number of the tube side, 
        //
        // However, there needs to be a correction factor of Pr/Pr_wall 
        //
        // Wall temperature was in turn estimated using h_s and h_t 
        // in Du's paper
        //
        // We could iteratively calculate h_s and h_t using some 
        // algorithm to find Pr_wall. 
        //
        // I find it ridiculous however, to spend so much effort
        // on something so simple. 
        //
        // There are two methods we can use
        //
        // (1) don't calculate Pr_wall correction 
        // (2) use the solid array control volume temperature to calculate 
        // Pr_wall.
        //
        // I'm just going to use the latter

        let wall_side_bulk_temp: ThermodynamicTemperature = 
            sthe.inner_pipe_shell_array_for_single_tube.try_get_bulk_temperature()
            .unwrap();


        let nusselt_tube_side: Ratio;

        // get reynolds and prandtl

        let tube_side_dynamic_viscosity: DynamicViscosity = 
            LiquidMaterial::YD325.try_get_dynamic_viscosity(
                average_tube_side_temp).unwrap();

        let tube_side_prandtl: Ratio = 
            LiquidMaterial::YD325.try_get_prandtl_liquid(
                average_tube_side_temp, fluid_pressure).unwrap();

        let tube_wall_side_prandtl: Ratio = 
            LiquidMaterial::YD325.try_get_prandtl_liquid(
                wall_side_bulk_temp, fluid_pressure).unwrap();

        let reynolds_tube_side: Ratio = 
            m_t * tube_side_id   
            / tube_side_flow_area
            / tube_side_dynamic_viscosity
            / number_of_tubes as f64;

        let darcy_friction_factor: f64 = 
            darcy(
                reynolds_tube_side.get::<ratio>(), 
                (SolidMaterial::SteelSS304L.surface_roughness().unwrap()/
                 tube_side_id).get::<ratio>()
                ).unwrap();

        let darcy_friction_factor: Ratio = 
            Ratio::new::<ratio>(darcy_friction_factor);


        let gnielinski_data = match tube_side_nusselt_correlation {
            NusseltCorrelation::PipeGnielinskiGeneric(mut data) => {
                data.darcy_friction_factor = 
                    darcy_friction_factor.into();

                data
            },
            _ => todo!()
        };

        let tube_side_nusselt_correlation = 
            NusseltCorrelation::PipeGnielinskiGeneric(
                gnielinski_data);


        nusselt_tube_side = tube_side_nusselt_correlation
            .estimate_based_on_prandtl_darcy_and_reynolds_wall_correction
            (
                tube_side_prandtl,
                tube_wall_side_prandtl,
                darcy_friction_factor,
                reynolds_tube_side
            ).unwrap();

        // now for the tube side heat transfer coeff 
        //
        // Nu = hD/k 
        // k = conductivity (denoted lambda)

        let lambda_tube: ThermalConductivity =  
            LiquidMaterial::YD325.try_get_thermal_conductivity
            (average_tube_side_temp).unwrap();

        let h_t: HeatTransfer = 
            nusselt_tube_side * lambda_tube / tube_side_id;

        //// debug tube side parameters
        //dbg!(&(
        //        reynolds_tube_side,
        //        tube_side_prandtl,
        //        tube_wall_side_prandtl,
        //        nusselt_tube_side
        //)
        //);

        // now to calculate for h_s 
        //
        // 1/u = 1/h_t d_o/d_i + d_o/(2 lambda_w) ln (d_o/d_i) 
        // + 1/h_s

        // 1/u
        let one_over_u = u_calc_using_lmtd.recip();

        // 1/h_t d_o/d_i
        let reciprocal_tube_side_fluid_term = h_t.recip() * 
            tube_side_od/tube_side_id;


        // d_o/(2 lambda_w) ln (d_o/d_i)
        //
        // lambda_w, based on 16.3 W/(m K) in Du's paper
        let _lambda_wall_du_paper: ThermalConductivity = 
            ThermalConductivity::new::<watt_per_meter_kelvin>(
                16.3);
        let lambda_wall: ThermalConductivity = 
            SolidMaterial::SteelSS304L 
            .try_get_thermal_conductivity(wall_side_bulk_temp).unwrap();

        let reciprocal_tube_side_solid_term = 
            tube_side_od/(2.0 as f64 * lambda_wall) 
            * (tube_side_od/tube_side_id).get::<ratio>().ln();
        
        let _one_over_u_postprocess = u_calc_from_postprocess.recip();


        // 1/h_s = 1/u - 1/h_t d_o/d_i - d_o/(2 lambda_w) ln (d_o/d_i) 
        let one_over_hs = 
            one_over_u - reciprocal_tube_side_fluid_term - 
            reciprocal_tube_side_solid_term;


        // shell side heat trf coeff
        let h_s: HeatTransfer = one_over_hs.recip();

        // used to debug tube side nusselt
        //dbg!(&(reciprocal_tube_side_solid_term,
        //        h_t,
        //        h_s,
        //        u_calc_using_lmtd,
        //        one_over_u,
        //        reciprocal_tube_side_fluid_term,
        //        h_t,
        //        lambda_wall,
        //        wall_side_bulk_temp));
        // now for shell side nusselt
        // Nu_s = h_s D_e/k_s

        let lambda_shell_fluid: ThermalConductivity = 
            LiquidMaterial::HITEC.try_get_thermal_conductivity(
                average_shell_side_temp).unwrap();

        let nusselt_number_shell_calculated: Ratio = 
            h_s * shell_side_fluid_hydraulic_diameter / 
            lambda_shell_fluid;

        // then shell side reynolds number 

        let mu_shell_side: DynamicViscosity = 
            LiquidMaterial::HITEC.try_get_dynamic_viscosity(
                average_shell_side_temp).unwrap();

        let reynolds_shell_side: Ratio = 
            m_s.abs() * shell_side_fluid_hydraulic_diameter / 
            shell_side_flow_area / 
            mu_shell_side;

        // for debugging, cos Nusselt is underestimated by about 1.5x 
        // consistently

        let shell_side_fluid_bulk_temp: ThermodynamicTemperature = 
            sthe.shell_side_fluid_array.try_get_bulk_temperature()
            .unwrap();

        let shell_side_fluid_bulk_prandtl = 
            LiquidMaterial::HITEC.try_get_prandtl_liquid(
                shell_side_fluid_bulk_temp, fluid_pressure).unwrap();

        let mut est_wall_temp_hitec = wall_side_bulk_temp;
        if wall_side_bulk_temp < LiquidMaterial::HITEC.min_temperature() {
            est_wall_temp_hitec = LiquidMaterial::HITEC.min_temperature();
        }


        let shell_side_fluid_wall_prandtl = 
            LiquidMaterial::HITEC.try_get_prandtl_liquid(
                est_wall_temp_hitec, fluid_pressure).unwrap();

        let shell_side_fluid_film_prandtl_estimate = 
            shell_side_fluid_bulk_prandtl;
        // Nu = C (Re^m - 280.0) Pr_bulk^0.4 ( 1.0 + (D_e/l)^(2/3) ) ( Pr_f / Pr_w )^0.25
        // For Du's Heat exchanger, 
        // C = 0.04318,
        // m = 0.7797
        let nusselt_number_direct_from_correlation = 
            custom_gnielinski_turbulent_nusselt_correlation(
                Ratio::new::<ratio>(0.04318),
                0.7797,
                shell_side_fluid_film_prandtl_estimate,
                shell_side_fluid_bulk_prandtl,
                shell_side_fluid_wall_prandtl,
                reynolds_shell_side,
                shell_side_length_to_diameter
            );

        // for debugging, get u from conductances 

        let _u_from_conductance = 
            sthe.overall_htc_based_on_conductance(
                correct_for_prandtl_wall_temperatures_u_and_ua,
                m_t.abs(),
                m_s.abs()
                );




        dbg!(&(
                shell_inlet_temperature.get::<degree_celsius>(),
                tube_inlet_temperature.get::<degree_celsius>(),
                //tube_side_inlet_cv_temperature.get::<degree_celsius>(),
                //shell_side_inlet_cv_temperature.get::<degree_celsius>(),
                shell_side_outlet_temperature.get::<degree_celsius>(),
                tube_side_outlet_temperature.get::<degree_celsius>(),
                //m_t,
                //m_s,
                // ua, 
                u_calc_using_lmtd,
                //u_from_conductance,
                reynolds_shell_side,
                shell_side_fluid_bulk_prandtl,
                shell_side_fluid_wall_prandtl,
                nusselt_number_shell_calculated,
                nusselt_number_direct_from_correlation,
                u_calc_from_postprocess,
        ));

        // check whether correlation input into object is same as 
        // direct correlation. It is checked to be ok
        //dbg!(
        //    &(
        //        nusselt_number_direct_from_correlation,
        //        nusselt_from_correlation_object,
        //        shell_side_fluid_bulk_prandtl,
        //        shell_side_fluid_wall_prandtl
        //    )
        //);

        return (reynolds_shell_side,
            shell_side_fluid_bulk_prandtl,
            shell_side_fluid_wall_prandtl,
            nusselt_number_shell_calculated);

    };

    // first test 

    let clone_for_test_one: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();

    let (reynolds_num_a1,
        bulk_prandtl_a1,
        wall_prandtl_a1,
        nusselt_number_a1) = test_thread(clone_for_test_one,
        tube_inlet_temperature,
        shell_inlet_temperature,
        m_t,
        m_s);

    dbg!(&(reynolds_num_a1,
            bulk_prandtl_a1,
            wall_prandtl_a1,
            nusselt_number_a1));
    assert_relative_eq!(
        reynolds_num_a1.get::<ratio>(),
        3496.0,
        max_relative = 0.01,
        );

    assert_relative_eq!(
        bulk_prandtl_a1.get::<ratio>(),
        24.16,
        max_relative = 0.01,
        );

    assert_relative_eq!(
        wall_prandtl_a1.get::<ratio>(),
        37.89,
        max_relative = 0.01,
        );

    assert_relative_eq!(
        nusselt_number_a1.get::<ratio>(),
        42.47,
        max_relative = 0.01,
        );

    let one_test_only: bool = false;

    // second and subseuent 

    if !one_test_only {
        let clone_for_test_two: SimpleShellAndTubeHeatExchanger = 
            sthe_one_shell_one_tube.clone();
        let test_two = thread::spawn(
            move || {
                let inlet_temp_salt = 
                    ThermodynamicTemperature::new::<degree_celsius>(224.93);
                let inlet_temp_oil = 
                    ThermodynamicTemperature::new::<degree_celsius>(74.49);
                let vol_flowrate_salt = 
                    VolumeRate::new::<cubic_meter_per_hour>(12.63);
                let vol_flowrate_oil = 
                    VolumeRate::new::<cubic_meter_per_hour>(15.635);

                // intermediate calcs

                let inlet_rho_salt = 
                    LiquidMaterial::HITEC.try_get_density(inlet_temp_salt).unwrap();
                let inlet_rho_oil = 
                    LiquidMaterial::YD325.try_get_density(inlet_temp_oil).unwrap();

                let m_t: MassRate = inlet_rho_oil * vol_flowrate_oil;
                let m_s: MassRate = -inlet_rho_salt * vol_flowrate_salt;

                let tube_inlet_temperature = inlet_temp_oil;
                let shell_inlet_temperature = inlet_temp_salt;

                let (reynolds_num_a2,
                    bulk_prandtl_a2,
                    wall_prandtl_a2,
                    nusselt_number_a2) = test_thread(clone_for_test_two,
                    tube_inlet_temperature,
                    shell_inlet_temperature,
                    m_t,
                    m_s);

                dbg!(&(reynolds_num_a2,
                        bulk_prandtl_a2,
                        wall_prandtl_a2,
                        nusselt_number_a2));
                // assert that the Reynolds, Prandtl bulk and 
                // Prandtl wall are equal to some amount

                assert_relative_eq!(
                    reynolds_num_a2.get::<ratio>(),
                    3808.0,
                    max_relative = 0.01,
                );

                assert_relative_eq!(
                    bulk_prandtl_a2.get::<ratio>(),
                    22.19,
                    max_relative = 0.01,
                );

                assert_relative_eq!(
                    wall_prandtl_a2.get::<ratio>(),
                    37.89,
                    max_relative = 0.01,
                );

                assert_relative_eq!(
                    nusselt_number_a2.get::<ratio>(),
                    45.52,
                    max_relative = 0.01,
                );
            }
        );


        let clone_for_test_three: SimpleShellAndTubeHeatExchanger = 
            sthe_one_shell_one_tube.clone();
        let test_three = thread::spawn(
            move || {
                let inlet_temp_salt = 
                    ThermodynamicTemperature::new::<degree_celsius>(236.91);
                let inlet_temp_oil = 
                    ThermodynamicTemperature::new::<degree_celsius>(74.49);
                let vol_flowrate_salt = 
                    VolumeRate::new::<cubic_meter_per_hour>(12.63);
                let vol_flowrate_oil = 
                    VolumeRate::new::<cubic_meter_per_hour>(15.635);

                // intermediate calcs

                let inlet_rho_salt = 
                    LiquidMaterial::HITEC.try_get_density(inlet_temp_salt).unwrap();
                let inlet_rho_oil = 
                    LiquidMaterial::YD325.try_get_density(inlet_temp_oil).unwrap();

                let m_t: MassRate = inlet_rho_oil * vol_flowrate_oil;
                let m_s: MassRate = -inlet_rho_salt * vol_flowrate_salt;

                let tube_inlet_temperature = inlet_temp_oil;
                let shell_inlet_temperature = inlet_temp_salt;

                let (reynolds_num_a3,
                    bulk_prandtl_a3,
                    wall_prandtl_a3,
                    nusselt_number_a3) = test_thread(
                    clone_for_test_three,
                    tube_inlet_temperature,
                    shell_inlet_temperature,
                    m_t,
                    m_s);

                dbg!(&(reynolds_num_a3,
                        bulk_prandtl_a3,
                        wall_prandtl_a3,
                        nusselt_number_a3));
                // assert that the Reynolds, Prandtl bulk and 
                // Prandtl wall are equal to some amount

                assert_relative_eq!(
                    reynolds_num_a3.get::<ratio>(),
                    4203.0,
                    max_relative = 0.01,
                );

                assert_relative_eq!(
                    bulk_prandtl_a3.get::<ratio>(),
                    20.12,
                    max_relative = 0.01,
                );

                assert_relative_eq!(
                    wall_prandtl_a3.get::<ratio>(),
                    37.89,
                    max_relative = 0.01,
                );

                assert_relative_eq!(
                    nusselt_number_a3.get::<ratio>(),
                    48.98,
                    max_relative = 0.01,
                );
            }
        );


        let clone_for_test_four: SimpleShellAndTubeHeatExchanger = 
            sthe_one_shell_one_tube.clone();
        let test_four = thread::spawn(
            move || {
                let inlet_temp_salt = 
                    ThermodynamicTemperature::new::<degree_celsius>(236.91);
                let inlet_temp_oil = 
                    ThermodynamicTemperature::new::<degree_celsius>(80.41);
                let vol_flowrate_salt = 
                    VolumeRate::new::<cubic_meter_per_hour>(12.63);
                let vol_flowrate_oil = 
                    VolumeRate::new::<cubic_meter_per_hour>(15.635);

                // intermediate calcs

                let inlet_rho_salt = 
                    LiquidMaterial::HITEC.try_get_density(inlet_temp_salt).unwrap();
                let inlet_rho_oil = 
                    LiquidMaterial::YD325.try_get_density(inlet_temp_oil).unwrap();

                let m_t: MassRate = inlet_rho_oil * vol_flowrate_oil;
                let m_s: MassRate = -inlet_rho_salt * vol_flowrate_salt;

                let tube_inlet_temperature = inlet_temp_oil;
                let shell_inlet_temperature = inlet_temp_salt;

                let (reynolds_num_a4,
                    bulk_prandtl_a4,
                    wall_prandtl_a4,
                    nusselt_number_a4) = test_thread(
                    clone_for_test_four,
                    tube_inlet_temperature,
                    shell_inlet_temperature,
                    m_t,
                    m_s);

                dbg!(&(reynolds_num_a4,
                        bulk_prandtl_a4,
                        wall_prandtl_a4,
                        nusselt_number_a4));
                // assert that the Reynolds, Prandtl bulk and 
                // Prandtl wall are equal to some amount

                assert_relative_eq!(
                    reynolds_num_a4.get::<ratio>(),
                    4205.0,
                    max_relative = 0.01,
                );

                assert_relative_eq!(
                    bulk_prandtl_a4.get::<ratio>(),
                    20.11,
                    max_relative = 0.01,
                );

                assert_relative_eq!(
                    wall_prandtl_a4.get::<ratio>(),
                    37.89,
                    max_relative = 0.01,
                );

                assert_relative_eq!(
                    nusselt_number_a4.get::<ratio>(),
                    48.96,
                    max_relative = 0.01,
                );
            }
        );


        let clone_for_test_five: SimpleShellAndTubeHeatExchanger = 
            sthe_one_shell_one_tube.clone();
        let test_five = thread::spawn(
            move || {
                let inlet_temp_salt = 
                    ThermodynamicTemperature::new::<degree_celsius>(214.93);
                let inlet_temp_oil = 
                    ThermodynamicTemperature::new::<degree_celsius>(90.41);
                let vol_flowrate_salt = 
                    VolumeRate::new::<cubic_meter_per_hour>(12.63);
                let vol_flowrate_oil = 
                    VolumeRate::new::<cubic_meter_per_hour>(15.635);

                // intermediate calcs

                let inlet_rho_salt = 
                    LiquidMaterial::HITEC.try_get_density(inlet_temp_salt).unwrap();
                let inlet_rho_oil = 
                    LiquidMaterial::YD325.try_get_density(inlet_temp_oil).unwrap();

                let m_t: MassRate = inlet_rho_oil * vol_flowrate_oil;
                let m_s: MassRate = -inlet_rho_salt * vol_flowrate_salt;

                let tube_inlet_temperature = inlet_temp_oil;
                let shell_inlet_temperature = inlet_temp_salt;

                let (reynolds_num_a5,
                    bulk_prandtl_a5,
                    wall_prandtl_a5,
                    nusselt_number_a5) = test_thread(
                    clone_for_test_five,
                    tube_inlet_temperature,
                    shell_inlet_temperature,
                    m_t,
                    m_s);

                dbg!(&(reynolds_num_a5,
                        bulk_prandtl_a5,
                        wall_prandtl_a5,
                        nusselt_number_a5));
                // assert that the Reynolds, Prandtl bulk and 
                // Prandtl wall are equal to some amount

                assert_relative_eq!(
                    reynolds_num_a5.get::<ratio>(),
                    3506.0,
                    max_relative = 0.01,
                );

                assert_relative_eq!(
                    bulk_prandtl_a5.get::<ratio>(),
                    24.09,
                    max_relative = 0.01,
                );

                assert_relative_eq!(
                    wall_prandtl_a5.get::<ratio>(),
                    37.89,
                    max_relative = 0.01,
                );

                assert_relative_eq!(
                    nusselt_number_a5.get::<ratio>(),
                    42.51,
                    max_relative = 0.01,
                );
            }
        );


        let clone_for_test_six: SimpleShellAndTubeHeatExchanger = 
            sthe_one_shell_one_tube.clone();
        let test_six = thread::spawn(
            move || {
                let inlet_temp_salt = 
                    ThermodynamicTemperature::new::<degree_celsius>(224.93);
                let inlet_temp_oil = 
                    ThermodynamicTemperature::new::<degree_celsius>(80.49);
                let vol_flowrate_salt = 
                    VolumeRate::new::<cubic_meter_per_hour>(12.63);
                let vol_flowrate_oil = 
                    VolumeRate::new::<cubic_meter_per_hour>(15.635);

                // intermediate calcs

                let inlet_rho_salt = 
                    LiquidMaterial::HITEC.try_get_density(inlet_temp_salt).unwrap();
                let inlet_rho_oil = 
                    LiquidMaterial::YD325.try_get_density(inlet_temp_oil).unwrap();

                let m_t: MassRate = inlet_rho_oil * vol_flowrate_oil;
                let m_s: MassRate = -inlet_rho_salt * vol_flowrate_salt;

                let tube_inlet_temperature = inlet_temp_oil;
                let shell_inlet_temperature = inlet_temp_salt;

                let (reynolds_num_a6,
                    bulk_prandtl_a6,
                    wall_prandtl_a6,
                    nusselt_number_a6) = test_thread(
                    clone_for_test_six,
                    tube_inlet_temperature,
                    shell_inlet_temperature,
                    m_t,
                    m_s);

                dbg!(&(reynolds_num_a6,
                        bulk_prandtl_a6,
                        wall_prandtl_a6,
                        nusselt_number_a6));
                // assert that the Reynolds, Prandtl bulk and 
                // Prandtl wall are equal to some amount

                assert_relative_eq!(
                    reynolds_num_a6.get::<ratio>(),
                    3811.0,
                    max_relative = 0.01,
                );

                assert_relative_eq!(
                    bulk_prandtl_a6.get::<ratio>(),
                    22.17,
                    max_relative = 0.01,
                );

                assert_relative_eq!(
                    wall_prandtl_a6.get::<ratio>(),
                    37.89,
                    max_relative = 0.01,
                );

                assert_relative_eq!(
                    nusselt_number_a6.get::<ratio>(),
                    45.52,
                    max_relative = 0.01,
                );
            }
        );


        let clone_for_test_seven: SimpleShellAndTubeHeatExchanger = 
            sthe_one_shell_one_tube.clone();
        let test_seven = thread::spawn(
            move || {
                let inlet_temp_salt = 
                    ThermodynamicTemperature::new::<degree_celsius>(236.91);
                let inlet_temp_oil = 
                    ThermodynamicTemperature::new::<degree_celsius>(90.49);
                let vol_flowrate_salt = 
                    VolumeRate::new::<cubic_meter_per_hour>(12.63);
                let vol_flowrate_oil = 
                    VolumeRate::new::<cubic_meter_per_hour>(15.635);

                // intermediate calcs

                let inlet_rho_salt = 
                    LiquidMaterial::HITEC.try_get_density(inlet_temp_salt).unwrap();
                let inlet_rho_oil = 
                    LiquidMaterial::YD325.try_get_density(inlet_temp_oil).unwrap();

                let m_t: MassRate = inlet_rho_oil * vol_flowrate_oil;
                let m_s: MassRate = -inlet_rho_salt * vol_flowrate_salt;

                let tube_inlet_temperature = inlet_temp_oil;
                let shell_inlet_temperature = inlet_temp_salt;

                let (reynolds_num_a7,
                    bulk_prandtl_a7,
                    wall_prandtl_a7,
                    nusselt_number_a7) = test_thread(
                    clone_for_test_seven,
                    tube_inlet_temperature,
                    shell_inlet_temperature,
                    m_t,
                    m_s);

                dbg!(&(reynolds_num_a7,
                        bulk_prandtl_a7,
                        wall_prandtl_a7,
                        nusselt_number_a7));
                // assert that the Reynolds, Prandtl bulk and 
                // Prandtl wall are equal to some amount

                assert_relative_eq!(
                    reynolds_num_a7.get::<ratio>(),
                    4212.0,
                    max_relative = 0.01,
                );

                assert_relative_eq!(
                    bulk_prandtl_a7.get::<ratio>(),
                    20.07,
                    max_relative = 0.01,
                );

                assert_relative_eq!(
                    wall_prandtl_a7.get::<ratio>(),
                    37.89,
                    max_relative = 0.01,
                );

                assert_relative_eq!(
                    nusselt_number_a7.get::<ratio>(),
                    48.95,
                    max_relative = 0.01,
                );
            }
        );

        test_two.join().unwrap();
        test_three.join().unwrap();
        test_four.join().unwrap();
        test_five.join().unwrap();
        test_six.join().unwrap();
        test_seven.join().unwrap();


    }


    // note: I'm using a panic to reveal the dbg! information 
    //todo!("using panic for debug info");


}

