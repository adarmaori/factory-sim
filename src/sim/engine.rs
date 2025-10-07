use super::handlers;
use super::{BufId, Buffer, Event, EventKind, MachId, Machine};
use std::collections::{BinaryHeap, HashMap};
use tracing::{debug, trace};

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
