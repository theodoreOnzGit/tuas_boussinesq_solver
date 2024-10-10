///
/// This module's test attempted to tweak the insulation thickness
/// to ambient in order to obtain the correct dhx inlet temperature 
/// unfortunately, the parasitic heat losses were not sufficient 
/// to achieve this objective,
///
/// The next thing is to do as the RELAP and SAM model did,
/// which is to increase the overall heat transfer coefficient (U) rather 
/// than just the wall side or ambient air side heat transfer coefficient. 
/// Either by multiplying the heat transfer 
/// area density by a certain amount (SAM) or applying a multiplicative 
/// factor (page 40-41 of Zweibaum's thesis)
///
///
/// Zweibaum, N. (2015). Experimental validation of passive 
/// safety system models: Application to design and optimization 
/// of fluoride-salt-cooled, high-temperature reactors. 
/// University of California, Berkeley.
///
/// previous tests indicated that increasing the heat transfer to ambient 
/// or even the nusselt number did not significantly impact parasitic heat 
/// losses. Hence, the way to calibrate parasitic heat loss is by decreasing 
/// the insulation thickness from 0.0508 m to something less.
///
/// dataset number,pri loop mass flowrate (kg/s),Heater outlet (DegC),DHX shell top (DegC),DHX shell bottom (DegC),Heater inlet (DegC),
/// C-1,0.02003,75.22747,71.47752,53.60943,50.45784,
/// C-2,0.02367,82.41863,78.36713,57.13467,53.79036,
/// C-3,0.02635,87.78188,84.37342,59.82845,56.71891,
/// C-4,0.02949,94.71628,90.97595,63.9812,60.83029,
/// C-5,0.0319,100.37023,96.20228,67.05336,64.07406,
/// C-6,0.03412,105.25073,101.3375,69.85085,67.1654,
/// C-7,0.03562,110.34289,106.43149,73.21226,70.6215,
/// C-8,0.03593,115.52364,111.37615,76.13202,73.63344,
/// C-9,0.03547,119.96879,116.05003,79.02407,76.54479,
pub mod insulation_thickness_calibration;

/// This module's test attempted to tweak the heat trasnfer coeffcient (htc) 
/// to ambient in order to obtain the correct dhx inlet temperature 
/// unfortunately, the parasitic heat losses were not sufficient 
/// to achieve this objective,
///
/// The next thing is to do as the RELAP and SAM model did,
/// which is to reduce the convective thermal resistance between 
/// fluid and wall. Either by multiplying the heat transfer 
/// area density by a certain amount (SAM) or applying a multiplicative 
/// factor (page 40-41 of Zweibaum's thesis). This module aims to do that
///
/// Zweibaum, N. (2015). Experimental validation of passive 
/// safety system models: Application to design and optimization 
/// of fluoride-salt-cooled, high-temperature reactors. 
/// University of California, Berkeley.
///
/// anyhow, this also failed, the multiplicative regression also failed 
/// dhx inlet temperature was still around 74 degrees C even at 
/// multiplcation factors of about ~30 - 15000 (max time = 3000s, little or 
/// no change)
///
/// dataset number,pri loop mass flowrate (kg/s),Heater outlet (DegC),DHX shell top (DegC),DHX shell bottom (DegC),Heater inlet (DegC),
/// C-1,0.02003,75.22747,71.47752,53.60943,50.45784,
/// C-2,0.02367,82.41863,78.36713,57.13467,53.79036,
/// C-3,0.02635,87.78188,84.37342,59.82845,56.71891,
/// C-4,0.02949,94.71628,90.97595,63.9812,60.83029,
/// C-5,0.0319,100.37023,96.20228,67.05336,64.07406,
/// C-6,0.03412,105.25073,101.3375,69.85085,67.1654,
/// C-7,0.03562,110.34289,106.43149,73.21226,70.6215,
/// C-8,0.03593,115.52364,111.37615,76.13202,73.63344,
/// C-9,0.03547,119.96879,116.05003,79.02407,76.54479,
pub mod heat_transfer_to_ambient_calibration;

/// this module's test attempted to tweak the pipe nusselt number 
/// in order to calibrate the parasitic heat losses 
/// The problem is that even after adjusting the nusselt number up 
/// by 20 times, there was no appreciable increase in parasitic 
/// heat loss
///
/// I suspected that the parasitic heat loss was dominated by the 
/// insulation thermal resistance. Tested this by tuning the nusselt 
/// number up ~15000 times and the heat transfer to ambient up by 
/// 100 times. 
///
/// It seems that Zweibaum's method and the method used in SAM for adjusting 
/// up heat transfer coefficient is referring to overall heat transfer 
/// coefficient rather than the heat transfer coefficient to air or to 
/// the fluid in the tube
pub mod pipe_nusselt_number_calibration;

