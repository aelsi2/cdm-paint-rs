use crate::cdm::Crit;
use crate::graphics::Point;
use bitmask_enum::bitmask;
use core::cell::RefCell;
use core::ptr;

#[bitmask(u8)]
pub enum Buttons {
    None = 0,
    A = 1,
    B = 2,
    Down = 4,
    Up = 8,
    Right = 16,
    Left = 32,
    R = 64,
    L = 128,

    Directions = Self::Down.bits | Self::Up.bits | Self::Left.bits | Self::Right.bits,
    Actions = Self::A.bits | Self::B.bits | Self::L.bits | Self::R.bits,
    Shoulders = Self::L.bits | Self::R.bits,
}

unsafe extern "C" {
    static input_state: crate::io::Buttons;
    safe fn timer_disable();
    safe fn timer_enable();
    safe fn display_set_primary_cursor(position: Point);
    safe fn display_set_secondary_cursor(position: Point);
}

impl Buttons {
    pub fn x(self) -> i16 {
        match self & (Buttons::Left | Buttons::Right) {
            Buttons::Left => -1,
            Buttons::Right => 1,
            _ => 0,
        }
    }

    pub fn y(self) -> i16 {
        match self & (Buttons::Up | Buttons::Down) {
            Buttons::Up => -1,
            Buttons::Down => 1,
            _ => 0,
        }
    }

    pub fn point(self) -> Point {
        Point::new(self.x(), self.y())
    }

    pub fn current() -> Buttons {
        unsafe {
            return ptr::read_volatile(&input_state);
        }
    }
}

struct InputState {
    transition_counter: usize,
    is_repeating: bool,
    joy_old: Buttons,
    handler: Option<fn(Buttons) -> ()>,
}

static INPUT_STATE: Crit<RefCell<InputState>> = Crit::new(RefCell::new(InputState {
    transition_counter: 0,
    is_repeating: false,
    joy_old: Buttons::None,
    handler: None,
}));

const TRANSITION_MAX: usize = 3;

pub struct Input;

impl Input {
    pub fn set_handler(handler: fn(Buttons) -> ()) {
        let guard = INPUT_STATE.enter();
        let mut state = guard.borrow_mut();
        state.handler = Some(handler);
    }
}

#[unsafe(no_mangle)]
unsafe extern "cdm-isr" fn on_input() {
    let guard = INPUT_STATE.enter();
    let mut state = guard.borrow_mut();
    let Some(on_input) = state.handler else {
        return;
    };

    let joy_new = Buttons::current();

    let joy_pressed = joy_new & !state.joy_old;
    let joy_dirs = joy_pressed & Buttons::Directions;
    let joy_actions = joy_pressed & Buttons::Actions;

    if joy_dirs != Buttons::None {
        if !state.is_repeating {
            on_input(joy_dirs);
            state.transition_counter = TRANSITION_MAX;
        }
        timer_enable();
    }
    if joy_actions != Buttons::None {
        on_input(joy_actions);
    }
    state.joy_old = joy_new;
}

#[unsafe(no_mangle)]
unsafe extern "cdm-isr" fn on_timer() {
    let guard = INPUT_STATE.enter();
    let mut state = guard.borrow_mut();
    let Some(on_input) = state.handler else {
        return;
    };

    let joy_dirs = Buttons::current() & Buttons::Directions;

    if state.is_repeating && joy_dirs != Buttons::None {
        on_input(joy_dirs);
        state.transition_counter = TRANSITION_MAX;
    }

    if state.transition_counter > 0 {
        state.transition_counter -= 1;
    } else if state.is_repeating {
        state.is_repeating = false;
        timer_disable();
    } else {
        state.is_repeating = true;
    }
}

pub struct Display;

impl Display {
    pub fn set_primary_cursor(point: Point) {
        display_set_primary_cursor(point);
    }
    pub fn set_secondary_cursor(point: Point) {
        display_set_secondary_cursor(point);
    }
}
