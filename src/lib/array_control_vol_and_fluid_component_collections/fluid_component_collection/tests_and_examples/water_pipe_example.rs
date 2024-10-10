

/// Example 2:
///
/// We saw previously how to create an air pipe
/// now we shall make a slanted water pipe
/// with some internal pressure source (as if it had a pump attached
/// to it)
///
/// we shall improve on how we can create the pipes
/// to do so, we shall use the FluidComponent trait and the 
/// FluidPipeCalcPressureChange trait
///
#[test]
pub fn water_pipe_with_internal_pump_example_2() {

    use std::f64::consts::PI;

    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidPipeCalcPressureChange;
    use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidPipeCalcPressureLoss;
    use uom::si::dynamic_viscosity::poise;
    use uom::si::f64::*;
    use uom::si::length::{meter, inch, millimeter};
    use uom::si::mass_density::kilogram_per_cubic_meter;
    use uom::si::mass_rate::kilogram_per_second;
    use uom::si::pressure::pascal;
    use uom::si::angle::degree;
    // first we want to start with a water pipe struct,
    // this time, we use the constructor to define both
    // pipe properties and fluid properties
    //
    // this is still an isothermal case
    //
    // you may want to implement the traits so that you know what data
    // you need to have

    struct WaterPipe {
        mass_flowrate: MassRate,
        pressure_loss: Pressure,
        dynamic_viscosity: DynamicViscosity,
        density: MassDensity,
        form_loss_k: f64,
        absolute_roughness: Length,
        incline_angle: Angle,
        internal_pressure_source: Pressure,
        pipe_length: Length,
        hydraulic_diameter: Length,
    }

    impl FluidPipeCalcPressureChange for WaterPipe {}

    impl FluidPipeCalcPressureLoss for WaterPipe {
        fn get_pipe_form_loss_k(&mut self) -> f64 {
            return self.form_loss_k;
        }

        fn get_pipe_form_loss_k_immutable(&self) -> f64 {
            return self.form_loss_k;
        }

        fn get_pipe_absolute_roughness(&mut self) -> Length {
            return self.absolute_roughness;
        }

        fn get_pipe_absolute_roughness_immutable(&self) -> Length {
            return self.absolute_roughness;
        }
    }

    impl FluidComponentTrait for WaterPipe {
        fn get_internal_pressure_source(&mut self) -> Pressure {
            return self.internal_pressure_source;
        }

        fn get_internal_pressure_source_immutable(&self) -> Pressure {
            return self.internal_pressure_source;
        }

        fn set_internal_pressure_source(
            &mut self,
            internal_pressure_source: Pressure){
            self.internal_pressure_source = internal_pressure_source;
        }

        fn get_component_length(&mut self) -> Length {
            return self.pipe_length;
        }


        fn get_component_length_immutable(&self) -> Length {
            return self.pipe_length;
        }

        fn get_incline_angle(&mut self) -> Angle {
            return self.incline_angle;
        }

        fn get_incline_angle_immutable(&self) -> Angle {
            return self.incline_angle;
        }

        fn get_fluid_density_at_ref_temperature(&mut self) -> MassDensity {
            return self.density;
        }

        fn get_fluid_density_immutable_at_ref_temperature(&self) -> MassDensity {
            return self.density;
        }

        fn get_fluid_viscosity_at_ref_temperature(&mut self) -> DynamicViscosity {
            return self.dynamic_viscosity;
        }

        fn get_fluid_viscosity_immutable_at_ref_temperature(&self) -> DynamicViscosity {
            return self.dynamic_viscosity;
        }

        fn get_hydraulic_diameter(&mut self) -> Length {
            return self.hydraulic_diameter;
        }

        fn get_hydraulic_diameter_immutable(&self) -> Length {
            return self.hydraulic_diameter;
        }

        fn get_cross_sectional_area(&mut self) -> Area {
            return self.get_hydraulic_diameter()*
                self.get_hydraulic_diameter()*
                PI/4.0_f64;
        }

        fn get_cross_sectional_area_immutable(&self) -> Area {
            return self.get_hydraulic_diameter_immutable()*
                self.get_hydraulic_diameter_immutable()*
                PI/4.0_f64;
        }

        fn set_pressure_loss(&mut self, pressure_loss: Pressure){
            self.pressure_loss = pressure_loss;
        }

        fn set_mass_flowrate(&mut self, mass_flowrate: MassRate){
            self.mass_flowrate = mass_flowrate;
        }

        fn get_mass_flowrate(&mut self) -> MassRate {
            // get pipe parameters and flow conditions
            // from the get methods
            let form_loss_k = self.get_pipe_form_loss_k();
            let absolute_roughness = self.get_pipe_absolute_roughness();
            let cross_sectional_area = self.get_cross_sectional_area();
            let hydraulic_diameter = self.get_hydraulic_diameter();
            let fluid_viscosity = self.get_fluid_viscosity_at_ref_temperature();
            let fluid_density = self.get_fluid_density_at_ref_temperature();
            let pipe_length = self.get_component_length();
            let pressure_loss = self.pressure_loss;
            let incline_angle = self.get_incline_angle();
            let internal_pressure_source = self.get_internal_pressure_source();

            let pressure_change = 
                -pressure_loss 
                + internal_pressure_source 
                + WaterPipe::get_hydrostatic_pressure_change(
                    pipe_length,
                    incline_angle,
                    fluid_density);

            let mass_flowrate = 
                WaterPipe::pipe_calculate_mass_flowrate_from_pressure_change(
                    pressure_change, 
                    cross_sectional_area, 
                    hydraulic_diameter, 
                    fluid_viscosity, 
                    fluid_density, 
                    pipe_length, 
                    absolute_roughness, 
                    form_loss_k,
                    incline_angle,
                    internal_pressure_source).unwrap();

            // you can return the mass flowrate straightaway
            // or set the struct variable first and then
            // return it

            self.set_mass_flowrate(mass_flowrate);

            return self.mass_flowrate;

        }

        fn get_mass_flowrate_from_pressure_loss_immutable(
            &self,
            pressure_loss: Pressure) -> MassRate {
            // get pipe parameters and flow conditions
            // from the get methods
            let form_loss_k = self.get_pipe_form_loss_k_immutable();
            let absolute_roughness = self.get_pipe_absolute_roughness_immutable();
            let cross_sectional_area = self.get_cross_sectional_area_immutable();
            let hydraulic_diameter = self.get_hydraulic_diameter_immutable();
            let fluid_viscosity = self.get_fluid_viscosity_immutable_at_ref_temperature();
            let fluid_density = self.get_fluid_density_immutable_at_ref_temperature();
            let pipe_length = self.get_component_length_immutable();
            let incline_angle = self.get_incline_angle_immutable();
            let internal_pressure_source = self.get_internal_pressure_source_immutable();

            let pressure_change = 
                -pressure_loss 
                + internal_pressure_source 
                + <WaterPipe as FluidPipeCalcPressureChange>::
                get_hydrostatic_pressure_change(
                    pipe_length,
                    incline_angle,
                    fluid_density);

            let mass_flowrate = 
                WaterPipe::pipe_calculate_mass_flowrate_from_pressure_change(
                    pressure_change, 
                    cross_sectional_area, 
                    hydraulic_diameter, 
                    fluid_viscosity, 
                    fluid_density, 
                    pipe_length, 
                    absolute_roughness, 
                    form_loss_k,
                    incline_angle,
                    internal_pressure_source).unwrap();

            // you can return the mass flowrate straightaway
            // or set the struct variable first and then
            // return it


            return mass_flowrate;

        }

        fn get_pressure_loss(&mut self) -> Pressure {


            // get pipe parameters and flow conditions
            // from the get methods
            let form_loss_k = self.get_pipe_form_loss_k();
            let absolute_roughness = self.get_pipe_absolute_roughness();
            let cross_sectional_area = self.get_cross_sectional_area();
            let mass_flowrate = self.mass_flowrate;
            let hydraulic_diameter = self.get_hydraulic_diameter();
            let viscosity = self.get_fluid_viscosity_at_ref_temperature();
            let density = self.get_fluid_density_at_ref_temperature();
            let pipe_legnth = self.get_component_length();


            // calculate the pressure loss

            let pressure_loss = 
                WaterPipe::pipe_calc_pressure_loss(
                    mass_flowrate,
                    cross_sectional_area,
                    hydraulic_diameter,
                    viscosity,
                    density,
                    pipe_legnth,
                    absolute_roughness,
                    form_loss_k).unwrap();

            // you can return the pressure loss straightaway
            // or set the struct variable first and then
            // return it

            self.pressure_loss = pressure_loss;

            return self.pressure_loss;
        }

        fn get_pressure_loss_immutable(
            &self,
            mass_flowrate: MassRate) -> Pressure {


            // get pipe parameters and flow conditions
            // from the get methods
            let form_loss_k = self.get_pipe_form_loss_k_immutable();
            let absolute_roughness = self.get_pipe_absolute_roughness_immutable();
            let cross_sectional_area = self.get_cross_sectional_area_immutable();
            let hydraulic_diameter = self.get_hydraulic_diameter_immutable();
            let viscosity = self.get_fluid_viscosity_immutable_at_ref_temperature();
            let density = self.get_fluid_density_immutable_at_ref_temperature();
            let pipe_legnth = self.get_component_length_immutable();


            // calculate the pressure loss

            let pressure_loss = 
                WaterPipe::pipe_calc_pressure_loss(
                    mass_flowrate,
                    cross_sectional_area,
                    hydraulic_diameter,
                    viscosity,
                    density,
                    pipe_legnth,
                    absolute_roughness,
                    form_loss_k).unwrap();

            // you can return the pressure loss straightaway
            // or set the struct variable first and then
            // return it


            return pressure_loss;
        }

        fn set_pressure_change(&mut self, pressure_change: Pressure){
            // we use the following formula
            // pressure_change = -pressure_loss + hydrostatic_pressure +
            // internal pressure source
            //
            // by setting pressure change, we are indirectly setting
            // pressure loss
            //


            let pressure_loss = -pressure_change +
                WaterPipe::get_hydrostatic_pressure_change(
                    self.pipe_length,
                    self.incline_angle,
                    self.get_fluid_density_at_ref_temperature()) +
                self.get_internal_pressure_source();

            self.set_pressure_loss(pressure_loss);
        }

        fn get_pressure_change(&mut self) -> Pressure {

            let form_loss_k = self.get_pipe_form_loss_k();
            let absolute_roughness = self.get_pipe_absolute_roughness();
            let cross_sectional_area = self.get_cross_sectional_area();
            let hydraulic_diameter = self.get_hydraulic_diameter();
            let fluid_viscosity = self.get_fluid_viscosity_at_ref_temperature();
            let fluid_density = self.get_fluid_density_at_ref_temperature();
            let pipe_length = self.get_component_length();
            let incline_angle = self.get_incline_angle();
            let internal_pressure_source = self.get_internal_pressure_source();
            let mass_flowrate = self.mass_flowrate;


            // return the pressure change value
            let pressure_change = WaterPipe::pipe_calc_pressure_change(
                mass_flowrate,
                cross_sectional_area,
                hydraulic_diameter,
                fluid_viscosity,
                fluid_density,
                pipe_length,
                absolute_roughness,
                form_loss_k,
                incline_angle,
                internal_pressure_source).unwrap();

            self.set_pressure_change(pressure_change);

            return pressure_change;

        }
    }

    // lastly we implement the constructor,
    // since we know the pipe has water flowing through,
    // density and viscosity are fixed
    //
    // Everything else though, has to be set by the user
    // mass flowrate and pressure loss can be
    // set to 0 by default
    // 
    // internal pressure source is also set to 0,
    // it is up to the user to set internal pressure source
    impl WaterPipe {
        fn new(form_loss_k: f64,
            absolute_roughness: Length,
            incline_angle: Angle,
            pipe_length: Length,
            hydraulic_diameter: Length) -> Self {

            return Self {
                mass_flowrate: MassRate::new::<kilogram_per_second>(0.0),
                pressure_loss: Pressure::new::<pascal>(0.0),
                dynamic_viscosity: DynamicViscosity::new::<poise>(0.01),
                density: MassDensity::new::<kilogram_per_cubic_meter>(1000.0),
                form_loss_k: form_loss_k,
                absolute_roughness: absolute_roughness,
                incline_angle: incline_angle,
                internal_pressure_source: Pressure::new::<pascal>(0.0),
                pipe_length: pipe_length,
                hydraulic_diameter: hydraulic_diameter,
            };
        }
    }

    // and just like that we've finished defining our water pipe
    //
    // pipe shall be 1m long, angled 25 degrees
    // 1 inch diameter
    // form loss is 0.5
    // copper, 0.002 mm roughness

    let mut water_pipe_1 = WaterPipe::new(
        0.5, // form losses
        Length::new::<millimeter>(0.002), // surface roughness
        Angle::new::<degree>(25.0), // incline angle
        Length::new::<meter>(1.0), // pipe length
        Length::new::<inch>(1.0)); // pipe inner diameter


    // let's set mass flowrate at 0.5 kg/s
    water_pipe_1.set_mass_flowrate(
        MassRate::new::<kilogram_per_second>(0.5)
    );

    // find the pressure change

    let pressure_change = water_pipe_1.get_pressure_change();

    // pressure change is -4861 Pa
    approx::assert_relative_eq!(
        pressure_change.value,
        -4861_f64,
        max_relative = 0.01 );

    // likewise when i get my mass flowrate from pressure change
    // i should get the same value




    water_pipe_1.set_pressure_change(
        Pressure::new::<pascal>(-4861_f64));

    let mass_flowrate = 
        water_pipe_1.get_mass_flowrate();

    approx::assert_relative_eq!(
        mass_flowrate.value,
        0.5,
        max_relative = 0.01 );

    // last but not least, i want to check our immutable versions
    // of these functions and see if they work well
    //
    // the immutable versions of the methods take in &self rather
    // than &mut self, this enables safety in terms of parallelism
    // and may help with the use of peroxide iteration libraries
    // which are numerical root finders. These root finders 
    // cannot use mutable functions


    assert_eq!(mass_flowrate,
        water_pipe_1.get_mass_flowrate_from_pressure_change_immutable(
            Pressure::new::<pascal>(-4861_f64)));

    // and that concludes the example! You can now set 
    // the water pipe to anything you want.
    //
    // of course, it will be good to have common enums and cases
    // that can return surface roughness of commonly used material
    // as well as densities, viscosities, etc.
    //
    // Likely I'll put them in some property library stored as a trait



}

