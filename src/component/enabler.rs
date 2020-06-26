use crate::circuit::gate::ANDGate;
use crate::circuit::wire::Wire;
use crate::common::BUS_WIDTH;

pub struct Enabler {
    gates: Vec<ANDGate>, // must be of a length of "BUS_WIDTH"
}
impl Enabler {
    pub fn new(is: Vec<Wire>, e: Wire, os: Vec<Wire>) -> Self {
        assert_eq!(is.len(), BUS_WIDTH);
        assert_eq!(os.len(), BUS_WIDTH);

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
