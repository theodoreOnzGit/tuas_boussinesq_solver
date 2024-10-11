use uom::si::f64::*;

use super::{churchill_friction_factor, dimensionalisation};
use crate::thermal_hydraulics_error::TuasLibError;

/// a function calculates pressure
/// loss given a mass flowrate and pipe properties
pub fn pipe_calc_pressure_loss(
    mut fluid_mass_flowrate: MassRate,
    cross_sectional_area: Area,
    hydraulic_diameter: Length,
    fluid_viscosity: DynamicViscosity,
    fluid_density: MassDensity,
    pipe_length: Length,
    absolute_roughness: Length,
    form_loss_k: f64) -> Result<Pressure,TuasLibError> {
    // first let's calculate roughness ratio

    let roughness_ratio_quantity = absolute_roughness/hydraulic_diameter;

    let roughness_ratio = roughness_ratio_quantity;

    // second i want to take care of reverse flow

    let mut reverse_flow = false;
    if fluid_mass_flowrate.value < 0.0 {
        reverse_flow = true;
    }

    if reverse_flow {
        fluid_mass_flowrate = fluid_mass_flowrate * -1.0;
    }

    // and let's get the reynolds_number and L/D
    let reynolds_number = dimensionalisation::calc_reynolds_from_mass_rate(
        fluid_mass_flowrate,
        cross_sectional_area,
        hydraulic_diameter,
        fluid_viscosity);

    let length_to_diameter_ratio 
        = pipe_length/hydraulic_diameter;

    // then let's obtain the pipe Bejan Number
    // given the Re

    let bejan_number = churchill_friction_factor::get_bejan_number_d(
        reynolds_number.into(),
        roughness_ratio.into(),
        length_to_diameter_ratio.into(),
        form_loss_k)?;

    // once we get bejan_number, we can get the pressure loss terms
    //
    let pressure_loss = dimensionalisation::calc_bejan_to_pressure(
        bejan_number,
        hydraulic_diameter,
        fluid_density,
        fluid_viscosity);


    // now before i exit, i want to make sure reverse flow is taken care
    // of
    if reverse_flow {
        return Ok(pressure_loss * -1.0);
    }

    return Ok(pressure_loss);
}



/// a function which calculates pressure
/// loss given a mass flowrate and pipe properties
pub fn pipe_calc_mass_flowrate(
    pressure_loss: Pressure,
    cross_sectional_area: Area,
    hydraulic_diameter: Length,
    fluid_viscosity: DynamicViscosity,
    fluid_density: MassDensity,
    pipe_length: Length,
    absolute_roughness: Length,
    form_loss_k: f64) -> Result<MassRate,TuasLibError> {

    // first let's get our relevant ratios:
    let roughness_ratio_quantity = absolute_roughness/hydraulic_diameter;

    let roughness_ratio = roughness_ratio_quantity;

    let length_to_diameter_ratio 
        = pipe_length/hydraulic_diameter;

    // then get Bejan number:

    let bejan_number_calculated_using_diameter = 
        dimensionalisation::calc_bejan_from_pressure(
            pressure_loss, hydraulic_diameter, 
            fluid_density, fluid_viscosity);

    // let's get Re
    let reynolds_number_calculated_using_diameter = 
        churchill_friction_factor::get_reynolds_from_bejan(
            bejan_number_calculated_using_diameter.into(),
            roughness_ratio.into(),
            length_to_diameter_ratio.into(),
            form_loss_k)?;


    // and finally return mass flowrate
    //
    let fluid_mass_flowrate = 
        dimensionalisation::calc_reynolds_to_mass_rate(
            cross_sectional_area,
            reynolds_number_calculated_using_diameter.into(),
            hydraulic_diameter,
            fluid_viscosity);

    return Ok(fluid_mass_flowrate);

}
