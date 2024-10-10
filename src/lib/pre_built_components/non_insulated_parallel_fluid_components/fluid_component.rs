use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::array_control_vol_and_fluid_component_collections::fluid_component_collection::fluid_component_traits::FluidComponentTrait;

use super::NonInsulatedParallelFluidComponent;
use uom::si::f64::*;


impl FluidComponentTrait for NonInsulatedParallelFluidComponent {
    /// for getting and setting mass flowrate, we multiply or divide by 
    /// the number of tubes
    ///
    /// we return on a basis of total number of tubes
    fn get_mass_flowrate(&mut self) -> MassRate  {
        let mut pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();

        let single_tube_mass_rate = 
            pipe_fluid_array.get_mass_flowrate();

        single_tube_mass_rate * (self.number_of_tubes as f64)
    }

    fn set_mass_flowrate(&mut self, mass_flowrate: MassRate) {
        
        // the therminol array must be accessed first 
        // and then cloned

        let mut pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();

        let single_tube_mass_rate = 
            mass_flowrate / (self.number_of_tubes as f64);

        pipe_fluid_array.set_mass_flowrate(single_tube_mass_rate);

        // unfortunately, this makes setting mass flowrate quite 
        // expensive as we need to clone it everytime

        self.pipe_fluid_array.set(pipe_fluid_array.into()).unwrap();
    }

    fn get_mass_flowrate_from_pressure_loss_immutable(
        &self, pressure_loss: Pressure) -> MassRate {
        let pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();

        let single_pipe_flowrate = 
            pipe_fluid_array
            .get_mass_flowrate_from_pressure_loss_immutable(
                pressure_loss);

        single_pipe_flowrate * (self.number_of_tubes as f64)
    }

    fn get_pressure_loss(&mut self) -> Pressure {
        let mut pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();

        pipe_fluid_array.get_pressure_loss()
    }

    fn set_pressure_loss(&mut self, pressure_loss: Pressure) {
        let mut pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();

        pipe_fluid_array.set_pressure_loss(pressure_loss)
    }

    fn get_pressure_loss_immutable(
        &self, mass_flowrate: MassRate) -> Pressure {
        let pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();

        let single_pipe_mass_flowrate: MassRate = 
            mass_flowrate / (self.number_of_tubes as f64);

        pipe_fluid_array
            .get_pressure_loss_immutable(single_pipe_mass_flowrate)
    }

    fn get_cross_sectional_area(&mut self) -> Area {
        let pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();

        let single_pipe_xs_area = 
            pipe_fluid_array.get_cross_sectional_area_immutable();

        single_pipe_xs_area * (self.number_of_tubes as f64)
    }

    fn get_cross_sectional_area_immutable(&self) -> Area {
        let pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();

        let single_pipe_xs_area = 
            pipe_fluid_array.get_cross_sectional_area_immutable();

        single_pipe_xs_area * (self.number_of_tubes as f64)
    }

    fn get_hydraulic_diameter(&mut self) -> Length {
        let pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();

        // we proved before in the fluid array that hydraulic diameter 
        // for one tube is the same as the hydraulic diameter for a parallel 
        // set of tubes
        pipe_fluid_array.get_hydraulic_diameter_immutable()
    }

    fn get_hydraulic_diameter_immutable(&self) -> Length {
        let pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();

        // we proved before in the fluid array that hydraulic diameter 
        // for one tube is the same as the hydraulic diameter for a parallel 
        // set of tubes
        pipe_fluid_array.get_hydraulic_diameter_immutable()
    }

    fn get_fluid_viscosity_at_ref_temperature(&mut self) -> DynamicViscosity {
        let pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();
        // the pipe fluid array gets the bulk average temperature automatically
        // and then computes an averaged viscosity

        pipe_fluid_array.get_fluid_viscosity_immutable()
    }

    fn get_fluid_viscosity_immutable_at_ref_temperature(&self) -> DynamicViscosity {
        let pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();
        // the pipe fluid array gets the bulk average temperature automatically
        // and then computes an averaged viscosity

        pipe_fluid_array.get_fluid_viscosity_immutable()
    }

    fn get_fluid_density_at_ref_temperature(&mut self) -> MassDensity {
        let pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();
        // the pipe fluid array gets the bulk average temperature automatically
        // and then computes an averaged density

        pipe_fluid_array.get_fluid_density_immutable()
    }

    fn get_fluid_density_immutable_at_ref_temperature(&self) -> MassDensity {
        let pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();
        // the pipe fluid array gets the bulk average temperature automatically
        // and then computes an averaged density

        pipe_fluid_array.get_fluid_density_immutable()
    }

    fn get_component_length(&mut self) -> Length {
        let pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();

        // component length
        // for one tube is the same as the hydraulic diameter for a parallel 
        // set of tubes
        pipe_fluid_array.get_component_length_immutable()
    }

    fn get_component_length_immutable(&self) -> Length {
        let pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();

        // component length
        // for one tube is the same as the hydraulic diameter for a parallel 
        // set of tubes
        pipe_fluid_array.get_component_length_immutable()
    }

    fn get_incline_angle(&mut self) -> Angle {
        let pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();

        // incline_angle
        // for one tube is the same as the hydraulic diameter for a parallel 
        // set of tubes
        pipe_fluid_array.get_incline_angle_immutable()
    }

    fn get_incline_angle_immutable(&self) -> Angle {
        let pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();

        // incline_angle
        // for one tube is the same as the hydraulic diameter for a parallel 
        // set of tubes
        pipe_fluid_array.get_incline_angle_immutable()
    }

    fn get_internal_pressure_source(&mut self) -> Pressure {
        let pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();

        // internal pressure
        // for one tube is the same as the hydraulic diameter for a parallel 
        // set of tubes
        pipe_fluid_array.get_internal_pressure_source_immutable()
    }

    fn get_internal_pressure_source_immutable(&self) -> Pressure {
        let pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();

        // internal pressure
        // for one tube is the same as the hydraulic diameter for a parallel 
        // set of tubes
        pipe_fluid_array.get_internal_pressure_source_immutable()
    }

    fn set_internal_pressure_source(
        &mut self,
        internal_pressure: Pressure) {
        let mut pipe_fluid_array: FluidArray = 
        self.pipe_fluid_array.clone().try_into().unwrap();

        pipe_fluid_array.set_internal_pressure_source(internal_pressure);

        // internal pressure
        // for one tube is the same as the hydraulic diameter for a parallel 
        // set of tubes
        self.pipe_fluid_array = pipe_fluid_array.into();


    }
}
