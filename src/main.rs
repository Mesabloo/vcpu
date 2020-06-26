use vcpu::circuit::wire::Wire;
use vcpu::component::memory::bit::MemoryBit;
use vcpu::units::bit::{OFF, ON};

fn main() {
    let in1 = Wire::default();
    let in2 = Wire::default();
    let out = Wire::default();
    in2.set(ON);
    in1.set(ON);
    // in1 is OFF

    let mem = MemoryBit::new(in1.clone(), in2.clone(), out.clone());
    mem.run();

    assert_eq!(ON, out.state());

    in2.set(OFF);
    in1.set(OFF);

    mem.run();

    assert_eq!(ON, out.state());

    println!("Done!");
}
