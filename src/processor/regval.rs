use crate::utils::*;
use super::*;

#[derive(Debug, PartialEq)]
pub struct FlagUpdate {
    pub carry: bool,
    pub overflow: bool
}
impl FlagUpdate {
    pub fn new(carry: bool, overflow: bool) -> FlagUpdate {
        Self {
            carry, overflow
        }
    }
}

macro_rules! add {
    ($width:ty, $fn_x:ident, $mask:expr, $variant:ident, $val:ident, $rhs:ident, $carry:ident) => {
        {
            let hi_pre = ($val & $mask);
            let rhs = $rhs.$fn_x()?;
            let new = $val.wrapping_add(rhs).wrapping_add($carry as $width);
            let hi_post = new & $mask;

            let carry = new <= $val;
            let overflow = hi_pre != hi_post;
        
            Some((RegVal::$variant(new), FlagUpdate::new(carry, overflow)))
        }
    };
}
macro_rules! sub {
    ($width:ty, $fn_x:ident, $mask:expr, $variant:ident, $val:ident, $rhs:ident, $carry:ident) => {
        {
            let hi_pre = ($val & $mask);
            let rhs = $rhs.$fn_x()?;
            let new = $val.wrapping_sub(rhs).wrapping_sub(!$carry as $width);
            let hi_post = new & $mask;

            let carry = !(new >= $val);
            let overflow = hi_pre != hi_post;
        
            Some((RegVal::$variant(new), FlagUpdate::new(carry, overflow)))
        }
    };
}

macro_rules! one_output {
    ($name:ident) => {
        pub fn $name(self, rhs: RegVal, carry: bool) -> Option<(RegVal, FlagUpdate)> {
            match self {
                Self::Byte(b) => {
                    $name!(u8, unwrap_u8, 0x80, Byte, b, rhs, carry)
                }
                Self::Word(w) => {
                    $name!(u16, unwrap_u16, 0x80_00, Word, w, rhs, carry)
                }
                Self::Dword(d) => {
                    $name!(u32, unwrap_u32, 0x80_00_00_00, Dword, d, rhs, carry)
                }
            }
        }
    };
}

macro_rules! mul {
    ($width:ty, $width_s:ty, $mul_width:ty, $split_width:ty, $fn_x:ident, $val:ident, $rhs:ident, $variant:ident) => {
        {
            let lhs = $val as $width_s as $mul_width;
            let rhs = $rhs.$fn_x()? as $width_s as $mul_width;

            let new = (lhs * rhs) as $split_width;
            let (lo, hi) = new.half_split();
            let lo_wrap = RegVal::$variant(lo);
            let hi_wrap = RegVal::$variant(hi);
            let overflow = hi != 0;
            let flags = FlagUpdate::new(false, overflow);
            Some(((lo_wrap, hi_wrap), flags))
        }
    };
}
macro_rules! mul_fn {
    ($name:ident, $width_s:ty, $mul_width:ty) => {
        pub fn $name(self, rhs: RegVal, carry: bool) -> Option<((RegVal, RegVal), FlagUpdate)> {
            match self {
                Self::Byte(b) => {
                    mul!(u8, $width_s, $mul_width, u16, unwrap_u8, b, rhs, Byte)
                }
                Self::Word(w) => {
                    mul!(u16, $width_s, $mul_width, u32, unwrap_u16, w, rhs, Word)
                }
                Self::Dword(d) => {
                    mul!(u32, $width_s, $mul_width, u64, unwrap_u32, d, rhs, Dword)
                }
            }
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
                mul!(u8, u8, u16, u16, unwrap_u8, b, rhs, Byte)
            }
            Self::Word(w) => {
                mul!(u16, u16, u32, u32, unwrap_u16, w, rhs, Word)
            }
            Self::Dword(d) => {
                mul!(u32, u32, u64, u64, unwrap_u32, d, rhs, Dword)
            }
        }
    }
    pub fn imul(self, rhs: RegVal) -> Option<((RegVal, RegVal), FlagUpdate)> {
        match self {
            Self::Byte(b) => {
                mul!(u8, i8, i16, u16, unwrap_u8, b, rhs, Byte)
            }
            Self::Word(w) => {
                mul!(u16, i16, i32, u32, unwrap_u16, w, rhs, Word)
            }
            Self::Dword(d) => {
                mul!(u32, i32, i64, u64, unwrap_u32, d, rhs, Dword)
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
        assert_eq!(lhs.add(rhs, false), Some((RegVal::Byte(50), FlagUpdate::new(false, false))));

        let lhs = RegVal::Byte(20);
        let rhs = RegVal::Byte(30);
        assert_eq!(lhs.add(rhs, true), Some((RegVal::Byte(51), FlagUpdate::new(false, false))));

        let lhs = RegVal::Byte(30);
        let rhs = RegVal::Byte(-20i8 as u8);
        assert_eq!(lhs.add(rhs, false), Some((RegVal::Byte(10), FlagUpdate::new(true, false))));
        let lhs = RegVal::Byte(20);
        let rhs = RegVal::Byte(-30i8 as u8);
        assert_eq!(lhs.add(rhs, false), Some((RegVal::Byte(-10i8 as u8), FlagUpdate::new(false, true))));

        let lhs = RegVal::Byte(30);
        let rhs = RegVal::Byte(20);
        assert_eq!(lhs.sub(rhs, true), Some((RegVal::Byte(10), FlagUpdate::new(true, false))));
    }

    #[test]
    fn mul() {
        let lhs = RegVal::Byte(-0x12i8 as u8);
        let rhs = RegVal::Byte(0x34);
        assert_eq!(lhs.imul(rhs), Some(((RegVal::Byte(0x58), RegVal::Byte(0xfc)), FlagUpdate::new(false, true))));
    }

    #[test]
    fn extend() {
        let v = RegVal::Word(-10i16 as u16);
        assert_eq!(v.sign_extend(RegSize::Dword), Ok(RegVal::Dword(-10i32 as u32)));
    }
}
