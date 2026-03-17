mod display;
mod input;
mod menu;

pub use display::Display;
pub use input::Buttons;
pub use input::Input;
pub use input::on_input;
pub use input::on_timer;
pub use menu::Menu;

use display::CursorRegs;
use display::DisplayRegs;
use menu::MenuRegs;

#[repr(C)]
struct MmioRegs {
    display: DisplayRegs,
    timer: bool,
    display_cursors: CursorRegs,
    menu: MenuRegs,
    input: Buttons,
}

#[used]
#[unsafe(link_section = ".mmio")]
static mut MMIO: MmioRegs = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
