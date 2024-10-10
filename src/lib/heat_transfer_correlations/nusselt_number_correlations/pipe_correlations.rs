use uom::si::f64::*;
use uom::si::ratio::ratio;


/// A nusselt correlation for CIET heater v1.0
///
/// it returns Nu = 8.0 
/// for Re < 2000.0
///
/// and returns Nu = 5.44 + 0.034*Re^(0.82)
/// for Re >= 2000.0
/// ```rust
/// extern crate approx;
/// use thermal_hydraulics_rs::tuas_boussinesq_solver::heat_transfer_correlations::
/// nusselt_number_correlations::pipe_correlations;
/// 
///
/// // for Re < 2000, return 8
/// let Re_laminar = 1500.0;
///
/// let Nu_laminar_test = pipe_correlations::nusselt_ciet_heater_v1_0(Re_laminar);
///
/// approx::assert_relative_eq!(8.0, Nu_laminar_test, max_relative=0.001);
///
/// // the following two tests are taken from table 3-1 of:
/// // <http://fhr.nuc.berkeley.edu/wp-content/uploads/2015/04/14-009_CIET-IRP-Final-Report.pdf>
/// // this is page 33 out of 103 for the document
///
/// // this test is accurate to within 1% of stated value
///
/// let Re_turbulent = 2768_f64;
/// let Nu_turbulent_test = pipe_correlations::
/// nusselt_ciet_heater_v1_0(Re_turbulent);
///
/// approx::assert_relative_eq!(28.0, Nu_turbulent_test, max_relative=0.01);
///
/// // this test is accurate to within 3% of stated value
///
/// let Re_turbulent_2 = 3932_f64;
/// let Nu_turbulent_test_2 = pipe_correlations::
/// nusselt_ciet_heater_v1_0(Re_turbulent_2);
///
/// approx::assert_relative_eq!(36.0, Nu_turbulent_test_2, max_relative=0.03);
/// 
///
///
/// ```
///
/// Note that there is a discontinuity at Re = 2000
/// and that this is test bay data...
/// When heater was installed in CIET, there were different results
///
pub fn nusselt_ciet_heater_v1_0(reynolds_number: f64)-> f64 {

    if reynolds_number >= 2000_f64 {
        return 5.44 + 0.034*reynolds_number.powf(0.82);
    }

    return 8.0;

}


/// Dittus Boelter Correlation
///
/// <https://www.e3s-conferences.org/articles/e3sconf/pdf/2017/01/e3sconf_wtiue2017_02008.pdf>
///
///
/// Meant for turbulent flow
/// Smooth surface tubes
/// Heiss, J. F., & Coull, J. (1951). Nomograph of Dittus-Boelter 
/// equation for heating and cooling 
/// liquids. Industrial & Engineering Chemistry, 43(5), 1226-1229.
///
///
/// <http://herve.lemonnier.sci.free.fr/TPF/NE/Winterton.pdf>
///
/// The original paper is here
///
/// Dittus, F. W., & Boelter, L. M. K. (1985). Heat transfer in 
/// automobile radiators of the tubular type. International 
/// communications in heat and mass transfer, 12(1), 3-22.
///
/// The Dittus Boelter correlation has two forms,
/// one for heating and one for cooling
///
/// By heating I mean that the fluid is heated
/// and heat is transfered from the tube walls to the 
/// heater
///
/// And by cooling I mean that the fluid is cooled
/// and the wall takes heat from the fluid
///
/// ```rust
/// extern crate approx;
/// use thermal_hydraulics_rs::tuas_boussinesq_solver::heat_transfer_correlations::
/// nusselt_number_correlations::pipe_correlations;
///
/// // here we have an example for heating
/// // Re = 10000, Pr = 17
///
///
/// let Re = 10000_f64;
/// let Pr = 17_f64;
///
/// let heating_ref_nu = 0.023 * Re.powf(0.8) * Pr.powf(0.4);
///
/// let heating_test_bool = true;
///
/// let mut test_Nu = pipe_correlations::dittus_boelter_correlation(Re, Pr,
/// heating_test_bool);
///
/// approx::assert_relative_eq!(heating_ref_nu, test_Nu, 
/// max_relative=0.01);
///
/// // here we have an example for cooling
/// // Re = 10000, Pr = 17
///
/// let cooling_ref_nu = 0.023 * Re.powf(0.8) * Pr.powf(0.3);
///
/// let cooling_test_bool = false;
///
/// test_Nu = pipe_correlations::dittus_boelter_correlation(Re, Pr,
/// cooling_test_bool);
///
/// approx::assert_relative_eq!(cooling_ref_nu, test_Nu, 
/// max_relative=0.01);
/// ```
///
///<https://www.nuclear-power.com/nuclear-engineering/heat-transfer/convection-convective-heat-transfer/sieder-tate-equation/>
///
/// Unfortunately, Dittus Boelter correlation is valid
/// only for small to moderate temperature differences
///
/// For larger temperature differences, use Sieder-Tate
/// 
/// 
///
pub fn dittus_boelter_correlation(reynolds_number: f64, prandtl_number: f64,
                                  heating: bool) -> f64 {

    if heating == true {
        let nusselt_number = 0.023 * reynolds_number.powf(0.8) * prandtl_number.powf(0.4);
        return nusselt_number;
    }
    else {
        let nusselt_number = 0.023 * reynolds_number.powf(0.8) * prandtl_number.powf(0.3);
        return nusselt_number;
    }

}

/// Sieder Tate Relationship
///
/// <https://www.e3s-conferences.org/articles/e3sconf/pdf/2017/01/e3sconf_wtiue2017_02008.pdf>
///
/// <https://www.nuclear-power.com/nuclear-engineering/heat-transfer/convection-convective-heat-transfer/sieder-tate-equation/>
///
/// Note that properties here are evaluated at Tavg (ie average bulk fluid
/// temperature)
///
/// For pipe or heat exchanger,
/// it could be 
///
/// Tavg = (T_outlet + T_inlet)/2
///
/// the Re, Pr is generally evaluated at fluid temperature
/// whereas the fluid viscosity ratio is the ratio of viscosity at
/// the bulk fluid temperature to 
/// fluid viscosity at wall temperature
///
/// Yang, X., Yang, X., Ding, J., Shao, Y., & Fan, H. (2012). 
/// Numerical simulation study on the heat transfer 
/// characteristics of the tube receiver of the 
/// solar thermal power tower. Applied Energy, 90(1), 142-147.
///
/// viscosity_ratio = mu_f / mu_s
///
/// note that this ratio is a dynamic viscosity ratio, not 
/// kinematic viscosity ratio
///
///
/// The range of applicability (from Perry's Handbook)
/// is 
/// 0.7 < Pr < 16700
/// and 
/// 4000 < Re_D <10000
///
/// and 
///
/// 0.0044 < viscosity_ratio <  9.75
///
/// The viscosity ratio bounds are estimated from the 
/// the seider tate laminar heat transfer correlation,
/// i assumed they are of the same bounds. Did not check
/// however.
/// 
/// This is for fully developed turbulent flow only
///
/// viscosity_ratio = 5.0;
///
///
/// ```rust
///
/// extern crate approx;
/// use thermal_hydraulics_rs::tuas_boussinesq_solver::heat_transfer_correlations::
/// nusselt_number_correlations::pipe_correlations;
///
/// let Re = 8000_f64;
/// let Pr = 17_f64;
///
/// // the viscosity ratio is assumed to be 5
///
/// let viscosity_ratio = 5.0_f64;
///
/// let nu_f_reference = 0.027 * Re.powf(0.8) 
/// * Pr.powf(1.0/3.0) * 
/// viscosity_ratio.powf(0.14);
///
/// let test_nu = pipe_correlations::sieder_tate_correlation(
/// Re, Pr, viscosity_ratio);
///
/// approx::assert_relative_eq!(nu_f_reference, test_nu, 
/// max_relative=0.01);
///
/// ```
///
///
///
/// meant for turbulent flow
pub fn sieder_tate_correlation(reynolds_number: f64, prandtl_number: f64, 
                               viscosity_ratio_fluid_over_wall: f64) -> f64 {

    if prandtl_number < 0.7 {
        panic!("Sieder Tate Pr < 0.7, too low");
    }

    if prandtl_number > 16700_f64 {
        panic!("Sieder Tate Pr > 16700, too high");
    }

    if reynolds_number < 4000_f64 {
        panic!("Sieder Tate Re < 4000, laminar or transition");
    }

    if reynolds_number > 10000_f64 {
        panic!("Sieder Tate Re > 10000, too high");
    }

    if viscosity_ratio_fluid_over_wall < 0.0044 {
        panic!("Sieder Tate viscosity_ratio_fluid_over_wall < 4000, 
               laminar or transition");
    }

    if viscosity_ratio_fluid_over_wall > 9.75 {
        panic!("Sieder Tate viscosity_ratio_fluid_over_wall > 
               10000, too high");
    }

    let nusselt_number_f = 0.027 * reynolds_number.powf(0.8) * prandtl_number.powf(0.33333333333) * 
        viscosity_ratio_fluid_over_wall.powf(0.14);

    return nusselt_number_f;
}

/// Gnielinski Equation for liquids
///
///
/// <https://www.e3s-conferences.org/articles/e3sconf/pdf/2017/01/e3sconf_wtiue2017_02008.pdf>
///
/// turbulent flow, all kinds of tubes
///
/// However, flow should be fully developed
///
/// ```rust
///
/// extern crate approx;
/// use thermal_hydraulics_rs::tuas_boussinesq_solver::heat_transfer_correlations::
/// nusselt_number_correlations::pipe_correlations;
///
/// let Re = 8000_f64;
/// let Pr_fluid = 17_f64;
/// let Pr_wall = 12_f64;
/// let darcy_friction_factor = 0.005_f64;
///
/// // let's now calculate the nusslet number
///
/// let prandtl_ratio = Pr_fluid/Pr_wall;
///
/// let darcy_ratio: f64 = darcy_friction_factor/8.0;
///
/// let numerator: f64 = darcy_ratio * (Re - 1000_f64) * Pr_fluid *
///     prandtl_ratio.powf(0.11);
/// let denominator:f64 = 1_f64 + 12.7_f64 * darcy_ratio.powf(0.5) *
///     (Pr_fluid.powf(2.0/3.0) - 1.0);
/// 
///
///
/// let nu_f_reference = numerator/denominator;
///
/// let test_nu = pipe_correlations::gnielinski_correlation_liquids_fully_developed(
/// Re,Pr_fluid, Pr_wall,darcy_friction_factor);
/// ///
/// approx::assert_relative_eq!(nu_f_reference, test_nu, 
/// max_relative=0.01);
///
/// ```
///
pub fn gnielinski_correlation_liquids_fully_developed(reynolds_number: f64, 
    prandtl_number_bulk_fluid: f64,
    prandtl_number_wall: f64,
    darcy_friction_factor: f64) -> f64 {

    if prandtl_number_bulk_fluid < 0.5 {
        panic!("gnielinski Pr_fluid < 0.5, too low");
    }

    if prandtl_number_bulk_fluid > 1e5_f64 {
        panic!("gnielinski Pr_fluid > 1e5, too high");
    }

    if prandtl_number_wall < 0.5 {
        panic!("gnielinski Pr_wall < 0.5, too low");
    }

    if prandtl_number_wall > 1e5_f64 {
        panic!("gnielinski Pr_wall > 1e5, too high");
    }

    let prandtl_ratio: f64 = prandtl_number_bulk_fluid/prandtl_number_wall;

    if prandtl_ratio < 0.05 {
        panic!("gnielinski prandtl_ratio < 0.05, too low");
    }

    if prandtl_ratio > 20_f64 {
        panic!("gnielinski prandtl_ratio > 20, too high");
    }

    if reynolds_number < 2300_f64 {
        panic!("gnielinski Re < 2300, laminar or transition");
    }

    if reynolds_number > 1e6_f64 {
        panic!("gnielinski Re > 1e6, too high");
    }

    // now we start calculating
    let darcy_ratio: f64 = darcy_friction_factor/8.0;

    let numerator: f64 = darcy_ratio * (reynolds_number - 1000_f64) * prandtl_number_bulk_fluid *
        prandtl_ratio.powf(0.11);
    let denominator:f64 = 1_f64 + 12.7_f64 * darcy_ratio.powf(0.5) *
        (prandtl_number_bulk_fluid.powf(0.666667) - 1.0);

    let fluid_nusselt_number = numerator/denominator;
    

    return fluid_nusselt_number;
}



/// returns a nusselt number of 4.36,
///
/// This is an estimate for constant heat flux nusselt number
/// for fully developed thermal and velocity boundary layers
/// 
///
/// ```rust
/// extern crate approx;
/// use thermal_hydraulics_rs::tuas_boussinesq_solver::heat_transfer_correlations::
/// nusselt_number_correlations::pipe_correlations;
///
/// let nu_reference = 4.36_f64;
/// let Re = 1800_f64;
/// let nu_test = pipe_correlations::laminar_nusselt_uniform_heat_flux_fully_developed(
/// Re);
///
///
/// approx::assert_relative_eq!(nu_reference, nu_test, 
/// max_relative=0.01);
///
///
///
/// ```
pub fn laminar_nusselt_uniform_heat_flux_fully_developed(
    reynolds_number: f64) -> f64 {
    if reynolds_number > 2300_f64 {
        panic!("turbulent Re > 2300");
    }

    return 4.36;
}

/// returns a nusselt number of 3.66,
///
/// This is an estimate for constant wall temperature nusselt number
/// for fully developed thermal and velocity boundary layers
/// 
/// Re is measured at bulk temp
/// T_bulk = (T_in + T_out)/2
///
/// ```rust
/// extern crate approx;
/// use thermal_hydraulics_rs::tuas_boussinesq_solver::heat_transfer_correlations::
/// nusselt_number_correlations::pipe_correlations;
///
/// let nu_reference = 3.66_f64;
/// let Re = 1800_f64;
/// let nu_test = pipe_correlations::laminar_nusselt_uniform_wall_temperature_fully_developed(
/// Re);
///
///
/// approx::assert_relative_eq!(nu_reference, nu_test, 
/// max_relative=0.01);
///
///
///
/// ```
pub fn laminar_nusselt_uniform_wall_temperature_fully_developed(
    reynolds_number: f64) -> f64 {
    if reynolds_number > 2300_f64 {
        panic!("turbulent Re > 2300");
    }

    return 3.66;
}

/// estimates Nusselt Number for developing flow 
/// in laminar regime
/// for tubes
/// constant wall temperature
///
/// Re, Pr is measured at bulk temp
/// T_bulk = (T_in + T_out)/2
///
/// for fully developed flow, we need L/D to be about 20 or more
///
/// in Gnielinsiki's paper, when we have Pr = 0.7, we then have
/// and L/D about 1000, or D/L  = 0.001, then we can have
/// Nusselt number almost 3.66
///
/// For higher Prandtl numbers, the tendancy is for Nusselt numbers
/// to increase more especially due to influence of developing flow.
///
/// The second test case is for Pr about 70, which is the other extreme
/// case. From Gnielinski's paper, Nusselt number is about
/// 8.0 for Re = 2000 and d/L = 0.001
///
///
///
/// ```rust
///
/// extern crate approx;
/// use thermal_hydraulics_rs::tuas_boussinesq_solver::heat_transfer_correlations::
/// nusselt_number_correlations::pipe_correlations;
///
/// let mut nu_reference = 3.66_f64;
/// let mut Re = 1000_f64;
/// let mut Pr = 0.7_f64;
/// let lengthToDiameterRatio = 1000_f64;
///
/// let mut nu_test = pipe_correlations::laminar_nusselt_uniform_wall_temperature_developing(
/// Re,
/// Pr,
/// lengthToDiameterRatio);
///
///
///
/// approx::assert_relative_eq!(nu_reference, nu_test, 
/// max_relative=0.01);
/// 
/// // this is the second part of the test
///
/// nu_reference = 8_f64;
/// Re = 2000_f64;
/// Pr = 70_f64;
///
/// nu_test = pipe_correlations::laminar_nusselt_uniform_wall_temperature_developing(
/// Re,
/// Pr,
/// lengthToDiameterRatio);
///
///
/// approx::assert_relative_eq!(nu_reference, nu_test, 
/// max_relative=0.05);
///
/// ```
///
///
///
///
pub fn laminar_nusselt_uniform_wall_temperature_developing(
    reynolds_number: f64, prandtl_number: f64, length_to_diameter_ratio: f64) -> f64 {


    if reynolds_number > 2300_f64 {
        panic!("
               laminar_nusselt_uniform_wall_temperature_developing 
               error 
               turbulent Re > 2300");
    }

    if reynolds_number == 0_f64 {
        // if Re = 0, no flow, 
        // we should have Nu = 1, which is as good as conduction
        return 1.0;
    }

    if reynolds_number < 0_f64 {
        panic!("laminar_nusselt_uniform_wall_temperature_developing 
               error Re < 0");
    }

    if prandtl_number < 0_f64 {
        panic!("laminar_nusselt_uniform_wall_temperature_developing 
               error Pr < 0");
    }

    if length_to_diameter_ratio <= 0_f64 {
        panic!("laminar_nusselt_uniform_wall_temperature_developing 
               error lengthToDiameterRatio < 0");
    }

    let diameter_to_length_ratio = length_to_diameter_ratio.powf(-1.0);

    let term_1 :f64 = 3.66;
    let term_2 :f64 = 1.615 * (reynolds_number * prandtl_number * diameter_to_length_ratio).
        powf(0.3333333333) - 0.7;

    let term_3 :f64 = (reynolds_number * prandtl_number * diameter_to_length_ratio).powf(0.5) *
        (2.0/(1.0 + 22.0 * prandtl_number)).powf(0.166666666666666667);


    let nusselt_number = (term_1.powf(3.0) +
                          0.7_f64.powf(3.0) +
                          term_2.powf(3.0) +
                          term_3.powf(3.0)).powf(0.33333333333333);


    return nusselt_number;
}


/// estimates Nusselt Number for developing flow 
/// in laminar regime
/// for tubes
/// constant heat flux
///
/// Re, Pr is measured at bulk temp
/// T_bulk = (T_in + T_out)/2
///
/// note that wall temperatures are not required in this case
///
/// for fully developed flow, we need L/D to be about 20 or more
///
/// in Gnielinsiki's paper, when we have Pr = 0.7, we then have
/// and L/D about 1000, or D/L  = 0.001, then we can have
/// Nusselt number almost 3.66 for uniform wall temperature
///
/// Nu = 3.66 is the Nusselt number for uniform wall temperature
/// fully developed flow
///
/// Hence, for constant heat flux, to get a value of 4.36 which
/// is the value for fully developed flow, we need about Pr = 0.7
/// D/L = 0.001, or L/D = 1000, and Re about 1000-2300
///
///
/// There are some pieces of data available from Gnielinski's correlation
/// for Pr = 0.7
/// L/D = 10000,
///
/// The flow seems to also be fully developed here.
///
/// the value seems to be 4.36 no matter the Reynold's number
///
/// Now in CIET from Zweibaum's PhD thesis
/// , the parasitic heat losses in steady state heat
/// transfer were underestimated by about 75% in the priamry loop
/// and about 50% in the DRACS loop when using normal correlations
/// even for experiments thought to be steady state,
///
/// One possible contribution to this error is where Nusselt
/// numbers are underestimated in the laminar regime due to
/// flow development. Of course, there could be heat losses
/// due to instruments, connected heat structures and etc,
/// but a lower convective thermal resistance at the pipe wall 
/// would increase heat transfer anyhow.
///
/// In an oversimplistic test, I take the fully developed flow
/// constant heat flux nusselt numer of 4.36, multiply that
/// by 1.75 to account for 75% underestimation, and compare that to a 
/// typical nusselt number generated by this correlation 
/// in the laminar regime.
///
/// A typical pipe in the CTAH loop has the following parameters:
///
/// long L/D ratio is about 87
/// the typical Pr at dowtherm A temp about 80C is 
/// 17 or 18 and Re =  200 therabout.
///
/// In test 4, we see that the 1.75 correction factor
/// applied to Nu = 4.36 is within 8% of the value generated by
/// this correlation when CIET parameters are used.
///
/// This looks promising of course. But we have not taken into
/// account conductive thermal resistance and insulation.
///
/// Nevertheless, it is promising to look into this as a potential
/// source of error.
///
/// ```rust
///
/// extern crate approx;
/// use thermal_hydraulics_rs::tuas_boussinesq_solver::heat_transfer_correlations::
/// nusselt_number_correlations::pipe_correlations;
///
/// let mut nu_reference = 4.36_f64;
///
///
/// // test 1
///
/// let mut Re = 1000_f64;
/// let mut Pr = 0.7_f64;
/// let mut lengthToDiameterRatio = 1000_f64;
///
/// let mut nu_test = pipe_correlations::laminar_nusselt_uniform_heat_flux_developing(
/// Re,
/// Pr,
/// lengthToDiameterRatio);
///
///
///
/// approx::assert_relative_eq!(nu_reference, nu_test, 
/// max_relative=0.01);
///
/// // test 2
///
/// Re = 1600_f64;
/// lengthToDiameterRatio = 10000_f64;
///
/// nu_test = pipe_correlations::laminar_nusselt_uniform_heat_flux_developing(
/// Re,
/// Pr,
/// lengthToDiameterRatio);
///
/// approx::assert_relative_eq!(nu_reference, nu_test, 
/// max_relative=0.01);
///
/// // test 3
///
///
/// Re = 2300_f64;
/// lengthToDiameterRatio = 10000_f64;
///
/// nu_test = pipe_correlations::laminar_nusselt_uniform_heat_flux_developing(
/// Re,
/// Pr,
/// lengthToDiameterRatio);
///
/// approx::assert_relative_eq!(nu_reference, nu_test, 
/// max_relative=0.01);
///
///
/// // test 4 (CIET prototypical test)
///
/// nu_reference = 4.36_f64 * 1.75;
///
/// Re = 200_f64;
/// lengthToDiameterRatio = 87_f64;
/// Pr = 18_f64;
///
/// nu_test = pipe_correlations::laminar_nusselt_uniform_heat_flux_developing(
/// Re,
/// Pr,
/// lengthToDiameterRatio);
///
/// approx::assert_relative_eq!(nu_reference, nu_test, 
/// max_relative=0.08);
///
///
/// 
/// ```
///
/// For fully developed flow, multiple data points were available
/// for comparison. Otherwise, there were not as many data points.
///
///
///
pub fn laminar_nusselt_uniform_heat_flux_developing(
    reynolds_number: f64, prandtl_number: f64, length_to_diameter_ratio: f64) -> f64 {
    if reynolds_number > 2300_f64 {
        panic!("turbulent Re > 2300");
    }

    if reynolds_number > 2300_f64 {
        panic!("
               laminar_nusselt_uniform_heat_flux_developing 
               error 
               turbulent Re > 2300");
    }

    if reynolds_number == 0_f64 {
        // if Re = 0, no flow, 
        // we should have Nu = 1, which is as good as conduction
        return 1.0;
    }

    if reynolds_number < 0_f64 {
        panic!("laminar_nusselt_uniform_heat_flux_developing 
               error Re < 0");
    }

    if prandtl_number < 0_f64 {
        panic!("laminar_nusselt_uniform_heat_flux_developing 
               error Pr < 0");
    }

    if length_to_diameter_ratio <= 0_f64 {
        panic!("laminar_nusselt_uniform_heat_flux_developing 
               error lengthToDiameterRatio < 0");
    }


    let diameter_to_length_ratio = length_to_diameter_ratio.powf(-1.0);

    let term_1 :f64 = 4.354;
    let term_2 :f64 = 1.953 * (reynolds_number * prandtl_number * diameter_to_length_ratio).
        powf(0.3333333333) - 0.6;

    let term_3 :f64 = 0.924 * (reynolds_number * prandtl_number * diameter_to_length_ratio).powf(0.5) *
        (prandtl_number).powf(-0.166666666666666667);


    let nusselt_number = (term_1.powf(3.0) +
                          0.6_f64.powf(3.0) +
                          term_2.powf(3.0) +
                          term_3.powf(3.0)).powf(0.33333333333333);


    return nusselt_number;
}


/// estimates Nusselt Number for developing flow 
/// in turbulent regime (Re > 4000)
/// for tubes
/// regardless of boundary conditions (constant heat flux, wall temp 
/// mixed or anything else)
///
///
/// Re, Pr_fluid is measured at bulk temp
/// T_bulk = (T_in + T_out)/2
///
/// Pr_wall is liquid Pr at wall temperature
///
/// using gnielinski's data, we can get a Nu of 16 
/// at Pr_fluid = 0.7, Re = 5000
/// Pr_wall = 0.7
/// d/L = 0.0001 or
/// L/D  = 10000
///
/// darcy friction factor at these conditions 
/// Re = 5000, L/D = 10000 is calculated
/// for smooth tubes
///
/// darcy_friction_factor = 1.8 * log10 (Re) - 1.5
///
/// ```rust
///
/// extern crate approx;
/// use thermal_hydraulics_rs::tuas_boussinesq_solver::fluid_mechanics_correlations::
/// darcy;
/// use thermal_hydraulics_rs::tuas_boussinesq_solver::heat_transfer_correlations::
/// nusselt_number_correlations::pipe_correlations;
///
/// let mut nu_reference = 16_f64;
///
///
/// // test 1
///
/// let mut Re = 5000_f64;
/// let mut Pr = 0.7_f64;
/// let mut Pr_wall = 0.7_f64;
/// let mut lengthToDiameterRatio = 10000_f64;
///
/// let mut darcy_friction_factor :f64 = 
/// darcy(Re, 0.0).unwrap();
///
/// let mut nu_test = pipe_correlations::gnielinski_turbulent_correlation_liquids_developing_bulk_fluid_prandtl(
/// Re,
/// Pr,
/// Pr_wall,
/// darcy_friction_factor,
/// lengthToDiameterRatio);
///
///
///
/// approx::assert_relative_eq!(nu_reference, nu_test, 
/// max_relative=0.02);
/// ```
///
pub fn gnielinski_turbulent_correlation_liquids_developing_bulk_fluid_prandtl(
    reynolds_number: f64, prandtl_number_bulk_fluid: f64, 
    prandtl_number_wall: f64,
    darcy_friction_factor: f64,
    length_to_diameter_ratio: f64) -> f64 {

    if reynolds_number < 4000_f64 {
        panic!("laminar or transition Re < 4000");
    }


    if prandtl_number_bulk_fluid < 0.46_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error Pr_fluid < 0.46, out of experimental data range");
    }

    if prandtl_number_wall < 0.46_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error Pr_wall < 0.46, out of experimental data range");
    }

    if prandtl_number_bulk_fluid > 346_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error Pr_fluid > 346, out of experimental data range");
    }

    if prandtl_number_wall > 346_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error Pr_wall > 346, out of experimental data range");
    }

    if length_to_diameter_ratio <= 0_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error lengthToDiameterRatio < 0");
    }


    let prandtl_ratio: f64 = prandtl_number_bulk_fluid/prandtl_number_wall;

    let entrance_region_correction: f64 = 
        1.0 + length_to_diameter_ratio.powf(-0.6666666667);

    // now we start calculating
    let darcy_ratio: f64 = darcy_friction_factor/8.0;

    let numerator: f64 = darcy_ratio * (reynolds_number - 1000_f64) * prandtl_number_bulk_fluid *
        prandtl_ratio.powf(0.11);
    let denominator:f64 = 1_f64 + 12.7_f64 * darcy_ratio.powf(0.5) *
        (prandtl_number_bulk_fluid.powf(0.666667) - 1.0);

    let fluid_nusselt_number = numerator/denominator*
        entrance_region_correction;
    

    return fluid_nusselt_number;

}
        

/// estimates Nusselt Number for thermally developing flow 
/// in turbulent regime (Re > 4000)
/// for tubes
/// regardless of boundary conditions
/// regardless of boundary conditions (constant heat flux, wall temp 
/// mixed or anything else)
///
///
/// Re, Pr_fluid is measured at film temp
/// T_film = (T_bulk + T_wall)/2
///
/// Where:
/// T_bulk = (T_in + T_out)/2
/// 
/// For the correction factor, 
///
/// (Pr_bulk/Pr_wall) is used.
///
/// You may choose to set Pr_bulk = Pr_film if you so wish, but there 
/// is flexibility in this aspect
///
pub fn gnielinski_turbulent_correlation_liquids_developing(
    reynolds_number_film: f64, 
    prandtl_number_bulk_fluid: f64, 
    prandtl_number_film: f64,
    prandtl_number_wall: f64,
    darcy_friction_factor: f64,
    length_to_diameter_ratio: f64) -> f64 {

    if reynolds_number_film < 4000_f64 {
        panic!("laminar or transition Re < 4000");
    }


    if prandtl_number_bulk_fluid < 0.46_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error Pr_fluid < 0.46, out of experimental data range");
    }

    if prandtl_number_wall < 0.46_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error Pr_wall < 0.46, out of experimental data range");
    }

    if prandtl_number_bulk_fluid > 346_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error Pr_fluid > 346, out of experimental data range");
    }

    if prandtl_number_wall > 346_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error Pr_wall > 346, out of experimental data range");
    }

    if length_to_diameter_ratio <= 0_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error lengthToDiameterRatio < 0");
    }


    let prandtl_ratio: f64 = prandtl_number_bulk_fluid/prandtl_number_wall;

    let entrance_region_correction: f64 = 
        1.0 + length_to_diameter_ratio.powf(-0.6666666667);

    // now we start calculating
    let darcy_ratio: f64 = darcy_friction_factor/8.0;

    let numerator: f64 = darcy_ratio * (reynolds_number_film - 1000_f64) 
        * prandtl_number_film *
        prandtl_ratio.powf(0.11);
    let denominator:f64 = 1_f64 + 12.7_f64 * darcy_ratio.powf(0.5) *
        (prandtl_number_film.powf(0.666667) - 1.0);

    let fluid_nusselt_number = numerator/denominator*
        entrance_region_correction;
    

    return fluid_nusselt_number;

}

/// Gnielinski correlation for developing
/// flow regimes (both thermally and hydrodynamically)
/// for pipe flows with liquids
///
/// and for turbulent, developing and lamianr regimes
/// uses uniform heat flux correlations in laminar regime
///
/// Gnielinski, V. (2013). On heat 
/// transfer in tubes. International Journal 
/// of Heat and Mass Transfer, 63, 134-140.
///
/// The reference test data is as follows:
/// at Pr_fluid = 0.7, Pr_wall = 0.7
/// Re = 3000,
///
/// d/L = 0.0001 (L/D = 10000)
/// Nu is approximately 8.2
///
///
/// ```rust
///
/// extern crate approx;
/// extern crate thermal_hydraulics_rs;
/// use thermal_hydraulics_rs::tuas_boussinesq_solver::fluid_mechanics_correlations::
/// darcy;
/// use thermal_hydraulics_rs::tuas_boussinesq_solver::heat_transfer_correlations::
/// nusselt_number_correlations::pipe_correlations;
///
/// // test 1 (transition region)
///
/// let mut nu_reference = 8.2_f64;
/// let mut Re = 3000_f64;
/// let mut Pr = 0.7_f64;
/// let mut Pr_wall = 0.7_f64;
/// let mut lengthToDiameterRatio = 10000_f64;
///
/// let mut darcy_friction_factor :f64 = 
/// darcy(Re, 0.0).unwrap();
///
/// let mut nu_test =
/// pipe_correlations::
/// gnielinski_correlation_interpolated_uniform_heat_flux_liquids_developing_bulk_fluid_prandtl(
/// Re,
/// Pr,
/// Pr_wall,
/// darcy_friction_factor,
/// lengthToDiameterRatio);
///
///
///
/// approx::assert_relative_eq!(nu_reference, nu_test, 
/// max_relative=0.02);
///
///
/// // test 2 (turbulent regime)
///
/// let mut nu_reference = 16_f64;
/// let mut Re = 5000_f64;
/// let mut Pr = 0.7_f64;
/// let mut Pr_wall = 0.7_f64;
/// let mut lengthToDiameterRatio = 10000_f64;
///
/// let mut darcy_friction_factor :f64 = 
/// darcy(Re, 0.0).unwrap();
///
/// let mut nu_test =
/// pipe_correlations::
/// gnielinski_correlation_interpolated_uniform_heat_flux_liquids_developing_bulk_fluid_prandtl(
/// Re,
/// Pr,
/// Pr_wall,
/// darcy_friction_factor,
/// lengthToDiameterRatio);
///
///
///
/// approx::assert_relative_eq!(nu_reference, nu_test, 
/// max_relative=0.02);
///
/// // test 3 (laminar regime)
///
/// let mut nu_reference = 4.36;
/// let mut Re = 1000_f64;
/// let mut Pr = 0.7_f64;
/// let mut Pr_wall = 0.7_f64;
/// let mut lengthToDiameterRatio = 10000_f64;
///
/// let mut darcy_friction_factor :f64 = 
/// darcy(Re, 0.0).unwrap();
///
/// let mut nu_test =
/// pipe_correlations::
/// gnielinski_correlation_interpolated_uniform_heat_flux_liquids_developing_bulk_fluid_prandtl(
/// Re,
/// Pr,
/// Pr_wall,
/// darcy_friction_factor,
/// lengthToDiameterRatio);
///
///
///
/// approx::assert_relative_eq!(nu_reference, nu_test, 
/// max_relative=0.02);
/// ```
pub fn gnielinski_correlation_interpolated_uniform_heat_flux_liquids_developing_bulk_fluid_prandtl(
    reynolds: f64, 
    prandtl_number_fluid: f64, 
    prandtl_number_wall: f64,
    darcy_friction_factor: f64,
    length_to_diameter_ratio: f64) -> f64 {



    if prandtl_number_fluid < 0.46_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error Pr_fluid < 0.46, out of experimental data range");
    }

    if prandtl_number_wall < 0.46_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error Pr_wall < 0.46, out of experimental data range");
    }

    if prandtl_number_fluid > 346_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error Pr_fluid > 346, out of experimental data range");
    }

    if prandtl_number_wall > 346_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error Pr_wall > 346, out of experimental data range");
    }

    if length_to_diameter_ratio <= 0_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error lengthToDiameterRatio < 0");
    }

    // if this is turbulent flow, use the
    // turbulent correlation
    if reynolds > 4000_f64 {
        let fluid_nusselt_number = 
            gnielinski_turbulent_correlation_liquids_developing_bulk_fluid_prandtl(
                reynolds, 
                prandtl_number_fluid, 
                prandtl_number_wall, 
                darcy_friction_factor, 
                length_to_diameter_ratio);

        return fluid_nusselt_number;
    }

    // if this is laminar flow, 
    // use laminar flow correlation for uniform heat flux
    if reynolds < 2300_f64 {
        let fluid_nusselt_number = 
            laminar_nusselt_uniform_heat_flux_developing(
                reynolds, 
                prandtl_number_fluid, 
                length_to_diameter_ratio);

        return fluid_nusselt_number;
    }

    // if in transition region, then interpolate

    let laminar_nusselt = 
            laminar_nusselt_uniform_heat_flux_developing(
                2300_f64, 
                prandtl_number_fluid, 
                length_to_diameter_ratio);

    let turbulent_nusselt = 
        gnielinski_turbulent_correlation_liquids_developing_bulk_fluid_prandtl(
            4000_f64, 
            prandtl_number_fluid, 
            prandtl_number_wall, 
            darcy_friction_factor, 
            length_to_diameter_ratio);


    // the interpolation factor is known as gamma
    // in gnielinski's paper
    let gamma = (reynolds - 2300_f64)/(4000_f64 - 2300_f64);

    let fluid_nusselt_number = 
        (1_f64 - gamma) * laminar_nusselt +
        gamma * turbulent_nusselt;
    
    return fluid_nusselt_number;


}

/// Gnielinski correlation for developing
/// flow regimes (both thermally and hydrodynamically)
/// for pipe flows with liquids
///
/// and for turbulent, developing and lamianr regimes
/// uses uniform heat flux correlations in laminar regime
///
/// Gnielinski, V. (2013). On heat 
/// transfer in tubes. International Journal 
/// of Heat and Mass Transfer, 63, 134-140.
///
/// rather than use only the bulk and wall prandt number
/// for nusselt calculation,
/// a film prandtl number is also used here. 
/// This film prandtl number will be used to calculate the nusselt
/// number in all regimes, 
///
/// Whereas the bulk and wall prandtl number are only used in the 
/// correction factor in the turbulent and transition regime.
///
pub fn gnielinski_correlation_interpolated_uniform_heat_flux_liquids_developing(
    reynolds_number_film: f64, 
    prandtl_number_bulk_fluid: f64, 
    prandtl_number_film: f64,
    prandtl_number_wall: f64,
    darcy_friction_factor: f64,
    length_to_diameter_ratio: f64) -> f64 {



    if prandtl_number_bulk_fluid < 0.46_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error Pr_fluid < 0.46, out of experimental data range");
    }

    if prandtl_number_film < 0.46_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error Pr_film < 0.46, out of experimental data range");
    }

    if prandtl_number_wall < 0.46_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error Pr_wall < 0.46, out of experimental data range");
    }

    if prandtl_number_bulk_fluid > 346_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error Pr_fluid > 346, out of experimental data range");
    }

    if prandtl_number_film > 346_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error Pr_film > 346, out of experimental data range");
    }

    if prandtl_number_wall > 346_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error Pr_wall > 346, out of experimental data range");
    }

    if length_to_diameter_ratio <= 0_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error lengthToDiameterRatio < 0");
    }

    // if this is turbulent flow, use the
    // turbulent correlation
    if reynolds_number_film > 4000_f64 {
        let fluid_nusselt_number = 
            gnielinski_turbulent_correlation_liquids_developing(
                reynolds_number_film, 
                prandtl_number_bulk_fluid, 
                prandtl_number_film,
                prandtl_number_wall, 
                darcy_friction_factor, 
                length_to_diameter_ratio);

        return fluid_nusselt_number;
    }

    // if this is laminar flow, 
    // use laminar flow correlation for uniform heat flux
    if reynolds_number_film < 2300_f64 {
        let fluid_nusselt_number = 
            laminar_nusselt_uniform_heat_flux_developing(
                reynolds_number_film, 
                prandtl_number_film, 
                length_to_diameter_ratio);

        return fluid_nusselt_number;
    }

    // if in transition region, then interpolate

    let laminar_nusselt = 
            laminar_nusselt_uniform_heat_flux_developing(
                2300_f64, 
                prandtl_number_film, 
                length_to_diameter_ratio);

    let turbulent_nusselt = 
        gnielinski_turbulent_correlation_liquids_developing(
            4000_f64, 
            prandtl_number_bulk_fluid, 
            prandtl_number_film,
            prandtl_number_wall, 
            darcy_friction_factor, 
            length_to_diameter_ratio);


    // the interpolation factor is known as gamma
    // in gnielinski's paper
    let gamma = (reynolds_number_film - 2300_f64)/(4000_f64 - 2300_f64);

    let fluid_nusselt_number = 
        (1_f64 - gamma) * laminar_nusselt +
        gamma * turbulent_nusselt;
    
    return fluid_nusselt_number;


}

/// from Du's paper
///
/// Du, B. C., He, Y. L., Qiu, Y., Liang, Q., & Zhou, Y. P. (2018). 
/// Investigation on heat transfer characteristics of molten salt in 
/// a shell-and-tube heat exchanger. International Communications 
/// in Heat and Mass Transfer, 96, 61-68.
///
/// we have a generic Gnielinski type correlation, 
/// empirically fitted to experimental data. This is in the form:
///
/// Nu = C (Re^m - 280.0) Pr^0.4 ( 1.0 + (D_e/l)^(2/3) ) ( Pr_f / Pr_w )^0.25
///
/// Du did not mention which Pr to use 
/// I'm going to assume this is Pr_film 
///
/// Nu = C (Re^m - 280.0) Pr_film^0.4 ( 1.0 + (D_e/l)^(2/3) ) ( Pr_f / Pr_w )^0.25
///
/// Technically this Pr is Pr(T_film) where 
/// T_film = (T_wall + T_bulkfluid)/2 
///
/// a simpler estimate is:
/// Pr_film = (Pr_wall + Pr_fluid)/2
///
/// However, the simplest is just to use Pr_bulk as Pr 
/// this may underestimate Nusselt number, as Pr in the bulk fluid is 
/// usually lower, but it may well work
///
/// anyway, I just forced the user to give another argument (Pr_film)
/// After some debugging however, i found this unnecessary.
/// Pr_film should equal Pr_bulk by default
///
/// For Du's Heat exchanger, 
/// C = 0.04318,
/// m = 0.7797
/// 
/// No specific bounds are given
pub fn custom_gnielinski_turbulent_nusselt_correlation(
    correlation_coefficient_c: Ratio,
    reynolds_exponent_m: f64,
    prandtl_number_film: Ratio,
    prandtl_number_fluid: Ratio,
    prandtl_number_wall: Ratio,
    reynolds_number: Ratio,
    length_to_diameter_ratio: Ratio,
    ) -> Ratio {

    let reynolds_num_float: f64 = reynolds_number.get::<ratio>();

    // (Re^m - 280.0)
    let reynolds_bracket_term: f64 = 
        reynolds_num_float.powf(reynolds_exponent_m) - 280.0;

    // Pr_film^0.4
    // 
    let prandtl_term = 
        prandtl_number_film.get::<ratio>().powf(0.4);

    // ( 1.0 + (D_e/l)^(2/3) )
    // I'm providing l/d rather than d/l
    // so it is raised to -2/3, which I approximate as 
    // -0.6666666667
    let length_to_diameter_term = 
        1.0 + length_to_diameter_ratio.get::<ratio>().powf(-0.6666666667);

    // (Pr_f/Pr_w)^0.25
    let prandtl_correction_term: f64 = 
        (prandtl_number_fluid/prandtl_number_wall).get::<ratio>().powf(0.25);


    let nusselt_number = 
        correlation_coefficient_c * 
        reynolds_bracket_term *
        prandtl_term *
        length_to_diameter_term * 
        prandtl_correction_term;



    return nusselt_number;
}


/// from Du's paper
///
/// Du, B. C., He, Y. L., Qiu, Y., Liang, Q., & Zhou, Y. P. (2018). 
/// Investigation on heat transfer characteristics of molten salt in 
/// a shell-and-tube heat exchanger. International Communications 
/// in Heat and Mass Transfer, 96, 61-68.
///
/// we have a generic Gnielinski type correlation, 
/// empirically fitted to experimental data. This is in the form:
///
/// Nu = C (Re^m - 280.0) Pr_f^0.4 ( 1.0 + (D_e/l)^(2/3) ) ( Pr_f / Pr_w )^0.25
///
/// For Du's Heat exchanger, 
/// C = 0.04318,
/// m = 0.7797
/// 
/// 
/// However, this does not cover the transition or laminar regimes,
/// I used Gnielinski correlation for developing
/// flow regimes (both thermally and hydrodynamically)
/// for pipe flows with liquids
///
/// and for turbulent, developing and lamianr regimes
/// uses uniform heat flux correlations in laminar regime
///
/// Gnielinski, V. (2013). On heat 
/// transfer in tubes. International Journal 
/// of Heat and Mass Transfer, 63, 134-140.
///
/// No specific bounds are given for Prandtl number or otherwise
/// 
///
/// the transition regime for pipes is around Re = 2300 - 4000 
/// this is taken from the Re for transition in pipes 
///
/// However, for transitions in tube bundles, we expect them 
/// for around Re = 40-100 
///
/// Takemoto, Y., Kawanishi, K., & Mizushima, J. (2010). Heat transfer 
/// in the flow through a bundle of tubes and transitions of the flow. 
/// International journal of heat and mass transfer, 53(23-24), 5411-5419.
///
/// I will use the Re from 40-100 as the transition regime
/// at Re of 40 and below, Nu is the same as for pipe lamniar flow
///
/// IT MAY NOT BE APPLICABLE IN THIS CASE, but its a decent estimate
///
/// darcy friction factor is not used for this case
pub fn custom_gnielinski_correlation_interpolated_uniform_heat_flux_liquids_developing(
    correlation_coefficient_c: Ratio,
    reynolds_exponent_m: f64,
    prandtl_number_film: Ratio,
    prandtl_number_fluid: Ratio,
    prandtl_number_wall: Ratio,
    reynolds_number: Ratio,
    length_to_diameter_ratio: Ratio,
    ) -> f64 {

    let transition_regime_reynolds_high_bound_float = 
        100_f64;

    let transition_regime_reynolds_low_bound_float = 
        40_f64;
        



    if length_to_diameter_ratio.get::<ratio>() <= 0_f64 {
        panic!("gnielinski_correlation_liquids_developing \n
               error lengthToDiameterRatio < 0");
    }

    let reynolds = reynolds_number.get::<ratio>();
    

    // if this is turbulent flow, use the
    // turbulent correlation
    if reynolds > transition_regime_reynolds_high_bound_float {
        let fluid_nusselt_number: f64 = 
            custom_gnielinski_turbulent_nusselt_correlation(
                correlation_coefficient_c,
                reynolds_exponent_m,
                prandtl_number_film,
                prandtl_number_fluid, 
                prandtl_number_wall, 
                reynolds_number, 
                length_to_diameter_ratio).get::<ratio>();

        return fluid_nusselt_number;
    }

    // if this is laminar flow, 
    // use laminar flow correlation for uniform heat flux
    if reynolds < transition_regime_reynolds_low_bound_float {
        let fluid_nusselt_number = 
            laminar_nusselt_uniform_heat_flux_developing(
                reynolds, 
                prandtl_number_fluid.get::<ratio>(), 
                length_to_diameter_ratio.get::<ratio>());

        return fluid_nusselt_number;
    }

    // if in transition region, then interpolate

    let laminar_nusselt = 
            laminar_nusselt_uniform_heat_flux_developing(
                transition_regime_reynolds_low_bound_float, 
                prandtl_number_fluid.get::<ratio>(), 
                length_to_diameter_ratio.get::<ratio>());

    let turbulent_nusselt = 
        custom_gnielinski_turbulent_nusselt_correlation(
            correlation_coefficient_c,
            reynolds_exponent_m,
            prandtl_number_film,
            prandtl_number_fluid, 
            prandtl_number_wall, 
            reynolds_number, 
            length_to_diameter_ratio).get::<ratio>();


    // the interpolation factor is known as gamma
    // in gnielinski's paper
    let gamma = (reynolds - transition_regime_reynolds_low_bound_float)
        /(transition_regime_reynolds_high_bound_float 
            - transition_regime_reynolds_low_bound_float);

    let fluid_nusselt_number = 
        (1_f64 - gamma) * laminar_nusselt +
        gamma * turbulent_nusselt;
    
    return fluid_nusselt_number;


}
