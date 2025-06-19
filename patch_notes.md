## v 0.0.8 

Updating regression testing for CIET coupled natural circulation. This will 
include temperature data for DHX STHE, Heater and TCHX for comparison 
with experimental data. Minor correction to TCHX controller in coupled nat 
circ loop, to use pipe 34 as bulk temperature for controller measurement.

Regression is still IN PROGRESS, this is NOT completed or tested properly.

This is also done for isolated dracs loop. However, changing the TCHX 
measurements makes no differnce to the test, it still passes.

The most important change is change the trait implementation for 
"calculate_mass_flowrate_from_pressure_change". The previous mass flowrate 
iteration bounds were from -10 kg/s to 10 kg/s. This is okay for CIET, but 
for FHRs, which have flowrates about 1200 kg/s as for the gFHR, this algorithm 
crashes. To enable convergence for the FHRs, I increased the bounds to 
100,000 kg/s.

## v 0.0.7

Mostly tweaks and improvements for the CIET Educational Simulator.
Can't quite remember.

## v 0.0.6 

For this version, there were key bugfixes and enhancements 
for the CIET Educational Simulator,
and also for all components within CIET. The mistake was that the
shell outer diameter for all components in CIET was calculated by adding the 
thickness to the shell inner diameter when it should have been two 
times the thickness. As a result, all coupled natural circulation 
regression tests had to be changed. Thankfully in the tests, they were still 
largely correct and within the 6.2% error bounds of experimental data.

Additionally, the MX-10 and CIET Heater v2 have been converted to 
NonInsulatedPorousMediaFluidComponent and InsulatedPorousMediaFluidComponent 
structs respectively. Regression and validation were performed for both.
Moreover, additional validation tests using CIET heater data were 
performed on NonInsulatedFluidComponent and verification using an analytical 
solution was performed on the InsulatedFluidComponent structs. 

Also now, the CIET Educational Simulator again works natively on windows.



## v 0.0.5 

For this version, the key thing is the addition of the CIET educational
simulator example. It showcases three loops in a GUI with heater controlled 
by user, valves that can be toggled for CTAH and DHX branch and a CTAH 
and TCHX with set point controllable by the user.


## v 0.0.4 

The main thing is demonstration of a full educational simulator backend 
for CIET that runs in real-time. This demonstrates that the CIET 
simulator with all three branches can run in real-time even with additional 
computational burden and timesteps of 0.2s. 
On my 4.0 GHz i7-10875H, this simulation was able to run on single 
thread at under 460s of computational time, for 6300s of simulated 
natural circulation. 

Likewise, when forward and reverse flow heat transfer was 
simulated for 30s across all three branches, the simulations took about 4-5 s 
on this same core and same timestep with heat transfer calculations and 
fluid mechanics calculations all running serially. This is good because 
it demonstrates the ability for a full simulation CIET to be run in 
real-time for educational purposes. Now, this simulation should still be 
validated using transient data. But that is for future regression testing.

There were also some bugfixes for HeatTransferEntity classes that previously 
did not correctly connect SingleCVNode HeatTransferEntity objects 
to FluidArray HeatTransferEntity objects.

Added pyrogel material for FLiBe loop UW madison,
also tidied up the temperature from enthalpy functions and 
enthalpy functions. Moved the enthalpy functions to their own 
respective materials. 

Still need to do work on the ClamshellRadiativeHeater class.

## v 0.0.3 

for coupled dracs loop, added parasitic heat loss 
tests to ver 6 and ver 7, where 
heater nusselt numbers were also calibrated properly, and boundary 
conditions for heater set to adiabatic.

Also note that pipe 3 was adjusted to use sam parameters. 
That is, it has a K value of 17.15 as opposed to the relap value of 3.15.


## v 0.0.2 

Renamed thermal_hydraulics_error as tuas_lib_error. Added back in 
basic conjugate heat transfer and semi infinite medium tests. Also 
added CIET heater examples as tests.

## v 0.0.1

This is the first update of the tuas_boussinesq_solver,
which is named after the Tuas industrial area in Singapore.

It is also an acronym for Thermo-hydraulic Uniphase Advection and Convection 
Solvers (TUAS). It was ported from v0.0.12 of thermal_hydraulics_rs library 
where upon receiving advice, it was better to segregate the solvers into 
separate github repositories.

In v0.0.1, I have added all calibrated coupled DRACS loop results 
for datasets A, B and C within the SAM publication. It has matched the 
DRACS loop flowrate experimental data to within 6.1%, and pri loop flowrate 
experimental data to within 4.4%. In contrast to SAM, agreement with 
experimental data, the max error was 6.76% for the DRACS loop flowrate 
and 6.65% for the primary loop flowrate. See reference:

Zou, L., Hu, G., O'Grady, D., & Hu, R. (2021). Code validation of 
SAM using natural-circulation experimental data from the compact integral 
effects test (CIET) facility. Nuclear Engineering and Design, 377, 111144.

Given that the calibrated simulation with the TUAS solver agreed better with 
experimental data than SAM, I consider this validation effort successful.
The tests are parked under the pre_built_components module, where we have 
the ciet_steady_state_natural_circulation_test_components module. Inside that,
I put the coupled_dracs_loop_tests modules with dataset_a, dataset_b 
and dataset_c.
