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
use uom::si::ratio::ratio;
use uom::si::mass_rate::kilogram_per_second;
use uom::si::pressure::atmosphere;
use ndarray::*;
use std::f64::consts::PI;

impl FluidArray {

    /// generic constructor,
    /// basically returns the default
    pub fn new() -> Self {

        return FluidArray::default();
    }

    /// returns a fluid in the shape of a cylinder
    /// uses gnielinski correlation for nusselt number data
    /// and the darcy friction factor for friction factor losses
    ///
    /// You will specify the number of inner nodes, and 
    /// the code will generate a control volume with the cylinder 
    /// sectioned into equally spaced nodes of equal length
    pub fn new_cylinder(
        length: Length,
        hydraulic_diameter: Length,
        initial_temperature: ThermodynamicTemperature,
        initial_pressure: Pressure,
        adjacent_solid_material: SolidMaterial, 
        liquid_material: LiquidMaterial,
        pipe_form_loss: Ratio,
        user_specified_inner_nodes: usize,
        pipe_incline_angle: Angle,
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

        let liquid_material: Material = 
        Material::Liquid(
            liquid_material
        );
        
        let cross_sectional_area = PI * hydraulic_diameter * hydraulic_diameter 
            * 0.25;




        let surface_roughness = 
        adjacent_solid_material.surface_roughness().unwrap();

        let pipe_default_form_loss = pipe_form_loss;

        let pipe_losses: DimensionlessDarcyLossCorrelations 
        = DimensionlessDarcyLossCorrelations::new_pipe(
            default_length,
            surface_roughness,
            hydraulic_diameter,
            pipe_default_form_loss,
        );
        
        let pipe_prandtl = try_get_prandtl(
            liquid_material,
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
            node_length,
            hydraulic_diameter,
            liquid_material,
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
            material_control_volume: liquid_material,
            pressure_control_volume: default_pressure,
            volume_fraction_array: vol_frac_array,
            mass_flowrate: MassRate::new::<kilogram_per_second>(0.0),
            pressure_loss: Pressure::new::<atmosphere>(0.0),
            wetted_perimeter: 4.0 * cross_sectional_area / hydraulic_diameter,
            incline_angle: pipe_incline_angle,
            internal_pressure_source: Pressure::new::<atmosphere>(0.0),
            fluid_component_loss_properties: pipe_losses,
            nusselt_correlation: pipe_nusselt,
            lateral_adjacent_array_temperature_vector: vec![],
            lateral_adjacent_array_conductance_vector: vec![],
            q_vector: vec![],
            q_fraction_vector: vec![],
        }

    }

    /// returns a fluid array cv in the shape of a cylindrical 
    /// shell
    ///
    /// Nusselt number is by default Gnielinski Correlation 
    /// but you should be able to change it to whatever 
    ///
    /// Reynolds number is a pipe version, but it uses 
    /// the hydraulic diameter (4A/P) for computation
    ///
    pub fn new_annular_cylinder(
        length: Length,
        inner_diameter: Length,
        outer_diameter: Length,
        initial_temperature: ThermodynamicTemperature,
        initial_pressure: Pressure,
        adjacent_solid_material: SolidMaterial, 
        liquid_material: LiquidMaterial,
        pipe_form_loss: Ratio,
        user_specified_inner_nodes: usize,
        pipe_incline_angle: Angle,
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

        let liquid_material: Material = 
        Material::Liquid(
            liquid_material
        );

        // cross sectional area and hydraulic diameter
        let cross_sectional_area = 
        PI * outer_diameter * outer_diameter * 0.25 -
        PI * inner_diameter * inner_diameter * 0.25;

        // wetted perimeter calculations 
        //
        // for an annular tube, the fluid contacts the inner "tube" 
        // and outer "tube" at their respective inner and outer 
        // diameters

        let calculated_wetted_perimeter: Length 
        = PI * inner_diameter + PI * outer_diameter;

        let hydraulic_diameter: Length = 4.0 * cross_sectional_area / 
            calculated_wetted_perimeter;

        // surface_roughness
        //
        // to be frank, the inner and outer material could be different 
        // but just pick representative average material
        let surface_roughness = 
        adjacent_solid_material.surface_roughness().unwrap();

        let pipe_default_form_loss = pipe_form_loss;

        let pipe_losses: DimensionlessDarcyLossCorrelations 
        = DimensionlessDarcyLossCorrelations::new_pipe(
            default_length,
            surface_roughness,
            hydraulic_diameter,
            pipe_default_form_loss,
        );


        
        let pipe_prandtl = try_get_prandtl(
            liquid_material,
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
            node_length,
            hydraulic_diameter,
            liquid_material,
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
            material_control_volume: liquid_material,
            pressure_control_volume: default_pressure,
            volume_fraction_array: vol_frac_array,
            mass_flowrate: MassRate::new::<kilogram_per_second>(0.0),
            pressure_loss: Pressure::new::<atmosphere>(0.0),
            wetted_perimeter: calculated_wetted_perimeter,
            incline_angle: pipe_incline_angle,
            internal_pressure_source: Pressure::new::<atmosphere>(0.0),
            fluid_component_loss_properties: pipe_losses,
            nusselt_correlation: pipe_nusselt,
            lateral_adjacent_array_temperature_vector: vec![],
            lateral_adjacent_array_conductance_vector: vec![],
            q_vector: vec![],
            q_fraction_vector: vec![],
        }

    }

    ///// returns a fluid array cv meant to represent a porous column
    ///// 
    ///// Nusselt number is by default Wakao correlation
    ///// but you should be able to change it to whatever 
    /////
    ///// Reynolds number is based on Ergun's correlation 
    ///// but you can change it to whatever you like
    /////
    //pub fn new_porous_media_liquid_column(
    //    length: Length,
    //    hydraulic_diameter: Length,
    //    initial_temperature: ThermodynamicTemperature,
    //    initial_pressure: Pressure,
    //    adjacent_solid_material: SolidMaterial, 
    //    liquid_material: LiquidMaterial,
    //    pipe_form_loss: Ratio,
    //    user_specified_inner_nodes: usize,
    //    pipe_incline_angle: Angle,
    //) -> Self {

    //    todo!();

    //    let default_length = length;
    //    let default_temp = initial_temperature;
    //    let default_pressure = initial_pressure;
    //    let number_of_temperature_nodes = 2 + user_specified_inner_nodes;
    //    let node_length: Length = default_length/
    //    number_of_temperature_nodes as f64;

    //    let vol_frac_default: f64 = 1.0 / 
    //    number_of_temperature_nodes as f64;
    //    
    //    // temperature array 
    //    let mut default_temp_array: Array1<ThermodynamicTemperature> 
    //    = Array::default(number_of_temperature_nodes);
    //    default_temp_array.fill(default_temp);

    //    // vol frac array 

    //    let mut vol_frac_array: Array1<f64>
    //    = Array::zeros(number_of_temperature_nodes);
    //    vol_frac_array.fill(vol_frac_default);

    //    let liquid_material: Material = 
    //    Material::Liquid(
    //        liquid_material
    //    );
    //    
    //    let cross_sectional_area = PI * hydraulic_diameter * hydraulic_diameter 
    //        * 0.25;




    //    let surface_roughness = 
    //    adjacent_solid_material.surface_roughness().unwrap();

    //    let pipe_default_form_loss = pipe_form_loss;

    //    let pipe_losses: DimensionlessDarcyLossCorrelations 
    //    = DimensionlessDarcyLossCorrelations::new_pipe(
    //        default_length,
    //        surface_roughness,
    //        hydraulic_diameter,
    //        pipe_default_form_loss,
    //    );
    //    
    //    let pipe_prandtl = liquid_prandtl(
    //        liquid_material,
    //        default_temp,
    //        default_pressure
    //    ).unwrap();

    //    let pipe_data: GnielinskiData = 
    //    GnielinskiData {
    //        reynolds: Ratio::new::<ratio>(0.0),
    //        prandtl_bulk: pipe_prandtl,
    //        prandtl_wall: pipe_prandtl, //todo, must take into account 
    //        // wall prandtl number.. how?
    //        // 
    //        // Probably, the fluid node is going to be inserted into 
    //        // another larger struct containing multiple fluid nodes,
    //        // that struct will manage what the surrounding material is
    //        darcy_friction_factor: Ratio::new::<ratio>(0.0),
    //        length_to_diameter: default_length/hydraulic_diameter,
    //    };

    //    let pipe_nusselt: NusseltCorrelation = 
    //    NusseltCorrelation::PipeGnielinskiGeneric(
    //        pipe_data
    //    );

    //    

    //    let default_heat_cv_node: SingleCVNode = 
    //    SingleCVNode::new_cylinder(
    //        node_length,
    //        hydraulic_diameter,
    //        liquid_material,
    //        default_temp,
    //        default_pressure
    //    ).unwrap().try_into().unwrap();


    //    return Self {
    //        back_single_cv: default_heat_cv_node.clone(),
    //        front_single_cv: default_heat_cv_node,
    //        inner_nodes: user_specified_inner_nodes,
    //        total_length: default_length,
    //        xs_area: cross_sectional_area,
    //        temperature_array_current_timestep: default_temp_array,
    //        material_control_volume: liquid_material,
    //        pressure_control_volume: default_pressure,
    //        volume_fraction_array: vol_frac_array,
    //        mass_flowrate: MassRate::new::<kilogram_per_second>(0.0),
    //        pressure_loss: Pressure::new::<atmosphere>(0.0),
    //        wetted_perimeter: PI * hydraulic_diameter,
    //        incline_angle: pipe_incline_angle,
    //        internal_pressure_source: Pressure::new::<atmosphere>(0.0),
    //        pipe_loss_properties: pipe_losses,
    //        nusselt_correlation: pipe_nusselt,
    //        lateral_adjacent_array_temperature_vector: vec![],
    //        lateral_adjacent_array_conductance_vector: vec![],
    //        q_vector: vec![],
    //        q_fraction_vector: vec![],
    //    }

    //}

    /// odd shaped pipe, where one defines an arbitrary flow area 
    /// without specifying hydraulic diameter or wetted perimeter
    ///
    /// Other default nusselt numbers are the same as a 
    /// pipe, such as using Gnielinski correlation 
    ///
    /// and the moody chart (churchill correlation) 
    ///
    /// hydraulic diameter is (4/PI * xs_area).sqrt()
    /// wetted perimeter is assumed to be PI*D 
    ///
    /// if unsure, just use your own nusselt and reynolds number 
    /// to calculate fluid properties
    /// 
    pub fn new_odd_shaped_pipe(
        length: Length,
        hydraulic_diameter: Length,
        cross_sectional_area: Area,
        initial_temperature: ThermodynamicTemperature,
        initial_pressure: Pressure,
        adjacent_solid_material: SolidMaterial, 
        liquid_material: LiquidMaterial,
        pipe_form_loss: Ratio,
        user_specified_inner_nodes: usize,
        pipe_incline_angle: Angle,
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

        let liquid_material: Material = 
        Material::Liquid(
            liquid_material
        );
        

        let surface_roughness = 
        adjacent_solid_material.surface_roughness().unwrap();


        let pipe_losses: DimensionlessDarcyLossCorrelations 
        = DimensionlessDarcyLossCorrelations::new_pipe(
            default_length,
            surface_roughness,
            hydraulic_diameter,
            pipe_form_loss,
        );
        
        let pipe_prandtl = try_get_prandtl(
            liquid_material,
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
            node_length,
            hydraulic_diameter,
            liquid_material,
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
            material_control_volume: liquid_material,
            pressure_control_volume: default_pressure,
            volume_fraction_array: vol_frac_array,
            mass_flowrate: MassRate::new::<kilogram_per_second>(0.0),
            pressure_loss: Pressure::new::<atmosphere>(0.0),
            wetted_perimeter: 4.0 * cross_sectional_area / hydraulic_diameter,
            incline_angle: pipe_incline_angle,
            internal_pressure_source: Pressure::new::<atmosphere>(0.0),
            fluid_component_loss_properties: pipe_losses,
            nusselt_correlation: pipe_nusselt,
            lateral_adjacent_array_temperature_vector: vec![],
            lateral_adjacent_array_conductance_vector: vec![],
            q_vector: vec![],
            q_fraction_vector: vec![],
        }
    }

    /// creates a custom component where the user specifies 
    /// f_darcy + L/D K 
    /// for the fluid component
    ///
    /// using a
    /// Reynold's power correlation in the form 
    /// f_darcy = a + b Re^(c)
    pub fn new_custom_component(
        length: Length,
        hydraulic_diameter: Length,
        cross_sectional_area: Area,
        initial_temperature: ThermodynamicTemperature,
        initial_pressure: Pressure,
        liquid_material: LiquidMaterial,
        a: Ratio,
        b: Ratio,
        c: f64,
        user_specified_inner_nodes: usize,
        pipe_incline_angle: Angle,
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

        let liquid_material: Material = 
        Material::Liquid(
            liquid_material
        );
        
        let pipe_losses: DimensionlessDarcyLossCorrelations 
        = DimensionlessDarcyLossCorrelations::
            new_simple_reynolds_power_component(
                a,
                b,
                c
            );
        
        let pipe_prandtl = try_get_prandtl(
            liquid_material,
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
            node_length,
            hydraulic_diameter,
            liquid_material,
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
            material_control_volume: liquid_material,
            pressure_control_volume: default_pressure,
            volume_fraction_array: vol_frac_array,
            mass_flowrate: MassRate::new::<kilogram_per_second>(0.0),
            pressure_loss: Pressure::new::<atmosphere>(0.0),
            wetted_perimeter: 4.0 * cross_sectional_area / hydraulic_diameter,
            incline_angle: pipe_incline_angle,
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
