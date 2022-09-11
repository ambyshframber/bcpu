use super::memory::MemoryMap;
use crate::utils::*;
use consts::*;
use regval::RegVal;
use std::mem::size_of;

mod consts;
mod regval;
#[cfg(test)]
mod tests;

const POINTERS_LEN: usize = 4 * size_of::<u32>() + 1;
const SEGMENTS_LEN: usize = 4;
const SPECIAL_LEN: usize = 4 * size_of::<u32>();

#[derive(Default)]
pub struct Registers {
    xa: u32, xb: u32, xc: u32, xd: u32,
    xsp: u32, xbp: u32, xsi: u32, xdi: u32, xrp: u32,
    segments: [u8; SEGMENTS_LEN],
    special: [u8; SPECIAL_LEN],
}
impl Registers {
    fn read(&self, regid: u8) -> Result<RegVal> {
        if (0..0x10).contains(&regid) {
            let v = match regid & !0b11 {
                0 => self.xa,
                4 => self.xb,
                8 => self.xc,
                0xc => self.xd,
                _ => unreachable!(),
            };
            Ok(RegVal::from_u32(v, regid & 0b11))
        }
        else if (0x10..0x1a).contains(&regid) { // all pointers except rop
            let v = match regid & !1 {
                0 => self.xsp,
                2 => self.xbp,
                4 => self.xsi,
                6 => self.xdi,
                8 => self.xrp,
                _ => unreachable!(),
            };
            Ok(RegVal::from_u32(v, !regid & 1))
        }
        else {
            todo!()
        }
    }
    fn read_16(&self, regid: u8) -> Result<u16> {
        self.read(regid)
            .map(|v| v.unwrap_u16().ok_or(Exception::InvalidOperation))
            .flatten()
    }
    fn write(&mut self, regid: u8, val: RegVal) -> Result<()> {
        if (0..0x10).contains(&regid) {
            let in_val = val.to_u32();
            let sel = regid & 0b11;
            match regid & !0b11 {
                0 => self.xa = mix_u32(self.xa, in_val, sel),
                4 => self.xb = mix_u32(self.xb, in_val, sel),
                8 => self.xc = mix_u32(self.xc, in_val, sel),
                0xc => self.xd = mix_u32(self.xd, in_val, sel),
                _ => unreachable!(),
            }
            Ok(())
        }
        else {
            todo!()
        }
    }
    fn size(&self, regid: u8) -> Result<RegSize> {
        if (0..0x10).contains(&regid) {
            let sel = regid & 0b11;
            Ok(match sel {
                0 => RegSize::Word,
                1 => RegSize::Dword,
                _ => RegSize::Byte,
            })
        }
        else if (0x10..0x1a).contains(&regid) {
            Ok(match regid & 1 {
                0 => RegSize::Word,
                1 => RegSize::Dword,
                _ => unreachable!(),
            })
        }
        else {
            todo!()
        }
    }
}
enum RegSize {
    Byte,
    Word,
    Dword,
}

pub struct Processor {
    registers: Registers,
}
impl Processor {
    pub fn new() -> Processor {
        Processor {
            registers: Registers::default()
        }
    }
    fn clock(&mut self, mem: &mut MemoryMap) {
        let instruction = self.get_instruction_byte(mem);
        let mut operands = Vec::new();

        let ops_res = loop {
            if let Some(operand) = self.read_operand(mem) {
                match operand {
                    Ok(o) => operands.push(o),
                    Err(e) => break Err(e),
                }
            }
            else {
                break Ok(());
            }
        };

        match ops_res {
            Ok(_) => { // proceed to instruction decode
                
            }
            Err(e) => { // interrupt processor here
                
            }
        }
    }

    fn mov(&mut self, src: Operand, dest: Operand, sign_ext: bool) -> Result<()> {
        if dest.is_const() {
            Err(Exception::InvalidOperation)
        }
        else {
            let src_v = src.value(&self.registers)?;
            let size = dest.size(&self.registers)?;
            let src_final = if sign_ext {
                src_v.sign_extend(size)
            }
            else {
                src_v.zero_extend(size)
            }?;

            if !self.is_testing() {
                dest.write_back(&mut self.registers, src_final)?;
            }

            Ok(())
        }
    }

    fn get_flat_pc(&self) -> u32 {
        let pc = self.registers.read(Spec::PC as u8).unwrap().to_u32();
        let co = self.registers.read(Offs::CO as u8).unwrap().to_u32() << 8;
        pc + co
    }
    fn increment_pc(&mut self) {
        let pc = self
            .registers
            .read(Spec::PC as u8)
            .unwrap()
            .unwrap_u16()
            .unwrap()
            .wrapping_add(1);
        self.registers.write(Spec::PC as u8, RegVal::Word(pc));
    }
    fn decrement_pc(&mut self) {
        let pc = self
            .registers
            .read(Spec::PC as u8)
            .unwrap()
            .unwrap_u16()
            .unwrap()
            .wrapping_sub(1);
        self.registers.write(Spec::PC as u8, RegVal::Word(pc));
    }

    fn is_testing(&self) -> bool {
        false // FIX THIS ASSHOLE
    }

    fn get_instruction_byte(&mut self, mem: &mut MemoryMap) -> u8 {
        let b = mem.read(self.get_flat_pc());
        self.increment_pc();
        b
    }

    fn read_operand(&mut self, mem: &mut MemoryMap) -> Option<Result<Operand>> {
        let operand = self.get_instruction_byte(mem);
        if operand & 0x80 != 0 {
            self.decrement_pc(); // decrement pc again
            return None;
        }
        Some(match operand {
            0x70 => {
                let b = self.get_instruction_byte(mem);
                Ok(Operand::Const(RegVal::Byte(b)))
            }
            0x71 => {
                let lo = self.get_instruction_byte(mem);
                let hi = self.get_instruction_byte(mem);
                let v = u16::from_le_bytes([lo, hi]);
                Ok(Operand::Const(RegVal::Word(v)))
            }
            _ => Ok(Operand::Register(operand)),
        })
    }
}

enum Operand {
    Const(RegVal),
    Register(u8),
}
impl Operand {
    fn is_const(&self) -> bool {
        matches!(self, Operand::Const(_))
    }
    fn value(&self, registers: &Registers) -> Result<RegVal> {
        match self {
            Operand::Const(c) => Ok(*c),
            Operand::Register(r) => registers.read(*r),
        }
    }
    fn write_back(&self, registers: &mut Registers, val: RegVal) -> Result<()> {
        match self {
            Operand::Register(r) => registers.write(*r, val),
            _ => Ok(()),
        }
    }
    fn size(&self, registers: &Registers) -> Result<RegSize> {
        match self {
            Self::Register(r) => registers.size(*r),
            Self::Const(c) => todo!(),
        }
    }
}

fn address(addr: u16, offset: u16) -> u32 {
    (addr as u32) + ((offset as u32) << 8)
}

type Result<T> = std::result::Result<T, Exception>;
#[derive(Debug, PartialEq)]
enum Exception {
    InvalidOperation = 0,
    IllegalOperation = 1,
}
