
use crate::boussinesq_thermophysical_properties::prandtl::try_get_prandtl;
use crate::boussinesq_thermophysical_properties::SolidMaterial;
use crate::boussinesq_thermophysical_properties::Material;
use crate::boussinesq_thermophysical_properties::LiquidMaterial;
use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
use crate::heat_transfer_correlations::nusselt_number_correlations::input_structs::GnielinskiData;
use crate::single_control_vol::SingleCVNode;

use super::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
use super::FluidArray;
use uom::si::f64::*;
use uom::si::area::square_meter;
use uom::si::length::meter;
use uom::si::ratio::ratio;
use uom::si::angle::radian;
use uom::si::mass_rate::kilogram_per_second;
use uom::si::pressure::atmosphere;
use uom::si::thermodynamic_temperature::kelvin;
use ndarray::*;
use std::f64::consts::PI;

impl Default for FluidArray {
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

        let hydraulic_diameter: Length = (4.0 * cross_sectional_area / PI).sqrt();



        let surface_roughness = 
        SolidMaterial::SteelSS304L.surface_roughness().unwrap();

        let pipe_default_form_loss = Ratio::new::<ratio>(0.0);

        let pipe_losses: DimensionlessDarcyLossCorrelations 
        = DimensionlessDarcyLossCorrelations::new_pipe(
            default_length,
            surface_roughness,
            hydraulic_diameter,
            pipe_default_form_loss,
        );
        
        let pipe_prandtl = try_get_prandtl(
            therminol,
            default_temp,
            default_pressure
        ).unwrap();

        let pipe_data: GnielinskiData = 
        GnielinskiData {
            reynolds: Ratio::new::<ratio>(0.0),
            prandtl_bulk: pipe_prandtl,
            prandtl_wall: pipe_prandtl, //todo, must take into account 
            // wall prandtl number.. how?
            // 
            // Probably, the fluid node is going to be inserted into 
            // another larger struct containing multiple fluid nodes,
            // that struct will manage what the surrounding material is
            darcy_friction_factor: Ratio::new::<ratio>(0.0),
            length_to_diameter: default_length/hydraulic_diameter,
        };

        let pipe_nusselt: NusseltCorrelation = 
        NusseltCorrelation::PipeGnielinskiGeneric(
            pipe_data
        );

        let default_heat_cv_node: SingleCVNode = 
        SingleCVNode::new_cylinder(
            0.5 * default_length,
            hydraulic_diameter,
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
            mass_flowrate: MassRate::new::<kilogram_per_second>(0.0),
            pressure_loss: Pressure::new::<atmosphere>(0.0),
            wetted_perimeter: PI * hydraulic_diameter,
            incline_angle: Angle::new::<radian>(0.0),
            internal_pressure_source: Pressure::new::<atmosphere>(0.0),
            fluid_component_loss_properties: pipe_losses,
            nusselt_correlation: pipe_nusselt,
            lateral_adjacent_array_temperature_vector: vec![],
            lateral_adjacent_array_conductance_vector: vec![],
            q_vector: vec![],
            q_fraction_vector: vec![],
        }

    }
}
