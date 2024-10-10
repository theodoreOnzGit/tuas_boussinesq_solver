
/// 
/// As mentioned by Du:
/// 1/U = 1/h_t d_o/d_i + d_o/(2 lambda_w) ln (d_o/d_i) 
/// + 1/h_s
///
/// I want to check if my tube nusselt number is correct 
/// so I'll need to obtain the heat trf coeff first
///
/// this checks the centre term: 
/// d_o/(2 lambda_w) ln (d_o/d_i) 
/// I calculated this in Libreoffice Calc to be 1/6920 
/// = 0.00014449 (m^2 K)/W
#[test]
pub fn check_wall_side_htc(){
    use uom::si::f64::*;
    use uom::si::ratio::ratio;

    use uom::si::{heat_transfer::watt_per_square_meter_kelvin, thermal_conductivity::watt_per_meter_kelvin};
    use uom::si::length::meter;

    let lambda_wall_du_paper: ThermalConductivity = 
        ThermalConductivity::new::<watt_per_meter_kelvin>(
            16.3);
    // from Du's heat exchanger type, except we use one inner tube
    let tube_side_od = Length::new::<meter>(0.014);
    let tube_side_id = Length::new::<meter>(0.01);
    let reciprocal_tube_side_solid_term = 
        tube_side_od/(2.0 as f64 * lambda_wall_du_paper) 
        * (tube_side_od/tube_side_id).get::<ratio>().ln();
    
    let wall_heat_transfer_expected = 
        HeatTransfer::new::<watt_per_square_meter_kelvin>(6920.0);


    approx::assert_relative_eq!(
        reciprocal_tube_side_solid_term.recip().get::<watt_per_square_meter_kelvin>(),
        wall_heat_transfer_expected.get::<watt_per_square_meter_kelvin>(),
        max_relative = 0.01
        );

}

/// high bound and low bound heat transfer coeff on shell side 
/// are 1036 - 1638 W/(m2 K)
#[test]
pub fn check_high_bound_and_low_bound_shell_heat_trf_coeff(){
    use uom::si::f64::*;
    use uom::si::ratio::ratio;

    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::si::length::meter;
    use uom::si::heat_transfer::watt_per_square_meter_kelvin;

    use crate::prelude::beta_testing::LiquidMaterial;

    let high_bound_nusselt_shell: Ratio = Ratio::new::<ratio>(65.95);
    let low_bound_nusselt_shell: Ratio = Ratio::new::<ratio>(42.5);

    let hydraulic_diameter_shell: Length = Length::new::<meter>(0.01755);

    let low_bound_temp: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(200.0);
    let high_bound_temp: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(240.0);

    let high_bound_thermal_cond: ThermalConductivity = 
        LiquidMaterial::HITEC.try_get_thermal_conductivity(
            low_bound_temp
        ).unwrap();

    let low_bound_thermal_cond: ThermalConductivity = 
        LiquidMaterial::HITEC.try_get_thermal_conductivity(
            high_bound_temp
        ).unwrap();

    let high_bound_htc: HeatTransfer = 
        high_bound_nusselt_shell * high_bound_thermal_cond 
        / hydraulic_diameter_shell;
    let low_bound_htc: HeatTransfer = 
        low_bound_nusselt_shell * low_bound_thermal_cond 
        / hydraulic_diameter_shell;

    approx::assert_relative_eq!(
        high_bound_htc.get::<watt_per_square_meter_kelvin>(),
        1638.0,
        max_relative = 0.03
        );

    approx::assert_relative_eq!(
        low_bound_htc.get::<watt_per_square_meter_kelvin>(),
        1036.0,
        max_relative = 0.03
        );

}
