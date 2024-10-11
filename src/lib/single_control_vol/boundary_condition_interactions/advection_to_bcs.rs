use uom::num_traits::Zero;
use uom::si::f64::*;


use crate::boussinesq_thermophysical_properties::density::try_get_rho;
use crate::boussinesq_thermophysical_properties::specific_enthalpy::try_get_h;
use crate::heat_transfer_correlations::heat_transfer_interactions::advection_heat_rate;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::DataAdvection;
use crate::single_control_vol::SingleCVNode;
use crate::thermal_hydraulics_error::TuasLibError;


impl SingleCVNode {
    /// for advection calculations with heat flux or heat addition BC,
    /// the temperature of flows flowing in and out of the BC will be 
    /// determined by that of the control volume
    ///
    /// it will be the same temperature as that of the control volume 
    /// at that current timestep
    ///
    /// this will be quite similar to how OpenFOAM treats inflows and outflows 
    /// at zero gradient BCs
    /// 
    #[inline]
    pub fn calculate_cv_front_bc_back_advection_non_set_temperature(
        &mut self,
        advection_data: DataAdvection
    ) -> Result<(), TuasLibError>{

        let mass_flow_from_bc_to_cv = advection_data.mass_flowrate;

        let specific_enthalpy_cv: AvailableEnergy = 
            self.current_timestep_control_volume_specific_enthalpy;

        // for the constant temperature bc, 
        // I'll just assume the fluid flowing from the bc is the same as the 
        // fluid in the cv
        //
        // and the pressure is same as the cv

        let control_vol_material = self.material_control_volume;
        let control_vol_pressure = self.pressure_control_volume;
        let control_vol_temperature = self.get_temperature_from_enthalpy_and_set()?;

        let specific_enthalpy_bc_zero_gradient: AvailableEnergy = try_get_h(
            control_vol_material,
            control_vol_temperature,
            control_vol_pressure
        )?;
        // calculate heat rate 

        let heat_flowrate_from_bc_to_cv: Power 
            = advection_heat_rate(mass_flow_from_bc_to_cv,
                specific_enthalpy_bc_zero_gradient,
                specific_enthalpy_cv,)?;

        // push to cv
        self.rate_enthalpy_change_vector.
            push(heat_flowrate_from_bc_to_cv);


        let density_cv = advection_data.fluid_density_heat_transfer_entity_2;

        // we also need to calculate bc density 
        // now this doesn't quite work well for compressible flow but for 
        // now I'll let it be

        let density_bc: MassDensity = try_get_rho(
            control_vol_material,
            control_vol_temperature,
            control_vol_pressure
        )?;

        let volumetric_flowrate: VolumeRate;

        if mass_flow_from_bc_to_cv > MassRate::zero() {
            // if mass flowrate is positive, flow is moving from bc 
            // to cv 
            // then the density we use is bc 

            volumetric_flowrate = mass_flow_from_bc_to_cv/density_bc;

        } else {
            // if mass flowrate is positive, flow is moving from cv
            // to bc
            // then the density we use is cv

            volumetric_flowrate = mass_flow_from_bc_to_cv/density_cv;
        }


        // for courant number
        self.volumetric_flowrate_vector.push(
            volumetric_flowrate);


        Ok(())
    }

    /// for advection calculations with heat flux or heat addition BC,
    /// the temperature of flows flowing in and out of the BC will be 
    /// determined by that of the control volume
    ///
    /// it will be the same temperature as that of the control volume 
    /// at that current timestep
    /// 
    /// this will be quite similar to how OpenFOAM treats inflows and outflows 
    /// at zero gradient BCs
    #[inline]
    pub fn calculate_bc_front_cv_back_advection_non_set_temperature(
        &mut self,
        advection_data: DataAdvection
    ) -> Result<(), TuasLibError>{

        let mass_flow_from_cv_to_bc = advection_data.mass_flowrate;

        let specific_enthalpy_cv: AvailableEnergy = 
            self.current_timestep_control_volume_specific_enthalpy;

        // for the constant temperature bc, 
        // I'll just assume the fluid flowing from the bc is the same as the 
        // fluid in the cv
        //
        // and the pressure is same as the cv

        let control_vol_material = self.material_control_volume;
        let control_vol_pressure = self.pressure_control_volume;
        let control_vol_temperature = self.get_temperature_from_enthalpy_and_set()?;

        let specific_enthalpy_bc_zero_gradient: AvailableEnergy = try_get_h(
            control_vol_material,
            control_vol_temperature,
            control_vol_pressure
        )?;
        // calculate heat rate 

        let heat_flowrate_from_bc_to_cv: Power 
            = advection_heat_rate(mass_flow_from_cv_to_bc,
                specific_enthalpy_cv,
                specific_enthalpy_bc_zero_gradient,)?;

        // push to cv
        self.rate_enthalpy_change_vector.
            push(-heat_flowrate_from_bc_to_cv);


        let density_cv = advection_data.fluid_density_heat_transfer_entity_2;

        // we also need to calculate bc density 
        // now this doesn't quite work well for compressible flow but for 
        // now I'll let it be

        let density_bc: MassDensity = try_get_rho(
            control_vol_material,
            control_vol_temperature,
            control_vol_pressure
        )?;

        let volumetric_flowrate: VolumeRate;

        if mass_flow_from_cv_to_bc > MassRate::zero() {
            // if mass flowrate is positive, flow is moving from bc 
            // to cv 
            // then the density we use is bc 

            volumetric_flowrate = mass_flow_from_cv_to_bc/density_bc;

        } else {
            // if mass flowrate is positive, flow is moving from cv
            // to bc
            // then the density we use is cv

            volumetric_flowrate = mass_flow_from_cv_to_bc/density_cv;
        }


        // for courant number
        self.volumetric_flowrate_vector.push(
            -volumetric_flowrate);


        Ok(())
    }


    /// for advection calculations with set temperature BCs
    ///
    /// the temperature of the flow will be set by the control volume or 
    /// BC which is upwind (that is, upstream of the flow)
    /// 
    pub fn calculate_cv_front_bc_back_advection_set_temperature(
        &mut self,
        boundary_condition_temperature: ThermodynamicTemperature,
        advection_data: DataAdvection
    ) -> Result<(), TuasLibError>{


        let mass_flow_from_bc_to_cv = advection_data.mass_flowrate;

        let specific_enthalpy_cv: AvailableEnergy = 
            self.current_timestep_control_volume_specific_enthalpy;

        // for the constant temperature bc, 
        // I'll just assume the fluid flowing from the bc is the same as the 
        // fluid in the cv
        //
        // and the pressure is same as the cv

        let control_vol_material = self.material_control_volume;
        let control_vol_pressure = self.pressure_control_volume;

        let specific_enthalpy_bc: AvailableEnergy = try_get_h(
            control_vol_material,
            boundary_condition_temperature,
            control_vol_pressure
        )?;
        // calculate heat rate 

        let heat_flowrate_from_bc_to_cv: Power 
            = advection_heat_rate(mass_flow_from_bc_to_cv,
                specific_enthalpy_bc,
                specific_enthalpy_cv,)?;

        // push to cv
        self.rate_enthalpy_change_vector.
            push(heat_flowrate_from_bc_to_cv);


        let density_cv = advection_data.fluid_density_heat_transfer_entity_2;

        // we also need to calculate bc density 
        // now this doesn't quite work well for compressible flow but for 
        // now I'll let it be

        let density_bc: MassDensity = try_get_rho(
            control_vol_material,
            boundary_condition_temperature,
            control_vol_pressure
        )?;

        let volumetric_flowrate: VolumeRate;

        if mass_flow_from_bc_to_cv > MassRate::zero() {
            // if mass flowrate is positive, flow is moving from bc 
            // to cv 
            // then the density we use is bc 

            volumetric_flowrate = mass_flow_from_bc_to_cv/density_bc;

        } else {
            // if mass flowrate is positive, flow is moving from cv
            // to bc
            // then the density we use is cv

            volumetric_flowrate = mass_flow_from_bc_to_cv/density_cv;
        }


        // for courant number
        self.volumetric_flowrate_vector.push(
            volumetric_flowrate);


        Ok(())
    }

    /// for advection calculations with set temperature BCs
    ///
    /// the temperature of the flow will be set by the control volume or 
    /// BC which is upwind (that is, upstream of the flow)
    #[inline]
    pub fn calculate_bc_front_cv_back_advection_set_temperature(
        &mut self,
        boundary_condition_temperature: ThermodynamicTemperature,
        advection_data: DataAdvection
        ) -> Result<(), TuasLibError>{

        let mass_flow_from_cv_to_bc = advection_data.mass_flowrate;

        let specific_enthalpy_cv: AvailableEnergy = 
            self.current_timestep_control_volume_specific_enthalpy;

        // for the constant temperature bc, 
        // I'll just assume the fluid flowing from the bc is the same as the 
        // fluid in the cv
        //
        // and the pressure is same as the cv

        let control_vol_material = self.material_control_volume;
        let control_vol_pressure = self.pressure_control_volume;

        let specific_enthalpy_bc: AvailableEnergy = try_get_h(
            control_vol_material,
            boundary_condition_temperature,
            control_vol_pressure
            )?;
        // calculate heat rate 

        let heat_flowrate_from_bc_to_cv: Power 
            = advection_heat_rate(mass_flow_from_cv_to_bc,
                                  specific_enthalpy_cv,
                                  specific_enthalpy_bc,)?;

        // push to cv
        self.rate_enthalpy_change_vector.
            push(-heat_flowrate_from_bc_to_cv);


        let density_cv = advection_data.fluid_density_heat_transfer_entity_2;

        // we also need to calculate bc density 
        // now this doesn't quite work well for compressible flow but for 
        // now I'll let it be

        let density_bc: MassDensity = try_get_rho(
            control_vol_material,
            boundary_condition_temperature,
            control_vol_pressure
            )?;

        let volumetric_flowrate: VolumeRate;

        if mass_flow_from_cv_to_bc > MassRate::zero() {
            // if mass flowrate is positive, flow is moving from bc 
            // to cv 
            // then the density we use is bc 

            volumetric_flowrate = mass_flow_from_cv_to_bc/density_bc;

        } else {
            // if mass flowrate is positive, flow is moving from cv
            // to bc
            // then the density we use is cv

            volumetric_flowrate = mass_flow_from_cv_to_bc/density_cv;
        }


        // for courant number
        self.volumetric_flowrate_vector.push(
            -volumetric_flowrate);


        Ok(())
    }
}
