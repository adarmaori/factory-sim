use crate::sim::{Event, EventKind, MachId, Sim};
use tracing::{debug, error, info, trace, warn};

impl Sim {
    pub(crate) fn handle_try_start(&mut self, m: MachId) {
        debug!(?m, time = self.time, "try_start");
        let (busy, input, output, speed, min_input, output_amount) = {
            let machine = self.machines.get_mut(&m).unwrap();
            (
                machine.busy,
                machine.input,
                machine.output,
                machine.speed,
                machine.min_input,
                machine.output_amount,
            )
        };
        // Check if machine in busy
        if busy {
            trace!(?m, "machine busy; skipping");
            return;
        }
        // Check if output buffer is ready to recieve
        {
            let out = self.buffers.get(&output).unwrap();
            if out.capacity - out.amount < output_amount {
                trace!(?m, "output full; skipping");
                return;
            }
        }
        // Check if the input buffer has the stuff
        {
            let inp = self.buffers.get_mut(&input).unwrap();
            if inp.amount < min_input {
                trace!(?m, "input empty; skipping");
                return;
            }
            // take items into machine
            inp.amount -= min_input;
        }

        let machine = self.machines.get_mut(&m).unwrap();
        machine.busy = true;
        // Schedule the finish event
        let finish_time = self.time + 1.0 / speed;
        let finish_event = Event {
            kind: EventKind::Finish(m),
            time: finish_time,
        };
        self.schedule(finish_event);
    }

    pub(crate) fn handle_finish(&mut self, m: MachId) {
        let machine = self.machines.get_mut(&m).unwrap();
        if !machine.busy {
            // Raise some sort of error
        }
        machine.busy = false;
        let output = self.buffers.get_mut(&machine.output).unwrap();
        output.amount += machine.output_amount;

        self.handle_try_start(m);
    }
}
