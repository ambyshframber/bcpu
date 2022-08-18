const TOTAL_REGISTER_LENGTH: usize = 32;
const A: u8 = 0;
const AL: u8 = 0x10;
const AH: u8 = 0x11;

fn get_register_offset(regid: u8) -> Option<usize> {
    if regid & 0b1000_0000 == 0 && regid & 0b0110_0000 != 0 {
        unimplemented!("special purpose registers are not yet implemented")
    }
    else {
        let regid_new = regid & 0xf;
        if regid & 0b1_0000 == 0 { // 16 bit
            Some(regid_new << 1)
        }
        else { // 8 bit
            Some(regid_new)
        }
    }.map(|x| x as usize)
}

pub struct Processor {
    registers: [u8; TOTAL_REGISTER_LENGTH], // all data in addressable registers. multibyte registers are stored little endian
}
