use super::MMIO;
use crate::graphics::FrameBuf;
use crate::graphics::Point;

#[repr(C, packed)]
pub(super) struct DisplayRegs {
    row: [u16; 2],
    row_index: u8,
}

#[repr(C)]
pub(super) struct CursorRegs {
    primary: i16,
    secondary: i16,
}

const CURSOR_DISABLE: i16 = -1;

pub fn set_cur1(point: Option<Point>) {
    let value = match point {
        Some(point) => point.into(),
        None => CURSOR_DISABLE,
    };
    unsafe { core::ptr::write_volatile(&raw mut MMIO.display_cursors.primary, value) };
}

pub fn set_cur2(point: Option<Point>) {
    let value = match point {
        Some(point) => point.into(),
        None => CURSOR_DISABLE,
    };
    unsafe { core::ptr::write_volatile(&raw mut MMIO.display_cursors.secondary, value) };
}

#[unsafe(naked)]
pub extern "C" fn update_range(fb: &FrameBuf, line_min: isize, line_max: isize) {
    core::arch::naked_asm!(
        "push r4",
        "push r5",
        "add r1, r0",
        "add r1, r0",
        "add r1, r0",
        "add r1, r0",
        "ldi r3, {mmio}+4",
        "ldi r4, {mmio}",
        "br 1f",
        "0:",
        "stb r3, r1",
        "ldw r0, r5",
        "stw r4, r5",
        "add r4, 2",
        "add r0, 2",
        "ldw r0, r5",
        "stw r4, r5",
        "sub r4, 2",
        "add r0, 2",
        "inc r1",
        "1:",
        "cmp r1, r2",
        "ble 0b",
        "pop r5",
        "pop r4",
        "rts",
        mmio = sym MMIO
    );
}
