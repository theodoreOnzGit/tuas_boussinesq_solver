use std::f64::consts::PI;

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
///
/// formula inspected ok 8:34pm 06 nov
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

    // And C = D asin(E) - asin (F)
    let c = e.asin() * d - f.asin();

    // B = 2/R atan (2 * sqrt(R^2 - 1)/H) - H/(2 R) * C 
    let mut b = 2.0 * one_over_r * (2.0 * (r_sq-1.0).sqrt() * one_over_h).atan();
    b -= h_value * 0.5 * one_over_r * c;

    // F_(2-2) =  1 - 1/R - ((H^2 + 4 R^2)^0.5 -H)/(4 R) + 1/PI * B
    let view_factor_value: f64 = 
        1.0 - one_over_r
        - 0.25 * one_over_r * ((h_sq+ 4.0 * r_sq).sqrt() - h_value)
        + PI.recip() * b;



    return Ratio::new::<ratio>(view_factor_value);


}

/// F_(2-1) = 1/R * ( 1 - B - 1/PI C )
///
/// C = D - E - F 
///
/// D = acos (hsq_minus_rsq_plus_one/hsq_plus_rsq_minus_one)
/// E = e1 acos (e2)
///
/// e1 = sqrt( hsq_plus_rsq_plus_one^2 - 4.0 * rsq)/(2H)
/// e2 = (hsq_minus_rsq_plus_one)/(R * hsq_plus_rsq_minus_one)
///
///
/// hsq_minus_rsq_plus_one = H^2 - R^2 + 1
/// hsq_plus_rsq_minus_one = H^2 + R^2 - 1
/// hsq_plus_rsq_plus_one = H^2 + R^2 + 1
///
/// F = (hsq_minus_rsq_plus_one)/(2H) asin (1/R)
///
/// B = (hsq_plus_rsq_minus_one)/(4 H)
///
///
/// outer cylinder to inner cylinder view factor
///
/// visually inspected 8:39pm 06 nov 2024
pub fn outer_cylinder_to_inner_cylinder_view_factor(
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
    let hsq_minus_rsq_plus_one = h_sq - r_sq + 1.0;
    let hsq_plus_rsq_minus_one = h_sq + r_sq - 1.0;
    let hsq_plus_rsq_plus_one = h_sq + r_sq + 1.0;

    // F = (hsq_minus_rsq_plus_one)/(2H) asin (1/R)
    let f = hsq_minus_rsq_plus_one * 0.5 * one_over_h * (one_over_r.asin());

    // E = e1 acos (e2)
    //
    // e1 = sqrt( hsq_plus_rsq_plus_one^2 - 4.0 * rsq)/(2H)
    let e1 = (hsq_plus_rsq_plus_one.powf(2.0) - 4.0 * r_sq).sqrt() * 
        0.5 * one_over_h;

    // e2 = (hsq_minus_rsq_plus_one)/(R * hsq_plus_rsq_minus_one)
    let e2 = hsq_minus_rsq_plus_one / (hsq_plus_rsq_minus_one) * one_over_r;

    let e_value = e1 * (e2.acos());


    // D = acos (hsq_minus_rsq_plus_one/hsq_plus_rsq_minus_one)
    let d = (hsq_minus_rsq_plus_one/hsq_plus_rsq_minus_one).acos();

    // C = D - E - F 
    let c = d - e_value - f;

    let b = hsq_plus_rsq_minus_one * 0.25 * one_over_h;
    // F_(2-1) = 1/R * ( 1 - B - 1/PI C )

    let view_factor_value: f64 = 
        one_over_r * (1.0 - b - PI.recip() * c);

    return Ratio::new::<ratio>(view_factor_value);


}


/// outer cylinder to annular end ring (enclosing space between 
/// coaxial cylinders)
///
///
/// terms:
/// r_1 = inner_radius
/// r_2 = outer_radius
///
/// H = cylinder_height/r_2 
/// R = r_1/r_2
///
/// X = sqrt(1-R^2)
/// Y = (R(1 - R^2 - H^2))/(1 - R^2 + H^2)
///
/// F_(1-2) = 1/PI * (A + B + C - D + E)
///
///
/// A =  R (atan(X/H) -  atan(2X/H))
///
/// B = H/4 * ( asin(2R^2 - 1) - asin (R))
///
/// C = X^2/(4 H) * (PI/2 + asin(R))
///
/// D = d1 * d2 
///
/// d1 = sqrt( (1 + R^2 + H^2)^2 -  4 R^2) / (4H)
/// d2 = PI/2 + asin(Y)
///
/// E = e1 * e2 
///
/// e1 = sqrt (4 + H^2)/4 
/// e2 = PI/2 + asin(1 -  2 R^2 H^2 / (4 X^2 + H^2))
pub fn outer_cylinder_to_annular_end_ring_view_factor(
    inner_diameter: Length,
    outer_diameter: Length,
    cylinder_height: Length) -> Ratio {

    // R = r_1/r_2
    let ratio_r: Ratio = inner_diameter/outer_diameter;
    let r_2: Length = 0.5 * outer_diameter;
    // H = cylinder_height/r_2 
    let ratio_h: Ratio = cylinder_height/r_2;

    let r_value: f64 = ratio_r.into();
    let h_value: f64 = ratio_h.into();

    // square and inverse values
    let r_sq = r_value.powf(2.0);
    let h_sq = h_value.powf(2.0);
    let one_over_h = h_value.recip();

    // x and y
    let x_value = (1.0 - r_sq).sqrt();
    let y_value = r_value * (1.0 - r_sq - h_sq)/(1.0 - r_sq + h_sq);

    // common ratios 
    let x_by_h = x_value/h_value;
    let x_sq = x_value.powf(2.0);


    // factors
    // A =  R (atan(X/H) -  atan(2X/H))

    let a = r_value * (x_by_h.atan() - (2.0 * x_by_h).atan());

    // B = H/4 * ( asin(2R^2 - 1) - asin (R))
    
    let b = h_value * 0.25 * ((2.0 * r_sq - 1.0).asin() - r_value.asin());

    // C = X^2/(4 H) * (PI/2 + asin(R))

    let c = x_sq * 0.25 * one_over_h * (0.5 * PI + r_value.asin());

    // D = d1 * d2 
    //
    // d1 = sqrt (1 + R^2 + H^2)^2 -  4 R^2) / (4H)
    // d2 = PI/2 + asin(Y)
    let d1 = ((1.0 + r_sq + h_sq).powf(2.0) - 4.0 * r_sq).sqrt() * 
        0.25 * one_over_h;

    let d2 = 0.5 * PI + y_value.asin();

    let d = d1 * d2;

    //
    // E = e1 * e2 
    //
    // e1 = sqrt (4 + H^2)/4 
    // e2 = PI/2 + asin(1 -  2 R^2 H^2 / (4 X^2 + H^2))

    let e1 = (4.0 + h_sq).sqrt() * 0.25;

    let e2 = 0.5 * PI + (1.0 - 2.0 * r_sq * h_sq /(4.0 * x_sq + h_sq)).asin();
    let e_value = e1 * e2 ;


    // F_(1-2) = 1/PI * (A + B + C - D + E)
    let view_factor_value = PI.recip() * (a + b + c - d + e_value);

    return Ratio::new::<ratio>(view_factor_value);

}



#[cfg(test)]
#[test]
pub fn cocentric_cylinders_view_factor_shld_equal_one_for_outer_cyl(){
    use uom::si::length::meter;


    let inner_diameter = Length::new::<meter>(1.0);
    let outer_diameter = Length::new::<meter>(2.0);

    let cylinder_height = Length::new::<meter>(5.0);

    let self_view_factor = outer_cylinder_self_view_factor(
        inner_diameter, outer_diameter, cylinder_height);

    let outer_to_inner_cyl_view_factor = 
        outer_cylinder_to_inner_cylinder_view_factor(
            inner_diameter, outer_diameter, cylinder_height);

    // this needs to be multiplied twice or added twice 
    // as there are two ends
    let outer_cyl_to_annular_end_ring_view_factor = 
        outer_cylinder_to_annular_end_ring_view_factor(
            inner_diameter, outer_diameter, cylinder_height);

    let total_view_factor = 
        self_view_factor + 
        outer_to_inner_cyl_view_factor + 
        outer_cyl_to_annular_end_ring_view_factor
        + outer_cyl_to_annular_end_ring_view_factor;

    approx::assert_relative_eq!(
        total_view_factor.get::<ratio>(),
        1.0,
        max_relative = 1e-5
        );

}


/// using view factor algebra, compute inner cylinder to outer cylinder 
/// view factor
///
/// A_inner (F_inner to outer) =  A_outer (F_outer to inner)
///
/// A_outer/A_inner = (PI D L)_outer/(PI D L)_inner
pub fn inner_cylinder_to_outer_cylinder_view_factor(
    inner_diameter: Length,
    outer_diameter: Length,
    cylinder_height: Length) -> Ratio {

    let view_factor_outer_to_inner: Ratio = 
        outer_cylinder_to_inner_cylinder_view_factor(
            inner_diameter, 
            outer_diameter, 
            cylinder_height);

    let outer_area_by_inner_area: Ratio = outer_diameter/inner_diameter;

    return view_factor_outer_to_inner * outer_area_by_inner_area;


}

/// inner surface cylinder to annular end 
///
///
/// F(1-2) = V + 1/(2 pi) * [W - X * Y - Z]
///
/// V = B/(8RL)
///
/// W = acos(A/B)
///
/// X = 1/(2L) sqrt( (A+2)^2/R^2 - 4)
///
/// Y = acos( A * R / B)
///
/// Z = A/(2 R L) * asin(R)
///
///
/// F(1-2) A1 = F(2-1) A2
///
///
pub fn inner_cylinder_to_annular_end_ring_view_factor(
    inner_diameter: Length,
    outer_diameter: Length,
    cylinder_height: Length) -> Ratio {

    // R = r_1/r_2
    let ratio_r: Ratio = inner_diameter/outer_diameter;
    let r_2: Length = 0.5 * outer_diameter;
    // H = cylinder_height/r_2 
    let ratio_l: Ratio = cylinder_height/r_2;

    let r_value: f64 = ratio_r.into();
    let l_value: f64 = ratio_l.into();

    // square and inverse values
    let r_sq = r_value.powf(2.0);
    let l_sq = l_value.powf(2.0);
    let one_over_l = l_value.recip();
    let one_over_r = r_value.recip();

    let a: f64 = l_sq + r_sq - 1.0;
    let b: f64 = l_sq - r_sq + 1.0;


    // V = B/(8RL)

    let v: f64 = b * 0.125 * one_over_r * one_over_l;
    //
    // W = acos(A/B)


    let w = (a/b).acos();

    // X = 1/(2L) sqrt( (A+2)^2/R^2 - 4)

    let x = 0.5 * one_over_l * ( (a + 2.0).powf(2.0)/r_sq - 4.0 ).sqrt();

    // Y = acos( A * R / B)

    let y = (a * r_value/b).acos();

    // Z = A/(2 R L) * asin(R)

    let z = (a * 0.5 * one_over_r * one_over_l) * (r_value.asin());

    let view_factor_value =  v + 1.0/(2.0 * PI) * (w - x * y - z);


    return Ratio::new::<ratio>(view_factor_value);

}


#[cfg(test)]
#[test]
pub fn cocentric_cylinders_view_factor_shld_equal_one_for_inner_cyl(){
    use uom::si::length::meter;


    let inner_diameter = Length::new::<meter>(1.0);
    let outer_diameter = Length::new::<meter>(2.0);

    let cylinder_height = Length::new::<meter>(5.0);



    let inner_to_outer_cyl_view_factor = 
        inner_cylinder_to_outer_cylinder_view_factor(
            inner_diameter, outer_diameter, cylinder_height);

    // this needs to be multiplied twice or added twice 
    // as there are two ends
    let inner_cyl_to_annular_end_ring_view_factor = 
        inner_cylinder_to_annular_end_ring_view_factor(
            inner_diameter, outer_diameter, cylinder_height);

    let total_view_factor = 
        inner_to_outer_cyl_view_factor 
        + inner_cyl_to_annular_end_ring_view_factor
        + inner_cyl_to_annular_end_ring_view_factor;

    approx::assert_relative_eq!(
        total_view_factor.get::<ratio>(),
        1.0,
        max_relative = 1e-5
        );

}
