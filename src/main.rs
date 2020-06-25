use vcpu::circuit::{gate::ANDGate, wire::Wire};
use vcpu::units::bit::{OFF, ON};

fn main() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let out = Wire::default();
    in2.set(ON);
    // in1 is OFF

    let and = ANDGate::new(in1, in2, out.clone());
    and.run();

    let out_bit = out.state();
    assert_eq!(OFF, out_bit);

    println!("Done!");
}
