



/// basically, for all smaller unit tests mean to debug the flow simulation
pub mod debugging_tests;

#[test]
pub fn isothermal_dhx_ctah_and_heater_branch_code_to_code_verification_test(){
    /// the main difference with validation is that this 
    /// compares the results of the present solver to that of the 
    /// previous solver

    // let's construct the branches with test pressures and obtain 
    // mass flowrates
    use crate::pre_built_components::
        ciet_isothermal_test_components::
        isothermal_ctah_heater_branch_test::IsothermalCIETSolvers;
    use crate::array_control_vol_and_fluid_component_collections::
        fluid_component_collection::fluid_component_super_collection::FluidComponentSuperCollection;
    use uom::si::f64::*;
    use super::ciet_branch_builders_isothermal::heater_branch_builder_isothermal_test;
    use super::ciet_branch_builders_isothermal::ctah_branch_builder_isothermal_test;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::pressure::pascal;
    use crate::pre_built_components::
        ciet_isothermal_test_components::
        ciet_branch_builders_isothermal::dhx_branch_builder_isothermal_test;
    use uom::si::thermodynamic_temperature::degree_celsius;

    fn verify_mass_flowrate_given_pressure_change(
        test_pressure: Pressure,
        expected_mass_flow: MassRate){
        let verification_temperature = 
            ThermodynamicTemperature::new::<degree_celsius>(20.0);
        let heater_branch = heater_branch_builder_isothermal_test(
            verification_temperature);
        let ctah_branch = ctah_branch_builder_isothermal_test(
            test_pressure,
            verification_temperature);
        let dhx_branch = dhx_branch_builder_isothermal_test(verification_temperature);

        // expected flowrate 

        // you'll now need to add this into a super collection
        let mut dhx_ctah_and_heater_branches = 
            FluidComponentSuperCollection::default();
        dhx_ctah_and_heater_branches.set_orientation_to_parallel();
        dhx_ctah_and_heater_branches.fluid_component_super_vector
            .push(heater_branch);
        dhx_ctah_and_heater_branches.fluid_component_super_vector
            .push(ctah_branch);
        dhx_ctah_and_heater_branches.fluid_component_super_vector
            .push(dhx_branch);

        // obtain flowrate 
        // the overall pressure change across each branch must be
        // equal
        //
        // the total mass flowrate through the collection of parallel 
        // branches is zero
        let pressure_change_across_each_branch = 
            FluidComponentSuperCollection::
            isothermal_ciet_solve_pressure_change_across_each_branch_for_ctah_and_heater_branch(
                &dhx_ctah_and_heater_branches);




        // once we have pressure change across each branch,
        // then we can calculate mass flowrate.

        let mass_flowrate_across_each_branch: Vec<MassRate> = 
            dhx_ctah_and_heater_branches.
            get_mass_flowrate_across_each_parallel_branch(
                pressure_change_across_each_branch
            );


        // let's obtain the mass flowrate, The first one is the heater branch
        // the second one is the ctah branch, 
        // for that matter, we are more interested in the ctah branch

        let mut mass_flowrate_test: MassRate = 
            mass_flowrate_across_each_branch[1];


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
            // give or take, 2% is reasonable
            //
            // according to the residual plots I plotted, anything below 
            // mass flowrate of 0.02 would have error bars of about 80 Pa
            //
            // but this agrees to within 0.5% which is reasonable for reproducing 
            // test results
            //

            approx::assert_relative_eq!(
                mass_flowrate_test.get::<kilogram_per_second>().abs(),
                expected_mass_flow.get::<kilogram_per_second>().abs(),
                max_relative=0.005);

        } else {

            // for flowrates less than 0.05 kg/s
            // we agree to within 0.5%

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
        MassRate::new::<kilogram_per_second>(0.00396)
        );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(200.0), 
        MassRate::new::<kilogram_per_second>(0.00796)
        );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(500.0), 
        MassRate::new::<kilogram_per_second>(0.0185)
        );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(1000.0), 
        MassRate::new::<kilogram_per_second>(0.0329)
        );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(2000.0), 
        MassRate::new::<kilogram_per_second>(0.0552)
        );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(4000.0), 
        MassRate::new::<kilogram_per_second>(0.0885)
        );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(6000.0), 
        MassRate::new::<kilogram_per_second>(0.115)
        );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(8000.0), 
        MassRate::new::<kilogram_per_second>(0.137)
        );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(10000.0), 
        MassRate::new::<kilogram_per_second>(0.157)
        );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(12000.0), 
        MassRate::new::<kilogram_per_second>(0.175)
        );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(15000.0), 
        MassRate::new::<kilogram_per_second>(0.199)
        );
    verify_mass_flowrate_given_pressure_change(
        Pressure::new::<pascal>(16000.0), 
        MassRate::new::<kilogram_per_second>(0.207)
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
