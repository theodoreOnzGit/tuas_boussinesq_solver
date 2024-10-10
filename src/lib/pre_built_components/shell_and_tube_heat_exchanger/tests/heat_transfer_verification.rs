

/// this is a basic test for shell and tube heat exchanger
///
/// I'm assuming an adiabatic bc to the outside
/// and switching off the insulation boolean
///
#[test]
pub fn basic_test_shell_and_tube_heat_exchanger(){

    use std::f64::consts::PI;

    use std::thread;

    use uom::si::time::second;
    use uom::si::thermodynamic_temperature::kelvin;

    use crate::heat_transfer_correlations::
        heat_transfer_interactions::heat_transfer_interaction_enums
        ::HeatTransferInteractionType;
    use crate::boundary_conditions::BCType;


    use uom::si::angle::degree;
    //use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::length::{meter, millimeter};
    use uom::si::pressure::atmosphere;
    use uom::si::ratio::ratio;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::ConstZero;
    use uom::si::mass_rate::kilogram_per_second;
    //use uom::si::area::square_meter;

    use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;

    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
    use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
    use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
    use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::pre_built_components::shell_and_tube_heat_exchanger::SimpleShellAndTubeHeatExchanger;
    use uom::si::f64::*;
    let number_of_tubes = 1_u32;
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
        - PI * 0.25 * tube_side_od * tube_side_od;

    let shell_side_fluid_hydraulic_diameter: Length = 
        (shell_side_id * shell_side_id - tube_side_od * tube_side_od)/
        (shell_side_id + tube_side_od);

    let hitec: LiquidMaterial = LiquidMaterial::HITEC;
    let steel: SolidMaterial = SolidMaterial::SteelSS304L;

    let form_loss = Ratio::new::<ratio>(0.0);

    // initial temperature is 250C 
    let initial_temperature = ThermodynamicTemperature::
        new::<degree_celsius>(250.0);
    // ambient temperature is 25C 
    let ambient_temperature = ThermodynamicTemperature::
        new::<degree_celsius>(25.0);

    // adiabatic, heat transfer for ambient is zero 
    let heat_transfer_to_ambient = HeatTransfer::ZERO;

    let number_of_inner_nodes = 8;

    let incline_angle = Angle::new::<degree>(0.0);

    // inner fluid_array
    // the nusselt correlation here is a standard pipe correlation 
    // but I don't use that
    let tube_side_fluid_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            pipe_length,
            tube_side_id,
            tube_side_flow_area,
            initial_temperature,
            fluid_pressure,
            steel,
            hitec,
            form_loss,
            number_of_inner_nodes,
            incline_angle
        );

    // shell side fluid_array
    // the nusselt correlation here is a standard pipe correlation 
    // but I don't use that
    let shell_side_fluid_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            pipe_length,
            shell_side_fluid_hydraulic_diameter,
            shell_side_flow_area,
            initial_temperature,
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
            initial_temperature, 
            solid_pressure, 
            steel, 
            number_of_inner_nodes
            );

    let inner_shell: SolidColumn 
        = SolidColumn::new_cylindrical_shell(
            pipe_length, 
            tube_side_id, 
            tube_side_od, 
            initial_temperature, 
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
    // nusselt correlaitons
    let shell_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            pipe_length, 
            Length::new::<millimeter>(0.001), 
            shell_side_fluid_hydraulic_diameter, 
            form_loss
            );


    let tube_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            pipe_length, 
            Length::new::<millimeter>(0.001), 
            tube_side_id, 
            form_loss
            );

    // nusselt correlations, 4.36 by default

    let shell_side_nusselt_correlation_to_tubes = 
        NusseltCorrelation::PipeConstantHeatFluxFullyDeveloped;

    let tube_side_nusselt_correlation = 
        NusseltCorrelation::PipeConstantHeatFluxFullyDeveloped;

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
            shell_side_nusselt_correlation_to_tubes, 
            shell_side_nusselt_correlation_parasitic: shell_side_nusselt_correlation_to_outer_shell, 
            tube_side_nusselt_correlation, 
            insulation_thickness: dummy_insulation_thickness,
        };

    //let correct_for_prandtl_wall_temperatures = true;
    //let overall_heat_exchg_coeff = 
    //    sthe_one_shell_one_tube.overall_heat_transfer_coeff_u_shell_side(
    //        correct_for_prandtl_wall_temperatures).unwrap();

    //let u_val_watts_per_sqm_kelvin = 
    //    overall_heat_exchg_coeff.get::<watt_per_square_meter_kelvin>();
    //let shell_side_tube_area: Area = 
    //    sthe_one_shell_one_tube.tube_bundle_heat_transfer_area_shell_side();

    //let shell_side_tube_area_m2 = 
    //    shell_side_tube_area.get::<square_meter>();

    //let ua: ThermalConductance = 
    //    overall_heat_exchg_coeff * shell_side_tube_area;

    // m_t/m_s = 2,
    // shell side flowrate is half of tube side mass flowrate
    //
    // but the direction differs in a counter current setup, 
    // so I'll have to add a negative sign
    let m_t = MassRate::new::<kilogram_per_second>(0.005);
    let m_s = -m_t * 0.5;

    let tube_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(230.0);
    let shell_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(270.0);

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

        let max_time = Time::new::<second>(4e4_f64);
        let timestep = Time::new::<second>(0.5);
        let mut simulation_time = Time::ZERO;

        // for simplicity, I'm going to use an average HITEC 
        // density at 250C (or just the shell and tube inlet temp)
        let average_temp_kelvin = 0.5* tube_inlet_temperature.value +
            0.5*shell_inlet_temperature.value;
        let average_temp = ThermodynamicTemperature::new::<kelvin>(
            average_temp_kelvin);
        let average_hitec_density = 
            LiquidMaterial::HITEC.try_get_density(average_temp).unwrap();

        let tube_inlet_density = average_hitec_density;
        let tube_back_cv_density = average_hitec_density;
        let tube_front_cv_density = average_hitec_density;
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

            let prandtl_wall_correction_setting = false;

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

    let test_thread = move |mut sthe: SimpleShellAndTubeHeatExchanger,
    tube_inlet_temperature: ThermodynamicTemperature,
    shell_inlet_temperature: ThermodynamicTemperature,
    m_t: MassRate,
    m_s: MassRate,| {


        let tube_outlet_temperature: ThermodynamicTemperature 
            = obtain_outlet_steady_state_temp(
                &mut sthe, 
                tube_inlet_temperature, 
                shell_inlet_temperature, 
                m_t, 
                m_s);
        let correct_for_prandtl_wall_temperatures = false;

        let ua: ThermalConductance 
            = sthe.overall_heat_transfer_coeff_u_shell_side(
            correct_for_prandtl_wall_temperatures).unwrap() * 
            sthe.circular_tube_bundle_heat_transfer_area_shell_side();

        // shell side outlet temperature 
        let shell_side_outlet_temperature: ThermodynamicTemperature = {
            let temperature_vec_shell_side = 
                sthe.shell_side_fluid_array. 
                get_temperature_vector().unwrap();

            *temperature_vec_shell_side.first().unwrap()

        };

        dbg!(&(tube_inlet_temperature.get::<degree_celsius>(),
                shell_inlet_temperature.get::<degree_celsius>(),
                tube_outlet_temperature.get::<degree_celsius>(),
                shell_side_outlet_temperature.get::<degree_celsius>(),
                m_t,
                m_s,
                ua));


    };

    // first test 

    let clone_for_test_one: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();

    let test_one = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_one,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );

    // second test 

    let clone_for_test_two: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();
    let m_t = MassRate::new::<kilogram_per_second>(0.05);
    let m_s = -m_t * 0.5;

    let test_two = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_two,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );


    // third test



    // too high or too low flowrates means instabilities
    // or not reached steady state yet
    // ie 0.5 kg/s or 5e-4 kg/s
    //{
    //    let m_t = MassRate::new::<kilogram_per_second>(0.05);
    //    let m_s = -m_t * 0.5;
    //    let tube_outlet_temperature: ThermodynamicTemperature 
    //        = obtain_outlet_steady_state_temp(
    //            &mut sthe_one_shell_one_tube, 
    //            tube_inlet_temperature, 
    //            shell_inlet_temperature, 
    //            m_t, 
    //            m_s);
    //    dbg!(&m_t);
    //    dbg!(&tube_outlet_temperature);
    //}
    
    // let's change the shell and tube inlet temperatures instead
    let m_t = MassRate::new::<kilogram_per_second>(0.05);
    let m_s = -m_t * 0.5;
    let tube_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(180_f64);
    let shell_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(280_f64);
    let clone_for_test_three: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();
    let test_three = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_three,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );

    // invert shell and tube temperatures
    let m_t = MassRate::new::<kilogram_per_second>(0.05);
    let m_s = -m_t * 0.5;
    let tube_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(280_f64);
    let shell_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(180_f64);


    let clone_for_test_four: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();
    let test_four = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_four,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );

    // test 5
    // slightly lower mass flowrate 
    // not quite steady state...
    let m_t = MassRate::new::<kilogram_per_second>(0.02);
    let m_s = -m_t * 0.5;
    let tube_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(180_f64);
    let shell_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(280_f64);
    let clone_for_test_five: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();
    let test_five = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_five,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );
    // test 6
    // slightly increase mass flowrate 
    //
    let m_t = MassRate::new::<kilogram_per_second>(0.045);
    let m_s = -m_t * 0.5;
    let tube_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(180_f64);
    let shell_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(280_f64);
    let clone_for_test_six: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();
    let test_six = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_six,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );

    // test 7
    // slightly increase mass flowrate 
    // 0.1 kg/s 220C tube, 310C shell had numerical instability
    // 0.07 kg/s 220C tube, 310C shell had numerical instability given the 
    // time step settings
    // change temperatures
    let m_t = MassRate::new::<kilogram_per_second>(0.04);
    let m_s = -m_t * 0.5;
    let tube_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(220_f64);
    let shell_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(270_f64);
    let clone_for_test_seven: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();
    let test_seven = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_seven,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );
    //join and unwrap all threads 

    test_one.join().unwrap();
    test_two.join().unwrap();
    test_three.join().unwrap();
    test_four.join().unwrap();
    test_five.join().unwrap();
    test_six.join().unwrap();
    test_seven.join().unwrap();




    // note: use a panic to reveal the dbg! information 
    //todo!();


}


/// this is a basic test for shell and tube heat exchanger
///
/// I'm assuming an adiabatic bc to the outside
/// and switching off the insulation boolean
//
/// this time, I reduce the diameter of the shell side, so it 
/// reaches steady state faster
#[test]
pub fn basic_test_shell_and_tube_heat_exchanger_set_two(){

    use std::f64::consts::PI;

    use std::thread;

    use uom::si::time::second;
    use uom::si::thermodynamic_temperature::kelvin;

    use crate::heat_transfer_correlations::
        heat_transfer_interactions::heat_transfer_interaction_enums
        ::HeatTransferInteractionType;
    use crate::boundary_conditions::BCType;


    use uom::si::angle::degree;
    //use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::length::{meter, millimeter};
    use uom::si::pressure::atmosphere;
    use uom::si::ratio::ratio;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::ConstZero;
    use uom::si::mass_rate::kilogram_per_second;
    //use uom::si::area::square_meter;

    use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;

    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
    use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
    use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
    use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::pre_built_components::shell_and_tube_heat_exchanger::SimpleShellAndTubeHeatExchanger;
    use uom::si::f64::*;
    let number_of_tubes = 1_u32;
    let fluid_pressure = Pressure::new::<atmosphere>(1.0);
    let solid_pressure = Pressure::new::<atmosphere>(1.0);

    // from Du's heat exchanger type, except we use one inner tube
    let tube_side_od = Length::new::<meter>(0.014);
    let tube_side_id = Length::new::<meter>(0.01);
    let shell_side_od = Length::new::<meter>(0.048);
    let shell_side_id = Length::new::<meter>(0.04);
    let pipe_length = Length::new::<meter>(1.95);

    let tube_side_flow_area: Area 
        = PI * 0.25 * tube_side_id * tube_side_id;

    let shell_side_flow_area: Area 
        = PI * 0.25 * shell_side_id * shell_side_id 
        - PI * 0.25 * tube_side_od * tube_side_od;

    let shell_side_fluid_hydraulic_diameter: Length = 
        (shell_side_id * shell_side_id - tube_side_od * tube_side_od)/
        (shell_side_id + tube_side_od);

    let hitec: LiquidMaterial = LiquidMaterial::HITEC;
    let steel: SolidMaterial = SolidMaterial::SteelSS304L;

    let form_loss = Ratio::new::<ratio>(0.0);

    // initial temperature is 250C 
    let initial_temperature = ThermodynamicTemperature::
        new::<degree_celsius>(250.0);
    // ambient temperature is 25C 
    let ambient_temperature = ThermodynamicTemperature::
        new::<degree_celsius>(25.0);

    // adiabatic, heat transfer for ambient is zero 
    let heat_transfer_to_ambient = HeatTransfer::ZERO;

    let number_of_inner_nodes = 8;

    let incline_angle = Angle::new::<degree>(0.0);

    // inner fluid_array
    // the nusselt correlation here is a standard pipe correlation 
    // but I don't use that
    let tube_side_fluid_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            pipe_length,
            tube_side_id,
            tube_side_flow_area,
            initial_temperature,
            fluid_pressure,
            steel,
            hitec,
            form_loss,
            number_of_inner_nodes,
            incline_angle
        );

    // shell side fluid_array
    // the nusselt correlation here is a standard pipe correlation 
    // but I don't use that
    let shell_side_fluid_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            pipe_length,
            shell_side_fluid_hydraulic_diameter,
            shell_side_flow_area,
            initial_temperature,
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
            initial_temperature, 
            solid_pressure, 
            steel, 
            number_of_inner_nodes
            );

    let inner_shell: SolidColumn 
        = SolidColumn::new_cylindrical_shell(
            pipe_length, 
            tube_side_id, 
            tube_side_od, 
            initial_temperature, 
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
    // nusselt correlaitons
    let shell_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            pipe_length, 
            Length::new::<millimeter>(0.001), 
            shell_side_fluid_hydraulic_diameter, 
            form_loss
            );


    let tube_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            pipe_length, 
            Length::new::<millimeter>(0.001), 
            tube_side_id, 
            form_loss
            );

    // nusselt correlations, 4.36 by default

    let shell_side_nusselt_correlation_to_tubes = 
        NusseltCorrelation::PipeConstantHeatFluxFullyDeveloped;

    let tube_side_nusselt_correlation = 
        NusseltCorrelation::PipeConstantHeatFluxFullyDeveloped;

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
            shell_side_nusselt_correlation_to_tubes, 
            shell_side_nusselt_correlation_parasitic: shell_side_nusselt_correlation_to_outer_shell, 
            tube_side_nusselt_correlation, 
            insulation_thickness: dummy_insulation_thickness,
        };

    //let correct_for_prandtl_wall_temperatures = true;
    //let overall_heat_exchg_coeff = 
    //    sthe_one_shell_one_tube.overall_heat_transfer_coeff_u_shell_side(
    //        correct_for_prandtl_wall_temperatures).unwrap();

    //let u_val_watts_per_sqm_kelvin = 
    //    overall_heat_exchg_coeff.get::<watt_per_square_meter_kelvin>();
    //let shell_side_tube_area: Area = 
    //    sthe_one_shell_one_tube.tube_bundle_heat_transfer_area_shell_side();

    //let shell_side_tube_area_m2 = 
    //    shell_side_tube_area.get::<square_meter>();

    //let ua: ThermalConductance = 
    //    overall_heat_exchg_coeff * shell_side_tube_area;

    // m_t/m_s = 2,
    // shell side flowrate is half of tube side mass flowrate
    //
    // but the direction differs in a counter current setup, 
    // so I'll have to add a negative sign
    let m_t = MassRate::new::<kilogram_per_second>(0.005);
    let m_s = -m_t * 0.5;

    let tube_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(230.0);
    let shell_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(270.0);

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

        let max_time = Time::new::<second>(4e4_f64);
        let timestep = Time::new::<second>(0.1);
        let mut simulation_time = Time::ZERO;

        // for simplicity, I'm going to use an average HITEC 
        // density at 250C (or just the shell and tube inlet temp)
        let average_temp_kelvin = 0.5* tube_inlet_temperature.value +
            0.5*shell_inlet_temperature.value;
        let average_temp = ThermodynamicTemperature::new::<kelvin>(
            average_temp_kelvin);
        let average_hitec_density = 
            LiquidMaterial::HITEC.try_get_density(average_temp).unwrap();

        let tube_inlet_density = average_hitec_density;
        let tube_back_cv_density = average_hitec_density;
        let tube_front_cv_density = average_hitec_density;
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

            let prandtl_wall_correction_setting = false;

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

    let test_thread = move |mut sthe: SimpleShellAndTubeHeatExchanger,
    tube_inlet_temperature: ThermodynamicTemperature,
    shell_inlet_temperature: ThermodynamicTemperature,
    m_t: MassRate,
    m_s: MassRate,| {


        let tube_outlet_temperature: ThermodynamicTemperature 
            = obtain_outlet_steady_state_temp(
                &mut sthe, 
                tube_inlet_temperature, 
                shell_inlet_temperature, 
                m_t, 
                m_s);
        let correct_for_prandtl_wall_temperatures = false;

        let ua: ThermalConductance 
            = sthe.overall_heat_transfer_coeff_u_shell_side(
            correct_for_prandtl_wall_temperatures).unwrap() * 
            sthe.circular_tube_bundle_heat_transfer_area_shell_side();

        // shell side outlet temperature 
        let shell_side_outlet_temperature: ThermodynamicTemperature = {
            let temperature_vec_shell_side = 
                sthe.shell_side_fluid_array. 
                get_temperature_vector().unwrap();

            *temperature_vec_shell_side.first().unwrap()

        };

        dbg!(&(tube_inlet_temperature.get::<degree_celsius>(),
                shell_inlet_temperature.get::<degree_celsius>(),
                tube_outlet_temperature.get::<degree_celsius>(),
                shell_side_outlet_temperature.get::<degree_celsius>(),
                m_t,
                m_s,
                ua));


    };

    // first test 

    let clone_for_test_one: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();

    let test_one = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_one,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );

    // second test 

    let clone_for_test_two: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();
    let m_t = MassRate::new::<kilogram_per_second>(0.05);
    let m_s = -m_t * 0.5;

    let test_two = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_two,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );


    // third test



    // too high or too low flowrates means instabilities
    // or not reached steady state yet
    // ie 0.5 kg/s or 5e-4 kg/s
    //{
    //    let m_t = MassRate::new::<kilogram_per_second>(0.05);
    //    let m_s = -m_t * 0.5;
    //    let tube_outlet_temperature: ThermodynamicTemperature 
    //        = obtain_outlet_steady_state_temp(
    //            &mut sthe_one_shell_one_tube, 
    //            tube_inlet_temperature, 
    //            shell_inlet_temperature, 
    //            m_t, 
    //            m_s);
    //    dbg!(&m_t);
    //    dbg!(&tube_outlet_temperature);
    //}
    
    // let's change the shell and tube inlet temperatures instead
    let m_t = MassRate::new::<kilogram_per_second>(0.05);
    let m_s = -m_t * 0.5;
    let tube_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(180_f64);
    let shell_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(280_f64);
    let clone_for_test_three: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();
    let test_three = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_three,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );

    // invert shell and tube temperatures
    let m_t = MassRate::new::<kilogram_per_second>(0.05);
    let m_s = -m_t * 0.5;
    let tube_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(280_f64);
    let shell_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(180_f64);


    let clone_for_test_four: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();
    let test_four = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_four,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );

    // test 5
    // slightly lower mass flowrate 
    // not quite steady state...
    let m_t = MassRate::new::<kilogram_per_second>(0.02);
    let m_s = -m_t * 0.5;
    let tube_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(180_f64);
    let shell_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(280_f64);
    let clone_for_test_five: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();
    let test_five = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_five,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );
    // test 6
    // slightly increase mass flowrate 
    //
    let m_t = MassRate::new::<kilogram_per_second>(0.045);
    let m_s = -m_t * 0.5;
    let tube_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(180_f64);
    let shell_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(280_f64);
    let clone_for_test_six: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();
    let test_six = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_six,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );

    // test 7
    // slightly increase mass flowrate 
    // 0.1 kg/s 220C tube, 310C shell had numerical instability
    // 0.07 kg/s 220C tube, 310C shell had numerical instability given the 
    // time step settings
    // change temperatures
    let m_t = MassRate::new::<kilogram_per_second>(0.04);
    let m_s = -m_t * 0.5;
    let tube_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(220_f64);
    let shell_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(270_f64);
    let clone_for_test_seven: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();
    let test_seven = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_seven,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );
    //join and unwrap all threads 

    test_one.join().unwrap();
    test_two.join().unwrap();
    test_three.join().unwrap();
    test_four.join().unwrap();
    test_five.join().unwrap();
    test_six.join().unwrap();
    test_seven.join().unwrap();




    // note: use a panic to reveal the dbg! information 
    //todo!();


}


/// this is a basic test for shell and tube heat exchanger
///
/// I'm assuming an adiabatic bc to the outside
/// and switching off the insulation boolean
///
/// this test is similar to the first set, except now I have 19 tubes
/// similar to Du
#[test]
pub fn basic_test_shell_and_tube_heat_exchanger_set_three(){

    use std::f64::consts::PI;

    use std::thread;

    use uom::si::time::second;
    use uom::si::thermodynamic_temperature::kelvin;

    use crate::heat_transfer_correlations::
        heat_transfer_interactions::heat_transfer_interaction_enums
        ::HeatTransferInteractionType;
    use crate::boundary_conditions::BCType;


    use uom::si::angle::degree;
    //use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::length::{meter, millimeter};
    use uom::si::pressure::atmosphere;
    use uom::si::ratio::ratio;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::ConstZero;
    use uom::si::mass_rate::kilogram_per_second;
    //use uom::si::area::square_meter;

    use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;

    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
    use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
    use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
    use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::pre_built_components::shell_and_tube_heat_exchanger::SimpleShellAndTubeHeatExchanger;
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
    let steel: SolidMaterial = SolidMaterial::SteelSS304L;

    let form_loss = Ratio::new::<ratio>(0.0);

    // initial temperature is 250C 
    let initial_temperature = ThermodynamicTemperature::
        new::<degree_celsius>(250.0);
    // ambient temperature is 25C 
    let ambient_temperature = ThermodynamicTemperature::
        new::<degree_celsius>(25.0);

    // adiabatic, heat transfer for ambient is zero 
    let heat_transfer_to_ambient = HeatTransfer::ZERO;

    let number_of_inner_nodes = 8;

    let incline_angle = Angle::new::<degree>(0.0);

    // inner fluid_array
    // the nusselt correlation here is a standard pipe correlation 
    // but I don't use that
    let tube_side_fluid_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            pipe_length,
            tube_side_id,
            tube_side_flow_area,
            initial_temperature,
            fluid_pressure,
            steel,
            hitec,
            form_loss,
            number_of_inner_nodes,
            incline_angle
        );

    // shell side fluid_array
    // the nusselt correlation here is a standard pipe correlation 
    // but I don't use that
    let shell_side_fluid_array: FluidArray = 
        FluidArray::new_odd_shaped_pipe(
            pipe_length,
            shell_side_fluid_hydraulic_diameter,
            shell_side_flow_area,
            initial_temperature,
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
            initial_temperature, 
            solid_pressure, 
            steel, 
            number_of_inner_nodes
            );

    let inner_shell: SolidColumn 
        = SolidColumn::new_cylindrical_shell(
            pipe_length, 
            tube_side_id, 
            tube_side_od, 
            initial_temperature, 
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
    // nusselt correlaitons
    let shell_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            pipe_length, 
            Length::new::<millimeter>(0.001), 
            shell_side_fluid_hydraulic_diameter, 
            form_loss
            );


    let tube_loss_correlations: DimensionlessDarcyLossCorrelations
        = DimensionlessDarcyLossCorrelations::new_pipe(
            pipe_length, 
            Length::new::<millimeter>(0.001), 
            tube_side_id, 
            form_loss
            );

    // nusselt correlations, 4.36 by default

    let shell_side_nusselt_correlation_to_tubes = 
        NusseltCorrelation::PipeConstantHeatFluxFullyDeveloped;

    let tube_side_nusselt_correlation = 
        NusseltCorrelation::PipeConstantHeatFluxFullyDeveloped;

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
            shell_side_nusselt_correlation_to_tubes, 
            shell_side_nusselt_correlation_parasitic: shell_side_nusselt_correlation_to_outer_shell, 
            tube_side_nusselt_correlation, 
            insulation_thickness: dummy_insulation_thickness,
        };

    //let correct_for_prandtl_wall_temperatures = true;
    //let overall_heat_exchg_coeff = 
    //    sthe_one_shell_one_tube.overall_heat_transfer_coeff_u_shell_side(
    //        correct_for_prandtl_wall_temperatures).unwrap();

    //let u_val_watts_per_sqm_kelvin = 
    //    overall_heat_exchg_coeff.get::<watt_per_square_meter_kelvin>();
    //let shell_side_tube_area: Area = 
    //    sthe_one_shell_one_tube.tube_bundle_heat_transfer_area_shell_side();

    //let shell_side_tube_area_m2 = 
    //    shell_side_tube_area.get::<square_meter>();

    //let ua: ThermalConductance = 
    //    overall_heat_exchg_coeff * shell_side_tube_area;

    // m_t/m_s = 2,
    // shell side flowrate is half of tube side mass flowrate
    //
    // but the direction differs in a counter current setup, 
    // so I'll have to add a negative sign
    let m_t = MassRate::new::<kilogram_per_second>(0.005);
    let m_s = -m_t * 0.5;

    let tube_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(230.0);
    let shell_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(270.0);

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

        let max_time = Time::new::<second>(4e4_f64);
        let timestep = Time::new::<second>(0.5);
        let mut simulation_time = Time::ZERO;

        // for simplicity, I'm going to use an average HITEC 
        // density at 250C (or just the shell and tube inlet temp)
        let average_temp_kelvin = 0.5* tube_inlet_temperature.value +
            0.5*shell_inlet_temperature.value;
        let average_temp = ThermodynamicTemperature::new::<kelvin>(
            average_temp_kelvin);
        let average_hitec_density = 
            LiquidMaterial::HITEC.try_get_density(average_temp).unwrap();

        let tube_inlet_density = average_hitec_density;
        let tube_back_cv_density = average_hitec_density;
        let tube_front_cv_density = average_hitec_density;
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

            let prandtl_wall_correction_setting = false;

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

    let test_thread = move |mut sthe: SimpleShellAndTubeHeatExchanger,
    tube_inlet_temperature: ThermodynamicTemperature,
    shell_inlet_temperature: ThermodynamicTemperature,
    m_t: MassRate,
    m_s: MassRate,| {


        let tube_outlet_temperature: ThermodynamicTemperature 
            = obtain_outlet_steady_state_temp(
                &mut sthe, 
                tube_inlet_temperature, 
                shell_inlet_temperature, 
                m_t, 
                m_s);
        let correct_for_prandtl_wall_temperatures = false;

        let ua: ThermalConductance 
            = sthe.overall_heat_transfer_coeff_u_shell_side(
            correct_for_prandtl_wall_temperatures).unwrap() * 
            sthe.circular_tube_bundle_heat_transfer_area_shell_side();

        // shell side outlet temperature and inlet cv temperature
        let (shell_side_inlet_cv_temperature, 
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
        let tube_side_inlet_cv_temperature: ThermodynamicTemperature = {

            let temperature_vec_tube_side = 
                sthe.tube_side_fluid_array_for_single_tube. 
                get_temperature_vector().unwrap();

            let tube_side_inlet_cv_temperature = 
                *temperature_vec_tube_side.first().unwrap();

            tube_side_inlet_cv_temperature

        };

        dbg!(&(tube_inlet_temperature.get::<degree_celsius>(),
        tube_side_inlet_cv_temperature.get::<degree_celsius>(),
        shell_inlet_temperature.get::<degree_celsius>(),
        shell_side_inlet_cv_temperature.get::<degree_celsius>(),
        tube_outlet_temperature.get::<degree_celsius>(),
        shell_side_outlet_temperature.get::<degree_celsius>(),
        m_t,
        m_s,
        ua));


    };

    // first test 

    let clone_for_test_one: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();

    let test_one = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_one,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );

    // second test 

    let clone_for_test_two: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();
    let m_t = MassRate::new::<kilogram_per_second>(0.05);
    let m_s = -m_t * 0.5;

    let test_two = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_two,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );


    // third test



    // too high or too low flowrates means instabilities
    // or not reached steady state yet
    // ie 0.5 kg/s or 5e-4 kg/s
    //{
    //    let m_t = MassRate::new::<kilogram_per_second>(0.05);
    //    let m_s = -m_t * 0.5;
    //    let tube_outlet_temperature: ThermodynamicTemperature 
    //        = obtain_outlet_steady_state_temp(
    //            &mut sthe_one_shell_one_tube, 
    //            tube_inlet_temperature, 
    //            shell_inlet_temperature, 
    //            m_t, 
    //            m_s);
    //    dbg!(&m_t);
    //    dbg!(&tube_outlet_temperature);
    //}
    
    // let's change the shell and tube inlet temperatures instead
    let m_t = MassRate::new::<kilogram_per_second>(0.05);
    let m_s = -m_t * 0.5;
    let tube_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(180_f64);
    let shell_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(280_f64);
    let clone_for_test_three: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();
    let test_three = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_three,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );

    // invert shell and tube temperatures
    let m_t = MassRate::new::<kilogram_per_second>(0.05);
    let m_s = -m_t * 0.5;
    let tube_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(280_f64);
    let shell_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(180_f64);


    let clone_for_test_four: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();
    let test_four = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_four,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );

    // test 5
    // slightly lower mass flowrate 
    // not quite steady state...
    let m_t = MassRate::new::<kilogram_per_second>(0.02);
    let m_s = -m_t * 0.5;
    let tube_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(180_f64);
    let shell_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(280_f64);
    let clone_for_test_five: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();
    let test_five = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_five,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );
    // test 6
    // slightly increase mass flowrate 
    //
    let m_t = MassRate::new::<kilogram_per_second>(0.045);
    let m_s = -m_t * 0.5;
    let tube_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(180_f64);
    let shell_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(280_f64);
    let clone_for_test_six: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();
    let test_six = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_six,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );

    // test 7
    // slightly increase mass flowrate 
    // 0.1 kg/s 220C tube, 310C shell had numerical instability
    // 0.07 kg/s 220C tube, 310C shell had numerical instability given the 
    // time step settings
    // change temperatures
    let m_t = MassRate::new::<kilogram_per_second>(0.04);
    let m_s = -m_t * 0.5;
    let tube_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(220_f64);
    let shell_inlet_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(270_f64);
    let clone_for_test_seven: SimpleShellAndTubeHeatExchanger = 
        sthe_one_shell_one_tube.clone();
    let test_seven = 
        thread::spawn(
            move || {
                test_thread(
                    clone_for_test_seven,
                    tube_inlet_temperature, 
                    shell_inlet_temperature, 
                    m_t, 
                    m_s,) }
        );
    //join and unwrap all threads 

    test_one.join().unwrap();
    test_two.join().unwrap();
    test_three.join().unwrap();
    test_four.join().unwrap();
    test_five.join().unwrap();
    test_six.join().unwrap();
    test_seven.join().unwrap();




    // note: use a panic to reveal the dbg! information 
    //todo!();


}
