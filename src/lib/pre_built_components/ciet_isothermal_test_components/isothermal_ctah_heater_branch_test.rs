use roots::SimpleConvergency;
use roots::find_root_brent;
use uom::si::f64::*;
use uom::ConstZero;
use uom::si::mass_rate::kilogram_per_second;
use uom::si::pressure::pascal;

use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::{fluid_component_collection::{FluidComponentCollection, FluidComponentCollectionMethods}, fluid_component_super_collection::FluidComponentSuperCollection};


/// checks if the ctah branch is working correctly
/// pressure change from mass flowrate
/// when we add various components to a series FluidComponentCollection 
/// this is a regression test
#[test]
pub fn heater_branch_with_heater_v2_test(){

    use crate::pre_built_components::ciet_isothermal_test_components::*;
    use crate::array_control_vol_and_fluid_component_collections::
        fluid_component_collection::fluid_component_collection::FluidComponentCollection;
    use crate::array_control_vol_and_fluid_component_collections::
        fluid_component_collection::fluid_component_collection::FluidComponentCollectionOreintation;
    use crate::array_control_vol_and_fluid_component_collections::
        fluid_component_collection::fluid_component::FluidComponent;
    use uom::si::mass_rate::kilogram_per_second;
    use crate::array_control_vol_and_fluid_component_collections::
        fluid_component_collection::fluid_component_collection::FluidComponentCollectionMethods;

    use uom::si::pressure::pascal;
    use super::new_pipe_3_relap_model;
    use super::new_pipe_4;
    use super::new_branch_5;
    // first let's construct the heater branch
    // probably need the heater top and bottom head later
    let initial_temperature = ThermodynamicTemperature::new::<degree_celsius>(21.7);
    
    let branch_5 = new_branch_5(initial_temperature);
    let pipe_4 = new_pipe_4(initial_temperature);
    let pipe_3 = new_pipe_3_relap_model(initial_temperature);

    let static_mixer_2 = new_static_mixer_10_label_2(initial_temperature);
    let static_mixer_pipe_2a = new_pipe_2a(initial_temperature);
    let heater_top_head_1a = new_heater_top_head_1a(initial_temperature);
    let heated_section_1 = new_heated_section_version_1_label_1_without_inner_annular_pipe(initial_temperature);
    let heater_bottom_head_1b = new_heater_bottom_head_1b(initial_temperature);

    let pipe_18 = new_pipe_18(initial_temperature);

    // from top to bottom convention, that is branch 5 to 1b
    // but first, need to convert them into fluid components first 
    let branch_5_component: FluidComponent = 
        FluidComponent::FluidArray(
            branch_5.pipe_fluid_array.clone().try_into().unwrap()
            );

    let pipe_4_component: FluidComponent = 
        FluidComponent::FluidArray(
            pipe_4.pipe_fluid_array.clone().try_into().unwrap()
            );

    let pipe_3_component: FluidComponent = 
        FluidComponent::FluidArray(
            pipe_3.pipe_fluid_array.clone().try_into().unwrap()
            );

    let static_mixer_2_component: FluidComponent = 
        FluidComponent::FluidArray(
            static_mixer_2.pipe_fluid_array.try_into().unwrap()
            );

    let static_mixer_pipe_2a_component: FluidComponent = 
        FluidComponent::FluidArray(
            static_mixer_pipe_2a.pipe_fluid_array.try_into().unwrap()
            );

    let heater_top_head_1a_component: FluidComponent = 
        FluidComponent::FluidArray(
            heater_top_head_1a.pipe_fluid_array.try_into().unwrap()
            );

    let heater_1_component: FluidComponent = 
        FluidComponent::FluidArray(
            heated_section_1.pipe_fluid_array.try_into().unwrap()
            );

    let heater_bottom_head_1b_component: FluidComponent = 
        FluidComponent::FluidArray(
            heater_bottom_head_1b.pipe_fluid_array.try_into().unwrap()
            );

    let pipe_18_component: FluidComponent = 
        FluidComponent::FluidArray(
            pipe_18.pipe_fluid_array.try_into().unwrap()
            );


    let heater_branch = 
        FluidComponentCollection{
            components: vec![
                branch_5_component,
                pipe_4_component,
                pipe_3_component,
                static_mixer_2_component,
                static_mixer_pipe_2a_component,
                heater_top_head_1a_component,
                heater_1_component,
                heater_bottom_head_1b_component,
                pipe_18_component
            ],
            orientation: FluidComponentCollectionOreintation::Series,
        };


    // let's check the hydrostatic pressure, 0.0 kg/s fluid flow 
    {
        // now let's push a 0.1kg/s fluid flow through this pipe series
        //
        let pipe_fluid_flow = MassRate::new::<kilogram_per_second>(0.0);

        // and then let's get the pressure change

        let series_pipe_pressure_change = heater_branch.
            get_pressure_change(pipe_fluid_flow);

        // pressure change is around 39041 Pa
        approx::assert_relative_eq!(
            series_pipe_pressure_change.get::<pascal>(),
            39041.0,
            max_relative=0.001);
    }
    // now let's push a 0.1kg/s fluid flow through this pipe series
    //
    let pipe_fluid_flow = MassRate::new::<kilogram_per_second>(0.1);

    // and then let's get the pressure change

    let series_pipe_pressure_change = heater_branch.
        get_pressure_change(pipe_fluid_flow);

    // pressure change is around 36451 Pa
    approx::assert_relative_eq!(
        series_pipe_pressure_change.get::<pascal>(),
        36451.0,
        max_relative=0.001);
}


/// checks if the ctah branch is working correctly
/// pressure change from mass flowrate
/// when we add various components to a series FluidComponentCollection 
/// this is a regression test
#[test]
pub fn ctah_branch_test(){

    use crate::pre_built_components::
        ciet_isothermal_test_components::*;
    use crate::array_control_vol_and_fluid_component_collections::
        fluid_component_collection::fluid_component_collection::FluidComponentCollection;
    use uom::si::mass_rate::kilogram_per_second;
    use crate::array_control_vol_and_fluid_component_collections::
        fluid_component_collection::fluid_component_collection::FluidComponentCollectionMethods;

    use uom::si::pressure::pascal;
    // first let's construct the ctah branch
    // this is pipe 6 all the way to branch 17
    let initial_temperature = ThermodynamicTemperature::new::<degree_celsius>(21.7);

    let static_mixer_41_label_6 = new_static_mixer_41_label_6(initial_temperature);
    let pipe_6a = new_pipe_6a(initial_temperature);
    let ctah_vertical_label_7a = new_inactive_ctah_vertical(initial_temperature);
    let ctah_horizontal_label_7b = new_inactive_ctah_horizontal(initial_temperature);
    let pipe_8a = new_pipe_8a(initial_temperature);
    let static_mixer_40_label_8 = new_static_mixer_40_label_8(initial_temperature);
    let pipe_9 = new_pipe_9(initial_temperature);
    let pipe_10 = new_pipe_10(initial_temperature);
    let pipe_11 = new_pipe_11(initial_temperature);
    let pipe_12 = new_pipe_12(initial_temperature);
    let ctah_pump = new_ctah_pump(initial_temperature);
    let pipe_13 = new_pipe_13(initial_temperature);
    let pipe_14 = new_pipe_14(initial_temperature);
    let flowmeter_40_14a = new_flowmeter_40_14a(initial_temperature);
    let pipe_15 = new_pipe_15(initial_temperature);
    let pipe_16 = new_pipe_16(initial_temperature);
    let branch_17 = new_branch_17(initial_temperature);


    // now I want to add each of these to the fluid component 
    // collection without the constant hassle of having to convert types

    let mut ctah_branch = FluidComponentCollection::new_series_component_collection();
    
    ctah_branch.clone_and_add_component(&static_mixer_41_label_6);
    ctah_branch.clone_and_add_component(&pipe_6a);
    ctah_branch.clone_and_add_component(&ctah_vertical_label_7a);
    ctah_branch.clone_and_add_component(&ctah_horizontal_label_7b);
    ctah_branch.clone_and_add_component(&pipe_8a);
    ctah_branch.clone_and_add_component(&static_mixer_40_label_8);
    ctah_branch.clone_and_add_component(&pipe_9);
    ctah_branch.clone_and_add_component(&pipe_10);
    ctah_branch.clone_and_add_component(&pipe_11);
    ctah_branch.clone_and_add_component(&pipe_12);
    ctah_branch.clone_and_add_component(&ctah_pump);
    ctah_branch.clone_and_add_component(&pipe_13);
    ctah_branch.clone_and_add_component(&pipe_14);
    ctah_branch.clone_and_add_component(&flowmeter_40_14a);
    ctah_branch.clone_and_add_component(&pipe_15);
    ctah_branch.clone_and_add_component(&pipe_16);
    ctah_branch.clone_and_add_component(&branch_17);

    // let's check the hydrostatic pressure, 0.0 kg/s fluid flow 
    {
        let pipe_fluid_flow = MassRate::new::<kilogram_per_second>(0.0);

        // and then let's get the pressure change

        let series_pipe_pressure_change = ctah_branch.
            get_pressure_change(pipe_fluid_flow);

        // pressure change is around 39041 Pa
        approx::assert_relative_eq!(
            series_pipe_pressure_change.get::<pascal>(),
            39041.0,
            max_relative=0.001);
    }
}




/// this is a isothermal pressure drop 
/// validation test against experimental data for the 
/// ctah and heater branch
///
#[test]
pub fn isothermal_ctah_and_heater_branch_validation_test(){

    /// the main difference with validation is that this 
    /// compares the results of the present solver to that of the 
    /// previous solver

    // let's construct the branches with test pressures and obtain 
    // mass flowrates
    use uom::si::f64::*;
    use super::ciet_branch_builders_isothermal::heater_branch_builder_isothermal_test;
    use super::ciet_branch_builders_isothermal::ctah_branch_builder_isothermal_test;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::pressure::pascal;
    use uom::si::thermodynamic_temperature::degree_celsius;

    fn validate_mass_flowrate_given_pressure_change(
        test_pressure: Pressure,
        expected_mass_flow: MassRate){
        let validation_temperature = 
            ThermodynamicTemperature::new::<degree_celsius>(20.0);
        let heater_branch = heater_branch_builder_isothermal_test(validation_temperature);
        let ctah_branch = ctah_branch_builder_isothermal_test(
            test_pressure,
            validation_temperature);

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
        let pressure_change_across_each_branch = 
            FluidComponentSuperCollection::
            isothermal_ciet_solve_pressure_change_across_each_branch_for_ctah_and_heater_branch(
                &ctah_and_heater_branches);




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
        } else if mass_flowrate_test.get::<kilogram_per_second>() > 0.05 {
            // max error is 5% 
            //
            // flowmeter reading error is 2%
            // plus 50 pascals worth of mass flowrate,
            // plus additional 3 % due to correlation error as the 
            // experimental correlation is derived from data points read from 
            // a graph
            //
            // give or take, 5% is reasonable
            //
            // according to the residual plots I plotted, anything below 
            // mass flowrate of 0.05 would have error bars of about 80 Pa
            //

            approx::assert_relative_eq!(
                mass_flowrate_test.get::<kilogram_per_second>().abs(),
                expected_mass_flow.get::<kilogram_per_second>().abs(),
                max_relative=0.05);

        } else {

            // for flowrates less than 0.05 kg/s
            // larger error allowance is given

            approx::assert_relative_eq!(
                mass_flowrate_test.get::<kilogram_per_second>().abs(),
                expected_mass_flow.get::<kilogram_per_second>().abs(),
                max_relative=0.12);

        }

        

    }

    // now let's validate this across all flowrates 

    validate_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(0.0), 
        MassRate::new::<kilogram_per_second>(0.0)
        );
    validate_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(90.0), 
        MassRate::new::<kilogram_per_second>(0.00263)
        );
    validate_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(180.0), 
        MassRate::new::<kilogram_per_second>(0.00527)
        );
    validate_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(480.0), 
        MassRate::new::<kilogram_per_second>(0.0127)
        );
    validate_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(970.0), 
        MassRate::new::<kilogram_per_second>(0.0236)
        );
    validate_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(1960.0), 
        MassRate::new::<kilogram_per_second>(0.0418)
        );
    validate_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(3950.0), 
        MassRate::new::<kilogram_per_second>(0.0706)
        );
    validate_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(5940.0), 
        MassRate::new::<kilogram_per_second>(0.0938)
        );
    validate_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(7930.0), 
        MassRate::new::<kilogram_per_second>(0.114)
        );
    validate_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(9930.0), 
        MassRate::new::<kilogram_per_second>(0.132)
        );
    validate_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(11930.0), 
        MassRate::new::<kilogram_per_second>(0.148)
        );
    validate_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(14920.0), 
        MassRate::new::<kilogram_per_second>(0.170)
        );
    validate_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(15920.0), 
        MassRate::new::<kilogram_per_second>(0.177)
        );
    
    //// reverse flow tests (later)
    //validate_mass_flowrate_given_pressure_change(
    //    Pressure::new::<pascal>(-2000.0), 
    //    MassRate::new::<kilogram_per_second>(-0.0418)
    //    );
    //validate_mass_flowrate_given_pressure_change(
    //    Pressure::new::<pascal>(-10000.0), 
    //    MassRate::new::<kilogram_per_second>(-0.132)
    //    );

}


/// this is a isothermal pressure drop 
/// validation test against experimental data for the 
/// ctah and heater branch
///
/// This is a regression test against older versions of 
/// TUAS 
///
#[test]
pub fn isothermal_ctah_and_heater_branch_code_to_code_verification_test(){

    // let's construct the branches with test pressures and obtain 
    // mass flowrates
    use uom::si::f64::*;
    use super::ciet_branch_builders_isothermal::heater_branch_builder_isothermal_test;
    use super::ciet_branch_builders_isothermal::ctah_branch_builder_isothermal_test;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::pressure::pascal;
    use uom::si::thermodynamic_temperature::degree_celsius;

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
        let pressure_change_across_each_branch = 
            FluidComponentSuperCollection::
            isothermal_ciet_solve_pressure_change_across_each_branch_for_ctah_and_heater_branch(
                &ctah_and_heater_branches);




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

/// specific functions and traits meant to solve mass flowrate 
/// and pressure drop for CIET only
pub trait IsothermalCIETSolvers{

    /// assuming the branch only contains the ctah and heater branch 
    /// and the max flowrate is 0.25 kg/s
    ///
    /// we can solve for pressure change across each of the branches
    /// 
    fn isothermal_ciet_solve_pressure_change_across_each_branch_for_ctah_and_heater_branch(
        ctah_and_heater_branch: &FluidComponentSuperCollection) -> Pressure {

        let pressure_change_root = |pressure_change_pascals: f64| -> f64 {
            //# both branches must be subject to the same
            //# pressure change since they are in parallel

            // given a pressure change, obtain the mass flowrate

            let mass_flowrate_vector_kg_per_s: Vec<f64> = 
                ctah_and_heater_branch
                .fluid_component_super_vector
                .iter()
                .map(|branch: &FluidComponentCollection|{

                    let pressure_change: Pressure = 
                        Pressure::new::<pascal>(pressure_change_pascals);

                    let branch_mass_flowrate = 
                        branch.get_mass_flowrate_from_pressure_change(
                            pressure_change);

                    branch_mass_flowrate.get::<kilogram_per_second>()


                })
            .collect();


            let total_mass_flowrate_ks_per_s: f64 = 
                mass_flowrate_vector_kg_per_s.into_iter().sum();

            return total_mass_flowrate_ks_per_s;
        };

        // let's get hydrostatic pressure (and pressure sources) next
        // I can pick any branch to get hydrostatic pressure
        //
        // So I'll
        // get the first element of the vector
        //
        // then apply zero mass flowrate
        let hydrostatic_pressure: Pressure = 
            ctah_and_heater_branch
            .fluid_component_super_vector
            .first() 
            .unwrap()
            .get_pressure_change(MassRate::ZERO); // 

        let upper_bound: Pressure = 
            hydrostatic_pressure +
            Pressure::new::<pascal>(50000_f64);

        let lower_bound: Pressure = 
            hydrostatic_pressure +
            Pressure::new::<pascal>(-50000_f64);


        let mut convergency = SimpleConvergency { eps:1e-12_f64, max_iter:30 };


        let pressure_change_pascals 
            = find_root_brent(
                upper_bound.get::<pascal>(),
                lower_bound.get::<pascal>(),
                &pressure_change_root,
                &mut convergency).unwrap();


        Pressure::new::<pascal>(pressure_change_pascals)


    }

}

impl IsothermalCIETSolvers for FluidComponentSuperCollection {
}

/// basically, for all smaller unit tests mean to debug the flow simulation
pub mod debugging_tests;
