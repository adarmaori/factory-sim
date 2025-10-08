use factory_sim::prelude::*;
use tracing::{debug, info, trace};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("trace")
        .with_target(false)
        .compact()
        .init();

    let iron = BufId(0);
    let gear = BufId(1);
    let gearbox = BufId(2);
    let gear_casting = MachId(0);
    let gearbox_assembly = MachId(1);

    let mut sim = Sim::new();
    sim.buffers.insert(iron, Buffer::new(100, 0));
    sim.buffers.insert(gear, Buffer::new(100, 0));
    sim.buffers.insert(gearbox, Buffer::new(100, 0));
    sim.machines.insert(
        gear_casting,
        Machine::new(iron, gear, 2.0).with_output_amount(10),
    );
    sim.machines.insert(
        gearbox_assembly,
        Machine::new(gear, gearbox, 0.1).with_min_input(5),
    );
    sim.on_change = Some(manager);
    sim.run(100.0);

    println!(
        "t={:.1} iron={} gear={} gearbox={}",
        sim.time,
        sim.buffers[&iron].amount,
        sim.buffers[&gear].amount,
        sim.buffers[&gearbox].amount
    );
}

fn manager(factory: &mut Sim) {
    trace!("manager called");
    use EventKind::*;
    let iron = BufId(0);
    let gear = BufId(1);
    let gearbox = BufId(2);
    let gear_casting = MachId(0);
    let gearbox_assembly = MachId(1);

    let amount_iron = factory.buffers[&iron].amount;
    if amount_iron < 90 {
        info!("delivering some iron");
        factory.schedule_at(factory.time + 5.0, SetBuffer(iron, amount_iron + 10)); // Simulated
        // shipment of iron that takes 5 seconds
    }
}
