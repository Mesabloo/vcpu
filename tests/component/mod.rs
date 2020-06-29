#![cfg(test)]

use vcpu::circuit::wire::*;
use vcpu::component::{enabler::*, shifter::*};
use vcpu::units::bit::{OFF, ON};

#[test]
fn left_shifter_works() {
    let bus_in = Bus::default();
    let bus_out = Bus::default();
    let shift_in = Wire::default();
    let shift_out = Wire::default();
    let shifter = LeftShifter::new(bus_in.clone(), bus_out.clone(), shift_in, shift_out.clone());

    bus_in[1].set(ON);
    bus_in[7].set(ON);
    shifter.run();

    assert_eq!(bus_in[1].state(), ON);
    assert_eq!(bus_in[7].state(), ON);
    assert_eq!(bus_out[0].state(), ON);
    assert_eq!(bus_out[6].state(), ON);
    assert_eq!(shift_out.state(), OFF);
}

#[test]
fn left_shifter_carry() {
    let bus_in = Bus::default();
    let bus_out = Bus::default();
    let shift_in = Wire::default();
    let shift_out = Wire::default();
    let shifter = LeftShifter::new(bus_in.clone(), bus_out.clone(), shift_in, shift_out.clone());

    bus_in[0].set(ON);
    shifter.run();

    assert_eq!(bus_out[0].state(), OFF);
    assert_eq!(shift_out.state(), ON);
}

#[test]
fn left_shifter_in() {
    let bus_in = Bus::default();
    let bus_out = Bus::default();
    let shift_in = Wire::default();
    let shifter = LeftShifter::new(
        bus_in.clone(),
        bus_out.clone(),
        shift_in.clone(),
        shift_in.clone(),
    );

    bus_in[0].set(ON);
    shifter.run();

    assert_eq!(bus_out[0].state(), OFF);
    assert_eq!(bus_out[7].state(), ON);
    assert_eq!(shift_in.state(), ON);
}

#[test]
fn left_shifter_unibus() {
    let bus = Bus::default();
    let shift_in = Wire::default();
    let shift_out = Wire::default();
    let shifter = LeftShifter::new(bus.clone(), bus.clone(), shift_in, shift_out);

    bus[2].set(ON);
    shifter.run();

    assert_eq!(bus[1].state(), ON);
}

#[test]
fn right_shifter_works() {
    let bus_in = Bus::default();
    let bus_out = Bus::default();
    let shift_in = Wire::default();
    let shift_out = Wire::default();
    let shifter = RightShifter::new(bus_in.clone(), bus_out.clone(), shift_in, shift_out.clone());

    bus_in[1].set(ON);
    bus_in[6].set(ON);
    shifter.run();

    assert_eq!(bus_in[1].state(), ON);
    assert_eq!(bus_in[6].state(), ON);
    assert_eq!(bus_out[2].state(), ON);
    assert_eq!(bus_out[7].state(), ON);
    assert_eq!(shift_out.state(), OFF);
}

#[test]
fn right_shifter_carry() {
    let bus_in = Bus::default();
    let bus_out = Bus::default();
    let shift_in = Wire::default();
    let shift_out = Wire::default();
    let shifter = RightShifter::new(bus_in.clone(), bus_out.clone(), shift_in, shift_out.clone());

    bus_in[7].set(ON);
    shifter.run();

    assert_eq!(bus_out[7].state(), OFF);
    assert_eq!(shift_out.state(), ON);
}

#[test]
fn right_shifter_in() {
    let bus_in = Bus::default();
    let bus_out = Bus::default();
    let shift_in = Wire::default();
    let shifter = RightShifter::new(
        bus_in.clone(),
        bus_out.clone(),
        shift_in.clone(),
        shift_in.clone(),
    );

    bus_in[7].set(ON);
    shifter.run();

    assert_eq!(bus_out[0].state(), ON);
    assert_eq!(bus_out[7].state(), OFF);
    assert_eq!(shift_in.state(), ON);
}

#[test]
fn right_shifter_unibus() {
    let bus = Bus::default();
    let shift_in = Wire::default();
    let shift_out = Wire::default();
    let shifter = RightShifter::new(bus.clone(), bus.clone(), shift_in, shift_out);

    bus[2].set(ON);
    shifter.run();

    assert_eq!(bus[3].state(), ON);
}

#[test]
fn enabler_no_enable() {
    let bus_in = Bus::default();
    let bus_out = Bus::default();
    let enable = Wire::default();
    let enabler = Enabler::new(bus_in.clone(), enable, bus_out.clone());

    bus_in[2].set(ON);
    bus_in[7].set(ON);
    enabler.run();

    assert_eq!(bus_out[2].state(), OFF);
    assert_eq!(bus_out[7].state(), OFF);
    assert_eq!(bus_in[2].state(), ON);
    assert_eq!(bus_in[7].state(), ON);
}

#[test]
fn enabler_enabled() {
    let bus_in = Bus::default();
    let bus_out = Bus::default();
    let enable = Wire::default();
    let enabler = Enabler::new(bus_in.clone(), enable.clone(), bus_out.clone());

    bus_in[2].set(ON);
    bus_in[7].set(ON);
    enable.set(ON);
    enabler.run();

    assert_eq!(bus_out[2].state(), ON);
    assert_eq!(bus_out[7].state(), ON);
    assert_eq!(bus_in[2].state(), ON);
    assert_eq!(bus_in[7].state(), ON);
}

pub mod memory;
