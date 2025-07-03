

/// this is a isothermal pressure drop 
/// regression test 
/// get pressure change from mass flowrate
#[test] 
pub fn dhx_branch_pressure_change_regression_test(){

    // let's construct the branches with test pressures and obtain 
    // mass flowrates
    use crate::
        array_control_vol_and_fluid_component_collections::
        fluid_component_collection::
        fluid_component_collection::FluidComponentCollectionMethods;
    use crate::pre_built_components::
        ciet_isothermal_test_components::
        ciet_branch_builders_isothermal::dhx_branch_builder_isothermal_test;
    use uom::si::f64::*;
    use uom::ConstZero;
    use uom::si::mass_rate::kilogram_per_second;
    use approx::assert_abs_diff_eq;
    use uom::si::pressure::pascal;

    use uom::si::thermodynamic_temperature::degree_celsius;

    let test_temperature = ThermodynamicTemperature::
        new::<degree_celsius>(21.7);
    let dhx_branch = dhx_branch_builder_isothermal_test(test_temperature);

    // pressure change at 0 kg/s 
    let pressure_change_at_zero_kg_per_s = 
        dhx_branch.get_pressure_change(MassRate::ZERO);

    // pressure change should be 39041 +/- 1 Pa
    assert_abs_diff_eq!(pressure_change_at_zero_kg_per_s.get::<pascal>(), 
        39041.0, epsilon=1.0,);

    // now at 0.18 kg/s (reverse flow)
    // pressure change at 0 kg/s 
    let pressure_change_at_0_18_kg_per_s = 
        dhx_branch.get_pressure_change(
            MassRate::new::<kilogram_per_second>(-0.18));

    // pressure change should be 44494.0 +/- 100 Pa
    // this is based on the old dhx branch values 
    // form the old ciet isothermal server
    assert_abs_diff_eq!(pressure_change_at_0_18_kg_per_s.get::<pascal>(), 
        44494.0, epsilon=100.0,);


}
