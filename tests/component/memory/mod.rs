#![cfg(test)]

use vcpu::circuit::wire::{Bus, Wire};
use vcpu::units::bit::{OFF, ON};
use vcpu::component::memory::{bit::*, byte::*};

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

#[test]
fn memory_byte_no_set() {
    let b_in = Bus::default();
    let b_out = Bus::default();
    let set = Wire::default();
    let mem_byte = MemoryByte::new(b_in.clone(), set.clone(), b_out.clone());

    b_in[2].set(ON);
    b_in[5].set(ON);
    mem_byte.run();

    assert_eq!(b_out[2].state(), OFF);
    assert_eq!(b_out[5].state(), OFF);
}

#[test]
fn memory_byte_set() {
    let b_in = Bus::default();
    let b_out = Bus::default();
    let set = Wire::default();
    let mem_byte = MemoryByte::new(b_in.clone(), set.clone(), b_out.clone());

    b_in[2].set(ON);
    b_in[5].set(ON);
    set.set(ON);
    mem_byte.run();

    assert_eq!(b_out[2].state(), ON);
    assert_eq!(b_out[5].state(), ON);
}

#[test]
fn memory_byte_set_unset() {
    let b_in = Bus::default();
    let b_out = Bus::default();
    let set = Wire::default();
    let mem_byte = MemoryByte::new(b_in.clone(), set.clone(), b_out.clone());

    b_in[2].set(ON);
    b_in[5].set(ON);
    set.set(ON);
    mem_byte.run();
    b_in[2].set(OFF);
    b_in[5].set(OFF);
    set.set(OFF);
    mem_byte.run();

    assert_eq!(b_out[2].state(), ON);
    assert_eq!(b_out[5].state(), ON);
}