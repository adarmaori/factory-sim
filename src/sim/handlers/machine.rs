use crate::sim::{BufId, EventKind, MachId, Machine, Sim, SimError};
use anyhow::Result;
use tracing::{debug, info};

impl Sim {
    pub fn add_machine(&mut self, machine: Machine) -> MachId {
        let id = MachId(self.next_mach_id);
        self.machines.insert(id, machine);
        self.next_mach_id += 1;
        id
    }

    pub fn get_machine(&self, m: MachId) -> Result<&Machine, SimError> {
        if let Some(res) = self.machines.get(&m) {
            Ok(res)
        } else {
            Err(SimError::MachineDoesntExist(m))
        }
    }

    pub fn get_machine_mut(&mut self, m: MachId) -> Result<&mut Machine, SimError> {
        if let Some(res) = self.machines.get_mut(&m) {
            Ok(res)
        } else {
            Err(SimError::MachineDoesntExist(m))
        }
    }

    pub(crate) fn handle_try_start(&mut self, m: MachId) -> Result<bool, SimError> {
        debug!(?m, time = self.time, "try_start");

        let input_takes: Vec<(BufId, usize)> = {
            let machine = self.get_machine(m)?;
            if machine.busy {
                debug!(?m, time = self.time, "machine is busy; skipping");
                return Ok(false);
            }

            // Validate inputs and collect which buffers/amounts to take
            let mut takes = Vec::with_capacity(machine.requires.len());
            for &(item_id, count) in machine.requires.iter() {
                if let Some(&bid) = machine.input.get(&item_id) {
                    let b = self.get_buffer(bid)?;
                    if b.amount < count {
                        return Ok(false);
                    }
                    takes.push((bid, count));
                } else {
                    return Err(SimError::ItemHasNoBuffer(item_id));
                }
            }

            // Validate outputs have enough space
            for &(item_id, count) in machine.creates.iter() {
                if let Some(&bid) = machine.output.get(&item_id) {
                    let b = self.get_buffer(bid)?;
                    if b.capacity - b.amount < count {
                        return Ok(false);
                    }
                } else {
                    return Err(SimError::ItemHasNoBuffer(item_id));
                }
            }

            takes
        };

        for &(bid, count) in &input_takes {
            let b = self.get_buffer_mut(bid)?;
            b.amount -= count;
            info!("Filling machine {:?} from buffer {:?}", m, bid);
        }

        let machine = self.get_machine_mut(m)?;
        machine.busy = true;
        let finish_time = 1.0 / machine.speed;
        self.schedule_in(finish_time, EventKind::Finish(m));

        Ok(true)
    }

    pub(crate) fn handle_finish(&mut self, m: MachId) -> Result<bool, SimError> {
        let machine = self.get_machine_mut(m)?;
        if !machine.busy {
            return Err(SimError::InvalidCommand(
                "Trying to stop an inactive machine".to_string(),
            ));
        }
        machine.busy = false;
        let additions: Vec<(BufId, usize)> = {
            let mut res = vec![];
            for &(item_id, count) in machine.creates.iter() {
                if let Some(bid) = machine.output.get(&item_id) {
                    res.push((*bid, count));
                } else {
                    // Return Error
                    return Err(SimError::ItemHasNoBuffer(item_id));
                }
            }
            res
        };
        for addition in additions {
            let buf = self.get_buffer_mut(addition.0)?;
            buf.add_amount(addition.1)?;
        }
        Ok(true)
    }
}
