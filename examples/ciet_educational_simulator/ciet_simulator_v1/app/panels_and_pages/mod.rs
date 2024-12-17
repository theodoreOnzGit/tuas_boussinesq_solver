
#[derive(serde::Deserialize, serde::Serialize,PartialEq,Clone)]
pub(crate) enum Panel {
    MainPage,
    CTAHPump,
    CTAH,
    Heater,
    DHX,
    TCHX,
    SchematicDiagram,
    NodalisedDiagram,
}

pub mod main_page;

pub mod heater_page;

pub mod ctah_page;

/// page for controlling pumps and valves along the CTAH
pub mod ctah_pump_page;

/// page for controlling valves along the dhx branch 
/// and for seeing the DHX more closely
pub mod dhx_page;


pub mod ciet_data;

/// contains code for natural circulation only
pub mod nat_circ_simulation;

pub mod tchx_page;

/// contains code for the full educational simulator of CIET 
/// both forced and natural circulation
pub mod full_simulation;

/// citation and disclaimer page code
pub mod citations_and_disclaimers;
