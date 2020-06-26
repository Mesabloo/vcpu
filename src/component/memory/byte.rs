use crate::circuit::wire::{Bus, Wire};
use crate::component::memory::bit::MemoryBit;

pub struct MemoryByte {
    mem: Vec<MemoryBit>,
}
impl MemoryByte {
    pub fn new(is: Bus, s: Wire, os: Bus) -> Self {
        MemoryByte {
            mem: is
                .into_iter()
                .zip(os.into_iter())
                .map(|(i, o)| MemoryBit::new(i, s.clone(), o))
                .collect(),
        }
    }

    pub fn run(&self) {
        self.mem.iter().for_each(|m| m.run());
    }
}
