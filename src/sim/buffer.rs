use std::fmt::Display;

use crate::sim::{SimError, ids::ItemId};
use tracing::debug;

#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn set_amount(&mut self, amount: usize) -> Result<(), SimError> {
        debug!("buffer.set_amount called");
        if amount > self.capacity {
            Err(SimError::BufferOverflow(self.clone()))
        } else {
            self.amount = amount;
            Ok(())
        }
    }

    pub fn add_amount(&mut self, amount: usize) -> Result<(), SimError> {
        self.set_amount(self.amount + amount)
    }

    pub fn take_amount(&mut self, amount: usize) -> Result<(), SimError> {
        if self.amount >= amount {
            self.set_amount(self.amount - amount)
        } else {
            Err(SimError::BufferUnderflow(self.clone()))
        }
    }
}

impl Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}*{}", self.item, self.capacity)
    }
}

#[cfg(test)]
mod tests {
    use crate::sim::*;

    #[test]
    fn create_buffer() {
        let b = Buffer::new(100, 0, ids::ItemId(0));
        assert_eq!(
            b,
            Buffer {
                capacity: 100,
                amount: 0,
                item: ids::ItemId(0)
            }
        );
    }

    #[test]
    fn set_amount() {
        let mut b = Buffer::new(100, 0, ids::ItemId(0));

        // Normal case, should pass
        let res = b.set_amount(10);
        assert!(res.is_ok());
        assert_eq!(b.amount, 10);

        // overflow, should fail
        let res = b.set_amount(101);
        assert!(matches!(res, Err(SimError::BufferOverflow(_))));

        // edge cases, should work
        let res = b.set_amount(0);
        assert!(res.is_ok());
        assert_eq!(b.amount, 0);

        let res = b.set_amount(100);
        assert!(res.is_ok());
        assert_eq!(b.amount, 100);
    }

    #[test]
    fn add_amount() {
        let mut b = Buffer::new(100, 0, ids::ItemId(0));

        let res = b.add_amount(10);
        assert!(res.is_ok());
        assert_eq!(b.amount, 10);

        let res = b.add_amount(100);
        assert!(matches!(res, Err(SimError::BufferOverflow(_))));

        // Should work
        let res = b.add_amount(0);
        assert!(res.is_ok());
        assert_eq!(b.amount, 10);

        // Should also work, edge case
        let res = b.add_amount(90);
        assert!(res.is_ok());
        assert_eq!(b.amount, 100);
    }

    #[test]
    fn take_amount() {
        let mut b = Buffer::new(100, 0, ids::ItemId(0));

        let _ = b.set_amount(95);

        assert!(b.take_amount(2).is_ok());
        assert_eq!(b.amount, 93);

        assert!(matches!(
            b.take_amount(94),
            Err(SimError::BufferUnderflow(_))
        ));

        assert!(b.take_amount(93).is_ok());
        assert_eq!(b.amount, 0);
    }
}
