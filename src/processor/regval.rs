use crate::utils::NumSplit;

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

#[derive(Debug, PartialEq)]
pub enum RegVal {
    Byte(u8),
    Word(u16),
    Dword(u32),
}
impl RegVal {
    pub fn unwrap_u8(self) -> Option<u8> {
        match self {
            Self::Byte(b) => Some(b),
            _ => None
        }
    }
    pub fn unwrap_u16(self) -> Option<u16> {
        match self {
            Self::Word(w) => Some(w),
            _ => None
        }
    }
    pub fn unwrap_u32(self) -> Option<u32> {
        match self {
            Self::Dword(d) => Some(d),
            _ => None
        }
    }
    pub fn to_u32(self) -> u32 {
        match self {
            Self::Byte(v) => v as u32,
            Self::Word(v) => v as u32,
            Self::Dword(v) => v as u32,
        }
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

        let lhs = RegVal::Byte(30);
        let rhs = RegVal::Byte(20);
        assert_eq!(lhs.sub(rhs, true), Some((RegVal::Byte(10), FlagUpdate::new(true, false))));

        let lhs = RegVal::Byte(-0x12i8 as u8);
        let rhs = RegVal::Byte(0x34);
        assert_eq!(lhs.imul(rhs), Some(((RegVal::Byte(0x58), RegVal::Byte(0xfc)), FlagUpdate::new(false, true))));
    }
}
