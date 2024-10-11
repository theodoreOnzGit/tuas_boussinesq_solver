use std::f64::consts::PI;


use super::heat_transfer_interaction_enums::HeatTransferInteractionType;
use uom::si::f64::*;

use crate::boussinesq_thermophysical_properties::density::try_get_rho;
use crate::boussinesq_thermophysical_properties::specific_enthalpy::try_get_h;
use crate::boussinesq_thermophysical_properties::thermal_diffusivity::try_get_alpha_thermal_diffusivity;
use crate::boussinesq_thermophysical_properties::Material;
use crate::heat_transfer_correlations::heat_transfer_interactions::*;
use crate::single_control_vol::SingleCVNode;
use crate::thermal_hydraulics_error::TuasLibError;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::DataAdvection;

use uom::num_traits::Zero;



/// calculates interaction for a single cv and a constant temperature BC
#[inline]
pub fn calculate_single_cv_node_front_constant_temperature_back(
    boundary_condition_temperature: ThermodynamicTemperature,
    control_vol: &mut SingleCVNode,
    interaction: HeatTransferInteractionType) -> Result<(), TuasLibError> {
    // this code is pretty crappy but I'll match advection first

    match interaction {
        HeatTransferInteractionType::Advection(
        advection_dataset) => {

                control_vol.calculate_cv_front_bc_back_advection_set_temperature(
                    boundary_condition_temperature, 
                    advection_dataset)?;
                return Ok(());
            },
        _ => (),
    }

    // if anything else, use conductance

    control_vol.calculate_single_cv_node_constant_temperature_conductance(
        boundary_condition_temperature,
        interaction)?;

    return Ok(());
}
/// calculates the interaction between a heat flux BC and 
/// a control volume 
///
/// (heat flux bc) ------------------ (single cv)
///
/// the cv is at the front 
/// heat addition is at the back
pub fn calculate_single_cv_front_heat_flux_back(
    heat_flux_into_control_vol: HeatFluxDensity,
    control_vol: &mut SingleCVNode,
    interaction: HeatTransferInteractionType) -> Result<(), TuasLibError> {

    // first, obtain a heat transfer area from the constant heat flux 
    // BC
    let heat_transfer_area: Area = match interaction{
        HeatTransferInteractionType::
            UserSpecifiedThermalConductance(_) => 
            {
                println!("please specify interaction type as \n 
                UserSpecifiedHeatFluxCustomArea or Similar");
                return Err(TuasLibError::WrongHeatTransferInteractionType);
            }
        ,

        HeatTransferInteractionType::
            SingleCartesianThermalConductanceOneDimension(_, _) => 
            {
                println!("please specify interaction type as \n 
                UserSpecifiedHeatFluxCustomArea or Similar");
                return Err(TuasLibError::WrongHeatTransferInteractionType);
            }
        ,

        HeatTransferInteractionType::
            DualCartesianThermalConductance(_, _) => 
            {
                println!("please specify interaction type as \n 
                UserSpecifiedHeatFluxCustomArea or Similar");
                return Err(TuasLibError::WrongHeatTransferInteractionType);
            }
        ,
        HeatTransferInteractionType::SimpleRadiation
            (_area_coeff, _hot_temperature, _cold_temperature) => 
            {
                println!("please specify interaction type as \n 
                UserSpecifiedHeatFluxCustomArea or Similar");
                return Err(TuasLibError::WrongHeatTransferInteractionType);
            }
        ,

        HeatTransferInteractionType::
            DualCylindricalThermalConductance(_, _, _) => 
            {
                println!("please specify interaction type as \n 
                UserSpecifiedHeatFluxCustomArea or Similar");
                return Err(TuasLibError::WrongHeatTransferInteractionType);
            }
        ,

        HeatTransferInteractionType::
            CylindricalConductionConvectionLiquidOutside(_, _) => 
            {
                println!("please specify interaction type as \n 
                UserSpecifiedHeatFluxCustomArea or Similar");
                return Err(TuasLibError::WrongHeatTransferInteractionType);
            }
        ,

        HeatTransferInteractionType::
            CylindricalConductionConvectionLiquidInside(_, _) => 
            {
                println!("please specify interaction type as \n 
                UserSpecifiedHeatFluxCustomArea or Similar");
                return Err(TuasLibError::WrongHeatTransferInteractionType);
            }
        ,

        HeatTransferInteractionType::
            UserSpecifiedHeatAddition => 
            {
                println!("please specify interaction type as \n 
                UserSpecifiedHeatFluxCustomArea or Similar");
                return Err(TuasLibError::WrongHeatTransferInteractionType);
            }
        ,

        HeatTransferInteractionType::
            DualCartesianThermalConductanceThreeDimension(_) => 
            {
                println!("please specify interaction type as \n 
                UserSpecifiedHeatFluxCustomArea or Similar");
                return Err(TuasLibError::WrongHeatTransferInteractionType);
            }
        ,
        // these interaction types are acceptable
        HeatTransferInteractionType::
            UserSpecifiedHeatFluxCustomArea(area) => area,

        HeatTransferInteractionType::
            UserSpecifiedHeatFluxCylindricalOuterArea(
            cylinder_length, od) => {
                let od: Length = od.into();
                let cylinder_length: Length  = cylinder_length.into();

                let area = PI * od * cylinder_length;
                area
            },

        HeatTransferInteractionType::
            UserSpecifiedHeatFluxCylindricalInnerArea(
            cylinder_length, id) => {
                let id: Length = id.into();
                let cylinder_length: Length  = cylinder_length.into();

                let area = PI * id * cylinder_length;
                area

            },

        HeatTransferInteractionType::
            UserSpecifiedConvectionResistance(_) => 
            {
                println!("please specify interaction type as \n 
                UserSpecifiedHeatFluxCustomArea or Similar");
                return Err(TuasLibError::WrongHeatTransferInteractionType);
            }
        HeatTransferInteractionType::
            Advection(advection_data) => 
            {
                calculate_cv_front_bc_back_advection_for_heat_flux_or_heat_addition(
                    control_vol,
                    advection_data)?;

                return Ok(());
            },
    };

    let heat_flowrate_into_control_vol: Power = 
    heat_flux_into_control_vol * heat_transfer_area;

    control_vol.rate_enthalpy_change_vector.
        push(heat_flowrate_into_control_vol);

    // auto time stepping doesn't work for constant heat flux 
    // or specified power as well. 
    // it is best to see at the end of all power calculations what 
    // is the temperature change

    // For liquid CV, still need to calculate time scale based 
    // on convection flow
    // match statement is meant to tell that liquid CVs are not quite 
    // ready for use
    //
    // Actually, for solid CV, I will also need to recalculate time scale 
    // based on the material thermal thermal_diffusivity
    let cv_material = control_vol.material_control_volume;
    match cv_material {
        Material::Solid(_) => {
            // in this case, we just have one cv and one bc 
            // so we only consider thermal inertia of this cv 
            calculate_mesh_stability_conduction_timestep_for_single_node_and_bc(
                control_vol,
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

/// calculates the interaction between a heat addition BC and 
/// a control volume 
///
/// (single cv) ------------------ (heat addition bc)
///
/// the heat addition is at the front, the cv is at the back
#[inline]
pub fn calculate_constant_heat_addition_front_single_cv_back(
    control_vol: &mut SingleCVNode,
    heat_added_to_control_vol: Power,
    interaction: HeatTransferInteractionType
    ) -> Result<(), TuasLibError> {

    // ensure that the interaction is UserSpecifiedHeatAddition
    // or advection
    // otherwise, return error 

    match interaction {

        HeatTransferInteractionType::UserSpecifiedHeatAddition => {
            // return a void value, that would be dropped 
            // instantly
            //
            // it pretty much has the same meaning as break
            ()
        },
        HeatTransferInteractionType::Advection(advection_data) => {
            calculate_bc_front_cv_back_advection_for_heat_flux_or_heat_addition(
                control_vol,
                advection_data)?;

            return Ok(());
            
        },


        _ => {
            println!("you need to specify that the interaction type \n 
            is UserSpecifiedHeatAddition");
            return Err(TuasLibError::WrongHeatTransferInteractionType);

        },
    };


    control_vol.rate_enthalpy_change_vector.
        push(heat_added_to_control_vol);

    // auto time stepping doesn't work for constant heat flux 
    // or specified power as well. 
    // it is best to see at the end of all power calculations what 
    // is the temperature change
    //
    // For liquid CV, still need to calculate time scale based 
    // on convection flow
    // match statement is meant to tell that liquid CVs are not quite 
    // ready for use
    // Actually, for solid CV, I will also need to recalculate time scale 
    // based on the material thermal thermal_diffusivity
    


    let cv_material = control_vol.material_control_volume;
    match cv_material {
        Material::Solid(_) => {
            // in this case, we just have one cv and one bc 
            // so we only consider thermal inertia of this cv 
            calculate_mesh_stability_conduction_timestep_for_single_node_and_bc(
                control_vol,
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

/// calculates the interaction between a heat addition BC and 
/// a control volume 
///
/// (heat addition) ------------------ (single cv)
///
/// the heat addition is at the front, the cv is at the back
#[inline]
pub fn calculate_single_cv_front_constant_heat_addition_back(
    heat_added_to_control_vol: Power,
    control_vol: &mut SingleCVNode,
    interaction: HeatTransferInteractionType
    ) -> Result<(), TuasLibError> {

    // ensure that the interaction is UserSpecifiedHeatAddition
    // or advection
    // otherwise, return error 

    match interaction {

        HeatTransferInteractionType::UserSpecifiedHeatAddition => {
            // return a void value, that would be dropped 
            // instantly
            //
            // it pretty much has the same meaning as break
            ()
        },
        HeatTransferInteractionType::Advection(advection_data) => {
            calculate_cv_front_bc_back_advection_for_heat_flux_or_heat_addition(
                control_vol,
                advection_data)?;

            return Ok(());

        },

        _ => {
            println!("you need to specify that the interaction type \n 
            is UserSpecifiedHeatAddition");
            return Err(TuasLibError::WrongHeatTransferInteractionType);

        },
    };


    control_vol.rate_enthalpy_change_vector.
        push(heat_added_to_control_vol);

    // auto time stepping doesn't work for constant heat flux 
    // or specified power as well. 
    // it is best to see at the end of all power calculations what 
    // is the temperature change
    //
    // For liquid CV, still need to calculate time scale based 
    // on convection flow
    // match statement is meant to tell that liquid CVs are not quite 
    // ready for use
    // Actually, for solid CV, I will also need to recalculate time scale 
    // based on the material thermal thermal_diffusivity
    


    let cv_material = control_vol.material_control_volume;
    match cv_material {
        Material::Solid(_) => {
            // in this case, we just have one cv and one bc 
            // so we only consider thermal inertia of this cv 
            calculate_mesh_stability_conduction_timestep_for_single_node_and_bc(
                control_vol,
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

/// calculates the relevant timestep for stability based on mesh size 
/// between control volume and boundary conditions
#[inline]
pub fn calculate_mesh_stability_conduction_timestep_for_single_node_and_bc(
    control_vol: &mut SingleCVNode,
    interaction: HeatTransferInteractionType) -> Result<Time,TuasLibError> {

    // here we have timestep based on the generic lengthscale of the 
    // control volume 
    let mut cv_timestep:Time = 
    control_vol.calculate_conduction_timestep()?;

    // we may have other time scales based on differing length scales 
    // of the control volume 
    //
    // so we will need to calculate time scales based on these other 
    // length scales and then calculate each of their own time scales.
    // if shorter, then we need to append it to the respective control 
    // volumes

    let cv_material = control_vol.material_control_volume.clone();
    let cv_pressure = control_vol.pressure_control_volume.clone();
    let cv_temperature = control_vol.temperature;

    
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
            },
        HeatTransferInteractionType::
            UserSpecifiedThermalConductance(_) => {

                // if a conductance is specified, don't 
                // do anything

            },

        HeatTransferInteractionType::
            UserSpecifiedHeatFluxCustomArea(area) => {
                // when a normal area is given,
                // we can calculate volume to area ratio 

                let cv_volume = control_vol.volume.clone();

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

            },

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
            },

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
            },

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
            },


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

            },

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

            },

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

            },

        HeatTransferInteractionType::
            UserSpecifiedHeatFluxCylindricalOuterArea(l, od) => {

                // this is treated like a custom area kind of thing 
                // so we calculate the area first

                let cylinder_length: Length = l.into();
                let outer_diameter: Length = od.into();

                let area: Area = PI * outer_diameter * cylinder_length;

                // and then do the boilerplate code

                let cv_volume = control_vol.volume.clone();

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
            },

        HeatTransferInteractionType::
            UserSpecifiedHeatFluxCylindricalInnerArea(l, id) => {

                // this is treated like a custom area kind of thing 
                // so we calculate the area first

                let cylinder_length: Length = l.into();
                let inner_diameter: Length = id.into();

                let area: Area = PI * inner_diameter * cylinder_length;

                // and then do the boilerplate code

                let cv_volume = control_vol.volume.clone();

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

                
            },

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
        },

        HeatTransferInteractionType::SimpleRadiation
            (_area_coeff, _hot_temperature, _cold_temperature) => 
            {
                // radiation can be construed as a conduction 
                // process if the optical thickness is thick enough 
                // but I'm not implementing auto timestepping for radiation 
                // until further notice 
                ()
            }
        ,
    }


    control_vol.max_timestep_vector.push(cv_timestep);

    return Ok(cv_timestep);

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
/// 
#[inline]
pub (crate) fn calculate_cv_front_bc_back_advection_for_heat_flux_or_heat_addition(
    control_vol: &mut SingleCVNode,
    advection_data: DataAdvection
) -> Result<(), TuasLibError>{

    let mass_flow_from_bc_to_cv = advection_data.mass_flowrate;

    let specific_enthalpy_cv: AvailableEnergy = 
    control_vol.current_timestep_control_volume_specific_enthalpy;

    // for the constant temperature bc, 
    // I'll just assume the fluid flowing from the bc is the same as the 
    // fluid in the cv
    //
    // and the pressure is same as the cv

    let control_vol_material = control_vol.material_control_volume;
    let control_vol_pressure = control_vol.pressure_control_volume;
    let control_vol_temperature = control_vol.temperature;

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
    control_vol.rate_enthalpy_change_vector.
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
    control_vol.volumetric_flowrate_vector.push(
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
///
#[inline]
pub (crate) fn calculate_bc_front_cv_back_advection_for_heat_flux_or_heat_addition(
    control_vol: &mut SingleCVNode,
    advection_data: DataAdvection
) -> Result<(), TuasLibError>{

    let mass_flow_from_cv_to_bc = advection_data.mass_flowrate;

    let specific_enthalpy_cv: AvailableEnergy = 
    control_vol.current_timestep_control_volume_specific_enthalpy;

    // for the constant temperature bc, 
    // I'll just assume the fluid flowing from the bc is the same as the 
    // fluid in the cv
    //
    // and the pressure is same as the cv

    let control_vol_material = control_vol.material_control_volume;
    let control_vol_pressure = control_vol.pressure_control_volume;
    let control_vol_temperature = control_vol.temperature;

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
    control_vol.rate_enthalpy_change_vector.
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
    control_vol.volumetric_flowrate_vector.push(
        -volumetric_flowrate);


    Ok(())
}
