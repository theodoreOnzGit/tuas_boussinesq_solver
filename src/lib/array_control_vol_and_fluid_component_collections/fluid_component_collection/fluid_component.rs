use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use uom::si::f64::*;

use super::fluid_component_traits::FluidComponentTrait;


#[derive(Debug,Clone,PartialEq)]
/// FluidComponents are pipes and fittings you can connect in parallel
/// such that you can calculate mass flowrate and pressure drop from them
pub enum FluidComponent {
    /// these are arrays of control volumes connected in series
    FluidArray(FluidArray),
    /// these are parallel arrays of fluid arrays (which themselves 
    /// are control volumes in series)
    ///
    /// one fluid array represents one tube in this parallel array
    ///
    /// to get the heat transfer overall, multiply by number of tubes 
    /// if given an overall mass flowrate and one wants to find 
    /// the mass flowrate through one tube, then divide by number 
    /// of tubes (the u32 value)
    ParallelUniformFluidArray(FluidArray,u32),
}

impl FluidComponentTrait for FluidComponent {
    fn get_mass_flowrate(&mut self) -> MassRate  {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.get_mass_flowrate()
            },
            // get mass flowrate on a total basis 
            //
            // this is important because we use 
            //
            // Re = (mass flowrate)/(cross sectional area) * DH/mu
            //
            // we have (mass flowrate)/(cross sectional area)
            // being the same whether for one tube or for 12 tubes (or any 
            // number of tubes)
            //
            // mu is the same whether for the tube bundle or individual 
            // tubes (assumed)
            //
            // DH (hydraulic diameter) is the 
            // same whether for one tube or 12 tubes (see calculation)
            //
            // Therefore Re is the same for one tube or 12 tubes (or  
            // any n number of tubes)
            //
            //
            FluidComponent::ParallelUniformFluidArray(
                fluid_array, number_of_tubes) => {
                // get the mass flowrate through one tube first 

                let flow_through_one_tube = fluid_array.get_mass_flowrate();
                // then total mass flowrate is multiplied by 
                // number of tubes 

                return flow_through_one_tube * (*number_of_tubes as f64);

            },
        }
    }

    fn set_mass_flowrate(&mut self, mass_flowrate: MassRate) {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.set_mass_flowrate(mass_flowrate);
            },
            // set mass flowrate on a total basis 
            //
            // this is important because we use 
            //
            // Re = (mass flowrate)/(cross sectional area) * DH/mu
            //
            // we have (mass flowrate)/(cross sectional area)
            // being the same whether for one tube or for 12 tubes (or any 
            // number of tubes)
            //
            // mu is the same whether for the tube bundle or individual 
            // tubes (assumed)
            //
            // DH (hydraulic diameter) is the 
            // same whether for one tube or 12 tubes (see calculation)
            //
            // Therefore Re is the same for one tube or 12 tubes (or  
            // any n number of tubes)
            //
            //
            FluidComponent::ParallelUniformFluidArray(
                fluid_array, number_of_tubes) => {

                let flow_through_all_tubes = mass_flowrate;

                let flow_through_one_tube = 
                    flow_through_all_tubes / (*number_of_tubes as f64);

                fluid_array.set_mass_flowrate(flow_through_one_tube);

            },
        }
    }

    fn get_mass_flowrate_from_pressure_loss_immutable(
        &self, pressure_loss: Pressure) -> MassRate {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.get_mass_flowrate_from_pressure_loss_immutable(pressure_loss)
            },
            // get mass flowrate on a total basis based on a fixed pressure 
            // loss
            //
            // this is important because we use 
            //
            // Re = (mass flowrate)/(cross sectional area) * DH/mu
            //
            // we have (mass flowrate)/(cross sectional area)
            // being the same whether for one tube or for 12 tubes (or any 
            // number of tubes)
            //
            // mu is the same whether for the tube bundle or individual 
            // tubes (assumed)
            //
            // DH (hydraulic diameter) is the 
            // same whether for one tube or 12 tubes (see calculation)
            //
            // Therefore Re is the same for one tube or 12 tubes (or  
            // any n number of tubes)
            //
            FluidComponent::ParallelUniformFluidArray(
                fluid_array, number_of_tubes) => {

                let flow_through_one_tube = 
                    fluid_array
                    .get_mass_flowrate_from_pressure_loss_immutable(
                        pressure_loss);

                let flow_through_all_tubes = 
                    flow_through_one_tube *
                    (*number_of_tubes as f64);

                flow_through_all_tubes


            },
        }
    }

    fn get_pressure_loss(&mut self) -> Pressure {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.get_pressure_loss()
            },
            // get pressure loss based on a set mass flowrate
            //
            FluidComponent::ParallelUniformFluidArray(fluid_array, _) => {
                // since components are in parallel, pressure loss is the 
                // same for each tube
                fluid_array.get_pressure_loss()
            },
        }
    }

    fn set_pressure_loss(&mut self, pressure_loss: Pressure) {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.set_pressure_loss(pressure_loss)
            },
            FluidComponent::ParallelUniformFluidArray(fluid_array, _) => {
                // since components are in parallel, pressure loss is the 
                // same for each tube
                fluid_array.set_pressure_loss(pressure_loss)
            },
        }
    }

    fn get_pressure_loss_immutable(
        &self, mass_flowrate: MassRate) -> Pressure {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.get_pressure_loss_immutable(mass_flowrate)
            },
            FluidComponent::ParallelUniformFluidArray(
                fluid_array, number_of_tubes) => {
                let flow_through_all_tubes = mass_flowrate;
                let flow_through_one_tube = flow_through_all_tubes 
                    / (*number_of_tubes as f64);

                fluid_array.get_pressure_loss_immutable(flow_through_one_tube)
            },
        }
    }

    fn get_cross_sectional_area(&mut self) -> Area {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.get_cross_sectional_area()
            },
            // cross sectional area is on a total basis
            //
            // this is important because we use 
            //
            // Re = (mass flowrate)/(cross sectional area) * DH/mu
            //
            // we have (mass flowrate)/(cross sectional area)
            // being the same whether for one tube or for 12 tubes (or any 
            // number of tubes)
            //
            // mu is the same whether for the tube bundle or individual 
            // tubes (assumed)
            //
            // DH (hydraulic diameter) is the 
            // same whether for one tube or 12 tubes (see calculation)
            //
            // Therefore Re is the same for one tube or 12 tubes (or  
            // any n number of tubes)
            //
            FluidComponent::ParallelUniformFluidArray(
                fluid_array, number_of_tubes) => {
                let xs_area_one_tube = 
                    fluid_array.get_cross_sectional_area();

                xs_area_one_tube * (*number_of_tubes as f64)
            },
        }
    }

    fn get_cross_sectional_area_immutable(&self) -> Area {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.get_cross_sectional_area_immutable()
            },
            // cross sectional area is on a total basis
            FluidComponent::ParallelUniformFluidArray(
                fluid_array, number_of_tubes) => {
                let xs_area_one_tube = 
                    fluid_array.get_cross_sectional_area_immutable();

                xs_area_one_tube * (*number_of_tubes as f64)
            },
        }
    }

    fn get_hydraulic_diameter(&mut self) -> Length {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.get_hydraulic_diameter()
            },
            // hydraulic diameter is on a total basis
            // this is important because we use 
            //
            // Re = (mass flowrate)/(cross sectional area) * DH/mu
            //
            // we have (mass flowrate)/(cross sectional area)
            // being the same whether for one tube or for 12 tubes (or any 
            // number of tubes)
            //
            // mu is the same whether for the tube bundle or individual 
            // tubes (assumed)
            //
            // DH (hydraulic diameter) is the 
            // same whether for one tube or 12 tubes (see calculation)
            //
            // Therefore Re is the same for one tube or 12 tubes (or  
            // any n number of tubes)
            //
            FluidComponent::ParallelUniformFluidArray(
                fluid_array, number_of_tubes) => {
                let hydraulic_diameter_one_tube 
                    = fluid_array.get_hydraulic_diameter();

                let cross_sectional_area_one_tube: Area = 
                    fluid_array.get_cross_sectional_area();
                // D_H = 4A/P 
                // where P is the wetted perimeter of one tube 
                let wetted_perimeter_one_tube: Length
                    = 4.0 * fluid_array.get_cross_sectional_area()/
                    hydraulic_diameter_one_tube;

                let total_xs_area: Area = 
                    cross_sectional_area_one_tube *
                    (*number_of_tubes as f64);


                let total_wetted_perimeter: Length = 
                    wetted_perimeter_one_tube * 
                    (*number_of_tubes as f64);

                let hydraulic_diameter_overall: Length 
                    = total_xs_area / total_wetted_perimeter;

                hydraulic_diameter_overall

            },
        }
    }

    fn get_hydraulic_diameter_immutable(&self) -> Length {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.get_hydraulic_diameter_immutable()
            },
            // hydraulic diameter is on a total basis
            // this is important because we use 
            //
            // Re = (mass flowrate)/(cross sectional area) * DH/mu
            //
            // we have (mass flowrate)/(cross sectional area)
            // being the same whether for one tube or for 12 tubes (or any 
            // number of tubes)
            //
            // mu is the same whether for the tube bundle or individual 
            // tubes (assumed)
            //
            // DH (hydraulic diameter) is the 
            // same whether for one tube or 12 tubes (see calculation)
            //
            // Therefore Re is the same for one tube or 12 tubes (or  
            // any n number of tubes)
            //
            FluidComponent::ParallelUniformFluidArray(
                fluid_array, number_of_tubes) => {
                let hydraulic_diameter_one_tube 
                    = fluid_array.get_hydraulic_diameter_immutable();

                let cross_sectional_area_one_tube: Area = 
                    fluid_array.get_cross_sectional_area_immutable();
                // D_H = 4A/P 
                // where P is the wetted perimeter of one tube 
                let wetted_perimeter_one_tube: Length
                    = 4.0 * fluid_array.get_cross_sectional_area_immutable()/
                    hydraulic_diameter_one_tube;

                let total_xs_area: Area = 
                    cross_sectional_area_one_tube *
                    (*number_of_tubes as f64);


                let total_wetted_perimeter: Length = 
                    wetted_perimeter_one_tube * 
                    (*number_of_tubes as f64);

                let hydraulic_diameter_overall: Length 
                    = total_xs_area / total_wetted_perimeter;

                hydraulic_diameter_overall

            },
        }
    }

    fn get_fluid_viscosity_at_ref_temperature(&mut self) -> DynamicViscosity {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.get_fluid_viscosity()
            },
            FluidComponent::ParallelUniformFluidArray(fluid_array,_) => {
                fluid_array.get_fluid_viscosity()
            },
        }
    }

    fn get_fluid_viscosity_immutable_at_ref_temperature(&self) -> DynamicViscosity {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.get_fluid_viscosity_immutable()
            },
            FluidComponent::ParallelUniformFluidArray(fluid_array,_) => {
                fluid_array.get_fluid_viscosity_immutable()
            },
        }
    }

    fn get_fluid_density_at_ref_temperature(&mut self) -> MassDensity {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.get_fluid_density()
            },
            FluidComponent::ParallelUniformFluidArray(fluid_array,_) => {
                fluid_array.get_fluid_density()
            },
        }
    }

    fn get_fluid_density_immutable_at_ref_temperature(&self) -> MassDensity {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.get_fluid_density_immutable()
            },
            FluidComponent::ParallelUniformFluidArray(fluid_array,_) => {
                fluid_array.get_fluid_density_immutable()
            },
            
        }
    }

    fn get_component_length(&mut self) -> Length {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.get_component_length()
            },
            // component lengths are same whether overall or individual
            FluidComponent::ParallelUniformFluidArray(fluid_array,_) => {
                fluid_array.get_component_length()
            },
        }
    }

    fn get_component_length_immutable(&self) -> Length {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.get_component_length_immutable()
            },
            // component lengths are same whether overall or individual
            FluidComponent::ParallelUniformFluidArray(fluid_array,_) => {
                fluid_array.get_component_length_immutable()
            },
        }
    }

    fn get_incline_angle(&mut self) -> Angle {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.get_incline_angle()
            },
            FluidComponent::ParallelUniformFluidArray(fluid_array,_) => {
                fluid_array.get_incline_angle()
            },
        }
    }

    fn get_incline_angle_immutable(&self) -> Angle {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.get_incline_angle_immutable()
            },
            FluidComponent::ParallelUniformFluidArray(fluid_array,_) => {
                fluid_array.get_incline_angle_immutable()
            },
        }
    }

    fn get_internal_pressure_source(&mut self) -> Pressure {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.get_internal_pressure_source()
            },
            // getting or 
            // setting one pressure source is the same as setting all of them
            FluidComponent::ParallelUniformFluidArray(fluid_array,_) => {
                fluid_array.get_internal_pressure_source()
            },
        }
    }

    fn get_internal_pressure_source_immutable(&self) -> Pressure {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.get_internal_pressure_source_immutable()
            },
            // getting or 
            // setting one pressure source is the same as setting all of them
            FluidComponent::ParallelUniformFluidArray(fluid_array,_) => {
                fluid_array.get_internal_pressure_source_immutable()
            },
        }
    }

    fn set_internal_pressure_source(
        &mut self,
        internal_pressure: Pressure) {
        match self {
            FluidComponent::FluidArray(fluid_array) => {
                fluid_array.set_internal_pressure_source(internal_pressure)
            },
            // getting or 
            // setting one pressure source is the same as setting all of them
            FluidComponent::ParallelUniformFluidArray(fluid_array,_) => {
                fluid_array.set_internal_pressure_source(internal_pressure)
            },
        }
    }
}
