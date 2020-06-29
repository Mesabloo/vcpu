#![cfg(test)]

use vcpu::circuit::wire::{Bus, Wire};
use vcpu::component::memory::{bit::*, byte::*, register::*};
use vcpu::units::bit::{OFF, ON};

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

#[test]
fn register_no_set_no_enable() {
    let b1_in = Bus::default();
    let b1_out = Bus::default();
    let set1 = Wire::default();
    let enable1 = Wire::default();
    let r_in = Register::new(b1_in.clone(), set1.clone(), enable1.clone(), b1_out.clone());

    b1_in[1].set(ON);
    b1_in[3].set(ON);
    b1_in[6].set(ON);
    r_in.run();

    assert_eq!(b1_out[1].state(), OFF);
    assert_eq!(b1_out[3].state(), OFF);
    assert_eq!(b1_out[6].state(), OFF);
}

#[test]
fn register_set_no_enable() {
    let b1_in = Bus::default();
    let b1_out = Bus::default();
    let set1 = Wire::default();
    let enable1 = Wire::default();
    let r_in = Register::new(b1_in.clone(), set1.clone(), enable1.clone(), b1_out.clone());

    b1_in[1].set(ON);
    b1_in[3].set(ON);
    b1_in[6].set(ON);
    set1.set(ON);
    r_in.run();

    assert_eq!(b1_out[1].state(), OFF);
    assert_eq!(b1_out[3].state(), OFF);
    assert_eq!(b1_out[6].state(), OFF);
}

#[test]
fn register_set_enable() {
    let b1_in = Bus::default();
    let b1_out = Bus::default();
    let set1 = Wire::default();
    let enable1 = Wire::default();
    let r_in = Register::new(b1_in.clone(), set1.clone(), enable1.clone(), b1_out.clone());

    b1_in[1].set(ON);
    b1_in[3].set(ON);
    b1_in[6].set(ON);
    set1.set(ON);
    enable1.set(ON);
    r_in.run();

    assert_eq!(b1_out[1].state(), ON);
    assert_eq!(b1_out[3].state(), ON);
    assert_eq!(b1_out[6].state(), ON);
}

#[test]
fn register_set_no_enable_enable() {
    let b1_in = Bus::default();
    let b1_out = Bus::default();
    let set1 = Wire::default();
    let enable1 = Wire::default();
    let r_in = Register::new(b1_in.clone(), set1.clone(), enable1.clone(), b1_out.clone());

    b1_in[1].set(ON);
    b1_in[3].set(ON);
    b1_in[6].set(ON);
    set1.set(ON);
    r_in.run();
    b1_in[1].set(OFF);
    b1_in[3].set(OFF);
    b1_in[6].set(OFF);
    set1.set(OFF);
    enable1.set(ON);
    r_in.run();

    assert_eq!(b1_out[1].state(), ON);
    assert_eq!(b1_out[3].state(), ON);
    assert_eq!(b1_out[6].state(), ON);
}

#[test]
fn register_no_set_enable() {
    let b1_in = Bus::default();
    let b1_out = Bus::default();
    let set1 = Wire::default();
    let enable1 = Wire::default();
    let r_in = Register::new(b1_in.clone(), set1.clone(), enable1.clone(), b1_out.clone());

    b1_in[1].set(ON);
    b1_in[3].set(ON);
    b1_in[6].set(ON);
    enable1.set(ON);
    r_in.run();

    assert_eq!(b1_out[1].state(), OFF);
    assert_eq!(b1_out[3].state(), OFF);
    assert_eq!(b1_out[6].state(), OFF);
}
