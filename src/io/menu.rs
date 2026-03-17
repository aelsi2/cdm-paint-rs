use super::MMIO;
use crate::graphics::Color;
use crate::graphics::Fill;
use crate::graphics::Tool;

#[repr(C)]
pub(super) struct MenuRegs {
    data: MenuData,
    cursor: i8,
}

#[derive(Clone, Copy, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
struct MenuData(u8);

impl MenuData {
    pub const fn new(tool: Tool, color: Color, fill: Fill) -> Self {
        let tool_data = tool as u8;
        let color_data = (color as u8) << 3;
        let fill_data = (fill as u8) << 4;
        MenuData(tool_data | color_data | fill_data)
    }
}

pub fn set_data(tool: Tool, color: Color, fill: Fill) {
    let data = MenuData::new(tool, color, fill);
    unsafe { core::ptr::write_volatile(&raw mut MMIO.menu.data, data) };
}

pub fn set_cursor(pos: Option<i8>) {
    let value = if let Some(pos) = pos { pos } else { -1 };
    unsafe { core::ptr::write_volatile(&raw mut MMIO.menu.cursor, value) };
}
