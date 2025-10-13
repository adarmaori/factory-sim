use thiserror::Error;

use crate::sim::{BufId, Buffer, MachId, ids::ItemId};

#[derive(Error, Debug)]
pub enum SimError {
    #[error("Machine does not exist: {0}")]
    MachineDoesntExist(MachId),

    #[error("Buffer does not exist: {0}")]
    BufferDoesntExist(BufId),

    #[error("Item has no buffer connected to it: {0}")]
    ItemHasNoBuffer(ItemId),

    #[error("Trying to overfill a buffer: {0}")]
    BufferOverflow(Buffer),

    #[error("Invalid command: {0}")]
    InvalidCommand(String),

    #[error("Trying to take more than buffer has: {0}")]
    BufferUnderflow(Buffer),
}
