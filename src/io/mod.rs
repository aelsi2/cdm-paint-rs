pub mod display;
pub mod input;
pub mod menu;

pub use input::Buttons;
pub use input::on_input;
pub use input::on_timer;

use cdm_uart::Uart;
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
    uart_flags: u8,
    uart_data: u8,
}

pub fn uart() -> Uart {
    unsafe { Uart::new(&raw mut MMIO.uart_flags, &raw mut MMIO.uart_data) }
}

unsafe extern "C" {
    #[link_name = "__MMIO"]
    static mut MMIO: MmioRegs;
}
