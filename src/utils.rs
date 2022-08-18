pub trait NumExtensions<T> {
    fn zero_extend(self) -> T;
    fn sign_extend(self) -> T;
}
impl NumExtensions<u16> for u8 {
    fn zero_extend(self) -> u16 {
        self as u16
    }
    fn sign_extend(self) -> u16 {
        self as i8 as i16 as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_ext() {
        assert_eq!(0b1000_1111u8.sign_extend(), 0b1111_1111_1000_1111u16);
        assert_eq!(0b1000_1111u8.zero_extend(), 0b0000_0000_1000_1111u16)
    }
}
