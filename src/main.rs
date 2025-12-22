#![no_std]
#![no_main]

mod cdm;
mod drawing;
mod editor;
mod graphics;
mod io;

use cdm::Crit;
use core::cell::RefCell;
use drawing::DrawingCtx;
use editor::Editor;
use editor::EditorMode;
use editor::Queue;
use graphics::Shape;
use io::Buttons;
use io::Display;
use io::Input;
use io::Menu;

static QUEUE: Crit<RefCell<Queue<Shape, 16>>> = Crit::new(RefCell::new(Queue::new()));
static EDITOR: Crit<RefCell<Editor>> = Crit::new(RefCell::new(Editor::new()));

#[unsafe(no_mangle)]
extern "cdm-isr" fn main() {
    EDITOR.with(|ed| update_ui(&*ed.borrow()));
    Input::set_handler(Some(on_input));
    let mut ctx = DrawingCtx::new();
    loop {
        if let Some(shape) = {
            let guard = QUEUE.enter();
            guard.borrow_mut().dequeue()
        } {
            ctx.draw_shape(&shape);
            Display::update_range(&ctx.frame_buf, ctx.dirty_start, ctx.dirty_end);
            ctx.reset_dirty();
        } else {
            cdm::wait();
        }
    }
}

fn on_input(btn: Buttons) {
    let guard = EDITOR.enter();
    let ed = &mut *guard.borrow_mut();

    ed.move_cursor(btn.xy());
    if btn.has_any(Buttons::Shoulders) {
        ed.toggle_mode();
    }
    if btn.has_any(Buttons::A) {
        match ed.mode {
            EditorMode::Normal if ed.needs_cur2() => ed.set_cur2(),
            EditorMode::Normal => {
                let guard = QUEUE.enter();
                ed.enqueue(&mut *guard.borrow_mut())
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
    update_ui(ed);
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
