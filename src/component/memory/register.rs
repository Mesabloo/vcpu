use crate::component::memory::byte::MemoryByte;
use crate::component::enabler::Enabler;
use crate::common::BUS_WIDTH;
use crate::circuit::wire::Wire;

pub struct Register {
    mem: MemoryByte,
    en: Enabler,
}
impl Register {
    pub fn new(is: Vec<Wire>, s: Wire, e: Wire, os: Vec<Wire>) -> Self {
        assert_eq!(is.len(), BUS_WIDTH);
        assert_eq!(os.len(), BUS_WIDTH);

        let tmp = vec![Wire::default(); BUS_WIDTH];

        Register{
            mem: MemoryByte::new(is, s, tmp.clone()),
            en: Enabler::new(tmp, e, os),
        }
    }

    pub fn run(&self) {
        self.mem.run();
        self.en.run();
    }
}