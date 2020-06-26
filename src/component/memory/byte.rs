use crate::circuit::wire::Wire;
use crate::common::BUS_WIDTH;
use crate::component::memory::bit::MemoryBit;

pub struct MemoryByte {
    mem: Vec<MemoryBit>,
}
impl MemoryByte {
    pub fn new(is: Vec<Wire>, s: Wire, os: Vec<Wire>) -> Self {
        assert_eq!(is.len(), BUS_WIDTH);
        assert_eq!(os.len(), BUS_WIDTH);

        MemoryByte {
            mem: is
                .into_iter()
                .zip(os.into_iter())
                .map(|(i, o)| MemoryBit::new(i, s.clone(), o))
                .collect(),
        }
    }

    pub fn run(&self) {
        self.mem.iter().for_each(|m| m.run());
    }
}
