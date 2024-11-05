/// In the reference,
/// Britsch, K., Anderson, M., Brooks, P., & 
/// Sridharan, K. (2019). Natural circulation 
/// FLiBe loop overview. International Journal of 
/// Heat and Mass Transfer, 134, 970-983.
///
/// Heater power is given in the tables A6. from 952 watts in test 1 
/// to 1652 watts in test 10 
///
/// Now, there are four heaters, so I'm not sure if the heater power in A6 
/// refers to heat added by individual heaters or heat added by the four 
/// heaters
///
/// Hence, I needed to back calculate what is the heat addition to the flibe 
/// fluid and compare that against the heat added by the four heaters
///
/// regression numbers calculated in libreoffice calc
#[cfg(test)]
#[test]
pub fn heater_check_if_four_heater_test1(){
    use std::f64::consts::PI;

    use uom::si::f64::*;
    use uom::si::length::{inch, millimeter};
    use uom::si::power::watt;
    use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
    use uom::si::temperature_interval::degree_celsius;
    use uom::si::velocity::centimeter_per_second;

    use uom::si::mass_density::kilogram_per_cubic_meter;
    let (expt_individual_heater_power_watts, regression_flibe_heat_addition_watts, flibe_velocity_cm_per_s): 
        (f64, f64, f64) = (952.0, 2288.64525709192,2.75);

    let regression_heat_retention_fraction: f64 = 0.601009783900188;

    let salt_temp_change_degc: f64 = 59.0;

    let shell_od = Length::new::<inch>(1.0);
    let pipe_thickness = Length::new::<millimeter>(3.0);
    let shell_id = shell_od - 2.0*pipe_thickness;
    let flow_area = PI/4.0 * shell_id * shell_id;
    let flibe_speed = Velocity::new::<centimeter_per_second>(flibe_velocity_cm_per_s);

    let flibe_vol_flowrate = flow_area * flibe_speed;
    // estimated from 
    // Britsch, K., Anderson, M., Brooks, P., & 
    // Sridharan, K. (2019). Natural circulation 
    // FLiBe loop overview. International Journal of 
    // Heat and Mass Transfer, 134, 970-983.
    // table 2
    let flibe_est_density = MassDensity::new::<kilogram_per_cubic_meter>(2000.0);

    let flibe_mass_flowrate: MassRate = flibe_est_density * flibe_vol_flowrate;
    let flibe_cp = SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(2386_f64);
    let flibe_delta_temp = TemperatureInterval::new::<degree_celsius>(salt_temp_change_degc);

    let flibe_heat_added: Power = flibe_mass_flowrate * flibe_cp* flibe_delta_temp;

    let flibe_heat_added_watts = flibe_heat_added.get::<watt>();

    approx::assert_relative_eq!
        (
            flibe_heat_added_watts,
            regression_flibe_heat_addition_watts,
            max_relative=0.0001
        );

    let heat_added_by_four_heaters = expt_individual_heater_power_watts * 4.0;

    let heat_fraction_absorbed_by_flibe = flibe_heat_added_watts/heat_added_by_four_heaters;

    // if heater power in a6 refers to individual heater powers,
    // then heat absorbed by flibe should be less than the heat input by the four 
    // heaters
    let table_a6_heater_power_refers_to_individual_heater: bool = 
        heat_fraction_absorbed_by_flibe < 1.0;

    assert!(table_a6_heater_power_refers_to_individual_heater);

    approx::assert_relative_eq!
        (
            heat_fraction_absorbed_by_flibe,
            regression_heat_retention_fraction,
            max_relative=0.0001
        );

}

/// same as test 1 but for test 5
/// regression numbers calculated in libreoffice calc
#[cfg(test)]
#[test]
pub fn heater_check_if_four_heater_test5(){
    use std::f64::consts::PI;

    use uom::si::f64::*;
    use uom::si::length::{inch, millimeter};
    use uom::si::power::watt;
    use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
    use uom::si::temperature_interval::degree_celsius;
    use uom::si::velocity::centimeter_per_second;

    use uom::si::mass_density::kilogram_per_cubic_meter;
    let (expt_individual_heater_power_watts, regression_flibe_heat_addition_watts, flibe_velocity_cm_per_s): 
        (f64, f64, f64) = (1471.0, 5213.45631446023, 6.72);

    let regression_heat_retention_fraction: f64 = 0.886039482403166;

    let salt_temp_change_degc: f64 = 55.0;

    let shell_od = Length::new::<inch>(1.0);
    let pipe_thickness = Length::new::<millimeter>(3.0);
    let shell_id = shell_od - 2.0*pipe_thickness;
    let flow_area = PI/4.0 * shell_id * shell_id;
    let flibe_speed = Velocity::new::<centimeter_per_second>(flibe_velocity_cm_per_s);

    let flibe_vol_flowrate = flow_area * flibe_speed;
    // estimated from 
    // Britsch, K., Anderson, M., Brooks, P., & 
    // Sridharan, K. (2019). Natural circulation 
    // FLiBe loop overview. International Journal of 
    // Heat and Mass Transfer, 134, 970-983.
    // table 2
    let flibe_est_density = MassDensity::new::<kilogram_per_cubic_meter>(2000.0);

    let flibe_mass_flowrate: MassRate = flibe_est_density * flibe_vol_flowrate;
    let flibe_cp = SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(2386_f64);
    let flibe_delta_temp = TemperatureInterval::new::<degree_celsius>(salt_temp_change_degc);

    let flibe_heat_added: Power = flibe_mass_flowrate * flibe_cp* flibe_delta_temp;

    let flibe_heat_added_watts = flibe_heat_added.get::<watt>();

    approx::assert_relative_eq!
        (
            flibe_heat_added_watts,
            regression_flibe_heat_addition_watts,
            max_relative=0.0001
        );

    let heat_added_by_four_heaters = expt_individual_heater_power_watts * 4.0;

    let heat_fraction_absorbed_by_flibe = flibe_heat_added_watts/heat_added_by_four_heaters;

    // if heater power in a6 refers to individual heater powers,
    // then heat absorbed by flibe should be less than the heat input by the four 
    // heaters
    let table_a6_heater_power_refers_to_individual_heater: bool = 
        heat_fraction_absorbed_by_flibe < 1.0;

    assert!(table_a6_heater_power_refers_to_individual_heater);

    approx::assert_relative_eq!
        (
            heat_fraction_absorbed_by_flibe,
            regression_heat_retention_fraction,
            max_relative=0.0001
        );

}


/// same as test 1 but for test 10
/// regression numbers calculated in libreoffice calc
#[cfg(test)]
#[test]
pub fn heater_check_if_four_heater_test10(){
    use std::f64::consts::PI;

    use uom::si::f64::*;
    use uom::si::length::{inch, millimeter};
    use uom::si::power::watt;
    use uom::si::specific_heat_capacity::joule_per_kilogram_kelvin;
    use uom::si::temperature_interval::degree_celsius;
    use uom::si::velocity::centimeter_per_second;

    use uom::si::mass_density::kilogram_per_cubic_meter;
    let (expt_individual_heater_power_watts, regression_flibe_heat_addition_watts, flibe_velocity_cm_per_s): 
        (f64, f64, f64) = (1644.0, 5695.16500801763, 4.75);

    let regression_heat_retention_fraction: f64 = 0.866053072995382;

    let salt_temp_change_degc: f64 = 85.0;

    let shell_od = Length::new::<inch>(1.0);
    let pipe_thickness = Length::new::<millimeter>(3.0);
    let shell_id = shell_od - 2.0*pipe_thickness;
    let flow_area = PI/4.0 * shell_id * shell_id;
    let flibe_speed = Velocity::new::<centimeter_per_second>(flibe_velocity_cm_per_s);

    let flibe_vol_flowrate = flow_area * flibe_speed;
    // estimated from 
    // Britsch, K., Anderson, M., Brooks, P., & 
    // Sridharan, K. (2019). Natural circulation 
    // FLiBe loop overview. International Journal of 
    // Heat and Mass Transfer, 134, 970-983.
    // table 2
    let flibe_est_density = MassDensity::new::<kilogram_per_cubic_meter>(2000.0);

    let flibe_mass_flowrate: MassRate = flibe_est_density * flibe_vol_flowrate;
    let flibe_cp = SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(2386_f64);
    let flibe_delta_temp = TemperatureInterval::new::<degree_celsius>(salt_temp_change_degc);

    let flibe_heat_added: Power = flibe_mass_flowrate * flibe_cp* flibe_delta_temp;

    let flibe_heat_added_watts = flibe_heat_added.get::<watt>();

    approx::assert_relative_eq!
        (
            flibe_heat_added_watts,
            regression_flibe_heat_addition_watts,
            max_relative=0.0001
        );

    let heat_added_by_four_heaters = expt_individual_heater_power_watts * 4.0;

    let heat_fraction_absorbed_by_flibe = flibe_heat_added_watts/heat_added_by_four_heaters;

    // if heater power in a6 refers to individual heater powers,
    // then heat absorbed by flibe should be less than the heat input by the four 
    // heaters
    let table_a6_heater_power_refers_to_individual_heater: bool = 
        heat_fraction_absorbed_by_flibe < 1.0;

    assert!(table_a6_heater_power_refers_to_individual_heater);

    approx::assert_relative_eq!
        (
            heat_fraction_absorbed_by_flibe,
            regression_heat_retention_fraction,
            max_relative=0.0001
        );

}

#[cfg(test)]
#[test]
pub fn parasitic_heat_loss_calibration_dry_run_1(){
    let (tc_11_degc, tc_12_degc, tc_14_degc, tc_21_degc,
        tc_24_degc, tc_32_degc, tc_35_degc) = 
        (514.1,520.8,535.9,542.3,490.4,487.9,483.4);

    let (individual_heater_power_watts,
        _flibe_velocity_cm_per_s,
        flibe_mass_flowrate_kg_per_s,
        _flibe_density_est_kg_per_m3,
        _specific_heat_capacity_j_per_kg_k,
        _heat_added_to_fluid_watts_regression) =
        (952.0,2.75,0.0162575849026945,2000.0,2386.0,2288.64525709192);

    let max_time_seconds = 2000.0;
    let regression_temperature_tolerance_kelvin = 0.4;
    let (bottom_cross_insulation_thickness_cm, 
        riser_insulation_thickness_cm) = (0.17,0.035);

    calibrate_uw_madison_parasitic_heat_loss_fixed_flowrate(
        tc_11_degc, 
        tc_12_degc, 
        tc_14_degc, 
        tc_21_degc, 
        tc_24_degc, 
        tc_32_degc, 
        tc_35_degc, 
        individual_heater_power_watts, 
        flibe_mass_flowrate_kg_per_s, 
        max_time_seconds,
        regression_temperature_tolerance_kelvin,
        bottom_cross_insulation_thickness_cm,
        riser_insulation_thickness_cm);
}

/// calibrates parasitic heat losses for the heater given a fixed flowrate 
///
///
///
/// the data is as follows
///
/// test no.,TC 11(degC),TC 12(degC),TC 14(degC),TC 21(degC),TC 24(degC),TC 32(degC),TC 35(degC)
/// 1,(514.1,520.8,535.9,542.3,490.4,487.9,483.4)
/// 2,(576.3,580.4,592,600.9,553.1,550.4,546.7)
/// 3,(638.5,645.9,669.1,667.6,620.9,619,617.4)
/// 4,(538.6,543.8,569.9,571.4,502.1,496.8,497.4)
/// 5,(692.2,699.8,722,720.5,672.7,675.7,665.7)
/// 6,(590.7,597.6,623,626.8,562.3,560.8,560.2)
/// 7,(548.8,553.8,572.7,583.4,510.2,509.5,508.2)
/// 8,(603.9,611.5,638.7,641.4,572.6,567.2,571.3)
/// 9,(572.5,578.9,600.2,612,536.2,529.3,535.6)
/// 10,(549.9,556,587,589.8,499.2,500,504.6)
///
/// Individual	Heater Power (W)	flibe velocity cm/s	flibe mass flowrate (kg/s)	flibe density est (kg/m3)	specific heat capacity (J/ (kg K))	Total heat added to fluid (watts)
/// 1,(952.0,2.75,0.0162575849026945,2000.0,2386.0,2288.64525709192)
/// 2,(1125.0,3.48,0.020573234713228,2000.0,2386.0,2650.73785339114)
/// 3,(1298.0,5.76,0.0340522505598256,2000.0,2386.0,4062.43349178719)
/// 4,(1298.0,3.59,0.0212235381093357,2000.0,2386.0,3747.31278273675)
/// 5,(1471.0,6.72,0.0397276256531298,2000.0,2386.0,5213.45631446023)
/// 6,(1471.0,5.49,0.0324560513148338,2000.0,2386.0,5188.48927529195)
/// 7,(1471.0,5.07,0.0299730747115131,2000.0,2386.0,5363.68171962528)
/// 8,(1644.0,5.43,0.0321013403715022,2000.0,2386.0,5361.5658688483)
/// 9,(1644.0,4.31,0.0254800694293139,2000.0,2386.0,4620.45387003407)
/// 10,(1644.0,4.75,0.028081283013745,2000.0,2386.0,5695.16500801763)
///
/// This is from:
///
/// Britsch, K. R., Doniger, W., Anderson, M., & 
/// Sridharan, K. (2018). Operation Data from the UW 
/// Natural Circulation FLiBe Flow Loop. 
/// University of Wisconsin-Madison.
///
///
/// this data is used in individual tests 
/// note that the natural circulation flowrate is fixed in the parasitic 
/// heat loss calibration
/// so this is treated as if it were forced circulation
///
///
/// Debug log:
///
/// 04 nov 2024 singapore time 11:41
///
/// FLiBe temperatures reaching too low (below 459C)
///
/// Possible reasons are that the heat transfer coefficient to surroundings 
/// is too low, that the heat transfer coefficient as stipulated by the 
/// controller is too high. This can cause two problems. Firstly, the time 
/// step could be too large for a heat transfer coefficient too high. 
/// This may cause numerical instabilities which cause the fluid 
/// temperature to be too low. Or perhaps if the timestep was not the 
/// issue, the controller signal oscillations may cause such a high 
/// heat transfer coefficient during the oscillations that causes this 
/// issue.
///
/// If controller oscillations were the issue, I'm going to reduce controller 
/// gain. Or reduce the heat transfer coefficient scaling from 40 W/(m^2 K) 
/// to something lower. Maybe 10 W/(m^2 K). This is because less heat transfer 
/// coefficient is required in the FLiBe loop case, as the temperature differential 
/// is much greater here than in Dowtherm A.
///
/// Changelog:
/// - reduced heat trf coeff scaling from 40 to 10 (low temp at 351 s)
/// - reduced timestep from 0.5 to 0.1 (low temp at around 320s simulation time)
/// - temperature interval changed from 80C to 50C, this is based on table A9 rather than CIET
/// temperature differences (318s has low temperature)
/// - reduced controller gain from 1.75 to 0.75 (low temp around 321 s)
/// - increased timestep from 0.1 to 0.5, the timestep didn't seem to 
/// affect the simulation time wherein there was a low temperature,
/// but i also changed it back to 0.1s after
/// much (low temp at around 321s simulation time)
/// - Integral, reset time, or frequency changed to a lower time, from 1s to 5s
/// (low temp at 456s)
///
/// It seems controller tuning is the answer to stabilise this simulation 
/// as the simulation time wherein the low temp was gotten was at 456s now.
/// 
/// Last changes for stability:
/// - Integral, reset time, or frequency changed to a lower time, from 5s to 15s
/// (low temp at 456s), all 4000s of simulations now completed
/// - increased timestep from 0.1 to 0.5, the timestep didn't seem to 
/// affect the simulation time wherein there was a low temperature,
///
/// Changes for simulation speedup, and others:
/// - increase controller gain from 0.75 to 1.75
/// - needed to change the controller to use the correct set point
/// - realised that the same controller object was being used for both 
/// top cross and downcomer, corrected that
/// - changed integral/reset time back to 5s (temperature too low again)
/// - temperature tolerance now set to 0.4 K, 
/// - so at 4000s, the top cross and downcomer set points agree to within 0.4 K
/// - max time now 2000s, the top cross and downcomer set points agree to within 0.4 K
/// - attempted changing timestep to 1.5s and 1.0s, resulting temperatures too low, not 
/// changing timestep
///
///
/// I saw that the downcomer heat transfer coeff was 5 W/(m^2 K) so I was quite 
/// concerned that since this is the lowest allowed heat transfer coeffcient,
/// something was not functioning correctly
/// - changing back to 4000s does not appreciably change heat transfer coeff,
/// coeffs on top cross or downcomer sections, so 2000s is sufficient.
///
/// More debugging:
///
/// it seems that TC21 has a higher temperature than TC14 in 
/// test 1,2, 4, 6, 7, 8, 9 and 10. This shows that from TC21 to TC14, 
/// heat was added. Now, the high amount of noise makes it difficult (4-6 K 
/// for each measurement) makes it doubtful as to whether this is 
/// a statistically significant increase. 
///
/// However, the consistent increase across several experiments makes it 
/// seem as if heat was actually added in that zone. 
///
/// How could it be added?
///
/// In the paper, it was shown that the heaters were radiant clamshell 
/// heaters, specifically Watlow Ceramic Fiber Semi-Cylindrical heaters.
/// Page 972 on top left of pdf file. If radiant heat transfer was the 
/// dominant mode of heat transfer, perhaps modelling the radiant 
/// heater as an electrical heater is problematic.
///
/// We may need to model radiative heat transfer for this FLiBe loop in 
/// order to get the heat transfer rates right. Radiative heat transfer 
/// modelling might warrant its own paper especially within the fluid.
///
/// It is more important though to model radiative heat transfer from 
/// the heaters to TC14/TC21. The clamshell heater dimensions were given 
/// and we should use that to model the RHT. Heater exteriors with pyrogel 
/// were also shown to be fairly constant at 300C (page 972). Some thermal 
/// resistance models may be good for this aspect of modelling radiative 
/// heat transfer.
///
///
/// For radiation heat transfer, it is likely that view factors are 
/// important. Now, I can do some view factor calculations or even 
/// empirically calibrate things.
///
/// Basically, model 
/// RHT as 
/// heating element to the pipe surface, via thermal resistor and view factor
/// heating element to surroundigns via thermal resistor and view factor
/// heating element to other surface, ie (between TC14 and 21) via thermal resistor and view factor 
///
/// Dimensions may be complex and hard to replicate. So perhaps model 
/// calibration is important here.
///
/// By view factor algebra, the area times sum of all view factors adds up to one.
/// Basically, the radiative heater should be modelled as a component
/// 
///
#[cfg(test)]
pub fn calibrate_uw_madison_parasitic_heat_loss_fixed_flowrate(
    tc_11_degc_expt: f64,
    tc_12_degc: f64,
    tc_14_degc_expt: f64,
    tc_21_degc_expt: f64,
    tc_24_degc_expt: f64,
    tc_32_degc: f64,
    tc_35_degc_expt: f64,
    individual_heater_power_watts: f64,
    flibe_mass_flowrate_kg_per_s: f64,
    max_time_seconds: f64,
    regression_temperature_tolerance_kelvin: f64,
    bottom_cross_insulation_thickness_cm: f64,
    riser_insulation_thickness_cm: f64){

    use uom::si::length::centimeter;
    use uom::si::{f64::*, mass_rate::kilogram_per_second, power::watt};

    use uom::si::{ratio::ratio, time::millisecond};

    use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::pre_built_components::uw_madison_flibe_loop_components::flibe_loop_iteration_1::components::*;
    use crate::pre_built_components::uw_madison_flibe_loop_components::flibe_loop_iteration_1::thermal_hydraulics_calculations::{uw_madison_flibe_loop_advance_timestep, uw_madison_flibe_loop_iteration_1_temperature_diagnostics, uw_madison_flibe_loop_link_up_components};
    use crate::prelude::beta_testing::FluidArray;
    use uom::ConstZero;

    use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::time::second;
    use chem_eng_real_time_process_control_simulator::alpha_nightly::transfer_fn_wrapper_and_enums::TransferFnTraits;
    use chem_eng_real_time_process_control_simulator::alpha_nightly::controllers::ProportionalController;
    use chem_eng_real_time_process_control_simulator::alpha_nightly::controllers::AnalogController;

    let timestep = Time::new::<second>(0.5);
    let input_power_per_heater = Power::new::<watt>(individual_heater_power_watts);
    let input_power_per_two_heaters = input_power_per_heater * 2.0;
    let mut current_simulation_time = Time::ZERO;
    let max_simulation_time = Time::new::<second>(max_time_seconds);
    
    // horizontal-ish cooler (top cross) exit temp
    let top_cross_exit_temp_set_point = 
        ThermodynamicTemperature::new::<degree_celsius>(
            tc_24_degc_expt);

    // vertical cooler (downcomer) exit temp
    let downcomer_exit_temp_set_point = 
        ThermodynamicTemperature::new::<degree_celsius>(
            tc_35_degc_expt);

    let _downcomer_rough_midpoint_temp_expt = 
        ThermodynamicTemperature::new::<degree_celsius>(
            tc_32_degc);

    // horizontal-ish heater (bottom cross) exit temp 
    let _bottom_cross_exit_temp_expt = 
        ThermodynamicTemperature::new::<degree_celsius>(
            tc_11_degc_expt);

    // vertical heater (riser) exit temp
    let _riser_exit_temp_expt = 
        ThermodynamicTemperature::new::<degree_celsius>(
            tc_14_degc_expt);

    let _riser_midpoint_temp_expt = 
        ThermodynamicTemperature::new::<degree_celsius>(
            tc_12_degc);

    // horizontal-ish cooler (top cross) entrance temp
    let _top_cross_entrance_temp_expt = 
        ThermodynamicTemperature::new::<degree_celsius>(
            tc_21_degc_expt);

    // heater insulation thickness 
    let bottom_cross_insulation_thickness = 
        Length::new::<centimeter>(bottom_cross_insulation_thickness_cm);
    let riser_insulation_thickness = 
        Length::new::<centimeter>(riser_insulation_thickness_cm);

    // other settings
    let (mass_flowrate_clockwise, 
        hot_leg_diagonal_heater_power, 
        hot_leg_vertical_heater_power) =
        (
            MassRate::new::<kilogram_per_second>(flibe_mass_flowrate_kg_per_s),
            input_power_per_two_heaters,
            input_power_per_two_heaters,
        );


    let reference_cooler_htc = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(10.0);
    let average_temperature_for_density_calcs = 
        ThermodynamicTemperature::new::<degree_celsius>(
            0.5*(tc_21_degc_expt+tc_35_degc_expt));


    // PID controller settings
    let controller_gain = Ratio::new::<ratio>(1.75);
    let integral_time: Time = Time::new::<second>(15.0);
    let derivative_time: Time = Time::new::<second>(1.0);
    // derivative time ratio
    let alpha: Ratio = Ratio::new::<ratio>(1.0);

    let mut pid_controller_top_cross: AnalogController = 
        AnalogController::new_filtered_pid_controller(controller_gain,
            integral_time,
            derivative_time,
            alpha).unwrap();

    let mut pid_controller_downcomer: AnalogController = 
        pid_controller_top_cross.clone();

    // we also have a measurement delay of 0.0001 s 
    // or 0.1 ms
    let measurement_delay = Time::new::<millisecond>(0.1);

    let mut measurement_delay_block: AnalogController = 
        ProportionalController::new(Ratio::new::<ratio>(1.0)).unwrap().into();

    measurement_delay_block.set_dead_time(measurement_delay);



    let initial_temperature = downcomer_exit_temp_set_point;


    // cold leg 
    let mut pipe_2 = new_uw_flibe_pipe_2(initial_temperature);
    let mut pipe_3 = new_uw_flibe_pipe_3(initial_temperature);
    let mut pipe_4 = new_uw_flibe_pipe_4(initial_temperature);
    let mut pipe_5 = new_uw_flibe_pipe_5(initial_temperature);
    let mut pipe_6 = new_uw_flibe_pipe_6(initial_temperature);
    let mut pipe_7 = new_uw_flibe_pipe_7(initial_temperature);

    // cold leg to hot leg bend 
    let mut pipe_8 = new_uw_flibe_pipe_8(initial_temperature);
    let mut pipe_9 = new_uw_flibe_pipe_9(initial_temperature);
    let mut pipe_10 = new_uw_flibe_pipe_10(initial_temperature);

    // hot leg 
    let mut pipe_11_bottom_cross_heater = new_uw_flibe_pipe_11_bottom_cross_heater(initial_temperature);
    let mut pipe_12 = new_uw_flibe_pipe_12(initial_temperature);
    let mut pipe_13 = new_uw_flibe_pipe_13(initial_temperature);
    let mut pipe_1_riser_heater = new_uw_flibe_pipe_1_riser_heater(initial_temperature);

    let ambient_htc = HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);

    // for postprocessing 
    let mut downcomer_heat_transfer_coeff: HeatTransfer = HeatTransfer::ZERO;
    let mut top_cross_heat_transfer_coeff: HeatTransfer = HeatTransfer::ZERO;
        
    let ((mut tc_21_estimate,mut tc_24_estimate,mut tc_35_estimate),
    (mut tc_11_estimate,mut tc_14_estimate)) = 
        ((initial_temperature,initial_temperature,initial_temperature),
        (initial_temperature,initial_temperature));

    // calculation loop
    while current_simulation_time < max_simulation_time {

        // placeholder for heat transfer coeff, need to deal with 
        // controller tho
        let cold_leg_diagonal_heat_transfer_coeff = 
        {
            // first, calculate the set point error 

            let reference_temperature_interval_deg_celsius = 50.0;

            // error = y_sp - y_measured
            let set_point_abs_error_deg_celsius = 
                - top_cross_exit_temp_set_point.get::<kelvin>()
                + tc_24_estimate.get::<kelvin>();

            let nondimensional_error: Ratio = 
                (set_point_abs_error_deg_celsius/
                 reference_temperature_interval_deg_celsius).into();
            // let's get the output 

            let dimensionless_heat_trf_input: Ratio
                = pid_controller_top_cross.set_user_input_and_calc(
                    nondimensional_error, 
                    current_simulation_time).unwrap();
            let mut top_cross_heat_trf_output = 
                dimensionless_heat_trf_input * reference_cooler_htc
                + reference_cooler_htc;

            // make sure it cannot be less than a certain amount 
            let top_cross_minimum_heat_transfer = 
                HeatTransfer::new::<watt_per_square_meter_kelvin>(
                    5.0);

            // this makes it physically realistic
            if top_cross_heat_trf_output < top_cross_minimum_heat_transfer {
                top_cross_heat_trf_output = top_cross_minimum_heat_transfer;
            }

            top_cross_heat_trf_output
        };

        // downcomer bit
        let cold_leg_vertical_heat_transfer_coeff = 
        { 
            // first, calculate the set point error 

            let reference_temperature_interval_deg_celsius = 80.0;

            // error = y_sp - y_measured
            let set_point_abs_error_deg_celsius = 
                - downcomer_exit_temp_set_point.get::<kelvin>()
                + tc_35_estimate.get::<kelvin>();

            let nondimensional_error: Ratio = 
                (set_point_abs_error_deg_celsius/
                 reference_temperature_interval_deg_celsius).into();
            // let's get the output 

            let dimensionless_heat_trf_input: Ratio
                = pid_controller_downcomer.set_user_input_and_calc(
                    nondimensional_error, 
                    current_simulation_time).unwrap();
            let mut downcomer_heat_trf_output = 
                dimensionless_heat_trf_input * reference_cooler_htc
                + reference_cooler_htc;

            // make sure it cannot be less than a certain amount 
            let downcomer_minimum_heat_transfer = 
                HeatTransfer::new::<watt_per_square_meter_kelvin>(
                    5.0);

            // this makes it physically realistic
            if downcomer_heat_trf_output < downcomer_minimum_heat_transfer {
                downcomer_heat_trf_output = downcomer_minimum_heat_transfer;
            }

            downcomer_heat_trf_output

        };

        // adjust cooler heat transfer coeff

        top_cross_heat_transfer_coeff = cold_leg_diagonal_heat_transfer_coeff;
        downcomer_heat_transfer_coeff = cold_leg_vertical_heat_transfer_coeff;

        // adjust parasitic heat losses 

        pipe_1_riser_heater.calibrate_insulation_thickness(
            riser_insulation_thickness);
        pipe_11_bottom_cross_heater.calibrate_insulation_thickness(
            bottom_cross_insulation_thickness);

        // link up the heat transfer entities 

        uw_madison_flibe_loop_link_up_components(
            mass_flowrate_clockwise, 
            hot_leg_diagonal_heater_power, 
            hot_leg_vertical_heater_power, 
            cold_leg_diagonal_heat_transfer_coeff, 
            cold_leg_vertical_heat_transfer_coeff, 
            average_temperature_for_density_calcs, 
            ambient_htc, 
            &mut pipe_1_riser_heater, 
            &mut pipe_2, 
            &mut pipe_3, 
            &mut pipe_4, 
            &mut pipe_5, 
            &mut pipe_6, 
            &mut pipe_7, 
            &mut pipe_8, 
            &mut pipe_9, 
            &mut pipe_10, 
            &mut pipe_11_bottom_cross_heater, 
            &mut pipe_12, 
            &mut pipe_13);

        uw_madison_flibe_loop_advance_timestep(
            timestep, 
            &mut pipe_1_riser_heater, 
            &mut pipe_2, 
            &mut pipe_3, 
            &mut pipe_4, 
            &mut pipe_5, 
            &mut pipe_6, 
            &mut pipe_7, 
            &mut pipe_8, 
            &mut pipe_9, 
            &mut pipe_10, 
            &mut pipe_11_bottom_cross_heater, 
            &mut pipe_12, 
            &mut pipe_13);

        let print_debug_results_settings = true;
        ((tc_21_estimate,tc_24_estimate,tc_35_estimate),(tc_11_estimate,tc_14_estimate))
            = uw_madison_flibe_loop_iteration_1_temperature_diagnostics(
                &mut pipe_1_riser_heater, 
                &mut pipe_2, 
                &mut pipe_3, 
                &mut pipe_4, 
                &mut pipe_5, 
                &mut pipe_6, 
                &mut pipe_7, 
                &mut pipe_8, 
                &mut pipe_9, 
                &mut pipe_10, 
                &mut pipe_11_bottom_cross_heater, 
                &mut pipe_12, 
                &mut pipe_13,
                print_debug_results_settings);

        current_simulation_time += timestep;

        if print_debug_results_settings == true {
            dbg!(&current_simulation_time);
        }
    }

    let top_cross_entrance_tc_21_simulated_degc = tc_21_estimate.get::<degree_celsius>();
    let top_cross_exit_tc_24_simulated_degc = tc_24_estimate.get::<degree_celsius>();
    let bottom_cross_entrance_tc_35_simulated_degc = tc_35_estimate.get::<degree_celsius>();
    let riser_entrance_tc_11_simulated_degc = tc_11_estimate.get::<degree_celsius>();
    let riser_exit_tc_14_simulated_degc = tc_14_estimate.get::<degree_celsius>();

    dbg!(&(
            top_cross_entrance_tc_21_simulated_degc,
            top_cross_exit_tc_24_simulated_degc,
            bottom_cross_entrance_tc_35_simulated_degc,
            riser_entrance_tc_11_simulated_degc,
            riser_exit_tc_14_simulated_degc
    ));

    dbg!(&(top_cross_heat_transfer_coeff,
            downcomer_heat_transfer_coeff));

    // assert set points equal to experimental data to within some tolerance
    approx::assert_abs_diff_eq!(
        top_cross_exit_tc_24_simulated_degc,
        tc_24_degc_expt,
        epsilon=regression_temperature_tolerance_kelvin);

    approx::assert_abs_diff_eq!(
        bottom_cross_entrance_tc_35_simulated_degc,
        tc_35_degc_expt,
        epsilon=regression_temperature_tolerance_kelvin);

    // assert heater exit temperatures
    //
    approx::assert_abs_diff_eq!(
        riser_entrance_tc_11_simulated_degc,
        tc_11_degc_expt,
        epsilon=regression_temperature_tolerance_kelvin);

    approx::assert_abs_diff_eq!(
        riser_exit_tc_14_simulated_degc,
        tc_14_degc_expt,
        epsilon=regression_temperature_tolerance_kelvin);

    // assert other temperatures around the loop

    approx::assert_abs_diff_ne!(
        top_cross_entrance_tc_21_simulated_degc,
        tc_21_degc_expt,
        epsilon=regression_temperature_tolerance_kelvin);



}


