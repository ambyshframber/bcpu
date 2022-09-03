use std::mem::size_of;
use regval::RegVal;
use consts::*;
use crate::utils::*;

mod consts;
mod regval;
mod tests;

const GPRS_LEN: usize = 4 * size_of::<u32>();
const POINTERS_LEN: usize = 4 * size_of::<u32>() + 1;
const SEGMENTS_LEN: usize = 4;
const SPECIAL_LEN: usize = 4 * size_of::<u32>();

pub struct Registers {
    gprs: [u8; GPRS_LEN],
    pointers: [u8; POINTERS_LEN],
    segments: [u8; SEGMENTS_LEN],
    special: [u8; SPECIAL_LEN],
}
impl Registers {
    pub fn read(&self, regid: u8) -> Option<RegVal> {
        unimplemented!()
    }
    pub fn write(&self, regid: u8, val: u32) {
        unimplemented!()
    }
}

pub struct Processor {
    registers: Registers,
}
impl Processor {
    fn get_flat_pc(&self) -> u32 {
        let pc = self.registers.read(Spec::PC as u8).unwrap().to_u32();
        let co = self.registers.read(Offs::CO as u8).unwrap().to_u32() << 16;
        pc | co
    }
    fn set_flat_pc(&mut self, pc: u32) {
        let (pc, co) = pc.half_split();
        self.registers.write(Spec::PC as u8, pc as u32);
        self.registers.write(Offs::CO as u8, co as u32);
    }
    fn modify_pc<F>(&mut self, f: F)
    where F: Fn(u32) -> u32 {
        let pc = self.get_flat_pc();
        self.set_flat_pc(f(pc))
    }
}
