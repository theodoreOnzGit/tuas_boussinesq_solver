
/// components for first iteration of UW madison FLiBe loop model
///
/// Pipes have 
/// OD of 2.54 cm (1 in) and 3mm thick wall according to literature 
/// means ID is 2.54cm - 2*3mm
///
/// For schematics, please refer to:
///
/// [Component no.],[description],[length (m)],[angle],[Delta x],[Delta y],
/// [1],[hot leg insulated heater vertical],[1.47],[90],[9.00115397373305E-17],[1.47],
/// [2],[opening to tank],[0.12],[0],[0.12],[0],
/// [3],[top left bend vertical],[0.0897],[-90],[5.49254089417588E-18],[-0.0897],
/// [4],[top left bend diagonal],[0.0897],[-58.9],[0.0463330395993378],[-0.0768071574886494],
/// [5],[cold leg horizontal],[1.35],[-10],[1.32949046656648],[-0.234425039850356],
/// [6],[Cold leg corner bend],[0.0897],[-20],[0.084290428084496],[-0.0306792068563125],
/// [7],[cold leg vertical],[1.53],[-90],[9.36854801347725E-17],[-1.53],
/// [8],[cold to hot leg bend 1],[0.0598],[-100],[-0.0103841610244824],[-0.05889150363013],
/// [9],[cold to hot leg bend 2],[0.0598],[-150],[-0.0517883191463094],[-0.0299],
/// [10],[cold to hot leg bend 3],[0.0598],[180],[-0.0598],[7.32338785890117E-18],
/// [11],[hot leg horizontal-ish],[1.42],[160],[-1.33436352151599],[0.48566860352245],
/// [12],[hot leg bend 1],[0.0897],[130],[-0.0576580485888826],[0.0687141865477723],
/// [13],[hot leg bend 2],[0.0697],[158],[-0.0646247146633051],[0.0261100795610891],
///
///
/// for heat losses, and insulation, the paper used pyrogel. 
/// However, thicknesses and heat loss aren't really calibrated well yet. 
///
/// Pyrogel data seems to be limited. But potentially useful to add to 
/// the library. The actual one is Pyrogel HPS, but what I found online is 
/// Pyrogel XT and Pyrogel HT. There are several kinds of pyrogel.
pub mod components;

/// does fluid mechanics calculations through each branch
pub mod thermal_hydraulics_calculations;

/// parasitic heat loss calibrations
pub mod parasitic_heat_loss_calibration;



