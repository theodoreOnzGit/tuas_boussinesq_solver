

/// 
/// used a manual calculation where 
/// radiation from 750 C body goes to a 650 C body 
///
/// P = sigma A (T_hot^4 - T_cold^4)
///
/// stefan boltzmann constant used is 5.670e-8 W/(m^2 K^4)
///
/// assuming emissivity is 1, area is 1 m^2 etc 
/// 750C = 1023.15 kelvin
/// 650C = 923.15 kelvin
///
/// P = (5.67e-8 W/(m^2 K^4)) * 1 m^2 * (1023.15^4 - 923.15^4) K^4 
/// P = 20956.91616 W
/// 
/// Basically, we should have this value
/// P = H_rad(T_hot - T_cold)
/// 
#[test]
pub fn radiation_conductance_unit_test(){

    use uom::si::area::square_meter;
    use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};
    use uom::si::f64::*;
    use uom::si::{power::watt, temperature_interval};

    use super::simple_radiation_conductance;
    let hot_temperature: ThermodynamicTemperature =
        ThermodynamicTemperature::new::<degree_celsius>(750.0);
    let cold_temperature: ThermodynamicTemperature =
        ThermodynamicTemperature::new::<degree_celsius>(650.0);

    let area_coeff = Area::new::<square_meter>(1.0);

    let radiation_thermal_conductance = 
        simple_radiation_conductance(
            area_coeff, hot_temperature, cold_temperature);

    let temperature_interval: ThermodynamicTemperature
        = hot_temperature - 
        TemperatureInterval::new::<temperature_interval::kelvin> (
            cold_temperature.get::<kelvin>()
        )
        ;

    let power: Power =  
        radiation_thermal_conductance * 
        temperature_interval;

    // assert that it is 
    // P = 20956.91616 W

    approx::assert_relative_eq!(
        power.get::<watt>(),
        20956.91616,
        max_relative=1e-4
        );


}
