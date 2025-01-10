use std::ops::{DerefMut, Deref};
use std::sync::{Arc, Mutex};
use peroxide::prelude::erfc;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::boundary_conditions::BCType;
use crate::boussinesq_thermophysical_properties::thermal_diffusivity::try_get_alpha_thermal_diffusivity;
use crate::boussinesq_thermophysical_properties::{Material, SolidMaterial};
use crate::control_volume_dimensions::XThicknessThermalConduction;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::pre_built_components::heat_transfer_entities::preprocessing::{calculate_timescales_for_heat_transfer_entity, link_heat_transfer_entity};
use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
use crate::tuas_lib_error::TuasLibError;
use uom::si::f64::*;
use uom::si::length::{centimeter, meter};
use uom::si::power::watt;
use uom::si::ratio::ratio;
use uom::si::temperature_interval::degree_celsius as interval_deg_c;
use uom::si::pressure::atmosphere;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::time::second;


/// This is an example of transient conduction case where analytical 
/// solutions have been well known

/// this is basically the same copper medium test,
/// except that now I use an array CVs
/// this is with 10 total nodes (8 inner nodes and two boundary nodes)
///
/// the other test prints csv, this one only asserts
/// 
#[test]
fn arraycv_transient_conduction_copper_medium_with_assert_only() -> Result<(),TuasLibError>{

    // let's do the thread spawn for the calculated solution before 
    // the analytical solution

    // before we start, we need the copper thermal_diffusivity

    let copper = Material::Solid(SolidMaterial::Copper);
    let pressure = Pressure::new::<atmosphere>(1.0);
    let copper_initial_temperature = 
    ThermodynamicTemperature::new::<degree_celsius>(21.67);

    let boundary_condition_temperature = 
    ThermodynamicTemperature::new::<degree_celsius>(80.0);


    // note that diffusivity changes with temperature, but we shall not 
    // assume this is the case, and just obtain an approximate 
    // analytical solution 

    let copper_avg_temperature: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(45.0);
    let copper_thermal_diffusivity_alpha: DiffusionCoefficient 
    = try_get_alpha_thermal_diffusivity(copper, copper_avg_temperature, pressure)?;

    // delta x is the node to node length
    let delta_x: Length = Length::new::<centimeter>(2.0);
    // let's make the up to 0.20 m of nodes, 
    // we have 10 nodes in all, so each node has
    // about 2cm of thermal resistance between the nodes
    //
    // then specify each node is made of copper, 
    // it shall be a 1D kind of conduction node
    // for 1D conduction type nodes, we just take 1m^2 area as a basis 
    // and apply adiabatic BC to the surface normal 
    // 
    // so I'll need to make 1 BC and 10 control volumes
    // simulate the transient temperatures for up to 20s

    let copper_array_cv: HeatTransferEntity 
        = SolidColumn::new_one_dimension_volume(
            Length::new::<centimeter>(20.0),
            copper_initial_temperature,
            pressure,
            copper.try_into()?,
            8,).into();

    let array_cv_pointer = Arc::new(
        Mutex::new(copper_array_cv)
    );



    let copper_surface_temperature_boundary_condition = 
    HeatTransferEntity::BoundaryConditions(
        BCType::UserSpecifiedTemperature(boundary_condition_temperature));

    let surf_temp_bc_ptr = Arc::new(
        Mutex::new(copper_surface_temperature_boundary_condition)
    );

    // final temperature for array_cv at 19cm 

    let mut final_temp_degc_arr_cv_19cm = 0.0;
    let mut final_temp_degc_analytical_19cm = 0.0;


    // time settings
    let max_time: Time = Time::new::<second>(20.0);

    let timestep: Time = Time::new::<second>(0.1);
    let timestep_ptr = Arc::new(
        Mutex::new(timestep)
    );
    let max_time_ptr = Arc::new(max_time);

    // array cv calculation loop
    {

        let mut array_cv_in_loop = array_cv_pointer.lock().unwrap();

        let mut surf_temp_bc_in_loop = surf_temp_bc_ptr.lock().unwrap();

        let mut timestep_in_loop = timestep_ptr.lock().unwrap();
        let max_time_ptr_in_loop = max_time_ptr;


        // let's establish interactions between each of the nodes
        //
        // the first node would have a 1cm thermal resistance as it 
        // is closest to the wall 
        //
        //
        // | 
        // | 
        // | 1cm                2cm                 2cm
        // -------- * ------------------------- * ---------------
        // |        node 1                      node 2 
        // | 
        // | 
        // 
        //

        let node_half_length: Length = 
        delta_x * 0.5;
        let node_half_length: XThicknessThermalConduction = 
        node_half_length.into();

        let first_node_thermal_resistance = 
        HeatTransferInteractionType::
            SingleCartesianThermalConductanceOneDimension(copper,
                node_half_length);

        // so between the array cv and bc, i will attach bc to left 
        // of array cv 


        // the other bit is that I must have an adiabatic BC at the back 
        // otherwise I will have problems 
        //
        // So i must make an interaction type and the 

        let heat_flow_interaction: HeatTransferInteractionType = 
        HeatTransferInteractionType::UserSpecifiedHeatAddition;

        let mut adiabatic_bc = HeatTransferEntity::BoundaryConditions(
            BCType::UserSpecifiedHeatAddition(Power::new::<watt>(0.0))
        );

        // now link it together 


        let mut current_time_simulation_time = Time::new::<second>(0.0);

        let mut timestep_value;
        while current_time_simulation_time <= *max_time_ptr_in_loop {

            // first let's link the heat transfer entities 
            // link heat transfer entities 
            // the array cv to its two boundary conditions
            // and calculate their relevant timescales
            link_heat_transfer_entity(&mut array_cv_in_loop,
                &mut adiabatic_bc,
                heat_flow_interaction).unwrap();

            link_heat_transfer_entity(&mut surf_temp_bc_in_loop,
                &mut array_cv_in_loop,
                first_node_thermal_resistance).unwrap();



            // now we need to update the timestep 
            // we'll just use the cv-bc timestep because that has 
            // the smallest lengthscale, should be the shortest
            //
            let max_temperature_change: TemperatureInterval = 
            TemperatureInterval::new::<
                uom::si::temperature_interval::degree_celsius>(2.0);


            // basically I need to get the max timestep,
            // so I get it from the SolidColumn  method 
            //
            // the max timestep is then loaded into the resulting 
            // solid column
            //
            // what I then do is to replace the array_cv_in_loop by 
            // the new solid column.
            //
            // It's a little computationally expensive but it does the 
            // job

            let mut one_d_array_clone: SolidColumn = 
                array_cv_in_loop.deref().clone().try_into().unwrap();

            let timestep_from_api = one_d_array_clone.get_max_timestep(
                max_temperature_change).unwrap();

            *array_cv_in_loop.deref_mut() = one_d_array_clone.into();

            timestep_value = timestep_from_api;
            // update timestep value


            let first_node_timescale: Time = 
            calculate_timescales_for_heat_transfer_entity(
                &mut surf_temp_bc_in_loop,
                &mut array_cv_in_loop,
                first_node_thermal_resistance).unwrap();

            // compare timestep for first node timescale and 
            // existing timestep 

            if timestep_value > first_node_timescale {
                timestep_value = first_node_timescale;
            }
            *timestep_in_loop.deref_mut() = timestep_value;
            // advance timestep

            HeatTransferEntity::advance_timestep(
                array_cv_in_loop.deref_mut(),
                *timestep_in_loop).unwrap();


            // now let's capture the temperature data first 
            // to do, write tempereature
            let temperature_vector = 
                array_cv_in_loop.get_temperature_vector().unwrap();

            let temp_19cm = temperature_vector.last().unwrap();

            let final_temp_ptr_19cm = &mut final_temp_degc_arr_cv_19cm;

            *final_temp_ptr_19cm = temp_19cm.get::<degree_celsius>();

            current_time_simulation_time += *timestep_in_loop.deref();

        }

    }




    // then let's do the analytical solution
    let theta_error_fn = |fourier_number_x_t: Ratio| 
    -> Result<Ratio,String> {

        let fourier_value = fourier_number_x_t.value;

        // there's going to be a case where the Fourier number is exactly 
        // zero
        // We cannot divide things by zero 
        // but erfc of anything bigger than 2 is close to zero 
        // so if Fo -> 0, erfc(1/Fo) -> 0 
        // hence, a Fourier number of zero results in theta = 0 
        //

        if fourier_value == 0.0 {
            let theta_ratio = Ratio::new::<ratio>(0.0);
            return Ok(theta_ratio);
        }

        if fourier_value < 0.0 {

            return Err("negative fourier value".to_string());
        }


        // theta (x,t) = erfc (1 / (2.0 * sqrt{Fo(x,t)}) )
        let exponent_denominator = 2.0 * fourier_value.sqrt();

        let exponent: f64 = 1.0/exponent_denominator;

        let theta_value = erfc(exponent);

        let theta_ratio = Ratio::new::<ratio>(theta_value);

        return Ok(theta_ratio);

    };

    // let's do from t = 0 to t = 20 in 4 second intervals
    //
    // then we will print out the temperature profiles from x = 0 to 
    // x = 1m
    // Based on some preliminary calculations, the maximum length where 
    // 1/(2 sqrt Fo)  = 2 
    // at t = 20  
    // is about 0.2 m
    //
    // therefore, I'll place about 5 nodes there from x = 0m to 
    // x = 0.2m, these will record for us the temperature profile 
    // at a certain time

    let time_vector: Vec<Time> = vec![
        Time::new::<second>(0.0),
        Time::new::<second>(4.0),
        Time::new::<second>(8.0),
        Time::new::<second>(12.0),
        Time::new::<second>(16.0),
        Time::new::<second>(20.0),
    ];

    let length_vector: Vec<Length> = vec![
        Length::new::<meter>(0.01),
        Length::new::<meter>(0.05),
        Length::new::<meter>(0.11),
        Length::new::<meter>(0.15),
        Length::new::<meter>(0.19),
    ];



    for time in time_vector.iter() {

        // initialise a temperature vector 

        let mut temp_vector: Vec<ThermodynamicTemperature> = 
        vec![];

        // make a nested length loop 

        for length in length_vector.iter() {

            // if length is zero, then BC implies that temperature 
            // must be BC temperature 

            if length.value == 0.0 {
                temp_vector.push(boundary_condition_temperature);
            }

            // if time is zero, then initial conditions mean that 
            // temperature is the copper initial temperature

            else if time.value == 0.0 {
                temp_vector.push(copper_initial_temperature);
            } 
            else {

                // calc fourier number 
                let fourier_number: Ratio = 
                (copper_thermal_diffusivity_alpha * *time)
                / *length 
                / *length;

                let theta = theta_error_fn(fourier_number)?;

                // theta = (T(x,t) - T_i)/(T_BC - T_i)

                let temperature_diff: TemperatureInterval = 
                TemperatureInterval::new::<interval_deg_c>(
                    boundary_condition_temperature.value - 
                    copper_initial_temperature.value);

                let temperature_x_t: ThermodynamicTemperature = 
                copper_initial_temperature + 
                theta * temperature_diff;

                temp_vector.push(temperature_x_t);
            }

        }

        // once the temperature vector is finished, we can write it 
        // to csv file 

        let _temperature_1 = temp_vector[0];
        let _temperature_2 = temp_vector[1];
        let _temperature_3 = temp_vector[2];
        let _temperature_4 = temp_vector[3];
        let temperature_5 = temp_vector[4];

        let final_temp_ptr_19cm_analytical = 
            &mut final_temp_degc_analytical_19cm;

        *final_temp_ptr_19cm_analytical = temperature_5.get::<degree_celsius>();


    }

    // this setup is meant to be emulated using control volumes with 
    // some thermal resistances between them

    // the maximum difference for this coarse mesh is 0.3K
    approx::assert_abs_diff_eq!(
        final_temp_degc_analytical_19cm,
        final_temp_degc_arr_cv_19cm,
        epsilon=0.3
        );
    Ok(())

}
