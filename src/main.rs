#![no_std]
#![no_main]

mod cdm;
mod graphics;
mod io;

use cdm::Crit;
use core::cell::Cell;
use io::Input;
use io::Display;
use io::Buttons;
use graphics::Point;

static CURSOR: Crit<Cell<Point>> = Crit::new(Cell::new(Point::zero()));

#[unsafe(no_mangle)]
extern "cdm-isr" fn main() {
    Input::set_handler(on_input);
    loop {}
}

fn on_input(btn: Buttons) {
    let guard = CURSOR.enter();
    let mut point = guard.get();
    point += btn.point();
    guard.replace(point);
    Display::set_primary_cursor(point);
}
