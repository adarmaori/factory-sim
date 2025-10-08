use crate::sim::{BufId, Buffer, Sim};
use tracing::{debug, error};

impl Sim {
    pub fn add_buffer(&mut self, buf: Buffer) -> BufId {
        let id = BufId(self.next_buf_id);
        self.buffers.insert(id, buf);
        self.next_buf_id += 1;
        id
    }
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
        }
    }
    pub(crate) fn handle_set_buffer(&mut self, b: BufId, amount: usize) {
        debug!(?b, time = self.time, "set_buffer to {}", amount);
        self.set_buffer(b, amount);
    }

    pub(crate) fn handle_add_to_buffer(&mut self, b: BufId, amount: usize) {
        debug!(?b, time = self.time, "adding {} to buffer", amount);
        let current = self.buffers.get(&b).unwrap().amount;
        self.set_buffer(b, amount + current);
    }

    pub(crate) fn handle_take_from_buffer(&mut self, b: BufId, amount: usize) {
        debug!(?b, time = self.time, "adding {} to buffer", amount);
        let current = self.buffers.get(&b).unwrap().amount;
        if current < amount {
            error!(
                ?b,
                time = self.time,
                amount = amount,
                exists = current,
                "tried to take from buffer more than buffer has"
            );
        } else {
            self.set_buffer(b, current - amount);
        }
    }
}
