mod display;
mod input;
mod menu;

pub use display::Display;
pub use input::Buttons;
pub use input::Input;
pub use menu::Menu;
pub use input::on_input;
pub use input::on_timer;

core::arch::global_asm!(include_str!("./io.asm"));
