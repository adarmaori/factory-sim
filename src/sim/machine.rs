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
    pub fn new(
        input: BufId,
        output: BufId,
        min_input: usize,
        output_amount: usize,
        speed: f64,
    ) -> Self {
        Self {
            input,
            output,
            min_input,
            output_amount,
            speed,
            busy: false,
        }
    }
}
