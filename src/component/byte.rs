use crate::circuit::wire::Wire;
use crate::component::memory::MemoryGate;
use crate::common::BUS_WIDTH;

pub struct Byte {
    mem: Vec<MemoryGate>,
}
impl Byte {
    pub fn new(is: Vec<Wire>, s: Wire, os: Vec<Wire>) -> Self {
        assert_eq!(is.len(), BUS_WIDTH);
        assert_eq!(os.len(), BUS_WIDTH);

        Byte {
            mem: is
                .into_iter()
                .zip(os.into_iter())
                .map(|(i, o)| MemoryGate::new(i, s.clone(), o))
                .collect(),
        }
    }

    pub fn run(&self) {
        self.mem.iter().for_each(|m| m.run());
    }
}
