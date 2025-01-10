/// Transient Validation test using the experimental data,
/// 
/// we are specifically going for the transient at 4050s
///
/// the outlet temp at 4100s after power step from 2530 kW to 6080 kW 
/// is 97.31 degC approx
///
/// this is 50s in after transient. 
///
/// If the temperatures 50s after the transient match, the test passes.
///
///
///
///
/// On page 46 and 47 of Zweibaum's thesis, the following transient was done 
/// power step transient at the following approximate times:
///
/// Note: these are approximate experimental datapoints using graphreader.com 
/// to trace out experimental data
/// Time(s), Heater power (W)
/// 3052.544,2528.239
/// 3282.08,2518.272
/// 3365.044,4232.558
/// 3984.513,4242.525
/// 4050.885,6076.412
/// 4664.823,6086.379
/// 4709.071,7093.023
/// 5372.788,7093.023
/// 5405.973,7611.296
/// 6030.973,7601.329
/// 6069.69,7162.791
/// 6871.681,7152.824
/// 6888.274,5438.538
/// 7529.867,5438.538
/// 7551.991,2707.641
/// 7975.111,2707.641
///
/// Based on fig 2-21
/// The resulting approx temperatures for the transient at about 
/// 4050s were (power going from 4.24 kW to 6.08 kW) :
///
///
/// Time (s),Heater Outlet Temp (degC)
/// 3901.87,92.889
/// 3905.403,92.705
/// 3913.716,92.889
/// 3918.703,92.766
/// 3927.847,92.828
/// 3943.641,92.643
/// 3960.266,92.582
/// 3964.007,93.258
/// 3975.229,92.52
/// 3978.554,93.996
/// 3984.788,92.643
/// 3986.866,93.566
/// 3997.257,92.582
/// 4003.076,93.012
/// 4009.726,92.336
/// 4013.051,93.381
/// 4021.363,92.336
/// 4034.248,92.398
/// 4036.741,93.627
/// 4040.898,93.381
/// 4045.885,92.582
/// 4049.626,92.459
/// 4058.77,93.811
/// 4067.498,95.102
/// 4072.901,96.025
/// 4083.292,96.824
/// 4099.917,97.316
/// 4109.476,97.746
/// 4117.789,97.746
/// 4131.92,98.484
/// 4140.648,98.668
/// 4156.442,98.422
/// 4162.677,98.73
/// 4165.586,99.406
/// 4168.495,98.607
/// 4173.483,99.16
/// 4176.392,98.361
/// 4188.03,98.545
/// 4203.408,98.914
/// 4212.552,98.852
/// 4219.618,98.791
/// 4234.996,98.422
/// 4246.218,98.484
/// 4252.037,98.484
/// 4254.115,99.713
/// 4258.687,98.545
/// 4265.337,98.607
/// 4266.584,99.775
/// 4271.155,98.422
/// 4276.143,98.545
/// 4277.805,99.221
/// 4281.962,98.422
/// 4287.365,99.037
/// 4292.352,98.791
/// 4302.743,98.668
/// 4305.653,99.467
/// 4312.303,98.545
/// 4315.212,99.283
/// 4319.368,98.484
/// 4322.278,99.283
/// 4328.096,98.668
/// 4336.825,98.422
/// 4346.8,98.484
/// 4352.618,99.098
/// 4359.268,98.484
/// 4370.49,98.238
/// 4377.14,98.607
/// 4386.7,98.422
/// 4393.973,98.484

/// Time(s), Heater Inlet Temp (degC)
/// 3906.858,79.057
/// 3950.707,79.119
/// 3986.035,78.934
/// 4058.77,78.996
/// 4111.554,79.119
/// 4142.311,79.611
/// 4180.964,79.488
/// 4204.655,79.365
/// 4233.333,79.242
/// 4257.855,79.426
/// 4294.846,79.242
/// 4321.862,79.18
/// 4363.009,79.18
/// 4383.998,78.996
///
/// I'm testing temperatures 50s into the transient. Which should be 
/// about 97.316 degC (+/- 0.5 degC of thermocouple uncertainty)
///
/// 
#[test]
pub fn transient_test_stepup_4050s_fig_2_21_zweibaum_thesis_for_heater_v1_eight_nodes_validation(){
    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use crate::prelude::beta_testing::*;

    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use core::time;
    use std::{thread::{self}, time::SystemTime};
    use uom::{si::{time::second, power::kilowatt}, ConstZero};
    use uom::si::mass_rate::kilogram_per_second;
    // construct structs



    // heater v1 example
    let mut heater_power = Power::new::<kilowatt>(4.24);
    let initial_temperature: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(79.067);
    let final_experimental_outlet_temp =
        ThermodynamicTemperature::new::<degree_celsius>(97.316);
    let inlet_temperature = initial_temperature;
    let ambient_air_temp: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(21.76);

    let number_of_inner_temperature_nodes: usize = 10-2;
    
    let mut heater_v1 = InsulatedPorousMediaFluidComponent::new_ciet_heater_v1_with_annular_pipe(
        initial_temperature,
        ambient_air_temp,
        number_of_inner_temperature_nodes
    );


    let mut heater_top_head
        = InsulatedPorousMediaFluidComponent::new_ciet_heater_v1_top_head(
            initial_temperature, 
            ambient_air_temp, 
            0);

    let mut heater_bottom_head 
        = InsulatedPorousMediaFluidComponent::new_ciet_heater_v1_bottom_head(
            initial_temperature, 
            ambient_air_temp, 
            0);
    // note: mx10 potentially has a memory leak
    let mut static_mixer_mx_10_object: InsulatedPorousMediaFluidComponent 
    = InsulatedPorousMediaFluidComponent::new_static_mixer_2_mx10(
        initial_temperature,
        ambient_air_temp);

    let mut static_mixer_mx_10_pipe: InsulatedPorousMediaFluidComponent 
    = InsulatedPorousMediaFluidComponent::new_static_mixer_pipe_2a_mx10(
        initial_temperature,
        ambient_air_temp);

    // heat transfer coeff calibrated to 6.0 W/(m^2 K) 
    let htc_calibrated = HeatTransfer::new::<watt_per_square_meter_kelvin>(6.0);

    heater_v1.heat_transfer_to_ambient = htc_calibrated;
    heater_top_head.heat_transfer_to_ambient = htc_calibrated;
    heater_bottom_head.heat_transfer_to_ambient = htc_calibrated;
    static_mixer_mx_10_object.heat_transfer_to_ambient = htc_calibrated;
    static_mixer_mx_10_pipe.heat_transfer_to_ambient = htc_calibrated;

    //let struct_support_equiv_diameter: Length = Length::new::<inch>(0.5);
    //let struc_support_equiv_length: Length = Length::new::<foot>(1.0);

    //let mut structural_support_heater_bottom_head: HeatTransferEntity 
    //= SingleCVNode::new_cylinder(
    //    struc_support_equiv_length,
    //    struct_support_equiv_diameter,
    //    SolidMaterial::SteelSS304L.into(),
    //    initial_temperature,
    //    Pressure::new::<atmosphere>(1.0),
    //).unwrap();

    //let mut structural_support_heater_top_head: HeatTransferEntity = 
    //structural_support_heater_bottom_head.clone();

    //let approx_support_conductance: ThermalConductance = {

    //    // for conductance, it is the half length that counts 
    //    //
    //    // bc -------- (support cv) ------------- heater head

    //    let conductivity = SolidMaterial::SteelSS304L.try_get_thermal_conductivity(
    //        initial_temperature
    //    ).unwrap();

    //    let xs_area_support = PI * 0.25 * struct_support_equiv_diameter 
    //    * struct_support_equiv_diameter;
    //    

    //    0.5 * conductivity * xs_area_support / struc_support_equiv_length

    //};

    //let support_conductance_interaction = HeatTransferInteractionType::
    //    UserSpecifiedThermalConductance(approx_support_conductance);


    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    //let mut ambient_air_temp_bc: HeatTransferEntity = 
    //inlet_bc.clone();

    // time settings 

    let max_time = Time::new::<second>(350.0);
    let timestep = Time::new::<second>(0.1);
    let mut simulation_time = Time::ZERO;
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);

    let mut final_outlet_temp = ThermodynamicTemperature::ZERO;

    let loop_time = SystemTime::now();
    // main loop
    // note: possible memory leak
    
    let main_loop = thread::spawn( move || {
        while max_time > simulation_time {

            // time start 
            let loop_time_start = loop_time.elapsed().unwrap();
            // create interactions 


            // let's get heater temperatures for post processing
            // as well as the interaction
            // for simplicity, i use the boussineseq approximation,
            // which assumes that heat transfer is governed by 
            // average density (which doesn't change much for liquid 
            // anyway)

            let connect_static_mixer_10 = true; 

            let mut therminol_array_clone: FluidArray 
            = heater_v1.pipe_fluid_array.clone().try_into().unwrap();

            let _therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            therminol_array_clone.get_temperature_vector().unwrap();

            let heater_surface_array_clone: SolidColumn 
            = heater_v1.pipe_shell.clone().try_into().unwrap();

            let heater_surface_array_temp: Vec<ThermodynamicTemperature> = 
            heater_surface_array_clone.get_temperature_vector().unwrap();

            let heater_fluid_bulk_temp: ThermodynamicTemperature = 
            therminol_array_clone.try_get_bulk_temperature().unwrap();

            let heater_top_head_bare_therminol_clone: FluidArray = 
            heater_top_head.pipe_fluid_array.clone().try_into().unwrap();

            let heater_top_head_exit_temperature: ThermodynamicTemperature = 
            heater_top_head_bare_therminol_clone.get_temperature_vector()
                .unwrap().into_iter().last().unwrap();

            if connect_static_mixer_10 {
                let static_mixer_therminol_clone: FluidArray = 
                static_mixer_mx_10_object.pipe_fluid_array.clone().try_into().unwrap();

                let _static_mixer_exit_temperature: ThermodynamicTemperature
                = static_mixer_therminol_clone.get_temperature_vector().unwrap()
                    .into_iter().last().unwrap();

                let static_mixer_pipe_therminol_clone: FluidArray = 
                static_mixer_mx_10_pipe.pipe_fluid_array.clone().try_into().unwrap();

                let bt_12_temperature: ThermodynamicTemperature = 
                static_mixer_pipe_therminol_clone.get_temperature_vector().unwrap() 
                    .into_iter().last().unwrap();

                // bt_12_temperature, which is actually the output temperature of static 
                // mixer 10
                dbg!(bt_12_temperature
                .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));
            }

            let heater_therminol_avg_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                heater_fluid_bulk_temp).unwrap();

            let generic_advection_interaction = 
            HeatTransferInteractionType::new_advection_interaction(
                mass_flowrate,
                heater_therminol_avg_density,
                heater_therminol_avg_density,
            );
            // all unused values to try and mitigate memory leaking
            {
                // prints therminol temperature 

                // print outlet temperature 
                dbg!(heater_top_head_exit_temperature
                .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));

                // print surface temperature 
                dbg!(heater_surface_array_temp);

                //// print therminol temperature 
                //dbg!("Therminol Array Temp: ", therminol_array_temperature);

                //// print twisted tape temperature 
                //dbg!("twisted tape Temp: 
                //note: conduction occurs, so first node is hotter\n 
                //than the therminol fluid", twisted_tape_temperature);

                // print loop time 
                // dbg diagnostics probably not the cause of mem leaks
                //println!("{:?}",time_taken_for_calculation_loop.as_micros());
            }

            // make axial connections to BCs 
            //
            // note: need to speed up this part, too slow

            heater_bottom_head.pipe_fluid_array.link_to_back(
                &mut inlet_bc,
                generic_advection_interaction
            ).unwrap();

            heater_v1.pipe_fluid_array.link_to_back(
                &mut heater_bottom_head.pipe_fluid_array,
                generic_advection_interaction
            ).unwrap();

            heater_v1.pipe_fluid_array.link_to_front(
                &mut heater_top_head.pipe_fluid_array,
                generic_advection_interaction
            ).unwrap();

            
            if connect_static_mixer_10 {
                heater_top_head.pipe_fluid_array.link_to_front(
                    &mut static_mixer_mx_10_object.pipe_fluid_array,
                    generic_advection_interaction
                ).unwrap();

                static_mixer_mx_10_object.pipe_fluid_array.link_to_front(
                    &mut static_mixer_mx_10_pipe.pipe_fluid_array,
                    generic_advection_interaction
                ).unwrap();

                static_mixer_mx_10_pipe.pipe_fluid_array.link_to_front(
                    &mut outlet_bc,
                    generic_advection_interaction
                ).unwrap();

            } else {

                heater_top_head.pipe_fluid_array.link_to_front(
                    &mut outlet_bc,
                    generic_advection_interaction
                ).unwrap();
            }
            
            //// and axial connections for heater top and bottom heads 
            //// to support 
            ////
            //// parallelise this

            //heater_bottom_head_bare.steel_shell.link_to_back(
            //    &mut structural_support_heater_bottom_head,
            //    support_conductance_interaction
            //).unwrap();

            //heater_top_head_bare.steel_shell.link_to_front(
            //    &mut structural_support_heater_top_head,
            //    support_conductance_interaction
            //).unwrap();

            //// link the top and bottom head support to the environment 
            //// parallelise this
            //
            //plus potential memory leak here

            //structural_support_heater_bottom_head.link_to_front(
            //    &mut ambient_air_temp_bc,
            //    support_conductance_interaction
            //).unwrap();
            //structural_support_heater_top_head.link_to_front(
            //    &mut ambient_air_temp_bc,
            //    support_conductance_interaction
            //).unwrap();


            // make other connections
            //
            // this is the serial version
            //heater_v2_bare.lateral_and_miscellaneous_connections(
            //    mass_flowrate,
            //    heater_power
            //);
            let wait: bool = false;

            // parallel calc probably not the cause of memory leak
            if wait {

                let ten_millis = time::Duration::from_millis(10);

                thread::sleep(ten_millis);

            } else {
                let porous_media_side_steady_state_power = Power::ZERO;
                let heater_top_bottom_head_power = Power::ZERO;
                let prandtl_wall_correction_setting = true;

                // 
                if simulation_time > Time::new::<second>(300.0) {
                    heater_power = Power::new::<kilowatt>(6.08);
                }


                // make other connections by spawning a new thread 
                // this is the parallel version
                heater_v1.lateral_and_miscellaneous_connections(
                        prandtl_wall_correction_setting,
                        mass_flowrate,
                        heater_power,
                        porous_media_side_steady_state_power).unwrap();

                heater_bottom_head.lateral_and_miscellaneous_connections(
                        prandtl_wall_correction_setting,
                        mass_flowrate,
                        heater_top_bottom_head_power,
                        heater_top_bottom_head_power).unwrap();

                heater_top_head.lateral_and_miscellaneous_connections(
                        prandtl_wall_correction_setting,
                        mass_flowrate,
                        heater_top_bottom_head_power,
                        heater_top_bottom_head_power).unwrap();


                static_mixer_mx_10_object.lateral_and_miscellaneous_connections_mx10(
                    mass_flowrate);

                static_mixer_mx_10_pipe.lateral_and_miscellaneous_connections_mx10(
                    mass_flowrate);


                //// calculate timestep (serial method)
                //heater_v2_bare.advance_timestep(
                //    timestep);

                // calculate timestep (thread spawn method, parallel) 


                heater_v1.advance_timestep(timestep);
                heater_top_head.advance_timestep(timestep);
                heater_bottom_head.advance_timestep(timestep);
                static_mixer_mx_10_pipe.advance_timestep(timestep);
                static_mixer_mx_10_object.advance_timestep(timestep);



            } 


            // for outlet temperature, we use static mixer mx10 pipe 
            // temperature 

            final_outlet_temp = 
                static_mixer_mx_10_pipe
                .pipe_fluid_array
                .try_get_bulk_temperature()
                .unwrap();

            simulation_time += timestep;

            let time_taken_for_calculation_loop = loop_time.elapsed().unwrap()
            - loop_time_start;

            dbg!(time_taken_for_calculation_loop);

        }
        // assert final temp to within 0.5 degC of thermocouple error
        approx::assert_abs_diff_eq!(
            final_experimental_outlet_temp.get::<degree_celsius>(),
            final_outlet_temp.get::<degree_celsius>(),
            epsilon=0.5);

    });

    main_loop.join().unwrap();



}

/// Transient Validation test using the experimental data,
/// 
/// we are specifically going for the transient at about 6890s
///
/// the outlet temp at 6920 after power step from 7.15 kW to 5.44 kW
/// was about 97.5 degC
///
/// this is 30s in after transient. 
///
/// If the temperatures 30s after the transient match, the test passes.
///
///
///
///
/// On page 46 and 47 of Zweibaum's thesis, the following transient was done 
/// power step transient at the following approximate times:
///
/// Note: these are approximate experimental datapoints using graphreader.com 
/// to trace out experimental data
/// Time(s), Heater power (W)
/// 3052.544,2528.239
/// 3282.08,2518.272
/// 3365.044,4232.558
/// 3984.513,4242.525
/// 4050.885,6076.412
/// 4664.823,6086.379
/// 4709.071,7093.023
/// 5372.788,7093.023
/// 5405.973,7611.296
/// 6030.973,7601.329
/// 6069.69,7162.791
/// 6871.681,7152.824
/// 6888.274,5438.538
/// 7529.867,5438.538
/// 7551.991,2707.641
/// 7975.111,2707.641
///
/// Based on fig 2-21
/// The resulting approx temperatures for the transient at about 
/// 4050s were (power going from 4.24 kW to 6.08 kW) :
/// step down test
///
/// Time(s), Heater outlet temp degC
/// 6756.469,101.722
/// 6759.182,102.835
/// 6764.19,101.474
/// 6769.616,101.66
/// 6771.703,103.021
/// 6776.294,101.598
/// 6782.972,101.536
/// 6793.406,101.845
/// 6801.336,101.598
/// 6805.927,102.711
/// 6810.935,101.784
/// 6811.77,103.021
/// 6816.361,101.784
/// 6824.29,101.784
/// 6829.299,101.289
/// 6831.803,102.464
/// 6838.481,101.165
/// 6846.411,101.536
/// 6850.584,102.464
/// 6855.593,101.351
/// 6857.262,103.763
/// 6860.601,101.722
/// 6863.94,102.649
/// 6869.366,101.289
/// 6876.461,101.536
/// 6878.13,102.835
/// 6884.391,101.598
/// 6891.486,102.773
/// 6894.407,101.474
/// 6900.25,102.773
/// 6901.085,100.856
/// 6904.841,101.969
/// 6908.18,99.433
/// 6913.606,98.629
/// 6923.205,97.33
/// 6931.97,96.773
/// 6940.317,96.155
/// 6944.491,96.526
/// 6946.995,98.753
/// 6950.751,96.959
/// 6952.838,98.196
/// 6957.012,96.093
/// 6960.351,97.763
/// 6963.272,98.443
/// 6964.524,95.598
/// 6974.124,95.536
/// 6980.384,98.134
/// 6984.14,95.722
/// 6985.81,98.443
/// 6990.401,95.784
/// 6996.244,96.402
/// 6998.748,94.546
/// 7006.678,94.361
/// 7010.017,95.474
/// 7015.86,94.918
/// 7021.285,97.454
/// 7026.711,94.423
/// 7030.885,96.588
/// 7035.058,94.979
/// 7041.319,95.474
/// 7050.918,94.423
/// 7060.518,94.794
/// 7063.022,97.082
/// 7067.613,94.794
/// 7070.952,96.216
/// 7075.96,94.856
/// 7083.472,95.289
/// 7090.985,94.485
/// 7101.836,95.412
/// 7109.349,95.598
/// 7115.192,94.361
/// 7117.696,96.402
/// 7121.452,94.361
/// 7126.461,96.835
/// 7130.217,95.103
/// 7138.982,95.103
/// 7140.651,96.34
/// 7145.242,98.381
/// 7150.25,95.412
/// 7157.346,94.67
/// 7164.441,96.588
/// 7166.528,98.567
/// 7168.197,96.588
/// 7174.875,95.227
/// 7179.466,96.711
/// 7186.561,95.227
/// 7194.073,96.835
/// 7199.082,95.784
/// 7202.838,94.67
/// 7213.689,94.979
/// 7215.776,98.691
/// 7221.202,95.784
/// 7230.384,94.979
/// 7235.81,98.258
/// 7243.948,96.464
///
/// Time(s), Heater inlet temp degC
/// 6756.886,79.082
/// 6782.137,79.144
/// 6810.1,79.144
/// 6867.696,78.897
/// 6898.581,78.835
/// 6937.396,79.206
/// 6973.706,78.897
/// 7021.703,77.784
/// 7045.91,77.784
/// 7087.646,77.907
/// 7132.304,78.526
/// 7162.354,78.588
/// 7197.412,78.773
/// 7225.793,78.835
/// 7245.618,78.959

/// On page 46 and 47 of Zweibaum's thesis, the following transient was done 
/// power step transient at the following approximate times:
///
/// Note: these are approximate experimental datapoints using graphreader.com 
/// to trace out experimental data
#[test]
pub fn transient_test_stepdown_4050s_fig_2_21_zweibaum_thesis_for_heater_v1_eight_nodes_validation(){
    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use crate::prelude::beta_testing::*;

    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use core::time;
    use std::{thread::{self}, time::SystemTime};
    use uom::{si::{time::second, power::kilowatt}, ConstZero};
    use uom::si::mass_rate::kilogram_per_second;
    // construct structs



    // heater v1 example
    let mut heater_power = Power::new::<kilowatt>(7.15);
    let initial_temperature: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(78.9);
    let final_experimental_outlet_temp =
        ThermodynamicTemperature::new::<degree_celsius>(97.5);
    let inlet_temperature = initial_temperature;
    let ambient_air_temp: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(21.76);

    let number_of_inner_temperature_nodes: usize = 10-2;
    
    let mut heater_v1 = InsulatedPorousMediaFluidComponent::new_ciet_heater_v1_with_annular_pipe(
        initial_temperature,
        ambient_air_temp,
        number_of_inner_temperature_nodes
    );


    let mut heater_top_head
        = InsulatedPorousMediaFluidComponent::new_ciet_heater_v1_top_head(
            initial_temperature, 
            ambient_air_temp, 
            0);

    let mut heater_bottom_head 
        = InsulatedPorousMediaFluidComponent::new_ciet_heater_v1_bottom_head(
            initial_temperature, 
            ambient_air_temp, 
            0);
    // note: mx10 potentially has a memory leak
    let mut static_mixer_mx_10_object: InsulatedPorousMediaFluidComponent 
    = InsulatedPorousMediaFluidComponent::new_static_mixer_2_mx10(
        initial_temperature,
        ambient_air_temp);

    let mut static_mixer_mx_10_pipe: InsulatedPorousMediaFluidComponent 
    = InsulatedPorousMediaFluidComponent::new_static_mixer_pipe_2a_mx10(
        initial_temperature,
        ambient_air_temp);

    // heat transfer coeff calibrated to 6.0 W/(m^2 K) 
    let htc_calibrated = HeatTransfer::new::<watt_per_square_meter_kelvin>(6.0);

    heater_v1.heat_transfer_to_ambient = htc_calibrated;
    heater_top_head.heat_transfer_to_ambient = htc_calibrated;
    heater_bottom_head.heat_transfer_to_ambient = htc_calibrated;
    static_mixer_mx_10_object.heat_transfer_to_ambient = htc_calibrated;
    static_mixer_mx_10_pipe.heat_transfer_to_ambient = htc_calibrated;

    //let struct_support_equiv_diameter: Length = Length::new::<inch>(0.5);
    //let struc_support_equiv_length: Length = Length::new::<foot>(1.0);

    //let mut structural_support_heater_bottom_head: HeatTransferEntity 
    //= SingleCVNode::new_cylinder(
    //    struc_support_equiv_length,
    //    struct_support_equiv_diameter,
    //    SolidMaterial::SteelSS304L.into(),
    //    initial_temperature,
    //    Pressure::new::<atmosphere>(1.0),
    //).unwrap();

    //let mut structural_support_heater_top_head: HeatTransferEntity = 
    //structural_support_heater_bottom_head.clone();

    //let approx_support_conductance: ThermalConductance = {

    //    // for conductance, it is the half length that counts 
    //    //
    //    // bc -------- (support cv) ------------- heater head

    //    let conductivity = SolidMaterial::SteelSS304L.try_get_thermal_conductivity(
    //        initial_temperature
    //    ).unwrap();

    //    let xs_area_support = PI * 0.25 * struct_support_equiv_diameter 
    //    * struct_support_equiv_diameter;
    //    

    //    0.5 * conductivity * xs_area_support / struc_support_equiv_length

    //};

    //let support_conductance_interaction = HeatTransferInteractionType::
    //    UserSpecifiedThermalConductance(approx_support_conductance);


    let mut inlet_bc: HeatTransferEntity = BCType::new_const_temperature( 
        inlet_temperature).into();

    let mut outlet_bc: HeatTransferEntity = BCType::new_adiabatic_bc().into();

    //let mut ambient_air_temp_bc: HeatTransferEntity = 
    //inlet_bc.clone();

    // time settings 

    let max_time = Time::new::<second>(330.0);
    let timestep = Time::new::<second>(0.1);
    let mut simulation_time = Time::ZERO;
    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);

    let mut final_outlet_temp = ThermodynamicTemperature::ZERO;

    let loop_time = SystemTime::now();
    // main loop
    // note: possible memory leak
    
    let main_loop = thread::spawn( move || {
        while max_time > simulation_time {

            // time start 
            let loop_time_start = loop_time.elapsed().unwrap();
            // create interactions 


            // let's get heater temperatures for post processing
            // as well as the interaction
            // for simplicity, i use the boussineseq approximation,
            // which assumes that heat transfer is governed by 
            // average density (which doesn't change much for liquid 
            // anyway)

            let connect_static_mixer_10 = true; 

            let mut therminol_array_clone: FluidArray 
            = heater_v1.pipe_fluid_array.clone().try_into().unwrap();

            let _therminol_array_temperature: Vec<ThermodynamicTemperature> = 
            therminol_array_clone.get_temperature_vector().unwrap();

            let heater_surface_array_clone: SolidColumn 
            = heater_v1.pipe_shell.clone().try_into().unwrap();

            let heater_surface_array_temp: Vec<ThermodynamicTemperature> = 
            heater_surface_array_clone.get_temperature_vector().unwrap();

            let heater_fluid_bulk_temp: ThermodynamicTemperature = 
            therminol_array_clone.try_get_bulk_temperature().unwrap();

            let heater_top_head_bare_therminol_clone: FluidArray = 
            heater_top_head.pipe_fluid_array.clone().try_into().unwrap();

            let heater_top_head_exit_temperature: ThermodynamicTemperature = 
            heater_top_head_bare_therminol_clone.get_temperature_vector()
                .unwrap().into_iter().last().unwrap();

            if connect_static_mixer_10 {
                let static_mixer_therminol_clone: FluidArray = 
                static_mixer_mx_10_object.pipe_fluid_array.clone().try_into().unwrap();

                let _static_mixer_exit_temperature: ThermodynamicTemperature
                = static_mixer_therminol_clone.get_temperature_vector().unwrap()
                    .into_iter().last().unwrap();

                let static_mixer_pipe_therminol_clone: FluidArray = 
                static_mixer_mx_10_pipe.pipe_fluid_array.clone().try_into().unwrap();

                let bt_12_temperature: ThermodynamicTemperature = 
                static_mixer_pipe_therminol_clone.get_temperature_vector().unwrap() 
                    .into_iter().last().unwrap();

                // bt_12_temperature, which is actually the output temperature of static 
                // mixer 10
                dbg!(bt_12_temperature
                .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));
            }

            let heater_therminol_avg_density: MassDensity = 
            LiquidMaterial::TherminolVP1.try_get_density(
                heater_fluid_bulk_temp).unwrap();

            let generic_advection_interaction = 
            HeatTransferInteractionType::new_advection_interaction(
                mass_flowrate,
                heater_therminol_avg_density,
                heater_therminol_avg_density,
            );
            // all unused values to try and mitigate memory leaking
            {
                // prints therminol temperature 

                // print outlet temperature 
                dbg!(heater_top_head_exit_temperature
                .into_format_args(degree_celsius,uom::fmt::DisplayStyle::Abbreviation));

                // print surface temperature 
                dbg!(heater_surface_array_temp);

                //// print therminol temperature 
                //dbg!("Therminol Array Temp: ", therminol_array_temperature);

                //// print twisted tape temperature 
                //dbg!("twisted tape Temp: 
                //note: conduction occurs, so first node is hotter\n 
                //than the therminol fluid", twisted_tape_temperature);

                // print loop time 
                // dbg diagnostics probably not the cause of mem leaks
                //println!("{:?}",time_taken_for_calculation_loop.as_micros());
            }

            // make axial connections to BCs 
            //
            // note: need to speed up this part, too slow

            heater_bottom_head.pipe_fluid_array.link_to_back(
                &mut inlet_bc,
                generic_advection_interaction
            ).unwrap();

            heater_v1.pipe_fluid_array.link_to_back(
                &mut heater_bottom_head.pipe_fluid_array,
                generic_advection_interaction
            ).unwrap();

            heater_v1.pipe_fluid_array.link_to_front(
                &mut heater_top_head.pipe_fluid_array,
                generic_advection_interaction
            ).unwrap();

            
            if connect_static_mixer_10 {
                heater_top_head.pipe_fluid_array.link_to_front(
                    &mut static_mixer_mx_10_object.pipe_fluid_array,
                    generic_advection_interaction
                ).unwrap();

                static_mixer_mx_10_object.pipe_fluid_array.link_to_front(
                    &mut static_mixer_mx_10_pipe.pipe_fluid_array,
                    generic_advection_interaction
                ).unwrap();

                static_mixer_mx_10_pipe.pipe_fluid_array.link_to_front(
                    &mut outlet_bc,
                    generic_advection_interaction
                ).unwrap();

            } else {

                heater_top_head.pipe_fluid_array.link_to_front(
                    &mut outlet_bc,
                    generic_advection_interaction
                ).unwrap();
            }
            
            //// and axial connections for heater top and bottom heads 
            //// to support 
            ////
            //// parallelise this

            //heater_bottom_head_bare.steel_shell.link_to_back(
            //    &mut structural_support_heater_bottom_head,
            //    support_conductance_interaction
            //).unwrap();

            //heater_top_head_bare.steel_shell.link_to_front(
            //    &mut structural_support_heater_top_head,
            //    support_conductance_interaction
            //).unwrap();

            //// link the top and bottom head support to the environment 
            //// parallelise this
            //
            //plus potential memory leak here

            //structural_support_heater_bottom_head.link_to_front(
            //    &mut ambient_air_temp_bc,
            //    support_conductance_interaction
            //).unwrap();
            //structural_support_heater_top_head.link_to_front(
            //    &mut ambient_air_temp_bc,
            //    support_conductance_interaction
            //).unwrap();


            // make other connections
            //
            // this is the serial version
            //heater_v2_bare.lateral_and_miscellaneous_connections(
            //    mass_flowrate,
            //    heater_power
            //);
            let wait: bool = false;

            // parallel calc probably not the cause of memory leak
            if wait {

                let ten_millis = time::Duration::from_millis(10);

                thread::sleep(ten_millis);

            } else {
                let porous_media_side_steady_state_power = Power::ZERO;
                let heater_top_bottom_head_power = Power::ZERO;
                let prandtl_wall_correction_setting = true;

                // 
                if simulation_time > Time::new::<second>(300.0) {
                    heater_power = Power::new::<kilowatt>(5.44);
                }


                // make other connections by spawning a new thread 
                // this is the parallel version
                heater_v1.lateral_and_miscellaneous_connections(
                        prandtl_wall_correction_setting,
                        mass_flowrate,
                        heater_power,
                        porous_media_side_steady_state_power).unwrap();

                heater_bottom_head.lateral_and_miscellaneous_connections(
                        prandtl_wall_correction_setting,
                        mass_flowrate,
                        heater_top_bottom_head_power,
                        heater_top_bottom_head_power).unwrap();

                heater_top_head.lateral_and_miscellaneous_connections(
                        prandtl_wall_correction_setting,
                        mass_flowrate,
                        heater_top_bottom_head_power,
                        heater_top_bottom_head_power).unwrap();


                static_mixer_mx_10_object.lateral_and_miscellaneous_connections_mx10(
                    mass_flowrate);

                static_mixer_mx_10_pipe.lateral_and_miscellaneous_connections_mx10(
                    mass_flowrate);


                //// calculate timestep (serial method)
                //heater_v2_bare.advance_timestep(
                //    timestep);

                // calculate timestep (thread spawn method, parallel) 


                heater_v1.advance_timestep(timestep);
                heater_top_head.advance_timestep(timestep);
                heater_bottom_head.advance_timestep(timestep);
                static_mixer_mx_10_pipe.advance_timestep(timestep);
                static_mixer_mx_10_object.advance_timestep(timestep);



            } 


            // for outlet temperature, we use static mixer mx10 pipe 
            // temperature 

            final_outlet_temp = 
                static_mixer_mx_10_pipe
                .pipe_fluid_array
                .try_get_bulk_temperature()
                .unwrap();

            simulation_time += timestep;

            let time_taken_for_calculation_loop = loop_time.elapsed().unwrap()
            - loop_time_start;

            dbg!(time_taken_for_calculation_loop);

        }
        // assert final temp to within 0.5 degC of thermocouple error
        approx::assert_abs_diff_eq!(
            final_experimental_outlet_temp.get::<degree_celsius>(),
            final_outlet_temp.get::<degree_celsius>(),
            epsilon=0.5);

    });

    main_loop.join().unwrap();



}
