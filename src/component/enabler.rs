use crate::circuit::gate::ANDGate;
use crate::circuit::wire::{Bus, Wire};
#[cfg(test)]
use crate::common::BUS_WIDTH;
#[cfg(test)]
use crate::units::bit::{OFF, ON};

pub struct Enabler {
    gates: Vec<ANDGate>, // must be of a length of "BUS_WIDTH"
}
impl Enabler {
    pub fn new(is: Bus, e: Wire, os: Bus) -> Self {
        Enabler {
            gates: is
                .into_iter()
                .zip(os.into_iter())
                .map(|(i, o)| ANDGate::new(i, e.clone(), o))
                .collect(),
        }
    }

    pub fn run(&self) {
        self.gates.iter().for_each(|g| g.run());
    }
}

#[cfg(test)]
#[test]
fn test_enabler() {
    let b1 = Bus::default();
    let enable = Wire::default();

    b1[2].set(ON);
    b1[3].set(ON);
    b1[4].set(ON);

    let enabler = Enabler::new(b1.clone(), enable.clone(), b1.clone());
    enabler.run();

    assert_eq!(
        b1.iter().map(|w| w.state()).collect::<Vec<_>>(),
        vec![OFF; BUS_WIDTH]
    );

    enable.set(ON);
    b1[2].set(ON);
    b1[3].set(ON);
    b1[4].set(ON);

    enabler.run();

    assert_eq!(
        b1.iter().map(|w| w.state()).collect::<Vec<_>>(),
        vec![OFF, OFF, ON, ON, ON, OFF, OFF, OFF]
    );
}
