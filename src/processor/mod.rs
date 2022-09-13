use super::memory::MemoryMap;
use crate::utils::*;
use consts::*;
use regval::RegVal;

mod consts;
mod regval;
#[cfg(test)]
mod tests;

#[derive(Default)]
pub struct Processor {
    xa: u32, xb: u32, xc: u32, xd: u32,
    xsp: u32, xbp: u32, xsi: u32, xdi: u32, xrp: u32, ro: u16,
    co: u16, do_: u16, eo: u16, so: u16,
    xidtp: u32, xidtl: u32, xpc: u32, xflags: u32,
}
impl Processor {
    fn read(&self, regid: u8) -> Result<RegVal> {
        if self.can_access(regid) {
            match regid {
                0..0x10 => {
                    let v = match regid & GPR_MASK {
                        0 => self.xa,
                        4 => self.xb,
                        8 => self.xc,
                        0xc => self.xd,
                        _ => unreachable!(),
                    };
                    Ok(RegVal::from_u32(v, regid & GPR_SEL_MASK)) 
                }
                0x10..0x1a => {
                    let v = match regid & PTR_MASK {
                        0 => self.xsp,
                        2 => self.xbp,
                        4 => self.xsi,
                        6 => self.xdi,
                        8 => self.xrp,
                        _ => unreachable!(),
                    };
                    Ok(RegVal::from_u32(v, regid & PTR_SEL_MASK))
                }
                0x1a => Ok(self.ro.into()),
                0x20 => Ok(self.co.into()),
                0x21 => Ok(self.do_.into()),
                0x22 => Ok(self.eo.into()),
                0x23 => Ok(self.so.into()),
                0x28..0x30 => {
                    let v = match regid & SPEC_MASK {
                        0 => self.xidtp,
                        2 => self.xidtl,
                        4 => self.xpc,
                        6 => self.xflags,
                        _ => unreachable!(),
                    };
                    Ok(RegVal::from_u32(v, regid & SPEC_SEL_MASK))
                }
                
                _ => Err(Exception::InvalidOperation)
            }
        }
        else {
            Err(Exception::IllegalOperation)
        }
    }
    fn read_16(&self, regid: u8) -> Result<u16> {
        self.read(regid)
            .map(|v| v.unwrap_u16().ok_or(Exception::InvalidOperation))
            .flatten()
    }
    fn write(&mut self, regid: u8, val: RegVal) -> Result<()> {
        if self.can_access(regid) {
            let in_val = val.to_u32();
            match regid {
                0..0x10 => {
                    let sel = regid & GPR_SEL_MASK;
                    match regid & GPR_MASK {
                        0 => self.xa = mix_u32(self.xa, in_val, sel),
                        4 => self.xb = mix_u32(self.xb, in_val, sel),
                        8 => self.xc = mix_u32(self.xc, in_val, sel),
                        0xc => self.xd = mix_u32(self.xd, in_val, sel),
                        _ => unreachable!(),
                    }
                    Ok(())
                }
                0x10..0x1a => { // all pointers except rop
                    let sel = regid & PTR_SEL_MASK;
                    match regid & PTR_MASK {
                        0 => self.xsp = mix_u32(self.xsp, in_val, sel),
                        2 => self.xbp = mix_u32(self.xbp, in_val, sel),
                        4 => self.xsi = mix_u32(self.xsi, in_val, sel),
                        6 => self.xdi = mix_u32(self.xdi, in_val, sel),
                        8 => self.xrp = mix_u32(self.xrp, in_val, sel),
                        _ => unreachable!(),
                    };
                    Ok(())
                }
                0x1a => { self.ro = in_val.half_split().0; Ok(()) }
                0x20 => { self.co = in_val.half_split().0; Ok(()) }
                0x21 => { self.do_ = in_val.half_split().0; Ok(()) }
                0x22 => { self.eo = in_val.half_split().0; Ok(()) }
                0x23 => { self.so = in_val.half_split().0; Ok(()) }
                0x28..0x30 => {
                    let sel = regid & SPEC_SEL_MASK;
                    match regid & SPEC_MASK {
                        0 => self.xidtp = mix_u32(self.xidtp, in_val, sel),
                        2 => self.xidtl = mix_u32(self.xidtl, in_val, sel),
                        4 => self.xpc = mix_u32(self.xpc, in_val, sel),
                        6 => self.xflags = mix_u32(self.xflags, in_val, sel),
                        _ => unreachable!(),
                    };
                    Ok(())
                }
    
                _ => todo!()
            }
        }
        else {
            Err(Exception::IllegalOperation)
        }
    }
    fn size(&self, regid: u8) -> Result<RegSize> {
        match regid {
            0..0x10 => {
                let sel = regid & 0b11;
                Ok(match sel {
                    0 => RegSize::Word,
                    1 => RegSize::Dword,
                    _ => RegSize::Byte,
                })
            }
            0x10..0x1a | 0x28..0x30 =>  {
                Ok(match regid & 1 {
                    0 => RegSize::Word,
                    1 => RegSize::Dword,
                    _ => unreachable!(),
                })
            }
            0x1a => Ok(RegSize::Word),
            0x20..0x24 => Ok(RegSize::Word),

            _ => Err(Exception::InvalidOperation)
        }
    }
    fn can_access(&self, regid: u8) -> bool {
        let privilege = (self.xflags & PRIV_MASK) >> 6;
        match regid {
            0x28..0x30 => {
                true // provisional
            }
            _ => true
        }
    }
}
enum RegSize {
    Byte,
    Word,
    Dword,
}

impl Processor {
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
                self.xrp = self.xpc;
                self.ro = self.co;

                
            }
        }
    }

    fn mov(&mut self, src: Operand, dest: Operand, sign_ext: bool) -> Result<()> {
        if dest.is_const() {
            Err(Exception::InvalidOperation)
        }
        else {
            let src_v = src.value(self)?;
            let size = dest.size(self)?;
            let src_final = if sign_ext {
                src_v.sign_extend(size)
            }
            else {
                src_v.zero_extend(size)
            }?;

            if !self.is_testing() {
                dest.write_back(self, src_final)?;
            }

            Ok(())
        }
    }

    fn get_flat_pc(&self) -> u32 {
        address(self.xpc.half_split().0, self.co)
    }
    fn increment_pc(&mut self) {
        self.xpc = self.xpc.wrapping_add(1)
    }
    fn decrement_pc(&mut self) {
        self.xpc = self.xpc.wrapping_sub(1)
    }

    fn is_testing(&self) -> bool {
        (self.xflags & TEST_MASK) != 0
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
    fn value(&self, registers: &Processor) -> Result<RegVal> {
        match self {
            Operand::Const(c) => Ok(*c),
            Operand::Register(r) => registers.read(*r),
        }
    }
    fn write_back(&self, registers: &mut Processor, val: RegVal) -> Result<()> {
        match self {
            Operand::Register(r) => registers.write(*r, val),
            _ => Ok(()),
        }
    }
    fn size(&self, registers: &Processor) -> Result<RegSize> {
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
