pub const GPR_MASK: u8 = 0b1100;
pub const GPR_SEL_MASK: u8 = 0b11;
pub const PTR_MASK: u8 = 0b1110;
pub const PTR_SEL_MASK: u8 = 0b1;
pub const SPEC_MASK: u8 = 0b110;
pub const SPEC_SEL_MASK: u8 = 0b1;

pub const CARRY_MASK: u32    = 0b0000_0000_0000_0001;
pub const NEGATIVE_MASK: u32 = 0b0000_0000_0000_0010;
pub const OVERFLOW_MASK: u32 = 0b0000_0000_0000_0100;
pub const ZERO_MASK: u32     = 0b0000_0000_0000_1000;
pub const TEST_MASK: u32     = 0b0000_0000_0001_0000;
pub const DSEG_MASK: u32     = 0b0000_0000_0010_0000;
pub const PRIV_MASK: u32     = 0b0000_0000_1100_0000;
pub const MODE32_MASK: u32   = 0b0000_0001_0000_0000;

pub enum GPRs {
    A	= 0x00,
    XA	= 0x01,
    AL	= 0x02,
    AH	= 0x03,
    
    B	= 0x04,
    XB	= 0x05,
    BL	= 0x06,
    BH	= 0x07,

    C	= 0x08,
    XC	= 0x09,
    CL	= 0x0a,

    D	= 0x0c,
    XD	= 0x0d,
    DL	= 0x0e,
}
pub enum Ptrs {
    SP  = 0x10,
    XSP  = 0x11,
    BP  = 0x12,
    XBP  = 0x13,
    SI  = 0x14,
    XSI  = 0x15,
    DI  = 0x16,
    XDI  = 0x17,

    RP  = 0x18,
    XRP  = 0x19,
    ROP = 0x1a,
}
pub enum Offs {
    CO  = 0x20,
    DO  = 0x21,
    EO  = 0x22,
    SO  = 0x23,
}
pub enum Spec {
    IDTP    = 0x28,
    IDTL    = 0x2a,
    PC      = 0x2c,
    FLAGS   = 0x2e
}
