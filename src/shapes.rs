use crate::drawing::DrawingCtx;
use crate::graphics::Color;
use crate::graphics::Fill;
use crate::graphics::Point;

pub trait Shape: Send {
    fn draw(&self, ctx: &mut DrawingCtx);
}

pub struct Rect {
    pt1: Point,
    pt2: Point,
    color: Color,
    fill: Fill,
}

impl Rect {
    pub const fn new(pt1: Point, pt2: Point, color: Color, fill: Fill) -> Self {
        Self {
            pt1,
            pt2,
            color,
            fill,
        }
    }
}

impl Shape for Rect {
    fn draw(&self, ctx: &mut DrawingCtx) {
        match self.fill {
            Fill::Off => ctx.draw_outline_rect(self.pt1, self.pt2, self.color),
            Fill::On => ctx.draw_filled_rect(self.pt1, self.pt2, self.color),
        }
    }
}

pub struct Ellipse {
    pt1: Point,
    pt2: Point,
    color: Color,
    fill: Fill,
}

impl Ellipse {
    pub const fn new(pt1: Point, pt2: Point, color: Color, fill: Fill) -> Self {
        Self {
            pt1,
            pt2,
            color,
            fill,
        }
    }
}

impl Shape for Ellipse {
    fn draw(&self, ctx: &mut DrawingCtx) {
        match self.fill {
            Fill::Off => ctx.draw_outline_ellipse(self.pt1, self.pt2, self.color),
            Fill::On => ctx.draw_filled_ellipse(self.pt1, self.pt2, self.color),
        }
    }
}

pub struct Line {
    pt1: Point,
    pt2: Point,
    color: Color,
}

impl Line {
    pub const fn new(pt1: Point, pt2: Point, color: Color) -> Self {
        Self { pt1, pt2, color }
    }
}

impl Shape for Line {
    fn draw(&self, ctx: &mut DrawingCtx) {
        ctx.draw_line(self.pt1, self.pt2, self.color)
    }
}

pub struct Pixel {
    point: Point,
    color: Color,
}

impl Pixel {
    pub const fn new(point: Point, color: Color) -> Self {
        Self { point, color }
    }
}

impl Shape for Pixel {
    fn draw(&self, ctx: &mut DrawingCtx) {
        ctx.draw_pixel(self.point, self.color)
    }
}

pub struct FloodFill {
    point: Point,
    color: Color,
}

impl FloodFill {
    pub const fn new(point: Point, color: Color) -> Self {
        Self { point, color }
    }
}

impl Shape for FloodFill {
    fn draw(&self, ctx: &mut DrawingCtx) {
        ctx.flood_fill(self.point, self.color)
    }
}

pub struct Clear {
    color: Color,
}

impl Clear {
    pub const fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Shape for Clear {
    fn draw(&self, ctx: &mut DrawingCtx) {
        ctx.clear(self.color)
    }
}
