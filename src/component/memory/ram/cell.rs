use crate::circuit::gate::ANDGate;
use crate::component::memory::register::Register;
use crate::circuit::wire::{Bus, Wire};

pub struct Cell {
    x: ANDGate,
    s: ANDGate,
    e: ANDGate,
    r: Register,
}
impl Cell {
    pub fn new(in1: Wire, in2: Wire, s: Wire, e: Wire, out: Bus) -> Self {
        let x_out = Wire::default();
        let s_out = Wire::default();
        let e_out = Wire::default();

        Cell {
            x: ANDGate::new(in1, in2, x_out.clone()),
            s: ANDGate::new(x_out.clone(), s, s_out.clone()),
            e: ANDGate::new(x_out, e, e_out.clone()),
            r: Register::new(out.clone(), s_out, e_out, out),
        }
    }

    pub fn run(&self) {
        self.x.run();
        self.s.run();
        self.e.run();
        self.r.run();
    }
}