use std::f64::consts::PI;

use uom::si::angle::degree;
use uom::si::f64::*;
use uom::si::heat_transfer::watt_per_square_meter_kelvin;
use uom::si::length::{centimeter, foot, inch, meter, millimeter};
use uom::si::mass_density::kilogram_per_cubic_meter;
use uom::si::mass_rate::kilogram_per_second;
use uom::si::pressure::atmosphere;
use uom::si::ratio::ratio;
use uom::si::thermodynamic_temperature::degree_celsius;

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


    





}

