use lua_device::LuaDevice;

mod lua_device;

pub struct MemoryMap {

}
struct MMapDevice {
    dev: Box<dyn Device>,
    start: usize,
    end: usize,
}

pub trait Device {
    fn write(&mut self, val: u8, offset: usize) -> DevMsg { DevMsg::None }
    fn read(&mut self, offset: usize) -> u8 { 0 }
    fn clock(&mut self) -> DevMsg { DevMsg::None }
}
pub enum DevMsg {
    None
}
