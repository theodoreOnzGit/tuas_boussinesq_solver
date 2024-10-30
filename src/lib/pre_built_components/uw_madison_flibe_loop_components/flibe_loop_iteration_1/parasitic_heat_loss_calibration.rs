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


