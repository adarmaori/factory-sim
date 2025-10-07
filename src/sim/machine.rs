use crate::sim::BufId;

#[derive(Debug)]
pub struct Machine {
    pub input: BufId,
    pub output: BufId,
    pub speed: f64, // Items per second
    pub busy: bool,
}

impl Machine {
    pub fn new(input: BufId, output: BufId, speed: f64) -> Self {
        Self {
            input,
            output,
            speed,
            busy: false,
        }
    }
}
