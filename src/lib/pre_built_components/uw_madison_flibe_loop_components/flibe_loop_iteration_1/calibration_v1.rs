#[cfg(test)]
pub fn regression_uw_flibe_loop_v1(
    input_power_watts: f64,
    max_time_seconds: f64,
    tc_11_expt_temp_degc: f64,
    tc_12_expt_temp_degc: f64,
    tc_14_expt_temp_degc: f64,
    tc_21_expt_temp_degc: f64,
    tc_24_expt_temp_degc: f64,
    tc_32_expt_temp_degc: f64,
    tc_35_expt_temp_degc: f64,
    experimentally_estimated_salt_velocity_cm_per_s: f64,
    mass_flowrate_relative_tolerance: f64,
    ){

    use uom::si::length::centimeter;
    use uom::si::{f64::*, mass_rate::kilogram_per_second, power::watt};

    use uom::si::{frequency::hertz, ratio::ratio, time::millisecond};

    use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::prelude::beta_testing::FluidArray;
    use uom::ConstZero;

    use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::time::second;
    use chem_eng_real_time_process_control_simulator::alpha_nightly::transfer_fn_wrapper_and_enums::TransferFnTraits;
    use chem_eng_real_time_process_control_simulator::alpha_nightly::controllers::ProportionalController;
    use chem_eng_real_time_process_control_simulator::alpha_nightly::controllers::AnalogController;


    // now, there are four separate segments of heater for the 
    // UW madison FLiBe loop
    //
    // under README.txt in the FLiBe loop data:
    //
    // Britsch, K. R., Doniger, W., Anderson, M., & 
    // Sridharan, K. (2018). Operation Data from the UW 
    // Natural Circulation FLiBe Flow Loop. 
    // University of Wisconsin-Madison.
    //
    // The heater signals are from 0-10V, multiply by 10 to obtain 
    // percent power. 
    //
    // Now, that only gives percent power. Not the wattage of each 
    // heater.
    //
    // So it's kinda hard to see 
    //
    // The other way is to back calculate the heat input into the 
    // FLiBe, and compare that with the heater power.
    let input_power = Power::new::<watt>(input_power_watts);

    // experimental data for tc 24 and 35 are used as set points
    let tc24_set_point = 
        ThermodynamicTemperature::new::<degree_celsius>(
            tc_24_expt_temp_degc);
    let tc35_set_point = 
        ThermodynamicTemperature::new::<degree_celsius>(
            tc_35_expt_temp_degc);
    let timestep = Time::new::<second>(0.5);
}
