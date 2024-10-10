use crate::array_control_vol_and_fluid_component_collections::one_d_fluid_array_with_lateral_coupling::FluidArray;
use crate::array_control_vol_and_fluid_component_collections::one_d_solid_array_with_lateral_coupling::SolidColumn;
use crate::boundary_conditions::BCType;
use crate::single_control_vol::SingleCVNode;
use crate::thermal_hydraulics_error::ThermalHydraulicsLibError;

use super::cv_types::CVType;
use super::HeatTransferEntity;


impl Into<HeatTransferEntity> for BCType {
    fn into(self) -> HeatTransferEntity {
        HeatTransferEntity::BoundaryConditions(self)
    }
}


impl TryFrom<HeatTransferEntity> for BCType {
    type Error = ThermalHydraulicsLibError;

    fn try_from(hte: HeatTransferEntity) -> Result<Self, Self::Error> {
        let bc_type: BCType = match hte {
            HeatTransferEntity::ControlVolume(_) => {
                return Err(
                    ThermalHydraulicsLibError::TypeConversionErrorHeatTransferEntity)
            },
            HeatTransferEntity::BoundaryConditions(bc_type) => {
                bc_type
            },
        };

        Ok(bc_type)
    }
}

impl Into<HeatTransferEntity> for CVType {
    fn into(self) -> HeatTransferEntity {
        HeatTransferEntity::ControlVolume(self)
    }
}

impl TryFrom<HeatTransferEntity> for CVType {
    type Error = ThermalHydraulicsLibError;

    fn try_from(value: HeatTransferEntity) -> Result<Self, Self::Error> {
        match value {
            HeatTransferEntity::ControlVolume(cv) => {
                return Ok(cv);
            },
            HeatTransferEntity::BoundaryConditions(_) => {
                return Err(ThermalHydraulicsLibError::TypeConversionErrorHeatTransferEntity);
            },
        }
    }
}

impl Into<HeatTransferEntity> for FluidArray{
    fn into(self) -> HeatTransferEntity {
        HeatTransferEntity::ControlVolume(CVType::FluidArrayCV(self))
    }
}

impl TryInto<FluidArray> for HeatTransferEntity {
    type Error = ThermalHydraulicsLibError;

    fn try_into(self) -> Result<FluidArray, Self::Error> {
        if let HeatTransferEntity::ControlVolume(
            CVType::FluidArrayCV(fluid_array)) = self {

            Ok(fluid_array)

        } else {
            return Err(ThermalHydraulicsLibError::TypeConversionErrorHeatTransferEntity);
        }

    }
}

impl Into<HeatTransferEntity> for SolidColumn{
    fn into(self) -> HeatTransferEntity {
        HeatTransferEntity::ControlVolume(CVType::SolidArrayCV(self))
    }
}

impl TryInto<SolidColumn> for HeatTransferEntity {
    type Error = ThermalHydraulicsLibError;

    fn try_into(self) -> Result<SolidColumn, Self::Error> {
        if let HeatTransferEntity::ControlVolume(
            CVType::SolidArrayCV(solid_array)) = self {

            Ok(solid_array)

        } else {
            return Err(ThermalHydraulicsLibError::TypeConversionErrorHeatTransferEntity);
        }

    }
}

impl Into<HeatTransferEntity> for SingleCVNode{
    fn into(self) -> HeatTransferEntity {
        HeatTransferEntity::ControlVolume(CVType::SingleCV(self))
    }
}

impl TryInto<SingleCVNode> for HeatTransferEntity {
    type Error = ThermalHydraulicsLibError;

    fn try_into(self) -> Result<SingleCVNode, Self::Error> {
        if let HeatTransferEntity::ControlVolume(
            CVType::SingleCV(single_cv)) = self {

            Ok(single_cv)

        } else {
            return Err(ThermalHydraulicsLibError::TypeConversionErrorHeatTransferEntity);
        }

    }
}


