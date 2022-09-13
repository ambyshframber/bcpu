use super::*;

#[test]
fn gpr_mov() {
    let mut p = Processor::default();

    let mov_res = p.mov(Operand::Const(0xffu8.into()), Operand::Register(GPRs::BH as u8), false);
    assert!(mov_res.is_ok());
    assert_eq!(p.xb, 0x0000_ff00);
    
    p.xa = 0x1234_5678;
    let mov_res = p.mov(Operand::Register(GPRs::A as u8), Operand::Register(GPRs::XC as u8), false);
    assert!(mov_res.is_ok());
    assert_eq!(p.xc, 0x0000_5678);

    let mov_res = p.mov(Operand::Const((-10i16 as u16).into()), Operand::Register(GPRs::XC as u8), true);
    assert!(mov_res.is_ok());
    assert_eq!(p.xc, -10i32 as u32);
}

#[test]
fn ptr_mov() {
    let mut p = Processor::default();
    
    p.xsp = 0x1234_5678;
    let mov_res = p.mov(Operand::Register(Ptrs::SP as u8), Operand::Register(Ptrs::BP as u8), false);
    assert!(mov_res.is_ok());
    assert_eq!(p.xbp, 0x0000_5678)
}
