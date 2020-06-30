use crate::circuit::gate::{MultiANDGate, NOTGate};
use crate::circuit::wire::Wire;

use itertools::Itertools;
use std::iter::repeat_with;

pub struct Decoder2X4 {
    not_a: NOTGate,
    not_b: NOTGate,
    not_a_not_b: MultiANDGate,
    not_a_b: MultiANDGate,
    a_not_b: MultiANDGate,
    a_b: MultiANDGate,
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
            not_a_not_b: MultiANDGate::new(vec![tmp_na.clone(), tmp_nb.clone()], out_na_nb),
            not_a_b: MultiANDGate::new(vec![tmp_na, in_b.clone()], out_na_b),
            a_not_b: MultiANDGate::new(vec![in_a.clone(), tmp_nb], out_a_nb),
            a_b: MultiANDGate::new(vec![in_a, in_b], out_a_b),
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
