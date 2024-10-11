



/// forward and reverse testing of getting a bejan number from reynolds 
/// number
#[test]
pub fn dimensionless_darcy_loss_correlation_get_pressure_loss() -> Result<(), 
    crate::tuas_lib_error::TuasLibError>
{
    use uom::si::f64::*; 
    use uom::si::ratio::ratio;
    use crate::array_control_vol_and_fluid_component_collections::
        one_d_fluid_array_with_lateral_coupling::DimensionlessDarcyLossCorrelations;
    use uom::si::length::meter;
    use uom::si::pressure::pascal;
    use uom::si::mass_density::kilogram_per_cubic_meter;
    use uom::si::dynamic_viscosity::centipoise;

    let flowmeter_dimensionless_correlation = 
        DimensionlessDarcyLossCorrelations::
        new_simple_reynolds_power_component(
            Ratio::new::<ratio>(18.0),
            Ratio::new::<ratio>(93000_f64),
            -1.35);

    // forward flow
    let mut reynolds_number = Ratio::new::<ratio>(4000.0);
    let hydraulic_diameter: Length = Length::new::<meter>(1.0);
    let fluid_density = MassDensity::new::<kilogram_per_cubic_meter>(1000.0);
    let fluid_viscosity = DynamicViscosity::new::<centipoise>(1.0);


    let pressure_loss_test = flowmeter_dimensionless_correlation.
        get_pressure_loss_from_reynolds(
            reynolds_number,
            hydraulic_diameter,
            fluid_density,
            fluid_viscosity).unwrap();


    approx::assert_relative_eq!(
        pressure_loss_test.get::<pascal>(),
        0.154204505,
        max_relative = 0.001 );

    // reverse flow
    reynolds_number = Ratio::new::<ratio>(-4000.0);

    let pressure_loss_test_reverse = flowmeter_dimensionless_correlation.
        get_pressure_loss_from_reynolds(
            reynolds_number,
            hydraulic_diameter,
            fluid_density,
            fluid_viscosity).unwrap();
    
    approx::assert_relative_eq!(
        pressure_loss_test_reverse.get::<pascal>(),
        -0.154204505,
        max_relative = 0.001 );


    Ok(())
}

/// forward and reverse testing of getting a reynolds number from bejan 
/// number
#[test]
pub fn dimensionless_darcy_loss_correlation_get_mass_flowrate_from_pressure_loss() -> Result<(), 
    crate::tuas_lib_error::TuasLibError>
{
    use uom::si::f64::*; 
    use uom::si::ratio::ratio;
    use crate::array_control_vol_and_fluid_component_collections::
        one_d_fluid_array_with_lateral_coupling::DimensionlessDarcyLossCorrelations;
    use uom::si::length::meter;
    use uom::si::mass_density::kilogram_per_cubic_meter;
    use uom::si::dynamic_viscosity::centipoise;
    use uom::si::pressure::pascal;

    let flowmeter_dimensionless_correlation = 
        DimensionlessDarcyLossCorrelations::
        new_simple_reynolds_power_component(
            Ratio::new::<ratio>(18.0),
            Ratio::new::<ratio>(93000_f64),
            -1.35);

    // forward flow
    let hydraulic_diameter: Length = Length::new::<meter>(1.0);
    let fluid_density = MassDensity::new::<kilogram_per_cubic_meter>(1000.0);
    let fluid_viscosity = DynamicViscosity::new::<centipoise>(1.0);
    let mut pressure_loss = Pressure::new::<pascal>(0.154204505);

    let reynolds_test = flowmeter_dimensionless_correlation.
        get_reynolds_from_pressure_loss(
            pressure_loss,
            hydraulic_diameter,
            fluid_density,
            fluid_viscosity).unwrap();

    approx::assert_relative_eq!(
        reynolds_test.get::<ratio>(),
        4000.0,
        max_relative = 0.001 );

    // reverse flow 
    pressure_loss = -Pressure::new::<pascal>(0.154204505);

    let reynolds_test_reverse = flowmeter_dimensionless_correlation.
        get_reynolds_from_pressure_loss(
            pressure_loss,
            hydraulic_diameter,
            fluid_density,
            fluid_viscosity).unwrap();

    approx::assert_relative_eq!(
        reynolds_test_reverse.get::<ratio>(),
        -4000.0,
        max_relative = 0.001 );

    Ok(())
}
