#![no_std]
#![no_main]

mod cdm;
mod graphics;
mod io;

use cdm::Crit;
use core::cell::Cell;
use graphics::FRAMEBUF_SIZE;
use graphics::FrameBuf;
use graphics::Point;
use io::Buttons;
use io::Display;
use io::Input;

static CURSOR: Crit<Cell<Point>> = Crit::new(Cell::new(Point::zero()));

static FB: FrameBuf = [0x3333; FRAMEBUF_SIZE];

#[unsafe(no_mangle)]
extern "cdm-isr" fn main() {
    Input::set_handler(Some(on_input));
    Display::set_cur2(None);
    Display::update(&FB);
    loop {}
}

fn on_input(btn: Buttons) {
    let guard = CURSOR.enter();
    let mut point = guard.get();
    point += btn.point();
    guard.replace(point);
    Display::set_cur1(Some(point));
}
