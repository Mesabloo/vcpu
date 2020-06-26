use crate::circuit::gate::{ANDGate, NOTGate};
use crate::circuit::wire::Wire;

pub struct Decoder2X4 {
    not_a: NOTGate,
    not_b: NOTGate,
    not_a_not_b: ANDGate,
    not_a_b: ANDGate,
    a_not_b: ANDGate,
    a_b: ANDGate,
}
impl Decoder2X4 {
    pub fn new(
        in_a: Wire,
        in_b: Wire,
        out_na_nb: Wire,
        out_na_b: Wire,
        out_a_nb: Wire,
        out_a_b: Wire,
    ) -> Self {
        let tmp_na = Wire::default();
        let tmp_nb = Wire::default();

        Decoder2X4 {
            not_a: NOTGate::new(in_a.clone(), tmp_na.clone()),
            not_b: NOTGate::new(in_b.clone(), tmp_nb.clone()),
            not_a_not_b: ANDGate::new(tmp_na.clone(), tmp_nb.clone(), out_na_nb),
            not_a_b: ANDGate::new(tmp_na, in_b.clone(), out_na_b),
            a_not_b: ANDGate::new(in_a.clone(), tmp_nb, out_a_nb),
            a_b: ANDGate::new(in_a, in_b, out_a_b),
        }
    }

    pub fn run(&self) {
        self.not_a.run();
        self.not_b.run();

        self.not_a_not_b.run();
        self.not_a_b.run();
        self.a_not_b.run();
        self.a_b.run();
    }
}