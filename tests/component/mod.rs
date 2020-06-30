#![cfg(test)]

use vcpu::circuit::wire::*;
use vcpu::component::{decoder::*, enabler::*, shifter::*};
use vcpu::units::bit::{OFF, ON};

use std::iter::repeat_with;

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

#[test]
fn decoder2x4_off_off() {
    let ins = repeat_with(Wire::default).take(2).collect::<Vec<_>>();
    let outs = repeat_with(Wire::default).take(4).collect::<Vec<_>>();
    let decoder = Decoder2X4::new(
        ins[0].clone(),
        ins[1].clone(),
        outs[0].clone(),
        outs[1].clone(),
        outs[2].clone(),
        outs[3].clone(),
    );

    ins[0].set(OFF);
    ins[1].set(OFF);
    decoder.run();

    assert_eq!(outs[0].state(), ON);
}

#[test]
fn decoder2x4_on_off() {
    let ins = repeat_with(Wire::default).take(2).collect::<Vec<_>>();
    let outs = repeat_with(Wire::default).take(4).collect::<Vec<_>>();
    let decoder = Decoder2X4::new(
        ins[0].clone(),
        ins[1].clone(),
        outs[0].clone(),
        outs[1].clone(),
        outs[2].clone(),
        outs[3].clone(),
    );

    ins[0].set(ON);
    ins[1].set(OFF);
    decoder.run();

    assert_eq!(outs[2].state(), ON);
}

#[test]
fn decoder2x4_off_on() {
    let ins = repeat_with(Wire::default).take(2).collect::<Vec<_>>();
    let outs = repeat_with(Wire::default).take(4).collect::<Vec<_>>();
    let decoder = Decoder2X4::new(
        ins[0].clone(),
        ins[1].clone(),
        outs[0].clone(),
        outs[1].clone(),
        outs[2].clone(),
        outs[3].clone(),
    );

    ins[0].set(OFF);
    ins[1].set(ON);
    decoder.run();

    assert_eq!(outs[1].state(), ON);
}

#[test]
fn decoder2x4_on_on() {
    let ins = repeat_with(Wire::default).take(2).collect::<Vec<_>>();
    let outs = repeat_with(Wire::default).take(4).collect::<Vec<_>>();
    let decoder = Decoder2X4::new(
        ins[0].clone(),
        ins[1].clone(),
        outs[0].clone(),
        outs[1].clone(),
        outs[2].clone(),
        outs[3].clone(),
    );

    ins[0].set(ON);
    ins[1].set(ON);
    decoder.run();

    assert_eq!(outs[3].state(), ON);
}

pub mod memory;
