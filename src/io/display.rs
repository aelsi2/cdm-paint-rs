use super::MMIO;
use crate::graphics::FrameBuf;
use crate::graphics::Point;

#[repr(C, packed)]
pub struct DisplayRegs {
    row: [u16; 2],
    row_index: u8,
}

#[repr(C)]
pub struct CursorRegs {
    primary: i16,
    secondary: i16,
}

const CURSOR_DISABLE: i16 = -1;

pub struct Display;

impl Display {
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

    pub fn update_range(fb: &FrameBuf, line_min: isize, line_max: isize) {
        unsafe {
            core::arch::asm!(
                "add r1, r0",
                "add r1, r0",
                "add r1, r0",
                "add r1, r0",
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
                in("r0") fb,
                in("r1") line_min,
                in("r2") line_max,
                in("r3") &raw mut MMIO.display.row_index,
                in("r4") &raw mut MMIO.display.row,
                lateout("r0") _, 
                lateout("r1") _,
                lateout("r2") _,
                lateout("r3") _,
                lateout("r4") _,
                lateout("r5") _,
                options(nostack),
            );
        }
    }
}
