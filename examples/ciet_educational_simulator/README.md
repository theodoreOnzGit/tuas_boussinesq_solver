# CIET Educational Simulator


# Intro 
(TBD) 

# Min Requirements 

- Rust 1.81
- a fast enough CPU to run in real-time (i7-10875H was good)
- 8 GB RAM, which is kind of the minimum for laptops in 2024
- 1920 x 1080 screen resolution (otherwise the full main page won't show properly)

# Basic installs before you run cargo run 

before
```bash
cargo run --release --example ciet_educational_simulator
```

you need, on ubuntu, basic dependencies besides the GUI

```bash
sudo apt install gcc libssl-dev pkg-config
```



# Todo List 

## dependency bugs on some Windows Subsystem for Linux (WSL)

I'm running into some conflicting dependency issues with 
egui on WSL (), but not on my Linux Mint and Arch Linux 
machine... not sure why. I think it's just 
my pc, some WSL versions don't have this 
problem It can run on native windows though, but WSL is problematic.

## stack overflow on native windows

Native windows cannot really run this due to stack overflow problems.
Your best bet on a windows machine is via a virtual machine or WSL
(which is also a virtual machine as of WSL 2).

## Performance

- Potentially, one could solve mass balances and energy balances on 
two different timesteps. This could help with simulation stability and 
performance. However, this technique needs to be tested. I wonder if Modelica 
has this capability. Or has this research been done? Perhaps can show novelty?


## Engineering 


- DRACS loop flowrates currently can only be simulated one way because 
I used absolute flowrates. This is a simplification that made coding easier.
This should change to be able to simulate negative flowrates. 


## User Interface and User Experience (UI and UX)


- branch blocking should be on the main page. perhaps it makes it easier?
use a radio button rather than a toggle button

- rather than paint the components at a specific pixel, use relative pixels.
This is because, the simulator won't work for resolutions other than 
1920 x 1080.


## Features


## Regression, Testing, Validation and Verification

- V&V yet to be done properly


### forced circulation 

- mesh refinement study for heater transient using Zweibaum's expt data (roughly using graph reader)
- validation for natural circulation 
- frequency response testing using (De Wet's Bode Plots)
- frequency response testing using (Poresky's Bode and Time Domain Plots)

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
