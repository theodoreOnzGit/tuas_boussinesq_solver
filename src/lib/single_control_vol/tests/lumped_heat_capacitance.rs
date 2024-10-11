
use std::f64::consts::PI;
use std::ops::{DerefMut, Deref};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::boundary_conditions::BCType;
use crate::boussinesq_thermophysical_properties::density::try_get_rho;
use crate::control_volume_dimensions::{OuterDiameterThermalConduction, SurfaceArea};
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::{DataUserSpecifiedConvectionResistance, HeatTransferInteractionType};
use crate::pre_built_components::heat_transfer_entities::cv_types::CVType;
use crate::pre_built_components::heat_transfer_entities::preprocessing::link_heat_transfer_entity;
use crate::pre_built_components::heat_transfer_entities::HeatTransferEntity;
use crate::single_control_vol::SingleCVNode;
use crate::tuas_lib_error::TuasLibError;




use crate::boussinesq_thermophysical_properties::specific_enthalpy::try_get_temperature_from_h;
use crate::boussinesq_thermophysical_properties::specific_enthalpy::try_get_h;
use crate::boussinesq_thermophysical_properties::{Material, SolidMaterial};
use uom::si::f64::*;
use uom::si::length::centimeter;
use uom::si::power::watt;
use uom::si::pressure::atmosphere;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::time::second;


//#[test]
//pub fn meant_to_fail(){
//
//    // now for rust , we don't have assert equal
//    // showing expected and test values
//    // we just see if left == right
//    // not like C#,
//    // where left is expected value,
//    // right is asserted value
//    //
//    assert_eq!(2.0,2.0);
//    unimplemented!();
//}

// This test prototypes the CIET 
// #[test]
//pub fn ciet_crude_heater_v_1_0 (){
//
//    use uom::si::thermodynamic_temperature::degree_celsius;
//    use uom::si::mass_rate::kilogram_per_second;
//    // for each timestep, the ciet heater must have 
//    // the inlet conditions specified
//    let fluid_temp_inlet = ThermodynamicTemperature::new::<
//        degree_celsius>(79.0);
//    let mass_flowrate = MassRate::new::<kilogram_per_second>(0.18);
//
//    // we also need to let it know that therminol VP1 is the correct 
//    // fluid 
//    //
//    // This can be put into some enum kind of thing, because 
//    // writing those correlations is rather repetitive
//    //
//    // suppose that is done, we need to use this to specify the 
//    // inlet enthalpy 
//    //
//    // at the bare minimum I need a function to convert the temperature 
//    // to an enthalpy
//    // 
//    // this is found in the fluid_mechanics_lib for now 
//
//    use thermal_hydraulics_rs::fluid_mechanics_lib::prelude::*;
//    use thermal_hydraulics_rs::fluid_mechanics_lib::therminol_component;
//
//    // we'll get the inlet specific enthalpy 
//
//    let inlet_specific_enthalpy = therminol_component::dowtherm_a_properties:: 
//        getDowthermAEnthalpy(fluid_temp_inlet);
//
//    // now we calculate inlet enthalpy
//    use thermal_hydraulics_rs::heat_transfer_lib:: 
//    control_volume_calculations::common_functions;
//
//    let inlet_enthalpy = common_functions::calculate_enthalpy_flow(
//        mass_flowrate, inlet_specific_enthalpy);
//
//    // print some output
//    println!("{:?}",inlet_enthalpy);
//
//    // next step, need to calculate heat flow between fluid and 
//    // environment, so we need a fluid temperature at 
//    // present timestep 
//    //
//    // Also need a heater_shell temperature at present timestep
//    //
//    let fluid_temperature_present_timestep = ThermodynamicTemperature:: 
//        new::<degree_celsius>(88.0);
//
//    let heater_shell_temperature_present_timestep = 
//    ThermodynamicTemperature::new::<degree_celsius>(138.0);
//
//    // we can assume the heater shell is a lumped capacitance model 
//    // or something
//    // otherwise there should be some temperature gradient between 
//    // heater center, heater inner surface and heater outer surface
//    // 
//    // We could simulate this using some kind of node system, 
//    // and we have then two finite volumes each with a lumped capacitance
//    // system
//
//    let heater_outer_shell_temperature_present_timestep = 
//    ThermodynamicTemperature::new::<degree_celsius>(143.0);
//
//    // we won't do axial nodalisation yet...
//    // but for radial nodalisation, we may perhaps use GeN-Foam code
//    // now we need to relate enthalpy and temperature via some 
//    // thermophysical properties
//    // 
//    // so all in all, we need three control volumes, 
//    // one for fluid, one for shell inner node, one for shell outer 
//    // node
//    //
//    // The shell itself will have power supplied to it and conduction 
//    // heat transfer, that's all. And we want to calculate control 
//    // volume enthalpy in the next timestep
//
//    // for the outer boundary conditions, 
//    // we also have an ambient_temperature
//    // and an associated heat transfer coefficient
//
//
//    let ambient_temperature = 
//    ThermodynamicTemperature::new::<degree_celsius>(21.67);
//
//    // let's now calculate the enthalpy at the next timestep for the 
//    // outer shell layer.
//
//    use thermal_hydraulics_rs::heat_transfer_lib::control_volume_calculations;
//
//    // we first need timestep, and we also determine the 
//    // enthalpy flows due to fluid movement to be zero
//    use uom::si::time::second;
//    use uom::si::power::watt;
//
//    let timestep = Time::new::<second>(0.1);
//    let solid_conductor_enthalpy_flow = 
//    Power::new::<watt>(0.0);
//
//    // the heat supplied to the system is I^2 R
//    // and we know resistance is R = rho L/A
//    // For the electrical heater we know potential drop across the 
//    // tube is the same, therefore, V is constant
//    //
//    // P = V^2/R for each tube node
//    // 
//    // P = V^2 A_{xs} / (rho L) 
//    //
//    // Hence, all else equal, power scales as cross sectional area.
//    // if we have heater power at 8 kW
//
//    let total_heater_power = Power::new::<watt>(8000_f64);
//
//    // we can take the outer node power to be the ratio of the 
//    // outer node area to the whole cross sectional area
//    // so circle area is pi D^2/48.0
//    // 
//
//    use uom::si::area::square_meter;
//
//    fn circle_area(diameter: Length) -> Area {
//        return diameter * diameter * PI / 4.0;
//    }
//
//    use uom::si::length::centimeter;
//
//    // we can specify the heater inner and heater outer 
//    // diameter
//    let heater_od = Length::new::<centimeter>(4.0);
//    let heater_id = Length::new::<centimeter>(3.81);
//    let heater_midpoint = (heater_od + heater_id)/2.0;
//
//    let heater_outer_tube_xs_area = circle_area(heater_od)
//        - circle_area(heater_id);
//
//    let heater_outer_tube_outer_node_xs_area = circle_area(heater_od)
//        - circle_area(heater_midpoint);
//
//    let heater_outer_power_fraction = heater_outer_tube_outer_node_xs_area/
//        heater_outer_tube_xs_area;
//
//    let heater_outer_power_fraction: f64 = 
//    heater_outer_power_fraction.value;
//
//    // now we can calculate heater power for outer node 
//
//    let heater_power_outer_node: Power = 
//    total_heater_power * heater_outer_power_fraction;
//
//    // work done is zero, not considering anything
//
//    let work_done_on_system = Power::new::<watt>(0.0);
//
//    // actually enthalpy flow in can also be a conduction thing, 
//    // but in terms of first law, it is Q to system
//    // we need now to calculate heat loss to environment
//    // and also heat transfer between this node and the inner node 
//    //
//    // Now, I'm going to assume the surface temperature is same 
//    // as the finite volume temperature, though of course, one should 
//    // perhaps put in a conduction resistance, so that at steady 
//    // state, the solution is the same as the resistance model.
//    //
//    // We don't keep track of the surface temperature per se, only 
//    // finite volume temperatures.
//    // 
//    // For heat transfer between nodes, there is also some thermal 
//    // resistance between nodes 
//    //
//
//    let h_to_air: HeatTransfer = 
//    HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);
//
//    // next is the thermal conductivity
//    // we'll probably need the thermal conductivity and 
//    // heat capacity of steel
//    //
//    // From Graves,1991:
//    // We write from LaTeX:
//
//    //\begin{equation}
//    //	k \left( \frac{W}{m \cdot K}  \right) = 
//    //	7.9318 + 0.023051~T(K) - 6.4166*10^{-6}~T(K)^2
//    //\end{equation}
//    //
//    //\begin{equation}
//    //	c_p \left( \frac{J}{g \cdot K}  \right) = 
//    //	0.4267 + 1.700* 10^{-4}~T(K) + 5.200*10^{-8}~T(K)^2
//    //\end{equation}
//    //
//    // We may want a library or at least a function which 
//    // accepts an enum saying what material it is,
//    // and then based on the enum and some other things like 
//    // temperature, it returns the value of the desired 
//    // property in unit safe values similar to coolprop
//    //
//    // Though of course, we don't want to overextend ourselves
//    //
//    // First of course, solid properties, we'll need copper,
//    // steel and fibreglass at the bare minimum
//    //
//    // now we basically need thermal resistances between each node 
//    //
//    // [T_dowtherm] --- T_inner_surface -- [T_heater_inner] --- 
//    // [T_heater_outer]
//    // --- T_outer_surface --- T_air 
//    // 
//    // Now, only T_dowtherm, T_heater_inner and T_heater outer, 
//    // which I bracketed, have thermal inertia, 
//    // the rest are boundary conditions
//    //
//    // In factor, we probably don't explicitly calculate inner surface 
//    // or outer surface temperatures in intermediate calculation 
//    // steps
//    // 
//    // The bracketed terms therefore represent control volumes which 
//    // I need to connect to each other 
//    //
//    // How can I do so without over-abstracting the thing?
//    //
//    // because every control volume interaction is quite complex 
//    //
//    // We could do a matrix style as with CFD, but matrices are 
//    // abstract, and make the code hard to read, unless of course 
//    // you do things like OPENFOAM where the PDEs are set by the user 
//    // through the solver syntax which is pretty easy to read.
//    //
//    // One idea I have is to have a big task list, that when I connect 
//    // control volumes interactions together, I add it to this big task 
//    // list 
//    //
//    // However, for annular shells, GeN-Foam already has syntax 
//    // written in C++ for GeN-Foam which i can port to Rust
//
//    
//
//
//
//    //let enthalpy_outer_shell_next_timestep = 
//    //control_volume_calculations::common_functions::
//    //get_control_volume_enthalpy_next_timestep(
//    //        timestep, 
//    //        solid_conductor_enthalpy_flow, 
//    //        solid_conductor_enthalpy_flow, 
//    //        heat_supplied_to_system, 
//    //        work_done_on_system, 
//    //        control_volume_enthalpy_current_timestep);
//
//
//
//
//
//    
//
//
//}

/// this is just a simple test to check if control volumes and boundary 
/// conditions are working properly
///
///
#[test]
//#[ignore = "lumped lumped-capacitance-test takes about 1 min, too long"]
fn lumped_heat_capacitance_steel_ball_in_air() -> Result<(), TuasLibError>{

    // first, a steel control vol
    // determine parameters first
    let steel = Material::Solid(SolidMaterial::SteelSS304L);
    let pressure = Pressure::new::<atmosphere>(1.0);
    let steel_initial_temperature = 
    ThermodynamicTemperature::new::<degree_celsius>(150.0);

    // Programming feature comment 1:
    // might want a constructor which shortens this process
    // of making the HeatTransferEntity

    let steel_initial_enthalpy = try_get_h(
        steel, 
        steel_initial_temperature, 
        pressure)?;



    let steel_ball_diameter = OuterDiameterThermalConduction::from(
        Length::new::<centimeter>(2.0));
    let diameter: Length = steel_ball_diameter.into();
    let steel_ball_radius: Length = diameter/2.0;

    // mass = rho * V 
    //
    // V = 4/3 pi R^3
    let steel_ball_volume: Volume = 4.0/3.0 * PI * 
        steel_ball_radius * steel_ball_radius * steel_ball_radius;

    let steel_ball_density: MassDensity = try_get_rho(
        steel,
        steel_initial_temperature,
        pressure)?;
    
    let steel_ball_mass: Mass = steel_ball_density * steel_ball_volume;

    let steel_control_vol = 
    HeatTransferEntity::ControlVolume(
        CVType::SingleCV(
                SingleCVNode { 
                current_timestep_control_volume_specific_enthalpy: 
                steel_initial_enthalpy, 
                next_timestep_specific_enthalpy: 
                steel_initial_enthalpy, 
                rate_enthalpy_change_vector: 
                vec![], 
                mass_control_volume: steel_ball_mass, 
                material_control_volume: steel, 
                pressure_control_volume: pressure,
                volume: steel_ball_volume, 
                max_timestep_vector: 
                vec![],
                mesh_stability_lengthscale_vector:
                vec![],
                volumetric_flowrate_vector:
                vec![],
                temperature: steel_initial_temperature,
            }
        )
    );

    // next thing is the boundary condition 
    // Programming feature comment 2:
    // might want another constructor here too

    let ambient_temperature: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(25.0);

    let ambient_temperature_boundary_condition = 
    HeatTransferEntity::BoundaryConditions(
        BCType::UserSpecifiedTemperature(ambient_temperature));

    // we will be running this using async code similar to what 
    // is used in ciet's opc-ua server

    // make an atomically reference counted pointer with  
    // mutex
    let steel_cv_pointer = Arc::new(
        Mutex::new(
            steel_control_vol
        )
    );

    let ambient_temp_ptr = Arc::new(
        Mutex::new(ambient_temperature_boundary_condition)
    );

    let timestep: Time = Time::new::<second>(5.0);
    let timestep_ptr = Arc::new(
        Mutex::new(timestep)
    );

    let max_time: Time = Time::new::<second>(19200.0);

    let max_time_ptr = Arc::new(max_time);

    // we have to move the Arc pointers into the calculation loop 
    // essentially, ownership is moved to the calculation loop 
    // and after that, when the loop goes out of scope, the 
    // ownership is gone,
    //
    // I need a way to get data outside the loop
    // so i'll use a clone function for the Arc
    // to create a second pointer
    //
    // the first pointer will be dropped in the calculation_loop
    // but the second will survive
    let steel_cv_pointer_final = steel_cv_pointer.clone();


    // this is the calculation that runs every loop
    let calculation_loop = move || {

        // first, I use the mutex lock to lock other threads from 
        // modifying the steel cv
        let mut steel_cv_in_loop = steel_cv_pointer.lock().unwrap();
        let mut ambient_bc_in_loop = ambient_temp_ptr.lock().unwrap();
        let timestep_in_loop = timestep_ptr.lock().unwrap();
        let max_time_ptr_in_loop = max_time_ptr;

        // let's make a csv writer too 

        use csv::Writer;
        let mut wtr = Writer::from_path("air_cooled_steel_sphere.csv")
            .unwrap();

        wtr.write_record(&["time_seconds","temperature_kelvin"])
            .unwrap();

        // let me create an interaction between the control vol 
        // and bc
        // programming feature comment 3: might want to create 
        // a constructor for this too

        let heat_transfer_coeff = HeatTransfer::new::
            <watt_per_square_meter_kelvin>(20.0);

        let area: Area = 4.0 * PI * steel_ball_radius * steel_ball_radius;
        let surf_area =  SurfaceArea::from(area);

        let convection_resistance_data: DataUserSpecifiedConvectionResistance 
        = DataUserSpecifiedConvectionResistance { 
            surf_area, 
            heat_transfer_coeff 
        };
        
        let heat_trf_interaction = HeatTransferInteractionType:: 
            UserSpecifiedConvectionResistance(convection_resistance_data);

        // now the time loop begins 
        //

        let mut current_time_simulation_time = Time::new::<second>(0.0);

        while current_time_simulation_time <= *max_time_ptr_in_loop {

            link_heat_transfer_entity(&mut steel_cv_in_loop, 
                &mut ambient_bc_in_loop, 
                heat_trf_interaction).unwrap();


            // let me get the enthalpy out, 
            // there are several nested enums, so it's quite cumbersome 
            // to do it this way. So don't, you're not meant to
            //
            // I might use associated functions or something
            //
            // programming feature comment 4: 
            // create associated function to extract current temperature 
            // value (return a result)

            let cv_type = match steel_cv_in_loop.deref_mut() {
                HeatTransferEntity::ControlVolume(cv_type) => cv_type,
                _ => todo!(),
            };

            let single_cv: &mut SingleCVNode = match cv_type {
                CVType::SingleCV(steel_cv) => steel_cv,
                _ => todo!(),
            };

            let temperature_for_export = 
            try_get_temperature_from_h(
                single_cv.material_control_volume, 
                single_cv.current_timestep_control_volume_specific_enthalpy, 
                pressure).unwrap();

            //panic!("{:?}",temperature_for_export);

            let time_string = current_time_simulation_time.value.to_string();
            let temperature_string = temperature_for_export.value.to_string();

            wtr.write_record(&[time_string,temperature_string])
                .unwrap();

            // might want to add a method in future to simplify this 
            // process

            // programming feature comment 5: 
            // create associated function to 
            // advance timestep 
            // probably need to match the heat transfer entity
            //
            // Also, for FLUID volumes only, the control volume has 
            // a fixed volume but varying density. Be sure to check 
            // that the mass of the CV changes with temperature
            // (i.e mass disappears)
            //
            // let's advance one timestep 
            // so we're not checking Courant Number yet, but 
            // we'll just use the timestep as is.
            // let's sum up enthalpy changes from the vector 

            let mut total_enthalpy_rate_change = 
            Power::new::<watt>(0.0);

            for enthalpy_chg_rate in 
                single_cv.rate_enthalpy_change_vector.clone().iter() {

                    total_enthalpy_rate_change += *enthalpy_chg_rate;
                }

            // if addition operations 
            // correct, we should not have a zero power 
            // change
            assert_ne!(total_enthalpy_rate_change, 
                Power::new::<watt>(0.0));

            // now, add the enthalpy change to the next timestep 
            //

            let enthalpy_next_timestep = total_enthalpy_rate_change * 
            timestep_in_loop.clone() +
            single_cv.current_timestep_control_volume_specific_enthalpy.
                clone()* single_cv.mass_control_volume.clone();

            let specific_enthalpy_next_timestep = 
            enthalpy_next_timestep/single_cv.mass_control_volume.clone();

            single_cv.next_timestep_specific_enthalpy 
                = specific_enthalpy_next_timestep;

            // at the end of each timestep, set 
            // current_timestep_control_volume_specific_enthalpy
            // to that of the next timestep

            single_cv.current_timestep_control_volume_specific_enthalpy
                = specific_enthalpy_next_timestep;

            // clear the vector 

            single_cv.rate_enthalpy_change_vector.clear();

            // update temperature
            single_cv.temperature = temperature_for_export;

            // increase timestep (last step)


            current_time_simulation_time += *timestep_in_loop.deref();
        }

        // with csvs being written,
        // use cargo watch -x test --ignore '*.csv'
        wtr.flush().unwrap();
        

    };

    // I'll probably want to use tokio in future, because the syntax 
    // is quite similar to opc-ua server API 
    // but for now, a thread spawn is enough 

    let calculation_thread = thread::spawn(calculation_loop);

    // in future, might want to have some associated functions which 
    // help construct higs

    calculation_thread.join().unwrap();

    // after finishing the loop, we can extract the data from the 
    // second pointer

    let mut steel_cv_final_state = steel_cv_pointer_final
        .lock().unwrap();

    let cv_type = match steel_cv_final_state.deref_mut() {
        HeatTransferEntity::ControlVolume(cv_type) => cv_type,
        _ => todo!(),
    };

    let single_cv: &mut SingleCVNode = match cv_type {
        CVType::SingleCV(steel_cv) => steel_cv,
        _ => todo!(),
    };

    let enthalpy_vec_final = single_cv.rate_enthalpy_change_vector.
        clone();


    // let's check the final enthalpy vec

    println!("{:?}",enthalpy_vec_final);


    return Ok(());
}


/// test of lumped_heat_capacitance_steel_ball_in_air 
/// with improved API for conveneince
#[test]
//#[ignore = "lumped lumped-capacitance-test takes about 1 min, too long"]
fn lumped_heat_capacitance_steel_ball_in_air_improved_api() 
-> Result<(), TuasLibError>{

    // first, a steel control vol
    // determine parameters first
    let steel = Material::Solid(SolidMaterial::SteelSS304L);
    let pressure = Pressure::new::<atmosphere>(1.0);
    let steel_initial_temperature = 
    ThermodynamicTemperature::new::<degree_celsius>(150.0);

    // Programming feature comment 1:
    // might want a constructor which shortens this process
    // of making the HeatTransferEntity


    let steel_ball_diameter = OuterDiameterThermalConduction::from(
        Length::new::<centimeter>(2.0));
    let diameter: Length = steel_ball_diameter.into();
    let steel_ball_radius = diameter * 0.5;


    // instead of manually constructing the control vol you can just 
    // use this constructor
    let steel_control_vol = 
    SingleCVNode::new_sphere(diameter, steel, steel_initial_temperature, 
        pressure)?;

    let steel_control_vol: HeatTransferEntity = 
        steel_control_vol.into();

    // next thing is the boundary condition 
    // Programming feature comment 2:
    // might want another constructor here too

    let ambient_temperature: ThermodynamicTemperature = 
    ThermodynamicTemperature::new::<degree_celsius>(25.0);

    let ambient_temperature_boundary_condition = 
    HeatTransferEntity::BoundaryConditions(
        BCType::UserSpecifiedTemperature(ambient_temperature));

    // we will be running this using async code similar to what 
    // is used in ciet's opc-ua server

    // make an atomically reference counted pointer with  
    // mutex
    let steel_cv_pointer = Arc::new(
        Mutex::new(
            steel_control_vol
        )
    );

    let ambient_temp_ptr = Arc::new(
        Mutex::new(ambient_temperature_boundary_condition)
    );

    let timestep: Time = Time::new::<second>(20.0);
    let timestep_ptr = Arc::new(
        Mutex::new(timestep)
    );

    let max_time: Time = Time::new::<second>(19200.0);

    let max_time_ptr = Arc::new(max_time);

    // we have to move the Arc pointers into the calculation loop 
    // essentially, ownership is moved to the calculation loop 
    // and after that, when the loop goes out of scope, the 
    // ownership is gone,
    //
    // I need a way to get data outside the loop
    // so i'll use a clone function for the Arc
    // to create a second pointer
    //
    // the first pointer will be dropped in the calculation_loop
    // but the second will survive
    let steel_cv_pointer_final = steel_cv_pointer.clone();


    // this is the calculation that runs every loop
    let calculation_loop = move || {

        // first, I use the mutex lock to lock other threads from 
        // modifying the steel cv
        let mut steel_cv_in_loop = steel_cv_pointer.lock().unwrap();
        let mut ambient_bc_in_loop = ambient_temp_ptr.lock().unwrap();
        let timestep_in_loop = timestep_ptr.lock().unwrap();
        let max_time_ptr_in_loop = max_time_ptr;

        // let's make a csv writer too 

        use csv::Writer;
        let mut wtr = Writer::from_path("air_cooled_steel_sphere_api_test.csv")
            .unwrap();

        wtr.write_record(&["time_seconds","temperature_kelvin"])
            .unwrap();

        // let me create an interaction between the control vol 
        // and bc
        // programming feature comment 3: might want to create 
        // a constructor for this too

        let heat_transfer_coeff = HeatTransfer::new::
            <watt_per_square_meter_kelvin>(20.0);

        let area: Area = 4.0 * PI * steel_ball_radius * steel_ball_radius;
        let surf_area =  SurfaceArea::from(area);

        let convection_resistance_data: DataUserSpecifiedConvectionResistance 
        = DataUserSpecifiedConvectionResistance { 
            surf_area, 
            heat_transfer_coeff 
        };
        
        let heat_trf_interaction = HeatTransferInteractionType:: 
            UserSpecifiedConvectionResistance(convection_resistance_data);

        // now the time loop begins 
        //

        let mut current_time_simulation_time = Time::new::<second>(0.0);

        while current_time_simulation_time <= *max_time_ptr_in_loop {

            link_heat_transfer_entity(&mut steel_cv_in_loop, 
                &mut ambient_bc_in_loop, 
                heat_trf_interaction).unwrap();


            // let me get the enthalpy out, 
            // there are several nested enums, so it's quite cumbersome 
            // to do it this way. So don't, you're not meant to
            //
            // I might use associated functions or something
            //
            // programming feature comment 4: 
            // create associated function to extract current temperature 
            // value (return a result)

            let temperature_for_export = 
            HeatTransferEntity::temperature(steel_cv_in_loop.deref_mut()) 
                .unwrap();

            let time_string = current_time_simulation_time.value.to_string();
            let temperature_string = temperature_for_export.value.to_string();

            wtr.write_record(&[time_string,temperature_string])
                .unwrap();

            // might want to add a method in future to simplify this 
            // process

            // programming feature comment 5: 
            // create associated function to 
            // advance timestep 
            // probably need to match the heat transfer entity
            //
            // Also, for FLUID volumes only, the control volume has 
            // a fixed volume but varying density. Be sure to check 
            // that the mass of the CV changes with temperature
            // (i.e mass disappears)
            //
            // let's advance one timestep 
            // so we're not checking Courant Number yet, but 
            // we'll just use the timestep as is.
            // let's sum up enthalpy changes from the vector 

            HeatTransferEntity::advance_timestep(
                steel_cv_in_loop.deref_mut(),*timestep_in_loop).unwrap();

            current_time_simulation_time += *timestep_in_loop.deref();
        }

        // with csvs being written,
        // use cargo watch -x test --ignore '*.csv'
        wtr.flush().unwrap();
        

    };

    // I'll probably want to use tokio in future, because the syntax 
    // is quite similar to opc-ua server API 
    // but for now, a thread spawn is enough 

    let calculation_thread = thread::spawn(calculation_loop);

    // in future, might want to have some associated functions which 
    // help construct higs

    calculation_thread.join().unwrap();

    // after finishing the loop, we can extract the data from the 
    // second pointer

    let mut steel_cv_final_state = steel_cv_pointer_final
        .lock().unwrap();

    let cv_type = match steel_cv_final_state.deref_mut() {
        HeatTransferEntity::ControlVolume(cv_type) => cv_type,
        _ => todo!(),
    };

    let single_cv: &mut SingleCVNode = match cv_type {
        CVType::SingleCV(steel_cv) => steel_cv,
        _ => todo!(),
    };

    let enthalpy_vec_final = single_cv.rate_enthalpy_change_vector.
        clone();


    // let's check the final enthalpy vec

    println!("{:?}",enthalpy_vec_final);


    return Ok(());
}
