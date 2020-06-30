use crate::common::BUS_WIDTH;
use crate::units::bit::Bit;
use std::cell::RefCell;
use std::iter::repeat_with;
use std::iter::IntoIterator;
use std::ops::Deref;
use std::ops::Index;
use std::ops::IndexMut;
use std::rc::Rc;
use std::slice::Iter;
use std::cmp::Ordering;

/// A wire is a basic unit containing only one bit at a time.
#[derive(Debug)]
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
impl Clone for Wire {
    fn clone(&self) -> Self {
        Wire(Rc::clone(&self.0))
    }
}
impl PartialEq for Wire {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Wire(rc1), Wire(rc2)) => rc1.as_ptr() == rc2.as_ptr(),
        }
    }
}
impl PartialOrd for Wire {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Wire(rc1), Wire(rc2)) => rc1.as_ptr().partial_cmp(&rc2.as_ptr()),
        }
    }
}
impl Eq for Wire {}
impl Ord for Wire {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Wire(rc1), Wire(rc2)) => rc1.as_ptr().cmp(&rc2.as_ptr()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Bus(Vec<Wire>); // length = BUS_WIDTH
impl Bus {
    pub fn new() -> Self {
        Bus(repeat_with(Wire::default).take(BUS_WIDTH).collect())
    }

    pub fn iter(&self) -> Iter<Wire> {
        self.0.iter()
    }
}
impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}
impl IntoIterator for Bus {
    type Item = Wire;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl Index<usize> for Bus {
    type Output = Wire;

    fn index(&self, i: usize) -> &Self::Output {
        self.0.index(i)
    }
}
impl IndexMut<usize> for Bus {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        self.0.index_mut(i)
    }
}
impl From<Vec<Wire>> for Bus {
    fn from(wires: Vec<Wire>) -> Self {
        assert_eq!(wires.len(), BUS_WIDTH);

        Bus(wires)
    }
}
impl Into<Vec<Wire>> for Bus {
    fn into(self) -> Vec<Wire> {
        self.0
    }
}
