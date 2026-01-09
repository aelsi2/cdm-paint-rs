#![no_std]
#![no_main]

extern crate alloc;

mod cdm;
mod drawing;
mod shapes;
mod editor;
mod graphics;
mod io;

use alloc::boxed::Box;
use alloc::collections::VecDeque;
use core::cell::RefCell;
use critical_section::Mutex;
use drawing::DrawingCtx;
use editor::Editor;
use editor::EditorMode;
use shapes::Shape;
use io::Buttons;
use io::Display;
use io::Input;
use io::Menu;

static QUEUE: Mutex<RefCell<VecDeque<Box<dyn Shape>>>> = Mutex::new(RefCell::new(VecDeque::new()));
static EDITOR: Mutex<RefCell<Editor>> = Mutex::new(RefCell::new(Editor::new()));

#[unsafe(no_mangle)]
extern "cdm-isr" fn main() {
    critical_section::with(|cs| unsafe {
        cdm::initialize();
        update_ui(&*EDITOR.borrow_ref_mut(cs));
    });
    Input::set_handler(Some(on_input));
    let mut ctx = DrawingCtx::new();
    loop {
        if let Some(shape) = { critical_section::with(|cs| QUEUE.borrow_ref_mut(cs).pop_front()) } {
            shape.draw(&mut ctx);
            Display::update_range(&ctx.frame_buf, ctx.dirty_start, ctx.dirty_end);
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
                    ed.enqueue(&mut QUEUE.borrow_ref_mut(cs));
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
        Menu::set_cursor(Some(editor.cur_menu as i8));
        Display::set_cur1(None);
    } else {
        Menu::set_cursor(None);
        Display::set_cur1(Some(editor.cur1));
    }
    Menu::set_data(editor.tool, editor.color, editor.fill);
    Display::set_cur2(editor.cur2);
}
