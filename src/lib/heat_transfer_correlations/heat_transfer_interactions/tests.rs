use std::f64::consts::PI;

use uom::si::{heat_transfer::watt_per_square_meter_kelvin, length::inch, pressure::atmosphere, thermal_conductivity::watt_per_meter_kelvin, thermodynamic_temperature::degree_celsius};

use crate::{boussinesq_thermophysical_properties::SolidMaterial, heat_transfer_correlations::{heat_transfer_interactions::{get_conductance_single_cylindrical_radial_solid_liquid, heat_transfer_geometry::CylindricalAndSphericalSolidFluidArrangement}, thermal_resistance::try_get_thermal_conductance_annular_cylinder}};




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
    // P = 20956.91616 W to within 0.01%,
    // the botlzmann constant used will change things.

    approx::assert_relative_eq!(
        power.get::<watt>(),
        20956.91616,
        max_relative=1e-4
        );


}

/// for an annular cylinder, the most common geometry used,
/// the thermal resistance is:
///
/// ln (r2/r1) / (2 PI * L * K)
///
/// the conductance is just the inverse 
///  (2 PI * L * K) / ln (r2/r1)  
///
/// the conductance lengthscale is just the conductance divided  by 
/// the thermal conductivity
///  (2 PI * L ) / ln (r2/r1)  
///
///  this is an easy thing to test
#[test]
pub fn radial_conductance_solid_inside_liquid_inside_test(){

    use uom::si::f64::*;
    
    use uom::si::length::meter;
    let pipe_length = Length::new::<meter>(1.0);
    let r_inner = Length::new::<inch>(1.0);
    let r_outer = Length::new::<inch>(5.0);

    let thermal_conductance_lengthscale_reference: Length = 
        2.0 * PI * pipe_length / (r_outer/r_inner).ln();

    // for thermal conductivity, let's get steel at 25C atmospheric 
    // pressure
    let steel = SolidMaterial::SteelSS304L;
    let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
    let room_temp = ThermodynamicTemperature::new::<degree_celsius>(25.0);

    // and for the heat transfer coeff, let's just set it so high 
    // that the thermal resistance is negligible

    let high_htc = HeatTransfer::new::<watt_per_square_meter_kelvin>(1e15 as f64);

    let solid_liquid_arrangement = 
        CylindricalAndSphericalSolidFluidArrangement::FluidOnInnerSurfaceOfSolidShell;
    // now, if we get the thermal conductance, it should be 
    // dominated by the solid side 
    let test_thermal_conductance_overall = 
        get_conductance_single_cylindrical_radial_solid_liquid(
            steel.into(), 
            room_temp, 
            atmospheric_pressure, 
            high_htc, 
            r_inner.into(), 
            r_outer.into(), 
            pipe_length.into(), 
            solid_liquid_arrangement).unwrap();

    // the thermal conductance lengthscale 
    // should be pretty much the same as 
    // the reference

    let thermal_conductance_lengthscale_test = 
        test_thermal_conductance_overall
        /steel.try_get_thermal_conductivity(room_temp).unwrap();

    approx::assert_relative_eq!(
        thermal_conductance_lengthscale_test.get::<meter>(),
        thermal_conductance_lengthscale_reference.get::<meter>(),
        max_relative=1e-9
        );

}
/// for an annular cylinder, the most common geometry used,
/// the thermal resistance is:
///
/// ln (r2/r1) / (2 PI * L * K)
///
/// the conductance is just the inverse 
///  (2 PI * L * K) / ln (r2/r1)  
///
/// the conductance lengthscale is just the conductance divided  by 
/// the thermal conductivity
///  (2 PI * L ) / ln (r2/r1)  
///
///  this is an easy thing to test
#[test]
pub fn radial_conductance_solid_inside_liquid_outside_test(){

    use uom::si::f64::*;
    
    use uom::si::length::meter;
    let pipe_length = Length::new::<meter>(1.0);
    let r_inner = Length::new::<inch>(1.0);
    let r_outer = Length::new::<inch>(5.0);

    let thermal_conductance_lengthscale_reference: Length = 
        2.0 * PI * pipe_length / (r_outer/r_inner).ln();

    // for thermal conductivity, let's get steel at 25C atmospheric 
    // pressure
    let steel = SolidMaterial::SteelSS304L;
    let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
    let room_temp = ThermodynamicTemperature::new::<degree_celsius>(25.0);

    // and for the heat transfer coeff, let's just set it so high 
    // that the thermal resistance is negligible

    let high_htc = HeatTransfer::new::<watt_per_square_meter_kelvin>(1e15 as f64);

    let solid_liquid_arrangement = 
        CylindricalAndSphericalSolidFluidArrangement::FluidOnOuterSurfaceOfSolidShell;
    // now, if we get the thermal conductance, it should be 
    // dominated by the solid side 
    let test_thermal_conductance_overall = 
        get_conductance_single_cylindrical_radial_solid_liquid(
            steel.into(), 
            room_temp, 
            atmospheric_pressure, 
            high_htc, 
            r_inner.into(), 
            r_outer.into(), 
            pipe_length.into(), 
            solid_liquid_arrangement).unwrap();

    // the thermal conductance lengthscale 
    // should be pretty much the same as 
    // the reference

    let thermal_conductance_lengthscale_test = 
        test_thermal_conductance_overall
        /steel.try_get_thermal_conductivity(room_temp).unwrap();

    approx::assert_relative_eq!(
        thermal_conductance_lengthscale_test.get::<meter>(),
        thermal_conductance_lengthscale_reference.get::<meter>(),
        max_relative=1e-9
        );

}

/// for an annular cylinder, the most common geometry used,
/// the thermal resistance is:
///
/// ln (r2/r1) / (2 PI * L * K)
///
/// the conductance is just the inverse 
///  (2 PI * L * K) / ln (r2/r1)  
///
/// the conductance lengthscale is just the conductance divided  by 
/// the thermal conductivity
///  (2 PI * L ) / ln (r2/r1)  
///
///  this is an easy thing to test
#[test]
pub fn radial_conductance_solid_test(){

    use uom::si::f64::*;
    
    use uom::si::length::meter;
    let pipe_length = Length::new::<meter>(1.0);
    let r_inner = Length::new::<inch>(1.0);
    let r_outer = Length::new::<inch>(5.0);

    let thermal_conductance_lengthscale_reference: Length = 
        2.0 * PI * pipe_length / (r_outer/r_inner).ln();

    // suppose i get a random thermal conductivity 
    //
    let thermal_conductivity_k = 
        ThermalConductivity::new::<watt_per_meter_kelvin>(5.0);

    // and then i try getting thermal conductance 
    let thermal_conductance_h = 
        try_get_thermal_conductance_annular_cylinder(
            r_inner, 
            r_outer, 
            pipe_length, 
            thermal_conductivity_k).unwrap();

    // the thermal conductance lengthscale should be the same 

    let thermal_conductance_lengthscale_test = 
        thermal_conductance_h/thermal_conductivity_k;


    assert_eq!(thermal_conductance_lengthscale_test,
        thermal_conductance_lengthscale_reference)


}

