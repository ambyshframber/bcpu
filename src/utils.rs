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

#[test]
fn num_splits() {
    assert_eq!(0x12_34u16.half_split(), (0x34, 0x12));
    assert_eq!(0x12_34_56_78u32.half_split(), (0x56_78, 0x12_34));
}
