use std::collections::HashMap;

use crate::sim::{BufId, Sim, ids::ItemId};

#[derive(Debug, Clone)]
pub struct Machine {
    pub input: HashMap<ItemId, BufId>,
    pub output: HashMap<ItemId, BufId>,
    pub requires: Vec<(ItemId, usize)>,
    pub creates: Vec<(ItemId, usize)>,
    pub speed: f64, // Items per second
    pub busy: bool,
}

impl Machine {
    pub fn new(requires: Vec<(ItemId, usize)>, creates: Vec<(ItemId, usize)>) -> Self {
        Self {
            input: HashMap::new(),
            output: HashMap::new(),
            requires,
            creates,
            speed: 1.0,
            busy: false,
        }
    }

    pub fn with_speed(self, speed: f64) -> Self {
        Self { speed, ..self }
    }

    pub fn add_input(&self, sim: &Sim, b: BufId) -> Self {
        let mut res = self.clone();
        let item = sim.buffers.get(&b).unwrap().item;
        res.input.insert(item, b);
        res
    }

    pub fn add_output(&self, sim: &Sim, b: BufId) -> Self {
        let mut res = self.clone();
        let item = sim.buffers.get(&b).unwrap().item;
        res.output.insert(item, b);
        res
    }

    pub fn is_connected(&self) -> bool {
        for (id, _) in &self.requires {
            // make sure there's a corresponding BufId
            match self.input.get(id) {
                Some(_) => {}
                None => return false,
            }
        }

        for (id, _) in &self.creates {
            // make sure there's a corresponding BufId
            match self.output.get(id) {
                Some(_) => {}
                None => return false,
            }
        }
        true
    }
}
