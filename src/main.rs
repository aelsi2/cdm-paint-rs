#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

extern crate alloc;

mod drawing;
mod editor;
mod graphics;
mod io;
mod shapes;

use alloc::{boxed::Box, collections::VecDeque};
use cdm_rt::{InterruptVector, Psr, interrupt_vectors};
use core::cell::RefCell;
use critical_section::Mutex;
use drawing::DrawingCtx;
use editor::{Editor, EditorMode};
use embedded_alloc::LlffHeap as Heap;
use io::{Buttons, display, input, menu};
use shapes::Shape;

static QUEUE: Mutex<RefCell<VecDeque<Box<dyn Shape>>>> = Mutex::new(RefCell::new(VecDeque::new()));
static EDITOR: Mutex<RefCell<Editor>> = Mutex::new(RefCell::new(Editor::new()));

interrupt_vectors![
    InterruptVector(io::on_input, Psr::None),
    InterruptVector(io::on_timer, Psr::None),
];

unsafe fn platform_init() {
    #[global_allocator]
    static HEAP: Heap = Heap::empty();
    embedded_alloc::init!(HEAP, 2048);
    input::set_handler(Some(on_input));
    unsafe { cdm::interrupt::enable() };
}

#[unsafe(no_mangle)]
extern "C" fn main() {
    unsafe { platform_init() };
    critical_section::with(|cs| update_ui(&*EDITOR.borrow_ref_mut(cs)));

    let mut ctx = DrawingCtx::new();
    loop {
        if let Some(shape) = { critical_section::with(|cs| QUEUE.borrow_ref_mut(cs).pop_front()) } {
            shape.draw(&mut ctx);
            display::update_range(&ctx.frame_buf, ctx.dirty_start, ctx.dirty_end);
            ctx.reset_dirty();
        }
    }
}

fn on_input(btn: Buttons) {
    critical_section::with(|cs| {
        let mut ed = EDITOR.borrow_ref_mut(cs);

        ed.move_cursor(btn.xy());
        if btn.has_any(Buttons::Shoulders) {
            ed.toggle_mode();
        }
        if btn.has_any(Buttons::A) {
            match ed.mode {
                EditorMode::Normal if ed.needs_cur2() => ed.set_cur2(),
                EditorMode::Normal => {
                    let shape = ed.pop_shape();
                    QUEUE.borrow_ref_mut(cs).push_back(shape);
                }
                EditorMode::Menu => ed.toggle_mode(),
            }
        }
        if btn.has_any(Buttons::B) {
            match ed.mode {
                EditorMode::Normal => ed.reset_cur2(),
                EditorMode::Menu => ed.toggle_mode(),
            }
        }
        update_ui(&*ed);
    });
}

fn update_ui(editor: &Editor) {
    if editor.mode == EditorMode::Menu {
        menu::set_cursor(Some(editor.cur_menu as i8));
        display::set_cur1(None);
    } else {
        menu::set_cursor(None);
        display::set_cur1(Some(editor.cur1));
    }
    menu::set_data(editor.tool, editor.color, editor.fill);
    display::set_cur2(editor.cur2);
}
