use super::DrawingCtx;
use crate::graphics::Color;
use crate::graphics::Point;
use core::cmp;

impl DrawingCtx {
    pub fn draw_line(&mut self, start: Point, end: Point, color: Color) {
        let (start, end) = (cmp::min(start, end), cmp::max(start, end));

        let x0 = start.x();
        let y0 = start.y();
        let x1 = end.x();
        let y1 = end.y();

        self.mark_dirty_range(y0 as usize, y1 as usize);

        if x0 == x1 {
            self.draw_vertical_line(start, end, color);
        } else if y0 == y1 {
            self.draw_horizontal_line(start, end, color);
        } else {
            let delta_x = i16::abs(x1 - x0);
            let delta_y = -i16::abs(y1 - y0);
            let dir_x = i16::signum(x1 - x0);
            let dir_y = i16::signum(y1 - y0);
            let mut error = delta_x + delta_y;
            let mut x = x0;
            let mut y = y0;

            loop {
                self.draw_pixel_impl(Point::new(x, y), color);
                let double_error = 2 * error;
                if double_error >= delta_y {
                    if x == x1 {
                        break;
                    }
                    error += delta_y;
                    x += dir_x;
                }
                if double_error <= delta_x {
                    if y == y1 {
                        break;
                    }
                    error += delta_x;
                    y += dir_y;
                }
            }
        }
    }
}
