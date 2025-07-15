
use std::f64::consts::PI;

use uom::si::angle::degree;
use uom::si::f64::*;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::length::{centimeter, foot, inch, meter, millimeter};
use uom::si::mass_rate::kilogram_per_second;
use uom::si::pressure::{atmosphere, pascal};
use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::degree_celsius;

use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
use crate::boussinesq_thermophysical_properties::{LiquidMaterial, SolidMaterial};
use crate::pre_built_components::insulated_pipes_and_fluid_components::InsulatedFluidComponent;

/// This first test documents how to use TUAS 
/// to solve for fluid flow in a single pipe
///
/// getting pressure drop using mass flowrate
/// 
#[test]
pub fn fluid_mechanics_basics(){

    // first we create a pipe object 
    // which can be intimidating...
    //
    // but just follow the guide, and do copy and paste 
    // the existing code so you can suit it to your own examples

    // the easiest way is to construct an insulated pipe 
    // of course, fluid doesn't just flow through pipes,
    // the pipes themselves could be shaped in ways other
    // than a circular pipe.
    //
    // I therefore call these objects InsulatedFluidComponent
    // objects
    //
    //
    // now to even construct one, you need several pieces of 
    // information:

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

    // for circular pipes, the inner diameter, hydraulic 
    // diameter are the same

    let shell_id = Length::new::<meter>(0.5);
    let hydraulic_diameter = shell_id;
    let flow_area = PI * 0.25 * hydraulic_diameter * hydraulic_diameter;
    let shell_od = shell_id + Length::new::<centimeter>(2.0);
    let insulation_thickness = Length::new::<inch>(1.0);
    let pipe_length = Length::new::<foot>(30.0);
    
    // next, the hydraulic form losses 
    // and surface roughness

    let form_loss = Ratio::new::<ratio>(5.0);
    let surface_roughness = Length::new::<millimeter>(1.0);

    // for heat transfer, you are supposed to specify materials 
    // and heat transfer to ambient 
    // using existing materials
    // your liquid can also be some pre-defined fluid such as FLiNak,
    // FLiBe or Dowtherm A (a.k.a Therminol VP-1).
    //
    // Now, these won't be used for calculating heat loss 
    // in this tutorial, but this will be useful in future.

    let pipe_shell_material = SolidMaterial::SteelSS304L;
    let insulation_material = SolidMaterial::Fiberglass;
    let pipe_fluid = LiquidMaterial::TherminolVP1;
    let htc_to_ambient = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(20.0);

    // lastly, you specify the nodalisation of the pipe 
    // this is very important for heat transfer, less 
    // so for fluid mechanics 
    //
    // By default, pipes InsulatedFluidComponent as of 
    // v0.0.11 will always have at least two nodes.
    // You then specify the number of inner nodes.
    //
    // so for a pipe with five nodes in total, 
    // we specify 5 - 2 = 3 inner nodes 
    //
    // let's do an example for seven total nodes 
    // for this, the number of inner nodes is 
    // 7 - 2 nodes = 5 inner nodes 
    //
    // I call these "inner nodes" because 
    // the two nodes outside are the "periphery" of the pipe 
    //
    // So for a seven node system, we have the following 
    // node (or control volume) structure:
    //
    // (outer node 1) -- 2 -- 3 -- 4 -- 5 -- 6 -- (outer node 7)
    //
    // so we have 5 inner nodes. This is important for heat 
    // transfer and not the fluid mechanics part.
    //
    // 
    // 

    let user_specified_inner_nodes = 5;

    // now, please take note of the imports from the units of measure (uom) crate on top.

    // now we use this function to make our new pipe 

    let pipe_1 = 
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

    // now, one can specify a mass flowrate, saw 100 kg/s 
    // and obtain a pressure drop 

    let test_mass_flowrate = 
        MassRate::new::<kilogram_per_second>(100.0);

    // the pressure drop here is by default calculated using 
    // churchill's correlation
    //
    // to do so, we use:
    let test_pressure_drop = 
        pipe_1.get_pressure_loss_immutable(test_mass_flowrate);

    // we should get a pressure drop, which you can view using 
    // the dbg! macro, which works like printing the result out 
    dbg!(&test_pressure_drop);

    // you can also use the println macro to do it 
    println!("{:?}",test_pressure_drop);

    // i prefer the former, the syntax is easier 
    

    // congrats, you have gone through tutorial 1 
    //
    // now every of these tutorials is also a 
    // regression test
    //
    // tests are basically like tutorial 
    // examples, except that they must reproduce 
    // certain results
    //
    // for example, the pressure drop here is 683.38 Pa
    //
    // And whenever I change my code, the pressure drop 
    // should reproduce the same value
    // so to do so, I use assertions. This is what the 
    // syntax looks like:
    
    approx::assert_relative_eq!(
        test_pressure_drop.get::<pascal>(),
        683.38,
        max_relative=1e-5
        );

    // so if you see this piece of code above, just know that 
    // it is there to ensure that results are correctly 
    // reproduced to some tolerance 
    // in this case, the pressure drop in pascals i should 
    // get is 683.38

}
