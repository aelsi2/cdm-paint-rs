use crate::graphics::Point;

unsafe extern "C" {
    safe fn display_set_primary_cursor(value: i16);
    safe fn display_set_secondary_cursor(value: i16);
}

const CURSOR_DISABLE: i16 = -1;

pub struct Display;

impl Display {
    pub fn set_cur1(point: Option<Point>) {
        display_set_primary_cursor(match point {
            Some(Point(val)) => val,
            None => CURSOR_DISABLE,
        });
    }

    pub fn set_cur2(point: Option<Point>) {
        display_set_secondary_cursor(match point {
            Some(Point(val)) => val,
            None => CURSOR_DISABLE,
        });
    }
}
