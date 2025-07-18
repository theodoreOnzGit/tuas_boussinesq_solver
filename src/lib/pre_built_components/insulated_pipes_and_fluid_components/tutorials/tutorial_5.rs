use std::f64::consts::PI;

use uom::si::angle::degree;
use uom::si::f64::*;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::length::{centimeter, foot, inch, meter, millimeter};
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::power::megawatt;
use uom::si::pressure::{atmosphere, pascal};
use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::time::second;

use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
use crate::boundary_conditions::BCType;
use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
use crate::pre_built_components::insulated_pipes_and_fluid_components::InsulatedFluidComponent;
use crate::prelude::beta_testing::{HeatTransferEntity, HeatTransferInteractionType};


/// Now that you've finished tutorial 4, which talks about 
/// calculating heat transfer through pipes, we want to move on 
/// to calculating BOTH heat transfer and fluid mechanics in every 
/// timestep
///
/// This tutorial shows how to do it without any parallelisation 
/// of threads, hence it is called serial computation
///
/// 
#[test]
pub fn thermal_hydraulics_flow_through_a_pipe_serial_computation(){

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


    let inlet_temp = ThermodynamicTemperature::new::<degree_celsius>(80.0);
    let inlet_bc = BCType::new_const_temperature(inlet_temp);
    let outlet_bc = BCType::new_adiabatic_bc();


    let mut inlet_bc_entity: HeatTransferEntity = inlet_bc.into();
    let mut outlet_bc_entity: HeatTransferEntity = outlet_bc.into();


    let test_pressure = 
        Pressure::new::<pascal>(683.38);

    let dummy_therminol_density = 
        MassDensity::new::<kilogram_per_cubic_meter>(1.0);




    let heater_power = Power::new::<megawatt>(10.0);
    let timestep = Time::new::<second>(0.1);
    let simulation_endtime = Time::new::<second>(300.0);
    let mut current_simulation_time = Time::new::<second>(0.0);

    // now let's begin our flow loop as usual
    while current_simulation_time < simulation_endtime {

        // let's do our calculation steps for each timestep 
        // firstly, fluid mechanics
        // 
        // we calculate the mass flowrate given a pressure drop
        //
        // I should get about 100 kg/s flowrate as per the last tests

        let test_mass_flowrate_100_kg_per_s = 
            pipe_1
            .get_mass_flowrate_from_pressure_loss_immutable(
                test_pressure);

        // secondly, we create the advection heat transfer interaction
        let advection_heat_transfer_interaction: HeatTransferInteractionType;
        advection_heat_transfer_interaction =
            HeatTransferInteractionType::
            new_advection_interaction(test_mass_flowrate_100_kg_per_s, 
                dummy_therminol_density, 
                dummy_therminol_density);

        // thirdly, to link the pipe to the inlet and outlet BCs
        pipe_1.pipe_fluid_array.link_to_back(
            &mut inlet_bc_entity, 
            advection_heat_transfer_interaction)
            .unwrap();

        pipe_1.pipe_fluid_array.link_to_front(
            &mut outlet_bc_entity, 
            advection_heat_transfer_interaction)
            .unwrap();

        // fourthly, to perform calculations within the pipe for radial 
        // (lateral)
        // heat transfer interactions 
        pipe_1.lateral_and_miscellaneous_connections_no_wall_correction(
            test_mass_flowrate_100_kg_per_s, 
            heater_power)
            .unwrap();

        // fifth, we advance the timestep to update the temperatures 
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

