use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::collection_series_and_parallel_functions::FluidComponentCollectionSeriesAssociatedFunctions;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component::FluidComponent;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_collection::FluidComponentCollection;
use roots::find_root_brent;
use roots::SimpleConvergency;
use uom::si::mass_rate::kilogram_per_second;
use uom::si::f64::*;
/// calculates mass flowrate from pressure change
/// for a given fluid component collection
/// it needs a vector of mutable references to
/// any object which implements FluidComponent
pub fn calculate_mass_flowrate_from_pressure_change_for_single_branch_fhr_sim_custom(
    pressure_change: Pressure,
    fluid_component_vector: &Vec<FluidComponent>) -> MassRate {

    // a few key issues here:
    //
    // the method i'm going to use here is iteration
    //
    // which means I have to guess a mass flowrate
    // and obtain pressure change until the
    // pressure change matches the desired pressure change
    //
    // How then can I guess it intelligently?
    // without having the user set bounds?
    // 
    // First, we can get a baseline pressure change
    // ie when mass flowrate = 0 
    // 
    // We can then set the mass flowrate > 0  to some amount
    // and mass flowrate < 0 to some amount and 
    // take a look at the trends
    //
    // for newtonian fluid flow, we should infer that
    // higher pressure loss means higher flowrate all else equal
    //
    // for the most part, we don't have osciallting functions
    // or inflexion points for pressure loss vs reynolds number
    //
    //
    // Hence, Newton Raphson should be quite stable in theory
    // 
    //
    // The other method should be bisection, if all else fails
    // I could use mass flowrate = 0 as one bound
    //
    // and an initial bound of mass flowrate = 1kg/s
    //
    // if i find that mass flowrate is more than 1kg/s (unlikely)
    //
    // increase bound by 10
    // and then check again
    //
    // then use 1kg/s as the lower bound and 10 kg/s as the upper bound
    // and then perform bisection (this is a fallback and may
    // tend to be slow)
    //
    // The last issue is how much error to tolerate in terms of
    // pressure change should the pressure change be zero
    //
    // my take is that it should be an absolute value
    // based on a real error scale
    //
    // it should be 1 mm h2o at room temp because
    // this is usually absolute the manotmeter error
    // This is about 9.8 pascals or 10 pascals
    //
    // Therefore, my absolute tolerance should be within 
    // 7 Pa


    // first let's find the pressure change at zero, 1 kg/s
    // and -1 kg/s


    let zero_mass_flow: MassRate 
        = MassRate::new::<kilogram_per_second>(0.0);



    let pressure_change_0kg_per_second: Pressure 
        = <FluidComponentCollection as FluidComponentCollectionSeriesAssociatedFunctions>
        ::calculate_pressure_change_from_mass_flowrate(
            zero_mass_flow, 
            fluid_component_vector);


    // now we will check if the difference is about 9 Pa
    // from zero flow
    // which is that manometer reading error
    //
    // if that is so, then return mass flowrate = 0


    let pressure_loss_pascals = 
        -(pressure_change - pressure_change_0kg_per_second).value;

    if pressure_loss_pascals.abs() < 9_f64 {
        return zero_mass_flow;
    }


    // present issue: 
    // trait objects can be moved (ie used once)
    // but after using, they are finished...
    //
    // i cannot exactly clone them because this is not object
    // safe. Ie, the cloning process cannot know the size
    // of the struct at compile time 
    // traits aren't exactly well suited for 
    // methods which take in the mutable state
    //
    // nevertheless
    //
    // I can extract the state of an object and convert that
    // into a vector with size known at compile time
    //
    // However, with many potential trait objects bearing the same
    // kind of method with different size, and different required
    // data
    //
    // eg. 3 pipes and 1 flowmeter  or variations of these
    //
    // i cannot really know the size of the trait object at compile
    // time, or the required properties they contain
    //
    // The solution then is to use mutable borrows of
    // these objects rather than the actual object itself 
    // which then becomes deleted
    //
    // So then parallelism with trait objects becomes QUITE
    // challenging due to the mutability requirement
    //
    // I just hope they are not really needed =(
    //
    // However, if the functions required do NOT need a mutable
    // reference to self or anything, then we are in good shape
    //
    // Doing so however, we then do not have our usual OOP paradigms
    // where we change object state before invoking a get()
    // function

    // if pressure loss is positive, we have forward flow
    // if pressure loss is negative, we have backflow
    //


    // if forward flow is true, then i want to iteratively calculate 
    // pressure changes using mass flowrates until the limit is reached

    // i'm going to use the peroxide library 
    //


    // this is for use in the roots library
    let mass_flow_from_pressure_chg_root = 
        |mass_flow_kg_per_s: f64| -> f64 {

            let mass_flow_kg_per_s_double = mass_flow_kg_per_s; 

            let mass_rate = 
                MassRate::new::<kilogram_per_second>(
                    mass_flow_kg_per_s_double);


            let pressure_change_tested = 
                <FluidComponentCollection as FluidComponentCollectionSeriesAssociatedFunctions>
                ::calculate_pressure_change_from_mass_flowrate(
                    mass_rate, 
                    fluid_component_vector);
            // now i've obtained the pressure change, i convert it to f64

            let pressure_change_user_stipulated_pascals_f64 = 
                pressure_change.value;

            // since we are finding root, then we must also
            // subtract it from our pressure change value


            let pressure_change_error: f64 =
                pressure_change_user_stipulated_pascals_f64 - 
                pressure_change_tested.value;

            return pressure_change_error;

        };

    // note: this function mutates the value of fluid_component_vector,
    // and is thus incompatible with peroxide libraries...
    // I'll need to rewrite the libraries in terms of immutable functions
    //
    // But having done so, I want to use the newton raphson method to
    // try and converge this result, hopefully within 30 iterations

    let mut convergency = SimpleConvergency { eps:1e-8f64, max_iter:70 };


    let mut mass_flowrate_result 
        = find_root_brent(
            10_f64,
            -10_f64,
            &mass_flow_from_pressure_chg_root,
            &mut convergency);


    // the above results only work for ranges of 15 kg/s and
    // -15 kg/s

    // now if the newton raphson method does not converge within the
    // set number of iterations, I want it to use bisection
    // which should fall back to bisection
    // This function is not meant ot be used by the end user
    // but is instead called by another function
    //
    //
    // this function is expected to take in an automatic
    // differentiation function
    //
    // which takes the following form
    //
    // function(mass_flowrate_kg_per_second) -> pressure_pascals
    //
    // both mass flowrate and pressure are f64 but in SI units
    // as i'm trying to solve pipe network problems and flow in series
    // i consider the highest volume of flow possible for such a system
    //
    // the pressure actually measures the difference between the
    // guessed pressure loss in the iteration
    // and the actual pressure loss specified by the user
    //
    // The guiness book of world records shows that the amazon
    // river has a flowrate of about 200,000 m3/s
    // https://www.guinnessworldrecords.com/world-records/greatest-river-flow
    //
    // in other words about 200,000,000 kg/s
    //
    // We never expect man made 
    // piping systems to have this much flow 
    //
    // But this would be a good upper bound for bisection solver.
    //
    //
    //
    // If we cannot find a root in this range,
    // then it's likely there is no possible root at all
    //
    // the inline thingy here is just to help the code
    // speed up a bit
    //
    // However, I don't want to go to such an upper limit so
    // quickly,
    //
    // I'll do 10,000 kg/s in each flow branch first
    // then 200,000,000


    mass_flowrate_result = 
        match mass_flowrate_result {
            Ok(_mass_flowrate) => 
                return MassRate::new::<kilogram_per_second>(_mass_flowrate),
            Err(_error_msg) => {

                mass_flowrate_result 
                    = find_root_brent(
                        10_000_f64,
                        -10_000_f64,
                        &mass_flow_from_pressure_chg_root,
                        &mut convergency);

                mass_flowrate_result
            }
        };

    mass_flowrate_result = 
        match mass_flowrate_result {
            Ok(_mass_flowrate) => 
                return MassRate::new::<kilogram_per_second>(_mass_flowrate),
            Err(_error_msg) => {

                mass_flowrate_result 
                    = find_root_brent(
                        20_000_000_f64,
                        -20_000_000_f64,
                        &mass_flow_from_pressure_chg_root,
                        &mut convergency);

                mass_flowrate_result
            }
        };
    //return mass_flowrate.unwrap();
    return MassRate::new::<kilogram_per_second>(mass_flowrate_result.unwrap());
}



