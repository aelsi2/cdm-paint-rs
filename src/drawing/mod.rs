mod internal;
mod line;

use crate::graphics::Block;
use crate::graphics::Color;
use crate::graphics::FrameBuf;
use crate::graphics::Point;
use crate::graphics::SCREEN_HEIGHT;
use crate::graphics::FRAMEBUF_SIZE;

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
}
