#![cfg(test)]

use vcpu::circuit::wire::{Bus, Wire};
use vcpu::units::bit::{OFF, ON};
use vcpu::component::memory::{bit::*};

#[test]
fn memory_bit_no_set() {
    let in1 = Wire::default();
    let set = Wire::default();
    let out = Wire::default();
    let mem_bit = MemoryBit::new(in1.clone(), set, out.clone());

    in1.set(ON);
    mem_bit.run();

    assert_eq!(out.state(), OFF);
}

#[test]
fn memory_bit_set() {
    let in1 = Wire::default();
    let set = Wire::default();
    let out = Wire::default();
    let mem_bit = MemoryBit::new(in1.clone(), set.clone(), out.clone());

    in1.set(ON);
    set.set(ON);
    mem_bit.run();

    assert_eq!(out.state(), ON);
}

#[test]
fn memory_bit_set_unset() {
    let in1 = Wire::default();
    let set = Wire::default();
    let out = Wire::default();
    let mem_bit = MemoryBit::new(in1.clone(), set.clone(), out.clone());

    in1.set(ON);
    set.set(ON);
    mem_bit.run();
    in1.set(OFF);
    set.set(OFF);
    mem_bit.run();

    assert_eq!(out.state(), ON);
}