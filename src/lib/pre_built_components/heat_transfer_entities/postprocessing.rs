
use uom::si::f64::*;

use super::HeatTransferEntity;
use super::cv_types::CVType;

use crate::boundary_conditions::BCType;
use crate::boussinesq_thermophysical_properties::density::try_get_rho;
use crate::thermal_hydraulics_error::TuasLibError;

impl HeatTransferEntity {

    #[inline]
    /// gets the temperature of the HeatTransferEntity 
    /// usually control volume at the current timestep
    pub fn temperature(entity: &mut HeatTransferEntity) -> 
    Result<ThermodynamicTemperature, TuasLibError> {

        let cv_temperature_result = match entity {
            Self::ControlVolume(cv_type) => {
                match cv_type {
                    CVType::SingleCV(single_cv) => {
                        single_cv.get_temperature_from_enthalpy_and_set()
                    },
                    CVType::FluidArrayCV(fluid_array_cv) => {
                        fluid_array_cv.try_get_bulk_temperature()
                    },
                    CVType::SolidArrayCV(solid_array_cv) => {
                        solid_array_cv.try_get_bulk_temperature()
                    },
                }
            },
            Self::BoundaryConditions(bc_type) => {
                match bc_type {
                    BCType::UserSpecifiedTemperature(temperature) => {
                        Ok(*temperature)
                    },
                    BCType::UserSpecifiedHeatFlux(_) | 
                        BCType::UserSpecifiedHeatAddition(_) => {
                            return Err(TuasLibError::NotImplementedForBoundaryConditions(
                            "getting temperature not \n 
                                implemented for BoundaryConditions".to_owned()));
                        },
                }
            },
        };


        return cv_temperature_result;
    }

    /// gets bulk temperature of the heat transfer entity
    #[inline]
    pub fn try_get_bulk_temperature(&mut self) -> Result<ThermodynamicTemperature,
    TuasLibError> {

        let bulk_temp: ThermodynamicTemperature = 
        HeatTransferEntity::temperature(self)?;

        Ok(bulk_temp)
    }

    /// gets a vector of temperatures
    #[inline]
    pub fn temperature_vector(entity: &mut HeatTransferEntity) ->
    Result<Vec<ThermodynamicTemperature>,TuasLibError> {


        // oof nested matching, kind of ugly but I'll live with it for now
        let cv_temperature_vector_result: 
        Result<Vec<ThermodynamicTemperature>, TuasLibError>
        = match entity {
            HeatTransferEntity::ControlVolume(cv_type) => {
                let temp_vector_result:
                Result<Vec<ThermodynamicTemperature>, TuasLibError>
                = match cv_type{
                    CVType::SingleCV(single_cv) => {
                        // the single cv only has one temperature anyway
                        let temperature = single_cv.get_temperature_from_enthalpy_and_set()?;

                        let mut temp_vector: Vec<ThermodynamicTemperature>
                        = vec![];

                        temp_vector.push(temperature);

                        Ok(temp_vector)
                    },
                    CVType::FluidArrayCV(fluid_array_cv) => {

                        fluid_array_cv.get_temperature_vector()

                    },
                    CVType::SolidArrayCV(solid_array_cv) => {

                        solid_array_cv.get_temperature_vector()
                    },
                };
                temp_vector_result
            },
            Self::BoundaryConditions(bc_type) => {
                match bc_type {
                    BCType::UserSpecifiedTemperature(temperature) => {

                        let mut temp_vector: Vec<ThermodynamicTemperature>
                        = vec![];

                        temp_vector.push(*temperature);
                        Ok(temp_vector)
                    },
                    BCType::UserSpecifiedHeatFlux(_) | 
                        BCType::UserSpecifiedHeatAddition(_) => {
                            return Err(TuasLibError::NotImplementedForBoundaryConditions(
                            "getting temperature not \n 
                                implemented for BoundaryConditions".to_owned()));
                        },
                }
            },
        };

        cv_temperature_vector_result
    }

    /// gets temperature vector of this HeatTransferEntity 
    #[inline]
    pub fn get_temperature_vector(&mut self) ->
    Result<Vec<ThermodynamicTemperature>, TuasLibError>{

        match self {
            HeatTransferEntity::ControlVolume(cv) => {
                // cv should return a temperature_vec here
                return cv.get_temperature_vector();
            },
            HeatTransferEntity::BoundaryConditions(bc) => {
                return bc.get_temperature_vector();
            },
        }

    }

    /// density vector 
    /// attempts to get a vector of densities
    #[inline]
    pub fn density_vector(entity: &mut HeatTransferEntity) ->
    Result<Vec<MassDensity>,TuasLibError> {

        let temperature_vector = Self::temperature_vector(entity)?;

        let material = match entity {
            HeatTransferEntity::ControlVolume(cv) => {
                match cv {
                    CVType::SingleCV(single_cv) => {
                        single_cv.material_control_volume.clone()
                    },
                    CVType::FluidArrayCV(fluid_array_cv) => {
                        fluid_array_cv.material_control_volume
                    },
                    CVType::SolidArrayCV(solid_array_cv) => {
                        solid_array_cv.material_control_volume
                    },
                }
            },
            HeatTransferEntity::BoundaryConditions(_) => {
                return Err(TuasLibError::
                    NotImplementedForBoundaryConditions(
                        "density not implemented for BC".to_string()
                    ));
            },
        };

        let pressure = match entity {
            HeatTransferEntity::ControlVolume(cv) => {
                match cv {
                    CVType::SingleCV(single_cv) => {
                        single_cv.pressure_control_volume
                    },
                    CVType::FluidArrayCV(fluid_array_cv) => {
                        fluid_array_cv.pressure_control_volume
                    },
                    CVType::SolidArrayCV(solid_array_cv) => {
                        solid_array_cv.pressure_control_volume
                    },
                }
            },
            HeatTransferEntity::BoundaryConditions(_) => {
                return Err(TuasLibError::
                    NotImplementedForBoundaryConditions(
                        "density not implemented for BC".to_string()
                    ));
            },

        };

        // get density 

        let mut density_vector: Vec<MassDensity> = vec![];

        for temperature in temperature_vector {
            let density = try_get_rho(
                material,
                temperature,
                pressure
            )?;

            density_vector.push(density);

        }

        // return density vector
        Ok(density_vector)


    }
}
