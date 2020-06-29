#![cfg(test)]

use vcpu::circuit::{gate::*, wire::*};
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

#[test]
fn bus_state_kept() {
    let b = Bus::default();
    b[0].set(ON);

    assert_eq!(
        b.iter().map(|w| w.state()).collect::<Vec<_>>(),
        vec![ON, OFF, OFF, OFF, OFF, OFF, OFF, OFF]
    );
}

#[test]
fn bus_state_shared() {
    let b1 = Bus::default();
    let b2 = b1.clone();

    b1[2].set(ON);

    assert_eq!(b1[2], b2[2]);

    let inner_b1: Vec<Wire> = b1.into();
    let inner_b2: Vec<Wire> = b2.into();
    assert_eq!(inner_b1, inner_b2);
}

#[test]
fn notgate_input_on() {
    let in1 = Wire::default();
    let out = Wire::default();
    let not = NOTGate::new(in1.clone(), out.clone());

    in1.set(ON);
    not.run();

    assert_eq!(out.state(), OFF);
}

#[test]
fn notgate_input_off() {
    let in1 = Wire::default();
    let out = Wire::default();
    let not = NOTGate::new(in1.clone(), out.clone());

    in1.set(OFF);
    not.run();

    assert_eq!(out.state(), ON);
}

#[test]
fn nandgate_input_off_off() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let out = Wire::default();
    let gate = NANDGate::new(in1.clone(), in2.clone(), out.clone());

    in1.set(OFF);
    in2.set(OFF);
    gate.run();

    assert_eq!(out.state(), ON);
}

#[test]
fn nandgate_input_off_on() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let out = Wire::default();
    let gate = NANDGate::new(in1.clone(), in2.clone(), out.clone());

    in1.set(OFF);
    in2.set(ON);
    gate.run();

    assert_eq!(out.state(), ON);
}

#[test]
fn nandgate_input_on_off() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let out = Wire::default();
    let gate = NANDGate::new(in1.clone(), in2.clone(), out.clone());

    in1.set(ON);
    in2.set(OFF);
    gate.run();

    assert_eq!(out.state(), ON);
}

#[test]
fn nandgate_input_on_on() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let out = Wire::default();
    let gate = NANDGate::new(in1.clone(), in2.clone(), out.clone());

    in1.set(ON);
    in2.set(ON);
    gate.run();

    assert_eq!(out.state(), OFF);
}

#[test]
fn andgate_input_on_on() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let out = Wire::default();
    let gate = ANDGate::new(in1.clone(), in2.clone(), out.clone());

    in1.set(ON);
    in2.set(ON);
    gate.run();

    assert_eq!(out.state(), ON);
}

#[test]
fn andgate_input_off_on() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let out = Wire::default();
    let gate = ANDGate::new(in1.clone(), in2.clone(), out.clone());

    in1.set(OFF);
    in2.set(ON);
    gate.run();

    assert_eq!(out.state(), OFF);
}

#[test]
fn andgate_input_on_off() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let out = Wire::default();
    let gate = ANDGate::new(in1.clone(), in2.clone(), out.clone());

    in1.set(ON);
    in2.set(OFF);
    gate.run();

    assert_eq!(out.state(), OFF);
}

#[test]
fn andgate_input_off_off() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let out = Wire::default();
    let gate = ANDGate::new(in1.clone(), in2.clone(), out.clone());

    in1.set(OFF);
    in2.set(OFF);
    gate.run();

    assert_eq!(out.state(), OFF);
}

#[test]
fn orgate_input_on_on() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let out = Wire::default();
    let gate = ORGate::new(in1.clone(), in2.clone(), out.clone());

    in1.set(ON);
    in2.set(ON);
    gate.run();

    assert_eq!(out.state(), ON);
}

#[test]
fn orgate_input_off_on() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let out = Wire::default();
    let gate = ORGate::new(in1.clone(), in2.clone(), out.clone());

    in1.set(OFF);
    in2.set(ON);
    gate.run();

    assert_eq!(out.state(), ON);
}

#[test]
fn orgate_input_on_off() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let out = Wire::default();
    let gate = ORGate::new(in1.clone(), in2.clone(), out.clone());

    in1.set(ON);
    in2.set(OFF);
    gate.run();

    assert_eq!(out.state(), ON);
}

#[test]
fn orgate_input_off_off() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let out = Wire::default();
    let gate = ORGate::new(in1.clone(), in2.clone(), out.clone());

    in1.set(OFF);
    in2.set(OFF);
    gate.run();

    assert_eq!(out.state(), OFF);
}

#[test]
fn xorgate_input_on_on() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let out = Wire::default();
    let gate = XORGate::new(in1.clone(), in2.clone(), out.clone());

    in1.set(ON);
    in2.set(ON);
    gate.run();

    assert_eq!(out.state(), OFF);
}

#[test]
fn xorgate_input_off_on() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let out = Wire::default();
    let gate = XORGate::new(in1.clone(), in2.clone(), out.clone());

    in1.set(OFF);
    in2.set(ON);
    gate.run();

    assert_eq!(out.state(), ON);
}

#[test]
fn xorgate_input_on_off() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let out = Wire::default();
    let gate = XORGate::new(in1.clone(), in2.clone(), out.clone());

    in1.set(ON);
    in2.set(OFF);
    gate.run();

    assert_eq!(out.state(), ON);
}

#[test]
fn xorgate_input_off_off() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let out = Wire::default();
    let gate = XORGate::new(in1.clone(), in2.clone(), out.clone());

    in1.set(OFF);
    in2.set(OFF);
    gate.run();

    assert_eq!(out.state(), OFF);
}
