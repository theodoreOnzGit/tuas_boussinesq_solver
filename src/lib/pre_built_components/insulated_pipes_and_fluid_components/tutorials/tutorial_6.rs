
use std::f64::consts::PI;

use uom::si::angle::degree;
use uom::si::f64::*;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::length::{centimeter, foot, inch, meter, micron, millimeter};
use uom::si::mass_density::{gram_per_cubic_centimeter, kilogram_per_cubic_meter};
use uom::si::mass_rate::kilogram_per_second;
use uom::si::power::megawatt;
use uom::si::pressure::{atmosphere, megapascal};
use uom::si::ratio::ratio;
use uom::si::specific_heat_capacity::calorie_per_gram_kelvin;
use uom::si::thermal_conductivity::watt_per_meter_kelvin;
use uom::si::thermodynamic_temperature::{degree_celsius, kelvin};
use uom::si::time::second;

use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
use crate::boundary_conditions::BCType;
use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
use crate::pre_built_components::insulated_pipes_and_fluid_components::InsulatedFluidComponent;
use crate::prelude::beta_testing::{HeatTransferEntity, HeatTransferInteractionType};


/// tutorial 6 covers flow in a gFHR like system
///
/// typically, pressure drop for the loop is around 0.2 MPa, and 
/// mass flowrate around 1200 kg/s
/// https://www.nrc.gov/docs/ML2208/ML22088A231.pdf
#[test]
pub fn gfhr_pipe_with_custom_graphite_material(){

    // First, we construct the pipe as usual

    let initial_temperature = 
        ThermodynamicTemperature::new::<degree_celsius>(500.0);

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


    let pipe_fluid = LiquidMaterial::FLiBe;
    // now we don't have graphite as a material in v0.0.11 
    //
    // but we can construct the material
    // first we need to specify the correlations used 
    // for thermophysical properties 

    // let see the cp 

    // https://webbook.nist.gov/cgi/cbook.cgi?ID=C7782425&Mask=2
    // T = 200 to 3500 K. Least squares fit of 'best' data gives: 
    // Cp = 0.538657 + 9.11129x10-6T - 90.2725T-1 - 43449.3T-2 
    // + 1.59309x107T3 - 1.43688x109T-4 cal/g*K (250 to 3000 K).; DH
    //
    // This applies from T = 200K to 3500K 
    // let's program that in 

    fn cp_graphite(t: ThermodynamicTemperature) -> SpecificHeatCapacity {
        // convert t to kelvin 
        let t_kelvin: f64 = t.get::<kelvin>();

        // the best I can get from this equation 
        // let's just assume it is correct
        let cp_cal_g_kelvin = 0.538657 
            + 9.11129e-6 * t_kelvin 
            - 90.2725 * t_kelvin.powf(-1.0)
            - 43449.3 * t_kelvin.powf(-2.0)
            + 1.59309e7 * t_kelvin.powf(-3.0)
            - 1.43688e9 * t_kelvin.powf(-4.0);

        return SpecificHeatCapacity::new::<calorie_per_gram_kelvin>(
            cp_cal_g_kelvin
        );
    }

    // next let's do thermal conductivity 
    // you can program in anything you like 
    //
    // note that while this is temperature independent,
    // however, the function you program must have a temperature input 
    // and thermal conductivity output
    fn thermal_cond_grpahite(_t: ThermodynamicTemperature) -> ThermalConductivity {
        // I'm going to use the value 36.3
        // for amorphus carbon
        // https://www.nature.com/articles/s41467-023-42195-5
        return ThermalConductivity::new::<watt_per_meter_kelvin>(
            36.3
        );
    }

    //
    // next let's do density
    // you can program in anything you like 
    //
    // note that while this is temperature independent,
    // however, the function you program must have a temperature input 
    // and density output, just as before
    fn density_graphite(_t: ThermodynamicTemperature) -> MassDensity {
        // i'm just going to use 2.20 g/cm3
        //
        // https://en.wikipedia.org/wiki/Graphite

        return MassDensity::new::<gram_per_cubic_centimeter>(
            2.20
        );

    }

    // we also need to specify surface roughness and 
    // the temperature bounds 
    // 
    //
    // I'm giving rough values here 
    // https://www.mdpi.com/2227-9717/9/10/1858


    let surface_roughness_graphite = Length::new::<micron>(1.95);

    // next temperature bounds 
    let graphite_low_temp_bound = ThermodynamicTemperature::new::<kelvin>(200.0);
    let graphite_high_temp_bound = ThermodynamicTemperature::new::<kelvin>(3500.0);

    // now this is how you define a custom material:
    let graphite = 
        SolidMaterial::CustomSolid(
            (graphite_low_temp_bound,graphite_high_temp_bound), 
            cp_graphite, 
            thermal_cond_grpahite, 
            density_graphite, 
            surface_roughness_graphite
        );

    let pipe_shell_material = graphite;
    let insulation_material = graphite;
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


    let inlet_temp = ThermodynamicTemperature::new::<degree_celsius>(1000.0);
    let inlet_bc = BCType::new_const_temperature(inlet_temp);
    let outlet_bc = BCType::new_adiabatic_bc();


    let mut inlet_bc_entity: HeatTransferEntity = inlet_bc.into();
    let mut outlet_bc_entity: HeatTransferEntity = outlet_bc.into();


    let test_pressure = 
        Pressure::new::<megapascal>(0.4);

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

        let test_mass_flowrate = 
            pipe_1
            .get_mass_flowrate_from_pressure_loss_immutable(
                test_pressure);

        // secondly, we create the advection heat transfer interaction
        let advection_heat_transfer_interaction: HeatTransferInteractionType;
        advection_heat_transfer_interaction =
            HeatTransferInteractionType::
            new_advection_interaction(test_mass_flowrate, 
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
            test_mass_flowrate, 
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
        
        // basically, in doing this, we calculate both fluid mechanics 
        // and thermal hydraulics within a single timestep. These are 
        // the basic equations that must for solved for any thermal hydraulics 
        // problem in pipe flow
    }



    let temperature_vector: Vec<ThermodynamicTemperature> = 
        pipe_1.pipe_fluid_array.get_temperature_vector().unwrap();


    let outlet_temperature: ThermodynamicTemperature = 
        *temperature_vector.iter().last().unwrap();

    // now, the outlet temperature is about 134.835C,
    // well not quite, but similar:

    let inlet_temperature: ThermodynamicTemperature = 
        temperature_vector[0];
    // this is also a sensing of how much mass flowrate you get
    let test_mass_flowrate = 
        pipe_1
        .get_mass_flowrate_from_pressure_loss_immutable(
            test_pressure);
    dbg!(&(outlet_temperature,
            inlet_temperature,
            test_mass_flowrate));
    // outlet temp is around 1274 Kelvin
    approx::assert_relative_eq!(
        outlet_temperature.get::<kelvin>(),
        1274.3878112984703,
        max_relative=1e-5
        );

    // inlet temp is around 1274 Kelvin
    approx::assert_relative_eq!(
        inlet_temperature.get::<kelvin>(),
        1273.3269075432977,
        max_relative=1e-5
        );

    // mass flowrate is around 3189 kg/s
    approx::assert_relative_eq!(
        test_mass_flowrate.get::<kilogram_per_second>(),
        3189.23364610169,
        max_relative=1e-5
        );

    // congratulations, you have finished tutorial 6


}

