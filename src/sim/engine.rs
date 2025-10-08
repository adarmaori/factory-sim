use super::handlers;
use super::{BufId, Buffer, Event, EventKind, MachId, Machine};
use std::collections::{BinaryHeap, HashMap};
use tracing::{debug, trace};

pub struct Sim {
    pub time: f64,
    pub events: BinaryHeap<Event>,
    pub event_seq: usize,
    pub machines: HashMap<MachId, Machine>,
    pub buffers: HashMap<BufId, Buffer>,
    pub on_change: Option<fn(&mut Self)>,
}

impl Default for Sim {
    fn default() -> Self {
        Self {
            time: 0.0,
            events: BinaryHeap::new(),
            event_seq: 0,
            machines: HashMap::new(),
            buffers: HashMap::new(),
            on_change: None,
        }
    }
}

impl Sim {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn schedule(&mut self, event: Event) {
        self.events.push(Event {
            seq: Some(self.event_seq),
            ..event
        });
        self.event_seq += 1;
    }

    pub fn schedule_at(&mut self, time: f64, kind: EventKind) {
        self.events.push(Event {
            time,
            kind,
            seq: Some(self.event_seq),
        });
        self.event_seq += 1;
    }
    fn handle_event(&mut self, event: Event) {
        match event.kind {
            EventKind::TryStart(mid) => self.handle_try_start(mid),
            EventKind::Finish(mid) => self.handle_finish(mid),
            EventKind::SetBuffer(bid, amount) => self.handle_set_buffer(bid, amount),
            EventKind::ClearBuffer(bid) => self.handle_set_buffer(bid, 0),
        }
        if let Some(func) = self.on_change {
            func(self);
        }
    }

    pub fn run(&mut self, until: f64) {
        if let Some(func) = self.on_change {
            // Potentially seperate out later to self.on_startup
            func(self);
        }
        while let Some(ev) = self.events.pop() {
            self.time = ev.time;
            if self.time > until {
                break;
            }
            self.handle_event(ev);
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
