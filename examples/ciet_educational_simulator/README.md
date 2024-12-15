# CIET Educational Simulator


# Intro 
(TBD) 

# Min Requirements 

- Rust 1.79
- a fast enough CPU to run in real-time (i7-10875H was good)
- 1920 x 1080 screen resolution (otherwise the full main page won't show properly)

# Todo List 

## Performance

- Mass balances during flow diode situation is not exactly good, 
it is basically check the three branch flow version first. If the 
mass flowrate in DHX branch is in opposite direction, then recalculate 
the two branch version. Done serially, it's kind of slow. Do it in 
parallel (implemented, and doesn't look wrong at least from the GUI)


- Should solution time be too slow for certain timesteps, then I want 
the thread not to sleep too long so that the simulation can speed up. 
So that means if elapsed time is more than simulation time, do not allow 
thread sleep so that the simulation can be in real-time. This makes it 
more palatable for slower computers. (done, fast fwd button does reasonably well)



### Very low priority

- Potentially, one could solve mass balances and energy balances on 
two different timesteps. This could help with simulation stability and 
performance. However, this technique needs to be tested. I wonder if Modelica 
has this capability. Or has this research been done? Perhaps can show novelty?


## Engineering 

- The CTAH and TCHX coolers don't function well with their PID, the 
cooling response is rather slow. I suspect the thermal resistance due 
to the Nusselt numbers within the pipe representing CTAH or TCHX is 
partly to blame. Hence, need to increase nusselt numbers within the CTAH/TCHX 
pipings, and readjust PID controllers.

- DRACS loop flowrates currently can only be simulated one way because 
I used absolute flowrates. This is a simplification that made coding easier.
This should change to be able to simulate negative flowrates. 


## User Interface and User Experience (UI and UX)

- fast forward button, toggle on and off (done)

- Pipe 5b is not crossing well with the top mixing node. adjust please

- branch blocking should be on the main page. perhaps it makes it easier?
use a radio button rather than a toggle button

- rather than paint the components at a specific pixel, use relative pixels.
This is because, the simulator won't work for resolutions other than 
1920 x 1080.

- spelling error in title "Ciet Simualtorv1"


## Features

- Heater page, 

should have graph of heater power, BT11 and BT12 overlaid on the same 
graph or two different graphs.

should have a csv file generator, so that I can take perhaps the last 
500s worth of data. (at least) Can improve in future to make it user 
customisable

- CTAH pump page 

Just inlet and outlet temperatures (pipe 12 and 13) nothing too interesting 
to display here. 

add a graph

- CTAH page 

inlet and outlet temperatures on a graph, 
csv generator 

also, display current heat transfer coefficient at the surface

- DHX Branch 

DHX should have inlet and outlet temperatures. Plus a full schematic 
diagram on all its individual temperatures.

Also show flowrates in both loops on a graph

- Dracs loop
also, display current heat transfer coefficient at the surface

## Regression, Testing, Validation and Verification


### forced circulation 

- mesh refinement study for heater transient using Zweibaum's expt data (roughly using graph reader)
- validation for natural circulation 
- frequency response testing using (De Wet's Bode Plots)

### Natural circulation and startup

Need to analyse Zweibaum's experimental data around the loop for best 
results.


# For the future 

Hope to show that TUAS is compatible with 3D libraries to make 3D simulators.
So this is a showpiece for 2D simulator systems


# credits 


- Heat exchanger, heater, cooler and pump artwork was by DWSIM (licensed under 
GPLv3)

DWSIM has been used successfully in literature and compared to commercial 
products: 

Tangsriwong, K., Lapchit, P., Kittijungjit, T., Klamrassamee, T., 
Sukjai, Y., & Laoonual, Y. (2020, March). Modeling of chemical processes using 
commercial and open-source software: A comparison between Aspen Plus and 
DWSIM. In IOP Conference Series: Earth and Environmental 
Science (Vol. 463, No. 1, p. 012057). IOP Publishing.
