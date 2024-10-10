use std::f64::consts::PI;


use crate::boussinesq_thermophysical_properties::Material;
use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::control_volume_dimensions::UNIT_AREA_SQ_METER_FOR_ONE_DIMENSIONAL_CALCS;
use crate::single_control_vol::SingleCVNode;

use super::SolidColumn;
use uom::si::area::square_meter;
use uom::si::f64::*;
use ndarray::*;

impl SolidColumn {

    /// generic constructor,
    /// basically returns the default
    pub fn new() -> Self {

        return SolidColumn::default();
    }
    /// returns a solid in the shape of a block
    ///
    /// You will specify the number of inner nodes, and 
    /// the code will generate a control volume with the cylinder 
    /// sectioned into equally spaced nodes of equal length
    pub fn new_block(
        length: Length,
        thickness: Length,
        width: Length,
        initial_temperature: ThermodynamicTemperature,
        initial_pressure: Pressure,
        solid_material: SolidMaterial,
        user_specified_inner_nodes: usize,
    ) -> Self {

        let default_length = length;
        let default_temp = initial_temperature;
        let default_pressure = initial_pressure;
        let number_of_temperature_nodes = 2 + user_specified_inner_nodes;
        let node_length: Length = default_length/
        number_of_temperature_nodes as f64;

        let vol_frac_default: f64 = 1.0 / 
        number_of_temperature_nodes as f64;
        
        // temperature array 
        let mut default_temp_array: Array1<ThermodynamicTemperature> 
        = Array::default(number_of_temperature_nodes);
        default_temp_array.fill(default_temp);

        // vol frac array 

        let mut vol_frac_array: Array1<f64>
        = Array::zeros(number_of_temperature_nodes);
        vol_frac_array.fill(vol_frac_default);

        let solid_material: Material = 
        Material::Solid(
            solid_material
        );

        // cross sectional area
        
        let cross_sectional_area = width * thickness;

        let default_heat_cv_node: SingleCVNode = 
        SingleCVNode::new_block(
            node_length,
            width,
            thickness,
            solid_material,
            default_temp,
            default_pressure
        ).unwrap().try_into().unwrap();


        return Self {
            back_single_cv: default_heat_cv_node.clone(),
            front_single_cv: default_heat_cv_node,
            inner_nodes: user_specified_inner_nodes,
            total_length: default_length,
            xs_area: cross_sectional_area,
            temperature_array_current_timestep: default_temp_array,
            material_control_volume: solid_material,
            pressure_control_volume: default_pressure,
            volume_fraction_array: vol_frac_array,
            lateral_adjacent_array_temperature_vector: vec![],
            lateral_adjacent_array_conductance_vector: vec![],
            q_vector: vec![],
            q_fraction_vector: vec![],
        }

    }

    /// returns a one dimensioned volume
    ///
    /// You will specify the number of inner nodes, and 
    /// the code will generate a control volume with the cylinder 
    /// sectioned into equally spaced nodes of equal length
    pub fn new_one_dimension_volume(
        length: Length,
        initial_temperature: ThermodynamicTemperature,
        initial_pressure: Pressure,
        solid_material: SolidMaterial,
        user_specified_inner_nodes: usize,
    ) -> Self {

        let default_length = length;
        let default_temp = initial_temperature;
        let default_pressure = initial_pressure;
        let number_of_temperature_nodes = 2 + user_specified_inner_nodes;
        let node_length: Length = default_length/
        number_of_temperature_nodes as f64;

        let vol_frac_default: f64 = 1.0 / 
        number_of_temperature_nodes as f64;
        
        // temperature array 
        let mut default_temp_array: Array1<ThermodynamicTemperature> 
        = Array::default(number_of_temperature_nodes);
        default_temp_array.fill(default_temp);

        // vol frac array 

        let mut vol_frac_array: Array1<f64>
        = Array::zeros(number_of_temperature_nodes);
        vol_frac_array.fill(vol_frac_default);

        let solid_material: Material = 
        Material::Solid(
            solid_material
        );

        // cross sectional area
        
        let cross_sectional_area: Area = Area::new::<square_meter>(
            UNIT_AREA_SQ_METER_FOR_ONE_DIMENSIONAL_CALCS);

        let width: Length = cross_sectional_area.sqrt();
        let thickness: Length = width;

        let default_heat_cv_node: SingleCVNode = 
        SingleCVNode::new_block(
            node_length,
            width,
            thickness,
            solid_material,
            default_temp,
            default_pressure
        ).unwrap().try_into().unwrap();


        return Self {
            back_single_cv: default_heat_cv_node.clone(),
            front_single_cv: default_heat_cv_node,
            inner_nodes: user_specified_inner_nodes,
            total_length: default_length,
            xs_area: cross_sectional_area,
            temperature_array_current_timestep: default_temp_array,
            material_control_volume: solid_material,
            pressure_control_volume: default_pressure,
            volume_fraction_array: vol_frac_array,
            lateral_adjacent_array_temperature_vector: vec![],
            lateral_adjacent_array_conductance_vector: vec![],
            q_vector: vec![],
            q_fraction_vector: vec![],
        }

    }

    /// returns a solid in the shape of a cylinder
    ///
    /// You will specify the number of inner nodes, and 
    /// the code will generate a control volume with the cylinder 
    /// sectioned into equally spaced nodes of equal length
    pub fn new_cylinder(
        length: Length,
        diameter: Length,
        initial_temperature: ThermodynamicTemperature,
        initial_pressure: Pressure,
        solid_material: SolidMaterial,
        user_specified_inner_nodes: usize,
    ) -> Self {

        let default_length = length;
        let default_temp = initial_temperature;
        let default_pressure = initial_pressure;
        let number_of_temperature_nodes = 2 + user_specified_inner_nodes;
        let node_length: Length = default_length/
        number_of_temperature_nodes as f64;

        let vol_frac_default: f64 = 1.0 / 
        number_of_temperature_nodes as f64;
        
        // temperature array 
        let mut default_temp_array: Array1<ThermodynamicTemperature> 
        = Array::default(number_of_temperature_nodes);
        default_temp_array.fill(default_temp);

        // vol frac array 

        let mut vol_frac_array: Array1<f64>
        = Array::zeros(number_of_temperature_nodes);
        vol_frac_array.fill(vol_frac_default);

        let solid_material: Material = 
        Material::Solid(
            solid_material
        );

        // cross sectional area
        
        let cross_sectional_area = PI * diameter * diameter 
            * 0.25;

        let default_heat_cv_node: SingleCVNode = 
        SingleCVNode::new_cylinder(
            node_length,
            diameter,
            solid_material,
            default_temp,
            default_pressure
        ).unwrap().try_into().unwrap();


        return Self {
            back_single_cv: default_heat_cv_node.clone(),
            front_single_cv: default_heat_cv_node,
            inner_nodes: user_specified_inner_nodes,
            total_length: default_length,
            xs_area: cross_sectional_area,
            temperature_array_current_timestep: default_temp_array,
            material_control_volume: solid_material,
            pressure_control_volume: default_pressure,
            volume_fraction_array: vol_frac_array,
            lateral_adjacent_array_temperature_vector: vec![],
            lateral_adjacent_array_conductance_vector: vec![],
            q_vector: vec![],
            q_fraction_vector: vec![],
        }

    }



    /// returns a solid array in the shape of a cylindrical
    /// shell
    ///
    /// You will specify the number of inner nodes, and 
    /// the code will generate a control volume with the cylinder 
    /// sectioned into equally spaced nodes of equal length
    pub fn new_cylindrical_shell(
        length: Length,
        inner_diameter: Length,
        outer_diameter: Length,
        initial_temperature: ThermodynamicTemperature,
        initial_pressure: Pressure,
        solid_material: SolidMaterial,
        user_specified_inner_nodes: usize,
    ) -> Self {

        let default_length = length;
        let default_temp = initial_temperature;
        let default_pressure = initial_pressure;
        let number_of_temperature_nodes = 2 + user_specified_inner_nodes;
        let node_length: Length = default_length/
        number_of_temperature_nodes as f64;

        let vol_frac_default: f64 = 1.0 / 
        number_of_temperature_nodes as f64;
        
        // temperature array 
        let mut default_temp_array: Array1<ThermodynamicTemperature> 
        = Array::default(number_of_temperature_nodes);
        default_temp_array.fill(default_temp);

        // vol frac array 

        let mut vol_frac_array: Array1<f64>
        = Array::zeros(number_of_temperature_nodes);
        vol_frac_array.fill(vol_frac_default);

        let solid_material: Material = 
        Material::Solid(
            solid_material
        );

        // cross sectional area
        
        let cross_sectional_area = 
        PI * outer_diameter * outer_diameter * 0.25 -
        PI * inner_diameter * inner_diameter * 0.25;




        

        let default_heat_cv_node: SingleCVNode = 
        SingleCVNode::new_cylinder(
            node_length,
            inner_diameter,
            solid_material,
            default_temp,
            default_pressure
        ).unwrap().try_into().unwrap();


        return Self {
            back_single_cv: default_heat_cv_node.clone(),
            front_single_cv: default_heat_cv_node,
            inner_nodes: user_specified_inner_nodes,
            total_length: default_length,
            xs_area: cross_sectional_area,
            temperature_array_current_timestep: default_temp_array,
            material_control_volume: solid_material,
            pressure_control_volume: default_pressure,
            volume_fraction_array: vol_frac_array,
            lateral_adjacent_array_temperature_vector: vec![],
            lateral_adjacent_array_conductance_vector: vec![],
            q_vector: vec![],
            q_fraction_vector: vec![],
        }

    }

    
}
