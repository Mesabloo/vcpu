use crate::circuit::wire::{Bus, Wire};
use crate::common::BUS_WIDTH;

pub struct LeftShifter {
    bus1: Bus,
    bus2: Bus,
    shift_in: Wire,
    shift_out: Wire,
}
impl LeftShifter {
    pub fn new(i: Bus, o: Bus, shift_in: Wire, shift_out: Wire) -> Self {
        LeftShifter {
            bus1: i,
            bus2: o,
            shift_in,
            shift_out,
        }
    }

    pub fn run(&self) {
        self.shift_out.set(self.bus1[0].state());
        for idx in 1..BUS_WIDTH {
            self.bus2[idx - 1].set(self.bus1[idx].state());
        }
        self.bus2[BUS_WIDTH - 1].set(self.shift_in.state());
    }
}

pub struct RightShifter {
    bus1: Bus,
    bus2: Bus,
    shift_in: Wire,
    shift_out: Wire,
}
impl RightShifter {
    pub fn new(i: Bus, o: Bus, shift_in: Wire, shift_out: Wire) -> Self {
        RightShifter {
            bus1: i,
            bus2: o,
            shift_in,
            shift_out,
        }
    }

    pub fn run(&self) {
        self.shift_out.set(self.bus1[BUS_WIDTH - 1].state());
        for idx in (1..BUS_WIDTH).rev() {
            self.bus2[idx].set(self.bus1[idx - 1].state());
        }
        self.bus2[0].set(self.shift_in.state());
    }
}
