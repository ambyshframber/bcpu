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
num_split!(i16, u8);
num_split!(i32, u16);
num_split!(i64, u32);

pub trait NumMerge<T> {
    fn merge(lo: T, hi: T) -> Self;
}
macro_rules! num_merge {
    ($src:ty, $dest:ty) => {
        impl NumMerge<$src> for $dest {
            fn merge(lo: $src, hi: $src) -> Self {
                let lo_bytes = lo.to_le_bytes();
                let hi_bytes = hi.to_le_bytes();
                let array = [lo_bytes, hi_bytes].concat().try_into().unwrap();
                <$dest>::from_le_bytes(array)
            }
        }
    };
}
num_merge!(u8, u16);
num_merge!(u16, u32);
num_merge!(u32, u64);
num_merge!(u8, i16);
num_merge!(u16, i32);
num_merge!(u32, i64);

pub trait MergeUp<T> {
    fn merge_up(lo: Self, hi: Self) -> T;
}
impl<T, U> MergeUp<T> for U
where T: NumMerge<U> {
    fn merge_up(lo: Self, hi: Self) -> T {
        T::merge(lo, hi)
    }
}

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

pub trait SignedWideMul<T> {
    fn widening_mul(self, rhs: Self) -> (T, T);
}
macro_rules! wide_mul {
    ($from:ty, $signed_wide:ty, $out:ty) => {
        impl SignedWideMul<$out> for $from {
            fn widening_mul(self, rhs: Self) -> ($out, $out) {
                let lhs = self as $signed_wide;
                let rhs = rhs as $signed_wide;
                (lhs * rhs).half_split()
            }
        }
    };
}
wide_mul!(i8, i16, u8);
wide_mul!(i16, i32, u16);
wide_mul!(i32, i64, u32);

pub trait TopBit {
    fn top_bit(self) -> bool;
}
macro_rules! top_bit {
    ($type:ty, $mask:expr) => {
        impl TopBit for $type {
            fn top_bit(self) -> bool {
                (self & $mask) != 0
            }
        }
    };
}
top_bit!(u8, 0x80);
top_bit!(u16, 0x80_00);
top_bit!(u32, 0x8000_0000);

#[test]
fn num_splits() {
    assert_eq!(0x12_34u16.half_split(), (0x34, 0x12));
    assert_eq!(0x12_34_56_78u32.half_split(), (0x56_78, 0x12_34));
}

#[test]
fn signed_merge() {
    let res = i16::merge(0x34, 0x12);
    assert_eq!(res, 0x1234);

    let (lo, hi) = (-0x1234i16 as u16).half_split();
    let res = i16::merge(lo, hi);
    assert_eq!(res, -0x1234)
}
