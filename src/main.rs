#![no_std]
#![no_main]

mod cdm;
mod drawing;
mod graphics;
mod io;

use cdm::Crit;
use core::cell::Cell;
use drawing::DrawingCtx;
use graphics::Color;
use graphics::Point;
use io::Buttons;
use io::Display;
use io::Input;

static CURSOR: Crit<Cell<Point>> = Crit::new(Cell::new(Point::zero()));

#[unsafe(no_mangle)]
extern "cdm-isr" fn main() {
    Input::set_handler(Some(on_input));
    Display::set_cur2(None);
    let mut ctx = DrawingCtx::new();
    ctx.clear(Color::White);
    ctx.draw_line(Point::new(5, 5), Point::new(11, 27), Color::Black);
    Display::update_range(&ctx.frame_buf, ctx.dirty_start, ctx.dirty_end);
    ctx.reset_dirty();
    loop {}
}

fn on_input(btn: Buttons) {
    let guard = CURSOR.enter();
    let mut point = guard.get();
    point += btn.point();
    guard.replace(point);
    Display::set_cur1(Some(point));
}
