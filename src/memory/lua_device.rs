use hlua::{Lua, LuaFunction, LuaTable};
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
    fn write(&mut self, val: u8, offset: usize) -> DevMsg { DevMsg::None }
    fn read(&mut self, offset: usize) -> u8 { 0 }
    fn clock(&mut self) -> DevMsg { DevMsg::None }
    
}
