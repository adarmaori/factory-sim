use crate::sim::{BufId, Buffer, Sim, SimError};
use tracing::debug;

impl Sim {
    pub fn add_buffer(&mut self, buf: Buffer) -> BufId {
        let id = BufId(self.next_buf_id);
        self.buffers.insert(id, buf);
        self.next_buf_id += 1;
        id
    }

    pub fn get_buffer(&self, b: BufId) -> Result<&Buffer, SimError> {
        if let Some(res) = self.buffers.get(&b) {
            Ok(res)
        } else {
            Err(SimError::BufferDoesntExist(b))
        }
    }

    pub fn get_buffer_mut(&mut self, b: BufId) -> Result<&mut Buffer, SimError> {
        if let Some(res) = self.buffers.get_mut(&b) {
            Ok(res)
        } else {
            Err(SimError::BufferDoesntExist(b))
        }
    }

    pub(crate) fn handle_set_buffer(&mut self, b: BufId, amount: usize) -> Result<bool, SimError> {
        debug!(?b, time = self.time, "set_buffer to {}", amount);
        let buf = self.get_buffer_mut(b)?;
        match buf.set_amount(amount) {
            Ok(()) => Ok(true),
            Err(e) => Err(e),
        }
    }

    pub(crate) fn handle_add_to_buffer(
        &mut self,
        b: BufId,
        amount: usize,
    ) -> Result<bool, SimError> {
        debug!(?b, time = self.time, "adding {} to buffer", amount);
        let buf = self.get_buffer_mut(b)?;
        match buf.add_amount(amount) {
            Ok(()) => Ok(true),
            Err(e) => Err(e),
        }
    }

    pub(crate) fn handle_take_from_buffer(
        &mut self,
        b: BufId,
        amount: usize,
    ) -> Result<bool, SimError> {
        debug!(?b, time = self.time, "taking {} from buffer", amount);
        let buf = self.get_buffer_mut(b)?;
        match buf.take_amount(amount) {
            Ok(()) => Ok(true),
            Err(e) => Err(e),
        }
    }
}
