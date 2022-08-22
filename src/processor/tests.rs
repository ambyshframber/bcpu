use super::{*, registers::*};

#[test]
fn test_reg_decode() {
    assert_eq!(get_register_offset(GPRs::A as u8), Some((0, false)));
    assert_eq!(get_register_offset(GPRs::AL as u8), Some((0, true)));
    assert_eq!(get_register_offset(GPRs::AH as u8), Some((1, true)));

    assert_eq!(get_register_offset(GPRs::CL as u8), Some((8, true)));
}
