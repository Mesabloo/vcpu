use vcpu::circuit::{gate::ANDGate, wire::WireRef};
use vcpu::units::bit::{OFF, ON};

fn main() {
    let in1 = WireRef::default();
    let in2 = WireRef::default();
    let out = WireRef::default();
    in2.borrow_mut().set(ON);
    // in1 is OFF

    let and = ANDGate::new(in1, in2, out.clone());
    and.run();

    let out_bit = out.borrow().state();
    assert_eq!(OFF, out_bit);

    println!("Done!");
}
