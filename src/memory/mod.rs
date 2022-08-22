use std::ops::Range;
use lua_device::LuaDevice;

mod rustmemory;
mod lua_device;

pub struct MemoryMap {
    devices: Vec<MMapDevice>
}
impl MemoryMap {
    fn find_device_get_offset(&mut self, addr: u32) -> Option<(&mut MMapDevice, u32, u32)> {
        for (dev_idx, d) in self.devices.iter().enumerate() {
            if let Some(range_idx) = d.mem_ranges.iter().position(|r| addr >= r.start && addr < r.end) {
                let dev = &mut self.devices[dev_idx];
                let offset = addr - dev.mem_ranges[range_idx].start;
                return Some((dev, offset, range_idx as u32))
            }
        };
        None
    }
    pub fn read(&mut self, addr: u32) -> u8 {
        if let Some((dev, offset, range_idx)) = self.find_device_get_offset(addr) {
            dev.dev.read(offset, range_idx as u32)
        } else { 0 }
    }
    pub fn read16(&mut self, addr: u32) -> [u8; 2] {
        if addr & 1 != 0 {
            [self.read(addr), self.read(addr + 1)]
        }
        else if let Some((dev, offset, range_idx)) = self.find_device_get_offset(addr) {
            dev.dev.read16(offset, range_idx as u32)
        } else { [0, 0] }
    }
    pub fn write(&mut self, val: u8, addr: u32) {
        if let Some((dev, offset, range_idx)) = self.find_device_get_offset(addr) {
            dev.dev.write(val, offset, range_idx as u32)
        }
    }
    pub fn write16(&mut self, val: [u8; 2], addr: u32) {
        if let Some((dev, offset, range_idx)) = self.find_device_get_offset(addr) {
            dev.dev.write16(val, offset, range_idx as u32)
        }
    }
}

struct MMapDevice {
    dev: Box<dyn Device>,
    mem_ranges: Vec<Range<u32>>
}

pub trait Device {
    fn write(&mut self, val: u8, offset: u32, range: u32);
    /// offset will ALWAYS be a multiple of 2
    fn write16(&mut self, val: [u8; 2], offset: u32, range: u32);
    fn read(&mut self, offset: u32, range: u32) -> u8 { 0 }
    /// offset will ALWAYS be a multiple of 2
    fn read16(&mut self, offset: u32, range: u32) -> [u8; 2] { [0, 0] }
    fn clock(&mut self) -> DevMsg { DevMsg::None }
}
pub enum DevMsg {
    None,
    Irq,
    Nmi
}
