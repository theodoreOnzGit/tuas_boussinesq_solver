use uom::si::f64::*;
use crate::thermal_hydraulics_error::TuasLibError;

use super::liquid_database;
use super::liquid_database::flibe::get_flibe_thermal_conductivity;
use super::liquid_database::flinak::get_flinak_thermal_conductivity;
use super::liquid_database::hitec_nitrate_salt::get_hitec_thermal_conductivity;
use super::liquid_database::yd_325_heat_transfer_oil::get_yd325_thermal_conductivity;
use super::solid_database::copper::copper_thermal_conductivity_zou_zweibaum_spline;
use super::solid_database::custom_solid_material;
use super::solid_database::fiberglass::fiberglass_thermal_conductivity_zou_zweibaum_spline;
use super::solid_database::ss_304_l::steel_304_l_libreoffice_spline_thermal_conductivity_zweibaum;
use super::solid_database::ss_304_l::steel_304_l_spline_thermal_conductivity;
use super::LiquidMaterial;
use super::Material;
use super::SolidMaterial;
use super::SolidMaterial::*;
use super::LiquidMaterial::*;
use super::liquid_database::dowtherm_a::get_dowtherm_a_thermal_conductivity;


/// returns thermal conductivity for a given material 
///
/// ```rust 
/// use uom::si::f64::*;
/// use uom::si::thermal_conductivity::watt_per_meter_kelvin;
/// use uom::si::thermodynamic_temperature::kelvin;
/// use tuas_boussinesq_solver::boussinesq_thermophysical_properties::
/// SolidMaterial::SteelSS304L;
/// use tuas_boussinesq_solver::boussinesq_thermophysical_properties::
/// Material;
/// use tuas_boussinesq_solver::boussinesq_thermophysical_properties::
/// thermal_conductivity::try_get_kappa_thermal_conductivity;
///
/// use uom::si::pressure::atmosphere;
///
/// let steel = Material::Solid(SteelSS304L);
/// let steel_temp = ThermodynamicTemperature::new::<kelvin>(350.0);
/// let pressure = Pressure::new::<atmosphere>(1.0);
///
/// // at 350K, we should expect thermal conductivity, 
/// // 15.58 W/(m K)
///
/// let steel_thermal_cond: ThermalConductivity = 
/// try_get_kappa_thermal_conductivity(steel, steel_temp, pressure).unwrap();
///
/// // Residuals from Graves et al. was about 3% at 350K for least 
/// // squares regression. So 2.8% error is reasonable
///
/// approx::assert_relative_eq!(
///     15.58,
///     steel_thermal_cond.value,
///     max_relative=0.028);
///
/// ``` 
#[inline]
pub fn try_get_kappa_thermal_conductivity(material: Material, 
    temperature: ThermodynamicTemperature,
    _pressure: Pressure) -> Result<ThermalConductivity,TuasLibError> {

    let thermal_conductivity: ThermalConductivity = match material {
        Material::Solid(_) => solid_thermal_conductivity(material, temperature)?,
        Material::Liquid(_) => liquid_thermal_conductivity(material, temperature)?
    };

    return Ok(thermal_conductivity);
}



// should the material happen to be a solid, use this function
fn solid_thermal_conductivity(material: Material,
    temperature: ThermodynamicTemperature) -> Result<ThermalConductivity,TuasLibError>{
    
    // first match the enum

    let solid_material: SolidMaterial = match material {
        Material::Solid(SteelSS304L) => SteelSS304L,
        Material::Solid(Fiberglass) => Fiberglass,
        Material::Solid(Copper) => Copper,
        Material::Solid(CustomSolid((low_bound_temp,high_bound_temp),cp,k,rho,roughness)) => {
            CustomSolid((low_bound_temp,high_bound_temp), cp, k, rho,roughness)
        },
        Material::Liquid(_) => panic!("solid_thermal_conductivity, use SolidMaterial enums only")
    };

    let thermal_conductivity: ThermalConductivity 
        = solid_material.try_get_thermal_conductivity(temperature)?;

    return Ok(thermal_conductivity);


}

impl LiquidMaterial {
    /// returns the liquid thermal conductivity in a result enum 
    #[inline]
    pub fn try_get_thermal_conductivity(&self,
        fluid_temp: ThermodynamicTemperature,) 
        -> Result<ThermalConductivity, TuasLibError>{

        let thermal_conductivity: ThermalConductivity = match self {
            DowthermA => get_dowtherm_a_thermal_conductivity(fluid_temp)?,
            TherminolVP1 => get_dowtherm_a_thermal_conductivity(fluid_temp)?,
            HITEC => get_hitec_thermal_conductivity(fluid_temp)?,
            YD325 => get_yd325_thermal_conductivity(fluid_temp)?,
            FLiBe => get_flibe_thermal_conductivity(fluid_temp)?,
            FLiNaK => get_flinak_thermal_conductivity(fluid_temp)?,
            CustomLiquid((low_bound_temp,high_bound_temp), _cp, k_fn, _mu_fn, _rho_fn) => {
                liquid_database::custom_liquid_material
                    ::get_custom_fluid_thermal_conductivity(fluid_temp, 
                        *k_fn, 
                        *high_bound_temp, 
                        *low_bound_temp)?
            },
        };

        Ok(thermal_conductivity)
    }

}

impl SolidMaterial {
    /// returns the liquid thermal conductivity in a result enum 
    #[inline]
    pub fn try_get_thermal_conductivity(&self,
        solid_temp: ThermodynamicTemperature,) 
        -> Result<ThermalConductivity, TuasLibError>{

            let thermal_conductivity: ThermalConductivity = match self {
                Fiberglass => fiberglass_thermal_conductivity_zou_zweibaum_spline(solid_temp)?,
                SteelSS304L => {

                    let conductivity_result = 
                        steel_304_l_libreoffice_spline_thermal_conductivity_zweibaum(
                            solid_temp);

                    match conductivity_result {
                        Ok(conductivity) => {
                            return Ok(conductivity);
                        },
                        Err(TuasLibError::ThermophysicalPropertyTemperatureRangeError) => {
                            return steel_304_l_spline_thermal_conductivity(solid_temp);
                        },
                        Err(_) => {
                            todo!()
                        }
                    }


                },
                Copper => copper_thermal_conductivity_zou_zweibaum_spline(solid_temp)?,
                CustomSolid((low_bound_temp,high_bound_temp),
                    _cp,k_fn,_rho_fn,_roughness) => {
                    custom_solid_material::get_custom_solid_thermal_conductivity(
                        solid_temp, 
                        *k_fn, 
                        *high_bound_temp, 
                        *low_bound_temp)?
                },
            };

            Ok(thermal_conductivity)
        }

}

// should the material happen to be a liquid, use this function
fn liquid_thermal_conductivity(material: Material, 
    fluid_temp: ThermodynamicTemperature) -> Result<ThermalConductivity,TuasLibError> {

    let liquid_material: LiquidMaterial = match material {
        Material::Liquid(DowthermA) => DowthermA,
        Material::Liquid(TherminolVP1) => TherminolVP1,
        Material::Liquid(HITEC) => HITEC,
        Material::Liquid(YD325)=> YD325,
        Material::Liquid(FLiBe) => FLiBe,
        Material::Liquid(FLiNaK) => FLiNaK,
        Material::Liquid(CustomLiquid((low_bound_temp,high_bound_temp),cp,k,mu,rho)) => {
            CustomLiquid((low_bound_temp,high_bound_temp), cp, k, mu, rho)
        },
        Material::Solid(_) => panic!(
        "liquid_thermal_conductivity, use LiquidMaterial enums only")
    };

    liquid_material.try_get_thermal_conductivity(fluid_temp)
}






#[test]
pub fn thermal_conductivity_test_steel(){

    use super::solid_database::ss_304_l::steel_ss_304_l_ornl_thermal_conductivity;
    use uom::si::thermodynamic_temperature::kelvin;
    // we're going to test thermal conductivity for steel,
    // first at 500K for both the spline and the correlation 
    // thermal conductivity, we expect at 350K 
    // 15.58 W/(m K)

    let thermal_cond_spline = steel_304_l_spline_thermal_conductivity(
        ThermodynamicTemperature::new::<kelvin>(350.0));

    approx::assert_relative_eq!(
        15.58,
        thermal_cond_spline.unwrap().value,
        max_relative=0.001);

    // now for the Graves et al. 1991 version, from ORNL
    //

    let thermal_cond_graves_et_al_1991 = 
    steel_ss_304_l_ornl_thermal_conductivity(
        ThermodynamicTemperature::new::<kelvin>(350.0));

    // between graves and the Zou/Zweibaum version,
    // there is abut 2.8\% difference
    //
    // Residuals from Graves et al. was about 3% at 350K for least 
    // squares regression. So this is reasonable
    approx::assert_relative_eq!(
        15.58,
        thermal_cond_graves_et_al_1991.unwrap().value,
        max_relative=0.028);

    // let's try now at 1000K 
    // we expect thermal thermal_conductivity to be at 23.83

    let thermal_cond_spline = 
    steel_304_l_spline_thermal_conductivity(
        ThermodynamicTemperature::new::<kelvin>(1000.0));

    approx::assert_relative_eq!(
        23.83,
        thermal_cond_spline.unwrap().value,
        max_relative=0.028);


}
