//! This module contains a library of liquid and solid 
//! thermophysical properties

use crate::thermal_hydraulics_error::ThermalHydraulicsLibError;
use uom::si::f64::*;
use uom::si::thermodynamic_temperature::degree_celsius;

/// basically,
/// insert this enum into a thermophysical property function 
/// or something
/// then it will extract the 
/// thermophysical property for you in unit safe method
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Material {
    /// Contains a list of selectable solids
    Solid(SolidMaterial),
    /// Contains a list of selectable liquids
    Liquid(LiquidMaterial)
}

impl Default for Material{
    fn default() -> Self {
        // defaults to copper 
        return Self::Solid(SolidMaterial::Copper);
    }
}

impl TryInto<SolidMaterial> for Material {
    type Error = ThermalHydraulicsLibError;

    fn try_into(self) -> Result<SolidMaterial, Self::Error> {
        match self {
            Material::Solid(material) => {
                Ok(material)
            },
            Material::Liquid(_) => {
                Err(ThermalHydraulicsLibError::TypeConversionErrorMaterial)
            },
        }
    }
}

impl TryInto<LiquidMaterial> for Material {
    type Error = ThermalHydraulicsLibError;

    fn try_into(self) -> Result<LiquidMaterial, Self::Error> {
        match self {
            Material::Solid(_) => {
                Err(ThermalHydraulicsLibError::TypeConversionErrorMaterial)
            },
            Material::Liquid(material) => {
                Ok(material)
            },
        }
    }
}

/// Contains a selection of solids with predefined material properties
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum SolidMaterial {
    /// stainless steel 304 L, 
    /// material properties from 
    /// Graves, R. S., Kollie, T. G., McElroy, D. L., 
    /// & Gilchrist, K. E. (1991). The thermal conductivity of 
    /// AISI 304L stainless steel. International journal of 
    /// thermophysics, 12, 409-415.
    SteelSS304L,
    /// Copper material
    Copper,
    /// Fiberglass material
    Fiberglass,
    /// Custom solid, for the user to decide the correlations himself 
    /// or herself
    CustomSolid(
        // lower and upper bound temperatures
        (ThermodynamicTemperature,ThermodynamicTemperature),
        // solid cp 
        fn(ThermodynamicTemperature) -> SpecificHeatCapacity,
        // thermal conductivity 
        fn(ThermodynamicTemperature) -> ThermalConductivity,
        // density 
        fn(ThermodynamicTemperature) -> MassDensity,
        // surface_roughness
        Length,
    ),
}

impl Into<Material> for SolidMaterial {
    fn into(self) -> Material {
        Material::Solid(self)
    }
}

/// Contains a selection of liquids with predefined material properties
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum LiquidMaterial {
    /// therminol VP1 
    TherminolVP1,
    /// DowthermA, using the same correlations as TherminolVP1
    DowthermA,
    /// HITEC salt, 7 wt% sodium nitrate, 40 wt% sodium nitrite, 53 wt% potassium nitrate
    HITEC,
    /// YD-325 Synthetic Heat transfer oil
    /// Qiu, Y., Li, M. J., Wang, W. Q., Du, B. C., & Wang, K. (2018). 
    /// An experimental study on the heat transfer performance of a prototype 
    /// molten-salt rod baffle heat exchanger for concentrated solar power. 
    /// Energy, 156, 63-72.
    YD325,
    /// 
    /// LiF - BeF2  in approx 67 mol% - 33 mol% combination
    /// Data taken from:
    ///
    /// Romatoski, R. R., & Hu, L. W. (2017). Fluoride salt coolant properties 
    /// for nuclear reactor applications: A review. Annals 
    /// of Nuclear Energy, 109, 635-647.
    /// properties for a custom liquid material 
    /// not covered in the database
    ///
    /// Sohal, M. S., Ebner, M. A., Sabharwall, P., & Sharpe, P. (2010). 
    /// Engineering database of liquid salt thermophysical and thermochemical 
    /// properties (No. INL/EXT-10-18297). Idaho National Lab.(INL), 
    /// Idaho Falls, ID (United States).
    FLiBe,

    /// 46.5-11.5-42.0 mol% LiF, NaF, KF respectively 
    /// eutectic composition
    /// 
    /// Data taken from:
    ///
    /// Romatoski, R. R., & Hu, L. W. (2017). Fluoride salt coolant properties 
    /// for nuclear reactor applications: A review. Annals 
    /// of Nuclear Energy, 109, 635-647.
    /// properties for a custom liquid material 
    /// not covered in the database
    ///
    /// Sohal, M. S., Ebner, M. A., Sabharwall, P., & Sharpe, P. (2010). 
    /// Engineering database of liquid salt thermophysical and thermochemical 
    /// properties (No. INL/EXT-10-18297). Idaho National Lab.(INL), 
    /// Idaho Falls, ID (United States).
    FLiNaK,


    /// Custom fluid, for the user to decide the correlations himself 
    /// or herself
    CustomLiquid(
        // lower and upper bound temperatures
        (ThermodynamicTemperature,ThermodynamicTemperature),
        // fluid cp 
        fn(ThermodynamicTemperature) -> SpecificHeatCapacity,
        // thermal conductivity 
        fn(ThermodynamicTemperature) -> ThermalConductivity,
        // viscosity 
        fn(ThermodynamicTemperature) -> DynamicViscosity,
        // density 
        fn(ThermodynamicTemperature) -> MassDensity,
    ),



}

impl Into<Material> for LiquidMaterial {
    fn into(self) -> Material {
        Material::Liquid(self)
    }
}

/// range check:
/// generic checker for whether a temperature value falls within 
/// the specified temperature range 

/// If it falls outside this range, return an error
/// or throw an error, and the program will not run
#[inline]
pub fn range_check(material: &Material,
    material_temperature: ThermodynamicTemperature,
    upper_temperature_limit: ThermodynamicTemperature,
    lower_temperature_limit: ThermodynamicTemperature) 
    -> Result<bool,ThermalHydraulicsLibError>{

    // first i convert the fluidTemp object into a degree 
    // celsius
    let temp_value_celsius = 
        material_temperature.get::<degree_celsius>();
    let low_temp_value_celsius = 
        lower_temperature_limit.get::<degree_celsius>();
    let high_temp_value_celsius = 
        upper_temperature_limit.get::<degree_celsius>();

    if temp_value_celsius < low_temp_value_celsius {
        let error_msg = "Your material temperature \n";
        let error_msg1 = "is too low :";
        let error_msg3 = "C \n";
        let error_msg4 = "\n the minimum is ".to_owned() + &low_temp_value_celsius.to_string() + "C";


        println!("{}{}{:?}{}{}",
               error_msg,
               error_msg1,
               temp_value_celsius,
               error_msg3,
               error_msg4);
        dbg!(&material);
        return Err(ThermalHydraulicsLibError::ThermophysicalPropertyTemperatureRangeError);
    }


    if temp_value_celsius > high_temp_value_celsius {
        let error_msg = "Your material temperature \n";
        let error_msg1 = "is too high :";
        let error_msg3 = "C \n";
        let error_msg4 = "\n the max is".to_owned()+ &high_temp_value_celsius.to_string()+"C";

        println!("{}{}{:?}{}{}",
               error_msg,
               error_msg1,
               temp_value_celsius,
               error_msg3,
               error_msg4);
        dbg!(&material);
        return Err(ThermalHydraulicsLibError::ThermophysicalPropertyTemperatureRangeError);
    }

    return Ok(true);

}

/// Density calculation
pub mod density;

/// Thermal conductivity calculation 
pub mod thermal_conductivity;

/// SpecificHeatCapacity calculation 
pub mod specific_heat_capacity;

/// dynamic viscosity calculation 
pub mod dynamic_viscosity;

/// specific enthalpy calculation
pub mod specific_enthalpy;

/// thermal diffusivity 
pub mod thermal_diffusivity;

/// momentum diffusivity (or kinematic viscosity)
pub mod momentum_diffusivity;

/// volumetric_heat_capacity 
pub mod volumetric_heat_capacity;

/// prandtl number 
pub mod prandtl;

/// surface roughness 
pub mod solid_material_surface_roughness;


/// functions for temperature ranges
/// this gives the max or min temperatures for each material
pub mod temperature_ranges;

/// database for liquids 
pub mod liquid_database;

/// database for solids 
pub mod solid_database;




