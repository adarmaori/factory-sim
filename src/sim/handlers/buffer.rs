use crate::sim::{BufId, Event, EventKind, MachId, Sim};
use tracing::{debug, error, info, trace, warn};

impl Sim {
    fn set_buffer(&mut self, b: BufId, amount: usize) {
        let buf = self.buffers.get_mut(&b).unwrap();
        if amount > buf.capacity {
            error!(
                ?b,
                time = self.time,
                amount = amount,
                capacity = buf.capacity,
                "tried to overfill buffer"
            );
        } else {
            buf.amount = amount;
            // Send signal to all subscribers
        }
    }
    pub(crate) fn handle_set_buffer(&mut self, b: BufId, amount: usize) {
        debug!(?b, time = self.time, "set_buffer");
        self.set_buffer(b, amount);
        info!(?b, time = self.time, "setting buffer");
    }
}
