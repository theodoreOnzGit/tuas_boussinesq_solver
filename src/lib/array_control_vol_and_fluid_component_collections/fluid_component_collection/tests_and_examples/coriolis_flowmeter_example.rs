/// Example 3,
/// 
/// suppose now we have a coriolis flowmeter
/// with a custom friction factor correlation
///
/// (f_darcy L/D + K) = 18 + 93000/Re^1.35
///
/// we shall use water to push flow through this coriolis flowmeter
///
#[test]
pub fn coriolis_flowmeter_empirical_custom_component_example_3() -> Result<(),
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
    use uom::si::length::millimeter;

    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidCustomComponentCalcPressureChange;

    // this is the test component
    pub struct CoriolisFlowmeter {
        pub loss_correlation: DimensionlessDarcyLossCorrelations,
        pub mass_flowrate: MassRate,
        pub pressure_loss: Pressure,
        pub pressure_source: Pressure,
        pub hydraulic_diameter: Length,
        pub incline_angle: Angle,
        pub component_length: Length,
        pub fluid_density: MassDensity,
        pub fluid_viscosity: DynamicViscosity,
        pub absolute_roughness: Length,
    }

    impl FluidCustomComponentCalcPressureLoss for CoriolisFlowmeter {
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

    impl FluidComponentTrait for CoriolisFlowmeter {
        fn get_mass_flowrate(&mut self) -> MassRate  {
            self.mass_flowrate
        }

        fn set_mass_flowrate(&mut self, mass_flowrate: MassRate) {
            self.mass_flowrate = mass_flowrate;

            // setting mass flowrate should result in new pressure loss
            let pressure_loss = self.get_pressure_loss_immutable(mass_flowrate);
            self.pressure_loss = pressure_loss;
        }

        fn get_mass_flowrate_from_pressure_loss_immutable(
            &self, pressure_loss: Pressure) -> MassRate {
            let mass_flowrate = CoriolisFlowmeter::fluid_custom_component_calc_mass_flowrate_from_pressure_loss(
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
            // setting pressure loss should result in new mass flowrate
            let mass_flowrate = CoriolisFlowmeter::fluid_custom_component_calc_mass_flowrate_from_pressure_loss(
                pressure_loss, 
                self.get_cross_sectional_area_immutable(), 
                self.get_hydraulic_diameter_immutable(), 
                self.get_fluid_viscosity_immutable_at_ref_temperature(), 
                self.get_fluid_density_immutable_at_ref_temperature(), 
                self.loss_correlation).unwrap();
            self.mass_flowrate = mass_flowrate;
        }

        fn get_pressure_loss_immutable(
            &self, mass_flowrate: MassRate) -> Pressure {
            let pressure_loss: Pressure = CoriolisFlowmeter::fluid_custom_component_calc_pressure_loss(
                mass_flowrate, 
                self.get_cross_sectional_area_immutable(), 
                self.get_hydraulic_diameter_immutable(), 
                self.get_fluid_viscosity_immutable_at_ref_temperature(), 
                self.get_fluid_density_immutable_at_ref_temperature(), 
                self.loss_correlation).unwrap();
            pressure_loss
        }

        fn get_cross_sectional_area(&mut self) -> Area {
            let hydraulic_diameter = self.hydraulic_diameter;
            let cross_sectional_area = hydraulic_diameter*hydraulic_diameter*PI/4.0_f64;
            cross_sectional_area
        }

        fn get_cross_sectional_area_immutable(&self) -> Area {
            let hydraulic_diameter = self.hydraulic_diameter;
            let cross_sectional_area = hydraulic_diameter*hydraulic_diameter*PI/4.0_f64;
            cross_sectional_area
        }

        fn get_hydraulic_diameter(&mut self) -> Length {
            self.hydraulic_diameter
        }

        fn get_hydraulic_diameter_immutable(&self) -> Length {
            self.hydraulic_diameter
        }

        fn get_fluid_viscosity_at_ref_temperature(&mut self) -> DynamicViscosity {
            self.fluid_viscosity
        }

        fn get_fluid_viscosity_immutable_at_ref_temperature(&self) -> DynamicViscosity {
            self.fluid_viscosity
        }

        fn get_fluid_density_at_ref_temperature(&mut self) -> MassDensity {
            self.fluid_density
        }

        fn get_fluid_density_immutable_at_ref_temperature(&self) -> MassDensity {
            self.fluid_density
        }

        fn get_component_length(&mut self) -> Length {
            self.component_length
        }

        fn get_component_length_immutable(&self) -> Length {
            self.component_length
        }

        fn get_incline_angle(&mut self) -> Angle {
            self.incline_angle
        }

        fn get_incline_angle_immutable(&self) -> Angle {
            self.incline_angle
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

    impl FluidCustomComponentCalcPressureChange for CoriolisFlowmeter {}

    // let's have a test mass flowrate 

    let fluid_mass_flowrate_expected = MassRate::new::<kilogram_per_second>(0.2);
    let fluid_viscosity = DynamicViscosity::new::<poise>(0.01);
    let fluid_density = MassDensity::new::<kilogram_per_cubic_meter>(1000.0);

    let hydraulic_diameter = 
        Length::new::<inch>(1.0);

    let incline_angle = 
        Angle::new::<degree>(90.0);

    let component_length = 
        Length::new::<meter>(0.5);

    let absolute_roughness = 
        Length::new::<millimeter>(0.001);

    let a = Ratio::new::<ratio>(18.0);
    let b = Ratio::new::<ratio>(93000_f64);
    let c: f64 = -1.35;

    let loss_correlation = DimensionlessDarcyLossCorrelations::new_simple_reynolds_power_component(
        a, b, c);
    
    // create object 
    let coriolis_flowmeter = CoriolisFlowmeter{ 
        loss_correlation,
        mass_flowrate: MassRate::ZERO,
        pressure_loss: Pressure::ZERO,
        pressure_source: Pressure::ZERO,
        hydraulic_diameter,
        incline_angle,
        component_length,
        fluid_density,
        fluid_viscosity,
        absolute_roughness,
    };


    // methods test for getting mass flowrate from pressure change
    {
        // forward test (immutable)
        
        let input_pressure_change = Pressure::new::<pascal>(-6335.0);

        let mass_flowrate_test = coriolis_flowmeter.
            get_mass_flowrate_from_pressure_change_immutable(input_pressure_change);

        // expected mass flowrate is 0.2 kg/s (positive)
        approx::assert_relative_eq!(
            mass_flowrate_test.get::<kilogram_per_second>(),
            fluid_mass_flowrate_expected.get::<kilogram_per_second>(),
            max_relative=0.01);
    }

    {
        // reverse test (immutable)

        let input_pressure_change = Pressure::new::<pascal>(-3474.0);

        let mass_flowrate_test = coriolis_flowmeter.
            get_mass_flowrate_from_pressure_change_immutable(input_pressure_change);

        // expected mass flowrate is -0.2 kg/s (other direction)
        approx::assert_relative_eq!(
            mass_flowrate_test.get::<kilogram_per_second>(),
            -fluid_mass_flowrate_expected.get::<kilogram_per_second>(),
            max_relative=0.01);
    }

    // methods test for getting pressure_change from mass flowrate
    {
        // forward test (immutable)
        
        let mass_flowrate = fluid_mass_flowrate_expected;

        let pressure_change_forward_test = coriolis_flowmeter.
            get_pressure_change_immutable(mass_flowrate);

        // expected pressure change is -6335 Pa
        approx::assert_relative_eq!(
            pressure_change_forward_test.get::<pascal>(),
            -6335.0,
            max_relative=0.01);
    }

    {
        // reverse test (immutable)

        let mass_flowrate = -fluid_mass_flowrate_expected;

        let pressure_change_forward_test = coriolis_flowmeter.
            get_pressure_change_immutable(mass_flowrate);

        // expected pressure change is -3474 Pa
        approx::assert_relative_eq!(
            pressure_change_forward_test.get::<pascal>(),
            -3474.0,
            max_relative=0.01);
    }

    // change coriolis_flowmeter to mutable
    let mut coriolis_flowmeter = coriolis_flowmeter;
    // methods test for getting mass flowrate from pressure change
    {
        // forward test (mutable)
        
        let input_pressure_change = Pressure::new::<pascal>(-6335.0);
        coriolis_flowmeter.set_pressure_change(input_pressure_change);

        let mass_flowrate_test = coriolis_flowmeter.
            get_mass_flowrate();

        // expected mass flowrate is 0.2 kg/s (positive)
        approx::assert_relative_eq!(
            mass_flowrate_test.get::<kilogram_per_second>(),
            fluid_mass_flowrate_expected.get::<kilogram_per_second>(),
            max_relative=0.01);
    }

    {
        // reverse test (mutable)

        let input_pressure_change = Pressure::new::<pascal>(-3474.0);

        coriolis_flowmeter.set_pressure_change(input_pressure_change);

        let mass_flowrate_test = coriolis_flowmeter.
            get_mass_flowrate();

        // expected mass flowrate is -0.2 kg/s (other direction)
        approx::assert_relative_eq!(
            mass_flowrate_test.get::<kilogram_per_second>(),
            -fluid_mass_flowrate_expected.get::<kilogram_per_second>(),
            max_relative=0.01);
    }

    // methods test for getting pressure_change from mass flowrate
    {
        // forward test (mutable)
        
        let mass_flowrate = fluid_mass_flowrate_expected;

        coriolis_flowmeter.set_mass_flowrate(mass_flowrate);
        let pressure_change_forward_test = coriolis_flowmeter.
            get_pressure_change();

        // expected pressure change is -6335 Pa
        approx::assert_relative_eq!(
            pressure_change_forward_test.get::<pascal>(),
            -6335.0,
            max_relative=0.01);
    }

    {
        // reverse test (mutable)

        let mass_flowrate = -fluid_mass_flowrate_expected;

        coriolis_flowmeter.set_mass_flowrate(mass_flowrate);
        let pressure_change_reverse_test = coriolis_flowmeter.
            get_pressure_change();

        // expected pressure change is -3474 Pa
        approx::assert_relative_eq!(
            pressure_change_reverse_test.get::<pascal>(),
            -3474.0,
            max_relative=0.01);
    }
    Ok(())
}
