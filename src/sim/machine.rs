use crate::sim::BufId;

#[derive(Debug)]
pub struct Machine {
    pub input: BufId,
    pub output: BufId,
    pub min_input: usize,
    pub output_amount: usize,
    pub speed: f64, // Items per second
    pub busy: bool,
}

impl Machine {
    pub fn new(input: BufId, output: BufId, speed: f64) -> Self {
        if speed <= 0.0 {
            panic!("Tried to create machine with non-positive speed");
        }
        Self {
            input,
            output,
            min_input: 1,
            output_amount: 1,
            speed,
            busy: false,
        }
    }

    pub fn with_min_input(&self, min_input: usize) -> Self {
        Self { min_input, ..*self }
    }
    pub fn with_output_amount(&self, output_amount: usize) -> Self {
        Self {
            output_amount,
            ..*self
        }
    }
}
