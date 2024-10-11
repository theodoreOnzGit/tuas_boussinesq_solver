
use std::f64::consts::PI;

use uom::si::f64::*;
use uom::si::thermodynamic_temperature::kelvin;


use crate::boussinesq_thermophysical_properties::thermal_diffusivity::try_get_alpha_thermal_diffusivity;
use crate::boussinesq_thermophysical_properties::Material;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::single_control_vol::SingleCVNode;
use crate::thermal_hydraulics_error::TuasLibError;

impl SingleCVNode {
    /// calculates a conductance interaction between the constant 
    /// temperature bc and cv
    ///
    /// for conductance, orientation of bc and cv does not usually matter
    /// 
    #[inline]
    pub fn calculate_single_cv_node_constant_temperature_conductance(
        &mut self,
        boundary_condition_temperature: ThermodynamicTemperature,
        interaction: HeatTransferInteractionType) -> Result<(), TuasLibError> {
        // first let's get the control volume temperatures out


        let cv_material = self.material_control_volume;

        let cv_pressure = self.pressure_control_volume;

        let cv_temperature = self.temperature;

        // for now we assume the boundary condition pressure is the same 
        // as the control volume pressure, because pressure is not 
        // specified or anything

        let bc_pressure = cv_pressure;




        // we'll need thermal conductance otherwise 
        let cv_bc_conductance: ThermalConductance = 
            interaction.get_thermal_conductance_based_on_interaction(
                cv_temperature, 
                boundary_condition_temperature, 
                cv_pressure, 
                bc_pressure, )?;

        // with conductance settled, we should be able to calculate a power 

        let bc_temp_minus_cv_temp_kelvin: f64 = 
            boundary_condition_temperature.get::<kelvin>() - 
            cv_temperature.get::<kelvin>();

        let bc_temp_mins_cv_temp: TemperatureInterval = 
            TemperatureInterval::new::<uom::si::temperature_interval::kelvin>(
                bc_temp_minus_cv_temp_kelvin);

        // heat flow to the destination 
        // be is proportional to -(T_final - T_initial) 
        // or -(T_destination - T_source)
        //
        // so if the destination is the boundary condition,
        //
        let heat_flowrate_from_cv_to_bc: Power = 
            - cv_bc_conductance * bc_temp_mins_cv_temp;

        // now, push the power change or heat flowrate 
        // to the control volume 
        //

        self.rate_enthalpy_change_vector.
            push(-heat_flowrate_from_cv_to_bc);

        // for constant temperature BC in interaction with CV,
        // we only need take into consideration the CV 
        // timescale, this is based on the fourier number or Nusselt 
        // number edited fourier number
        //
        // for solids mesh fourier number need only 
        // be done once, not every time 
        // an interaction is formed 
        //
        // probably the cell stability fourier number will be done in the 
        // constructor. however, with convection, the time scale must be 
        // recalculated at every time step. so it really depends whether 
        // it's solid or fluid control volume
        // 

        // Actually, for solid CV, I will also need to recalculate time scale 
        // based on the material thermal thermal_diffusivity
        // match statement is meant to tell that liquid CVs are not quite 
        // ready for use
        match cv_material {
            Material::Solid(_) => {
                // in this case, we just have one cv and one bc 
                // so we only consider thermal inertia of this cv 
                Self::calculate_mesh_stability_conduction_timestep_for_single_node_and_bc(
                    self,
                    interaction)?;
                ()
            },
            Material::Liquid(_) => {
                // liquid time scales should be calculated using courant 
                // number at the end of each timestep after volumetric flows 
                // in and out of the cv are calculated
                ()
            },
        }
        return Ok(());
    }

    /// for conduction between a control volume and a boundary condition,
    /// there is a certain threshold timestep based on mesh fourier number that 
    /// if exceeded, will cause numerical instability 
    /// 
    /// this function allows for calculation of this threshold times step
    pub fn calculate_mesh_stability_conduction_timestep_for_single_node_and_bc(
        &mut self,
        interaction: HeatTransferInteractionType) -> Result<Time,TuasLibError> {

        // here we have timestep based on the generic lengthscale of the 
        // control volume 
        let mut cv_timestep:Time = 
            self.calculate_conduction_timestep()?;

        // we may have other time scales based on differing length scales 
        // of the control volume 
        //
        // so we will need to calculate time scales based on these other 
        // length scales and then calculate each of their own time scales.
        // if shorter, then we need to append it to the respective control 
        // volumes

        let cv_material = self.material_control_volume.clone();
        let cv_pressure = self.pressure_control_volume.clone();
        let cv_temperature = self.temperature;


        let cv_alpha: DiffusionCoefficient = 
            try_get_alpha_thermal_diffusivity(cv_material,
                cv_temperature,
                cv_pressure)?;


        let max_mesh_fourier_number: f64 = 0.25;


        match interaction {
            HeatTransferInteractionType::
                UserSpecifiedHeatAddition => {

                    // do nothing

                    ()
                }
            ,
            HeatTransferInteractionType::
                UserSpecifiedThermalConductance(_) => {

                    // if a conductance is specified, don't 
                    // do anything

                }
            ,

            HeatTransferInteractionType::
                UserSpecifiedHeatFluxCustomArea(area) => {
                    // when a normal area is given,
                    // we can calculate volume to area ratio 

                    let cv_volume = self.volume.clone();

                    let volume_to_area_ratio: Length = cv_volume/area;

                    // we can calculate a timestep

                    let time_step_max_based_on_volume_to_area: Time 
                        = max_mesh_fourier_number *
                        volume_to_area_ratio * 
                        volume_to_area_ratio / 
                        cv_alpha;

                    // if the max timestep is shorter than this calculated 
                    // cv timestep, use it

                    if cv_timestep > time_step_max_based_on_volume_to_area {
                        cv_timestep = time_step_max_based_on_volume_to_area;
                    }

                }
            ,

            HeatTransferInteractionType::
                SingleCartesianThermalConductanceOneDimension(
                    material, x_thickness) => {

                    // the given material here overrides the normal 
                    // material 
                    let cv_alpha: DiffusionCoefficient = 
                        try_get_alpha_thermal_diffusivity(material,
                                                          cv_temperature,
                                                          cv_pressure)?;

                    // if we connect the cv to a boundary condition,
                    // then the length provided here is what we need 
                    // to bother with 

                    let lengthscale: Length = x_thickness.into();

                    let time_step_max_based_on_x_thickness: Time 
                        = max_mesh_fourier_number *
                        lengthscale * 
                        lengthscale / 
                        cv_alpha;

                    // if the max timestep is shorter than this calculated 
                    // cv timestep, use it

                    if cv_timestep > time_step_max_based_on_x_thickness {
                        cv_timestep = time_step_max_based_on_x_thickness;
                    }
                }
            ,

            HeatTransferInteractionType::
                DualCartesianThermalConductance(
                    (material_1, length_1), 
                    (material_2,length_2)) => {
                    // for a single node connected to a BC, you're 
                    // not really supposed to have a timescale 
                    // based on two or more lengths
                    //
                    // you only have one control volume bascially,
                    // and you should only use dual cartesian thermal 
                    // conductance for two control volumes
                    // I won't do anything based on this 
                    // or just use the generic timestep
                    //
                    // the other consideration is to take the shorter of 
                    // the two time steps and put it into the cv timestep 
                    let alpha_1: DiffusionCoefficient = 
                        try_get_alpha_thermal_diffusivity(material_1,
                                                          cv_temperature,
                                                          cv_pressure)?;

                    let alpha_2: DiffusionCoefficient = 
                        try_get_alpha_thermal_diffusivity(material_2,
                                                          cv_temperature,
                                                          cv_pressure)?;

                    let length_1: Length = length_1.into();
                    let length_2: Length = length_2.into();

                    let timestep_1: Time = max_mesh_fourier_number * 
                        length_1 *
                        length_1 / 
                        alpha_1;

                    let timestep_2: Time = max_mesh_fourier_number * 
                        length_2 *
                        length_2 / 
                        alpha_2;

                    if cv_timestep > timestep_1 {
                        cv_timestep = timestep_1;
                    }

                    if cv_timestep > timestep_2 {
                        cv_timestep = timestep_2;
                    }

                    // done!
                    ()
                }
            ,

            HeatTransferInteractionType::
                DualCylindricalThermalConductance(
                    (material_1, radius_1), 
                    (material_2,radius_2), 
                    _) => {
                    // for a single node connected to a BC, you're 
                    // not really supposed to have a timescale 
                    // based on two or more radiuss
                    //
                    // you only have one control volume bascially,
                    // and you should only use dual cartesian thermal 
                    // conductance for two control volumes
                    // I won't do anything based on this 
                    // or just use the generic timestep
                    //
                    // the other consideration is to take the shorter of 
                    // the two time steps and put it into the cv timestep 
                    let alpha_1: DiffusionCoefficient = 
                        try_get_alpha_thermal_diffusivity(material_1,
                                                          cv_temperature,
                                                          cv_pressure)?;

                    let alpha_2: DiffusionCoefficient = 
                        try_get_alpha_thermal_diffusivity(material_2,
                                                          cv_temperature,
                                                          cv_pressure)?;

                    let radius_1: Length = radius_1.into();
                    let radius_2: Length = radius_2.into();

                    let timestep_1: Time = max_mesh_fourier_number * 
                        radius_1 *
                        radius_1 / 
                        alpha_1;

                    let timestep_2: Time = max_mesh_fourier_number * 
                        radius_2 *
                        radius_2 / 
                        alpha_2;

                    if cv_timestep > timestep_1 {
                        cv_timestep = timestep_1;
                    }

                    if cv_timestep > timestep_2 {
                        cv_timestep = timestep_2;
                    }

                    // done!
                    ()
                }
            ,


            HeatTransferInteractionType::
                DualCartesianThermalConductanceThreeDimension(
                    data_dual_cartesian_conduction_data) => {

                    let material_1 = data_dual_cartesian_conduction_data.
                        material_1.clone();

                    let material_2 = data_dual_cartesian_conduction_data.
                        material_2.clone();


                    let length_1 : Length = data_dual_cartesian_conduction_data.
                        thickness_1.clone().into();

                    let length_2 : Length = data_dual_cartesian_conduction_data.
                        thickness_2.clone().into();
                    // for a single node connected to a BC, you're 
                    // not really supposed to have a timescale 
                    // based on two or more radiuss
                    //
                    // you only have one control volume bascially,
                    // and you should only use dual cartesian thermal 
                    // conductance for two control volumes
                    // I won't do anything based on this 
                    // or just use the generic timestep
                    //
                    // the other consideration is to take the shorter of 
                    // the two time steps and put it into the cv timestep 
                    let alpha_1: DiffusionCoefficient = 
                        try_get_alpha_thermal_diffusivity(material_1,
                                                          cv_temperature,
                                                          cv_pressure)?;

                    let alpha_2: DiffusionCoefficient = 
                        try_get_alpha_thermal_diffusivity(material_2,
                                                          cv_temperature,
                                                          cv_pressure)?;


                    let timestep_1: Time = max_mesh_fourier_number * 
                        length_1 *
                        length_1 / 
                        alpha_1;

                    let timestep_2: Time = max_mesh_fourier_number * 
                        length_2 *
                        length_2 / 
                        alpha_2;

                    if cv_timestep > timestep_1 {
                        cv_timestep = timestep_1;
                    }

                    if cv_timestep > timestep_2 {
                        cv_timestep = timestep_2;
                    }

                    // done!
                    ()

                }
            ,

            HeatTransferInteractionType::
                CylindricalConductionConvectionLiquidInside(
                    (material,radius,
                     temperature,pressure),_) => {

                    // for a single node connected to a BC, you're 
                    // not really supposed to have a timescale 
                    // based on two or more lengths
                    //
                    // you only have one control volume bascially,
                    // and you should only use dual cylindrical thermal 
                    // conductance for two control volumes
                    // I won't do anything based on this 
                    // or just use the generic timestep

                    let alpha_1: DiffusionCoefficient = 
                        try_get_alpha_thermal_diffusivity(material,
                                                          temperature,
                                                          pressure)?;

                    let length_1: Length =  radius.into();

                    let length_1 = length_1*0.5;

                    let timestep_1: Time = max_mesh_fourier_number * 
                        length_1 *
                        length_1 / 
                        alpha_1;

                    if cv_timestep > timestep_1 {
                        cv_timestep = timestep_1;
                    }
                    ()
                        // if the control volume is fluid, we will need 
                        // to introduce another time scale

                }
            ,

            HeatTransferInteractionType::
                CylindricalConductionConvectionLiquidOutside(
                    (material,radius,
                     temperature,pressure),_) => {

                    // for a single node connected to a BC, you're 
                    // not really supposed to have a timescale 
                    // based on two or more lengths
                    //
                    // you only have one control volume bascially,
                    // and you should only use dual cylindrical thermal 
                    // conductance for two control volumes
                    // I won't do anything based on this 
                    // or just use the generic timestep

                    let alpha_1: DiffusionCoefficient = 
                        try_get_alpha_thermal_diffusivity(material,
                                                          temperature,
                                                          pressure)?;

                    let length_1: Length =  radius.into();

                    let length_1 = length_1*0.5;

                    let timestep_1: Time = max_mesh_fourier_number * 
                        length_1 *
                        length_1 / 
                        alpha_1;

                    if cv_timestep > timestep_1 {
                        cv_timestep = timestep_1;
                    }
                    ()
                        // if the control volume is fluid, we will need 
                        // to introduce another time scale

                }
            ,

            HeatTransferInteractionType::
                UserSpecifiedHeatFluxCylindricalOuterArea(l, od) => {

                    // this is treated like a custom area kind of thing 
                    // so we calculate the area first

                    let cylinder_length: Length = l.into();
                    let outer_diameter: Length = od.into();

                    let area: Area = PI * outer_diameter * cylinder_length;

                    // and then do the boilerplate code

                    let cv_volume = self.volume.clone();

                    let volume_to_area_ratio: Length = cv_volume/area;

                    // we can calculate a timestep

                    let time_step_max_based_on_volume_to_area: Time 
                        = max_mesh_fourier_number *
                        volume_to_area_ratio * 
                        volume_to_area_ratio / 
                        cv_alpha;

                    // if the max timestep is shorter than this calculated 
                    // cv timestep, use it

                    if cv_timestep > time_step_max_based_on_volume_to_area {
                        cv_timestep = time_step_max_based_on_volume_to_area;
                    }
                }
            ,

            HeatTransferInteractionType::
                UserSpecifiedHeatFluxCylindricalInnerArea(l, id) => {

                    // this is treated like a custom area kind of thing 
                    // so we calculate the area first

                    let cylinder_length: Length = l.into();
                    let inner_diameter: Length = id.into();

                    let area: Area = PI * inner_diameter * cylinder_length;

                    // and then do the boilerplate code

                    let cv_volume = self.volume.clone();

                    let volume_to_area_ratio: Length = cv_volume/area;

                    // we can calculate a timestep

                    let time_step_max_based_on_volume_to_area: Time 
                        = max_mesh_fourier_number *
                        volume_to_area_ratio * 
                        volume_to_area_ratio / 
                        cv_alpha;

                    // if the max timestep is shorter than this calculated 
                    // cv timestep, use it

                    if cv_timestep > time_step_max_based_on_volume_to_area {
                        cv_timestep = time_step_max_based_on_volume_to_area;
                    }


                }
            ,

            HeatTransferInteractionType::
                UserSpecifiedConvectionResistance(_) => {

                    // if a resistance is specified, don't 
                    // do anything

                    ()
                },
            HeatTransferInteractionType::Advection(_) => {
                // advection has nothing to do with conduction timestep 
                // do nothing

                ()
            }
            ,
            HeatTransferInteractionType::
                SimpleRadiation
                (_area_coeff, _hot_temperature, _cold_temperature) => 
                {
                    // while radiation can be treated as conduction 
                    // in optically thick media, im not implementing this yet 
                    ()
                }
            ,

        }


        self.max_timestep_vector.push(cv_timestep);

        return Ok(cv_timestep);

    }
}
