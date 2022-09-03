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

pub enum RegVal {
    Byte(u8),
    Word(u16),
    Dword(u32),
}
impl RegVal {
    fn unwrap_u8(self) -> Option<u8> {
        match self {
            Self::Byte(b) => Some(b),
            _ => None
        }
    }
    fn unwrap_u16(self) -> Option<u16> {
        match self {
            Self::Word(w) => Some(w),
            _ => None
        }
    }
    fn unwrap_u32(self) -> Option<u32> {
        match self {
            Self::Dword(d) => Some(d),
            _ => None
        }
    }

    one_output!(add);
    one_output!(sub);
}
