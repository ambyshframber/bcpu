use hlua::{Lua, LuaFunction};
use super::{DevMsg, Device};

pub struct LuaDevice<'a> {
    lua: Lua<'a>,
}
impl<'a> LuaDevice<'a> {
    fn new(code: &str) -> (LuaDevice, String) {
        let mut lua = Lua::new();
        lua.execute::<()>(code).unwrap();
        let id: String = lua.get("DEVICE_ID").unwrap();
        let dev = LuaDevice {
            lua
        };
        (dev, id)
    }
}
impl Device for LuaDevice<'_> {
    fn write(&mut self, val: u8, offset: u32, range: u32) {}
    fn write16(&mut self, val: [u8; 2], offset: u32, range: u32) {}
    fn read(&mut self, offset: u32, range: u32) -> u8 { 0 }
    fn read16(&mut self, offset: u32, range: u32) -> [u8; 2] { [0, 0] }
    fn clock(&mut self) -> DevMsg { DevMsg::None }
}
