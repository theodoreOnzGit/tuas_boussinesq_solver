



#[test] 
pub fn heater_branch_pressure_change_test(){

    // let's construct the branches with test pressures and obtain 
    // mass flowrates
    use crate::
        array_control_vol_and_fluid_component_collections::
        fluid_component_collection::
        fluid_component_collection::FluidComponentCollectionMethods;
    use crate::pre_built_components::
        ciet_isothermal_test_components::
        ciet_branch_builders_isothermal::heater_branch_builder_isothermal_test;
    use uom::si::f64::*;
    use uom::ConstZero;
    use uom::si::mass_rate::kilogram_per_second;
    use approx::assert_abs_diff_eq;
    use uom::si::pressure::pascal;

    let test_temperature = ThermodynamicTemperature::
        new::<uom::si::thermodynamic_temperature::degree_celsius>(21.7);
    let heater_branch = heater_branch_builder_isothermal_test(test_temperature);

    // pressure change at 0 kg/s 
    let pressure_change_at_zero_kg_per_s = 
        heater_branch.get_pressure_change(MassRate::ZERO);

    // pressure change should be 39041 +/- 1 Pa
    assert_abs_diff_eq!(pressure_change_at_zero_kg_per_s.get::<pascal>(), 
        39041.0, epsilon=1.0,);

    // now at 0.18 kg/s
    // pressure change at 0 kg/s 
    let pressure_change_at_0_18_kg_per_s = 
        heater_branch.get_pressure_change(
            MassRate::new::<kilogram_per_second>(0.18));

    // pressure change should be 33417.0 +/- 100 Pa
    // this is based on the old heater branch values 
    // form the old ciet isothermal server
    assert_abs_diff_eq!(pressure_change_at_0_18_kg_per_s.get::<pascal>(), 
        33417.0, epsilon=100.0,);


}

#[test] 
pub fn ctah_branch_pressure_change_test(){

    // let's construct the branches with test pressures and obtain 
    // mass flowrates
    use crate::
        array_control_vol_and_fluid_component_collections::
        fluid_component_collection::
        fluid_component_collection::FluidComponentCollectionMethods;
    use crate::pre_built_components::
        ciet_isothermal_test_components::
        ciet_branch_builders_isothermal::ctah_branch_builder_isothermal_test;
    use uom::si::f64::*;
    use uom::ConstZero;
    use uom::si::mass_rate::kilogram_per_second;
    use approx::assert_abs_diff_eq;
    use uom::si::pressure::pascal;

    let test_temperature = ThermodynamicTemperature::
        new::<uom::si::thermodynamic_temperature::degree_celsius>(21.7);
    let pump_pressure = Pressure::ZERO;
    let ctah_branch = ctah_branch_builder_isothermal_test(
        pump_pressure,test_temperature);

    // pressure change at 0 kg/s 
    let pressure_change_at_zero_kg_per_s = 
        ctah_branch.get_pressure_change(MassRate::ZERO);

    // pressure change should be 39041 +/- 1 Pa
    assert_abs_diff_eq!(pressure_change_at_zero_kg_per_s.get::<pascal>(), 
        39041.0, epsilon=1.0,);

    // now at 0.18 kg/s
    // pressure change at 0 kg/s 
    let pressure_change_at_0_18_kg_per_s = 
        ctah_branch.get_pressure_change(
            MassRate::new::<kilogram_per_second>(0.18));

    // pressure change should be 28751.0 +/- 100 Pa
    // this is based on the old heater branch values 
    // form the old ciet isothermal server
    assert_abs_diff_eq!(pressure_change_at_0_18_kg_per_s.get::<pascal>(), 
        28751.0, epsilon=100.0,);


}
/// basically the ctah branch is rather buggy, so we have to 
/// test one branch at a time
#[test]
pub fn partial_ctah_branch_test(){

    use crate::pre_built_components::ciet_isothermal_test_components::{new_branch_17, new_ctah_pump, new_flowmeter_40_14a, new_inactive_ctah_horizontal, new_inactive_ctah_vertical, new_pipe_10, new_pipe_11, new_pipe_12, new_pipe_13, new_pipe_14, new_pipe_15, new_pipe_16, new_pipe_6a, new_pipe_8a, new_pipe_9, new_static_mixer_40, new_static_mixer_41};
    use uom::si::f64::*;
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_collection::FluidComponentCollection;
    use uom::si::mass_rate::kilogram_per_second;
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_collection::FluidComponentCollectionMethods;

    use uom::si::pressure::pascal;
    // first let's construct the ctah branch
    // this is pipe 6 all the way to branch 17
    let test_temperature = ThermodynamicTemperature::
        new::<uom::si::thermodynamic_temperature::degree_celsius>(21.7);

    let static_mixer_41_label_6 = new_static_mixer_41(test_temperature);
    let pipe_6a = new_pipe_6a(test_temperature);
    let ctah_vertical_label_7a = new_inactive_ctah_vertical(test_temperature);
    let ctah_horizontal_label_7b = new_inactive_ctah_horizontal(test_temperature);
    let pipe_8a = new_pipe_8a(test_temperature);
    let static_mixer_40_label_8 = new_static_mixer_40(test_temperature);
    let pipe_9 = new_pipe_9(test_temperature);
    let pipe_10 = new_pipe_10(test_temperature);
    let pipe_11 = new_pipe_11(test_temperature);
    let pipe_12 = new_pipe_12(test_temperature);
    let ctah_pump = new_ctah_pump(test_temperature);
    let pipe_13 = new_pipe_13(test_temperature);
    let pipe_14 = new_pipe_14(test_temperature);
    let flowmeter_40_14a = new_flowmeter_40_14a(test_temperature);
    let pipe_15 = new_pipe_15(test_temperature);
    let pipe_16 = new_pipe_16(test_temperature);
    let branch_17 = new_branch_17(test_temperature);


    // now I want to add each of these to the fluid component 
    // collection without the constant hassle of having to convert types
    //
    // first let's test up to pipe 10

    let mut ctah_branch = FluidComponentCollection::new_series_component_collection();
    
    ctah_branch.clone_and_add_component(&static_mixer_41_label_6);
    ctah_branch.clone_and_add_component(&pipe_6a);
    ctah_branch.clone_and_add_component(&ctah_vertical_label_7a);
    ctah_branch.clone_and_add_component(&ctah_horizontal_label_7b);
    ctah_branch.clone_and_add_component(&pipe_8a);
    ctah_branch.clone_and_add_component(&static_mixer_40_label_8);
    ctah_branch.clone_and_add_component(&pipe_9);
    ctah_branch.clone_and_add_component(&pipe_10);

    // let's test the pressure change, 0.18 kg/s fluid flow 
    {
        // now let's push a 0.18kg/s fluid flow through this pipe series
        //
        let pipe_fluid_flow = MassRate::new::<kilogram_per_second>(0.18);

        // and then let's get the pressure change

        let series_pipe_pressure_change = ctah_branch.
            get_pressure_change(pipe_fluid_flow);

        // pressure change is around 39041 Pa
        approx::assert_relative_eq!(
            series_pipe_pressure_change.get::<pascal>(),
            28347.0,
            max_relative=0.001);
    }

    // clear the ctah branch 
    ctah_branch.components.clear();
    ctah_branch.clone_and_add_component(&pipe_11);
    ctah_branch.clone_and_add_component(&pipe_12);
    ctah_branch.clone_and_add_component(&ctah_pump);
    ctah_branch.clone_and_add_component(&pipe_13);
    ctah_branch.clone_and_add_component(&pipe_14);
    ctah_branch.clone_and_add_component(&flowmeter_40_14a);
    ctah_branch.clone_and_add_component(&pipe_15);
    ctah_branch.clone_and_add_component(&pipe_16);
    ctah_branch.clone_and_add_component(&branch_17);

    // let's test the pressure change, 0.18 kg/s fluid flow 
    {
        // now let's push a 0.18kg/s fluid flow through this pipe series
        //
        let pipe_fluid_flow = MassRate::new::<kilogram_per_second>(0.18);

        // and then let's get the pressure change

        let series_pipe_pressure_change = ctah_branch.
            get_pressure_change(pipe_fluid_flow);

        // pressure change is around 39041 Pa
        approx::assert_relative_eq!(
            series_pipe_pressure_change.get::<pascal>(),
            442.0,
            max_relative=0.001);
    }
}

/// for the CTAH pump, I expect zero resistance or pressure drop 
/// that is assumed
///
/// This is correct
#[test]
pub fn ctah_pump_should_give_zero_resistance(){
    use uom::si::f64::*;
    use uom::si::mass_rate::kilogram_per_second;
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
    let test_temperature = ThermodynamicTemperature::
        new::<uom::si::thermodynamic_temperature::degree_celsius>(21.7);
    let ctah_pump = crate::pre_built_components::
        ciet_isothermal_test_components::new_ctah_pump(test_temperature);

    let mass_rate = MassRate::new::<kilogram_per_second>(0.18);

    let pressure_drop = ctah_pump.get_pressure_loss_immutable(mass_rate);

    approx::assert_abs_diff_eq!(
        pressure_drop.get::<uom::si::pressure::pascal>(),
        0.0,
        );

    let pressure_change = ctah_pump.get_pressure_change_immutable(mass_rate);

    approx::assert_abs_diff_eq!(
        pressure_change.get::<uom::si::pressure::pascal>(),
        0.0,
        )

}


/// this test checks the functionality of solving for mass flowrates 
/// across each branch while automatically detemrmining bounds
#[test]
pub fn isothermal_ctah_and_heater_branch_parallel_associated_functions_regression_test(){

    // let's construct the branches with test pressures and obtain 
    // mass flowrates
    use uom::si::f64::*;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::pressure::pascal;
    use crate::pre_built_components::
        ciet_isothermal_test_components::ciet_branch_builders_isothermal
        ::*;
    use uom::ConstZero;
    use uom::si::thermodynamic_temperature::degree_celsius;

    use crate::
        array_control_vol_and_fluid_component_collections::
        fluid_component_collection::
        fluid_component_super_collection::FluidComponentSuperCollection;
    use crate::
        array_control_vol_and_fluid_component_collections::
        fluid_component_collection::
        fluid_component_collection::FluidComponentCollectionMethods;

    fn verify_mass_flowrate_given_pressure_change(
        test_pressure: Pressure,
        expected_mass_flow: MassRate){

        let verification_temperature = 
            ThermodynamicTemperature::new::<degree_celsius>(20.0);
        let heater_branch = heater_branch_builder_isothermal_test(verification_temperature);
        let ctah_branch = ctah_branch_builder_isothermal_test(
            test_pressure,
            verification_temperature);

        // expected flowrate 

        // you'll now need to add this into a super collection
        let mut ctah_and_heater_branches = 
            FluidComponentSuperCollection::default();
        ctah_and_heater_branches.set_orientation_to_parallel();
        ctah_and_heater_branches.fluid_component_super_vector
            .push(heater_branch);
            ctah_and_heater_branches.fluid_component_super_vector
                .push(ctah_branch);

            // obtain flowrate 
            // the overall pressure change across each branch must be
            // equal
            //
            // the total mass flowrate through the collection of parallel 
            // branches is zero
            //
            // the get_pressure_change function is the main one 
            // we are testing now
            let pressure_change_across_each_branch = 
                ctah_and_heater_branches
                .get_pressure_change(MassRate::ZERO);





            // once we have pressure change across each branch,
            // then we can calculate mass flowrate.

            let mass_flowrate_across_each_branch: Vec<MassRate> = 
                ctah_and_heater_branches.
                get_mass_flowrate_across_each_parallel_branch(
                    pressure_change_across_each_branch
                );


            // let's obtain the mass flowrate, it should be zero 
            // or close to it 

            let mut mass_flowrate_test: MassRate = 
                *mass_flowrate_across_each_branch
                .first()
                .unwrap();

            // get absolute value
            mass_flowrate_test = mass_flowrate_test.abs();

            // if mass flowrate is zero, use abs diff
            if mass_flowrate_test.get::<kilogram_per_second>() == 0.0 {
                approx::assert_abs_diff_eq!(
                    mass_flowrate_test.get::<kilogram_per_second>(),
                    expected_mass_flow.get::<kilogram_per_second>(),
                    epsilon=0.0);
                return;
            } else if mass_flowrate_test.get::<kilogram_per_second>() < 0.02 {
                // for low flowrates
                // 
                // For verification, we have error bound 
                // of 0.5% to reproduce previous results
                // due to round off error of 3sf
                // 
                approx::assert_relative_eq!(
                    mass_flowrate_test.get::<kilogram_per_second>().abs(),
                    expected_mass_flow.get::<kilogram_per_second>().abs(),
                    max_relative=0.005);
            }
            else {
                // for large flowrate 
                // verification tests, we have error bound 
                // of 0.5% to reproduce previous results
                // due to round off error
                // of 3 sf
                approx::assert_relative_eq!(
                    mass_flowrate_test.get::<kilogram_per_second>().abs(),
                    expected_mass_flow.get::<kilogram_per_second>().abs(),
                    max_relative=0.005);

            }



    }

    // now let's verify this across all flowrates 

    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(0.0), 
        MassRate::new::<kilogram_per_second>(0.0)
    );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(100.0), 
        MassRate::new::<kilogram_per_second>(0.00263)
    );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(200.0), 
        MassRate::new::<kilogram_per_second>(0.00527)
    );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(500.0), 
        MassRate::new::<kilogram_per_second>(0.0127)
    );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(1000.0), 
        MassRate::new::<kilogram_per_second>(0.0236)
    );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(2000.0), 
        MassRate::new::<kilogram_per_second>(0.0418)
    );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(4000.0), 
        MassRate::new::<kilogram_per_second>(0.0706)
    );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(6000.0), 
        MassRate::new::<kilogram_per_second>(0.0938)
    );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(8000.0), 
        MassRate::new::<kilogram_per_second>(0.114)
    );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(10000.0), 
        MassRate::new::<kilogram_per_second>(0.132)
    );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(12000.0), 
        MassRate::new::<kilogram_per_second>(0.148)
    );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(15000.0), 
        MassRate::new::<kilogram_per_second>(0.170)
    );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(16000.0), 
        MassRate::new::<kilogram_per_second>(0.177)
    );

    //// reverse flow tests (later)
    //verify_mass_flowrate_given_pressure_change(
    //    Pressure::new::<pascal>(-2000.0), 
    //    MassRate::new::<kilogram_per_second>(-0.0418)
    //    );
    //verify_mass_flowrate_given_pressure_change(
    //    Pressure::new::<pascal>(-10000.0), 
    //    MassRate::new::<kilogram_per_second>(-0.132)
    //    );


}
