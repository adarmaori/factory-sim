use crate::prelude::{BufId, MachId};
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventKind {
    TryStart(MachId),             // Starting a build process
    Finish(MachId),               // Ending a build process
    ClearBuffer(BufId),           // Removing everything from a buffer to allow refilling
    SetBuffer(BufId, usize),      // Add some items to the buffer
    AddToBuffer(BufId, usize),    // Add some items to the buffer
    TakeFromBuffer(BufId, usize), // Add some items to the buffer
}

#[derive(Debug)]
pub struct Event {
    pub time: f64,
    pub seq: Option<usize>,
    pub kind: EventKind,
}

impl PartialEq for Event {
    fn eq(&self, o: &Self) -> bool {
        self.time == o.time
    }
}
impl Eq for Event {}
impl PartialOrd for Event {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for Event {
    fn cmp(&self, o: &Self) -> Ordering {
        if self.seq.is_none() || o.seq.is_none() {
            panic!("Tried to compare events with no seq value");
        }
        if self.seq == o.seq {
            panic!("Tried to compare events with the same seq value");
        }
        o.time
            .partial_cmp(&self.time)
            .unwrap_or(self.seq.partial_cmp(&o.seq).unwrap())
    }
}
