/// check hydraulic diameter calculated by manually against 
/// shell side hydraulic diameter 
/// in Du et al's paper within 3%
#[test]
pub fn check_shell_side_fluid_hydraulic_diameter(){
    use uom::si::f64::*;
    use uom::si::length::meter;
    use uom::si::ratio::ratio;

    // from Du's heat exchanger type, except we use one inner tube
    let tube_side_od = Length::new::<meter>(0.014);
    let number_of_tubes = 19_u32;
    let shell_side_id = Length::new::<meter>(0.1);

    let shell_side_fluid_hydraulic_diameter: Length = 
        (shell_side_id * shell_side_id - number_of_tubes as f64 *
         tube_side_od * tube_side_od)/
        (shell_side_id + number_of_tubes as f64 * tube_side_od);

    let hydraulic_diameter_by_l: Ratio = Ratio::new::<ratio>(0.009);
    let pipe_length = Length::new::<meter>(1.95);

    let given_de_by_du: Length = hydraulic_diameter_by_l * pipe_length;

    // max error is 3%
    approx::assert_relative_eq!(
        given_de_by_du.get::<meter>(),
        shell_side_fluid_hydraulic_diameter.get::<meter>(),
        max_relative = 0.03,
        );
}

/// basically, for a tube bundle, 
/// reynolds number is 
/// Re = 4/pi * V/(N_t d_i) 1/nu 
///
/// in Du's paper,
/// V is volumetric flowrate of 15.635 m^3/h 
/// N_t is number of tubes (19)
/// d_i = 0.01 m
///
/// The factor
/// 4/pi * V/(N_t d_i) should be 0.0291 m^2/s
#[test]
pub fn check_tube_side_reynolds_number_diffusivity(){
    use std::f64::consts::PI;

    use uom::si::{f64::*, ratio::ratio, volume_rate::cubic_meter_per_hour};
    use uom::si::length::meter;
    use uom::si::diffusion_coefficient::square_meter_per_second;
    
    let four_over_pi: Ratio = Ratio::new::<ratio>(4.0/PI);
    let vol_flowrate = VolumeRate::new::<cubic_meter_per_hour>(15.635);
    let d_i = Length::new::<meter>(0.01);
    let number_of_tubes: f64 = 19.0;

    let viscosity_scale: DiffusionCoefficient = 
        four_over_pi * vol_flowrate / (number_of_tubes * d_i);

    approx::assert_relative_eq!(
        viscosity_scale.get::<square_meter_per_second>(),
        0.0291,
        max_relative = 0.01
        );

}



/// shell side heat transfer area should be 1.63 m^2
#[test]
pub fn check_shell_side_heat_trf_area(){
    use uom::si::f64::*;
    use uom::si::length::meter;
    use uom::si::area::square_meter;
    use std::f64::consts::PI;

    // from Du's heat exchanger type, except we use one inner tube
    let tube_side_od = Length::new::<meter>(0.014);
    let number_of_tubes = 19_u32;
    let pipe_length = Length::new::<meter>(1.95);

    let shell_heat_trf_area: Area = 
        number_of_tubes as f64 * PI * tube_side_od * pipe_length;


    // max error is 1%
    approx::assert_relative_eq!(
        1.63,
        shell_heat_trf_area.get::<square_meter>(),
        max_relative = 0.01,
        );
}
