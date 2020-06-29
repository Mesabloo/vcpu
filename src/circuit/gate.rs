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

    pub fn run(&self) {
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

    pub fn run(&self) {
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

    pub fn run(&self) {
        self.out
            .set(if self.in1.state() == ON && self.in2.state() == ON {
                OFF
            } else {
                ON
            });
    }
}

pub struct MultiANDGate {
    ands: Vec<ANDGate>, // at least 1 gate
}
impl MultiANDGate {
    pub fn new(is: Vec<Wire>, out: Wire) -> Self {
        let n = is.len();
        assert!(n >= 2);

        let mut ands = vec![];
        if is.len() == 2 {
            ands.push(ANDGate::new(is[0].clone(), is[1].clone(), out));
        } else {
            let in1 = is[n - 1].clone();
            let in2 = is.into_iter().take(n - 1).fold_first(|in1, in2| {
                let out = Wire::default();
                ands.push(ANDGate::new(in1, in2, out.clone()));
                out
            }).unwrap();
            ands.push(ANDGate::new(in1, in2, out));
        }

        MultiANDGate { ands }
    }

    pub fn run(&self) {
        self.ands.iter().for_each(|g| g.run());
    }
}

/// Truth table:
///
///  a  |  b  |  c
/// OFF | OFF | OFF
/// OFF | ON  | ON
/// ON  | OFF | ON
/// ON  | ON  | ON
pub struct ORGate {
    not1: NOTGate,
    not2: NOTGate,
    nand: NANDGate,
}
impl ORGate {
    pub fn new(a: Wire, b: Wire, e: Wire) -> Self {
        let (c, d) = (Wire::default(), Wire::default());

        ORGate {
            not1: NOTGate::new(a, c.clone()),
            not2: NOTGate::new(b, d.clone()),
            nand: NANDGate::new(c, d, e),
        }
    }

    pub fn run(&self) {
        self.not1.run();
        self.not2.run();
        self.nand.run();
    }
}

/// Truth table:
///
///  a  |  b  |  c
/// OFF | OFF | OFF
/// OFF | ON  | ON
/// ON  | OFF | ON
/// ON  | ON  | OFF
pub struct XORGate {
    not1: NOTGate,
    not2: NOTGate,
    nand3: NANDGate,
    nand4: NANDGate,
    nand5: NANDGate,
}
impl XORGate {
    pub fn new(a: Wire, b: Wire, g: Wire) -> Self {
        let c = Wire::default();
        let d = Wire::default();
        let e = Wire::default();
        let f = Wire::default();

        XORGate {
            not1: NOTGate::new(a.clone(), c.clone()),
            not2: NOTGate::new(b.clone(), d.clone()),
            nand3: NANDGate::new(c, b, e.clone()),
            nand4: NANDGate::new(a, d, f.clone()),
            nand5: NANDGate::new(e, f, g),
        }
    }

    pub fn run(&self) {
        self.not1.run();
        self.not2.run();
        self.nand3.run();
        self.nand4.run();
        self.nand5.run();
    }
}