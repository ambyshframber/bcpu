use std::mem::size_of;
use regval::RegVal;

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
    
}
