use crate::circuit::gate::{MultiANDGate, NOTGate};
use crate::circuit::wire::Wire;

use std::iter::{repeat, repeat_with};
use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

/*
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
*/

pub struct Decoder<const IN: usize, const OUT: usize> {
    nots: Vec<NOTGate>,      // length = IN
    ands: Vec<MultiANDGate>, // length = OUT * 2
}
impl<const IN: usize, const OUT: usize> Decoder<IN, OUT> {
    pub fn new(ins: Vec<Wire>, outs: Vec<Wire>) -> Self {
        assert_eq!(ins.len(), IN);
        assert_eq!(outs.len(), OUT);
        assert_eq!(2usize.pow(IN as u32), OUT);

        let mut tmps: Vec<Wire> = repeat_with(Wire::default).take(IN).collect();
        let mut ins = ins;

        let nots: Vec<_> = tmps
            .clone()
            .into_iter()
            .zip(ins.clone().into_iter())
            .map(|(o, i)| NOTGate::new(i, o))
            .collect();

        tmps.append(&mut ins);

        let mut permutations = || {
            let n = tmps.iter().fold(0usize, |n, _| n + 1);
            let mut c: Vec<_> = repeat(0usize).take(n).collect();

            yield tmps.clone();

            let mut i = 0usize;
            while i < n {
                if c[i] < i {
                    let k = if i % 2 == 0 { 0 } else { c[i] };
                    tmps.swap(k, i);

                    yield tmps.clone();

                    c[i] += 1;
                    i = 0;
                } else {
                    c[i] = 0;
                    i += 1;
                }
            }
        };

        let mut wires: Vec<Vec<_>> = vec![];
        while let GeneratorState::Yielded(state) = Pin::new(&mut permutations).resume(()) {
            wires.push(state);
        }

        let ands: Vec<_> = wires
            .into_iter()
            .zip(outs.into_iter())
            .map(|(is, out)| MultiANDGate::new(is, out))
            .collect();

        Decoder { nots, ands }
    }

    pub fn run(&self) {
        self.nots.iter().for_each(|g| g.run());
        self.ands.iter().for_each(|g| g.run());
    }
}
