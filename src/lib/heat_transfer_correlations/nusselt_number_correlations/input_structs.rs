use uom::{si::{f64::*, ratio::ratio}, ConstZero};

use crate::thermal_hydraulics_error::TuasLibError;

use super::pipe_correlations::*;
/// contains information Nusselt Prandtl Reynold's
/// correlation
/// usually in the form:
///
/// Nu = a + b * Re^c * Pr^d (Pr/Pr_wall)^e
///
/// a is the constant 
/// b is the reynolds_prandtl_coefficient
/// c is the reynolds_power,
/// d is the prandtl_power,
/// e is the prandtl_correction_factor_power
#[derive(Clone,Copy,Debug, PartialEq)]
pub struct NusseltPrandtlReynoldsData {

    /// reynolds number input
    pub reynolds: Ratio,

    /// bulk fluid prandtl number
    pub prandtl_bulk: Ratio,

    /// wall prandtl number based on wall tmeperature
    pub prandtl_wall: Ratio,
    /// a in 
    /// Nu = a + b * Re^c * Pr^d (Pr/Pr_wall)^e
    pub constant: Ratio,
    /// b in 
    /// Nu = a + b * Re^c * Pr^d (Pr/Pr_wall)^e
    pub reynolds_prandtl_coefficient: Ratio,
    /// c in 
    /// Nu = a + b * Re^c * Pr^d (Pr/Pr_wall)^e
    pub reynolds_power: f64,
    /// d in 
    /// Nu = a + b * Re^c * Pr^d (Pr/Pr_wall)^e
    pub prandtl_power: f64,
    /// power for prandtl number correction factor
    /// e in 
    /// Nu = a + b * Re^c * Pr^d (Pr/Pr_wall)^e
    pub prandtl_correction_factor_power: f64,
}

impl NusseltPrandtlReynoldsData {

    /// obtains nusselt based on:
    /// Nu = a + b * Re^c * Pr^d (Pr/Pr_wall)^e
    #[inline]
    pub fn custom_reynolds_prandtl(&self) 
    -> Result<Ratio,TuasLibError>{
        let reynolds: Ratio =  self.reynolds;
        let prandtl_bulk: Ratio = self.prandtl_bulk;
        let prandtl_wall: Ratio = self.prandtl_wall;
        let a: Ratio = self.constant;
        let b: Ratio = self.reynolds_prandtl_coefficient;
        let c: f64 = self.reynolds_power;
        let d: f64 = self.prandtl_power;
        let e: f64 = self.prandtl_correction_factor_power;

        let nusselt: Ratio = a + 
        b * reynolds.get::<ratio>().powf(c) 
        * prandtl_bulk.get::<ratio>().powf(d) 
        * (prandtl_bulk/prandtl_wall).get::<ratio>().powf(e);

        return Ok(nusselt);
    }

    /// obtains nusselt based on:
    /// Nu = 0.04179 * reynolds^0.836 * prandtl^0.333
    ///
    /// ignores the coefficients, a,b,c,d,e in the struct
    #[inline]
    pub fn ciet_version_2_heater_uncorrected(&self) -> 
    Result<Ratio, TuasLibError>{

        let reynolds = self.reynolds;
        let prandtl = self.prandtl_bulk;
        let reynolds_power_0_836 = reynolds.value.powf(0.836);
        let prandtl_power_0_333 = prandtl.value.powf(0.333333333333333);

        let nusselt = Ratio::new::<ratio>(
            0.04179 * reynolds_power_0_836 * prandtl_power_0_333);

        Ok(nusselt)
    }


    /// ciet heater correlation for version 2, 
    ///
    /// Nu = 0.04179 * reynolds^0.836 * Pr_bulk^0.333
    /// * (Pr_bulk/Pr_wall)^0.11
    ///
    /// ignores the coefficients, a,b,c,d,e in the struct
    ///
    /// If reynolds number is negative, doesn't matter, just 
    /// take the absolute value of reynolds number
    #[inline]
    pub fn ciet_version_2_heater_prandtl_corrected(&self) -> 
    Result<Ratio, TuasLibError>{
        let nusselt_uncorrected 
        =  {
            let ref this = self;

            let reynolds = this.reynolds.abs();
            let prandtl = this.prandtl_bulk;
            let reynolds_power_0_836 = reynolds.value.powf(0.836);
            let prandtl_power_0_333 = prandtl.value.powf(0.333333333333333);

            let nusselt_uncorrected = Ratio::new::<ratio>(
                0.04179 * reynolds_power_0_836 * prandtl_power_0_333);

            nusselt_uncorrected
        };
        // nusselt number check ok

        let prandtl_wall = self.prandtl_wall;
        let prandtl_bulk = self.prandtl_bulk;

        let prandtl_bulk_to_wall_ratio = prandtl_bulk/prandtl_wall;

        let correction_factor: f64 
        = prandtl_bulk_to_wall_ratio.get::<ratio>().powf(0.11);

        return Ok(nusselt_uncorrected*correction_factor);

    }
}

impl Default for NusseltPrandtlReynoldsData {
    fn default() -> Self {
        NusseltPrandtlReynoldsData{
            reynolds: Ratio::default(),
            prandtl_bulk: Ratio::default(),
            prandtl_wall: Ratio::default(),
            constant: Ratio::default(),
            reynolds_prandtl_coefficient: Ratio::default(),
            reynolds_power: 0.0,
            prandtl_power: 0.0,
            prandtl_correction_factor_power: 0.0,
        }
    }
}


/// Wakao, N., & Funazkri, T. (1978). Effect 
/// of fluid dispersion coefficients on particle-to-fluid mass 
/// transfer coefficients in packed beds: correlation of 
/// Sherwood numbers. Chemical Engineering Science, 33(10), 1375-1384.
#[derive(Clone,Copy,Debug, PartialEq)]
pub struct WakaoData {
    /// reynolds number based on sphere diameter 
    pub reynolds: Ratio,
    /// prandtl number of the fluid
    pub prandtl_bulk: Ratio,
}

impl WakaoData {
    /// Returns: 
    ///
    /// Nu = 2.0 + 1.1 * Re^0.333 Pr^0.6
    /// note that reynolds number 
    /// and nusselt are based on pebble or particle 
    /// diameter
    ///
    /// for bed of packed spheres
    #[inline]
    pub fn get(&self) 
    -> Result<Ratio,TuasLibError>{
        let reynolds: Ratio =  self.reynolds;
        let prandtl_bulk: Ratio = self.prandtl_bulk;
        let a: Ratio = Ratio::new::<ratio>(2.0);
        let b: Ratio = Ratio::new::<ratio>(1.1);
        let c: f64 = 0.3333333333;
        let d: f64 = 0.6;

        let nusselt: Ratio = a + 
        b * reynolds.get::<ratio>().powf(c) 
        * prandtl_bulk.get::<ratio>().powf(d);

        return Ok(nusselt);
    }
}


/// contains data for gnielinski 
/// correlation of various
#[derive(Clone,Copy,Debug, PartialEq)]
pub struct GnielinskiData {
    /// reynolds number based on hydraulic_diameter
    pub reynolds: Ratio,
    /// bulk fluid prandtl number
    pub prandtl_bulk: Ratio,
    /// wall prandtl number based on wall temperature
    pub prandtl_wall: Ratio,
    /// friction factor, set by user
    pub darcy_friction_factor: Ratio,
    /// pipe length to diameter ratio 
    pub length_to_diameter: Ratio
}

impl Default for GnielinskiData {
    fn default() -> Self {
        Self {
            reynolds: Ratio::ZERO,
            prandtl_bulk: Ratio::ZERO,
            prandtl_wall: Ratio::ZERO,
            darcy_friction_factor: Ratio::ZERO,
            length_to_diameter: Ratio::new::<ratio>(1.0),
        }
    }
}

impl GnielinskiData {


    /// Gnielinski correlation but for developing flows 
    ///
    /// suitable for laminar, turbulent and transition flows
    ///
    /// for this, only bulk prandtl number and wall prandtl 
    /// numbers are used to calculate Nusselt rather than film 
    /// prandtl number
    #[inline]
    pub fn get_nusselt_for_developing_flow_bulk_fluid_prandtl(&self) 
    -> Result<Ratio,TuasLibError>{
        let reynolds: Ratio =  self.reynolds;
        let prandtl_bulk: Ratio = self.prandtl_bulk;
        let prandtl_wall: Ratio = self.prandtl_wall;
        let darcy_friction_factor = self.darcy_friction_factor;
        let length_to_diameter = self.length_to_diameter;

        let nusselt_value = 
        gnielinski_correlation_interpolated_uniform_heat_flux_liquids_developing_bulk_fluid_prandtl(
            reynolds.get::<ratio>(),
            prandtl_bulk.get::<ratio>(),
            prandtl_wall.get::<ratio>(),
            darcy_friction_factor.get::<ratio>(),
            length_to_diameter.get::<ratio>(),
        );

        return Ok(
            Ratio::new::<ratio>(nusselt_value)
        );

    }
    /// Gnielinski correlation but for developing flows 
    ///
    /// suitable for laminar, turbulent and transition flows
    ///
    /// for this, film prandtl numbers, bulk prandtl number and wall prandtl 
    /// numbers are used to calculate Nusselt number
    /// prandtl_film: Ratio = (prandtl_wall + prandtl_bulk)/2.0;
    #[inline]
    pub fn get_nusselt_for_developing_flow(&self) 
    -> Result<Ratio,TuasLibError>{
        let reynolds: Ratio =  self.reynolds;
        let prandtl_bulk: Ratio = self.prandtl_bulk;
        let prandtl_wall: Ratio = self.prandtl_wall;
        let prandtl_film: Ratio = (prandtl_wall + prandtl_bulk)/2.0;
        let darcy_friction_factor = self.darcy_friction_factor;
        let length_to_diameter = self.length_to_diameter;

        let nusselt_value = 
        gnielinski_correlation_interpolated_uniform_heat_flux_liquids_developing(
            reynolds.get::<ratio>(),
            prandtl_bulk.get::<ratio>(),
            prandtl_film.get::<ratio>(),
            prandtl_wall.get::<ratio>(),
            darcy_friction_factor.get::<ratio>(),
            length_to_diameter.get::<ratio>(),
        );

        return Ok(
            Ratio::new::<ratio>(nusselt_value)
        );

    }


    /// Custom Gnielinski correlation but for developing flows 
    ///
    /// suitable for laminar, turbulent and transition flows
    /// the transition regime is around Re = 2300 - 4000 
    /// this is taken from the Re for transition in pipes 
    /// IT MAY NOT BE APPLICABLE IN THIS CASE
    ///
    /// for the prandtl number of the film, I just took 
    /// Pr_film = 0.5 * (prandtl_number_wall + prandtl_number_bulk_fluid)
    #[inline]
    pub fn get_nusselt_for_custom_developing_flow_prandtl_film(&self,
        correlation_coefficient_c: Ratio,
        reynolds_exponent_m: f64) 
    -> Result<Ratio,TuasLibError>{
        let reynolds: Ratio =  self.reynolds;
        let prandtl_bulk: Ratio = self.prandtl_bulk;
        let prandtl_wall: Ratio = self.prandtl_wall;
        let length_to_diameter = self.length_to_diameter;

        let prandtl_film_estimate = 0.5 * (prandtl_wall + prandtl_bulk);


        let nusselt_value = 
        custom_gnielinski_correlation_interpolated_uniform_heat_flux_liquids_developing(
            correlation_coefficient_c,
            reynolds_exponent_m,
            prandtl_film_estimate,
            prandtl_bulk,
            prandtl_wall,
            reynolds,
            length_to_diameter,
        );

        return Ok(
            Ratio::new::<ratio>(nusselt_value)
        );

    }
    /// Custom Gnielinski correlation but for developing flows 
    ///
    /// suitable for laminar, turbulent and transition flows
    /// the transition regime is around Re = 2300 - 4000 
    /// this is taken from the Re for transition in pipes 
    /// IT MAY NOT BE APPLICABLE IN THIS CASE
    ///
    /// the custom gnielinski correlation has an extra prandtl_film_estimate 
    /// argument in it based on Du's original correlation:
    /// Nu = C (Re^m - 280.0) Pr^0.4 ( 1.0 + (D_e/l)^(2/3) ) ( Pr_f / Pr_w )^0.25
    ///
    /// Du did not mention which Pr to use 
    /// I'm going to assume this is Pr_film 
    ///
    /// Nu = C (Re^m - 280.0) Pr_film^0.4 ( 1.0 + (D_e/l)^(2/3) ) ( Pr_f / Pr_w )^0.25
    /// 
    /// Now, since this unidentified prandtl number (just thought to be 
    /// prandtl film) is an extra argument, 
    /// I could assume it is the same as prandtl bulk:
    /// 
    /// Nu = C (Re^m - 280.0) Pr_f^0.4 ( 1.0 + (D_e/l)^(2/3) ) ( Pr_f / Pr_w )^0.25
    ///
    /// This is what this function does
    #[inline]
    pub fn get_nusselt_for_custom_developing_flow_prandtl_bulk(&self,
        correlation_coefficient_c: Ratio,
        reynolds_exponent_m: f64) 
    -> Result<Ratio,TuasLibError>{
        let reynolds: Ratio =  self.reynolds;
        let prandtl_bulk: Ratio = self.prandtl_bulk;
        let prandtl_wall: Ratio = self.prandtl_wall;
        let length_to_diameter = self.length_to_diameter;

        let prandtl_film_estimate = prandtl_bulk;


        let nusselt_value = 
        custom_gnielinski_correlation_interpolated_uniform_heat_flux_liquids_developing(
            correlation_coefficient_c,
            reynolds_exponent_m,
            prandtl_film_estimate,
            prandtl_bulk,
            prandtl_wall,
            reynolds,
            length_to_diameter,
        );

        return Ok(
            Ratio::new::<ratio>(nusselt_value)
        );

    }

}

