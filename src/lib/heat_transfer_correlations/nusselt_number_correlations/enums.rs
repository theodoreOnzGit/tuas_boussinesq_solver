use uom::si::{f64::*, ratio::ratio};

use crate::thermal_hydraulics_error::TuasLibError;

use super::input_structs::{NusseltPrandtlReynoldsData, WakaoData, GnielinskiData};

/// Contains a collection of nusselt number correlations for use 
///
/// still under experimentation, so quite unstable
///
/// for now, you are supposed to construct a struct containing Pr and Re 
/// and etc and fit them into the enum,
///
/// use the get method to obtain the Nusselt Number
#[derive(Debug,Clone,Copy,Default, PartialEq)]
pub enum NusseltCorrelation {

    /// pipe nusselt number using Gnielinski Correlation 
    /// for laminar, turbulent and transition region
    ///
    /// laminar flow assumes constant heat flux
    ///
    /// For this correlation, two prandtl numbers are used for Nusselt number 
    /// estimation
    /// Pr_bulk and Pr_wall 
    /// 
    /// of course, you may use your own Pr_film instead of Pr_bulk 
    /// and obtain your Nusselt number based on Pr_film, but the 
    /// correction factor will become 
    ///
    /// (Pr_film/Pr_wall)^0.11
    ///
    /// for more fine grained control, please use another enum
    ///
    PipeGnielinskiGeneric(GnielinskiData),

    /// calibrated 
    /// pipe nusselt number using Gnielinski Correlation 
    /// for laminar, turbulent and transition region. Allows you to 
    /// insert a multiplicative ratio to calibrate the Gnielinski 
    /// correlation
    ///
    /// laminar flow assumes constant heat flux
    ///
    /// For this correlation, two prandtl numbers are used for Nusselt number 
    /// estimation
    /// Pr_bulk and Pr_wall 
    /// 
    /// of course, you may use your own Pr_film instead of Pr_bulk 
    /// and obtain your Nusselt number based on Pr_film, but the 
    /// correction factor will become 
    ///
    /// (Pr_film/Pr_wall)^0.11
    ///
    /// for more fine grained control, please use another enum
    PipeGnielinskiCalibrated(GnielinskiData, Ratio),

    /// pipe nusselt number using Gnielinski Correlation 
    /// for laminar, turbulent and transition region
    ///
    /// laminar flow assumes constant heat flux
    ///
    /// For this correlation, three prandtl numbers are 
    /// used for Nusselt number estimation
    /// Pr_bulk, Pr_film and Pr_wall 
    /// 
    /// Pr_film is used for Nusselt number estimation in all regimes,
    /// but Pr_bulk and Pr_wall are used only for the correction 
    /// factor in the turbulent and transition regime 
    ///
    /// Now, in the GnielinskiData object, 
    /// only the Pr_bulk and Pr_wall are provided
    /// so Pr_film is estimated using
    ///
    /// Pr_film = (Pr_bulk + Pr_wall)/2
    PipeGnielinskiGenericPrandtlFilm(GnielinskiData),

    /// pipe nusselt number using custom Gnielinski correlation 
    /// for laminar, turbulent and transition region 
    ///
    /// laminar flow assumes constant heat flux  (Nu = 4.354)
    ///
    /// Correlation be like:
    /// Nu = C (Re^m - 280.0) Pr_film^0.4 ( 1.0 + (D_e/l)^(2/3) ) ( Pr_f / Pr_w )^0.25
    /// User must supply C and m 
    ///
    /// For low Re flows, Nu = 4.36 is used. 
    /// The transition regime is around Re = 40-100
    /// This was totally random and arbitrary assuming that low Re 
    /// results in turbulent transition so to speak, 
    /// THESE MAY NOT BE APPLICABLE IN THIS CASE, so be careful 
    ///
    /// film prandtl numbers are used in this equation where 
    /// Pr_film = (Pr_bulk + Pr_wall)/2
    CustomGnielinskiGenericPrandtlFilm(GnielinskiData, Ratio, f64),

    /// pipe nusselt number using custom Gnielinski correlation 
    /// for laminar, turbulent and transition region 
    ///
    /// laminar flow assumes constant heat flux  (Nu = 4.354)
    ///
    /// Correlation be like:
    /// Nu = C (Re^m - 280.0) Pr_f^0.4 ( 1.0 + (D_e/l)^(2/3) ) ( Pr_f / Pr_w )^0.25
    /// User must supply C and m 
    ///
    /// For low Re flows, Nu = 4.36 is used. 
    /// The transition regime is around Re = 40-100
    /// This was totally random and arbitrary assuming that low Re 
    /// results in turbulent transition so to speak, 
    /// THESE MAY NOT BE APPLICABLE IN THIS CASE, so be careful 
    ///
    CustomGnielinskiGenericPrandtlBulk(GnielinskiData, Ratio, f64),

    /// nusselt number only for turbulent
    /// flow in pipes
    /// For this correlation, two prandtl numbers are used for Nusselt number 
    /// estimation
    /// Pr_bulk and Pr_wall 
    /// 
    /// of course, you may use your own Pr_film instead of Pr_bulk 
    /// and obtain your Nusselt number based on Pr_film, but the 
    /// correction factor will become 
    ///
    /// (Pr_film/Pr_wall)^0.11
    ///
    /// for more fine grained control, please use another enum
    PipeGnielinskiTurbulentPrandtlBulk(GnielinskiData),

    /// nusselt number for porous media 
    /// especially packed beds
    /// based on Wakao Correlation
    ///
    /// note: reynolds number based on pebble diameter
    /// Wakao, N., & Funazkri, T. (1978). Effect 
    /// of fluid dispersion coefficients on particle-to-fluid mass 
    /// transfer coefficients in packed beds: correlation of 
    /// Sherwood numbers. Chemical Engineering Science, 33(10), 1375-1384.
    ///
    /// only one prandtl number is required here, so you can use 
    /// bulk fluid prandtl number or film prandtl number as you wish
    Wakao(WakaoData),

    /// generic reynolds prandtl power correlation
    /// usually in the form:
    ///
    /// Nu = a + b * Re^c * Pr^d (Pr/Pr_wall)^e
    ///
    /// a is the constant 
    /// b is the reynolds_prandtl_coefficient
    /// c is the reynolds_power,
    /// d is the prandtl_power,
    /// e is the prandtl_correction_factor_power
    ///
    ///
    /// For this correlation, two prandtl numbers are used for Nusselt number 
    /// estimation
    /// Pr and Pr_wall 
    /// 
    /// for Pr, you may use your own Pr_film instead of Pr_bulk 
    /// and obtain your Nusselt number
    /// 
    /// just beware that if you use Pr_film, the correction factor becomes
    /// (Pr_film/Pr_wall)^0.11
    /// and if you use Pr_bulk, the correction factor becomes
    /// (Pr_bulk/Pr_wall)^0.11
    ///
    /// for more fine grained control, please use another enum
    ///
    /// only one reynolds number is given, so it is up to you what 
    /// reynolds number you want to supply
    ReynoldsPrandtl(NusseltPrandtlReynoldsData),

    /// returns a nusselt number of 4.36 for fully developed 
    /// constant heat flux flow
    #[default]
    PipeConstantHeatFluxFullyDeveloped,
    /// returns a nusselt number of 3.66 for fully developed 
    /// constant temperature flow
    PipeConstantTemperatureFullyDeveloped,

    /// ciet heater correlation for version 2, 
    ///
    /// Nu = 0.04179 * reynolds^0.836 * Pr_bulk^0.333
    /// * (Pr_bulk/Pr_wall)^0.11
    ///
    /// for Pr_bulk, you may use your own Pr_film instead of Pr_bulk 
    /// and obtain your Nusselt number
    /// 
    /// just beware that if you use Pr_film, the correction factor becomes
    /// (Pr_film/Pr_wall)^0.11
    /// and if you use Pr_bulk, the correction factor becomes
    /// (Pr_bulk/Pr_wall)^0.11
    ///
    /// for more fine grained control, please use another enum
    ///
    /// or you may choose to ignore the correction factor completely
    /// as I did in my dissertation 
    ///
    /// Ong, T. K. C. (2024). Digital Twins as Testbeds for 
    /// Iterative Simulated Neutronics Feedback Controller 
    /// Development (Doctoral dissertation, UC Berkeley).
    ///
    /// only one reynolds number is given, so it is up to you what 
    /// reynolds number you want to supply
    CIETHeaterVersion2(NusseltPrandtlReynoldsData),

    /// Ideal 1e9 
    /// Just returns a Nusselt number of 10^9 
    /// which may be suitable as an approximation for heat exchangers 
    IdealNusseltOneBillion,

    /// Fixed nusselt number,
    FixedNusselt(Ratio),
}

impl NusseltCorrelation {


    /// gets the nusselt based on user choice of of correlation
    pub fn try_get(&self) -> Result<Ratio, TuasLibError> {
        let nusselt_number: Ratio = 
        match self {
            NusseltCorrelation::PipeGnielinskiGeneric(data) => {
                return data.get_nusselt_for_developing_flow_bulk_fluid_prandtl();
            },
            NusseltCorrelation::PipeGnielinskiCalibrated(data,multiplicative_factor) => {
                let uncalibrated_nusselt = 
                    data.get_nusselt_for_developing_flow_bulk_fluid_prandtl()?;

                return Ok(*multiplicative_factor * uncalibrated_nusselt);
            },
            NusseltCorrelation::CustomGnielinskiGenericPrandtlFilm(
                data, correlation_coefficient_c, reynolds_exponent_m
            ) => 
            {
                return data.get_nusselt_for_custom_developing_flow_prandtl_film
                    (*correlation_coefficient_c,*reynolds_exponent_m)
            },
            NusseltCorrelation::PipeGnielinskiTurbulentPrandtlBulk(data) => {
                return data.get_nusselt_for_developing_flow_bulk_fluid_prandtl();
            },
            NusseltCorrelation::Wakao(wakao_data) => {
                return wakao_data.get();
            },
            NusseltCorrelation::ReynoldsPrandtl(reynolds_prandtl_data) => {
                return reynolds_prandtl_data.custom_reynolds_prandtl();
            },
            NusseltCorrelation::PipeConstantHeatFluxFullyDeveloped => {
                Ratio::new::<ratio>(4.354)
            },
            NusseltCorrelation::PipeConstantTemperatureFullyDeveloped => {
                Ratio::new::<ratio>(3.66)
            },
            NusseltCorrelation::CIETHeaterVersion2(data) => {
                return data.ciet_version_2_heater_prandtl_corrected();
            },
            NusseltCorrelation::IdealNusseltOneBillion => {
                Ratio::new::<ratio>(1e9_f64)
            },
            NusseltCorrelation::FixedNusselt(value) => {
                *value
            },
            NusseltCorrelation::PipeGnielinskiGenericPrandtlFilm(data) => {
                return data.get_nusselt_for_developing_flow();
            },
            NusseltCorrelation::CustomGnielinskiGenericPrandtlBulk(
                data, correlation_coefficient_c, reynolds_exponent_m
            ) => 
            {
                return data.get_nusselt_for_custom_developing_flow_prandtl_bulk
                    (*correlation_coefficient_c,*reynolds_exponent_m)
            },

        };

        return Ok(nusselt_number);
    }

    /// gets an estimate for nusselt number based on friction factor,
    /// bulk prandtl and reynolds number
    /// important for pipe gnielinski type nusseltcorrelations
    pub fn estimate_based_on_prandtl_darcy_and_reynolds_no_wall_correction(&self,
        bulk_prandtl_number_input: Ratio,
        darcy_friction_factor: Ratio,
        reynolds_number_input: Ratio,) -> Result<Ratio, TuasLibError>{

        match self {
            NusseltCorrelation::PipeGnielinskiGeneric(data) => {

                let mut modified_data = data.clone();
                modified_data.prandtl_wall = bulk_prandtl_number_input;
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.darcy_friction_factor = darcy_friction_factor;
                modified_data.reynolds = reynolds_number_input;
                return modified_data.get_nusselt_for_developing_flow_bulk_fluid_prandtl();
            },
            NusseltCorrelation::PipeGnielinskiCalibrated(data,multiplicative_factor) => {

                let mut modified_data = data.clone();
                modified_data.prandtl_wall = bulk_prandtl_number_input;
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.darcy_friction_factor = darcy_friction_factor;
                modified_data.reynolds = reynolds_number_input;
                let uncalibrated_nusselt = 
                    modified_data.get_nusselt_for_developing_flow_bulk_fluid_prandtl()?;
                return Ok(*multiplicative_factor * uncalibrated_nusselt);
            },
            NusseltCorrelation::PipeGnielinskiTurbulentPrandtlBulk(data) => {
                let mut modified_data = data.clone();
                modified_data.prandtl_wall = bulk_prandtl_number_input;
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.darcy_friction_factor = darcy_friction_factor;
                modified_data.reynolds = reynolds_number_input;
                return modified_data.get_nusselt_for_developing_flow_bulk_fluid_prandtl();
            },
            _ => return self.estimate_based_on_prandtl_and_reynolds_no_wall_correction(
                bulk_prandtl_number_input, reynolds_number_input),

        };


    }

    /// gets an estimate for the nusselt number based on user choice 
    /// of correlation, ignores wall temperature 
    ///
    /// note that this uses clone, so it's quite resource heavy
    #[inline]
    pub fn estimate_based_on_prandtl_and_reynolds_no_wall_correction(&self,
    bulk_prandtl_number_input: Ratio,
    reynolds_number_input: Ratio,) -> Result<Ratio, TuasLibError>{

        let nusselt_number: Ratio = 
        match self {
            NusseltCorrelation::PipeGnielinskiGeneric(data) => {

                let mut modified_data = data.clone();
                modified_data.prandtl_wall = bulk_prandtl_number_input;
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.reynolds = reynolds_number_input;
                return modified_data.get_nusselt_for_developing_flow_bulk_fluid_prandtl();
            },
            NusseltCorrelation::PipeGnielinskiCalibrated(data,multiplicative_factor) => {

                let mut modified_data = data.clone();
                modified_data.prandtl_wall = bulk_prandtl_number_input;
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.reynolds = reynolds_number_input;
                let uncalibrated_nusselt = 
                    modified_data.get_nusselt_for_developing_flow_bulk_fluid_prandtl()?;
                return Ok(*multiplicative_factor * uncalibrated_nusselt);
            },
            NusseltCorrelation::CustomGnielinskiGenericPrandtlFilm(
                data, correlation_coefficient_c, reynolds_exponent_m
            ) => 
            {
                let mut modified_data = data.clone();
                modified_data.prandtl_wall = bulk_prandtl_number_input;
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.reynolds = reynolds_number_input;
                return modified_data.get_nusselt_for_custom_developing_flow_prandtl_film
                    (*correlation_coefficient_c,*reynolds_exponent_m)
            },
            NusseltCorrelation::CustomGnielinskiGenericPrandtlBulk(
                data, correlation_coefficient_c, reynolds_exponent_m
            ) => 
            {
                let mut modified_data = data.clone();
                modified_data.prandtl_wall = bulk_prandtl_number_input;
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.reynolds = reynolds_number_input;
                return modified_data.get_nusselt_for_custom_developing_flow_prandtl_bulk
                    (*correlation_coefficient_c,*reynolds_exponent_m)
            },
            NusseltCorrelation::PipeGnielinskiTurbulentPrandtlBulk(data) => {
                let mut modified_data = data.clone();
                modified_data.prandtl_wall = bulk_prandtl_number_input;
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.reynolds = reynolds_number_input;
                return modified_data.get_nusselt_for_developing_flow_bulk_fluid_prandtl();
            },
            NusseltCorrelation::Wakao(wakao_data) => {
                let mut modified_data = wakao_data.clone();
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.reynolds = reynolds_number_input;
                return modified_data.get();
            },
            NusseltCorrelation::ReynoldsPrandtl(reynolds_prandtl_data) => {
                let mut modified_data = reynolds_prandtl_data.clone();
                modified_data.prandtl_wall = bulk_prandtl_number_input;
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.reynolds = reynolds_number_input;

                return modified_data.custom_reynolds_prandtl();
            },
            NusseltCorrelation::PipeConstantHeatFluxFullyDeveloped => {
                Ratio::new::<ratio>(4.354)
            },
            NusseltCorrelation::PipeConstantTemperatureFullyDeveloped => {
                Ratio::new::<ratio>(3.66)
            },
            NusseltCorrelation::CIETHeaterVersion2(data) => {
                let mut modified_data = data.clone();
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.prandtl_wall = bulk_prandtl_number_input;
                modified_data.reynolds = reynolds_number_input;

                return modified_data.ciet_version_2_heater_prandtl_corrected();
            },
            NusseltCorrelation::IdealNusseltOneBillion => {
                Ratio::new::<ratio>(1e9_f64)
            },
            NusseltCorrelation::FixedNusselt(value) => {
                *value
            },
            NusseltCorrelation::PipeGnielinskiGenericPrandtlFilm(data) => {

                let mut modified_data = data.clone();
                modified_data.prandtl_wall = bulk_prandtl_number_input;
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.reynolds = reynolds_number_input;
                return modified_data.get_nusselt_for_developing_flow_bulk_fluid_prandtl();
            },

        };

        return Ok(nusselt_number);
    }

    /// gets an estimate for the nusselt number based on user choice 
    /// of correlation, includes wall temperature correction
    pub fn estimate_based_on_prandtl_reynolds_and_wall_correction(&self,
    bulk_prandtl_number_input: Ratio,
    wall_prandtl_number_input: Ratio,
    reynolds_number_input: Ratio,) -> Result<Ratio, TuasLibError>{

        let nusselt_number: Ratio = 
        match self {
            NusseltCorrelation::PipeGnielinskiGeneric(data) => {

                let mut modified_data = data.clone();
                modified_data.prandtl_wall = wall_prandtl_number_input;
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.reynolds = reynolds_number_input;
                return modified_data.get_nusselt_for_developing_flow_bulk_fluid_prandtl();
            },
            NusseltCorrelation::PipeGnielinskiCalibrated(data,multiplicative_factor) => {

                let mut modified_data = data.clone();
                modified_data.prandtl_wall = bulk_prandtl_number_input;
                modified_data.prandtl_bulk = wall_prandtl_number_input;
                modified_data.reynolds = reynolds_number_input;
                let uncalibrated_nusselt = 
                    modified_data.get_nusselt_for_developing_flow_bulk_fluid_prandtl()?;
                return Ok(*multiplicative_factor * uncalibrated_nusselt);
            },
            NusseltCorrelation::CustomGnielinskiGenericPrandtlFilm(
                data, correlation_coefficient_c, reynolds_exponent_m
            ) => 
            {
                let mut modified_data = data.clone();
                modified_data.prandtl_wall = wall_prandtl_number_input;
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.reynolds = reynolds_number_input;
                return modified_data.get_nusselt_for_custom_developing_flow_prandtl_film
                    (*correlation_coefficient_c,*reynolds_exponent_m)
            },
            NusseltCorrelation::CustomGnielinskiGenericPrandtlBulk(
                data, correlation_coefficient_c, reynolds_exponent_m
            ) => 
            {
                let mut modified_data = data.clone();
                modified_data.prandtl_wall = wall_prandtl_number_input;
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.reynolds = reynolds_number_input;
                return modified_data.get_nusselt_for_custom_developing_flow_prandtl_bulk
                    (*correlation_coefficient_c,*reynolds_exponent_m)
            },
            NusseltCorrelation::PipeGnielinskiTurbulentPrandtlBulk(data) => {
                let mut modified_data = data.clone();
                modified_data.prandtl_wall = wall_prandtl_number_input;
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.reynolds = reynolds_number_input;
                return modified_data.get_nusselt_for_developing_flow_bulk_fluid_prandtl();
            },
            NusseltCorrelation::Wakao(wakao_data) => {
                let mut modified_data = wakao_data.clone();
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.reynolds = reynolds_number_input;
                return modified_data.get();
            },
            NusseltCorrelation::ReynoldsPrandtl(reynolds_prandtl_data) => {
                let mut modified_data = reynolds_prandtl_data.clone();
                modified_data.prandtl_wall = wall_prandtl_number_input;
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.reynolds = reynolds_number_input;

                return modified_data.custom_reynolds_prandtl();
            },
            NusseltCorrelation::PipeConstantHeatFluxFullyDeveloped => {
                Ratio::new::<ratio>(4.354)
            },
            NusseltCorrelation::PipeConstantTemperatureFullyDeveloped => {
                Ratio::new::<ratio>(3.66)
            },
            NusseltCorrelation::CIETHeaterVersion2(data) => {
                let mut modified_data = data.clone();
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.prandtl_wall = wall_prandtl_number_input;
                modified_data.reynolds = reynolds_number_input;

                return modified_data.ciet_version_2_heater_prandtl_corrected();
            },
            NusseltCorrelation::IdealNusseltOneBillion => {
                Ratio::new::<ratio>(1e9_f64)
            },
            NusseltCorrelation::FixedNusselt(value) => {
                *value
            },
            NusseltCorrelation::PipeGnielinskiGenericPrandtlFilm(data) => {

                let mut modified_data = data.clone();
                modified_data.prandtl_wall = wall_prandtl_number_input;
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.reynolds = reynolds_number_input;
                // the get_nusselt_for_developing_flow() function 
                // takes the prandtl_film = (prandtl_wall + prandtl_bulk)/2
                return modified_data.get_nusselt_for_developing_flow();
            },
        };

        return Ok(nusselt_number);
    }

    /// gets an estimate for nusselt number based on friction factor,
    /// bulk prandtl and reynolds number
    /// important for pipe gnielinski type nusseltcorrelations
    pub fn estimate_based_on_prandtl_darcy_and_reynolds_wall_correction(&self,
        bulk_prandtl_number_input: Ratio,
        wall_prandtl_number_input: Ratio,
        darcy_friction_factor: Ratio,
        reynolds_number_input: Ratio,) -> Result<Ratio, TuasLibError>{

        match self {
            NusseltCorrelation::PipeGnielinskiGeneric(data) => {

                let mut modified_data = data.clone();
                modified_data.prandtl_wall = wall_prandtl_number_input;
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.darcy_friction_factor = darcy_friction_factor;
                modified_data.reynolds = reynolds_number_input;
                return modified_data.get_nusselt_for_developing_flow_bulk_fluid_prandtl();
            },
            NusseltCorrelation::PipeGnielinskiCalibrated(data,multiplicative_factor) => {

                let mut modified_data = data.clone();
                modified_data.prandtl_wall = wall_prandtl_number_input;
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.darcy_friction_factor = darcy_friction_factor;
                modified_data.reynolds = reynolds_number_input;
                let uncalibrated_nusselt = 
                    modified_data.get_nusselt_for_developing_flow_bulk_fluid_prandtl()?;
                return Ok(*multiplicative_factor * uncalibrated_nusselt);
            },
            NusseltCorrelation::PipeGnielinskiTurbulentPrandtlBulk(data) => {
                let mut modified_data = data.clone();
                modified_data.prandtl_wall = wall_prandtl_number_input;
                modified_data.prandtl_bulk = bulk_prandtl_number_input;
                modified_data.darcy_friction_factor = darcy_friction_factor;
                modified_data.reynolds = reynolds_number_input;
                return modified_data.get_nusselt_for_developing_flow_bulk_fluid_prandtl();
            },
            _ => return self.estimate_based_on_prandtl_reynolds_and_wall_correction(
                bulk_prandtl_number_input, 
                wall_prandtl_number_input, reynolds_number_input),

        };


    }


    /// Returns `true` if the nusselt correlation is [`PipeConstantHeatFlux`].
    ///
    /// [`PipeConstantHeatFlux`]: NusseltCorrelation::PipeConstantHeatFlux
    #[must_use]
    pub(crate) fn _is_pipe_constant_heat_flux(&self) -> bool {
        matches!(self, Self::PipeConstantHeatFluxFullyDeveloped)
    }


}

