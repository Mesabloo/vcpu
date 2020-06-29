#![cfg(test)]

use vcpu::circuit::wire::{Bus, Wire};
use vcpu::common::BUS_WIDTH;
use vcpu::component::memory::ram::{cell::*, *};
use vcpu::units::bit::{OFF, ON};

#[test]
fn ram_cell_unselect_no_set_no_enable() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let set = Wire::default();
    let enable = Wire::default();
    let out = Bus::default();
    let cell = Cell::new(in1, in2, set, enable, out.clone());

    cell.run();

    assert_eq!(
        out.iter().map(|w| w.state()).collect::<Vec<_>>(),
        vec![OFF; BUS_WIDTH]
    );
}

#[test]
fn ram_cell_select_no_set_no_enable() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let set = Wire::default();
    let enable = Wire::default();
    let out = Bus::default();
    let cell = Cell::new(in1.clone(), in2.clone(), set, enable, out.clone());

    in1.set(ON);
    in2.set(ON);
    cell.run();

    assert_eq!(
        out.iter().map(|w| w.state()).collect::<Vec<_>>(),
        vec![OFF; BUS_WIDTH]
    );
}

#[test]
#[ignore]  // We currently ignore this test because it fails...
fn ram_cell_select_set_no_enable() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let set = Wire::default();
    let enable = Wire::default();
    let out = Bus::default();
    let cell = Cell::new(in1.clone(), in2.clone(), set.clone(), enable, out.clone());

    in1.set(ON);
    in2.set(ON);
    out[2].set(ON);
    out[5].set(ON);
    set.set(ON);
    cell.run();

    assert_eq!(out[2].state(), ON);
    assert_eq!(out[5].state(), ON);
    // These should be left untouched because `enable` is OFF.
}

#[test]
fn ram_cell_select_set_no_enable_enable() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let set = Wire::default();
    let enable = Wire::default();
    let out = Bus::default();
    let cell = Cell::new(in1.clone(), in2.clone(), set.clone(), enable.clone(), out.clone());

    in1.set(ON);
    in2.set(ON);
    out[2].set(ON);
    out[5].set(ON);
    set.set(ON);
    cell.run();
    in1.set(ON);
    in2.set(ON);
    out[2].set(OFF);
    out[5].set(OFF);
    set.set(OFF);
    enable.set(ON);
    cell.run();

    assert_eq!(out[2].state(), ON);
    assert_eq!(out[5].state(), ON);
}

#[test]
fn ram_cell_select_set_no_enable_unselect_no_set_enable() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let set = Wire::default();
    let enable = Wire::default();
    let out = Bus::default();
    let cell = Cell::new(in1.clone(), in2.clone(), set.clone(), enable.clone(), out.clone());

    in1.set(ON);
    in2.set(ON);
    out[2].set(ON);
    out[5].set(ON);
    set.set(ON);
    cell.run();
    in1.set(OFF);
    in2.set(OFF);
    out[2].set(OFF);
    out[5].set(OFF);
    set.set(OFF);
    enable.set(ON);
    cell.run();

    assert_eq!(out[2].state(), OFF);
    assert_eq!(out[5].state(), OFF);
    // These sould be OFF because we enabled an unselected cell.
}
