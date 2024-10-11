
use uom::si::f64::*;
use uom::si::acceleration::meter_per_second_squared;
use uom::si::mass_rate::kilogram_per_second;
use uom::ConstZero;

use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::fluid_component_calculation::DimensionlessDarcyLossCorrelations;
use crate::thermal_hydraulics_error::TuasLibError;
use crate::fluid_mechanics_correlations::dimensionalisation;
use crate::fluid_mechanics_correlations::churchill_friction_factor;
/// This is a generic fluid component trait,
/// which specifies that fluid components in general
/// should have the following properties accessed
/// via get and set methods
pub trait FluidComponentTrait {


    /// gets the mass flowrate of the component
    fn get_mass_flowrate(&mut self) -> MassRate ;

    /// sets the mass flowrate of the component
    fn set_mass_flowrate(&mut self, mass_flowrate: MassRate);

    /// gets the mass flowrate of component given a 
    /// fixed pressure change
    /// does so by immutably borrowing the object
    /// 
    fn get_mass_flowrate_from_pressure_change_immutable(
        &self, pressure_change: Pressure) -> MassRate {

        // the basic idea is to change the pressure change
        // variable into pressure loss and call the pressure loss
        // function
        // the default implementation is this:
        // pressure_change = -pressure_loss + hydrostatic_pressure_increase 
        // + pressure source
        //




        let pressure_loss = -pressure_change +
            self.get_hydrostatic_pressure_change_immutable_at_ref_temperature()+
            self.get_internal_pressure_source_immutable();


        let mass_rate = 
            self.get_mass_flowrate_from_pressure_loss_immutable(
                pressure_loss);

        return mass_rate;
    }


    /// gets the mass flowrate of component given a 
    /// fixed pressure change
    /// does so by immutably borrowing the object
    /// 
    fn get_mass_flowrate_from_pressure_loss_immutable(
        &self, pressure_loss: Pressure) -> MassRate;

    /// gets pressure loss
    fn get_pressure_loss(&mut self) -> Pressure;

    /// sets the pressure loss of the component
    fn set_pressure_loss(&mut self, pressure_loss: Pressure);

    /// gets the pressure loss of component given a 
    /// fixed mass flowrate
    /// does so by immutably borrowing the object
    fn get_pressure_loss_immutable(
        &self, mass_flowrate: MassRate) -> Pressure;


    /// gets cross sectional area
    fn get_cross_sectional_area(&mut self) -> Area;

    /// gets cross sectional area with immutable instance of self
    fn get_cross_sectional_area_immutable(&self) -> Area;

    /// gets hydraulic diamter
    fn get_hydraulic_diameter(&mut self) -> Length;

    /// gets hydraulic diamter with immutable instance of self
    fn get_hydraulic_diameter_immutable(&self) -> Length;

    /// gets fluid viscosity at some user set reference temperature
    fn get_fluid_viscosity_at_ref_temperature(&mut self) -> DynamicViscosity;

    /// gets fluid viscosity with an immutable instance of self
    ///at some user set reference temperature
    fn get_fluid_viscosity_immutable_at_ref_temperature(&self) -> DynamicViscosity;

    /// gets fluid density
    ///at some user set reference temperature
    fn get_fluid_density_at_ref_temperature(&mut self) -> MassDensity;

    /// gets fluid density with an immutable instance of self
    ///at some user set reference temperature
    fn get_fluid_density_immutable_at_ref_temperature(&self) -> MassDensity;

    /// gets the component length
    fn get_component_length(&mut self) -> Length;

    /// gets the component length immutably
    fn get_component_length_immutable(&self) -> Length;

    /// gets pressure change for a pipe given
    /// the set parameters
    fn get_pressure_change(&mut self) -> Pressure {

        // the default implementation is this:
        // pressure_change = -pressure_loss + hydrostatic_pressure_increase 
        // + pressure source
        //


        let pressure_loss = self.get_pressure_loss();

        // this is the second component: hydrostatic pressure


        let hydrostatic_pressure_increase = 
            self.get_hydrostatic_pressure_change_at_ref_temperature();

        // third component is pressure source

        let pressure_source = self.get_internal_pressure_source();

        return -pressure_loss + hydrostatic_pressure_increase + 
            pressure_source;
    }

    /// gets the pressure loss of component given a 
    /// fixed mass flowrate
    /// does so by immutably borrowing the object
    fn get_pressure_change_immutable(
        &self, mass_flowrate: MassRate) -> Pressure{


        // the default implementation is this:
        // pressure_change = -pressure_loss + hydrostatic_pressure_increase 
        // + pressure source
        //


        let pressure_loss = self.get_pressure_loss_immutable(
            mass_flowrate);

        // this is the second component: hydrostatic pressure

        let hydrostatic_pressure_increase = 
            self.get_hydrostatic_pressure_change_immutable_at_ref_temperature();

        // third component is pressure source

        let pressure_source = self.get_internal_pressure_source_immutable();

        return -pressure_loss + hydrostatic_pressure_increase + 
            pressure_source;
    }


    /// sets the pressure change for the given pipe
    fn set_pressure_change(&mut self, pressure_change: Pressure){

        // the default implementation is this:
        // pressure_change = -pressure_loss + hydrostatic_pressure_increase 
        // + pressure source
        //

        let hydrostatic_pressure_increase = 
            self.get_hydrostatic_pressure_change_at_ref_temperature();

        // third component is pressure source
        // for any internal pressure source or external, eg pumps

        let pressure_source = self.get_internal_pressure_source();

        // we then get the pressure loss term
        //

        let pressure_loss = -pressure_change + hydrostatic_pressure_increase +
            pressure_source;

        self.set_pressure_loss(pressure_loss);
    }

    

    /// gets the angle of incline for a pipe
    fn get_incline_angle(&mut self) -> Angle;

    /// gets the incline angle of the pipe with immutable self
    fn get_incline_angle_immutable(&self) -> Angle;

    /// gets the hydrostatic pressure change
    /// using h rho g
    ///
    /// the height increase is equal
    ///
    /// h = component_length * sin (incline_angle)
    ///
    /// component length is the shortest or straight line
    /// distance between
    /// inlet and outlet
    /// and incline angle is the angle that straight line makes
    /// with the horizontal plane
    fn get_hydrostatic_pressure_change_at_ref_temperature(
        &mut self) -> Pressure {

        let component_length =
            self.get_component_length();

        let incline_angle = 
            self.get_incline_angle();

        let fluid_density = 
            self.get_fluid_density_at_ref_temperature();

        let g: Acceleration = 
            Acceleration::new::<meter_per_second_squared>(-9.81);
        let delta_h: Length = component_length*incline_angle.sin();

        let hydrostatic_pressure_increase: Pressure =
            fluid_density * g * delta_h;

        return hydrostatic_pressure_increase;
    }

    /// gets the hydrostatic pressure change
    /// with an immutable instance of self
    /// using h rho g
    ///
    /// the height increase is equal
    ///
    /// h = pipe_length * sin (incline_angle)
    ///
    /// component length is the shortest or straight line
    /// distance between
    /// inlet and outlet
    /// and incline angle is the angle that straight line makes
    /// with the horizontal plane
    fn get_hydrostatic_pressure_change_immutable_at_ref_temperature(
        &self) -> Pressure {

        let component_length =
            self.get_component_length_immutable();

        let incline_angle = 
            self.get_incline_angle_immutable();

        let fluid_density = 
            self.get_fluid_density_immutable_at_ref_temperature();


        let g: Acceleration = 
            Acceleration::new::<meter_per_second_squared>(-9.81);
        let delta_h: Length = component_length*incline_angle.sin();

        let hydrostatic_pressure_increase: Pressure =
            fluid_density * g * delta_h;

        return hydrostatic_pressure_increase;
    }

    /// gets the pressure source for a fluid component
    fn get_internal_pressure_source(&mut self) -> Pressure;


    /// gets the pressure source for a fluid component
    /// with an immutable instance of self
    fn get_internal_pressure_source_immutable(&self) -> Pressure;

    /// sets the internal pressure source for a pipe
    fn set_internal_pressure_source(
        &mut self,
        internal_pressure: Pressure);

}

/// contains methods to get pressure loss 
/// and pressure change and mass flowrate based on 
/// current state of the fluid component collection
pub trait FluidComponentCollectionMethods{

    /// calculates pressure loss when given a mass flowrate
    fn get_pressure_loss(
        &self, 
        fluid_mass_flowrate: MassRate) -> Pressure {

        // for pressure losses, we compare the pressure change at
        // zero mass flowrate to pressure change at the desired
        // mass flowrate
        // noting that 
        //
        // pressure_change = - pressure_loss + hydrostatic pressure +
        // internal pressure


        let zero_mass_flow = MassRate::new::<kilogram_per_second>(0.0);

        let reference_pressure_change = 
            self.get_pressure_change(zero_mass_flow);

        let current_pressure_change = 
            self.get_pressure_change(fluid_mass_flowrate);

        let pressure_change_due_to_losses = 
            current_pressure_change - reference_pressure_change;

        let pressure_loss = -pressure_change_due_to_losses;

        return pressure_loss;

    }

    /// calculates pressure change when given a mass flowrate
    fn get_pressure_change(
        &self, 
        fluid_mass_flowrate: MassRate) -> Pressure;

    /// calculates mass flowrate from pressure change

    fn get_mass_flowrate_from_pressure_change(
        &self,
        pressure_change: Pressure) -> MassRate;

    /// calculates mass flowrate from pressure loss
    
    fn get_mass_flowrate_from_pressure_loss(
        &self,
        pressure_loss: Pressure) -> MassRate {

        // for this, the default implementation is
        // to obtain pressure change
        //
        // pressure_change = -pressure_loss +
        // hydrostatic pressure
        // + internal pressure
        //
        // to get the latter two terms, i can obtain
        // pressure change when mass flowrate is zero
        let zero_mass_flow = MassRate::new::<kilogram_per_second>(0.0);

        let reference_pressure_change = 
            self.get_pressure_change(zero_mass_flow);

        let pressure_change = 
            -pressure_loss + reference_pressure_change;

        // now let's calculate the mass flowrate

        return self.get_mass_flowrate_from_pressure_change(pressure_change);
    }


}

/// provides generic methods to calculate mass flowrate
/// and pressure losses for pipes
///
/// see FluidComponent example for how to use
pub trait FluidPipeCalcPressureLoss {

    /// gets form loss k for a pipe
    fn get_pipe_form_loss_k(&mut self) -> f64;

    /// gets form loss k for a pipe
    /// using an immutable reference to self
    fn get_pipe_form_loss_k_immutable(&self) -> f64;

    /// gets absolute roughness for a pipe
    fn get_pipe_absolute_roughness(&mut self) -> Length;


    /// gets absolute roughness for a pipe
    /// using an immutable reference to self
    fn get_pipe_absolute_roughness_immutable(&self) -> Length;
    

    /// a function calculates pressure
    /// loss given a mass flowrate and pipe properties
    fn pipe_calc_pressure_loss(
        mut fluid_mass_flowrate: MassRate,
        cross_sectional_area: Area,
        hydraulic_diameter: Length,
        fluid_viscosity: DynamicViscosity,
        fluid_density: MassDensity,
        pipe_length: Length,
        absolute_roughness: Length,
        form_loss_k: f64) -> Result<Pressure,TuasLibError> {
        // first let's calculate roughness ratio

        let roughness_ratio_quantity = absolute_roughness/hydraulic_diameter;

        let roughness_ratio = roughness_ratio_quantity;

        // second i want to take care of reverse flow

        let mut reverse_flow = false;
        if fluid_mass_flowrate.value < 0.0 {
            reverse_flow = true;
        }

        if reverse_flow {
            fluid_mass_flowrate = fluid_mass_flowrate * -1.0;
        }

        // and let's get the reynolds_number and L/D
        let reynolds_number = dimensionalisation::calc_reynolds_from_mass_rate(
            fluid_mass_flowrate,
            cross_sectional_area,
            hydraulic_diameter,
            fluid_viscosity);

        let length_to_diameter_ratio = pipe_length/hydraulic_diameter;

        // then let's obtain the pipe Bejan Number
        // given the Re

        let bejan_number = churchill_friction_factor::get_bejan_number_d(
            reynolds_number.into(),
            roughness_ratio.into(),
            length_to_diameter_ratio.into(),
            form_loss_k)?;

        // once we get bejan_number, we can get the pressure loss terms
        //
        let pressure_loss = dimensionalisation::calc_bejan_to_pressure(
            bejan_number.into(),
            hydraulic_diameter,
            fluid_density,
            fluid_viscosity);


        // now before i exit, i want to make sure reverse flow is taken care
        // of
        if reverse_flow {
            return Ok(pressure_loss * -1.0);
        }

        return Ok(pressure_loss);
    }



    /// a function which calculates pressure
    /// loss given a mass flowrate and pipe properties
    fn pipe_calc_mass_flowrate(
        pressure_loss: Pressure,
        cross_sectional_area: Area,
        hydraulic_diameter: Length,
        fluid_viscosity: DynamicViscosity,
        fluid_density: MassDensity,
        pipe_length: Length,
        absolute_roughness: Length,
        form_loss_k: f64) -> Result<MassRate,TuasLibError> {

        // first let's get our relevant ratios:
        let roughness_ratio_quantity = absolute_roughness/hydraulic_diameter;

        let roughness_ratio = roughness_ratio_quantity;

        let length_to_diameter_ratio = pipe_length/hydraulic_diameter;

        // then get Bejan number:

        let bejan_number_calculated_using_diameter = 
            dimensionalisation::calc_bejan_from_pressure(
            pressure_loss, hydraulic_diameter, 
            fluid_density, fluid_viscosity);

        // let's get Re
        let reynolds_number_calculated_using_diameter = 
            churchill_friction_factor::get_reynolds_from_bejan(
                bejan_number_calculated_using_diameter.into(),
                roughness_ratio.into(),
                length_to_diameter_ratio.into(),
                form_loss_k)?;


        // and finally return mass flowrate
        //
        let fluid_mass_flowrate = 
            dimensionalisation::calc_reynolds_to_mass_rate(
                cross_sectional_area,
                reynolds_number_calculated_using_diameter.into(),
                hydraulic_diameter,
                fluid_viscosity);

        return Ok(fluid_mass_flowrate);

    }
}

/// provides generic methods to calculate pressure change
/// to and from mass flowrate for an Inclined pipe
/// with some internal pressure source (eg. pump)
///
/// 
pub trait FluidPipeCalcPressureChange : FluidPipeCalcPressureLoss + 
FluidComponentTrait{

    /// calculates a pressure change of the pipe 
    /// given the 
    ///
    /// pressure_change = pressure_loss + hydrostatic_pressure + 
    /// internal_source_pressure
    fn pipe_calc_pressure_change(
        fluid_mass_flowrate: MassRate,
        cross_sectional_area: Area,
        hydraulic_diameter: Length,
        fluid_viscosity: DynamicViscosity,
        fluid_density: MassDensity,
        pipe_length: Length,
        absolute_roughness: Length,
        form_loss_k: f64,
        incline_angle: Angle,
        source_pressure: Pressure) -> Result<Pressure,TuasLibError> {

        // first we calculate the pressure loss
        // of the pipe
        // given a flat surface

        let pressure_loss = 
            <Self as FluidPipeCalcPressureLoss>::pipe_calc_pressure_loss(
                fluid_mass_flowrate,
                cross_sectional_area,
                hydraulic_diameter,
                fluid_viscosity,
                fluid_density,
                pipe_length,
                absolute_roughness,
                form_loss_k)?;

        let hydrostatic_pressure_increase: Pressure = 
            <Self as FluidPipeCalcPressureChange>::get_hydrostatic_pressure_change(
                pipe_length,
                incline_angle,
                fluid_density);

        let pressure_change = 
            -pressure_loss +
            hydrostatic_pressure_increase+
            source_pressure;


        return Ok(pressure_change);

    }

    /// calculates a mass flowrate given a pressure change
    /// for a fluid pipe
    fn pipe_calculate_mass_flowrate_from_pressure_change(
        pressure_change: Pressure,
        cross_sectional_area: Area,
        hydraulic_diameter: Length,
        fluid_viscosity: DynamicViscosity,
        fluid_density: MassDensity,
        pipe_length: Length,
        absolute_roughness: Length,
        form_loss_k: f64,
        incline_angle: Angle,
        source_pressure: Pressure) -> Result<MassRate,TuasLibError> {

        // now we need to calculate a pressure loss term
        // we use:
        // Pressure Change = - pressure loss + hydrostatic pressure +
        // source pressure
        //
        // so we just add pressure loss to both sides and subtract pressure
        // change to both sides
        // pressure loss  = - pressure change + hydrostatic pressure +
        // source pressure

        // for hydrostatic pressure gain
        // g is earth gravity at 9.81
        // delta H is positive upwards
        let hydrostatic_pressure_increase: Pressure =
            <Self as FluidPipeCalcPressureChange>::get_hydrostatic_pressure_change(
                pipe_length,
                incline_angle,
                fluid_density);

        // now calculate pressure loss
        let pressure_loss = 
            -pressure_change +
            hydrostatic_pressure_increase +
            source_pressure;

        let mass_rate = 
            <Self as FluidPipeCalcPressureLoss>::pipe_calc_mass_flowrate(
                pressure_loss,
                cross_sectional_area,
                hydraulic_diameter,
                fluid_viscosity,
                fluid_density,
                pipe_length,
                absolute_roughness,
                form_loss_k)?;

        return Ok(mass_rate);

    }

    /// calculates hydrostatic pressure change
    /// kind of boilerplate code but i want
    /// to use it as an associated function rather 
    /// than a method
    ///
    /// this is because i want the method in FluidComponent
    /// to take &mut self or &self
    /// so that we can have object safety (or something like that)
    fn get_hydrostatic_pressure_change(
        pipe_length: Length,
        incline_angle: Angle,
        fluid_density: MassDensity) -> Pressure {

        let g: Acceleration = 
            Acceleration::new::<meter_per_second_squared>(-9.81);
        let delta_h: Length = pipe_length*incline_angle.sin();

        let hydrostatic_pressure_increase: Pressure =
            fluid_density * g * delta_h;

        return hydrostatic_pressure_increase;
    }

}


/// provides generic methods to calculate pressure
/// loss for a custom fluid component (with flow flowing
/// inside it)
/// given a custom darcy friction factor and
/// custom form loss correlation
pub trait FluidCustomComponentCalcPressureLoss {


    /// returns the custom form loss factors
    /// for the component
    fn get_custom_loss_correlations(&mut self) ->
        DimensionlessDarcyLossCorrelations;

    /// returns the custom loss correlations
    /// for the component
    /// using an immutable reference to self
    fn get_custom_loss_correlations_immutable(&self) ->
        DimensionlessDarcyLossCorrelations;

    /// sets the custom darcy friction factor function
    /// usually a function of Re and roughness ratio
    /// for the component
    fn set_custom_loss_correlations(
        &mut self,
        custom_loss_correlation: DimensionlessDarcyLossCorrelations);

    /// gets the component absolute roughness for
    /// the component in question
    fn get_custom_component_absolute_roughness(
        &mut self) -> Length;

    /// gets the custom component absolute roughness 
    /// using an immutable reference to self
    fn get_custom_component_absolute_roughness_immutable(
        &self) -> Length;

    /// calculates pressure loss for a component given 
    /// pipe parameter inputs and
    /// custom darcy friction factor and custom form loss
    /// correlations
    fn fluid_custom_component_calc_pressure_loss(
        fluid_mass_flowrate: MassRate,
        cross_sectional_area: Area,
        hydraulic_diameter: Length,
        fluid_viscosity: DynamicViscosity,
        fluid_density: MassDensity,
        loss_correlation: DimensionlessDarcyLossCorrelations,) -> 
        Result<Pressure,TuasLibError> {

        // first we get our Reynolds number

        let reynolds_number_quantity_object = fluid_mass_flowrate/
            cross_sectional_area*
            hydraulic_diameter/
            fluid_viscosity;

        let reynolds_number_calculated_using_diameter = 
                reynolds_number_quantity_object;



        // now we have this, we can calculate bejan number


        let bejan_number_calculated_using_diameter = 
            loss_correlation.get_bejan_number_from_reynolds(
                reynolds_number_calculated_using_diameter)?;

        // once we get Be, we can get the pressure loss terms
        //
        let pressure_loss = dimensionalisation::calc_bejan_to_pressure(
            bejan_number_calculated_using_diameter.into(),
            hydraulic_diameter,
            fluid_density,
            fluid_viscosity);



        return Ok(pressure_loss);
    }

    /// calculates mass flowrate using input parameters
    fn fluid_custom_component_calc_mass_flowrate_from_pressure_loss(
        pressure_loss: Pressure,
        cross_sectional_area: Area,
        hydraulic_diameter: Length,
        fluid_viscosity: DynamicViscosity,
        fluid_density: MassDensity,
        loss_correlation: DimensionlessDarcyLossCorrelations,) 
        -> Result<MassRate,TuasLibError> {

            // need to first consider reverse flow and zero flow
            // if pressure loss is zero, then return a zero mass flowrate

            if pressure_loss == Pressure::ZERO {
                return Ok(MassRate::ZERO);
            }

            let reverse_flow: bool;
            let pressure_loss_abs: Pressure = pressure_loss.abs();

            if pressure_loss > Pressure::ZERO {
                // if pressure loss more than zero, we get forward flow 
                reverse_flow = false;
            } else {
                reverse_flow = true;
            }


            // then get Bejan number:
            // which is dimensionless pressure loss
            let bejan_number_calculated_using_diameter = 
                dimensionalisation::calc_bejan_from_pressure(
                    pressure_loss_abs, 
                    hydraulic_diameter, 
                    fluid_density, 
                    fluid_viscosity);


            let reynolds_number_calculated_using_diameter = 
                loss_correlation.get_reynolds_number_from_bejan(
                    bejan_number_calculated_using_diameter)?;


            // and finally return mass flowrate
            //
            let fluid_mass_flowrate = 
                dimensionalisation::calc_reynolds_to_mass_rate(
                    cross_sectional_area,
                    reynolds_number_calculated_using_diameter.into(),
                    hydraulic_diameter,
                    fluid_viscosity);

            if reverse_flow {
                return Ok(-fluid_mass_flowrate);
            } else {
                return Ok(fluid_mass_flowrate);
            }
        }
}

/// Contains default implementations for calculating
/// mass flowrate from pressure change and vice versea
///
/// refer to examples in fluid_component_calculation
/// to see how its used
pub trait FluidCustomComponentCalcPressureChange :
FluidCustomComponentCalcPressureLoss+ FluidComponentTrait{

    /// calculates the pressure change for a custom
    /// fluid component given a mass flowrate
    /// and other fluid component parameters
    fn fluid_custom_component_calc_pressure_change(
        fluid_mass_flowrate: MassRate,
        cross_sectional_area: Area,
        hydraulic_diameter: Length,
        fluid_viscosity: DynamicViscosity,
        fluid_density: MassDensity,
        component_length: Length,
        incline_angle: Angle,
        source_pressure: Pressure,
        loss_correlation: DimensionlessDarcyLossCorrelations) -> 
        Result<Pressure,TuasLibError> {

        // now we need to calculate a pressure loss term
        // we use:
        // Pressure Change = - pressure loss + hydrostatic pressure +
        // source pressure
        //
        // so we just add pressure loss to both sides and subtract pressure
        // change to both sides
        // pressure loss  = - pressure change + hydrostatic pressure +
        // source pressure
        //

        let pressure_loss = <Self as FluidCustomComponentCalcPressureLoss>::
            fluid_custom_component_calc_pressure_loss(
                fluid_mass_flowrate, 
                cross_sectional_area, 
                hydraulic_diameter, 
                fluid_viscosity, 
                fluid_density, 
                loss_correlation)?;


        let hydrostatic_pressure =
            <Self as FluidCustomComponentCalcPressureChange>::
            get_hydrostatic_pressure_change(
                component_length, 
                incline_angle, 
                fluid_density);

        let pressure_change =
            - pressure_loss 
            + hydrostatic_pressure 
            + source_pressure;


        return Ok(pressure_change);
    }

    /// calculates the mass flowrate given pressure change
    /// and other parameters of the component
    ///
    fn fluid_custom_component_calc_mass_flowrate_from_pressure_change(
        pressure_change: Pressure,
        cross_sectional_area: Area,
        hydraulic_diameter: Length,
        fluid_viscosity: DynamicViscosity,
        fluid_density: MassDensity,
        component_length: Length,
        incline_angle: Angle,
        source_pressure: Pressure,
        loss_correlation: DimensionlessDarcyLossCorrelations) -> 
        Result<MassRate,TuasLibError> {

        // now we need to calculate a pressure loss term
        // we use:
        // Pressure Change = - pressure loss + hydrostatic pressure +
        // source pressure
        //
        // so we just add pressure loss to both sides and subtract pressure
        // change to both sides
        // pressure loss  = - pressure change + hydrostatic pressure +
        // source pressure

        let hydrostatic_pressure = 
            <Self as FluidCustomComponentCalcPressureChange>::
            get_hydrostatic_pressure_change(
                component_length, 
                incline_angle, 
                fluid_density);

        let pressure_loss = 
            - pressure_change
            + hydrostatic_pressure 
            + source_pressure;

        // once we have pressure loss
        // we can get mass flowrate

        let mass_flowrate: MassRate 
            = <Self as FluidCustomComponentCalcPressureLoss>::
            fluid_custom_component_calc_mass_flowrate_from_pressure_loss(
                pressure_loss, 
                cross_sectional_area, 
                hydraulic_diameter, 
                fluid_viscosity, 
                fluid_density, 
                loss_correlation)?;

        return Ok(mass_flowrate);
    }

    /// calculates hydrostatic pressure change
    /// kind of boilerplate code but i want
    /// to use it as an associated function rather 
    /// than a method
    ///
    /// this is because i want the method in FluidComponent
    /// to take &mut self or &self
    /// so that we can have object safety (or something like that)
    fn get_hydrostatic_pressure_change(
        pipe_length: Length,
        incline_angle: Angle,
        fluid_density: MassDensity) -> Pressure {

        let g: Acceleration = 
            Acceleration::new::<meter_per_second_squared>(-9.81);
        let delta_h: Length = pipe_length*incline_angle.sin();

        let hydrostatic_pressure_increase: Pressure =
            fluid_density * g * delta_h;

        return hydrostatic_pressure_increase;
    }
}

/// unit tests for fluid_component traits 
pub mod unit_test_fluid_component_traits;
