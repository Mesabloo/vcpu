use crate::circuit::gate::{MultiANDGate, NOTGate};
use crate::circuit::wire::Wire;

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

pub struct Decoder4X16 {
    nots: Vec<NOTGate>,
    ands: Vec<MultiANDGate>,
}
impl Decoder4X16 {
    pub fn new(ins: Vec<Wire>, outs: Vec<Wire>) -> Self {
        assert_eq!(ins.len(), 4);
        assert_eq!(outs.len(), 16 /* 2usize.pow(4) */);

        let tmps: Vec<_> = repeat_with(Wire::default).take(ins.len()).collect();

        let nots: Vec<_> = ins
            .clone()
            .into_iter()
            .zip(tmps.clone().into_iter())
            .map(|(i, o)| NOTGate::new(i, o))
            .collect();

        // Let's write all the different combinations by hand:
        // outs[0]:  0/0/0/0
        // outs[1]:  0/0/0/1
        // outs[2]:  0/0/1/0
        // outs[3]:  0/0/1/1
        // outs[4]:  0/1/0/0
        // outs[5]:  0/1/0/1
        // outs[6]:  0/1/1/0
        // outs[7]:  0/1/1/1
        // outs[8]:  1/0/0/0
        // outs[9]:  1/0/0/1
        // outs[10]: 1/0/1/0
        // outs[11]: 1/0/1/1
        // outs[12]: 1/1/0/0
        // outs[13]: 1/1/0/1
        // outs[14]: 1/1/1/0
        // outs[15]: 1/1/1/1

        let ands = vec![
            MultiANDGate::new(tmps.clone(), outs[0].clone()),
            MultiANDGate::new(
                vec![
                    tmps[0].clone(),
                    tmps[1].clone(),
                    tmps[2].clone(),
                    ins[3].clone(),
                ],
                outs[1].clone(),
            ),
            MultiANDGate::new(
                vec![
                    tmps[0].clone(),
                    tmps[1].clone(),
                    ins[2].clone(),
                    tmps[3].clone(),
                ],
                outs[2].clone(),
            ),
            MultiANDGate::new(
                vec![
                    tmps[0].clone(),
                    tmps[1].clone(),
                    ins[2].clone(),
                    ins[3].clone(),
                ],
                outs[3].clone(),
            ),
            MultiANDGate::new(
                vec![
                    tmps[0].clone(),
                    ins[1].clone(),
                    tmps[2].clone(),
                    tmps[3].clone(),
                ],
                outs[4].clone(),
            ),
            MultiANDGate::new(
                vec![
                    tmps[0].clone(),
                    ins[1].clone(),
                    tmps[2].clone(),
                    ins[3].clone(),
                ],
                outs[5].clone(),
            ),
            MultiANDGate::new(
                vec![
                    tmps[0].clone(),
                    ins[1].clone(),
                    ins[2].clone(),
                    tmps[3].clone(),
                ],
                outs[6].clone(),
            ),
            MultiANDGate::new(
                vec![
                    tmps[0].clone(),
                    ins[1].clone(),
                    ins[2].clone(),
                    ins[3].clone(),
                ],
                outs[7].clone(),
            ),
            MultiANDGate::new(
                vec![
                    ins[0].clone(),
                    tmps[1].clone(),
                    tmps[2].clone(),
                    tmps[3].clone(),
                ],
                outs[8].clone(),
            ),
            MultiANDGate::new(
                vec![
                    ins[0].clone(),
                    tmps[1].clone(),
                    tmps[2].clone(),
                    ins[3].clone(),
                ],
                outs[9].clone(),
            ),
            MultiANDGate::new(
                vec![
                    ins[0].clone(),
                    tmps[1].clone(),
                    ins[2].clone(),
                    tmps[3].clone(),
                ],
                outs[10].clone(),
            ),
            MultiANDGate::new(
                vec![
                    ins[0].clone(),
                    tmps[1].clone(),
                    ins[2].clone(),
                    ins[3].clone(),
                ],
                outs[11].clone(),
            ),
            MultiANDGate::new(
                vec![
                    ins[0].clone(),
                    ins[1].clone(),
                    tmps[2].clone(),
                    tmps[3].clone(),
                ],
                outs[12].clone(),
            ),
            MultiANDGate::new(
                vec![
                    ins[0].clone(),
                    ins[1].clone(),
                    tmps[2].clone(),
                    ins[3].clone(),
                ],
                outs[13].clone(),
            ),
            MultiANDGate::new(
                vec![
                    ins[0].clone(),
                    ins[1].clone(),
                    ins[2].clone(),
                    tmps[3].clone(),
                ],
                outs[14].clone(),
            ),
            MultiANDGate::new(ins.clone(), outs[15].clone()),
        ];
        // We will try to find a generic way of doing this based on const generics. For now it doesn't really matter.
        // The problem right now is that the order of combinations matters.

        Decoder4X16 { nots, ands }
    }
    pub fn run(&self) {
        self.nots.iter().for_each(|g| g.run());
        self.ands.iter().for_each(|g| g.run());
    }
}
