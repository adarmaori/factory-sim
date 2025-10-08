use crate::sim::ids::ItemId;

#[derive(Debug)]
pub struct Buffer {
    pub capacity: usize,
    pub amount: usize,
    pub item: ItemId,
}

impl Buffer {
    pub fn new(capacity: usize, amount: usize, item: ItemId) -> Self {
        Self {
            capacity,
            amount,
            item,
        }
    }
}
