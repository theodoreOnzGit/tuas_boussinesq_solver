use crate::boussinesq_thermophysical_properties::Material;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::single_control_vol::SingleCVNode;

use super::SolidColumn;

use std::f64::consts::PI;


use uom::si::f64::*;
use uom::si::area::square_meter;
use uom::si::length::meter;
use uom::si::pressure::atmosphere;
use uom::si::thermodynamic_temperature::kelvin;
use ndarray::*;
impl Default for SolidColumn {

    fn default() -> Self {
        // just a simple 2m pipe 
        // at 300K 
        //
        // 0.01 m^2 xs_area
        //
        // A = pi * D^2/4

        let default_length = Length::new::<meter>(2.0);
        let default_temp = ThermodynamicTemperature::new::<kelvin>(300.0);
        let default_pressure = Pressure::new::<atmosphere>(1.0);

        let mut default_temp_array: Array1<ThermodynamicTemperature> 
        = Array::default(2);
        default_temp_array.fill(default_temp);

        let therminol: Material = 
        Material::Liquid(
            LiquidMaterial::TherminolVP1
        );
        
        let cross_sectional_area = Area::new::<square_meter>(0.01);

        let diameter: Length = (4.0 * cross_sectional_area / PI).sqrt();



        

        let default_heat_cv_node: SingleCVNode = 
        SingleCVNode::new_cylinder(
            0.5 * default_length,
            diameter,
            therminol,
            default_temp,
            default_pressure
        ).unwrap().try_into().unwrap();


        return Self {
            back_single_cv: default_heat_cv_node.clone(),
            front_single_cv: default_heat_cv_node,
            inner_nodes: 0,
            total_length: default_length,
            xs_area: cross_sectional_area,
            temperature_array_current_timestep: default_temp_array.clone(),
            material_control_volume: therminol,
            pressure_control_volume: Pressure::new::<atmosphere>(1.0),
            volume_fraction_array: array![0.5, 0.5],
            lateral_adjacent_array_temperature_vector: vec![],
            lateral_adjacent_array_conductance_vector: vec![],
            q_vector: vec![],
            q_fraction_vector: vec![],
        }

    }
}
