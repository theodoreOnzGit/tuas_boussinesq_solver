
/// tests the implementation of a custom fluid component trait 
///
/// checks if getting pressure loss from a specified mass flowrate works 
/// okay
///
/// regression test 
#[test]
pub fn test_custom_fluid_component_pressure_loss() -> Result<(),
crate::tuas_lib_error::TuasLibError>{
    // this tests the calc pressure loss for fluid component 
    use uom::si::f64::*;
    use uom::ConstZero;
    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidCustomComponentCalcPressureLoss;
    use uom::si::length::inch;
    use std::f64::consts::PI;
    use uom::si::ratio::ratio;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::mass_density::kilogram_per_cubic_meter;
    use uom::si::dynamic_viscosity::poise;
    use uom::si::pressure::pascal;

    // this is the test component
    pub struct TestComponent {
        pub loss_correlation: DimensionlessDarcyLossCorrelations
    }

    impl FluidCustomComponentCalcPressureLoss for TestComponent {
        fn get_custom_loss_correlations(&mut self) ->
            crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations {
                self.loss_correlation.clone()
        }

        fn get_custom_loss_correlations_immutable(&self) ->
            crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations {
                self.loss_correlation.clone()
        }

        fn set_custom_loss_correlations(
            &mut self,
            custom_loss_correlation: crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations) {
            self.loss_correlation = custom_loss_correlation;
        }

        fn get_custom_component_absolute_roughness(
            &mut self) -> Length {
            Length::ZERO
        }

        fn get_custom_component_absolute_roughness_immutable(
            &self) -> Length {
            Length::ZERO
        }
    }

    // let's have a test mass flowrate 

    let fluid_mass_flowrate = MassRate::new::<kilogram_per_second>(0.2);
    let hydraulic_diameter = Length::new::<inch>(1.0);
    let cross_sectional_area = hydraulic_diameter*hydraulic_diameter*PI/4.0_f64;
    let fluid_viscosity = DynamicViscosity::new::<poise>(0.01);
    let fluid_density = MassDensity::new::<kilogram_per_cubic_meter>(1000.0);

    let a = Ratio::new::<ratio>(18.0);
    let b = Ratio::new::<ratio>(93000_f64);
    let c: f64 = -1.35;

    let loss_correlation = DimensionlessDarcyLossCorrelations::new_simple_reynolds_power_component(
        a, b, c);
    
    // create object 
    let test_component_object = TestComponent{ loss_correlation };
    // calculate a pressure loss

    {
        // forward test 

        let pressure_loss: Pressure = TestComponent::fluid_custom_component_calc_pressure_loss(
            fluid_mass_flowrate, 
            cross_sectional_area, 
            hydraulic_diameter, 
            fluid_viscosity, 
            fluid_density, 
            test_component_object.loss_correlation).unwrap();

        // expected pressure loss is 1430 pascals
        approx::assert_relative_eq!(
            1430.0,
            pressure_loss.get::<pascal>(),
            max_relative=0.01);
    }

    {
        // reverse test 

        let pressure_loss: Pressure = TestComponent::fluid_custom_component_calc_pressure_loss(
            -fluid_mass_flowrate, 
            cross_sectional_area, 
            hydraulic_diameter, 
            fluid_viscosity, 
            fluid_density, 
            test_component_object.loss_correlation).unwrap();

        // expected pressure loss is -1430 pascals
        // (other direction)
        approx::assert_relative_eq!(
            -1430.0,
            pressure_loss.get::<pascal>(),
            max_relative=0.01);
    }


    Ok(())
}

/// tests the implementation of a custom fluid component trait 
///
/// checks if getting mass flowrate from pressure loss 
/// okay
///
/// regression test 
#[test]
pub fn test_custom_fluid_component_mass_flow_from_pressure_loss() -> Result<(),
crate::tuas_lib_error::TuasLibError>{
    // this tests the calc pressure loss for fluid component 
    use uom::si::f64::*;
    use uom::ConstZero;
    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidCustomComponentCalcPressureLoss;
    use uom::si::length::inch;
    use std::f64::consts::PI;
    use uom::si::ratio::ratio;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::mass_density::kilogram_per_cubic_meter;
    use uom::si::dynamic_viscosity::poise;
    use uom::si::pressure::pascal;

    // this is the test component
    pub struct TestComponent {
        pub loss_correlation: DimensionlessDarcyLossCorrelations
    }

    impl FluidCustomComponentCalcPressureLoss for TestComponent {
        fn get_custom_loss_correlations(&mut self) ->
            crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations {
                self.loss_correlation.clone()
        }

        fn get_custom_loss_correlations_immutable(&self) ->
            crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations {
                self.loss_correlation.clone()
        }

        fn set_custom_loss_correlations(
            &mut self,
            custom_loss_correlation: crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations) {
            self.loss_correlation = custom_loss_correlation;
        }

        fn get_custom_component_absolute_roughness(
            &mut self) -> Length {
            Length::ZERO
        }

        fn get_custom_component_absolute_roughness_immutable(
            &self) -> Length {
            Length::ZERO
        }
    }

    // let's have a test mass flowrate 

    let expected_fluid_mass_flowrate = MassRate::new::<kilogram_per_second>(0.2);
    let input_pressure_loss = Pressure::new::<pascal>(1430.0);
    let hydraulic_diameter = Length::new::<inch>(1.0);
    let cross_sectional_area = hydraulic_diameter*hydraulic_diameter*PI/4.0_f64;
    let fluid_viscosity = DynamicViscosity::new::<poise>(0.01);
    let fluid_density = MassDensity::new::<kilogram_per_cubic_meter>(1000.0);

    let a = Ratio::new::<ratio>(18.0);
    let b = Ratio::new::<ratio>(93000_f64);
    let c: f64 = -1.35;

    let loss_correlation = DimensionlessDarcyLossCorrelations::new_simple_reynolds_power_component(
        a, b, c);
    
    // create object 
    let test_component_object = TestComponent{ loss_correlation };
    // calculate a pressure loss

    {
        // forward test 

        let mass_flowrate = TestComponent::fluid_custom_component_calc_mass_flowrate_from_pressure_loss(
            input_pressure_loss, 
            cross_sectional_area, 
            hydraulic_diameter, 
            fluid_viscosity, 
            fluid_density, 
            test_component_object.loss_correlation).unwrap();

        // expected mass flowrate is 0.2 kg/s
        approx::assert_relative_eq!(
            expected_fluid_mass_flowrate.get::<kilogram_per_second>(),
            mass_flowrate.get::<kilogram_per_second>(),
            max_relative=0.01);
    }

    {
        // reverse test

        let mass_flowrate = TestComponent::fluid_custom_component_calc_mass_flowrate_from_pressure_loss(
            -input_pressure_loss, 
            cross_sectional_area, 
            hydraulic_diameter, 
            fluid_viscosity, 
            fluid_density, 
            test_component_object.loss_correlation).unwrap();

        // expected mass flowrate is 0.2 kg/s
        approx::assert_relative_eq!(
            -expected_fluid_mass_flowrate.get::<kilogram_per_second>(),
            mass_flowrate.get::<kilogram_per_second>(),
            max_relative=0.01);
    }


    Ok(())
}

/// tests the implementation of a custom fluid component trait 
///
/// checks if getting pressure change from mass flowrate is okay
/// okay
///
/// regression test 
#[test]
pub fn test_custom_fluid_component_pressure_change() -> Result<(),
crate::tuas_lib_error::TuasLibError>{
    // this tests the calc pressure loss for fluid component 
    use uom::si::f64::*;
    use uom::ConstZero;
    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidCustomComponentCalcPressureLoss;
    use uom::si::length::inch;
    use std::f64::consts::PI;
    use uom::si::ratio::ratio;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::mass_density::kilogram_per_cubic_meter;
    use uom::si::dynamic_viscosity::poise;
    use uom::si::pressure::pascal;
    use uom::si::length::meter;
    use uom::si::angle::degree;

    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidCustomComponentCalcPressureChange;

    // this is the test component
    pub struct TestComponent {
        pub loss_correlation: DimensionlessDarcyLossCorrelations,
        pub mass_flowrate: MassRate,
        pub pressure_loss: Pressure,
        pub pressure_source: Pressure,
    }

    impl FluidCustomComponentCalcPressureLoss for TestComponent {
        fn get_custom_loss_correlations(&mut self) ->
            crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations {
                self.loss_correlation.clone()
        }

        fn get_custom_loss_correlations_immutable(&self) ->
            crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations {
                self.loss_correlation.clone()
        }

        fn set_custom_loss_correlations(
            &mut self,
            custom_loss_correlation: crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations) {
            self.loss_correlation = custom_loss_correlation;
        }

        fn get_custom_component_absolute_roughness(
            &mut self) -> Length {
            Length::ZERO
        }

        fn get_custom_component_absolute_roughness_immutable(
            &self) -> Length {
            Length::ZERO
        }
    }

    impl FluidComponentTrait for TestComponent {
        fn get_mass_flowrate(&mut self) -> MassRate  {
            self.mass_flowrate
        }

        fn set_mass_flowrate(&mut self, mass_flowrate: MassRate) {
            self.mass_flowrate = mass_flowrate;
        }

        fn get_mass_flowrate_from_pressure_loss_immutable(
            &self, pressure_loss: Pressure) -> MassRate {
            let mass_flowrate = TestComponent::fluid_custom_component_calc_mass_flowrate_from_pressure_loss(
                pressure_loss, 
                self.get_cross_sectional_area_immutable(), 
                self.get_hydraulic_diameter_immutable(), 
                self.get_fluid_viscosity_immutable_at_ref_temperature(), 
                self.get_fluid_density_immutable_at_ref_temperature(), 
                self.loss_correlation).unwrap();
            mass_flowrate
        }

        fn get_pressure_loss(&mut self) -> Pressure {
            let mass_flowrate = self.mass_flowrate;
            self.get_pressure_loss_immutable(mass_flowrate)
        }

        fn set_pressure_loss(&mut self, pressure_loss: Pressure) {
            self.pressure_loss = pressure_loss;
        }

        fn get_pressure_loss_immutable(
            &self, mass_flowrate: MassRate) -> Pressure {
            let pressure_loss: Pressure = TestComponent::fluid_custom_component_calc_pressure_loss(
                mass_flowrate, 
                self.get_cross_sectional_area_immutable(), 
                self.get_hydraulic_diameter_immutable(), 
                self.get_fluid_viscosity_immutable_at_ref_temperature(), 
                self.get_fluid_density_immutable_at_ref_temperature(), 
                self.loss_correlation).unwrap();
            pressure_loss
        }

        fn get_cross_sectional_area(&mut self) -> Area {
            let hydraulic_diameter = Length::new::<inch>(1.0);
            let cross_sectional_area = hydraulic_diameter*hydraulic_diameter*PI/4.0_f64;
            cross_sectional_area
        }

        fn get_cross_sectional_area_immutable(&self) -> Area {
            let hydraulic_diameter = Length::new::<inch>(1.0);
            let cross_sectional_area = hydraulic_diameter*hydraulic_diameter*PI/4.0_f64;
            cross_sectional_area
        }

        fn get_hydraulic_diameter(&mut self) -> Length {
            let hydraulic_diameter = Length::new::<inch>(1.0);
            hydraulic_diameter
        }

        fn get_hydraulic_diameter_immutable(&self) -> Length {
            let hydraulic_diameter = Length::new::<inch>(1.0);
            hydraulic_diameter
        }

        fn get_fluid_viscosity_at_ref_temperature(&mut self) -> DynamicViscosity {
            let fluid_viscosity = DynamicViscosity::new::<poise>(0.01);
            fluid_viscosity
        }

        fn get_fluid_viscosity_immutable_at_ref_temperature(&self) -> DynamicViscosity {
            let fluid_viscosity = DynamicViscosity::new::<poise>(0.01);
            fluid_viscosity
        }

        fn get_fluid_density_at_ref_temperature(&mut self) -> MassDensity {
            let fluid_density = MassDensity::new::<kilogram_per_cubic_meter>(1000.0);
            fluid_density
        }

        fn get_fluid_density_immutable_at_ref_temperature(&self) -> MassDensity {
            let fluid_density = MassDensity::new::<kilogram_per_cubic_meter>(1000.0);
            fluid_density
        }

        fn get_component_length(&mut self) -> Length {
            let component_length = Length::new::<meter>(0.5);
            component_length
        }

        fn get_component_length_immutable(&self) -> Length {
            let component_length = Length::new::<meter>(0.5);
            component_length
        }

        fn get_incline_angle(&mut self) -> Angle {
            let incline_angle = 
                Angle::new::<degree>(90.0);
            incline_angle
        }

        fn get_incline_angle_immutable(&self) -> Angle {
            let incline_angle = 
                Angle::new::<degree>(90.0);
            incline_angle
        }

        fn get_internal_pressure_source(&mut self) -> Pressure {
            self.pressure_source
        }

        fn get_internal_pressure_source_immutable(&self) -> Pressure {
            self.pressure_source
        }

        fn set_internal_pressure_source(
            &mut self,
            internal_pressure: Pressure) {
            self.pressure_source = internal_pressure;
        }
    }

    impl FluidCustomComponentCalcPressureChange for TestComponent {}

    // let's have a test mass flowrate 

    let fluid_mass_flowrate = MassRate::new::<kilogram_per_second>(0.2);
    let hydraulic_diameter = Length::new::<inch>(1.0);
    let cross_sectional_area = hydraulic_diameter*hydraulic_diameter*PI/4.0_f64;
    let fluid_viscosity = DynamicViscosity::new::<poise>(0.01);
    let fluid_density = MassDensity::new::<kilogram_per_cubic_meter>(1000.0);

    let a = Ratio::new::<ratio>(18.0);
    let b = Ratio::new::<ratio>(93000_f64);
    let c: f64 = -1.35;

    let loss_correlation = DimensionlessDarcyLossCorrelations::new_simple_reynolds_power_component(
        a, b, c);
    
    // create object 
    let test_component_object = TestComponent{ 
        loss_correlation,
        mass_flowrate: MassRate::ZERO,
        pressure_loss: Pressure::ZERO,
        pressure_source: Pressure::ZERO,
    };

    // calculate a pressure loss first, then pressure change
    {
        // forward test 

        let pressure_loss: Pressure = test_component_object.
            get_pressure_loss_immutable(fluid_mass_flowrate);

        // expected pressure loss is 1430 pascals
        approx::assert_relative_eq!(
            1430.0,
            pressure_loss.get::<pascal>(),
            max_relative=0.01);
    }

    {
        // reverse test 

        let pressure_loss: Pressure = test_component_object.
            get_pressure_loss_immutable(-fluid_mass_flowrate);

        // expected pressure loss is -1430 pascals
        // (other direction)
        approx::assert_relative_eq!(
            -1430.0,
            pressure_loss.get::<pascal>(),
            max_relative=0.01);
    }


    // now pressure change
    {
        // forward test 

        let pressure_change: Pressure = 
            TestComponent::fluid_custom_component_calc_pressure_change(
                fluid_mass_flowrate, 
                cross_sectional_area, 
                hydraulic_diameter, 
                fluid_viscosity, 
                fluid_density, 
                test_component_object.get_component_length_immutable(), 
                test_component_object.get_incline_angle_immutable(), 
                test_component_object.get_internal_pressure_source_immutable(), 
                loss_correlation)?;

        // expected pressure change is -6335 pascals
        approx::assert_relative_eq!(
            -6335.0,
            pressure_change.get::<pascal>(),
            max_relative=0.01);
    }

    {
        // reverse test 

        let pressure_change: Pressure = 
            TestComponent::fluid_custom_component_calc_pressure_change(
                -fluid_mass_flowrate, 
                cross_sectional_area, 
                hydraulic_diameter, 
                fluid_viscosity, 
                fluid_density, 
                test_component_object.get_component_length_immutable(), 
                test_component_object.get_incline_angle_immutable(), 
                test_component_object.get_internal_pressure_source_immutable(), 
                loss_correlation)?;

        // expected pressure change is -3474 pascals
        // not 6335 pascals
        // because of 90 degree elevation
        // (other direction)
        approx::assert_relative_eq!(
            -3474.0,
            pressure_change.get::<pascal>(),
            max_relative=0.01);
    }

    Ok(())
}

/// tests the implementation of a custom fluid component trait 
///
/// checks if getting mass flowrate from pressure change is okay
/// okay
///
/// regression test 
#[test]
pub fn test_custom_fluid_component_mass_flowrate_from_pressure_change() -> Result<(),
crate::tuas_lib_error::TuasLibError>{
    // this tests the calc pressure loss for fluid component 
    use uom::si::f64::*;
    use uom::ConstZero;
    use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidCustomComponentCalcPressureLoss;
    use uom::si::length::inch;
    use std::f64::consts::PI;
    use uom::si::ratio::ratio;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::mass_density::kilogram_per_cubic_meter;
    use uom::si::dynamic_viscosity::poise;
    use uom::si::pressure::pascal;
    use uom::si::length::meter;
    use uom::si::angle::degree;

    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidCustomComponentCalcPressureChange;

    // this is the test component
    pub struct TestComponent {
        pub loss_correlation: DimensionlessDarcyLossCorrelations,
        pub mass_flowrate: MassRate,
        pub pressure_loss: Pressure,
        pub pressure_source: Pressure,
    }

    impl FluidCustomComponentCalcPressureLoss for TestComponent {
        fn get_custom_loss_correlations(&mut self) ->
            DimensionlessDarcyLossCorrelations {
                self.loss_correlation.clone()
        }

        fn get_custom_loss_correlations_immutable(&self) ->
            DimensionlessDarcyLossCorrelations {
                self.loss_correlation.clone()
        }

        fn set_custom_loss_correlations(
            &mut self,
            custom_loss_correlation: DimensionlessDarcyLossCorrelations) {
            self.loss_correlation = custom_loss_correlation;
        }

        fn get_custom_component_absolute_roughness(
            &mut self) -> Length {
            Length::ZERO
        }

        fn get_custom_component_absolute_roughness_immutable(
            &self) -> Length {
            Length::ZERO
        }
    }

    impl FluidComponentTrait for TestComponent {
        fn get_mass_flowrate(&mut self) -> MassRate  {
            self.mass_flowrate
        }

        fn set_mass_flowrate(&mut self, mass_flowrate: MassRate) {
            self.mass_flowrate = mass_flowrate;
        }

        fn get_mass_flowrate_from_pressure_loss_immutable(
            &self, pressure_loss: Pressure) -> MassRate {
            let mass_flowrate = TestComponent::fluid_custom_component_calc_mass_flowrate_from_pressure_loss(
                pressure_loss, 
                self.get_cross_sectional_area_immutable(), 
                self.get_hydraulic_diameter_immutable(), 
                self.get_fluid_viscosity_immutable_at_ref_temperature(), 
                self.get_fluid_density_immutable_at_ref_temperature(), 
                self.loss_correlation).unwrap();
            mass_flowrate
        }

        fn get_pressure_loss(&mut self) -> Pressure {
            let mass_flowrate = self.mass_flowrate;
            self.get_pressure_loss_immutable(mass_flowrate)
        }

        fn set_pressure_loss(&mut self, pressure_loss: Pressure) {
            self.pressure_loss = pressure_loss;
        }

        fn get_pressure_loss_immutable(
            &self, mass_flowrate: MassRate) -> Pressure {
            let pressure_loss: Pressure = TestComponent::fluid_custom_component_calc_pressure_loss(
                mass_flowrate, 
                self.get_cross_sectional_area_immutable(), 
                self.get_hydraulic_diameter_immutable(), 
                self.get_fluid_viscosity_immutable_at_ref_temperature(), 
                self.get_fluid_density_immutable_at_ref_temperature(), 
                self.loss_correlation).unwrap();
            pressure_loss
        }

        fn get_cross_sectional_area(&mut self) -> Area {
            let hydraulic_diameter = Length::new::<inch>(1.0);
            let cross_sectional_area = hydraulic_diameter*hydraulic_diameter*PI/4.0_f64;
            cross_sectional_area
        }

        fn get_cross_sectional_area_immutable(&self) -> Area {
            let hydraulic_diameter = Length::new::<inch>(1.0);
            let cross_sectional_area = hydraulic_diameter*hydraulic_diameter*PI/4.0_f64;
            cross_sectional_area
        }

        fn get_hydraulic_diameter(&mut self) -> Length {
            let hydraulic_diameter = Length::new::<inch>(1.0);
            hydraulic_diameter
        }

        fn get_hydraulic_diameter_immutable(&self) -> Length {
            let hydraulic_diameter = Length::new::<inch>(1.0);
            hydraulic_diameter
        }

        fn get_fluid_viscosity_at_ref_temperature(&mut self) -> DynamicViscosity {
            let fluid_viscosity = DynamicViscosity::new::<poise>(0.01);
            fluid_viscosity
        }

        fn get_fluid_viscosity_immutable_at_ref_temperature(&self) -> DynamicViscosity {
            let fluid_viscosity = DynamicViscosity::new::<poise>(0.01);
            fluid_viscosity
        }

        fn get_fluid_density_at_ref_temperature(&mut self) -> MassDensity {
            let fluid_density = MassDensity::new::<kilogram_per_cubic_meter>(1000.0);
            fluid_density
        }

        fn get_fluid_density_immutable_at_ref_temperature(&self) -> MassDensity {
            let fluid_density = MassDensity::new::<kilogram_per_cubic_meter>(1000.0);
            fluid_density
        }

        fn get_component_length(&mut self) -> Length {
            let component_length = Length::new::<meter>(0.5);
            component_length
        }

        fn get_component_length_immutable(&self) -> Length {
            let component_length = Length::new::<meter>(0.5);
            component_length
        }

        fn get_incline_angle(&mut self) -> Angle {
            let incline_angle = 
                Angle::new::<degree>(90.0);
            incline_angle
        }

        fn get_incline_angle_immutable(&self) -> Angle {
            let incline_angle = 
                Angle::new::<degree>(90.0);
            incline_angle
        }

        fn get_internal_pressure_source(&mut self) -> Pressure {
            self.pressure_source
        }

        fn get_internal_pressure_source_immutable(&self) -> Pressure {
            self.pressure_source
        }

        fn set_internal_pressure_source(
            &mut self,
            internal_pressure: Pressure) {
            self.pressure_source = internal_pressure;
        }
    }

    impl FluidCustomComponentCalcPressureChange for TestComponent {}

    // let's have a test mass flowrate 

    let fluid_mass_flowrate_expected = MassRate::new::<kilogram_per_second>(0.2);
    let hydraulic_diameter = Length::new::<inch>(1.0);
    let cross_sectional_area = hydraulic_diameter*hydraulic_diameter*PI/4.0_f64;
    let fluid_viscosity = DynamicViscosity::new::<poise>(0.01);
    let fluid_density = MassDensity::new::<kilogram_per_cubic_meter>(1000.0);

    let a = Ratio::new::<ratio>(18.0);
    let b = Ratio::new::<ratio>(93000_f64);
    let c: f64 = -1.35;

    let loss_correlation = DimensionlessDarcyLossCorrelations::new_simple_reynolds_power_component(
        a, b, c);
    
    // create object 
    let test_component_object = TestComponent{ 
        loss_correlation,
        mass_flowrate: MassRate::ZERO,
        pressure_loss: Pressure::ZERO,
        pressure_source: Pressure::ZERO,
    };

    // now pressure change
    {
        // forward test 
        
        let input_pressure_change = Pressure::new::<pascal>(-6335.0);

        let mass_flowrate_test: MassRate = 
            TestComponent::fluid_custom_component_calc_mass_flowrate_from_pressure_change(
                input_pressure_change, 
                cross_sectional_area, 
                hydraulic_diameter, 
                fluid_viscosity, 
                fluid_density, 
                test_component_object.get_component_length_immutable(), 
                test_component_object.get_incline_angle_immutable(), 
                test_component_object.get_internal_pressure_source_immutable(), 
                loss_correlation)?;

        // expected mass flowrate is 0.2 kg/s (positive)
        approx::assert_relative_eq!(
            mass_flowrate_test.get::<kilogram_per_second>(),
            fluid_mass_flowrate_expected.get::<kilogram_per_second>(),
            max_relative=0.01);
    }

    {
        // reverse test 

        let input_pressure_change = Pressure::new::<pascal>(-3474.0);

        let mass_flowrate_test: MassRate = 
            TestComponent::fluid_custom_component_calc_mass_flowrate_from_pressure_change(
                input_pressure_change, 
                cross_sectional_area, 
                hydraulic_diameter, 
                fluid_viscosity, 
                fluid_density, 
                test_component_object.get_component_length_immutable(), 
                test_component_object.get_incline_angle_immutable(), 
                test_component_object.get_internal_pressure_source_immutable(), 
                loss_correlation)?;

        // expected mass flowrate is -0.2 kg/s (other direction)
        approx::assert_relative_eq!(
            mass_flowrate_test.get::<kilogram_per_second>(),
            -fluid_mass_flowrate_expected.get::<kilogram_per_second>(),
            max_relative=0.01);
    }

    Ok(())
}
