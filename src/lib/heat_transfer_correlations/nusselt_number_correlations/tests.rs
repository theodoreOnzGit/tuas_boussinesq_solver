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
/// Re, Nu_shell
/// 3510.033,42.582
/// 3571.349,43.32
/// 3691.75,43.852
/// 3751.951,44.672
/// 3794.314,44.795
/// 3847.826,45.574
/// 3959.309,47.09
/// 4019.509,47.459
/// 4267.001,53.238
/// 4356.187,54.836
/// 4550.167,58.238
/// 4630.435,59.303
/// 4730.769,60.451
/// 4942.586,62.582
/// 5230.212,63.77
/// 5388.517,64.344
/// 5481.048,65.861
///
/// From the paper 
/// for the salt, temperatures range from 204-236 C 
/// Pr is from 19.82 to 24.03, 
///
/// Pr = 22 seems reasonable for bulk fluid (Pr_f)
///
/// and the correction factor Pr_f/Pr_w is from 
/// 0.4273 to 0.5646, decent estimate is 0.5
///
/// These values allow us to calculate the salt Nusselt numbers 
/// to reproduce the Re and Nu_shell data.
///
/// I'm going to try the values at Re = 3510 (which is in the transitional 
/// regime), Re = 4019, which is in turbulent regime, 
/// and Re = 5481, which is also in the turbulent regime
///
///
#[test] 
pub fn du_correlation_empirical_test(){
    use uom::si::length::meter;
    use uom::si::ratio::ratio;
    use uom::si::f64::*;

    use crate::heat_transfer_correlations::
        nusselt_number_correlations::pipe_correlations::
        custom_gnielinski_turbulent_nusselt_correlation;


    let c = Ratio::new::<ratio>(0.04318);
    let m = 0.7797_f64;

    // now some parameters to determine things,
    // from Du's paper
    
    let heat_exchg_length = Length::new::<meter>(1.95);
    let tube_od = Length::new::<meter>(0.014);
    let shell_id = Length::new::<meter>(0.1);

    let number_of_tubes = 19;

    // from Du's paper, eqn 14
    // D_e = (D_i^2 - N_t d_o^2)/(D_i + N_t d_o)
    let effective_diameter = (
        shell_id * shell_id - 
        number_of_tubes as f64 
        * tube_od * tube_od) / 
        (shell_id + number_of_tubes as f64 * tube_od);


    let length_to_diameter = heat_exchg_length/effective_diameter;

    // in Du's paper, D_e/l is 0.009
    // let me test for that
    {
        let diameter_to_length = 
            effective_diameter/heat_exchg_length;

        approx::assert_relative_eq!(
            diameter_to_length.get::<ratio>(),
            0.009,
            max_relative=0.5
            );
    }




    let film_prandtl_number = Ratio::new::<ratio>(22.0);
    // Pr_bulk/Pr_w  is about 0.5, or
    // Pr_f/Pr_w  is about 0.5, 
    // Pr_f in these comments is Pr_bulk_fluid, or Pr_bulk
    //
    // and i made it such that 
    //
    // Pr_film = 0.5 * (Pr_bulk + Pr_wall)
    //
    // and Pr_bulk/Pr_wall = 0.5 
    //
    // Pr_bulk = 0.5 Pr_wall
    // substituting:
    // Pr_film = 0.5 * (1.5 Pr_wall)
    //
    // Pr_film = 22
    // 22 = 0.5 * (1.5 Pr_wall)
    //
    // based on these substitutions:
    let wall_prandtl_number = film_prandtl_number * 2.0 / 1.5;
    let bulk_prandtl_number = 0.5 * wall_prandtl_number;



    // define a test closure so I can easily test
    let test_fn = |reynolds_float: f64, expected_nusselt_float: f64,
        tolerance: f64|{
            let reynolds_num = Ratio::new::<ratio>(reynolds_float);

            let nusselt = custom_gnielinski_turbulent_nusselt_correlation(
                c, 
                m, 
                film_prandtl_number,
                bulk_prandtl_number, 
                wall_prandtl_number, 
                reynolds_num, 
                length_to_diameter);

            // max tolerance is 8%
            approx::assert_relative_eq!(
                nusselt.get::<ratio>(),
                expected_nusselt_float,
                max_relative=tolerance
            );

        };

    // test for Re about 5481, 
    // We should expect nusselt of 65.861 
    // with tolerance of 8% from expt data given Du's paper
    //
    test_fn(5481.048, 65.861 ,0.08);

    // test for Re about 4019.509, 
    // We should expect nusselt of 47.459
    // with tolerance of 8% from expt data given Du's paper
    //
    test_fn(4019.509, 47.459 ,0.08);

    // test for Re about 3510.033, 
    // We should expect nusselt of 42.582
    // with tolerance of 8% from expt data given Du's paper
    //
    test_fn(3510.033, 42.582 ,0.08);

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
/// Re, Nu_shell
/// 3510.033,42.582
/// 3571.349,43.32
/// 3691.75,43.852
/// 3751.951,44.672
/// 3794.314,44.795
/// 3847.826,45.574
/// 3959.309,47.09
/// 4019.509,47.459
/// 4267.001,53.238
/// 4356.187,54.836
/// 4550.167,58.238
/// 4630.435,59.303
/// 4730.769,60.451
/// 4942.586,62.582
/// 5230.212,63.77
/// 5388.517,64.344
/// 5481.048,65.861
///
/// From the paper 
/// for the salt, temperatures range from 204-236 C 
/// Pr is from 19.82 to 24.03, 
///
/// Pr = 22 seems reasonable for bulk fluid (Pr_f)
///
/// and the correction factor Pr_f/Pr_w is from 
/// 0.4273 to 0.5646, decent estimate is 0.5
///
/// These values allow us to calculate the salt Nusselt numbers 
/// to reproduce the Re and Nu_shell data.
///
/// I'm going to try the values at Re = 3510 (which is in the transitional 
/// regime for pipes, but not tubes), Re = 4019, which is in turbulent regime, 
/// and Re = 5481, which is also in the turbulent regime
///
/// but this time, I'm going to use the 
/// custom_gnielinski_correlation_interpolated_uniform_heat_flux_liquids_developing
/// as a benchmark 
///
/// This is because the function will interpolate between laminar 
/// and turbulent, 
/// now 
///
///
///
#[test] 
pub fn du_interpolated_correlation_empirical_test(){
    use uom::si::length::meter;
    use uom::si::ratio::ratio;
    use uom::si::f64::*;

    use crate::heat_transfer_correlations::
        nusselt_number_correlations::pipe_correlations::
        custom_gnielinski_correlation_interpolated_uniform_heat_flux_liquids_developing;

    let c = Ratio::new::<ratio>(0.04318);
    let m = 0.7797_f64;

    // now some parameters to determine things,
    // from Du's paper
    
    let heat_exchg_length = Length::new::<meter>(1.95);
    let tube_od = Length::new::<meter>(0.014);
    let shell_id = Length::new::<meter>(0.1);

    let number_of_tubes = 19;

    // from Du's paper, eqn 14
    // D_e = (D_i^2 - N_t d_o^2)/(D_i + N_t d_o)
    let effective_diameter = (
        shell_id * shell_id - 
        number_of_tubes as f64 
        * tube_od * tube_od) / 
        (shell_id + number_of_tubes as f64 * tube_od);


    let length_to_diameter = heat_exchg_length/effective_diameter;

    // in Du's paper, D_e/l is 0.009
    // let me test for that
    {
        let diameter_to_length = 
            effective_diameter/heat_exchg_length;

        approx::assert_relative_eq!(
            diameter_to_length.get::<ratio>(),
            0.009,
            max_relative=0.5
            );
    }

    // let me use now the D_e/l of 0.009
    //let length_to_diameter = 1.0 / Ratio::new::<ratio>(0.009);



    let film_prandtl_number = Ratio::new::<ratio>(22.0);
    // Pr_bulk/Pr_w  is about 0.5, or
    // Pr_f/Pr_w  is about 0.5, 
    // Pr_f in these comments is Pr_bulk_fluid, or Pr_bulk
    //
    // and i made it such that 
    //
    // Pr_film = 0.5 * (Pr_bulk + Pr_wall)
    //
    // and Pr_bulk/Pr_wall = 0.5 
    //
    // Pr_bulk = 0.5 Pr_wall
    // substituting:
    // Pr_film = 0.5 * (1.5 Pr_wall)
    //
    // Pr_film = 22
    // 22 = 0.5 * (1.5 Pr_wall)
    //
    // based on these substitutions:
    let wall_prandtl_number = film_prandtl_number * 2.0 / 1.5;
    let bulk_prandtl_number = 0.5 * wall_prandtl_number;


    // define a test closure so I can easily test
    let test_fn = |reynolds_float: f64, expected_nusselt_float: f64,
        tolerance: f64|{
            let reynolds_num = Ratio::new::<ratio>(reynolds_float);

            let nusselt_float = 
                custom_gnielinski_correlation_interpolated_uniform_heat_flux_liquids_developing(
                c, 
                m, 
                film_prandtl_number,
                bulk_prandtl_number, 
                wall_prandtl_number, 
                reynolds_num, 
                length_to_diameter);

            // max tolerance is 8%
            approx::assert_relative_eq!(
                nusselt_float,
                expected_nusselt_float,
                max_relative=tolerance
            );

        };

    // test for Re about 5481, 
    // We should expect nusselt of 65.861 
    // with tolerance of 8% from expt data given Du's paper
    //
    test_fn(5481.048, 65.861 ,0.08);

    // test for Re about 4019.509, 
    // We should expect nusselt of 47.459
    // with tolerance of 8% from expt data given Du's paper
    //
    test_fn(4019.509, 47.459 ,0.08);

    // test for Re about 3510.033, 
    // We should expect nusselt of 42.582
    // with tolerance of 8% from expt data given Du's paper
    //
    test_fn(3510.033, 42.582 ,0.08);

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
/// Re, Nu_shell
/// 3510.033,42.582
/// 3571.349,43.32
/// 3691.75,43.852
/// 3751.951,44.672
/// 3794.314,44.795
/// 3847.826,45.574
/// 3959.309,47.09
/// 4019.509,47.459
/// 4267.001,53.238
/// 4356.187,54.836
/// 4550.167,58.238
/// 4630.435,59.303
/// 4730.769,60.451
/// 4942.586,62.582
/// 5230.212,63.77
/// 5388.517,64.344
/// 5481.048,65.861
///
/// From the paper 
/// for the salt, temperatures range from 204-236 C 
/// Pr is from 19.82 to 24.03, 
///
/// Pr = 22 seems reasonable for bulk fluid (Pr_f)
///
/// and the correction factor Pr_f/Pr_w is from 
/// 0.4273 to 0.5646, decent estimate is 0.5
///
/// These values allow us to calculate the salt Nusselt numbers 
/// to reproduce the Re and Nu_shell data.
///
/// I'm going to try the values at Re = 3510 (which is in the transitional 
/// regime for pipes, but not tubes), Re = 4019, which is in turbulent regime, 
/// and Re = 5481, which is also in the turbulent regime
///
/// but this time, I'm going to use the nusselt correlation struct 
/// directly
///
///
///
#[test] 
pub fn du_nusselt_enum_correlation_empirical_test(){
    use uom::si::length::meter;
    use uom::{si::ratio::ratio, ConstZero};
    use uom::si::f64::*;

    use super::input_structs::GnielinskiData;
    use super::enums::NusseltCorrelation;

    let c = Ratio::new::<ratio>(0.04318);
    let m = 0.7797_f64;

    // now some parameters to determine things,
    // from Du's paper
    
    let heat_exchg_length = Length::new::<meter>(1.95);
    let tube_od = Length::new::<meter>(0.014);
    let shell_id = Length::new::<meter>(0.1);

    let number_of_tubes = 19;

    // from Du's paper, eqn 14
    // D_e = (D_i^2 - N_t d_o^2)/(D_i + N_t d_o)
    let effective_diameter = (
        shell_id * shell_id - 
        number_of_tubes as f64 
        * tube_od * tube_od) / 
        (shell_id + number_of_tubes as f64 * tube_od);


    let length_to_diameter = heat_exchg_length/effective_diameter;

    // in Du's paper, D_e/l is 0.009
    // let me test for that
    {
        let diameter_to_length = 
            effective_diameter/heat_exchg_length;

        approx::assert_relative_eq!(
            diameter_to_length.get::<ratio>(),
            0.009,
            max_relative=0.5
            );
    }

    // let me use now the D_e/l of 0.009
    //let length_to_diameter = 1.0 / Ratio::new::<ratio>(0.009);

    let film_prandtl_number = Ratio::new::<ratio>(22.0);

    // Pr_bulk/Pr_w  is about 0.5, or
    // Pr_f/Pr_w  is about 0.5, 
    // Pr_f in these comments is Pr_bulk_fluid, or Pr_bulk
    //
    // and i made it such that 
    //
    // Pr_film = 0.5 * (Pr_bulk + Pr_wall)
    //
    // and Pr_bulk/Pr_wall = 0.5 
    //
    // Pr_bulk = 0.5 Pr_wall
    // substituting:
    // Pr_film = 0.5 * (1.5 Pr_wall)
    //
    // Pr_film = 22
    // 22 = 0.5 * (1.5 Pr_wall)
    //
    // based on these substitutions:
    let wall_prandtl_number = film_prandtl_number * 2.0 / 1.5;
    let bulk_prandtl_number = 0.5 * wall_prandtl_number;
    // Pr_w/Pr_f = 2.0 approx 
    //

    let gnielinski_params: GnielinskiData = 
        GnielinskiData { 
            reynolds: Ratio::ZERO, 
            prandtl_bulk: Ratio::ZERO, 
            prandtl_wall: Ratio::ZERO, 
            darcy_friction_factor: Ratio::ZERO, 
            length_to_diameter,
        };


    let du_nusselt_correlation: NusseltCorrelation 
        = NusseltCorrelation::CustomGnielinskiGenericPrandtlFilm(
            gnielinski_params, c, m);



    // define a test closure so I can easily test
    let test_fn = |reynolds_float: f64, expected_nusselt_float: f64,
        tolerance: f64|{
            let reynolds_num = Ratio::new::<ratio>(reynolds_float);

            let nusselt_float = 
                du_nusselt_correlation.estimate_based_on_prandtl_reynolds_and_wall_correction(
                bulk_prandtl_number, 
                wall_prandtl_number, 
                reynolds_num)
                .unwrap()
                .get::<ratio>();

            // max tolerance is 8%
            approx::assert_relative_eq!(
                nusselt_float,
                expected_nusselt_float,
                max_relative=tolerance
            );

        };

    // test for Re about 5481, 
    // We should expect nusselt of 65.861 
    // with tolerance of 8% from expt data given Du's paper
    //
    test_fn(5481.048, 65.861 ,0.08);

    // test for Re about 4019.509, 
    // We should expect nusselt of 47.459
    // with tolerance of 8% from expt data given Du's paper
    //
    test_fn(4019.509, 47.459 ,0.08);

    // test for Re about 3510.033, 
    // We should expect nusselt of 42.582
    // with tolerance of 8% from expt data given Du's paper
    //
    test_fn(3510.033, 42.582 ,0.08);

}
