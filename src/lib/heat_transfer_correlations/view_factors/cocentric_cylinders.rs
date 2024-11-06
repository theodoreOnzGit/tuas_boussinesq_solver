use std::f64::consts::PI;

use ndarray_linalg::Scalar;
use uom::si::{f64::*, ratio::ratio};

/// F_(2-2) =  1 - 1/R - ((H^2 + 4 R^2)^0.5 -H)/(4 R) + 1/PI * B
///
///
/// Where B = 2/R atan (2 * sqrt(R^2 - 1)/H) - H/(2 R) * C 
///
/// And C = D asin(E) - asin (F)
///
/// D = sqrt(4 R^2 + H^2)/H 
/// E = (H^2 + 4(R^2 - 1) - 2 H^2/R^2)/(H^2  + 4 (R^2 - 1))
///
/// F = (R^2 - 2)/R^2
pub fn outer_cylinder_self_view_factor(
    inner_diameter: Length,
    outer_diameter: Length,
    cylinder_height: Length) -> Ratio {

    let ratio_r: Ratio = outer_diameter/inner_diameter;
    let r_1: Length = 0.5 * inner_diameter;
    let ratio_h: Ratio = cylinder_height/r_1;

    let r_value: f64 = ratio_r.into();
    let h_value: f64 = ratio_h.into();

    let r_sq = r_value.powf(2.0);
    let h_sq = h_value.powf(2.0);
    let one_over_r = r_value.recip();
    let one_over_h = h_value.recip();
    // F = (R^2 - 2)/R^2
    let f = (r_sq - 2.0)/r_sq;

    // E = (H^2 + 4(R^2 - 1) - 2 H^2/R^2)/(H^2  + 4 (R^2 - 1))
    let e_numerator = h_sq + 4.0 * (r_sq - 1.0) - 2.0 * h_sq/r_sq;
    let e_denominator = h_sq + 4.0 * (r_sq - 1.0);

    let e = e_numerator/e_denominator;

    // D = sqrt(4 R^2 + H^2)/H 
    let d = (4.0 * r_sq + h_sq).sqrt() * one_over_h;

    let c = d * e.asin() - f.asin();

    let mut b = 2.0 * one_over_r * (2.0 * (r_sq-1.0).sqrt() * one_over_h).atan();
    b -= h_value * 0.5 * one_over_r * c;

    let view_factor_value: f64 = 
        1.0 - one_over_r
        - 0.25 * one_over_r * ((h_sq+ 4.0 * r_sq).sqrt() - h_value)
        + PI.recip() * b;



    return Ratio::new::<ratio>(view_factor_value);


}
