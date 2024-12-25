

/// from 212.7 C to 279.6 C prandtl number of 
/// hitec salt should vary from 14.2 to 23.3
#[test]
pub fn prandtl_number_range_check(){

    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::si::{pressure::atmosphere, ratio::ratio};

    use crate::prelude::beta_testing::LiquidMaterial;

    let high_bound_temp: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(279.6);

    let low_bound_temp: ThermodynamicTemperature = 
        ThermodynamicTemperature::new::<degree_celsius>(212.7);
    let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);

    let hitec = LiquidMaterial::HITEC;

    let high_bound_temp_prandtl: Ratio = 
        hitec.try_get_prandtl_liquid(high_bound_temp, atmospheric_pressure)
        .unwrap();
    let low_bound_temp_prandtl: Ratio = 
        hitec.try_get_prandtl_liquid(low_bound_temp, atmospheric_pressure)
        .unwrap();

    approx::assert_relative_eq!(
        high_bound_temp_prandtl.get::<ratio>(),
        14.2,
        max_relative=0.01
        );

    approx::assert_relative_eq!(
        low_bound_temp_prandtl.get::<ratio>(),
        23.3,
        max_relative=0.01
        );
}


/// "Plots" prandtl number from 200C to 240C in 5C intervals
#[test]
pub fn prandtl_number_range_from_200_to_240_celsius(){

    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::si::{pressure::atmosphere, ratio::ratio};

    use crate::prelude::beta_testing::LiquidMaterial;


    // test function
    fn check_prandtl_at_temp(temp_celsius: f64,
        prandtl_num: f64){

        let temp: ThermodynamicTemperature = 
            ThermodynamicTemperature::new::<degree_celsius>(temp_celsius);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);
        let hitec = LiquidMaterial::HITEC;
        let calculated_prandtl: Ratio = 
            hitec.try_get_prandtl_liquid(temp, atmospheric_pressure)
            .unwrap();

            approx::assert_relative_eq!(
                calculated_prandtl.get::<ratio>(),
                prandtl_num,
                max_relative=0.01
            );

    }

    check_prandtl_at_temp(200.0, 26.24);
    check_prandtl_at_temp(205.0, 25.02);
    check_prandtl_at_temp(210.0, 23.88);
    check_prandtl_at_temp(215.0, 22.80);
    check_prandtl_at_temp(220.0, 21.78);
    check_prandtl_at_temp(225.0, 20.79);
    check_prandtl_at_temp(230.0, 19.97);
    check_prandtl_at_temp(235.0, 19.19);
    check_prandtl_at_temp(240.0, 18.46);

}


/// thermal conductivity of HITEC at 200C and 240C 
#[test]
pub fn hitec_thermal_cond_at_200_and_240_celsius(){

    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::si::thermal_conductivity::watt_per_meter_kelvin;
    use crate::prelude::beta_testing::LiquidMaterial;


    // test function
    fn check_conductivity_at_temp(temp_celsius: f64,
        thermal_cond_watt_per_m_k: f64){

        let temp: ThermodynamicTemperature = 
            ThermodynamicTemperature::new::<degree_celsius>(temp_celsius);
        let hitec = LiquidMaterial::HITEC;
        let calculated_conductivity: ThermalConductivity = 
            hitec.try_get_thermal_conductivity(temp)
            .unwrap();

            approx::assert_relative_eq!(
                calculated_conductivity.get::<watt_per_meter_kelvin>(),
                thermal_cond_watt_per_m_k,
                max_relative=0.01
            );

    }

    check_conductivity_at_temp(200.0, 0.43602);
    check_conductivity_at_temp(240.0, 0.42806);

}
/// thermal conductivity of YD325 at 75 and 110 
#[test]
pub fn yd325_thermal_cond_at_75_and_110_celsius(){

    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::si::thermal_conductivity::watt_per_meter_kelvin;
    use crate::prelude::beta_testing::LiquidMaterial;


    // test function
    fn check_conductivity_at_temp(temp_celsius: f64,
        thermal_cond_watt_per_m_k: f64){

        let temp: ThermodynamicTemperature = 
            ThermodynamicTemperature::new::<degree_celsius>(temp_celsius);
        let yd325 = LiquidMaterial::YD325;
        let calculated_conductivity: ThermalConductivity = 
            yd325.try_get_thermal_conductivity(temp)
            .unwrap();

            approx::assert_relative_eq!(
                calculated_conductivity.get::<watt_per_meter_kelvin>(),
                thermal_cond_watt_per_m_k,
                max_relative=0.01
            );

    }

    check_conductivity_at_temp(75.0, 0.1183);
    check_conductivity_at_temp(110.0, 0.1160);

}

/// checks the viscosity,reynolds, darcy friction factor, Prandtl num 
/// and Nusselt number for the yd325 filled tubes 
/// of Du's expt 
///
/// in the range 75 and 110 
///
/// in nusselt calculation, we provide no wall correction
#[test]
pub fn yd325_tube_reynolds_prandtl_nusselt(){

    use uom::si::f64::*;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::si::pressure::atmosphere;
    use std::f64::consts::PI;

    use uom::si::{ratio::ratio, volume_rate::cubic_meter_per_hour};
    use uom::si::length::meter;
    use uom::si::diffusion_coefficient::square_meter_per_second;
    use crate::prelude::beta_testing::LiquidMaterial;
    use crate::heat_transfer_correlations::nusselt_number_correlations::enums::NusseltCorrelation;
    use crate::{heat_transfer_correlations::nusselt_number_correlations::input_structs::GnielinskiData, prelude::beta_testing::SolidMaterial};
    use crate::fluid_mechanics_correlations::darcy;

    fn test(temperature_val_celsius: f64,
        kinematic_viscosity_val_sq_m_per_s: f64,
        reynolds_num: f64,
        darcy_friction_factor: f64,
        prandtl_num: f64,
        nusselt_num: f64,){

        let temp = ThermodynamicTemperature::new::<degree_celsius>(
            temperature_val_celsius);
        let atmospheric_pressure = Pressure::new::<atmosphere>(1.0);


        let four_over_pi: Ratio = Ratio::new::<ratio>(4.0/PI);
        let vol_flowrate = VolumeRate::new::<cubic_meter_per_hour>(15.635);
        let d_i = Length::new::<meter>(0.01);
        let number_of_tubes: f64 = 19.0;
        let pipe_length = Length::new::<meter>(1.95);

        let viscosity_scale: DiffusionCoefficient = 
            four_over_pi * vol_flowrate / (number_of_tubes * d_i);

        let nu: DiffusionCoefficient = LiquidMaterial::YD325 
            .try_get_nu_momentum_diffusivity(
                temp, atmospheric_pressure).unwrap();

        let prandtl_calculated: Ratio = 
            LiquidMaterial::YD325
            .try_get_prandtl_liquid(temp, atmospheric_pressure).unwrap();

        let reynolds_calculated: Ratio = viscosity_scale/nu;

        let steel_surface_roughness: Length = 
            SolidMaterial::SteelSS304L.surface_roughness().unwrap();

        let roughness_ratio: Ratio = steel_surface_roughness/d_i;

        

        let darcy_friction_factor_calculated: f64 
            = darcy(reynolds_calculated.get::<ratio>(), 
                roughness_ratio.get::<ratio>()
                ).unwrap();

        dbg!(&(roughness_ratio,
                (pipe_length/d_i),
                darcy_friction_factor_calculated
                )
            );
        
        let tube_side_gnielinski_data: GnielinskiData = 
            GnielinskiData {
                reynolds: reynolds_calculated,
                prandtl_bulk: prandtl_calculated,
                prandtl_wall: prandtl_calculated,
                darcy_friction_factor: darcy_friction_factor_calculated.into(),
                length_to_diameter: (pipe_length/d_i),
            };

        let tube_side_nusselt_correlation = 
            NusseltCorrelation::PipeGnielinskiGeneric(
                tube_side_gnielinski_data);

        let nusselt_calculated: Ratio = 
            tube_side_nusselt_correlation.try_get_nusselt().unwrap();


        dbg!(
            &(
                temp,
                nu,
                reynolds_calculated,
                darcy_friction_factor_calculated,
                prandtl_calculated,
                nusselt_calculated
            )
        );

        approx::assert_relative_eq!(
            temperature_val_celsius,
            temp.get::<degree_celsius>(),
            max_relative=0.01
            );

        approx::assert_relative_eq!(
            kinematic_viscosity_val_sq_m_per_s,
            nu.get::<square_meter_per_second>(),
            max_relative=0.01
            );

        approx::assert_relative_eq!(
            reynolds_num,
            reynolds_calculated.get::<ratio>(),
            max_relative=0.01
            );

        approx::assert_relative_eq!(
            darcy_friction_factor,
            darcy_friction_factor_calculated,
            max_relative=0.01
            );

        approx::assert_relative_eq!(
            prandtl_num,
            prandtl_calculated.get::<ratio>(),
            max_relative=0.01
            );

        approx::assert_relative_eq!(
            nusselt_num,
            nusselt_calculated.get::<ratio>(),
            max_relative=0.01
            );

    }


    //  
    // the data is in this order:
    //  temperature_val_celsius: f64,
    //  kinematic_viscosity_val_sq_m_per_s: f64,
    //  reynolds_num: f64,
    //  darcy_friction_factor: f64,
    //  prandtl_num: f64,
    //  nusselt_num: f64,
    test(75.0, 3.644e-6, 7985.0, 0.03929, 59.10, 153.38);
    test(80.0, 3.227e-6, 9017.0, 0.03845, 52.77, 167.45);
    test(85.0, 2.857e-6, 10186.0, 0.03768, 47.10, 182.5);
    test(90.0, 2.531e-6, 11498.0, 0.03696, 42.07, 198.6);
    test(95.0, 2.246e-6, 12953.0, 0.03632, 37.64, 215.6);
    test(100.0, 2.00e-6, 14542.6, 0.03574, 33.79, 233.3);
    test(105.0, 1.79e-6, 16243.3, 0.03522, 30.49, 251.4);
    test(110.0, 1.61e-6, 18016.0, 0.03478, 27.71, 269.5);
}
