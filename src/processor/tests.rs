use super::{*, consts::GPRs};

#[test]
fn gpr_mov() {
    let mut p = Processor::new();

    let mov_res = p.mov(Operand::Const(0xffu8.into()), Operand::Register(GPRs::BH as u8), false);
    assert!(mov_res.is_ok());
    assert_eq!(p.registers.xb, 0x0000_ff00);
    
    p.registers.xa = 0x1234_5678;
    let mov_res = p.mov(Operand::Register(GPRs::A as u8), Operand::Register(GPRs::XC as u8), false);
    assert!(mov_res.is_ok());
    assert_eq!(p.registers.xc, 0x0000_5678);

    let mov_res = p.mov(Operand::Const((-10i16 as u16).into()), Operand::Register(GPRs::XC as u8), true);
    assert!(mov_res.is_ok());
    assert_eq!(p.registers.xc, -10i32 as u32);

}
