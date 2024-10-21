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
