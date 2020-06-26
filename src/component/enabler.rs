use crate::circuit::gate::ANDGate;
use crate::circuit::wire::{Bus, Wire};

pub struct Enabler {
    gates: Vec<ANDGate>, // must be of a length of "BUS_WIDTH"
}
impl Enabler {
    pub fn new(is: Bus, e: Wire, os: Bus) -> Self {
        Enabler {
            gates: is
                .into_iter()
                .zip(os.into_iter())
                .map(|(i, o)| ANDGate::new(i, e.clone(), o))
                .collect(),
        }
    }

    pub fn run(&self) {
        self.gates.iter().for_each(|g| g.run());
    }
}
