const TOTAL_REGISTER_LENGTH: usize = 32;

fn get_register_offset(regid: u8) -> Option<usize> {
    None
}

pub struct Processor {
    registers: [u8; TOTAL_REGISTER_LENGTH], // all data in addressable registers. multibyte registers are stored little endian
}
