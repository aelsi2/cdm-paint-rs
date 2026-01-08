use super::DrawingCtx;
use crate::collections::Stack;
use crate::graphics::Color;
use crate::graphics::Point;
use crate::graphics::SCREEN_HEIGHT;
use crate::graphics::SCREEN_WIDTH;

const STACK_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

impl DrawingCtx {
    pub fn flood_fill(&mut self, point: Point, color: Color) {
        let mut stack = Stack::<Point, STACK_SIZE>::new();

        _ = stack.push(point);
        while !stack.is_empty() {
            let point = stack.pop().unwrap();
            let row_start = Point::new(0, point.y());
            let row_end = Point::new(SCREEN_WIDTH as i16, point.y());

            let mut start = point - Point::ONE;
            while start >= row_start && self.get_pixel(start) != color {
                start -= Point::ONE;
            }

            let mut end = point;
            while end < row_end && self.get_pixel(end) != color {
                end += Point::ONE;
            }
            start += Point::ONE;
            end -= Point::ONE;

            let y = point.y();
            self.mark_dirty(y as usize);
            self.draw_horizontal_line(start, end, color);

            let mut push_adjacent = |start: Point, end: Point| {
                let mut added = false;
                let y = start.y();
                for x in start.x()..=end.x() {
                    let point = Point::new(x, y);
                    if self.get_pixel(point) == color {
                        added = false;
                    } else if !added {
                        _ = stack.push(point);
                        added = true;
                    }
                }
            };

            if y < SCREEN_HEIGHT as i16 - 1 {
                push_adjacent(start + Point::new(0, 1), end + Point::new(0, 1));
            }
            if y > 0 {
                push_adjacent(start - Point::new(0, 1), end - Point::new(0, 1));
            }
        }
    }
}
