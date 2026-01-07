# Intro 

For fluid mechanics, TUAS works via solution of simplified 
momentum and mass balances. We first consider an isothermal case of 
incompressible flow through parallel branches. This is the simplest 
case of fluid flow which is easiest to understand.

Much of this writeup is basically a summary of my (Theodore's) 
master's thesis written in UC Berkeley.

This is: Ong, T. K. C. (2023). Development of an 
Isothermal-Flow Digital Twin for the Compact Integral Effects 
Test (Doctoral dissertation, MA thesis. University of California, Berkeley).


It is not exactly publicly available, but I can provide snippets of 
information from it valuable for documenting TUAS.


# Series of Pipes (Isothermal-Flow) Mass and Momentum Balances

Suppose we have fluid flowing through a series of pipes. This is liquid 
which does not change density. This series of pipes is connected to 
form a branch. 

The mass balance here is trivial, as there is no mass accumulation through 
the pipes. And whatever shapes the pipes are in, provided there are no 
leaks, the mass flowrate through all pipes are equal. Of course, the 
velocities may differ based on Bernoulli's Equation, but the mass flowrate 
through the pipes is the same. And there is no mass accumulation within 
each pipe.

For momentum balance, this is simplified in a 1D form to a point such that 
the pressure change across the entire branch (ie series of pipes) is equal 
to the sum of pressure losses across the pipes due to friction or form losses,
any hydrostatic changes in pressure, as well as any pressure source 
such as pumps.

# Parallel Branches (Isothermal-Flow) Mass and Momentum Balances

Now, flow in a reactor may not be as simple as fluid going through a 
singular series of pipes. More often than not, it is meant to go through 
parallel branches of piping. 

Suppose fluid flows from one branch and splits off into two branches, 
before then merging back together to one branch. In an Isothermal-Flow 
case, we can say that the mass flowrate through the original branch is 
the sum of the mass flowrates through both branches.

Secondly, the momentum balance is represented in an analogue of Kirchoff’s Voltage
Law adopted for fluid components. Kirchoff’s Voltage Law states that the potential difference
(analogous to pressure change) around any closed loop is zero. Practically, what this means
is that the pressure change across any branch is the same.

The pressure change here is the sum of contributions from pressure losses, hydrostatic
pressure and any other pressure sources within the branch. The most common of which are
pumps, which must have their respective pressures iteratively calculated using pump curves.
For this discussion, we simplify this by assuming that the frequency of the pump’s variable
frequency drive (VFD) is adjusted to such a point where a fixed pressure increase is provided
by the pump. 

TUAS v0.0.10 does not currently have pump curves added, though this 
can be a useful future addition if needed.




