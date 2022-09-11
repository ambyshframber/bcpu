pub trait NumSplit<T> {
    /// returns (low, high)
    fn half_split(self) -> (T, T);
}

macro_rules! num_split {
    ($src:ty, $dest:ty) => {
        impl NumSplit<$dest> for $src {
            fn half_split(self) -> ($dest, $dest) {
                let a = self.to_le_bytes();
                let siz = std::mem::size_of::<$dest>();
                let lo = <$dest>::from_le_bytes(a[..siz].try_into().unwrap());
                let hi = <$dest>::from_le_bytes(a[siz..].try_into().unwrap());
                (lo, hi)
            }
        }
    };
}

num_split!(u16, u8);
num_split!(u32, u16);
num_split!(u64, u32);

pub trait ZeroExtend<T> {
    fn zero_extend(self) -> T;
}
macro_rules! zext {
    ($src:ty, $dest:ty) => {
        impl ZeroExtend<$dest> for $src {
            fn zero_extend(self) -> $dest {
                self as $dest
            }
        }
    };
}
zext!(u8, u8);
zext!(u8, u16);
zext!(u8, u32);
zext!(u16, u16);
zext!(u16, u32);
zext!(u32, u32);

pub trait SignExtend<T> {
    fn sign_extend(self) -> T;
}
macro_rules! sign_ext {
    ($src:ty, $int1:ty, $int2:ty, $dest:ty) => {
        impl SignExtend<$dest> for $src {
            fn sign_extend(self) -> $dest {
                self as $int1 as $int2 as $dest
            }
        }
    };
}
sign_ext!(u8, i8, i8, u8);
sign_ext!(u8, i8, i16, u16);
sign_ext!(u8, i8, i32, u32);
sign_ext!(u16, i16, i16, u16);
sign_ext!(u16, i16, i32, u32);
sign_ext!(u32, i32, i32, u32);

pub fn mix_u32(old_val: u32, in_val: u32, sel: u8) -> u32 {
    match sel {
        0 => (old_val & 0xffff_0000) | in_val,
        1 => in_val,
        2 => (old_val & 0xffff_ff00) | in_val,
        3 => (old_val & 0xffff_00ff) | in_val << 8,
        _ => unreachable!()
    }
}

#[test]
fn num_splits() {
    assert_eq!(0x12_34u16.half_split(), (0x34, 0x12));
    assert_eq!(0x12_34_56_78u32.half_split(), (0x56_78, 0x12_34));
}
