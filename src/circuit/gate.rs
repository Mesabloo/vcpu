pub trait Computeable<'a> {
    fn compute(self);
}

use crate::circuit::wire::Wire;

pub struct Gate<'a> {
    a: &'a Wire,
    b: &'a Wire,
    c: &'a mut Wire,
}
// NOTE: When implementing gates, despite the fact that our bits are represented currently as bools,
// we won't be using any of the boolean operators. In fact, we will code them all as if they never existed.

use crate::units::bit::{OFF, ON};

pub struct NotGate<'a>(Gate<'a>);
impl<'a> Computeable<'a> for NotGate<'a> {
    fn compute(self) {
        // We do not use the "b" argument because "a == b"

        // Truth table:
        //
        //   a  |  b  |  c
        //  ON  | ON  | OFF
        //  OFF | OFF | ON

        self.0.c.set(if self.0.a.state() == ON { OFF } else { ON });
    }
}

