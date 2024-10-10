use ndarray::*;

use uom::si::f64::*;
use uom::si::power::watt;
use uom::si::thermal_conductance::watt_per_kelvin;
use uom::si::thermodynamic_temperature::kelvin;

use crate::boussinesq_thermophysical_properties::specific_enthalpy::try_get_h;
use crate::single_control_vol::SingleCVNode;
use crate::thermal_hydraulics_error::ThermalHydraulicsLibError;

use super::standalone_fluid_nodes::solve_conductance_matrix_power_vector;


/// This is mostly a direct translation of GeN-Foam code, 
///
/// Now, this code is meant to calculate the internal temperature 
/// profile given an innermost temperature node, as well as
/// an outer cooling boundary condition
///
/// Here is the diagram provided from lumpedNuclearStructure.H:
/// Tmax            T[0]           T[1]          T[n-1]         Tsurface      
///   | --- H[0] --- | --- H[1] --- | --- H[2] --- |  --- H[n] --- |  
///
///
/// for this code, i intend to adapt it as follows
///
/// Tmax            T[0]           T[1]          T[n-1]         T_ambient
/// [ignored]       T_singleCV                              
///   | --- H[0] --- | --- H[1] --- | --- H[2] --- |  --- H[n] --- |  
///        [ignored]
///
/// There should be SingleCVNode on the inside, which gives the array 
/// cv a way to interact with other single CV nodes or other 
/// HeatTransferEntity objects
///
/// you must feed in a arrays of nodesNumber length long 
/// and nodesNumber + 1 for the thermal conductance arrays
#[inline]
pub (crate) fn _advance_timestep_for_externally_cooled_array_cv_no_insulation(
    inner_single_cv: &mut SingleCVNode,
    nodes_number: usize,
    dt: Time,
    total_volume: Volume,
    q: Power,
    coolant_temp: ThermodynamicTemperature,
    thermal_conductance_to_coolant: ThermalConductance,
    last_timestep_temperature_array: &mut Array1<ThermodynamicTemperature>,
    thermal_conductance_array: &mut Array1<ThermalConductance>, // nodesNumber +1 elements
    volume_fraction_array: &mut Array1<f64>,
    rho_cp: &mut Array1<VolumetricHeatCapacity>,
    power_distribution_array: &mut Array1<f64>,
) 
-> Result<Array1<ThermodynamicTemperature>,ThermalHydraulicsLibError>{

    // First things first, we need to set up 
    // how the CV interacts with the internal array
    // here is heat added to CV
    

    let rate_enthalpy_change_vector: Vec<Power> = 
    inner_single_cv.rate_enthalpy_change_vector.clone();
    
    // I'm going to compute the power source for the inner most node

    let mut total_enthalpy_rate_change_innermost_node = 
    Power::new::<watt>(0.0);

    for enthalpy_chg_rate in 
        rate_enthalpy_change_vector.iter() {

            total_enthalpy_rate_change_innermost_node += *enthalpy_chg_rate;
        }

    // this innermost node will be an extra term added to the 
    // heat source vector S
    //
    // The old temperature will need to be used to calculate 
    // new specific enthalpy for the system

    let mut temperature_array: Array1<ThermodynamicTemperature> = last_timestep_temperature_array.map (
        |temp_kelvin_ptr: &ThermodynamicTemperature| {

            return *temp_kelvin_ptr;
        }

    );

    

    

    // now we start calculation

    if nodes_number > 1 {
        let mut conductance_matrix: Array2<ThermalConductance> = 
        Array::zeros((nodes_number, nodes_number));
        let mut power_source_array: Array1<Power> = Array::zeros(nodes_number);

        //
        // now, Hs is the conductance vector, but in GeN-Foam this is done 
        // on a per unit volume basis, I'm not quite going to do that 
        //
        // second note, the time scheme for this is implicit, not explicit
        // this makes this scheme quite stable, and larger fourier numbers 
        // are allowed for timestepping, good news!
        //
        // However, one must note that heat capacitance and conductance 
        // are not changing during the timestep, they are based on the 
        // old temperatures, so this doesn't quite work as well 
        // max mesh fourier number should still be 0.25
        //
        // note that Hs[0] is never used, it may as well be 0
        {
            conductance_matrix[[0,1]] = -thermal_conductance_array[1];
            conductance_matrix[[0,0]] = volume_fraction_array[0] * rho_cp[0] * total_volume 
                / dt + thermal_conductance_array[1];
            power_source_array[0] = q * power_distribution_array[0] + 
                total_enthalpy_rate_change_innermost_node +
                last_timestep_temperature_array[0] * total_volume *
                volume_fraction_array[0] * rho_cp[0] /dt;
        }
        // Bulk medium
        if nodes_number > 2 {
            for i in 1..nodes_number-1 {

                conductance_matrix[[i,i+1]] = -thermal_conductance_array[i+1];
                conductance_matrix[[i,i-1]] = -thermal_conductance_array[i];
                conductance_matrix[[i,i]] = volume_fraction_array[i] * rho_cp[i] * total_volume / 
                    dt + thermal_conductance_array[i+1] + thermal_conductance_array[i];
                power_source_array[i] = q * power_distribution_array[i] + last_timestep_temperature_array[i] * volume_fraction_array[i] * 
                    total_volume * rho_cp[i] / dt;
            }
        }

        {
            let i = nodes_number-1;
            // this represents the total conductance to the outer 
            // node
            let conductance_to_coolant_fraction: ThermalConductance = 
            thermal_conductance_array[i+1]*thermal_conductance_to_coolant/(thermal_conductance_array[i+1]+thermal_conductance_to_coolant);
            conductance_matrix[[i,i-1]] = volume_fraction_array[i] * rho_cp[i] * total_volume / 
                dt + thermal_conductance_array[i] + conductance_to_coolant_fraction;
            conductance_matrix[[i,i]] = volume_fraction_array[i] * total_volume * rho_cp[i] / dt 
                + thermal_conductance_array[i] + conductance_to_coolant_fraction;
            power_source_array[i] = q * power_distribution_array[i] 
                + last_timestep_temperature_array[i] * volume_fraction_array[i] * rho_cp[i] * total_volume / dt 
                + conductance_to_coolant_fraction * coolant_temp;       
        }

        temperature_array = solve_conductance_matrix_power_vector(conductance_matrix,power_source_array)?;


    } else {
        let conductance_to_coolant_fraction: ThermalConductance = thermal_conductance_array[1]*thermal_conductance_to_coolant/(thermal_conductance_array[1]+thermal_conductance_to_coolant);
        let cv_conductance = volume_fraction_array[0] * total_volume * rho_cp[0] / dt + conductance_to_coolant_fraction;
        let cv_power_source = q * power_distribution_array[0] 
        + last_timestep_temperature_array[0] * volume_fraction_array[0] * rho_cp[0] * total_volume / dt 
        + conductance_to_coolant_fraction * coolant_temp;

        let temperature: TemperatureInterval = cv_power_source/cv_conductance;
        temperature_array[0] = ThermodynamicTemperature::new::<kelvin>( 
            temperature.value);
    }

    // Todo: probably need to synchronise error types in future
    let inner_node_enthalpy_next_timestep: AvailableEnergy = 
    try_get_h(
        inner_single_cv.material_control_volume,
        temperature_array[0],
        inner_single_cv.pressure_control_volume)?;

    inner_single_cv.current_timestep_control_volume_specific_enthalpy 
    = inner_node_enthalpy_next_timestep;

    // I also need to update the TOld vector 
    // This will ensure that the current temperature of the single 
    // cv node is equal to that of the matrix

    *last_timestep_temperature_array = temperature_array.mapv(
        |temperature_value| {
            return temperature_value;
        }
    );

    // set liquid cv mass 
    // probably also need to update error types in future
    inner_single_cv.set_liquid_cv_mass_from_temperature()?;
    inner_single_cv.rate_enthalpy_change_vector.clear();
    inner_single_cv.max_timestep_vector.clear();


    return Ok(temperature_array);
    
}


/// This is mostly a direct translation of GeN-Foam code, 
///
/// Now, this code is meant to calculate the internal temperature 
/// profile given an innermost temperature node, as well as
/// an outer cooling boundary condition
///
/// Here is the diagram provided from lumpedNuclearStructure.H:
/// Tmax            T[0]           T[1]          T[n-1]         Tsurface      
///   | --- H[0] --- | --- H[1] --- | --- H[2] --- |  --- H[n] --- |  
///
///
/// for this code, i intend to adapt it as follows
///
/// Tmax            T[0]           T[1]          T[n-1]         T_ambient
/// [ignored]       T_innersingleCV             T_outersingleCV [ignored]
///   | --- H[0] --- | --- H[1] --- | --- H[2] --- |  --- H[n] --- |  
///        [ignored]                                    [ignore]
///
/// There should be SingleCVNode on the inside, which gives the array 
/// cv a way to interact with other single CV nodes or other 
/// HeatTransferEntity objects
///
/// you must feed in a arrays of nodesNumber length long 
/// and nodesNumber + 1 for the thermal conductance arrays
///
/// for this to work, we must have 2 nodes at least
///
/// What this means is that Hs[0] and Hs[n] both are zero
/// we don't even calculate T_ambient
#[inline]
pub (crate) fn advance_timestep_for_specified_conductance_array_cv(
    inner_single_cv: &mut SingleCVNode,
    outer_single_cv: &mut SingleCVNode,
    inner_nodes: usize, // number of nodes excluding the two CVs
    dt: Time,
    total_volume: Volume,
    q: Power,
    power_distribution_array_ref: &Array1<f64>,
    last_timetep_temperature_array_ref: &Array1<ThermodynamicTemperature>,
    conductance_array_ref: &mut Array1<ThermalConductance>, // inner_nodes + 3 elements, 
    // first and last elements of this Hs array are always set to zero
    vol_fraction: &Array1<f64>,
    rho_cp: &Array1<VolumetricHeatCapacity>,
) 
-> Result<Array1<ThermodynamicTemperature>,ThermalHydraulicsLibError>{

    // the user specifies how many inner nodes there are 
    // nodesNumber is the total number of temperature nodes, 
    // including the temperatures of the inner_single_cv 
    // and outer_single_cv
    let nodes_number: usize = inner_nodes + 2;
    

    // First things first, we need to set up 
    // how the CV interacts with the internal array
    // here is heat added to CV
    let rate_enthalpy_change_vector_inner_node: Vec<Power> = 
    inner_single_cv.rate_enthalpy_change_vector.clone();
    
    // I'm going to compute the power source for the inner most node

    let mut total_enthalpy_rate_change_innermost_node = 
    Power::new::<watt>(0.0);

    for enthalpy_chg_rate in 
        rate_enthalpy_change_vector_inner_node.iter() {

            total_enthalpy_rate_change_innermost_node += *enthalpy_chg_rate;
        }

    
    // do the same for the outermost node
    let rate_enthalpy_change_vector_outer_node: Vec<Power> = 
    outer_single_cv.rate_enthalpy_change_vector.clone();
    
    // I'm going to compute the power source for the outer most node

    let mut total_enthalpy_rate_change_outermost_node = 
    Power::new::<watt>(0.0);

    for enthalpy_chg_rate in 
        rate_enthalpy_change_vector_outer_node.iter() {

            total_enthalpy_rate_change_outermost_node += *enthalpy_chg_rate;
        }



    let temperature_array: Array1<ThermodynamicTemperature>;

    // now we start calculation

    if nodes_number > 1 {
        let mut conductance_matrix: Array2<ThermalConductance> = 
        Array::zeros((nodes_number, nodes_number));
        let mut power_source_array: Array1<Power> = Array::zeros(nodes_number);

        //
        // now, Hs is the conductance vector, but in GeN-Foam this is done 
        // on a per unit volume basis, I'm not quite going to do that 
        //
        // second note, the time scheme for this is implicit, not explicit
        // this makes this scheme quite stable, and larger fourier numbers 
        // are allowed for timestepping, good news!
        //
        // However, one must note that heat capacitance and conductance 
        // are not changing during the timestep, they are based on the 
        // old temperatures, so this doesn't quite work as well 
        // max mesh fourier number should still be 0.25
        //
        // but for stability, 1.0 will work actually
        //
        // note that Hs[0] is never used, it may as well be 0
        {
            conductance_array_ref[0] = ThermalConductance::new::<watt_per_kelvin>(0.0);
            conductance_matrix[[0,1]] = -conductance_array_ref[1];
            conductance_matrix[[0,0]] = vol_fraction[0] * rho_cp[0] * total_volume 
                / dt + conductance_array_ref[1];
            power_source_array[0] = q * power_distribution_array_ref[0] + 
                total_enthalpy_rate_change_innermost_node +
                last_timetep_temperature_array_ref[0] * total_volume *
                vol_fraction[0] * rho_cp[0] /dt;
        }
        // Bulk medium
        if nodes_number > 2 {
            for i in 1..nodes_number-1 {

                conductance_matrix[[i,i+1]] = -conductance_array_ref[i+1];
                conductance_matrix[[i,i-1]] = -conductance_array_ref[i];
                conductance_matrix[[i,i]] = vol_fraction[i] * rho_cp[i] * total_volume / 
                    dt + conductance_array_ref[i+1] + conductance_array_ref[i];
                power_source_array[i] = q * power_distribution_array_ref[i] + last_timetep_temperature_array_ref[i] * vol_fraction[i] * 
                    total_volume * rho_cp[i] / dt;
            }
        }

        {
            let i = nodes_number-1;
            // this represents the total conductance to the outer 
            // node
            //  at the ambient temperature
            //
            //  so i'm not going to edit the equations
            //  but I set HtoCool to zero 
            //  and Tcool to room temp 
            //  doesn't matter what Tcool is because 
            //  the terms become zero anyhow
            let dummy_conductance_to_coolant_fraction: ThermalConductance = 
            ThermalConductance::new::<watt_per_kelvin>(0.0);
            let dummy_coolant_temperature_at_boundary = ThermodynamicTemperature::new::<kelvin>(293.0);
            conductance_array_ref[i] = ThermalConductance::new::<watt_per_kelvin>(0.0);

            conductance_matrix[[i,i-1]] = -conductance_array_ref[i];
            conductance_matrix[[i,i]] = vol_fraction[i] * total_volume * rho_cp[i] / dt 
                + conductance_array_ref[i] + dummy_conductance_to_coolant_fraction;
            power_source_array[i] = q * power_distribution_array_ref[i] 
                + total_enthalpy_rate_change_outermost_node
                + last_timetep_temperature_array_ref[i] * vol_fraction[i] * rho_cp[i] * total_volume / dt 
                + dummy_conductance_to_coolant_fraction * dummy_coolant_temperature_at_boundary;       

        }

        temperature_array = solve_conductance_matrix_power_vector(conductance_matrix,power_source_array)?;


    } else {
        panic!("nodesNumber must be at least 2");

        // legacy code 
        //
        //let HtoCool: ThermalConductance = Hs[1]*Hcool/(Hs[1]+Hcool);
        //let M = volFraction[0] * total_volume * rhoCp[0] / dt + HtoCool;
        //let S = q * qFraction[0] 
        //+ TOld[0] * volFraction[0] * rhoCp[0] * total_volume / dt 
        //+ HtoCool * Tcool;

        //let temperature: TemperatureInterval = S/M;
        //T[0] = ThermodynamicTemperature::new::<kelvin>( 
        //    temperature.value);
    }


    return Ok(temperature_array);
    
}
