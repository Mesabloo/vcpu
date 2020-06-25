use crate::units::bit::Bit;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

/// A wire is a basic unit containing only one bit at a time.
pub struct Wire(pub Bit);

impl Wire {
    pub fn set(&mut self, b: Bit) -> () {
        self.0 = b;
    }

    pub fn state(&self) -> Bit {
        self.0
    }
}

impl Default for Wire {
    fn default() -> Wire {
        Wire(Bit::default())
    }
}

#[derive(Clone)]
pub struct WireRef(Rc<RefCell<Wire>>);
impl Default for WireRef {
    fn default() -> WireRef {
        WireRef(Rc::new(RefCell::new(Wire::default())))
    }
}
impl Deref for WireRef {
    type Target = <Rc<RefCell<Wire>> as Deref>::Target;

    fn deref(&self) -> &Self::Target {
        match self {
            WireRef(rc) => rc.deref(),
        }
    }
}
