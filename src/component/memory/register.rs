use crate::circuit::wire::{Bus, Wire};
use crate::component::enabler::Enabler;
use crate::component::memory::byte::MemoryByte;

pub struct Register {
    mem: MemoryByte,
    en: Enabler,
}
impl Register {
    pub fn new(is: Bus, s: Wire, e: Wire, os: Bus) -> Self {
        let tmp = Bus::new();

        Register {
            mem: MemoryByte::new(is, s, tmp.clone()),
            en: Enabler::new(tmp, e, os),
        }
    }

    pub fn run(&self) {
        self.mem.run();
        self.en.run();
    }
}
