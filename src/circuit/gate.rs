use crate::circuit::wire::Wire;
use crate::units::bit::{OFF, ON};

// NOTE: When implementing gates, despite the fact that our bits are represented currently as bools,
// we won't be using any of the boolean operators. In fact, we will code them all as if they never existed.

/// Truth table:
///
///   a  |  b  |  c
///  ON  | ON  | OFF
///  OFF | OFF | ON
pub struct NOTGate {
    nand: NANDGate,
}
impl NOTGate {
    pub fn new(a: Wire, c: Wire) -> Self {
        NOTGate {
            nand: NANDGate::new(a.clone(), a, c),
        }
    }

    pub fn run(self) {
        self.nand.run();
    }
}

/// Truth table:
///
///  a  |  b  |  c
/// ON  | ON  | ON
/// OFF | OFF | OFF
/// OFF | ON  | OFF
/// ON  | OFF | OFF
pub struct ANDGate {
    nand: NANDGate,
    not: NOTGate,
}
impl ANDGate {
    pub fn new(a: Wire, b: Wire, c: Wire) -> Self {
        let x = Wire::default();
        ANDGate {
            nand: NANDGate::new(a, b, x.clone()),
            not: NOTGate::new(x, c),
        }
    }

    pub fn run(self) {
        self.nand.run();
        self.not.run();
    }
}

/// Truth table:
///
///  a  |  b  |  c
/// OFF | OFF | ON
/// OFF | ON  | OFF
/// ON  | OFF | OFF
/// ON  | ON  | OFF
pub struct NANDGate {
    in1: Wire,
    in2: Wire,
    out: Wire,
}
impl NANDGate {
    pub fn new(a: Wire, b: Wire, c: Wire) -> Self {
        NANDGate {
            in1: a,
            in2: b,
            out: c,
        }
    }

    pub fn run(self) {
        self.out
            .set(if self.in1.state() == ON && self.in2.state() == ON {
                OFF
            } else {
                ON
            });
    }
}

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
            g1: NANDGate {
                in1: i,
                in2: s.clone(),
                out: a.clone(),
            },
            g2: NANDGate {
                in1: a.clone(),
                in2: s,
                out: b.clone(),
            },
            g3: NANDGate {
                in1: a,
                in2: c.clone(),
                out: o.clone(),
            },
            g4: NANDGate {
                in1: o,
                in2: b,
                out: c,
            },
        }
    }

    pub fn run(self) {
        self.g1.run();
        self.g2.run();
        self.g3.run();
        self.g4.run();
    }
}
