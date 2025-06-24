use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::collection_series_and_parallel_functions::FluidComponentCollectionSeriesAssociatedFunctions;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component::FluidComponent;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_collection::FluidComponentCollection;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_collection::FluidComponentCollectionMethods;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_super_collection::FluidComponentSuperCollection;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::super_collection_series_and_parallel_functions::FluidComponentSuperCollectionParallelAssociatedFunctions;
use crate::pre_built_components::gfhr_pipe_tests::multi_branch::multi_branch_solvers::calculate_pressure_change_using_guessed_branch_mass_flowrate_fhr_sim_v1_custom;
use crate::pre_built_components::insulated_pipes_and_fluid_components::InsulatedFluidComponent;
use crate::pre_built_components::non_insulated_fluid_components::NonInsulatedFluidComponent;
use roots::find_root_brent;
use roots::find_root_inverse_quadratic;
use roots::find_root_regula_falsi;
use roots::SimpleConvergency;
use uom::si::mass_rate::kilogram_per_second;
use uom::si::pressure::pascal;
use uom::ConstZero;
use uom::si::f64::*;

/// for the gFHR primary loop,
/// there are four branches that need to be solved for flowrate 
///
/// this code handles the solution procedure
pub fn four_branch_pri_loop_flowrates_parallel(
    pump_pressure: Pressure,
    // reactor branch
    reactor_pipe_1: &InsulatedFluidComponent,
    // downcomer branch 1
    downcomer_pipe_2: &InsulatedFluidComponent,
    // downcomer branch 2
    downcomer_pipe_3: &InsulatedFluidComponent,
    // Intermediate heat exchanger branch 
    fhr_pipe_4: &InsulatedFluidComponent,
    fhr_pri_loop_pump: &NonInsulatedFluidComponent
    ) -> (MassRate, MassRate, MassRate, MassRate,){

    // note: this crashes due to non convergency issues...
    //thread '<unnamed>' panicked at C:\Users\fifad\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tuas_boussinesq_solver-0.0.7\src\lib\array_control_vol_an
    //d_fluid_component_collections\fluid_component_collection\collection_series_and_parallel_functions.rs:444:74:
    //called `Result::unwrap()` on an `Err` value: NoConvergency
    //note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //
    //
    //now, even by having more flowrate options, I'm still getting a no 
    //convergency error especially once pump pressure exceeds 
    // 10.0 - 13.0 Pa, and there is actually flowrate
    // I'm getting flowrates in excess of 10 kg/s 
    // 554 kg/s, and thats okay 
    //
    // but then I get NoConvergency errors

    let mut reactor_branch = 
        FluidComponentCollection::new_series_component_collection();

    reactor_branch.clone_and_add_component(reactor_pipe_1);




    let mut downcomer_branch_1 = 
        FluidComponentCollection::new_series_component_collection();

    downcomer_branch_1.clone_and_add_component(downcomer_pipe_2);




    let mut downcomer_branch_2 = 
        FluidComponentCollection::new_series_component_collection();

    downcomer_branch_2.clone_and_add_component(downcomer_pipe_3);




    let mut intermediate_heat_exchanger_branch =
        FluidComponentCollection::new_series_component_collection();

    let mut fhr_pipe_4_clone = fhr_pipe_4.clone();
    fhr_pipe_4_clone.set_internal_pressure_source(pump_pressure);
    intermediate_heat_exchanger_branch.clone_and_add_component(&fhr_pipe_4_clone);
    //let mut fhr_pump_clone: NonInsulatedFluidComponent 
    //    = fhr_pri_loop_pump.clone();
    //fhr_pump_clone.set_internal_pressure_source(pump_pressure);
    //intermediate_heat_exchanger_branch.clone_and_add_component(&fhr_pump_clone);


    

    let mut pri_loop_branches = 
        FluidComponentSuperCollection::default();

    pri_loop_branches.set_orientation_to_parallel();

    pri_loop_branches.fluid_component_super_vector.push(reactor_branch);
    pri_loop_branches.fluid_component_super_vector.push(downcomer_branch_1);
    pri_loop_branches.fluid_component_super_vector.push(downcomer_branch_2);
    pri_loop_branches.fluid_component_super_vector.push(intermediate_heat_exchanger_branch);

    let (reactor_branch_flow, downcomer_branch_1_flow,
        downcomer_branch_2_flow, intermediate_heat_exchanger_branch_flow)
        = calculate_iterative_mass_flowrate_across_branches_for_fhr_sim_v1(
            &pri_loop_branches);


    return (reactor_branch_flow, downcomer_branch_1_flow,
        downcomer_branch_2_flow, intermediate_heat_exchanger_branch_flow);
}
// debug log: 
// 
// thought it's a fluid properties bug
// tried changing from FLiBe to HITEC but not working
//
// basically from what I observed, the mass flowrate solvers 
// do get stuck at some value and yet do not converge there...
//
// I believe the tolerance is an issue... 
// so upon reducing tolerance, the mass flowrates are able to solve and  
// converge!
// but this produces wonky results
//
/// calculates pressure change given a mass
/// flowrate through a parallel collection of
/// fluid pipes or components
///
/// TODO: needs work and testing, doesn't work now
pub fn calculate_iterative_mass_flowrate_across_branches_for_fhr_sim_v1(
    fluid_component_super_collection: 
    &FluidComponentSuperCollection) -> (MassRate,
    MassRate, MassRate, MassRate) {

        let mass_flowrate = MassRate::ZERO;
        let fluid_component_collection_vector = 
            fluid_component_super_collection.get_immutable_vector();

        // for calculating pressure change in a parallel super
        // collection from
        // mass flowrate, 
        // i will need to iteratively guess the pressure change
        // across each pipe to get the specified mass flowrate

        // only thing is how do i do so?
        //
        // First thing first, I will need to guess some bounds for the brent
        // calculator, ie what pressure change bounds are appropriate?
        //
        // There are no standardised pressure change bounds for any of
        // these
        //
        // Nevertheless, they can be calculated,
        //
        // For reference, at zero mass flowrate, each parallel branch would
        // have a default pressure change. This may differ for each
        // branch. 
        //
        // taking the average of these pressure changes at zero flow case
        // i would get a pretty good guess of what the pressure change may
        // be like at zero flow
        //
        // this will then be my starting point and if i bound it by
        // the change between maximum and minimum pressure,
        // i should be able to get my bounds for zero flow
        // this case is simpler
        //
        //
        //
        //
        //
        // And then, when I supply a mass flowrate for each of these branches
        // there would be some pressure losses associated with that
        // mass flowrate
        // Again, the pressure losses expected from each branch would
        // be different
        //
        // since i supply a mass flowrate here already, I can use this
        // combined mass flowrate through all pipes
        //
        // the minimum pressure loss from any one of these branches
        // and subtract that from the maximum pressure loss
        //
        //
        //
        // This will form a pressure bound which i can plus and minus
        // minus from my default pressure change
        // 
        // Lastly, I need to add the difference between the maximum
        // and minimum of the pressure change at zero flow
        // perhaps multiply that by 1.5 to obtain pressure bounds as
        // well
        //
        // In this way, both flows due to pressure changes outside the      
        // parallel branches
        // and changes inside the parallel branches are accounted for
        //
        // in dynamic setting of bounds. 
        // and this should provide decent-ish initial guesses
        //

        // if mass flowrate over this series is zero, then we can calculate the bound
        // straightaway

        let user_requested_mass_flowrate = 
            mass_flowrate;

        let zero_mass_flowrate = 
            MassRate::new::<kilogram_per_second>(0.0);

        // if the mass flowrate is almost zero (1e-9 kg/s)
        // we assume flow is zero 
        // this is zero NET flow through the parallel structure
        // the branches themselves may still have flow going 
        // through them
        if user_requested_mass_flowrate.value.abs() < 1e-9_f64 {

            dbg!("zero flowrate through parallel branches");
            // in this case, the average mass flowrate through each of these
            // loops is very close to zero,
            //
            //
            // for a trivial solution zero flowrate is supplied
            // as a guess
            //
            // That is we have zero mass flowrate through the network 
            // of branches, 
            //
            // the easiest solution is each branch has zero mass flowrate


            // however, more often than not, the trivial solution doesn't work
            // I then need to obtain the largest difference in pressure changes 
            // between each branch if it has zero flow rate
            // we can get the max pressure difference between each branch 
            //
            let max_pressure_change_between_branches: Pressure = 
                <FluidComponentSuperCollection as FluidComponentSuperCollectionParallelAssociatedFunctions>:: 
                calculate_maximum_pressure_difference_between_branches(
                    zero_mass_flowrate, 
                    &fluid_component_collection_vector
                );


            // with this max pressure change, we can guess a maximum 
            // flowrate across each branch
            dbg!(&max_pressure_change_between_branches);
            dbg!("calculating max flow between brances");

            let debugging = true;
            // TODO: this is a temp fix
            let mut max_mass_flowrate_across_each_branch = 
                MassRate::new::<kilogram_per_second>(5000.0);
            // TODO: this is the buggy spot
            if !debugging {
                max_mass_flowrate_across_each_branch = 
                    <FluidComponentSuperCollection as FluidComponentSuperCollectionParallelAssociatedFunctions>:: 
                    calculate_maximum_mass_flowrate_given_pressure_drop_across_each_branch(
                        max_pressure_change_between_branches, 
                        &fluid_component_collection_vector);
            }
            // the above is an algorithm which allows us to calculate an 
            // upper limit for mass flowrate across each branch
            //
            // this may fail (or not) given a pressure bound 
            //
            // to avoid this issue, i can comment this out and set 
            // an upper limit of 5000 kg/s 




            // with a hypothetical mass flowrate across each branch 
            //
            // now we need to change the limits of the pressure change 
            // instead of +/- 10 kg/s to something larger,
            // say 100,000 kg/s
            //
            // I remember that +/- 10 kg/s is for CIET
            // but for FHR, the value is much larger. 
            // perhaps 100,000 kg/s is sufficient
            //
            // this is giving me problems here!
            // calculate_pressure_change_using_guessed_branch_mass_flowrate
            //

            dbg!("calculating pressure chg through branches..");
            // now with pressure change through the branches... 
            // I'm getting an oscillation issue
            // we do get to about zero 
            // -5.68e-13 and 7.105e-13 
            //
            // this is indeed about zero but not quite
            // I think for large mass flowrates, the tolerance is too tight
            //
            // I think normalisation would work. 
            let pressure_change = 
                calculate_pressure_change_using_guessed_branch_mass_flowrate_fhr_sim_v1_custom(
                    max_mass_flowrate_across_each_branch, 
                    user_requested_mass_flowrate, 
                    &fluid_component_collection_vector);

            dbg!("pressure chg calculated");
            dbg!(&pressure_change);


            let mut mass_flowrate_vector: Vec<MassRate> =
                vec![];

            // the mass_flowrate vector will have a length
            // equal to the fluid_component vector

            let new_vector_length =
                fluid_component_collection_vector.len();

            let default_mass_flowrate_value = 
                MassRate::new::<kilogram_per_second>(0.0);

            mass_flowrate_vector.resize(
                new_vector_length,
                default_mass_flowrate_value
            );

            for (index,fluid_component_pointer_collection) in 
                fluid_component_collection_vector.iter().enumerate() {

                    // first we get an immutable reference from
                    // the mutable reference

                    let fluid_component_collection = 
                        &*fluid_component_pointer_collection;


                    
                    dbg!("calculating mass flowrate for...");
                    dbg!(&fluid_component_collection);

                    let fluid_component_mass_flowrate = 
                        fluid_component_collection.get_mass_flowrate_from_pressure_change(
                            pressure_change);
                    dbg!(&fluid_component_mass_flowrate);

                    mass_flowrate_vector[index] = 
                        fluid_component_mass_flowrate;

            }

            // for fhr, sim specifically, we have 4 branches

            return (mass_flowrate_vector[0],
                mass_flowrate_vector[1],
                mass_flowrate_vector[2],
                mass_flowrate_vector[3]);
        }


        // if flow is non zero, then we will have to deal with 3 bounding cases
        // so that we can guess the bounds of root finding
        //
        // First case is where 
        // the internal circulation effect >> external flow 
        //
        // This will be similar to the zero pressure mass flowrate algorithm
        //
        // in that one can simply apply that mass flowrate
        // to all the branches, 
        //
        // assume that the pressure change will lie somewhere between
        // the pressure changes obtained in the various branches
        //
        // and use the maximum and minimum pressure changes to obtain bounds
        // and the solution to the equation
        //
        // For this to work, we know that the scale of the internal circulation
        // driving force is perhaps (max pressure change - min pressure change)
        //
        // if the maximum pressure loss caused by the flow is within 10% of
        // this driving force, i can say that case 1 applies. This is just
        // a guestimate
        //
        // So let's first get the zero mass flowrate pressure force measured


        // step 1: let's first get the pressure changes at
        // mass flowrate = 0.0
        //


        let zero_flow_pressure_change_est_vector = 
            <FluidComponentSuperCollection as FluidComponentSuperCollectionParallelAssociatedFunctions>::
            obtain_pressure_estimate_vector(
                zero_mass_flowrate, 
                &fluid_component_collection_vector);



        let max_pressure_change_at_zero_flow = 
            <FluidComponentSuperCollection as FluidComponentSuperCollectionParallelAssociatedFunctions>::
            obtain_maximum_pressure_from_vector(
                &zero_flow_pressure_change_est_vector);

        let min_pressure_change_at_zero_flow = 
            <FluidComponentSuperCollection as FluidComponentSuperCollectionParallelAssociatedFunctions>::
            obtain_minimum_pressure_from_vector(
                &zero_flow_pressure_change_est_vector);

        let internal_circulation_driving_force_scale = 
            max_pressure_change_at_zero_flow -
            min_pressure_change_at_zero_flow;

        // step 2: now i'll apply the user_specified flowrate to all the branches
        // and calculate pressure loss

        let user_specified_flow_pressure_loss_est_vector = 
            <FluidComponentSuperCollection as FluidComponentSuperCollectionParallelAssociatedFunctions>::
            obtain_pressure_loss_estimate_vector(
                user_requested_mass_flowrate, 
                &fluid_component_collection_vector);

        // note that these pressure loss values are likely positive
        // even if not though, what i'm looking for here is the
        // largest magnitude of all these pressure losses

        // to get a sense of the scale, i'm going to look for the average,
        // minimum and maximum pressure drop

        let user_specified_average_pressure_drop =
            <FluidComponentSuperCollection as FluidComponentSuperCollectionParallelAssociatedFunctions>::
            obtain_average_pressure_from_vector(
                &user_specified_flow_pressure_loss_est_vector);



        // now i can compare the magnitude of the internal driving force
        // to the user_specified_average_pressure_drop
        //
        // if the average pressure drop is <10% or the internal driving force,
        // then we can consider this a internal circulation dominant case

        let internal_circulation_dominant = 
            internal_circulation_driving_force_scale.value * 10.0 
            > user_specified_average_pressure_drop.value.abs();

        if internal_circulation_dominant {

            // in this case, the average mass flowrate through each of these
            // loops is very close to zero,
            // therefore zero flowrate is supplied
            // as a guess


            // we can get the max pressure difference between each branch 
            let max_pressure_change_between_branches: Pressure = 
                <FluidComponentSuperCollection as FluidComponentSuperCollectionParallelAssociatedFunctions>:: 
                calculate_maximum_pressure_difference_between_branches(
                    zero_mass_flowrate, 
                    &fluid_component_collection_vector
                );

            // with this max pressure change, we can guess a maximum 
            // flowrate across each branch

            let max_mass_flowrate_across_each_branch = 
                <FluidComponentSuperCollection as FluidComponentSuperCollectionParallelAssociatedFunctions>:: 
                calculate_maximum_mass_flowrate_given_pressure_drop_across_each_branch(
                    max_pressure_change_between_branches, 
                    &fluid_component_collection_vector);

            // with this maximum mass flowrate, one should be able to get 
            // pressure drop bounds for the branches

            let pressure_change = 
                <FluidComponentSuperCollection as FluidComponentSuperCollectionParallelAssociatedFunctions>::
                calculate_pressure_change_using_guessed_branch_mass_flowrate(
                    max_mass_flowrate_across_each_branch, 
                    user_requested_mass_flowrate, 
                    &fluid_component_collection_vector);
            let mut mass_flowrate_vector: Vec<MassRate> =
                vec![];

            // the mass_flowrate vector will have a length
            // equal to the fluid_component vector

            let new_vector_length =
                fluid_component_collection_vector.len();

            let default_mass_flowrate_value = 
                MassRate::new::<kilogram_per_second>(0.0);

            mass_flowrate_vector.resize(
                new_vector_length,
                default_mass_flowrate_value
            );

            for (index,fluid_component_pointer) in 
                fluid_component_collection_vector.iter().enumerate() {

                    // first we get an immutable reference from
                    // the mutable reference

                    let fluid_component = 
                        &*fluid_component_pointer;


                    let fluid_component_mass_flowrate = 
                        fluid_component.get_mass_flowrate_from_pressure_change(
                            pressure_change);

                    mass_flowrate_vector[index] = 
                        fluid_component_mass_flowrate;

                }

            // for fhr, sim specifically, we have 4 branches

            return (mass_flowrate_vector[0],
                mass_flowrate_vector[1],
                mass_flowrate_vector[2],
                mass_flowrate_vector[3]);
        }

        // next we can go to the other extreme, where external flowrate is 
        // dominant,
        //
        // in such a case, the internal circulation driving force (at zero flow)
        // is much smaller <10% of the external pressure driving force
        // which can be specified by the user specified average pressure drop
        // value
        //


        let external_circulation_dominant = 
            internal_circulation_driving_force_scale.value * 10.0 
            < user_specified_average_pressure_drop.value.abs();

        if external_circulation_dominant {

            // in such a case, the average guessed flowrate should be 
            // the total mass flowrate divided by the number of branches

            // this is not implemented for this case

            todo!();

        }

        // now that we've covered both of the extreme cases, we can check the third
        // case where the internal circulation force and external circulation force
        // both cannot be neglected
        //
        // in such a case, we expect the pressure change to be large enough
        // to be able to block flow in any one of the tubes

        // so it may be likely that flow in any one of those tubes is zero or
        // close to zero because some of the flow in those tubes are blocked by
        // the external pressure drop
        //
        // if scales are similar (and non zero, because we already handled the
        // zero case)
        //
        // we can take the internal driving force as a reference scale
        // calculate then 

        let pressure_deviation_percentage_from_internal_driving_force =
            (internal_circulation_driving_force_scale - 
             user_specified_average_pressure_drop).value.abs()
            /internal_circulation_driving_force_scale.value.abs()
            *100.0_f64;

        // if the deviation percentage is less than 80%, we can say they are quite
        // in the same order of magnitude or similar

        if pressure_deviation_percentage_from_internal_driving_force < 80.0 {


            // in this case, the guessed mass flowrate through each of these
            // loops can be very close to zero,
            // therefore zero flowrate is supplied
            // as a guess
            // the algorithm is similar to the internal pressure dominant
            // case,
            // but the reasoning is different
            //
            // this is not implemented for this case

            todo!();




        }

        // now if all of the cases are exhausted, we will just resort to a generic
        // method where the guessed flowrate for each branch is the 
        // user supplied mass flowrate/number of branches
        //
        // hopefully this will supply the correct pressure bounds to
        // guess the pressure change
        //
        // this is not implemented for this case

        todo!();

    }


