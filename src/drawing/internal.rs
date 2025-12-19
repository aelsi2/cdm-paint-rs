use crate::drawing::DrawingCtx;
use crate::graphics::BLOCKS_PER_ROW;
use crate::graphics::Block;
use crate::graphics::Color;
use crate::graphics::PIXELS_PER_BLOCK;
use crate::graphics::Point;
use core::cmp;

const MASK_LEFT: [Block; PIXELS_PER_BLOCK] = [
    0b1111111111111111,
    0b0111111111111111,
    0b0011111111111111,
    0b0001111111111111,
    0b0000111111111111,
    0b0000011111111111,
    0b0000001111111111,
    0b0000000111111111,
    0b0000000011111111,
    0b0000000001111111,
    0b0000000000111111,
    0b0000000000011111,
    0b0000000000001111,
    0b0000000000000111,
    0b0000000000000011,
    0b0000000000000001,
];

const MASK_RIGHT: [Block; PIXELS_PER_BLOCK] = [
    0b1000000000000000,
    0b1100000000000000,
    0b1110000000000000,
    0b1111000000000000,
    0b1111100000000000,
    0b1111110000000000,
    0b1111111000000000,
    0b1111111100000000,
    0b1111111110000000,
    0b1111111111000000,
    0b1111111111100000,
    0b1111111111110000,
    0b1111111111111000,
    0b1111111111111100,
    0b1111111111111110,
    0b1111111111111111,
];

const MASK_PIXEL: [Block; PIXELS_PER_BLOCK] = [
    0b1000000000000000,
    0b0100000000000000,
    0b0010000000000000,
    0b0001000000000000,
    0b0000100000000000,
    0b0000010000000000,
    0b0000001000000000,
    0b0000000100000000,
    0b0000000010000000,
    0b0000000001000000,
    0b0000000000100000,
    0b0000000000010000,
    0b0000000000001000,
    0b0000000000000100,
    0b0000000000000010,
    0b0000000000000001,
];

impl DrawingCtx {
    pub(super) fn draw_vertical_line(&mut self, start: Point, end: Point, color: Color) {
        let block = MASK_PIXEL[start.pixel_index()];
        for block_index in (start.block_index()..=end.block_index()).step_by(BLOCKS_PER_ROW) {
            self.draw_pattern(block, block_index, color);
        }
    }

    pub(super) fn draw_horizontal_line(&mut self, start: Point, end: Point, color: Color) {
        let start_bi = start.block_index();
        let end_bi = end.block_index();

        if start_bi == end_bi {
            let block = MASK_LEFT[start.pixel_index()] | MASK_RIGHT[end.pixel_index()];
            self.draw_pattern(block, start_bi, color);
        } else {
            let start_block = MASK_LEFT[start.pixel_index()];
            let end_block = MASK_RIGHT[end.pixel_index()];
            self.draw_pattern(start_block, start_bi, color);
            self.draw_pattern(end_block, end_bi, color);
        }
    }

    pub(super) fn mark_dirty_range(&mut self, start: usize, end: usize) {
        self.dirty_start = cmp::min(start as isize, self.dirty_start);
        self.dirty_end = cmp::max(end as isize, self.dirty_end);
    }

    pub(super) fn mark_dirty(&mut self, line: usize) {
        self.dirty_start = cmp::min(line as isize, self.dirty_start);
        self.dirty_end = cmp::max(line as isize, self.dirty_end);
    }

    pub(super) fn draw_pixel_impl(&mut self, point: Point, color: Color) {
        let block = MASK_PIXEL[point.pixel_index()];
        self.draw_pattern(block, point.block_index(), color);
    }

    pub(super) fn draw_pattern(&mut self, pattern: Block, index: usize, color: Color) {
        match color {
            Color::White => self.frame_buf[index] |= pattern,
            Color::Black => self.frame_buf[index] &= !pattern,
        };
    }
}
