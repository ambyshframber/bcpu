mod registers;
#[cfg(test)]
mod tests;

const TOTAL_REGISTER_LENGTH: usize = 32;

const fn get_register_offset(regid: u8) -> Option<(usize, bool)> {
    if regid < 0x10 { // GPR
        if regid == 0x03 {
            return Some((1, true))
        }
        if regid == 0x07 {
            return Some((5, true))
        }
        else {
            return Some(((regid & !0b11) as usize, regid & 0b11 != 0))
        }
    }
    if regid < 0x20 { // pointer
        
    }
    None
}

pub struct Processor {
    registers: [u8; TOTAL_REGISTER_LENGTH], // all data in addressable registers. multibyte registers are stored little endian
}
