
use std::ops::{DerefMut, Deref};
use std::sync::{Arc, Mutex};
use std::thread;

use peroxide::prelude::erfc;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::boundary_conditions::BCType;
use crate::boussinesq_thermophysical_properties::thermal_diffusivity::try_get_alpha_thermal_diffusivity;
use crate::boussinesq_thermophysical_properties::{Material, SolidMaterial};
use crate::control_volume_dimensions::XThicknessThermalConduction;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::pre_built_components::heat_transfer_entities::preprocessing::{calculate_timescales_for_heat_transfer_entity, link_heat_transfer_entity};
use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
use crate::single_control_vol::SingleCVNode;
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
///
/// This is conduction in a semi infinite medium with constant 
/// temperature boundary conditions 
///
/// These analytical solutions are well known
/// and contain error functions 
///
/// For constant temperature, the temperature at point x 
/// and time t is given by 
///
/// theta (x,t) = erfc (x / (2.0 * sqrt{alpha t}) )
///
///
/// Trojan, M. "Transient Heat Conduction in Semiinfinite 
/// Solid with Surface Convection." Encyclopedia of (2014).
///
/// One may note that the (x / (2.0 * sqrt{alpha t}) )
/// is simply the reciprocal of a Fourier like number multiplied by 
/// a constant
///
/// Fo = (alpha t)/L^2 
///
/// In this case, the length scale is x and t is the time.
///
/// How long must the medium be for it to be semi infinite? 
///
/// erfc (zeta) can be plotted on a graph 
/// at erfc(zeta) where zeta >= 2.0,
/// erfc(zeta) <= 0.005 (0.5\% change)
///
/// theta (x,t) = erfc (1 / (2.0 * sqrt{Fo(x,t)}) )
///
///
/// probably use some published reference, better quality
/// // https://www.unipamplona.edu.co/unipamplona/portalIG/home_34/recursos/01general/21082014/unidad_2_termo_ii.pdf
///
///
#[test]
fn transient_conduction_semi_infinite_copper_medium() 
-> Result<(), TuasLibError>{

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

    let node_length: Length = Length::new::<centimeter>(2.0);
    // let's make the up to 0.40 m of nodes, 
    // we have 10 nodes in all, so each node has
    // about 4cm of thermal resistance between the nodes
    //
    // then specify each node is made of copper, 
    // it shall be a 1D kind of conduction node
    // for 1D conduction type nodes, we just take 1m^2 area as a basis 
    // and apply adiabatic BC to the surface normal 
    // 
    // so I'll need to make 1 BC and 10 control volumes
    // simulate the transient temperatures for up to 20s

    let single_cv_node_1 : HeatTransferEntity = SingleCVNode::new_one_dimension_volume(
        node_length, copper, copper_initial_temperature, 
        pressure)?.into();
    let single_cv_node_2 : HeatTransferEntity = SingleCVNode::new_one_dimension_volume(
        node_length, copper, copper_initial_temperature, 
        pressure)?.into();
    let single_cv_node_3 : HeatTransferEntity = SingleCVNode::new_one_dimension_volume(
        node_length, copper, copper_initial_temperature, 
        pressure)?.into();
    let single_cv_node_4 : HeatTransferEntity = SingleCVNode::new_one_dimension_volume(
        node_length, copper, copper_initial_temperature, 
        pressure)?.into();
    let single_cv_node_5 : HeatTransferEntity = SingleCVNode::new_one_dimension_volume(
        node_length, copper, copper_initial_temperature, 
        pressure)?.into();
    let single_cv_node_6 : HeatTransferEntity = SingleCVNode::new_one_dimension_volume(
        node_length, copper, copper_initial_temperature, 
        pressure)?.into();
    let single_cv_node_7 : HeatTransferEntity = SingleCVNode::new_one_dimension_volume(
        node_length, copper, copper_initial_temperature, 
        pressure)?.into();
    let single_cv_node_8 : HeatTransferEntity = SingleCVNode::new_one_dimension_volume(
        node_length, copper, copper_initial_temperature, 
        pressure)?.into();
    let single_cv_node_9 : HeatTransferEntity = SingleCVNode::new_one_dimension_volume(
        node_length, copper, copper_initial_temperature, 
        pressure)?.into();
    let single_cv_node_10 : HeatTransferEntity = SingleCVNode::new_one_dimension_volume(
        node_length, copper, copper_initial_temperature, 
        pressure)?.into();

    let single_cv_1_ptr = Arc::new(
        Mutex::new(single_cv_node_1)
    );
    let single_cv_2_ptr = Arc::new(
        Mutex::new(single_cv_node_2)
    );
    let single_cv_3_ptr = Arc::new(
        Mutex::new(single_cv_node_3)
    );
    let single_cv_4_ptr = Arc::new(
        Mutex::new(single_cv_node_4)
    );
    let single_cv_5_ptr = Arc::new(
        Mutex::new(single_cv_node_5)
    );
    let single_cv_6_ptr = Arc::new(
        Mutex::new(single_cv_node_6)
    );
    let single_cv_7_ptr = Arc::new(
        Mutex::new(single_cv_node_7)
    );
    let single_cv_8_ptr = Arc::new(
        Mutex::new(single_cv_node_8)
    );
    let single_cv_9_ptr = Arc::new(
        Mutex::new(single_cv_node_9)
    );
    let single_cv_10_ptr = Arc::new(
        Mutex::new(single_cv_node_10)
    );

    let copper_surface_temperature_boundary_condition = 
    HeatTransferEntity::BoundaryConditions(
        BCType::UserSpecifiedTemperature(boundary_condition_temperature));

    let surf_temp_ptr = Arc::new(
        Mutex::new(copper_surface_temperature_boundary_condition)
    );

    // time settings
    let max_time: Time = Time::new::<second>(20.0);

    let timestep: Time = Time::new::<second>(0.1);
    let timestep_ptr = Arc::new(
        Mutex::new(timestep)
    );
    let max_time_ptr = Arc::new(max_time);

    let calculation_loop = move || {

        let mut single_cv_1_in_loop = single_cv_1_ptr.lock().unwrap();
        let mut single_cv_2_in_loop = single_cv_2_ptr.lock().unwrap();
        let mut single_cv_3_in_loop = single_cv_3_ptr.lock().unwrap();
        let mut single_cv_4_in_loop = single_cv_4_ptr.lock().unwrap();
        let mut single_cv_5_in_loop = single_cv_5_ptr.lock().unwrap();
        let mut single_cv_6_in_loop = single_cv_6_ptr.lock().unwrap();
        let mut single_cv_7_in_loop = single_cv_7_ptr.lock().unwrap();
        let mut single_cv_8_in_loop = single_cv_8_ptr.lock().unwrap();
        let mut single_cv_9_in_loop = single_cv_9_ptr.lock().unwrap();
        let mut single_cv_10_in_loop = single_cv_10_ptr.lock().unwrap();

        let mut surf_temp_bc_in_loop = surf_temp_ptr.lock().unwrap();

        let mut timestep_in_loop = timestep_ptr.lock().unwrap();
        let max_time_ptr_in_loop = max_time_ptr;

        use csv::Writer;
        let mut wtr = Writer::from_path("single_cv_semi_infinite_simulated_values.csv")
            .unwrap();

        wtr.write_record(&["time_seconds",
            "0cm_temperautre_kelvin",
            "1cm_temperature_kelvin",
            "3cm_temperature_kelvin",
            "5cm_temperature_kelvin",
            "7cm_temperature_kelvin",
            "9cm_temperature_kelvin",
            "11cm_temperature_kelvin",
            "13cm_temperature_kelvin",
            "15cm_temperature_kelvin",
            "17cm_temperature_kelvin",
            "19cm_temperature_kelvin",
        ]).unwrap();

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
        node_length * 0.5;
        let node_half_length: XThicknessThermalConduction = 
        node_half_length.into();
        let node_length: XThicknessThermalConduction = node_length.into();

        let first_node_thermal_resistance = 
        HeatTransferInteractionType::
            SingleCartesianThermalConductanceOneDimension(copper,
                node_half_length);


        let subsequent_node_thermal_resistance = 
        HeatTransferInteractionType::
            SingleCartesianThermalConductanceOneDimension(copper,
                node_length);


        let mut current_time_simulation_time = Time::new::<second>(0.0);


        while current_time_simulation_time <= *max_time_ptr_in_loop {

            // first let's link the heat transfer entities 


            // first node is very important, we have BC and CV linkage
            link_heat_transfer_entity(&mut surf_temp_bc_in_loop,
                &mut single_cv_1_in_loop,
                first_node_thermal_resistance).unwrap();

            // subsequent nodes have similar linkages
            link_heat_transfer_entity(&mut single_cv_1_in_loop,
                &mut single_cv_2_in_loop,
                subsequent_node_thermal_resistance).unwrap();

            link_heat_transfer_entity(&mut single_cv_2_in_loop,
                &mut single_cv_3_in_loop,
                subsequent_node_thermal_resistance).unwrap();

            link_heat_transfer_entity(&mut single_cv_3_in_loop,
                &mut single_cv_4_in_loop,
                subsequent_node_thermal_resistance).unwrap();

            link_heat_transfer_entity(&mut single_cv_4_in_loop,
                &mut single_cv_5_in_loop,
                subsequent_node_thermal_resistance).unwrap();

            link_heat_transfer_entity(&mut single_cv_5_in_loop,
                &mut single_cv_6_in_loop,
                subsequent_node_thermal_resistance).unwrap();

            link_heat_transfer_entity(&mut single_cv_6_in_loop,
                &mut single_cv_7_in_loop,
                subsequent_node_thermal_resistance).unwrap();

            link_heat_transfer_entity(&mut single_cv_7_in_loop,
                &mut single_cv_8_in_loop,
                subsequent_node_thermal_resistance).unwrap();

            link_heat_transfer_entity(&mut single_cv_8_in_loop,
                &mut single_cv_9_in_loop,
                subsequent_node_thermal_resistance).unwrap();

            link_heat_transfer_entity(&mut single_cv_9_in_loop,
                &mut single_cv_10_in_loop,
                subsequent_node_thermal_resistance).unwrap();

            // now let's capture the temperature data first 

            let bc_temperature: ThermodynamicTemperature 
            = boundary_condition_temperature;

            let cv_1_temperature = 
            HeatTransferEntity::temperature( 
                single_cv_1_in_loop.deref_mut()).unwrap();

            let cv_2_temperature = 
            HeatTransferEntity::temperature( 
                single_cv_2_in_loop.deref_mut()).unwrap();

            let cv_3_temperature = 
            HeatTransferEntity::temperature( 
                single_cv_3_in_loop.deref_mut()).unwrap();

            let cv_4_temperature = 
            HeatTransferEntity::temperature( 
                single_cv_4_in_loop.deref_mut()).unwrap();

            let cv_5_temperature = 
            HeatTransferEntity::temperature( 
                single_cv_5_in_loop.deref_mut()).unwrap();

            let cv_6_temperature = 
            HeatTransferEntity::temperature( 
                single_cv_6_in_loop.deref_mut()).unwrap();

            let cv_7_temperature = 
            HeatTransferEntity::temperature( 
                single_cv_7_in_loop.deref_mut()).unwrap();

            let cv_8_temperature = 
            HeatTransferEntity::temperature( 
                single_cv_8_in_loop.deref_mut()).unwrap();

            let cv_9_temperature = 
            HeatTransferEntity::temperature( 
                single_cv_9_in_loop.deref_mut()).unwrap();

            let cv_10_temperature = 
            HeatTransferEntity::temperature( 
                single_cv_10_in_loop.deref_mut()).unwrap();

            let time_string = current_time_simulation_time.value.to_string();

            wtr.write_record(&[time_string,
                bc_temperature.value.to_string(),
                cv_1_temperature.value.to_string(),
                cv_2_temperature.value.to_string(),
                cv_3_temperature.value.to_string(),
                cv_4_temperature.value.to_string(),
                cv_5_temperature.value.to_string(),
                cv_6_temperature.value.to_string(),
                cv_7_temperature.value.to_string(),
                cv_8_temperature.value.to_string(),
                cv_9_temperature.value.to_string(),
                cv_10_temperature.value.to_string(), ])
                .unwrap();

            // now we need to update the timestep 
            // we'll just use the cv-bc timestep because that has 
            // the smallest lengthscale, should be the shortest

            let timestep_from_api = 
            calculate_timescales_for_heat_transfer_entity(
                &mut surf_temp_bc_in_loop,
                &mut single_cv_1_in_loop,
                first_node_thermal_resistance).unwrap();


            let timestep_value = timestep_from_api;
            // update timestep value

            *timestep_in_loop.deref_mut() = timestep_value;

            HeatTransferEntity::advance_timestep(
                single_cv_1_in_loop.deref_mut(),
                *timestep_in_loop).unwrap();

            HeatTransferEntity::advance_timestep(
                single_cv_2_in_loop.deref_mut(),
                *timestep_in_loop).unwrap();
            HeatTransferEntity::advance_timestep(
                single_cv_3_in_loop.deref_mut(),
                *timestep_in_loop).unwrap();
            HeatTransferEntity::advance_timestep(
                single_cv_4_in_loop.deref_mut(),
                *timestep_in_loop).unwrap();
            HeatTransferEntity::advance_timestep(
                single_cv_5_in_loop.deref_mut(),
                *timestep_in_loop).unwrap();
            HeatTransferEntity::advance_timestep(
                single_cv_6_in_loop.deref_mut(),
                *timestep_in_loop).unwrap();
            HeatTransferEntity::advance_timestep(
                single_cv_7_in_loop.deref_mut(),
                *timestep_in_loop).unwrap();
            HeatTransferEntity::advance_timestep(
                single_cv_8_in_loop.deref_mut(),
                *timestep_in_loop).unwrap();
            HeatTransferEntity::advance_timestep(
                single_cv_9_in_loop.deref_mut(),
                *timestep_in_loop).unwrap();
            HeatTransferEntity::advance_timestep(
                single_cv_10_in_loop.deref_mut(),
                *timestep_in_loop).unwrap();

            current_time_simulation_time += *timestep_in_loop.deref();
        }
        wtr.flush().unwrap();
    };

    let calculation_thread = thread::spawn(calculation_loop);



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

    // let's make the csv writer 

    use csv::Writer;
    let mut wtr = Writer::from_path("analytical_1d_transient_conduction.csv")
        .unwrap();

    wtr.write_record(&["time_seconds",
        "analytical_temp_1cm", 
        "analytical_temp_5cm",
        "analytical_temp_11cm", 
        "analytical_temp_15cm", 
        "analytical_temp_19cm", 
    ]).unwrap();


    wtr.flush().unwrap();

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

        let temperature_1 = temp_vector[0];
        let temperature_2 = temp_vector[1];
        let temperature_3 = temp_vector[2];
        let temperature_4 = temp_vector[3];
        let temperature_5 = temp_vector[4];

        wtr.write_record(&[&time.value.to_string(),
            & *temperature_1.value.to_string(),
            & *temperature_2.value.to_string(),
            & *temperature_3.value.to_string(),
            & *temperature_4.value.to_string(),
            & *temperature_5.value.to_string(),
        ]).unwrap();

        wtr.flush().unwrap();


    }

    calculation_thread.join().unwrap();
    // this setup is meant to be emulated using control volumes with 
    // some thermal resistances between them

    //todo!("need to do 4 control vol and 1 BC for 1d transient conduction");
    Ok(())
}

/// this is basically the same copper medium test,
/// except that now I use an array CVs
/// this is with 10 total nodes (8 inner nodes and two boundary nodes)
/// 
#[test]
fn arraycv_transient_conduction_copper_medium() -> Result<(),TuasLibError>{

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

    let surf_temp_ptr = Arc::new(
        Mutex::new(copper_surface_temperature_boundary_condition)
    );

    // time settings
    let max_time: Time = Time::new::<second>(20.0);

    let timestep: Time = Time::new::<second>(0.1);
    let timestep_ptr = Arc::new(
        Mutex::new(timestep)
    );
    let max_time_ptr = Arc::new(max_time);

    let calculation_loop = move || {

        let mut array_cv_in_loop = array_cv_pointer.lock().unwrap();

        let mut surf_temp_bc_in_loop = surf_temp_ptr.lock().unwrap();

        let mut timestep_in_loop = timestep_ptr.lock().unwrap();
        let max_time_ptr_in_loop = max_time_ptr;

        use csv::Writer;
        let mut wtr = Writer::from_path("array_cv_semi_infinite_copper.csv")
            .unwrap();
        // header for the csv file
        wtr.write_record(&["time_seconds",
            "0cm_temperautre_kelvin",
            "1cm_temperature_kelvin",
            "3cm_temperature_kelvin",
            "5cm_temperature_kelvin",
            "7cm_temperature_kelvin",
            "9cm_temperature_kelvin",
            "11cm_temperature_kelvin",
            "13cm_temperature_kelvin",
            "15cm_temperature_kelvin",
            "17cm_temperature_kelvin",
            "19cm_temperature_kelvin",
        ]).unwrap();

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


            // now let's capture the temperature data first 
            {

                let bc_temperature: ThermodynamicTemperature =
                HeatTransferEntity::temperature(
                    &mut surf_temp_bc_in_loop).unwrap();


                let time_string = current_time_simulation_time.value.to_string();

                // to do, write tempereature
                let temperature_vector = 
                HeatTransferEntity::temperature_vector(
                    &mut array_cv_in_loop).unwrap();

                let mut string_vector: Vec<String> = vec![];
                // should have a method to get the temperature array easily
                //
                // insert time and first temperature 

                string_vector.push(time_string);
                string_vector.push(bc_temperature.value.to_string());

                for temp_reference in temperature_vector.iter() {
                    string_vector.push(temp_reference.value.to_string());


                }
                wtr.write_record(&string_vector).unwrap();
            }
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


            current_time_simulation_time += *timestep_in_loop.deref();

        }
        wtr.flush().unwrap();

    };

    let calculation_thread = thread::spawn(calculation_loop);



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

    // let's make the csv writer 

    use csv::Writer;
    let mut wtr = Writer::from_path("analytical_1d_transient_conduction.csv")
        .unwrap();

    wtr.write_record(&["time_seconds",
        "analytical_temp_1cm", 
        "analytical_temp_5cm",
        "analytical_temp_11cm", 
        "analytical_temp_15cm", 
        "analytical_temp_19cm", 
    ]).unwrap();


    wtr.flush().unwrap();

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

        let temperature_1 = temp_vector[0];
        let temperature_2 = temp_vector[1];
        let temperature_3 = temp_vector[2];
        let temperature_4 = temp_vector[3];
        let temperature_5 = temp_vector[4];

        wtr.write_record(&[&time.value.to_string(),
            & *temperature_1.value.to_string(),
            & *temperature_2.value.to_string(),
            & *temperature_3.value.to_string(),
            & *temperature_4.value.to_string(),
            & *temperature_5.value.to_string(),
        ]).unwrap();

        wtr.flush().unwrap();


    }

    calculation_thread.join().unwrap();
    // this setup is meant to be emulated using control volumes with 
    // some thermal resistances between them

    //todo!("need to do 4 control vol and 1 BC for 1d transient conduction");
    Ok(())

}
