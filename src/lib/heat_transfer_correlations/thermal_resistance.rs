//! this module  contains functions for thermal resistance
//!
//!
//! will probably need to cite later, [to be done] 
//! take fourier's law for example
//!
//! heat flux = - k dT/dx
//! Q/A  = -k dT/dx
//!
//! the thermal resistance here is the ratio of
//! the driving force (dT) to the heat flow (Q)
//!
//! in non differential form,
//! Q/A = -k (Delta T)/(Delta x)
//! 
//! -(Delta T)/Q = (Delta x)/(kA)
//!
//! and for convection
//!
//! heat flux (surface to fluid) = - h (T_fluid - T_surface)
//! 
//! Q/A = - (Delta T) h
//!
//! (Delta T)/Q = 1/(hA)
//!
//! The unit for thermal resistance here is kelvin per watt
//! 
//! For all intents and purposes however, 
//! we want to find the heat transfer given a set temperature difference
//! and properties of the pipe and etc
//!
//! hence, the output of the functions here will usually be power
//! given various inputs
//!
//!
use std::f64::consts::{PI, E};

use uom::si::temperature_interval;
use uom::si::f64::*;
/// subtracts two thermodynamic temperatures from each
/// other to obtain a temperature interval
///
/// two values are supplied, t1 and t2
///
/// So i'm going to subtract 83F from 600K
/// 83 F is 301.5 K approximately
///
///
/// ```rust
/// extern crate approx;
///
/// use uom::si::{temperature_interval, thermodynamic_temperature};
/// use uom::si::f64::*;
/// use tuas_boussinesq_solver::heat_transfer_correlations::
/// thermal_resistance::subtract_two_thermodynamic_temperatures;
///
/// let t1 = ThermodynamicTemperature::new::
/// <thermodynamic_temperature::kelvin>(600_f64);
///
/// let t2 = ThermodynamicTemperature::new::
/// <thermodynamic_temperature::degree_fahrenheit>(83_f64);
///
/// let expected_temp_value = t1.value - t2.value;
///
/// let test_temp = subtract_two_thermodynamic_temperatures(
/// t1,t2);
/// approx::assert_relative_eq!(expected_temp_value, test_temp.value, 
/// max_relative=0.001);
///
/// ```
///
///
///
pub fn subtract_two_thermodynamic_temperatures(
    t1: ThermodynamicTemperature,
    t2: ThermodynamicTemperature) -> TemperatureInterval {

    let temperature_interval_value = t1.value - t2.value;

    return TemperatureInterval::new::<temperature_interval::
        kelvin>(temperature_interval_value);

}

/// calcualtes heat flow using a thermal resistance model,
/// Q/A = - (Delta T) h
///
/// thermal resistance is:
/// (Delta T)/Q = 1/(hA)
///
/// we assume there are two convection thermal resistances to worry about
/// useful if we have a two fluid flow through
/// a single layer with some thermal resistance
///
/// useful if we have some pipe and insulation
/// and we have hot fluid in the pipe and we want to calculate heat loss
/// to the external environment
pub fn obtain_power_two_convection_two_conduction_thermal_resistance(
    temperature_of_heat_recipient: ThermodynamicTemperature,
    temperature_of_heat_source: ThermodynamicTemperature,
    average_surface_area_1: Area,
    heat_transfer_coefficient_1: HeatTransfer,
    average_surface_area_2: Area,
    heat_transfer_coefficient_2: HeatTransfer,
    average_thermal_conductivity_layer_1: ThermalConductivity,
    average_wall_surface_area_1: Area,
    length_of_wall_1: Length,
    average_thermal_conductivity_layer_2: ThermalConductivity,
    average_wall_surface_area_2: Area,
    length_of_wall_2: Length
    ) -> Power {


    // thermal resistance
    //
    

    // convection thermal ressistance 1
    let thermal_resistance_1 = 
        1.0_f64
        /average_surface_area_1
        /heat_transfer_coefficient_1;


    // convection thermal resistance 2
    let thermal_resistance_2 = 
        1.0_f64
        /average_surface_area_2
        /heat_transfer_coefficient_2;

    // (Delta x)/(kA) for first layer
    let thermal_resistance_3 = 
        length_of_wall_1
        /average_thermal_conductivity_layer_1
        /average_wall_surface_area_1;

    // (Delta x)/(kA) for second layer
    let thermal_resistance_4 = 
        length_of_wall_2
        /average_thermal_conductivity_layer_2
        /average_wall_surface_area_2;

    let thermal_resistance = 
        thermal_resistance_1 
        + thermal_resistance_2
        + thermal_resistance_3
        + thermal_resistance_4;

    // -Delta T
    let temperature_interval = 
        -subtract_two_thermodynamic_temperatures(
            temperature_of_heat_recipient,
            temperature_of_heat_source);


    let heat_flow: Power = 
        temperature_interval
        /thermal_resistance;

    return heat_flow;
}

/// calcualtes heat flow using a thermal resistance model,
/// Q/A = - (Delta T) h
///
/// thermal resistance is:
/// (Delta T)/Q = 1/(hA)
///
/// we assume there are one convection thermal resistances to worry about
/// useful if we want to place a thermal mass inside of the 
/// thermal resistances
///
/// or if we want to calculate the maximum temperature of a heated pebble
/// or cylinder or block
/// a single layer with some thermal resistance
pub fn obtain_power_one_convection_one_conduction_thermal_resistance(
    temperature_of_heat_recipient: ThermodynamicTemperature,
    temperature_of_heat_source: ThermodynamicTemperature,
    average_surface_area_1: Area,
    heat_transfer_coefficient_1: HeatTransfer,
    average_thermal_conductivity: ThermalConductivity,
    average_surface_area: Area,
    length_of_wall: Length
    ) -> Power {


    // thermal resistance
    //
    

    let thermal_resistance_1 = 
        1.0_f64
        /average_surface_area_1
        /heat_transfer_coefficient_1;


    // (Delta x)/(kA) for first layer
    let thermal_resistance_2 = 
        length_of_wall
        /average_thermal_conductivity
        /average_surface_area;

    let thermal_resistance = 
        thermal_resistance_1 
        + thermal_resistance_2;

    // -Delta T
    let temperature_interval = 
        -subtract_two_thermodynamic_temperatures(
            temperature_of_heat_recipient,
            temperature_of_heat_source);


    let heat_flow: Power = 
        temperature_interval
        /thermal_resistance;

    return heat_flow;
}

/// calcualtes heat flow using a thermal resistance model,
/// Q/A = - (Delta T) h
///
/// thermal resistance is:
/// (Delta T)/Q = 1/(hA)
///
/// we assume there are two convection thermal resistances to worry about
/// useful if we have a two fluid flow through
/// a single layer with some thermal resistance
pub fn obtain_power_two_convection_one_conduction_thermal_resistance(
    temperature_of_heat_recipient: ThermodynamicTemperature,
    temperature_of_heat_source: ThermodynamicTemperature,
    average_surface_area_1: Area,
    heat_transfer_coefficient_1: HeatTransfer,
    average_surface_area_2: Area,
    heat_transfer_coefficient_2: HeatTransfer,
    average_thermal_conductivity: ThermalConductivity,
    average_surface_area_thermal_cond: Area,
    length_of_wall: Length
    ) -> Power {


    // thermal resistance
    //
    

    let thermal_resistance_1 = 
        1.0_f64
        /average_surface_area_1
        /heat_transfer_coefficient_1;

    let thermal_resistance_2 = 
        1.0_f64
        /average_surface_area_2
        /heat_transfer_coefficient_2;

    // (Delta x)/(kA) for first layer
    let thermal_resistance_3 = 
        length_of_wall
        /average_thermal_conductivity
        /average_surface_area_thermal_cond;

    let thermal_resistance = 
        thermal_resistance_1 
        + thermal_resistance_2
        + thermal_resistance_3;

    // -Delta T
    let temperature_interval = 
        -subtract_two_thermodynamic_temperatures(
            temperature_of_heat_recipient,
            temperature_of_heat_source);


    let heat_flow: Power = 
        temperature_interval
        /thermal_resistance;

    return heat_flow;
}

/// calcualtes heat flow using a thermal resistance model,
/// Q/A = - (Delta T) h
///
/// thermal resistance is:
/// (Delta T)/Q = 1/(hA)
///
/// we assume there are two convection thermal resistances to worry about
/// useful if we have a two fluid flow througha  diathermal wall
pub fn obtain_power_through_double_convection_thermal_resistance(
    temperature_of_heat_recipient: ThermodynamicTemperature,
    temperature_of_heat_source: ThermodynamicTemperature,
    average_surface_area_1: Area,
    heat_transfer_coefficient_1: HeatTransfer,
    average_surface_area_2: Area,
    heat_transfer_coefficient_2: HeatTransfer,
    ) -> Power {


    // thermal resistance
    //
    

    let thermal_resistance_1 = 
        1.0_f64
        /average_surface_area_1
        /heat_transfer_coefficient_1;

    let thermal_resistance_2 = 
        1.0_f64
        /average_surface_area_2
        /heat_transfer_coefficient_2;

    let thermal_resistance = 
        thermal_resistance_1 
        + thermal_resistance_2;

    // -Delta T
    let temperature_interval = 
        -subtract_two_thermodynamic_temperatures(
            temperature_of_heat_recipient,
            temperature_of_heat_source);


    let heat_flow: Power = 
        temperature_interval
        /thermal_resistance;

    return heat_flow;
}

/// calcualtes heat flow using a thermal resistance model,
/// Q/A = - (Delta T) h
///
/// thermal resistance is:
/// (Delta T)/Q = 1/(hA)
pub fn obtain_power_through_single_convection_thermal_resistance(
    temperature_of_heat_recipient: ThermodynamicTemperature,
    temperature_of_heat_source: ThermodynamicTemperature,
    average_surface_area: Area,
    heat_transfer_coefficient: HeatTransfer,
    ) -> Power {


    // thermal resistance
    //
    

    let thermal_resistance = 
        1.0_f64
        /average_surface_area
        /heat_transfer_coefficient;

    // -Delta T
    let temperature_interval = 
        -subtract_two_thermodynamic_temperatures(
            temperature_of_heat_recipient,
            temperature_of_heat_source);


    let heat_flow: Power = 
        temperature_interval
        /thermal_resistance;

    return heat_flow;
}

/// calcualtes heat flow using a thermal resistance model,
/// -(Delta T)/Q = (Delta x)/(kA)
///
/// assumes there are two layers in the 1D system
pub fn obtain_power_through_two_layer_wall_thermal_resistance(
    temperature_of_heat_recipient: ThermodynamicTemperature,
    temperature_of_heat_source: ThermodynamicTemperature,
    average_thermal_conductivity_layer_1: ThermalConductivity,
    average_thermal_conductivity_layer_2: ThermalConductivity,
    average_surface_area_1: Area,
    average_surface_area_2: Area,
    length_of_wall_1: Length,
    length_of_wall_2: Length) -> Power {

    // (Delta x)/(kA) for first layer
    let thermal_resistance_1 = 
        length_of_wall_1
        /average_thermal_conductivity_layer_1
        /average_surface_area_1;

    // (Delta x)/(kA) for second layer
    let thermal_resistance_2 = 
        length_of_wall_2
        /average_thermal_conductivity_layer_2
        /average_surface_area_2;

    let thermal_resistance = 
        thermal_resistance_1 
        + thermal_resistance_2;

    // -Delta T
    let temperature_interval = 
        -subtract_two_thermodynamic_temperatures(
            temperature_of_heat_recipient,
            temperature_of_heat_source);


    let heat_flow: Power = 
        temperature_interval
        /thermal_resistance;

    return heat_flow;


}

/// calcualtes heat flow using a thermal resistance model,
/// -(Delta T)/Q = (Delta x)/(kA)
pub fn obtain_power_through_wall_thermal_resistance(
    temperature_of_heat_recipient: ThermodynamicTemperature,
    temperature_of_heat_source: ThermodynamicTemperature,
    average_thermal_conductivity: ThermalConductivity,
    average_surface_area: Area,
    length_of_wall: Length) -> Power {

    // (Delta x)/(kA)
    let thermal_resistance = 
        length_of_wall
        /average_thermal_conductivity
        /average_surface_area;

    // -Delta T
    let temperature_interval = 
        -subtract_two_thermodynamic_temperatures(
            temperature_of_heat_recipient,
            temperature_of_heat_source);


    let heat_flow: Power = 
        temperature_interval
        /thermal_resistance;

    return heat_flow;


}


/// thermal resistance for cylindrical annular region 
/// the String Error is temporary, probably need to refine in 
/// later editions with a proper error type
/// // <https://web2.clarkson.edu/projects/subramanian/ch330/notes/Conduction%20in%20the%20Cylindrical%20Geometry.pdf>
///
/// Thermal conductance is just inverse of thermal resistance
/// it is 
/// (2 * pi * L * K)/
/// ln(outer_radius/inner_radius)
///
///
///
#[inline]
pub fn try_get_thermal_conductance_annular_cylinder(
    inner_diameter: Length,
    outer_diameter: Length, 
    cylinder_length: Length,
    k: ThermalConductivity) -> Result<ThermalConductance, String>{

    // check if values are problematic 
    
    if inner_diameter.value <= 0.0 {
        return Err("inner diameter specified is less than or equal\n 
            to zero for annular cylinder thermal conductance".to_string());
    }

    if cylinder_length.value <= 0.0 {
        return Err("cylinder_length specified is less than or equal\n 
            to zero for annular cylinder thermal conductance".to_string());
    }

    if k.value <= 0.0 {
        return Err("negative thermal \n 
            thermal_conductivity supplied".to_string());
    }

    if outer_diameter <= inner_diameter {
        return Err("outer diameter specified is less than or equal\n 
            to inner_diameter for annular cylinder thermal conductance".to_string());
    }

    // Perform calculations

    let numerator: ThermalConductance = 2.0 * PI * cylinder_length * k;

    let outer_radius_divide_by_inner_radius: Ratio = 
    outer_diameter/inner_diameter;

    let denominator: f64 = outer_radius_divide_by_inner_radius
        .value
        .log(E);
    // E here is euler's number, as in 2.718
    // a constant within f64
    //
    //

    return Ok(numerator/denominator);

}

