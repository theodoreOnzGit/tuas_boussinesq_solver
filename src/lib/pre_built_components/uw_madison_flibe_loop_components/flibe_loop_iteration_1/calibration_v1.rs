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
    use crate::pre_built_components::ciet_isothermal_test_components::*;
    use crate::pre_built_components::ciet_steady_state_natural_circulation_test_components::coupled_dracs_loop_tests::dhx_constructor::new_dhx_sthe_version_1;
    use crate::pre_built_components::ciet_steady_state_natural_circulation_test_components::coupled_dracs_loop_tests::dracs_loop_calc_functions_no_tchx_calibration::dracs_loop_dhx_tube_temperature_diagnostics;
    use crate::pre_built_components::ciet_steady_state_natural_circulation_test_components::coupled_dracs_loop_tests::dracs_loop_calc_functions_sam_tchx_calibration::{coupled_dracs_fluid_mechanics_calc_abs_mass_rate_sam_tchx_calibration, coupled_dracs_loop_link_up_components_sam_tchx_calibration, dracs_loop_advance_timestep_except_dhx_sam_tchx_calibration};
    use crate::pre_built_components::ciet_steady_state_natural_circulation_test_components::coupled_dracs_loop_tests::pri_loop_calc_functions::{coupled_dracs_pri_loop_branches_fluid_mechanics_calc_abs_mass_rate, coupled_dracs_pri_loop_dhx_heater_link_up_components, pri_loop_advance_timestep_except_dhx, pri_loop_dhx_shell_temperature_diagnostics, pri_loop_heater_temperature_diagnostics};
    use crate::pre_built_components::
        ciet_steady_state_natural_circulation_test_components::dracs_loop_components::*;
    use crate::prelude::beta_testing::FluidArray;
    use uom::ConstZero;

    use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;
    use uom::si::time::second;
    use chem_eng_real_time_process_control_simulator::alpha_nightly::transfer_fn_wrapper_and_enums::TransferFnTraits;
    use chem_eng_real_time_process_control_simulator::alpha_nightly::controllers::ProportionalController;
    use chem_eng_real_time_process_control_simulator::alpha_nightly::controllers::AnalogController;


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
