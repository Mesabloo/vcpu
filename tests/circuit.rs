use vcpu::circuit::wire::*;
use vcpu::units::bit::{OFF, ON};

#[test]
fn wire_state_kept() {
    let w = Wire::default();
    
    assert_eq!(w.state(), OFF);

    w.set(ON);

    assert_eq!(w.state(), ON);
}

#[test]
fn wire_state_shared() {
    let w1 = Wire::default();
    let w2 = w1.clone();

    assert_eq!(w1.state(), w2.state());

    w1.set(ON);

    assert_eq!(w1.state(), w2.state());

    // This is useful because we do want a wire and its clone to point to the same wire
    // (it's some sort of a way to bypass the borrow checker, in fact, because we want to be able to use some wires multiple times and modify theù)
}