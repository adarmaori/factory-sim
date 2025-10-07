#[derive(Debug)]
pub struct Buffer {
    pub capacity: usize,
    pub amount: usize,
}

impl Buffer {
    pub fn new(capacity: usize, amount: usize) -> Self {
        Self { capacity, amount }
    }
}
