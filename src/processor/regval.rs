use crate::utils::*;
use super::*;
use std::any::type_name_of_val;

#[derive(Debug, PartialEq)]
pub struct FlagUpdate {
    pub carry: bool,
    pub negative: bool,
    pub overflow: bool,
    pub zero: bool,
}
impl FlagUpdate {
    pub fn new(carry: bool, overflow: bool, val: RegVal) -> FlagUpdate {
        let negative = val.is_negative();
        let zero = val.is_zero();
        FlagUpdate { carry, negative, overflow, zero }
    }
    pub fn update_reg(self, old: u32) -> u32 {
        let mut flags = 0;
        flags |= self.carry as u32;
        flags |= (self.negative as u32) << 1;
        flags |= (self.overflow as u32) << 2;
        flags |= (self.zero as u32) << 3;

        (old & !0xf) | flags
    }
}

macro_rules! add {
    ($width:ty, $fn_x:ident, $variant:ident, $val:ident, $rhs:ident, $carry:ident) => {
        {
            let hi_pre = $val.top_bit();
            let rhs = $rhs.$fn_x()?;
            let (new, carry) = $val.carrying_add(rhs, $carry);
            let hi_post = new.top_bit();

            let overflow = hi_pre != hi_post;

            let val = RegVal::$variant(new);
        
            Some((val, FlagUpdate::new(carry, overflow, val)))
        }
    };
}
macro_rules! sub {
    ($width:ty, $fn_x:ident, $variant:ident, $val:ident, $rhs:ident, $carry:ident) => {
        {
            let hi_pre = $val.top_bit();
            let rhs = $rhs.$fn_x()?;
            let (new, borrow) = $val.borrowing_sub(rhs, !$carry);
            let hi_post = new.top_bit();

            let overflow = hi_pre != hi_post;

            let val = RegVal::$variant(new);
        
            Some((val, FlagUpdate::new(!borrow, overflow, val)))
        }
    };
}

macro_rules! one_output {
    ($name:ident) => {
        pub fn $name(self, rhs: RegVal, carry: bool) -> Option<(RegVal, FlagUpdate)> {
            match self {
                Self::Byte(b) => {
                    $name!(u8, unwrap_u8, Byte, b, rhs, carry)
                }
                Self::Word(w) => {
                    $name!(u16, unwrap_u16, Word, w, rhs, carry)
                }
                Self::Dword(d) => {
                    $name!(u32, unwrap_u32, Dword, d, rhs, carry)
                }
            }
        }
    };
}

macro_rules! mul {
    ($width:ty, $width_s:ty, $fn_x:ident, $val:ident, $rhs:ident, $variant:ident) => {
        {
            let lhs = $val as $width_s;
            let rhs = $rhs.$fn_x()? as $width_s;

            let (lo, hi) = lhs.widening_mul(rhs);
            let lo_wrap = RegVal::$variant(lo);
            let hi_wrap = RegVal::$variant(hi);
            let overflow = hi != 0;
            let flags = FlagUpdate {
                overflow,
                carry: false,
                negative: hi_wrap.is_negative(),
                zero: hi_wrap.is_zero() && lo_wrap.is_zero()
            };
            Some(((lo_wrap, hi_wrap), flags))
        }
    };
}
macro_rules! div {
    ($width_s:ty, $fn_x:ident, $val:ident, $val_hi:ident, $rhs:ident) => {
        {
            let lhs_lo = $val;
            let lhs_hi = $val_hi.$fn_x()?;
            let lhs = <$width_s>::merge(lhs_lo, lhs_hi);
            let rhs = $rhs.$fn_x()? as $width_s;
            let quot: RegVal = (lhs / rhs).half_split().0.into();
            let rem: RegVal = (lhs % rhs).half_split().0.into();
            let zero = rem.is_zero();
            let negative = quot.is_negative();
            let flags = FlagUpdate {
                zero, negative,
                overflow: false,
                carry: false
            };
            Some(((quot, rem), flags))
        }
    };
}

macro_rules! unwrap_rv {
    ($name:ident, $variant:ident, $ret:ty) => {
        pub fn $name(self) -> Option<$ret> {
            match self {
                Self::$variant(v) => Some(v),
                _ => None
            }
        }
    };
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RegVal {
    Byte(u8),
    Word(u16),
    Dword(u32),
}
impl RegVal {
    unwrap_rv!(unwrap_u8, Byte, u8);
    unwrap_rv!(unwrap_u16, Word, u16);
    unwrap_rv!(unwrap_u32, Dword, u32);
    pub fn to_u32(&self) -> u32 {
        match *self {
            Self::Byte(v) => v as u32,
            Self::Word(v) => v as u32,
            Self::Dword(v) => v as u32,
        }
    }
    pub fn from_u32(val: u32, gpr_select: u8) -> Self {
        let bytes = val.to_le_bytes();
        match gpr_select {
            0 => val.half_split().0.into(),
            1 => val.into(),
            2 => bytes[0].into(),
            4 => bytes[1].into(),
            _ => unreachable!()
        }
    }

    pub fn is_zero(&self) -> bool {
        match *self {
            Self::Byte(v) => v == 0,
            Self::Word(v) => v == 0,
            Self::Dword(v) => v == 0,
        }
    }
    pub fn is_negative(&self) -> bool {
        match *self {
            Self::Byte(v) => (v & 0x80) != 0,
            Self::Word(v) => (v & 0x80_00) != 0,
            Self::Dword(v) => (v & 0x8000_0000) != 0,
        }
    }
    
    fn zero_extend_u8(self) -> Option<RegVal> {
        match self {
            Self::Byte(_) => Some(self),
            _ => None
        }
    }
    fn zero_extend_u16(self) -> Option<RegVal> {
        match self {
            Self::Byte(v) => Some(RegVal::Word(v.zero_extend())),
            Self::Word(v) => Some(RegVal::Word(v.zero_extend())),
            _ => None
        }
    }
    fn zero_extend_u32(self) -> Option<RegVal> {
        match self {
            Self::Byte(v) => Some(RegVal::Word(v.zero_extend())),
            Self::Word(v) => Some(RegVal::Word(v.zero_extend())),
            Self::Dword(v) => Some(RegVal::Dword(v.zero_extend())),
        }
    }
    fn sign_extend_u8(self) -> Option<RegVal> {
        match self {
            Self::Byte(v) => Some(RegVal::Byte(v.sign_extend())),
            _ => None
        }
    }
    fn sign_extend_u16(self) -> Option<RegVal> {
        match self {
            Self::Byte(v) => Some(RegVal::Word(v.sign_extend())),
            Self::Word(v) => Some(RegVal::Word(v.sign_extend())),
            _ => None
        }
    }
    fn sign_extend_u32(self) -> Option<RegVal> {
        match self {
            Self::Byte(v) => Some(RegVal::Dword(v.sign_extend())),
            Self::Word(v) => Some(RegVal::Dword(v.sign_extend())),
            Self::Dword(v) => Some(RegVal::Dword(v.sign_extend())),
        }
    }

    pub fn sign_extend(self, size: RegSize) -> Result<RegVal> {
        match size {
            RegSize::Byte => self.sign_extend_u8(),
            RegSize::Word => self.sign_extend_u16(),
            RegSize::Dword => self.sign_extend_u32(),
        }.ok_or(Exception::InvalidOperation)
    }
    pub fn zero_extend(self, size: RegSize) -> Result<RegVal> {
        match size {
            RegSize::Byte => self.zero_extend_u8(),
            RegSize::Word => self.zero_extend_u16(),
            RegSize::Dword => self.zero_extend_u32(),
        }.ok_or(Exception::InvalidOperation)
    }

    one_output!(add);
    one_output!(sub);

    pub fn mul(self, rhs: RegVal) -> Option<((RegVal, RegVal), FlagUpdate)> {
        match self {
            Self::Byte(b) => {
                mul!(u8, u8, unwrap_u8, b, rhs, Byte)
            }
            Self::Word(w) => {
                mul!(u16, u16, unwrap_u16, w, rhs, Word)
            }
            Self::Dword(d) => {
                mul!(u32, u32, unwrap_u32, d, rhs, Dword)
            }
        }
    }
    pub fn imul(self, rhs: RegVal) -> Option<((RegVal, RegVal), FlagUpdate)> {
        match self {
            Self::Byte(b) => {
                mul!(u8, i8, unwrap_u8, b, rhs, Byte)
            }
            Self::Word(w) => {
                mul!(u16, i16, unwrap_u16, w, rhs, Word)
            }
            Self::Dword(d) => {
                mul!(u32, i32, unwrap_u32, d, rhs, Dword)
            }
        }
    }

    pub fn div(self, hi: RegVal, rhs: RegVal) -> Option<((RegVal, RegVal), FlagUpdate)> {
        match self {
            Self::Byte(v) => {
                div!(u16, unwrap_u8, v, hi, rhs)
            }
            Self::Word(v) => {
                div!(u32, unwrap_u16, v, hi, rhs)
            }
            Self::Dword(v) => {
                div!(u64, unwrap_u32, v, hi, rhs)
            }
        }
    }
    pub fn idiv(self, hi: RegVal, rhs: RegVal) -> Option<((RegVal, RegVal), FlagUpdate)> {
        match self {
            Self::Byte(v) => {
                div!(i16, unwrap_u8, v, hi, rhs)
            }
            Self::Word(v) => {
                div!(i32, unwrap_u16, v, hi, rhs)
            }
            Self::Dword(v) => {
                div!(i64, unwrap_u32, v, hi, rhs)
            }
        }
    }
}

macro_rules! rv_from {
    ($t:ty, $variant:ident) => {      
        impl From<$t> for RegVal {
            fn from(v: $t) -> Self { Self::$variant(v) }
        }
    };
}

rv_from!(u8, Byte);
rv_from!(u16, Word);
rv_from!(u32, Dword);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_sub() {
        let lhs = RegVal::Byte(20);
        let rhs = RegVal::Byte(30);
        let res = RegVal::Byte(50);
        assert_eq!(lhs.add(rhs, false), Some((res, FlagUpdate::new(false, false, res))));

        let lhs = RegVal::Byte(20);
        let rhs = RegVal::Byte(30);
        let res = RegVal::Byte(51);
        assert_eq!(lhs.add(rhs, true), Some((res, FlagUpdate::new(false, false, res))));

        let lhs = RegVal::Byte(30);
        let rhs = RegVal::Byte(-20i8 as u8);
        let res = RegVal::Byte(10);
        assert_eq!(lhs.add(rhs, false), Some((RegVal::Byte(10), FlagUpdate::new(true, false, res))));
        let lhs = RegVal::Byte(20);
        let rhs = RegVal::Byte(-30i8 as u8);
        let res = RegVal::Byte(-10i8 as u8);
        assert_eq!(lhs.add(rhs, false), Some((RegVal::Byte(-10i8 as u8), FlagUpdate::new(false, true, res))));

        let lhs = RegVal::Byte(30);
        let rhs = RegVal::Byte(20);
        let res = RegVal::Byte(10);
        assert_eq!(lhs.sub(rhs, true), Some((RegVal::Byte(10), FlagUpdate::new(true, false, res))));
    }

    #[test]
    fn mul() {
        let lhs = RegVal::Byte(-0x12i8 as u8);
        let rhs = RegVal::Byte(0x34);
        assert_eq!(lhs.imul(rhs), Some(((RegVal::Byte(0x58), RegVal::Byte(0xfc)), FlagUpdate{
            carry: false,
            zero: false,
            negative: true,
            overflow: true
        })));
    }

    #[test]
    fn div() {
        let lhs_lo = RegVal::Byte(0x34);
        let lhs_hi = RegVal::Byte(0x12);
        let rhs = RegVal::Byte(0x56);
        let div_res = lhs_lo.div(lhs_hi, rhs);
        let correct = Some((
            (RegVal::Byte(0x36), RegVal::Byte(0x10)),
            FlagUpdate {
                carry: false, overflow: false, zero: false, negative: false
            }
        ));
        assert_eq!(div_res, correct);

        let (lo, hi) = (-0x1234i16 as u16).half_split();
        let lhs_lo: RegVal = lo.into();
        let lhs_hi = hi.into();
        let rhs = RegVal::Byte(0x56);
        let div_res = lhs_lo.idiv(lhs_hi, rhs);
        let correct = Some((
            (RegVal::Byte(0xca), RegVal::Byte(0xf0)),
            FlagUpdate {
                carry: false, overflow: false, zero: false, negative: true
            }
        ));
        assert_eq!(div_res, correct)
    }

    #[test]
    fn extend() {
        let v = RegVal::Word(-10i16 as u16);
        assert_eq!(v.sign_extend(RegSize::Dword), Ok(RegVal::Dword(-10i32 as u32)));
    }
}
