use std::f64::consts::PI;

use uom::si::angle::degree;
use uom::si::f64::*;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::length::{centimeter, foot, inch, meter, millimeter};
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::mass_rate::kilogram_per_second;
use uom::si::power::megawatt;
use uom::si::pressure::atmosphere;
use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::time::{minute, second};

use crate::boundary_conditions::BCType;
use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
use crate::pre_built_components::insulated_pipes_and_fluid_components::InsulatedFluidComponent;
use crate::prelude::beta_testing::{HeatTransferEntity, HeatTransferInteractionType};


/// Thermal hydraulics is more than just about fluid mechanics,
/// you also need to calculate heat transfer as well 
///
/// This is done manually (you can ignore this if you just want to do 
/// isothermal calcs)
/// 
/// Now, in this tutorial, we shall perform heated flow through a pipe 
/// using assuming fluid flows in at 100 kg/s, at 80 degrees C 
/// the pipe itself will then have a heating power of 10 MW 
///
/// we know from CIET, dowtherm flowing at 0.18 kg/s will be heated to 
/// from 80 - 110 degC by about 10 kW 
///
/// to produce the same temperature change in 100 kg/s of flow, 
/// we need about 10 kW * 100/0.18 =  5555.56 kW (5.56 MW)
/// 
/// hence, if we apply 10 MW, 
/// we expect the temperature at the outlet to be around 130-140 degC 
/// at steady state, assuming the specific heat capacity doesn't change 
/// too much
///
/// let's get started
///
/// 
#[test]
pub fn heated_flow_through_a_pipe(){

    // First, we construct the pipe as usual

    let initial_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(50.0);

    let ambient_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(25.0);

    let fluid_pressure = 
        Pressure::new::<atmosphere>(1.0);
    
    let solid_pressure = 
        Pressure::new::<atmosphere>(1.0);

    let incline_angle = 
        Angle::new::<degree>(0.0);


    let shell_id = Length::new::<meter>(0.5);
    let hydraulic_diameter = shell_id;
    let flow_area = PI * 0.25 * hydraulic_diameter * hydraulic_diameter;
    let shell_od = shell_id + Length::new::<centimeter>(2.0);
    let insulation_thickness = Length::new::<inch>(1.0);
    let pipe_length = Length::new::<foot>(30.0);
    

    let form_loss = Ratio::new::<ratio>(5.0);
    let surface_roughness = Length::new::<millimeter>(1.0);


    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);


    let user_specified_inner_nodes = 5;

    // note that for heat transfer, pipe components need to be 
    // mutable, after all, we expect the pipe temperatures to change 
    // after the calculation steps right? 
    //
    // Therefore, the pipes themselves need to be mutable
    //
    // hence the mut here:

    let mut pipe_1 = 
        InsulatedFluidComponent::new_insulated_pipe(
            initial_temperature, 
            ambient_temperature, 
            fluid_pressure, 
            solid_pressure, 
            flow_area, 
            incline_angle, 
            form_loss, 
            shell_id, 
            shell_od, 
            insulation_thickness, 
            pipe_length, 
            hydraulic_diameter, 
            pipe_shell_material, 
            insulation_material, 
            pipe_fluid, 
            htc_to_ambient, 
            user_specified_inner_nodes, 
            surface_roughness);

    // now for heat transfer calculation, these pipes need to be connected 
    // to boundary conditions (BC)

    // for this case, the inlet BC is constant temperature,
    // 80 degrees C 
    // and the outlet BC is just adiabatic.
    //
    // let's construct them here 

    let inlet_temp = ThermodynamicTemperature::new::<degree_celsius>(80.0);
    let inlet_bc = BCType::new_const_temperature(inlet_temp);
    let outlet_bc = BCType::new_adiabatic_bc();

    // now before each BC can be connected properly, they need 
    // to be converted into HeatTransferEntity objects first, 
    // to do so, I do the following:

    let mut inlet_bc_entity: HeatTransferEntity = inlet_bc.into();
    let mut outlet_bc_entity: HeatTransferEntity = outlet_bc.into();
    // of course, there is a more concise way to shrink these steps 
    // together within one line, but that would be confusing for a tutorial 
    // so for now, I'll write these out explicitly

    // note also that for interactions, they also need to be mutable
    // while BCs will not change during course of the calculation, 
    // they still need to be mutable from a programming standpoint.
    //
    // There is a programming explanation for this, 
    // but I'm not going to write it down yet, because it is talking about 
    // the programming structure, of TUAS in Rust rather than based on 
    // the underlying physics and engineering principles of the system 
    // Just take it as it is for now, that this will need to be mutable.


    // now, next step is to indicate that advection occurs 
    // between the BCs and the pipe 
    //
    // for advection calculations, we know 
    // that timesteps are constrained by the 
    // courant number. Ie the ratio of the timestep to the residence 
    // time inside the fluid volume 
    // 
    // if timesteps are too large, the simulation becomes unstable 
    //
    // to calculate courant number, 
    // we use the ratio of volumetric flowrates into 
    // the control volume to the volume of said control volume
    //
    // this is shown in the SingelCVNode part
    //
    // now, to get the volumetric flowrates, we need to obtain 
    // appropriate densities. This density will change depending 
    // on flow direction 
    //
    // In this case, of forward flow:
    // (v1) ---> (v2) ---> (v3)
    //
    // we take the density of fluid coming from v1 as the 
    // density of fluid flowing into v2 
    //
    // in the case of backflow:
    //
    // (v1) <--- (v2) <--- (v3) 
    //
    // we take the volume of v3 as the density of fluid going into 
    // v2
    //
    // if we don't care about courant number calculations,
    // we can skip just give any old density
    //

    

    // Now, with that in mind, we know that from the boundary conditions 
    // into the pipe, we have advection going on,
    // so let's create the heat transfer interaction 
    //
    //
    let advection_heat_transfer_interaction: HeatTransferInteractionType;

    // for this we will need a mass flowrate, 
    let test_mass_flowrate_100_kg_per_s = 
        MassRate::new::<kilogram_per_second>(100.0);

    // and then the densities as explained before. 
    // However, note that this is only important for 
    // calculating the courant number later on.
    // Doesn't really matter as much in this tutorial
    //
    let dummy_therminol_density = 
        MassDensity::new::<kilogram_per_cubic_meter>(1.0);


    advection_heat_transfer_interaction =
            HeatTransferInteractionType::
            new_advection_interaction(test_mass_flowrate_100_kg_per_s, 
                dummy_therminol_density, 
                dummy_therminol_density);

    // now that this interaction has been created, we can 
    // link up the pipe to the boundary conditions.

    // for advection heat transfer interactions, 
    // the positive flow direction convention used in TUAS is that 
    // flow from the "back" of the pipe to the "front" of the pipe is positive
    //

    // so the inlet bc is at the "back" and outlet bc is at the 
    // "front"
    //

    pipe_1.pipe_fluid_array.link_to_back(
        &mut inlet_bc_entity, 
        advection_heat_transfer_interaction)
        .unwrap();

    pipe_1.pipe_fluid_array.link_to_front(
        &mut outlet_bc_entity, 
        advection_heat_transfer_interaction)
        .unwrap();
    
    // now that the components are linked up
    // we also need to compute the radial conduction between 
    // the pipe fluid, metallic shell, insulation and surroundings
    //
    // it is also here that one inputs the power that is supplied into 
    // the pipe shell 
    //
    // over here, you will also specify if you want the wall correction 
    // for the Gnielinksi correlation to be switched on or off.

    let heater_power = Power::new::<megawatt>(10.0);

    pipe_1.lateral_and_miscellaneous_connections_no_wall_correction(
        test_mass_flowrate_100_kg_per_s, 
        heater_power)
        .unwrap();

    // now after all this, we are able to make calculations to proceed 
    // to the next timestep, that is to update the temperatures within 
    // the pipe based on the mass flowrates, boundary conditions, heater 
    // power and so on.

    let timestep = Time::new::<second>(0.1);

    // the choice of timestep depends on Courant number, among other 
    // timescales. 
    //
    // it will need to be small enough so that numerical instabilities 
    // do not occur, and for the calculations to be sufficiently 
    // accurate.

    pipe_1.advance_timestep(timestep).unwrap();
    inlet_bc_entity.advance_timestep_mut_self(timestep).unwrap();
    outlet_bc_entity.advance_timestep_mut_self(timestep).unwrap();
    
    

    // after advancing the timestep, you have completed ONE 
    // timestep only. You will need to repeat this several times 
    // throughout the simulation.

    // now, the inlet and outlet bcs don't technically need to advance 
    // timestep, but it is good practice to just advance the timestep 
    // of every heat transfer entity for the sake of consistency
    //
    // you can choose not to do it, but don't miss any component out!


    //
    // Suppose now we want to get this simulation to some steady 
    // state.
    //
    // we can specify an estimated simulation endtime (you can do this 
    // through trial and error) 

    // and the basic structure is to use a while loop 

    let simulation_endtime = Time::new::<second>(300.0);

    // then let's create a variable indicating the current simulation 
    // time, we of course start at zero

    let mut current_simulation_time = Time::new::<second>(0.0);

    // the setup looks like this:

    while current_simulation_time < simulation_endtime {

        // do your calculation steps here... 
        //

        // then at the end of the calculations, 
        // update the current simulation time by adding the timestep

        current_simulation_time += timestep;
    }

    // basically programming this way, the loop repeats until the 
    // current_simulation_time reaches the simulation_endtime, thereabout

    // now we are going to add the calculation steps,
    // let's set the current_simulation_time

    current_simulation_time = Time::new::<second>(0.0);

    while current_simulation_time < simulation_endtime {

        // let's do our calculation steps for each timestep 
        //

        // firstly, to link the pipe to the inlet and outlet BCs
        pipe_1.pipe_fluid_array.link_to_back(
            &mut inlet_bc_entity, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_1.pipe_fluid_array.link_to_front(
            &mut outlet_bc_entity, 
            advection_heat_transfer_interaction)
            .unwrap();

        // secondly, to perform calculations within the pipe for radial 
        // (lateral)
        // heat transfer interactions 
        pipe_1.lateral_and_miscellaneous_connections_no_wall_correction(
            test_mass_flowrate_100_kg_per_s, 
            heater_power)
            .unwrap();

        // thirdly, we advance the timestep to update the temperatures 
        // within the pipe
        pipe_1.advance_timestep(timestep).unwrap();
        inlet_bc_entity.advance_timestep_mut_self(timestep).unwrap();
        outlet_bc_entity.advance_timestep_mut_self(timestep).unwrap();


        // then at the end of the calculations, 
        // update the current simulation time by adding the timestep

        current_simulation_time += timestep;
    }


    // now for a bit of postprocessing,
    // let's get the outlet temperature 
    //
    // first we get the temperature profile of the pipe fluid array:

    let temperature_vector: Vec<ThermodynamicTemperature> = 
        pipe_1.pipe_fluid_array.get_temperature_vector().unwrap();

    // second, we get the last element of the temperature vector,
    // which by convention, is the "front" of the pipe

    let outlet_temperature: ThermodynamicTemperature = 
        *temperature_vector.iter().last().unwrap();

    // now, the outlet temperature is about 134.835C:

    approx::assert_relative_eq!(
        outlet_temperature.get::<degree_celsius>(),
        134.835,
        max_relative=1e-5
        );

    // we can run the test for another 10 minutes of simulation time 
    
    let simulation_endtime_10_min = Time::new::<minute>(10.0);
    current_simulation_time = Time::new::<second>(0.0);

    while current_simulation_time < simulation_endtime_10_min {

        // let's do our calculation steps for each timestep 
        //

        // firstly, to link the pipe to the inlet and outlet BCs
        pipe_1.pipe_fluid_array.link_to_back(
            &mut inlet_bc_entity, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_1.pipe_fluid_array.link_to_front(
            &mut outlet_bc_entity, 
            advection_heat_transfer_interaction)
            .unwrap();

        // secondly, to perform calculations within the pipe for radial 
        // (lateral)
        // heat transfer interactions 
        pipe_1.lateral_and_miscellaneous_connections_no_wall_correction(
            test_mass_flowrate_100_kg_per_s, 
            heater_power)
            .unwrap();

        // thirdly, we advance the timestep to update the temperatures 
        // within the pipe
        pipe_1.advance_timestep(timestep).unwrap();
        inlet_bc_entity.advance_timestep_mut_self(timestep).unwrap();
        outlet_bc_entity.advance_timestep_mut_self(timestep).unwrap();


        // then at the end of the calculations, 
        // update the current simulation time by adding the timestep

        current_simulation_time += timestep;
    }

    // let's do postprocessing again:
    let temperature_vector: Vec<ThermodynamicTemperature> = 
        pipe_1.pipe_fluid_array.get_temperature_vector().unwrap();

    // second, we get the last element of the temperature vector,
    // which by convention, is the "front" of the pipe

    let outlet_temperature: ThermodynamicTemperature = 
        *temperature_vector.iter().last().unwrap();

    // after 10 more minutes, the outlet temperature 
    // is about 134.835C:

    approx::assert_relative_eq!(
        outlet_temperature.get::<degree_celsius>(),
        134.835,
        max_relative=1e-5
        );

    // note that the temperature of the last element of the pipe is 
    // the outlet temperature because the control volume is well mixed. 
    // The temperature of the fluid flow coming out of the 
    // last control volume of the pipe is the same as the control 
    // volume of the pipe

    // now, if you want the inlet temperature, 
    // then we get the first 
    // element of the vector 
    // (we can access this using index 0):
    
    let inlet_temperature: ThermodynamicTemperature = 
        temperature_vector[0];


    // the inlet temperature is about 88.128C
    approx::assert_relative_eq!(
        inlet_temperature.get::<degree_celsius>(),
        88.1282,
        max_relative=1e-5
        );

    // congratulations, you have finished tutorial 4


}
