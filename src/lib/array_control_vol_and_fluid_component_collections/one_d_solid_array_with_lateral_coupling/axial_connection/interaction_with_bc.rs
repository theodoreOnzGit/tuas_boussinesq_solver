use std::f64::consts::PI;

use uom::si::f64::*;
use crate::heat_transfer_correlations::heat_transfer_interactions::heat_transfer_interaction_enums::HeatTransferInteractionType;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::single_control_vol::boundary_condition_interactions::constant_heat_addition_to_bcs::calculate_single_cv_front_constant_heat_addition_back;
use crate::single_control_vol::boundary_condition_interactions::constant_heat_addition_to_bcs::calculate_constant_heat_addition_front_single_cv_back;
use crate::single_control_vol::SingleCVNode;

use crate::tuas_lib_error::TuasLibError;

impl SolidColumn {

    /// attaches an constant heat flux BC to the front of this 
    /// array control volume 
    /// (back --- cv_self --- front) ----- - (constant heat flux)
    pub fn link_heat_flux_bc_to_front_of_this_cv(
        &mut self,
        heat_flux_into_control_vol: HeatFluxDensity,
        interaction: HeatTransferInteractionType) -> Result<(),TuasLibError>{

        // first, obtain a heat transfer area from the constant heat flux 
        // BC
        let heat_transfer_area: Area = match interaction{
            HeatTransferInteractionType::UserSpecifiedThermalConductance(_) => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);
            },

            HeatTransferInteractionType::SingleCartesianThermalConductanceOneDimension(_, _) => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);
            },

            HeatTransferInteractionType::DualCartesianThermalConductance(_, _) => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);
            },

            HeatTransferInteractionType::DualCylindricalThermalConductance(_, _, _) => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);
            },

            HeatTransferInteractionType::CylindricalConductionConvectionLiquidOutside(_, _) => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);
            },

            HeatTransferInteractionType::CylindricalConductionConvectionLiquidInside(_, _) => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);
            },

            HeatTransferInteractionType::UserSpecifiedHeatAddition => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);
            },

            HeatTransferInteractionType::DualCartesianThermalConductanceThreeDimension(_) => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);
            },
            // these interaction types are acceptable
            HeatTransferInteractionType:: UserSpecifiedHeatFluxCustomArea(area) => area,

            HeatTransferInteractionType:: UserSpecifiedHeatFluxCylindricalOuterArea(
                cylinder_length, od) => {
                let od: Length = od.into();
                let cylinder_length: Length  = cylinder_length.into();

                let area = PI * od * cylinder_length;
                area
            },

            HeatTransferInteractionType:: UserSpecifiedHeatFluxCylindricalInnerArea(
                cylinder_length, id) => {
                let id: Length = id.into();
                let cylinder_length: Length  = cylinder_length.into();

                let area = PI * id * cylinder_length;
                area

            },

            HeatTransferInteractionType:: UserSpecifiedConvectionResistance(_) => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);
            },
            HeatTransferInteractionType:: Advection(_) => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);
            },
            HeatTransferInteractionType::SimpleRadiation(_,_,_) => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);

            },
        };

        // once area is calculated, we can calculate heat flowrate into 
        // cv
        let heat_flowrate_into_control_vol: Power = 
        heat_flux_into_control_vol * heat_transfer_area;

        // then push the power to the front control volume, 

        // then push the power to the front control volume, 
        // then just do conduction timescales 
        // There's no advection in this case so no need to worry 

        let back_cv_ref = &mut self.front_single_cv;

        // push the power power to the back cv 

        back_cv_ref.rate_enthalpy_change_vector.push( 
            heat_flowrate_into_control_vol);
        // calculate conduction timescales
        back_cv_ref.calculate_mesh_stability_conduction_timestep_for_single_node_and_bc(
            interaction)?;

        // I don't calculate solid-liquid timescales here, 
        // kind of redundant 
        // I could implement it in future though
        // todo: nusselt number adjusted timescales for heatflux 

        return Ok(());
    }

    /// attaches an constant heat flux BC to the back of this 
    /// array control volume 
    /// (constant heat flux) ---- (back --- cv_self --- front)
    pub fn link_heat_flux_bc_to_back_of_this_cv(
        &mut self,
        heat_flux_into_control_vol: HeatFluxDensity,
        interaction: HeatTransferInteractionType) -> Result<(),TuasLibError>{

        // first, obtain a heat transfer area from the constant heat flux 
        // BC
        let heat_transfer_area: Area = match interaction{
            HeatTransferInteractionType::UserSpecifiedThermalConductance(_) => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);
            },

            HeatTransferInteractionType::SingleCartesianThermalConductanceOneDimension(_, _) => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);
            },

            HeatTransferInteractionType::DualCartesianThermalConductance(_, _) => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);
            },

            HeatTransferInteractionType::DualCylindricalThermalConductance(_, _, _) => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);
            },

            HeatTransferInteractionType::CylindricalConductionConvectionLiquidOutside(_, _) => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);
            },

            HeatTransferInteractionType::CylindricalConductionConvectionLiquidInside(_, _) => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);
            },

            HeatTransferInteractionType::UserSpecifiedHeatAddition => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);
            },

            HeatTransferInteractionType::DualCartesianThermalConductanceThreeDimension(_) => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);
            },
            // these interaction types are acceptable
            HeatTransferInteractionType:: UserSpecifiedHeatFluxCustomArea(area) => area,

            HeatTransferInteractionType:: UserSpecifiedHeatFluxCylindricalOuterArea(
                cylinder_length, od) => {
                let od: Length = od.into();
                let cylinder_length: Length  = cylinder_length.into();

                let area = PI * od * cylinder_length;
                area
            },

            HeatTransferInteractionType:: UserSpecifiedHeatFluxCylindricalInnerArea(
                cylinder_length, id) => {
                let id: Length = id.into();
                let cylinder_length: Length  = cylinder_length.into();

                let area = PI * id * cylinder_length;
                area

            },

            HeatTransferInteractionType:: UserSpecifiedConvectionResistance(_) => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);
            },
            HeatTransferInteractionType:: Advection(_) => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);
            },
            HeatTransferInteractionType::SimpleRadiation(_,_,_) => 
            {
                println!("please specify interaction type as \n 
                    UserSpecifiedHeatFluxCustomArea or Similar");
                return Err( TuasLibError::WrongHeatTransferInteractionType);

            },
        };

        // once area is calculated, we can calculate heat flowrate into 
        // cv
        let heat_flowrate_into_control_vol: Power = 
        heat_flux_into_control_vol * heat_transfer_area;

        // then push the power to the front control volume, 

        // then push the power to the front control volume, 
        // then just do conduction timescales 
        // There's no advection in this case so no need to worry 

        let back_cv_ref = &mut self.back_single_cv;

        // push the power power to the back cv 

        back_cv_ref.rate_enthalpy_change_vector.push( 
            heat_flowrate_into_control_vol);
        // calculate conduction timescales
        back_cv_ref.calculate_mesh_stability_conduction_timestep_for_single_node_and_bc(
            interaction)?;

        // I don't calculate solid-liquid timescales here, 
        // kind of redundant 
        // I could implement it in future though
        // todo: nusselt number adjusted timescales for heatflux 

        return Ok(());
    }

    /// attaches an constant heat rate BC to the front of this 
    /// array control volume 
    /// (back --- cv_self --- front) ---- (constant heat rate)
    pub fn link_heat_addition_to_front_of_this_cv(
        &mut self,
        heat_rate: Power,
        interaction: HeatTransferInteractionType) -> Result<(),TuasLibError> {

        if let HeatTransferInteractionType::Advection(_) = interaction {
            println!("You cannot have advection interactions for Solid Columns");
            return Err(TuasLibError::WrongHeatTransferInteractionType);
        }
        // we need to obtain the single cv from the array cv first 
        // and this will be the front cv or outer cv 
        //

        let single_cv_node_self: &mut SingleCVNode = &mut self.front_single_cv;

        // then link it to the other bc as per normal for 
        // single control volumes

        calculate_constant_heat_addition_front_single_cv_back(
            single_cv_node_self,
            heat_rate,
            interaction
        )

    }

    /// attaches an constant heat rate BC to the front of this 
    /// array control volume 
    /// (constant heat rate) --- (back --- cv_self --- front)
    pub fn link_heat_addition_to_back_of_this_cv(
        &mut self,
        heat_rate: Power,
        interaction: HeatTransferInteractionType) -> Result<(),TuasLibError> {
        if let HeatTransferInteractionType::Advection(_) = interaction {
            println!("You cannot have advection interactions for Solid Columns");
            return Err(TuasLibError::WrongHeatTransferInteractionType);
        }

        // we need to obtain the single cv from the array cv first 
        // and this will be the back cv or inner cv 
        //

        let single_cv_node_self: &mut SingleCVNode = 
        &mut self.back_single_cv;

        calculate_single_cv_front_constant_heat_addition_back(
            heat_rate,
            single_cv_node_self,
            interaction
        )
    }

    /// attaches an constant temperature BC to the front of this 
    /// array control volume 
    /// (back --- cv_self --- front) ---- (constant temperature)
    pub fn link_constant_temperature_to_front_of_this_cv(
        &mut self,
        bc_temperature: ThermodynamicTemperature,
        interaction: HeatTransferInteractionType) -> Result<(),TuasLibError> {
        if let HeatTransferInteractionType::Advection(_) = interaction {
            println!("You cannot have advection interactions for Solid Columns");
            return Err(TuasLibError::WrongHeatTransferInteractionType);
        }
        // we need to obtain the single cv from the array cv first 
        // and this will be the front cv or outer cv 
        //

        let single_cv_node_self: &mut SingleCVNode = 
        &mut self.front_single_cv;


        SingleCVNode::calculate_constant_temperature_front_single_cv_back (
            single_cv_node_self,
            bc_temperature,
            interaction
        )
    }

    /// attaches an constant temperature BC to the front of this 
    /// array control volume 
    /// (constant temperature) --- (back --- cv_self --- front)
    pub fn link_constant_temperature_to_back_of_this_cv(
        &mut self,
        bc_temperature: ThermodynamicTemperature,
        interaction: HeatTransferInteractionType) -> Result<(),TuasLibError> {
        if let HeatTransferInteractionType::Advection(_) = interaction {
            println!("You cannot have advection interactions for Solid Columns");
            return Err(TuasLibError::WrongHeatTransferInteractionType);
        }

        // we need to obtain the single cv from the array cv first 
        // and this will be the back cv or inner cv 
        //

        let single_cv_node_self: &mut SingleCVNode = 
        &mut self.back_single_cv;

        SingleCVNode::calculate_single_cv_node_front_constant_temperature_back (
            bc_temperature,
            single_cv_node_self,
            interaction
        )
    }


}
