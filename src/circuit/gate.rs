use crate::circuit::wire::Wire;

// NOTE: When implementing gates, despite the fact that our bits are represented currently as bools,
// we won't be using any of the boolean operators. In fact, we will code them all as if they never existed.

use crate::units::bit::{OFF, ON};

/// Truth table:
///
///   a  |  b  |  c
///  ON  | ON  | OFF
///  OFF | OFF | ON
pub fn not<'a>(a: &'a Wire, _b: &'a Wire, c: &'a mut Wire) {
    // We do not use the "b" argument because "a == b"
    c.set(if a.state() == ON { OFF } else { ON });
    // This could be written:
    // > nand(a, a, c)
    // because the "a" and "b" wires are the same.
}

/// Truth table:
///
///  a  |  b  |  c
/// ON  | ON  | ON
/// OFF | OFF | OFF
/// OFF | ON  | OFF
/// ON  | OFF | OFF
pub fn and<'a>(a: &'a Wire, b: &'a Wire, c: &'a mut Wire) {
    c.set(if a.state() == ON && b.state() == ON { ON } else { OFF });
    // This could be written:
    // > nand(a, b, c);
    // > not(c, c, c);
    // but it kinda sucks, because we can do the computation directly
    // and this also semantically means that the input wire of our not gate is the same
    // as the two input ones. This doesn't make a lot of sense.
}

/// Truth table:
///
///  a  |  b  |  c
/// OFF | OFF | ON
/// OFF | ON  | OFF
/// ON  | OFF | OFF
/// ON  | ON  | OFF
pub fn nand<'a>(a: &'a Wire, b: &'a Wire, c: &'a mut Wire) {
    c.set(if a.state() == OFF && b.state() == OFF { ON } else { OFF });
}
