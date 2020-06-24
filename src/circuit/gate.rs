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
