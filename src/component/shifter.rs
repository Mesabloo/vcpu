use crate::circuit::wire::{Bus, Wire};
use crate::common::BUS_WIDTH;
#[cfg(test)]
use crate::units::bit::{OFF, ON};

pub struct LeftShifter {
    bus1: Bus,
    bus2: Bus,
    shift_in: Wire,
    shift_out: Wire,
}
impl LeftShifter {
    pub fn new(i: Bus, o: Bus, shift_in: Wire, shift_out: Wire) -> Self {
        LeftShifter {
            bus1: i,
            bus2: o,
            shift_in,
            shift_out,
        }
    }

    pub fn run(&self) {
        self.shift_out.set(self.bus1[0].state());
        for idx in 1..BUS_WIDTH {
            self.bus2[idx - 1].set(self.bus1[idx].state());
        }
        self.bus2[BUS_WIDTH - 1].set(self.shift_in.state());
    }
}

#[cfg(test)]
#[test]
fn test_left_shifter() {
    let b1 = Bus::default();
    let s_in = Wire::default();
    let s_out = Wire::default();

    b1[1].set(ON);
    b1[6].set(ON);

    let shift = LeftShifter::new(b1.clone(), b1.clone(), s_in.clone(), s_out.clone());
    shift.run();

    assert_eq!(s_in.state(), OFF);
    assert_eq!(s_out.state(), OFF);
    let b1_wires: Vec<Wire> = b1.clone().into();
    assert_eq!(
        b1_wires.iter().map(|w| w.state()).collect::<Vec<_>>(),
        vec![ON, OFF, OFF, OFF, OFF, ON, OFF, OFF]
    );

    shift.run();

    assert_eq!(s_in.state(), OFF);
    assert_eq!(s_out.state(), ON);
    let b1_wires: Vec<Wire> = b1.clone().into();
    assert_eq!(
        b1_wires.iter().map(|w| w.state()).collect::<Vec<_>>(),
        vec![OFF, OFF, OFF, OFF, ON, OFF, OFF, OFF]
    );
}

pub struct RightShifter {
    bus1: Bus,
    bus2: Bus,
    shift_in: Wire,
    shift_out: Wire,
}
impl RightShifter {
    pub fn new(i: Bus, o: Bus, shift_in: Wire, shift_out: Wire) -> Self {
        RightShifter {
            bus1: i,
            bus2: o,
            shift_in,
            shift_out,
        }
    }

    pub fn run(&self) {
        self.shift_out.set(self.bus1[BUS_WIDTH - 1].state());
        for idx in (1..BUS_WIDTH).rev() {
            self.bus2[idx].set(self.bus1[idx - 1].state());
        }
        self.bus2[0].set(self.shift_in.state());
    }
}

#[cfg(test)]
#[test]
fn test_right_shifter() {
    let b1 = Bus::default();
    let s_in = Wire::default();
    let s_out = Wire::default();

    b1[2].set(ON);
    b1[6].set(ON);

    let shift = RightShifter::new(b1.clone(), b1.clone(), s_in.clone(), s_out.clone());
    shift.run();

    assert_eq!(s_in.state(), OFF);
    assert_eq!(s_out.state(), OFF);
    let b1_wires: Vec<Wire> = b1.clone().into();
    assert_eq!(
        b1_wires.iter().map(|w| w.state()).collect::<Vec<_>>(),
        vec![OFF, OFF, OFF, ON, OFF, OFF, OFF, ON]
    );

    shift.run();
    assert_eq!(s_in.state(), OFF);
    assert_eq!(s_out.state(), ON);
    let b1_wires: Vec<Wire> = b1.clone().into();
    assert_eq!(
        b1_wires.iter().map(|w| w.state()).collect::<Vec<_>>(),
        vec![OFF, OFF, OFF, OFF, ON, OFF, OFF, OFF]
    );
}
