#![no_std]
#![no_main]

mod cdm;
mod graphics;
mod io;

use cdm::Crit;
use core::cell::RefCell;
use io::Input;
use io::Display;
use io::Buttons;
use graphics::Point;

static CURSOR: Crit<RefCell<Point>> = Crit::new(RefCell::new(Point::zero()));

#[unsafe(no_mangle)]
extern "cdm-isr" fn main() {
    Input::set_handler(on_input);
    loop {}
}

// LLVM ERROR: Cannot select: t45: i16,ch = load<(dereferenceable load (s8) from @_RNvCs8YgN0NbIE59_12cdm_paint_rs6CURSOR.0, align 2), zext from i1> t15, t48, undef:i16
//   t48: i16 = CDMISD::LOAD_SYM TargetGlobalAddress:i16<ptr @_RNvCs8YgN0NbIE59_12cdm_paint_rs6CURSOR.0> 0
// In function: _RNvCs8YgN0NbIE59_12cdm_paint_rs8on_input
fn on_input(btn: Buttons) {
    let guard = CURSOR.enter();
    let mut point = guard.borrow_mut();
    *point += btn.point();
    Display::set_primary_cursor(*point);
}
