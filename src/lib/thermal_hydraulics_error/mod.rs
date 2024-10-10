use thiserror::Error;

/// Master Error type of this crate
#[derive(Debug, Error)]
pub enum ThermalHydraulicsLibError {
    /// linear algebra error
    #[error("linear algebra error")]
    LinalgError(#[from] ndarray_linalg::error::LinalgError),

    /// empty mass flowrate vector error 
    ///
    /// this case is where the mass flowrate vector in a control 
    /// volume is empty, 
    /// so we can't calculate a courant number

    #[error("cannot calculate courant number: mass flowrate \n 
        there is no mass flows going in or out of your \n 
        control volume")]
    CourantMassFlowVectorEmpty,


    /// it's a generic error which is a placeholder since I used 
    /// so many string errors
    #[error("Placeholder Error Type for Strings{0} ")]
    GenericStringError(String),

    /// error to indicate that function is not implemented for BC 
    #[error("{0}")]
    NotImplementedForBoundaryConditions(String),

    /// error for type conversions for heat transfer entity
    #[error("heat transfer entity is of the wrong type")]
    TypeConversionErrorHeatTransferEntity,

    /// error for type conversions for material
    #[error("material is of the wrong type for proper conversion")]
    TypeConversionErrorMaterial,

    /// error for temperature out of range for 
    /// thermophysical thermophysical_properties
    #[error("Temperature supplied for thermophysical_properties\n 
        function was out of range")]
    ThermophysicalPropertyTemperatureRangeError,

    /// generic thermophysical property error
    #[error("Thermophysical Property Error")]
    ThermophysicalPropertyError,

    /// wrong heat transfer interaction type
    #[error("Wrong Heat Transfer Interaction Type")]
    WrongHeatTransferInteractionType,
    
}

///  converts ThermalHydraulicsLibError from string error
impl From<String> for ThermalHydraulicsLibError {
    fn from(value: String) -> Self {
        Self::GenericStringError(value)
    }
}

impl Into<String> for ThermalHydraulicsLibError {
    fn into(self) -> String {
        match self {
            ThermalHydraulicsLibError::LinalgError(_) => {
                self.to_string()
            },
            ThermalHydraulicsLibError::CourantMassFlowVectorEmpty => {
                self.to_string()
            },
            ThermalHydraulicsLibError::GenericStringError(string) => {
                string
            },
            ThermalHydraulicsLibError::NotImplementedForBoundaryConditions(string) => {
                string
            },
            ThermalHydraulicsLibError::TypeConversionErrorHeatTransferEntity => {
                self.to_string()
            },
            ThermalHydraulicsLibError::TypeConversionErrorMaterial => {
                self.to_string()
            },
            ThermalHydraulicsLibError::ThermophysicalPropertyTemperatureRangeError => {
                self.to_string()
            },
            ThermalHydraulicsLibError::ThermophysicalPropertyError => {
                self.to_string()
            },
            ThermalHydraulicsLibError::WrongHeatTransferInteractionType => {
                self.to_string()
            },


        }
    }
}

