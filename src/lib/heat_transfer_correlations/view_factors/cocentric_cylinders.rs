use uom::si::f64::*;

/// F_(2-2) =  1 - 1/R - ((H^2 + 4 R^2)^0.5 -H)/(4 R) + 1/PI * B
///
///
/// Where B = 2/R atan (2 * sqrt(R^2 - 1)/H) - H/(2 R) * C 
///
/// And C = 
pub fn outer_cylinder_self_view_factor(
    inner_diameter: Length,
    outer_diameter: Length,
    cylinder_height: Length
    ){

    let ratio_r: Ratio = outer_diameter/inner_diameter;
    let r_1 = 0.5 * inner_diameter;
    let ratio_h: Ratio = cylinder_height/r_1;




}
