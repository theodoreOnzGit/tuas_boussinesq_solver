/// forward and reverse testing of getting a bejan number from reynolds 
/// number
#[test]
pub fn dimensionless_darcy_loss_correlation_get_be() -> Result<(), 
    crate::thermal_hydraulics_error::ThermalHydraulicsLibError>
{
    use uom::si::f64::*; 
    use uom::si::ratio::ratio;
    use crate::array_control_vol_and_fluid_component_collections::
        one_d_fluid_array_with_lateral_coupling::DimensionlessDarcyLossCorrelations;

    let flowmeter_dimensionless_correlation = 
        DimensionlessDarcyLossCorrelations::
        new_simple_reynolds_power_component(
            Ratio::new::<ratio>(18.0),
            Ratio::new::<ratio>(93000_f64),
            -1.35);

    // forward flow
    let mut reynolds_number = Ratio::new::<ratio>(4000.0);

    let bejan_test = flowmeter_dimensionless_correlation.
        get_bejan_number_from_reynolds(reynolds_number).unwrap();


    approx::assert_relative_eq!(
        bejan_test.value,
        154204505.0,
        max_relative = 0.001 );

    // reverse flow
    reynolds_number = Ratio::new::<ratio>(-4000.0);

    let bejan_test_reverse = flowmeter_dimensionless_correlation.
        get_bejan_number_from_reynolds(reynolds_number).unwrap();
    
    approx::assert_relative_eq!(
        bejan_test_reverse.value,
        -154204505.0,
        max_relative = 0.001 );


    Ok(())
}

/// forward and reverse testing of getting a reynolds number from bejan 
/// number
#[test]
pub fn dimensionless_darcy_loss_correlation_get_re_from_be() -> Result<(), 
    crate::thermal_hydraulics_error::ThermalHydraulicsLibError>
{
    use uom::si::f64::*; 
    use uom::si::ratio::ratio;
    use crate::array_control_vol_and_fluid_component_collections::
        one_d_fluid_array_with_lateral_coupling::DimensionlessDarcyLossCorrelations;

    let flowmeter_dimensionless_correlation = 
        DimensionlessDarcyLossCorrelations::
        new_simple_reynolds_power_component(
            Ratio::new::<ratio>(18.0),
            Ratio::new::<ratio>(93000_f64),
            -1.35);

    // forward flow
    let mut bejan_number = Ratio::new::<ratio>(154204505.0);

    let reynolds_test = flowmeter_dimensionless_correlation.
        get_reynolds_number_from_bejan(bejan_number).unwrap();

    approx::assert_relative_eq!(
        reynolds_test.value,
        4000.0,
        max_relative = 0.001 );

    // reverse flow 
    bejan_number = -Ratio::new::<ratio>(154204505.0);

    let reynolds_test_reverse = flowmeter_dimensionless_correlation.
        get_reynolds_number_from_bejan(bejan_number).unwrap();

    approx::assert_relative_eq!(
        reynolds_test_reverse.value,
        -4000.0,
        max_relative = 0.001 );

    Ok(())
}
