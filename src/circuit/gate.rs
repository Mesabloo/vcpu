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

