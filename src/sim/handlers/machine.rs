use crate::sim::{EventKind, MachId, Machine, Sim};
use anyhow::{Context, Error};
use tracing::{debug, info};

impl Sim {
    pub fn add_machine(&mut self, machine: Machine) -> MachId {
        let id = MachId(self.next_mach_id);
        self.machines.insert(id, machine);
        self.next_mach_id += 1;
        id
    }
    pub(crate) fn handle_try_start(&mut self, m: MachId) {
        debug!(?m, time = self.time, "try_start");

        // verify inputs exist and have enough
        let machine = self.machines.get_mut(&m).unwrap();
        if machine.busy {
            debug!(?m, time = self.time, "machine is busy; skipping");
            return;
        }
        // print!("is_connected: {}", machine.is_connected());
        for &(item_id, count) in machine.requires.iter() {
            if let Some(bid) = machine.input.get(&item_id) {
                let b = self.buffers.get(bid).unwrap();
                if b.amount < count {
                    return;
                }
            } else {
                panic!("Buffer not connected for item: {:?}", item_id);
            }
        }

        // verify outputs exist and have enough space
        for &(item_id, count) in machine.creates.iter() {
            if let Some(bid) = machine.output.get(&item_id) {
                let b = self.buffers.get(bid).unwrap();
                if b.capacity - b.amount < count {
                    return;
                }
            } else {
                panic!("Buffer not connected for item: {:?}", item_id);
            }
        }

        // Remove items from input buffers
        for &(item_id, count) in machine.requires.iter() {
            let bid = machine.input.get(&item_id).unwrap();
            let b = self.buffers.get_mut(bid).unwrap();
            b.amount -= count;
            info!(
                "Filling machine {:?} with item {:?} from buffer {:?}",
                m, item_id, bid
            );
        }

        machine.busy = true;
        // Schedule the finish event
        let finish_time = 1.0 / machine.speed;
        self.schedule_in(finish_time, EventKind::Finish(m));
    }

    pub(crate) fn handle_finish(&mut self, m: MachId) -> Result<(), Error> {
        let machine = self
            .machines
            .get_mut(&m)
            .context("Failed to access machine data")?;
        machine.busy = false;

        for &(item_id, count) in machine.creates.iter() {
            if let Some(bid) = machine.output.get(&item_id) {
                let b = self.buffers.get_mut(bid).unwrap();
                if b.capacity - b.amount < count {
                    panic!("Trying to overfill a buffer");
                } else {
                    b.amount += count;
                }
            } else {
                panic!("Not connected to all the required buffers");
            }
        }
        Ok(())
    }
}
