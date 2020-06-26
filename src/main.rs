use vcpu::circuit::wire::Wire;
use vcpu::common::BUS_WIDTH;
use vcpu::component::memory::register::Register;
use vcpu::units::bit::{Bit, OFF, ON};

use std::iter::repeat_with;

fn main() {
    let wires_in: Vec<Wire> = repeat_with(|| Wire::default()).take(BUS_WIDTH).collect();
    let wires_out: Vec<Wire> = repeat_with(|| Wire::default()).take(BUS_WIDTH).collect();
    let set = Wire::default();
    let enable = Wire::default();

    wires_in[0].set(ON);
    set.set(ON);

    let r = Register::new(
        wires_in.clone(),
        set.clone(),
        enable.clone(),
        wires_out.clone(),
    );
    r.run();

    assert_eq!(
        wires_out.iter().map(|w| w.state()).collect::<Vec<Bit>>(),
        vec![OFF; BUS_WIDTH]
    );

    wires_in[0].set(OFF);
    set.set(OFF);
    enable.set(ON);

    r.run();

    assert_eq!(
        wires_out.iter().map(|w| w.state()).collect::<Vec<Bit>>(),
        vec![ON, OFF, OFF, OFF, OFF, OFF, OFF, OFF]
    );

    println!("Done!");
}
