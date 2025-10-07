use std::collections::{BinaryHeap, HashMap};
use tracing::{debug, error, info, trace};

use crate::sim::{BufId, Buffer, Event, EventKind, MachId, Machine};

pub struct Sim {
    pub time: f64,
    pub events: BinaryHeap<Event>,
    pub machines: HashMap<MachId, Machine>,
    pub buffers: HashMap<BufId, Buffer>,
}

impl Default for Sim {
    fn default() -> Self {
        Self {
            time: 0.0,
            events: BinaryHeap::new(),
            machines: HashMap::new(),
            buffers: HashMap::new(),
        }
    }
}
impl Sim {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn schedule(&mut self, event: Event) {
        self.events.push(event);
    }

    fn handle_event(&mut self, event: Event) {
        // Check status of everything. If there's an inactive machine, try and activate it.
        match event.kind {
            EventKind::TryStart(mid) => self.handle_try_start(mid),
            EventKind::Finish(mid) => self.handle_finish(mid),
            EventKind::SetBuffer(bid, amount) => self.handle_set_buffer(bid, amount),
            EventKind::ClearBuffer(bid) => self.handle_set_buffer(bid, 0),
        }
    }

    fn handle_set_buffer(&mut self, b: BufId, amount: usize) {
        debug!(?b, time = self.time, "set_buffer");
        let buf = self.buffers.get_mut(&b).unwrap();
        if amount > buf.capacity {
            error!(
                ?b,
                time = self.time,
                amount = amount,
                capacity = buf.capacity,
                "tried to overfill buffer."
            );
            buf.amount = buf.capacity; // Should this be the default? Maybe just not fill it at all
        } else {
            buf.amount = amount;
        }
        info!(?b, time = self.time, "setting buffer");
    }

    fn handle_try_start(&mut self, m: MachId) {
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

    fn handle_finish(&mut self, m: MachId) {
        let machine = self.machines.get_mut(&m).unwrap();
        if !machine.busy {
            // Raise some sort of error
        }
        machine.busy = false;
        let output = self.buffers.get_mut(&machine.output).unwrap();
        output.amount += machine.output_amount;

        self.handle_try_start(m);
    }
    pub fn run(&mut self, until: f64) {
        while let Some(ev) = self.events.pop() {
            self.time = ev.time;
            if self.time > until {
                break;
            }
            self.handle_event(ev);
            let to_try: Vec<MachId> = self
                .machines
                .iter()
                .filter_map(|(mid, machine)| if !machine.busy { Some(*mid) } else { None })
                .collect();
            for mid in to_try {
                self.handle_try_start(mid);
            }
            self.print_state();
        }
    }

    pub fn print_state(&self) {
        trace!(time = self.time, "tick");
        for (id, buf) in &self.buffers {
            debug!(
                ?id,
                amount = buf.amount,
                capacity = buf.capacity,
                "buffer_state"
            );
        }
    }
}
