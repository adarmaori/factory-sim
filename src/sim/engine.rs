use super::{BufId, Buffer, Event, EventKind, MachId, Machine};
use std::collections::{BinaryHeap, HashMap};
use tracing::{debug, trace};

pub struct Sim {
    pub time: f64,
    pub events: BinaryHeap<Event>,
    next_event_seq: usize,
    pub next_mach_id: usize,
    pub next_buf_id: usize,
    pub machines: HashMap<MachId, Machine>,
    pub buffers: HashMap<BufId, Buffer>,
    pub on_change: Option<fn(&mut Self)>,
}

impl Default for Sim {
    fn default() -> Self {
        Self {
            time: 0.0,
            events: BinaryHeap::new(),
            next_event_seq: 0,
            next_mach_id: 0,
            next_buf_id: 0,
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
        if event.time <= self.time {
            panic!("Tried to schedule event in the past or instantaneously");
        }
        self.events.push(Event {
            seq: Some(self.next_event_seq),
            ..event
        });
        self.next_event_seq += 1;
    }

    pub fn schedule_at(&mut self, time: f64, kind: EventKind) {
        let e = Event {
            time,
            kind,
            seq: Some(self.next_event_seq),
        };
        self.schedule(e);
    }

    pub fn schedule_in(&mut self, time: f64, kind: EventKind) {
        self.schedule_at(self.time + time, kind);
    }

    fn handle_event(&mut self, event: Event) {
        match event.kind {
            EventKind::TryStart(mid) => self.handle_try_start(mid),
            EventKind::Finish(mid) => self.handle_finish(mid),
            EventKind::SetBuffer(bid, amount) => self.handle_set_buffer(bid, amount),
            EventKind::ClearBuffer(bid) => self.handle_set_buffer(bid, 0),
            EventKind::AddToBuffer(bid, amount) => self.handle_add_to_buffer(bid, amount),
            EventKind::TakeFromBuffer(bid, amount) => self.handle_take_from_buffer(bid, amount),
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
