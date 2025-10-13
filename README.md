# Factory Simulator

This is some experimenting I'm doing with Discrete-Event-Simulation, where I want to simulate a complex factory with multiple lines, machines, etc.


## Currently (Oct. 8th 2025)
As of right now there are two types of components in the factory, machines and buffers.

buffers are essentially boxes with a certain capacity. They recieve items from machines, and output them to other machines. Pretty simple.
The other component is the machines. They take items from the buffers, and output other items to other buffers. There can be multiple input and output buffers, but only one per ItemId.

### Events
The state of the simulation changes through events. Events can be scheduled, and the siulation engine (the Sim struct) pulls those events from a heap and executes them.

### Floor Manager
Every factory needs someone, or some function to manage it. In this case, after every event (that actually does something) happens, the engine calls the manager function (known as its on_change field), which's only ability is to schedule future events itself. If it does not do so and the events queue runs out, the simulation is over.

## TODO's
- [ ] Error handling: Only the manager should panic (Or not, depending on whether the error is recoverable or not). All other things should propegate errors up. Event handlers should all return Result<bool, SomeErrorType>, and the bool would indicate whether or not something actually happened. This'll help call the manager way less.
- [ ] Tests: Add creation and unit-tests for all the individual modules, and some E2E tests for the engine itself.
- [ ] Manager global state: Find a way to store and modify the manager's knowledge so it doesn't need to be computed every time.
- [ ] Containers: A bit different from the others (And maybe for after I've got a stable, tested and clean V0.1). I want to be able to take closed sets of machines and buffers and containerize them in a new machine, with only the set's inputs and outputs available for changing outside of it. This seems useful as a way to build abstraction, and should be possible with the current system (maybe slightly modifying the API, not sure yet).

- [ ] Randomization: Add things like QC fails, random time for completion, machine breakdowns, and reliability metrics along with the time metrics, which reminds me:
- [ ] Metrics: Summarize throughput, reliability, footprint, cost(?) to allow for intelligent system modeling
