use vcpu::circuit::{wire::Wire, gate::ANDGate};
use vcpu::units::bit::{ON, OFF};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let in1 = Rc::new(RefCell::new(Wire::default()));
    let in2 = Rc::new(RefCell::new(Wire::default()));
    let out = Rc::new(RefCell::new(Wire::default()));
    in1.borrow_mut().set(OFF);
    in2.borrow_mut().set(ON);
    out.borrow_mut().set(OFF);

    let and = ANDGate::new(in1, in2, out.clone());
    and.run();

    let out_bit = out.borrow().state();
    assert_eq!(OFF, out_bit);

    println!("Done!");
}
