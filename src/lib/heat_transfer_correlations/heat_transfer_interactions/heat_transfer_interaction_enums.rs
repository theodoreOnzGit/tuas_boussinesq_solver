use crate::control_volume_dimensions::*;
use crate::boussinesq_thermophysical_properties::{LiquidMaterial, Material};
use uom::si::f64::*;

use super::heat_transfer_geometry::*;

use crate::heat_transfer_correlations::heat_transfer_interactions::*;
use crate::tuas_lib_error::TuasLibError;



/// Contains possible heat transfer interactions between the nodes
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum HeatTransferInteractionType {
    /// The user specifies a thermal conductance between the nodes
    /// in units of power/kelvin
    UserSpecifiedThermalConductance(ThermalConductance),

    /// 1D Cartesian Coordinates Thermal Resistance
    /// We return a ThermalConductance because it's more convenient
    ///
    /// basically have two control volumes, each node represents a control 
    /// volume
    ///
    /// // ----------------------------
    /// // |                          |
    /// // *                          *
    /// // |                          |
    /// // ----------------------------
    /// // cv_1                      cv_2
    ///
    /// between them there is a thermal resistance 
    /// based on a q'' = k dT/dx
    ///
    /// we have one material which determines conductivity 
    /// and then a length which determines the distance between 
    /// the two control volumes
    ///
    SingleCartesianThermalConductanceOneDimension(
        Material,
        XThicknessThermalConduction
    ),


    /// suppose there are two blocks with the same cross sectional 
    /// area, each of its own thickness and material makeup 
    ///
    /// this is DualCartesianThermalConductanceThreeDimension
    /// we have three dimensional blocks, but the conduction is along 
    /// the thickness of the block, tube or cylinder
    DualCartesianThermalConductanceThreeDimension(
        DataDualCartesianThermalConductanceThreeDimension
    ),



    /// 1D Cartesian Coordinates Thermal Resistance, for solids only
    /// We return a ThermalConductance because it's more convenient
    ///
    /// basically have three control volumes
    ///
    /// // -------------------------------------------------------
    /// // |                          |                          |
    /// // *                          *                          *
    /// // |                          |                          |
    /// // -------------------------------------------------------
    /// // cv_1                      cv_2                     cv_3
    ///
    /// between them there is a thermal resistance 
    /// based on a q'' = k dT/dx
    ///
    /// we have two materials which determines conductivity 
    /// and then two lengths which determines the distance between 
    /// the two control volumes
    ///
    /// Information must be passed in as a tuple,
    ///
    ///
    DualCartesianThermalConductance(
        (Material, XThicknessThermalConduction),
        (Material, XThicknessThermalConduction),
    ),

    /// 1D Cylindrical Coordinates Thermal Resistance
    /// We return a ThermalConductance because it's more convenient
    ///
    /// basically have three control volumes 
    ///
    /// // -------------------------------------------------------
    /// // |                          |                          |
    /// // *                          *                          *
    /// // |                          |                          |
    /// // -------------------------------------------------------
    /// // cv_1                      cv_2                     cv_3
    ///
    /// between them there is a thermal resistance 
    /// based on a q'' = k dT/dr
    ///
    /// we have two materials which determines conductivity 
    /// and then two lengths which determines the distance between 
    /// the two control volumes 
    ///
    /// one also needs to determine the 
    /// inner diameter, outer diameter and length of the tube 
    ///  
    /// the first material and thickness argument represents 
    /// cv_1 to cv_2 (the inner shell)
    ///
    /// and the second entry pertains to the outer shell 
    /// cv_2 to cv_3, or the outer shell
    ///
    /// 
    ///
    DualCylindricalThermalConductance(
        (Material,RadialCylindricalThicknessThermalConduction),
        (Material,RadialCylindricalThicknessThermalConduction),
        (InnerDiameterThermalConduction, 
         OuterDiameterThermalConduction, 
         CylinderLengthThermalConduction)
    ),

    /// 1D Cylindrical Coordinates Thermal Resistance
    /// We return a ThermalConductance because it's more convenient
    ///
    /// basically have three control volumes along the outer wall
    ///
    /// -------------------------------------------------------> r
    /// // ----------------------------
    /// // |                          |                          
    /// // * solid_cv_1               *                          *
    /// // |                          |                         (T_f) 
    /// // ----------------------------
    /// //                        solid_surface              Fluid_node
    ///
    /// Where r is the radius 
    /// basically the liquid is on the outside (larger r)
    ///
    /// between solid_cv_1 and the solid_surface 
    /// cv_2 there is a thermal resistance 
    /// based on a q'' = k dT/dr
    ///
    /// between solid_surface and fluid_node, there is convection resistance
    /// specified by a Nusselt Number so that we get a heat transfer 
    /// coefficient
    ///
    /// For the conduction bit,
    /// we have one material which determines conductivity 
    /// and then length which determines the distance between 
    /// the two control volumes
    ///
    /// the thermal conductance is determined by 
    /// Thermal conductance 
    /// /// (2 * pi * L * K)/
    /// ln(outer_radius/inner_radius)
    ///
    /// using obtain_thermal_conductance_annular_cylinder
    /// under common_functions
    ///
    ///
    /// For convection, the heat flux from solid surface to fluid 
    /// is:
    ///
    /// q = h A(T_s - T_f)
    /// 
    /// for hA
    /// surface area is calculated by specifying an outer diameter 
    /// and a cylindrical axial length
    ///
    ///
    ///
    CylindricalConductionConvectionLiquidOutside(
        (Material,RadialCylindricalThicknessThermalConduction,
         ThermodynamicTemperature,Pressure),
         (HeatTransfer, 
          OuterDiameterThermalConduction, 
          CylinderLengthThermalConduction),
    ),

    /// 1D Cylindrical Coordinates Thermal Resistance
    /// We return a ThermalConductance because it's more convenient
    ///
    /// basically have three control volumes along the outer wall
    ///
    /// -------------------------------------------------------> r
    /// //                           ----------------------------
    /// //                           |                          |                          
    /// // *                         *         solid_cv_1       *                  
    /// //                           |                          |                   
    /// // fluid node                ----------------------------
    /// // (T_f)                solid_surface
    ///
    /// Where r is the radius 
    /// basically the liquid is on the inside (larger smaller r)
    ///
    /// between solid_cv_1 and solid_surface 
    /// there is a thermal resistance 
    /// based on a q'' = k dT/dr
    ///
    /// between solid_surface and fluid_node, there is convection resistance
    /// specified by a Nusselt Number so that we get a heat transfer 
    /// coefficient
    ///
    /// For the conduction bit,
    /// we have one material which determines conductivity 
    /// and then length which determines the distance between 
    /// the two control volumes
    ///
    /// the thermal conductance is determined by 
    /// Thermal conductance 
    /// /// (2 * pi * L * K)/
    /// ln(outer_radius/inner_radius)
    ///
    /// using obtain_thermal_conductance_annular_cylinder
    /// under common_functions
    ///
    ///
    /// For convection, the heat flux from solid surface to fluid 
    /// is:
    ///
    /// q = h A(T_s - T_f)
    /// 
    /// for hA
    /// surface area is calculated by specifying an outer diameter 
    /// and a cylindrical axial length
    ///
    ///
    ///
    CylindricalConductionConvectionLiquidInside(
        (Material,RadialCylindricalThicknessThermalConduction,
         ThermodynamicTemperature,Pressure),
         (HeatTransfer, 
          InnerDiameterThermalConduction, 
          CylinderLengthThermalConduction),
    ),


    /// The user Specifies a heat Addition for the BC
    /// The uom type is Power
    UserSpecifiedHeatAddition,

    /// Use this enum to specify a constant heat flux
    /// you will, of course, need to provide an area
    UserSpecifiedHeatFluxCustomArea(Area),

    /// Use this enum to identify that you are 
    /// specifying a curved cylindrical surface area 
    /// on the outer surface of a cylinder
    UserSpecifiedHeatFluxCylindricalOuterArea(
        CylinderLengthThermalConduction,
        OuterDiameterThermalConduction,
    ),

    /// Use this enum to identify that you are 
    /// specifying a curved cylindrical surface area 
    /// on the inner surface of a cylinder
    UserSpecifiedHeatFluxCylindricalInnerArea(
        CylinderLengthThermalConduction,
        InnerDiameterThermalConduction,
    ),


    /// For convection between solid and fluid, 
    /// the heat flux from solid surface to fluid 
    /// is:
    ///
    /// q = h A(T_s - T_f)
    /// 
    /// this interaction calculates power based on a given h and A 
    UserSpecifiedConvectionResistance(
        DataUserSpecifiedConvectionResistance
    ),


    /// For advection one would only specify the mass flowrate 
    /// from one control volume to another
    Advection(DataAdvection),


    /// for radiation heat transfer 
    /// The basic thing is that RHT power scales with 
    /// temperature 
    ///
    /// P = coefficient * (T_hot^4 - T_cold^4)
    ///
    /// If we wanted to calculate a conductance, we would note that 
    /// P = h A (T_hot - T_cold)
    ///
    /// h A = conductance 
    /// or we can use 
    /// H = conductance 
    ///
    /// H (T_hot - T_cold) = coefficient * (T_hot^4 - T_cold^4)
    ///
    /// note that temperatures are necessarily in kelvin
    ///
    /// decompose the power 4 relation 
    /// (a^2 - b^2) = (a+b)(a-b)
    ///
    /// (T_hot^4 - T_cold^4) = 
    /// (T_hot^2 + T_cold^2) 
    /// (T_hot^2 - T_cold^2)
    ///
    /// Decomposing again:
    /// (T_hot^4 - T_cold^4) = 
    /// (T_hot^2 + T_cold^2) 
    /// (T_hot + T_cold) 
    /// (T_hot - T_cold)
    ///
    ///
    /// H (T_hot - T_cold) = 
    /// coefficient * 
    /// (T_hot^2 + T_cold^2) 
    /// (T_hot + T_cold) 
    /// (T_hot - T_cold)
    ///
    /// Therefore, conductance can be expressed as:
    ///
    ///
    /// H = coefficient * (T_hot^2 + T_cold^2)*(T_hot + T_cold) 
    ///
    /// If one wants to be more precise with units,
    /// then we should use:
    ///
    /// P = sigma * coefficient * (T_hot^4 - T_cold^4)
    ///
    /// where sigma is the stefan boltzmann constant
    /// in W m^(-2) T^(-4)
    ///
    /// H = sigma * coefficient * (T_hot^2 + T_cold^2)*(T_hot + T_cold) 
    ///
    /// the coefficient is in units of area, so provide it yourself
    ///  
    SimpleRadiation(
        Area, 
    ),
}



/// here we have a struct for simple convection resistance
/// in three dimensions
/// on
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct DataUserSpecifiedConvectionResistance{

    /// surface area for heat convection
    pub surf_area: SurfaceArea,
    /// heat transfer coefficient in watts per meter per kelvin
    pub heat_transfer_coeff: HeatTransfer,

}

/// here we have a useful for necessary advection information 

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct DataAdvection{

    /// mass flowrate
    pub mass_flowrate: MassRate,
    /// fluid density of control volume on left
    ///
    /// which means when you link control volumes or boundary 
    /// link(cv1, cv2, interaction)
    ///
    /// the picture is like this 
    ///
    /// (cv1) ----> advection ---> (cv2)
    ///
    /// cv1 is the left control volume 
    /// cv2 is the right control volume
    ///
    /// now, the cv is not always a cv, it could be any heat
    /// transfer entity
    pub fluid_density_heat_transfer_entity_1: MassDensity,
    /// fluid density of control volume on left
    ///
    /// which means when you link control volumes or boundary 
    /// link(cv1, cv2, interaction)
    ///
    /// the picture is like this 
    ///
    /// (cv1) ----> advection ---> (cv2)
    ///
    /// cv1 is the left control volume 
    /// cv2 is the right control volume
    /// now, the cv is not always a cv, it could be any heat
    /// transfer entity
    pub fluid_density_heat_transfer_entity_2: MassDensity

}

impl DataAdvection {

    /// constructs an advection interaction by specifying 
    /// a fluid material 
    /// temperature of the heat transfer entity 1
    /// and temperature of heat transfer entity 2
    ///
    ///
    /// (heat tranfer entity 1) ----mass flowrate --> (heat transfer entity 2)
    ///
    #[inline]
    pub fn new_from_temperature_and_liquid_material(
        user_input_mass_flowrate: MassRate,
        fluid_material: LiquidMaterial,
        temperature_1: ThermodynamicTemperature,
        temperature_2: ThermodynamicTemperature,
    ) -> Self {


        let density_1 = fluid_material.try_get_density(temperature_1).unwrap();
        let density_2 = fluid_material.try_get_density(temperature_2).unwrap();
        return Self {
            mass_flowrate: user_input_mass_flowrate,
            fluid_density_heat_transfer_entity_1: density_1,
            fluid_density_heat_transfer_entity_2: density_2,
        };
    }
}

impl Into<HeatTransferInteractionType> for DataAdvection {
    fn into(self) -> HeatTransferInteractionType {
        HeatTransferInteractionType::Advection(self)
    }
}

impl TryFrom<HeatTransferInteractionType> for DataAdvection {
    type Error = TuasLibError;

    fn try_from(heat_transfer_interaction: HeatTransferInteractionType) -> Result<Self, Self::Error> {

        match heat_transfer_interaction {
            HeatTransferInteractionType::Advection(data_advection) => {
                return Ok(data_advection);
            },
            _ => return Err(TuasLibError::WrongHeatTransferInteractionType),
        }

    }
}


impl HeatTransferInteractionType {

    /// based on the heat transfer interaction type,
    /// we can calculate a thermal conductance given certain parameters
    /// 
    /// this function abstracts the details away for you
    pub fn get_thermal_conductance_based_on_interaction(
        &self,
        temperature_1: ThermodynamicTemperature,
        temperature_2: ThermodynamicTemperature,
        pressure_1: Pressure,
        pressure_2: Pressure) 
        -> Result<ThermalConductance, TuasLibError> 
    {
        let interaction = *self;

        let conductance: ThermalConductance = match 
            interaction {
                HeatTransferInteractionType::UserSpecifiedThermalConductance(
                    user_specified_conductance) => 
                {
                    user_specified_conductance
                }
                ,
                HeatTransferInteractionType::SingleCartesianThermalConductanceOneDimension(
                    material,thickness) => 
                {
                    get_conductance_single_cartesian_one_dimension(
                        material,
                        temperature_1, 
                        temperature_2, 
                        pressure_1, 
                        pressure_2, 
                        thickness)?
                }
                ,
                HeatTransferInteractionType::CylindricalConductionConvectionLiquidInside(
                    (solid_material, shell_thickness,
                     solid_temperature, solid_pressure), 
                    (h, inner_diameter, cylinder_length)) => 
                {

                    let id: Length = inner_diameter.clone().into();
                    let thicnkess: Length = shell_thickness.clone().into();

                    let od: Length = id+thicnkess;

                    let outer_diameter: OuterDiameterThermalConduction = 
                        OuterDiameterThermalConduction::from(od);

                    // after all the typing conversion, we can 
                    // get our conductance
                    get_conductance_single_cylindrical_radial_solid_liquid(
                        solid_material,
                        solid_temperature,
                        solid_pressure,
                        h,
                        inner_diameter,
                        outer_diameter,
                        cylinder_length,
                        CylindricalAndSphericalSolidFluidArrangement::
                        FluidOnInnerSurfaceOfSolidShell ,
                    )?
                }
                ,

                HeatTransferInteractionType::
                    CylindricalConductionConvectionLiquidOutside(
                        (solid_material, shell_thickness,
                         solid_temperature, solid_pressure), 
                        (h, outer_diameter, cylinder_length)) => {

                        let od: Length = outer_diameter.clone().into();
                        let thicnkess: Length = shell_thickness.clone().into();

                        let id: Length = od - thicnkess;

                        let inner_diameter: InnerDiameterThermalConduction = 
                            InnerDiameterThermalConduction::from(id);

                        // after all the typing conversion, we can 
                        // get our conductance
                        get_conductance_single_cylindrical_radial_solid_liquid(
                            solid_material,
                            solid_temperature,
                            solid_pressure,
                            h,
                            inner_diameter,
                            outer_diameter,
                            cylinder_length,
                            CylindricalAndSphericalSolidFluidArrangement::
                            FluidOnOuterSurfaceOfSolidShell ,
                        )?
                    }
                ,
                // note: actually function signatures are a little more 
                // friendly to use than packing enums with lots of stuff 
                // so may change stuffing enums with tuples to stuffing 
                // enums with a single struct
                HeatTransferInteractionType::DualCylindricalThermalConductance(
                    (inner_material,inner_shell_thickness),
                    (outer_material,outer_shell_thickness),
                    (inner_diameter,
                     outer_diameter,
                     cylinder_length)
                ) => {
                    // first, want to check if inner_diameter + 
                    // shell thicknesses is outer diameter 

                    let expected_outer_diameter: Length;
                    let id: Length = inner_diameter.into();
                    let inner_thickness: Length =  inner_shell_thickness.into();
                    let outer_thickness: Length =  outer_shell_thickness.into();

                    expected_outer_diameter = 
                        id + inner_thickness + outer_thickness;

                    let od: Length = outer_diameter.into();

                    // inner diameter and outer diameter values must be 
                    // equal to within 1 nanometer 1e-9 m
                    if (od.value - expected_outer_diameter.value).abs() > 1e-9
                    {

                        let mut error_str: String = "the inner diameter 
                            plus shell thicknesses do not equate 
                            to outer diameter".to_string();

                        error_str += "supplied outer diameter (m):";
                        error_str += &od.value.to_string();
                        error_str += "expected outer diameter (m):";
                        error_str += &expected_outer_diameter.value.to_string();


                        return Err(TuasLibError::
                            GenericStringError(error_str));
                    }

                    get_conductance_cylindrical_radial_two_materials(
                        inner_material,
                        outer_material,
                        temperature_1, //convention, 1 is inner shell
                        temperature_2, // convention 2, is outer shell
                        pressure_1,
                        pressure_2,
                        inner_diameter,
                        inner_shell_thickness,
                        outer_shell_thickness,
                        cylinder_length,
                    )?
                },
                HeatTransferInteractionType::UserSpecifiedHeatAddition  
                    => {
                        return Err(TuasLibError::
                            GenericStringError("interaction type needs to be \n 
                        thermal conductance".to_string()));
                    }
                ,

                HeatTransferInteractionType::
                    UserSpecifiedHeatFluxCustomArea(_) => {

                        return Err(TuasLibError::
                            GenericStringError("interaction type needs to be \n 
                        thermal conductance".to_string()));

                    }
                ,
                HeatTransferInteractionType::
                    UserSpecifiedHeatFluxCylindricalOuterArea(_,_) => {

                        return Err(TuasLibError::
                            GenericStringError("interaction type needs to be \n 
                        thermal conductance".to_string()));

                    }
                ,
                HeatTransferInteractionType::
                    UserSpecifiedHeatFluxCylindricalInnerArea(_,_) => {

                        return Err(TuasLibError::
                            GenericStringError("interaction type needs to be \n 
                        thermal conductance".to_string()));

                    }
                ,
                HeatTransferInteractionType::
                    DualCartesianThermalConductance(
                        (material_1, thickness_1),
                        (material_2,thickness_2)) => { 

                        let conductnace_layer_1: ThermalConductance 
                            = get_conductance_single_cartesian_one_dimension(
                                material_1,
                                temperature_1, 
                                temperature_1, 
                                pressure_1, 
                                pressure_1, 
                                thickness_1)?;

                        let conductnace_layer_2: ThermalConductance 
                            = get_conductance_single_cartesian_one_dimension(
                                material_2,
                                temperature_2, 
                                temperature_2, 
                                pressure_2, 
                                pressure_2, 
                                thickness_2)?;

                        let overall_resistance = 
                            1.0/conductnace_layer_2 
                            + 1.0/conductnace_layer_1;

                        // return the conductance or resistnace inverse

                        1.0/overall_resistance
                    }
                ,
                HeatTransferInteractionType::
                    DualCartesianThermalConductanceThreeDimension(
                        data_dual_cartesian_conduction) 
                    => {

                        let material_1 = 
                            data_dual_cartesian_conduction .material_1;

                        let material_2 = 
                            data_dual_cartesian_conduction .material_2;

                        let thickness_1 = 
                            data_dual_cartesian_conduction .thickness_1;

                        let thickness_2 = 
                            data_dual_cartesian_conduction .thickness_2;

                        let xs_area = 
                            data_dual_cartesian_conduction .xs_area;

                        get_conductance_dual_cartesian_three_dimensions(
                            material_1, 
                            material_2, 
                            temperature_1, 
                            temperature_2, 
                            pressure_1, 
                            pressure_2, 
                            xs_area, 
                            thickness_1,
                            thickness_2)?
                    }
                ,

                HeatTransferInteractionType::
                    UserSpecifiedConvectionResistance(
                        data_convection_resistance) 
                    => {

                        let heat_transfer_coeff: HeatTransfer = 
                            data_convection_resistance.heat_transfer_coeff;
                        let surf_area: Area = 
                            data_convection_resistance.surf_area.into();

                        heat_transfer_coeff * surf_area
                    }
                ,

                HeatTransferInteractionType::Advection(_) => {
                    return Err(TuasLibError::GenericStringError(
                            "advection interaction types \n 
                do not correspond to conductance".to_string()));
                }
                ,

                HeatTransferInteractionType::
                    SimpleRadiation
                    (area_coeff) => 
                    {
                        simple_radiation_conductance(
                            area_coeff, 
                            temperature_1, 
                            temperature_2)
                    }
                ,
            };

        return Ok(conductance);
    }
}

