use crate::graphics::Tool;
use crate::graphics::Color;
use crate::graphics::Fill;

#[derive(Clone, Copy, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
struct MenuData(u8);

pub struct Menu;

impl MenuData {
    pub const fn new(tool: Tool, color: Color, fill: Fill) -> Self {
        let tool_data = tool as u8;
        let color_data = (color as u8) << 3;
        let fill_data = (fill as u8) << 4;
        MenuData(tool_data | color_data | fill_data)
    }
}

unsafe extern "C" {
    safe fn menu_set_data(data: MenuData);
    safe fn menu_set_cursor(pos: i8);
}

impl Menu {
    pub fn set_data(tool: Tool, color: Color, fill: Fill) {
        let data = MenuData::new(tool, color, fill);
        menu_set_data(data);
    }

    pub fn set_cursor(pos: Option<i8>) {
        if let Some(pos) = pos {
            menu_set_cursor(pos);
        } else {
            menu_set_cursor(-1);
        }
    }
}
