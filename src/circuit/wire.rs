use crate::units::bit::Bit;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

/// A wire is a basic unit containing only one bit at a time.
#[derive(Clone)]
pub struct Wire(Rc<RefCell<Bit>>);
impl Wire {
    pub fn set(&self, b: Bit) {
        match self {
            Wire(rc) => {
                *rc.borrow_mut() = b;
            }
        }
    }

    pub fn state(&self) -> Bit {
        match self {
            Wire(rc) => *rc.borrow(),
        }
    }
}
impl Default for Wire {
    fn default() -> Wire {
        Wire(Rc::new(RefCell::new(Bit::default())))
    }
}
impl Deref for Wire {
    type Target = <Rc<RefCell<Bit>> as Deref>::Target;

    fn deref(&self) -> &Self::Target {
        match self {
            Wire(rc) => rc.deref(),
        }
    }
}
