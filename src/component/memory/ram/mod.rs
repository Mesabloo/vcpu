pub mod cell;

use crate::circuit::wire::{Bus, Wire};
use crate::common::BUS_WIDTH;
use crate::component::decoder::Decoder;
use crate::component::memory::register::Register;
use cell::Cell;

use std::iter::repeat_with;

const DECODER_IN_NB: usize = BUS_WIDTH / 2;
const DECODER_OUT_NB: usize = 2usize.pow(DECODER_IN_NB as u32);

pub struct RAM {
    mar: Register,
    dh: Decoder<DECODER_IN_NB, DECODER_OUT_NB>,
    dv: Decoder<DECODER_IN_NB, DECODER_OUT_NB>,
    cells: Vec<Cell>,
}
impl RAM {
    pub fn new(bus1: Bus, bus2: Bus, sa: Wire, s: Wire, e: Wire) -> Self {
        let mut reg_to_dec_s: Vec<_> = repeat_with(Wire::default).take(BUS_WIDTH).collect();
        let r = Register::new(bus1, sa.clone(), sa, Bus::from(reg_to_dec_s.clone()));

        let mut dec_to_bus_s: Vec<_> = repeat_with(Wire::default)
            .take(2 * DECODER_OUT_NB)
            .collect();

        let reg_to_dh = reg_to_dec_s.split_off(DECODER_IN_NB);
        let reg_to_dv = reg_to_dec_s;

        let dh_to_bus = dec_to_bus_s.split_off(DECODER_OUT_NB);
        let dv_to_bus = dec_to_bus_s;

        let dh = Decoder::new(reg_to_dh, dh_to_bus.clone());
        let dv = Decoder::new(reg_to_dv, dv_to_bus.clone());

        let cells = (0..2usize.pow(DECODER_OUT_NB as u32))
            .map(|i| {
                Cell::new(
                    dv_to_bus[i % DECODER_OUT_NB].clone(),
                    dh_to_bus[i / DECODER_OUT_NB].clone(),
                    s.clone(),
                    e.clone(),
                    bus2.clone(),
                )
            })
            .collect();

        RAM {
            mar: r,
            dh,
            dv,
            cells,
        }
    }

    pub fn run(&self) {
        self.mar.run();
        self.dh.run();
        self.dv.run();
        self.cells.iter().for_each(|c| c.run());
    }
}
