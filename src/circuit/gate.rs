use crate::circuit::wire::Wire;

// NOTE: When implementing gates, despite the fact that our bits are represented currently as bools,
// we won't be using any of the boolean operators. In fact, we will code them all as if they never existed.

use crate::units::bit::{OFF, ON};

use std::cell::RefCell;
use std::rc::Rc;

/// Truth table:
///
///   a  |  b  |  c
///  ON  | ON  | OFF
///  OFF | OFF | ON
pub struct NOTGate {
    nand: NANDGate,
}
impl NOTGate {
    pub fn new(a: Rc<RefCell<Wire>>, c: Rc<RefCell<Wire>>) -> Self {
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
    pub fn new(a: Rc<RefCell<Wire>>, b: Rc<RefCell<Wire>>, c: Rc<RefCell<Wire>>) -> Self {
        let x = Rc::new(RefCell::new(Wire::default()));
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
    in1: Rc<RefCell<Wire>>,
    in2: Rc<RefCell<Wire>>,
    out: Rc<RefCell<Wire>>,
}
impl NANDGate {
    pub fn new(a: Rc<RefCell<Wire>>, b: Rc<RefCell<Wire>>, c: Rc<RefCell<Wire>>) -> Self {
        NANDGate {
            in1: a,
            in2: b,
            out: c,
        }
    }

    pub fn run(self) {
        (*self.out).borrow_mut().set(
            if (*self.in1).borrow().state() == ON && (*self.in2).borrow().state() == ON {
                OFF
            } else {
                ON
            },
        );
    }
}
