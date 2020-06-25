use crate::circuit::gate::NANDGate;
use crate::circuit::wire::Wire;

pub struct MemoryGate {
    g1: NANDGate,
    g2: NANDGate,
    g3: NANDGate,
    g4: NANDGate,
}
impl MemoryGate {
    pub fn new(i: Wire, s: Wire, o: Wire) -> Self {
        let a = Wire::default();
        let b = Wire::default();
        let c = Wire::default();

        MemoryGate {
            g1: NANDGate::new(i, s.clone(), a.clone()),
            g2: NANDGate::new(a.clone(), s, b.clone()),
            g3: NANDGate::new(a, c.clone(), o.clone()),
            g4: NANDGate::new(o, b, c),
        }
    }

    pub fn run(&self) {
        self.g1.run();
        self.g2.run();
        self.g4.run();
        self.g3.run();
    }
}
