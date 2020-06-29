use crate::common::BUS_WIDTH;
use crate::units::bit::Bit;
#[cfg(test)]
use crate::units::bit::{OFF, ON};
use std::cell::RefCell;
use std::iter::repeat_with;
use std::iter::IntoIterator;
use std::ops::Deref;
use std::ops::Index;
use std::ops::IndexMut;
use std::rc::Rc;
use std::slice::Iter;

/// A wire is a basic unit containing only one bit at a time.
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
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

#[cfg(test)]
#[test]
fn test_bus() {
    let b = Bus::default();
    assert_eq!(b[0].state(), OFF);
    b[0].set(ON);
    assert_eq!(b[0].state(), ON);
    assert_eq!(b.clone()[0].state(), ON);
    assert_eq!(
        b.iter().map(|w| w.state()).collect::<Vec<_>>(),
        vec![ON, OFF, OFF, OFF, OFF, OFF, OFF, OFF]
    );

    let b = Bus::from(
        vec![ON, OFF, ON, OFF, ON, OFF, OFF, OFF]
            .into_iter()
            .map(|s| {
                let w = Wire::default();
                w.set(s);
                w
            })
            .collect::<Vec<_>>(),
    );
    assert_eq!(
        b.iter().map(|w| w.state()).collect::<Vec<_>>(),
        vec![ON, OFF, ON, OFF, ON, OFF, OFF, OFF]
    );
}
