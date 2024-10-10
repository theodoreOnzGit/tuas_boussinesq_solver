


use uom::si::f64::*;
use uom::si::thermodynamic_temperature::kelvin;

use crate::heat_transfer_correlations::heat_transfer_interactions::*;
use crate::single_control_vol::SingleCVNode;
use crate::thermal_hydraulics_error::ThermalHydraulicsLibError;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::{DataAdvection, HeatTransferInteractionType};

use uom::num_traits::Zero;

use super::try_get_thermal_conductance_based_on_interaction;
/// which calls other functions depending on whether the 
/// heat transfer interaction is conductance based on advection based
#[inline]
pub fn calculate_between_two_singular_cv_nodes(
    single_cv_1: &mut SingleCVNode,
    single_cv_2: &mut SingleCVNode,
    interaction: HeatTransferInteractionType)-> Result<(), ThermalHydraulicsLibError>{


    match interaction {
        HeatTransferInteractionType::UserSpecifiedThermalConductance(_) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::SingleCartesianThermalConductanceOneDimension(_, _) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::DualCartesianThermalConductanceThreeDimension(_) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::DualCartesianThermalConductance(_, _) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::DualCylindricalThermalConductance(_, _, _) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::CylindricalConductionConvectionLiquidOutside(_, _) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::CylindricalConductionConvectionLiquidInside(_, _) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::UserSpecifiedHeatAddition => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::UserSpecifiedHeatFluxCustomArea(_) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::UserSpecifiedHeatFluxCylindricalOuterArea(_, _) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::UserSpecifiedHeatFluxCylindricalInnerArea(_, _) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },
        HeatTransferInteractionType::UserSpecifiedConvectionResistance(_) => {
            calculate_conductance_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                interaction)
        },

        HeatTransferInteractionType::Advection(advection_data) => {
            calculate_advection_interaction_between_two_singular_cv_nodes(
                single_cv_1,
                single_cv_2,
                advection_data)
        },
        HeatTransferInteractionType::SimpleRadiation
            (_area_coeff, _hot_temperature, _cold_temperature) => 
            {

                calculate_conductance_interaction_between_two_singular_cv_nodes(
                    single_cv_1,
                    single_cv_2,
                    interaction)
            }
        ,
    }

}

/// for advection flows between two SingleCVNode objects,
/// and specified advection information,
///
/// this updates the heat transfer vector in both singlecv nodes
#[inline]
pub fn calculate_advection_interaction_between_two_singular_cv_nodes(
    single_cv_1: &mut SingleCVNode,
    single_cv_2: &mut SingleCVNode,
    advection_data: DataAdvection)-> Result<(), ThermalHydraulicsLibError>{

    let mass_flow_from_cv_1_to_cv_2 = advection_data.mass_flowrate;

    // for this, quite straightforward, 
    // get both specific enthalpy of both cvs 
    // calculate power flow 
    // and then update the power vector 
    //

    let specific_enthalpy_cv1: AvailableEnergy = 
    single_cv_1.current_timestep_control_volume_specific_enthalpy;

    let specific_enthalpy_cv2: AvailableEnergy = 
    single_cv_2.current_timestep_control_volume_specific_enthalpy;

    // calculate heat rate 

    let heat_flowrate_from_cv_1_to_cv_2: Power 
    = advection_heat_rate(mass_flow_from_cv_1_to_cv_2,
        specific_enthalpy_cv1,
        specific_enthalpy_cv2,)?;

    // by default, cv 1 is on the left, cv2 is on the right 
    //

    single_cv_1.rate_enthalpy_change_vector.
        push(-heat_flowrate_from_cv_1_to_cv_2);
    single_cv_2.rate_enthalpy_change_vector.
        push(heat_flowrate_from_cv_1_to_cv_2);

    // relevant timescale here is courant number
    //
    // the timescale can only be calculated after the mass flows 
    // in and out of the cv are sufficiently calculated
    // the only thing we can do here is push the mass flowrate 
    // into the individual mass flowrate vectors 
    //
    // by convention, mass flowrate goes out of cv1 and into cv2 
    // so positive mass flowrate here is positive for cv2 
    // and negative for cv1
    //
    // I'll need a density for the flow first

    let density_cv1 = advection_data.fluid_density_heat_transfer_entity_1;
    let density_cv2 = advection_data.fluid_density_heat_transfer_entity_2;

    let volumetric_flowrate: VolumeRate;

    if mass_flow_from_cv_1_to_cv_2 > MassRate::zero() {
        // if mass flowrate is positive, flow is moving from cv1 
        // to cv2 
        // then the density we use is cv1 

        volumetric_flowrate = mass_flow_from_cv_1_to_cv_2/density_cv1;

    } else {
        // if mass flowrate is positive, flow is moving from cv2
        // to cv1
        // then the density we use is cv2

        volumetric_flowrate = mass_flow_from_cv_1_to_cv_2/density_cv2;
    }

    // now that I've done the volume flowrate calculation, push the 
    // volumetric flowrate to each vector
    single_cv_1.volumetric_flowrate_vector.push(
        -volumetric_flowrate);
    single_cv_2.volumetric_flowrate_vector.push(
        volumetric_flowrate);

    

    // done! 
    Ok(())
}

/// if two singleCV nodes have a conductance or thermal resistance between 
/// them, their temperature differentials and conductance are used to 
/// calculate the heat flow between them.
#[inline]
pub fn calculate_conductance_interaction_between_two_singular_cv_nodes(
    single_cv_1: &mut SingleCVNode,
    single_cv_2: &mut SingleCVNode,
    interaction: HeatTransferInteractionType)-> Result<(), ThermalHydraulicsLibError>{

    // let's get the two temperatures of the control volumes first
    // so let me get the enthalpies, and then their respective 
    // temperatures 

    // we'll also need to get their pressures 
    let single_cv_1_pressure = single_cv_1.pressure_control_volume;
    let single_cv_2_pressure = single_cv_2.pressure_control_volume;

    // we will now get their respective temperatures 
    //
    // (note, this is extremely computationally expensive as it 
    // is iterative in nature)
    //
    // two solutions here, 
    // one: store cv temperature in single cv, 
    // so it can be readily accessed
    //
    // two: cheaper method of getting t from h.
    

    let single_cv_1_temperature: ThermodynamicTemperature;
    let single_cv_2_temperature: ThermodynamicTemperature;

    single_cv_1_temperature = single_cv_1.temperature;
    single_cv_2_temperature = single_cv_2.temperature;


    // now that we got their respective temperatures we can calculate 
    // the thermal conductance between them
    //
    // for conduction for instance, q = kA dT/dx 
    // conductance is watts per kelvin or 
    // q = (kA)/dx * dT
    // conductance here is kA/dx
    // thermal resistance is 1/conductance
    //
    // for convection, we get: 
    // q = hA (Delta T)
    // hA becomes the thermal conductance
    //
    // If we denote thermal conductance as Htc
    // 
    // Then a general formula for heat flowing from 
    // temperature T_1 to T_2 is 
    //
    // T_1 --> q --> T_2 
    //
    // q = - Htc (T_2 - T_1)

    // 
    let thermal_conductance = try_get_thermal_conductance_based_on_interaction(
        single_cv_1_temperature, 
        single_cv_2_temperature,
        single_cv_1_pressure, 
        single_cv_2_pressure, 
        interaction)?;

    // suppose now we have thermal conductance, we can now obtain the 
    // power flow
    //

    let cv_2_temp_minus_cv_1_temp_kelvin: f64 = 
        single_cv_2_temperature.get::<kelvin>() - 
        single_cv_1_temperature.get::<kelvin>();

    let cv_2_temp_minus_cv_1: TemperatureInterval = 
    TemperatureInterval::new::<uom::si::temperature_interval::kelvin>(
        cv_2_temp_minus_cv_1_temp_kelvin);

    let heat_flowrate_from_cv_1_to_cv_2: Power = 
    - thermal_conductance * cv_2_temp_minus_cv_1;

    // now, we add a heat loss term to cv_1 
    // and a heat gain term to cv_2 
    //
    // using timestep
    // the signs should cancel out

    single_cv_1.rate_enthalpy_change_vector.
        push(-heat_flowrate_from_cv_1_to_cv_2);
    single_cv_2.rate_enthalpy_change_vector.
        push(heat_flowrate_from_cv_1_to_cv_2);


    // for solids mesh fourier number need only 
    // be done once, not every time 
    // an interaction is formed 
    //
    // probably the cell stability fourier number will be done in the 
    // constructor. however, with convection, the time scale must be 
    // recalculated at every time step. so it really depends whether 
    // it's solid or fluid control volume
    // Actually, for solid CV, I will also need to recalculate time scale 
    // based on the material thermal thermal_diffusivity
    //
    // For liquid CV, still need to calculate time scale based 
    // on convection flow
    // match statement is meant to tell that liquid CVs are not quite 
    // ready for use
    //
    // but the liquid timescales are calculated at the cv level only 
    // after all volumetric flowrates are calculated
    //
    // so don't really need any new timescale calculations

    return Ok(());

}
