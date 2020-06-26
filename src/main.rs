use vcpu::circuit::wire::{Wire, Bus};
use vcpu::common::BUS_WIDTH;
use vcpu::component::memory::register::Register;
use vcpu::units::bit::{Bit, OFF, ON};

fn main() {
    let wires_in = Bus::new();
    let wires_out = Bus::new();
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
