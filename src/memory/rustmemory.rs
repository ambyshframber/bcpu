use std::alloc::{alloc, dealloc, Layout};
use std::ptr::null_mut;
use super::Device;

const BANK_SIZE: usize = 2usize.pow(16);
const BANK_LAYOUT: Layout = Layout::new::<[u8; BANK_SIZE]>();

struct RustMemory {
    init: bool,
    mem: *mut u8
}
impl RustMemory {
    fn new() -> RustMemory {
        RustMemory {
            init: false,
            mem: null_mut()
        }
    }
    fn init(&mut self) {
        unsafe {
            let ptr = alloc(BANK_LAYOUT);
            self.mem = ptr
        }
        self.init = true
    }
}
impl Drop for RustMemory {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.mem, BANK_LAYOUT)
        }
    }
}

impl Device for RustMemory {
    fn write(&mut self, val: u8, offset: u32, _range: u32) {
        if !self.init { self.init() }
        unsafe {
            *self.mem.offset(offset as isize) = val
        }
    }
    fn write16(&mut self, val: [u8; 2], offset: u32, _range: u32) {
        if !self.init { self.init() }
        let ofs = offset as isize;
        unsafe {
            *self.mem.offset(ofs) = val[0];
            *self.mem.offset(ofs+ 1) = val[1];
        }
    }
    fn read(&mut self, offset: u32, _range: u32) -> u8 {
        if !self.init { 0 }
        else {
            unsafe {
                *self.mem.offset(offset as isize)
            }
        }
    }
    fn read16(&mut self, offset: u32, _range: u32) -> [u8; 2] {
        if !self.init { [0; 2] }
        else {
            let ofs = offset as isize;
            unsafe {
                [*self.mem.offset(ofs), *self.mem.offset(ofs + 1)]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn alloc_de() {
        let mut m = RustMemory::new();
        for i in 0..256 {
            m.write(i as u8, i, 0)
        }
        for i in 0..256 {
            assert_eq!(m.read(i, 0), i as u8)
        }
    }
}
