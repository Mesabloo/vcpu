use crate::units::bit::Bit;

/// A wire is a basic unit containing only one bit at a time.
pub struct Wire(pub Bit);

impl Wire {
    pub fn set(&mut self, b: Bit) -> () {
        self.0 = b;
    }

    pub fn state(&self) -> Bit {
        self.0
    }
}

