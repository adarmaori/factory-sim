use factory_sim::prelude::*;

fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let iron = BufId(0);
    let gear = BufId(1);
    let asm = MachId(0);

    let mut sim = Sim::new();
    sim.buffers.insert(iron, Buffer::new(100, 75));
    sim.buffers.insert(gear, Buffer::new(100, 50));
    sim.machines.insert(asm, Machine::new(iron, gear, 2.0));

    sim.schedule(Event {
        time: 0.0,
        kind: EventKind::TryStart(asm),
    });
    sim.run(100.0);

    println!(
        "t={:.1} iron={} gear={}",
        sim.time, sim.buffers[&iron].amount, sim.buffers[&gear].amount
    );
}

