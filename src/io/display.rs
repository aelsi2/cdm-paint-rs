use crate::graphics::Point;
use crate::graphics::FrameBuf;

unsafe extern "C" {
    safe fn display_set_primary_cursor(value: i16);
    safe fn display_set_secondary_cursor(value: i16);
    safe fn display_write_range(fb: &FrameBuf, start: isize, end: isize);
}

const CURSOR_DISABLE: i16 = -1;

pub struct Display;

impl Display {
    pub fn set_cur1(point: Option<Point>) {
        display_set_primary_cursor(match point {
            Some(point) => point.value(),
            None => CURSOR_DISABLE,
        });
    }

    pub fn set_cur2(point: Option<Point>) {
        display_set_secondary_cursor(match point {
            Some(point) => point.value(),
            None => CURSOR_DISABLE,
        });
    }

    pub fn update_range(fb: &FrameBuf, line_min: isize, line_max: isize) {
        display_write_range(fb, line_min, line_max);
    }
}
