# Factory Simulator

This is some experimenting I'm doing with Discrete-Event-Simulation, where I want to simulate a complex factory with multiple lines, machines, etc.


## Currently (Oct. 7th 2025)
As of right now there are two types of components in the factory, machines and buffers.

buffers are essentially boxes with a certain capacity. They recieve items from machines, and output them to other machines. Pretty simple
The other component is the machines. They take items from the buffers (currently only one input buffer per machine), and output other items to other buffers (again, one output buffer per machine).

Each of these components has its own Id struct, and there is one AnyId enum to help with things that need to address both of them.

One of these things is a monitor. A component that's not yet fully built, the monitor will be alerted when changes are made to the components it's watching, and could schedule simulation events accordingly. I think this functionality would eventually need to be broken up, as multiple monitors might be needed for more complex scheduling policies, but we'll cross that bridge when we get there.
