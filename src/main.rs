use factory_sim::{prelude::*, sim::ids::ItemId};
use tracing::{info, trace};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("trace")
        .with_target(false)
        .compact()
        .init();

    let mut sim = Sim::new();

    const IRON: ItemId = ItemId(0);
    const GEAR: ItemId = ItemId(1);
    const GEARBOX: ItemId = ItemId(2);

    let iron = sim.add_buffer(Buffer::new(100, 0, IRON));
    let gear = sim.add_buffer(Buffer::new(100, 0, GEAR));
    let gearbox = sim.add_buffer(Buffer::new(100, 0, GEARBOX));

    let gear_casting = sim.add_machine(
        Machine::new(vec![(IRON, 1)], vec![(GEAR, 10)])
            .with_speed(2.0)
            .add_input(&sim, iron)
            .add_output(&sim, gear),
    );
    let gearbox_assembly = sim.add_machine(
        Machine::new(vec![(GEAR, 5)], vec![(GEARBOX, 1)])
            .with_speed(0.1)
            .add_input(&sim, gear)
            .add_output(&sim, gearbox),
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

    // Collect insights on the system from the available data
    // This'll eventually have to be save to some kind of database

    let amount_iron = factory.buffers[&iron].amount;
    let amount_gears = factory.buffers[&gear].amount;
    let amount_gearboxes = factory.buffers[&gearbox].amount;

    info!(?amount_iron, ?amount_gears, amount_gearboxes);
    // Deliver some iron
    if amount_iron < 50 {
        // Check if there's already a shipment on the way. This should eventually be a flag in some
        // database
        let mut on_route = false;
        for event in factory.events.iter() {
            if event.kind == AddToBuffer(iron, 10) {
                on_route = true;
            }
        }
        if !on_route {
            info!("delivering some iron");
            factory.schedule_in(5.0, AddToBuffer(iron, 10)); // Simulated shipment of iron that takes 5 seconds
        }
    }

    if amount_iron > 0 && amount_gears < 100 && !factory.machines[&gear_casting].busy {
        trace!("Trying to cast gears");
        factory.schedule_in(0.1, TryStart(gear_casting));
    }

    if amount_gears > 5 && amount_gearboxes < 100 && !factory.machines[&gearbox_assembly].busy {
        trace!("Trying to assemble gearboxes");
        factory.schedule_in(0.1, TryStart(gearbox_assembly));
    }
}
