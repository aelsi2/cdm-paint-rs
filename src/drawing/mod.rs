mod internal;
mod line;

use crate::graphics::Block;
use crate::graphics::Color;
use crate::graphics::FRAMEBUF_SIZE;
use crate::graphics::FrameBuf;
use crate::graphics::Point;
use crate::graphics::SCREEN_HEIGHT;
use core::cmp;

pub struct DrawingCtx {
    pub frame_buf: FrameBuf,
    pub dirty_start: isize,
    pub dirty_end: isize,
}

impl DrawingCtx {
    pub fn new() -> DrawingCtx {
        DrawingCtx {
            frame_buf: [0; FRAMEBUF_SIZE],
            dirty_start: SCREEN_HEIGHT as isize,
            dirty_end: -1,
        }
    }

    pub fn reset_dirty(&mut self) {
        self.dirty_end = -1;
        self.dirty_start = SCREEN_HEIGHT as isize;
    }

    pub fn clear(&mut self, color: Color) {
        let block: Block = match color {
            Color::White => 0xFFFF,
            Color::Black => 0x0000,
        };
        self.frame_buf.fill(block);
        self.mark_dirty_range(0, SCREEN_HEIGHT - 1);
    }

    pub fn draw_pixel(&mut self, point: Point, color: Color) {
        self.draw_pixel_impl(point, color);
        self.mark_dirty(point.y() as usize);
    }

    pub fn draw_filled_rect(&mut self, p1: Point, p2: Point, color: Color) {
        let min_row = cmp::min(p1.y(), p2.y());
        let max_row = cmp::max(p1.y(), p2.y());
        let min_col = cmp::min(p1.x(), p2.x());
        let max_col = cmp::max(p1.x(), p2.x());
        self.mark_dirty_range(min_row as usize, max_row as usize);

        for row in min_row..=max_row {
            self.draw_horizontal_line(Point::new(min_col, row), Point::new(max_col, row), color);
        }
    }

    pub fn draw_outline_rect(&mut self, p1: Point, p2: Point, color: Color) {
        let min_y = cmp::min(p1.y(), p2.y());
        let max_y = cmp::max(p1.y(), p2.y());
        let min_x = cmp::min(p1.x(), p2.x());
        let max_x = cmp::max(p1.x(), p2.x());
        self.mark_dirty_range(min_y as usize, max_y as usize);

        self.draw_horizontal_line(Point::new(min_x, max_y), Point::new(max_x, max_y), color);
        self.draw_horizontal_line(Point::new(min_x, min_y), Point::new(max_x, min_y), color);
        self.draw_vertical_line(Point::new(min_x, min_y), Point::new(min_x, max_y), color);
        self.draw_vertical_line(Point::new(max_x, min_y), Point::new(max_x, max_y), color);
    }
}
