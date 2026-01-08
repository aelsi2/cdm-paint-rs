use core::ops::{Add, AddAssign, Sub, SubAssign};

pub const PIXELS_PER_BLOCK: usize = 16;
pub const SCREEN_WIDTH: usize = 32;
pub const SCREEN_HEIGHT: usize = 32;
pub const BLOCKS_PER_ROW: usize = SCREEN_WIDTH / PIXELS_PER_BLOCK;
pub const FRAMEBUF_SIZE: usize = SCREEN_HEIGHT * BLOCKS_PER_ROW;

pub type Block = u16;
pub type FrameBuf = [Block; FRAMEBUF_SIZE];

#[derive(Clone, Copy, Default, Hash, Eq, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Point(i16);

#[derive(Clone, Copy, Default, Hash, Eq, PartialEq)]
#[repr(u8)]
pub enum Color {
    #[default]
    Black = 0,
    White = 1,
}

#[derive(Clone, Copy, Default, Hash, Eq, PartialEq)]
#[repr(u8)]
pub enum Tool {
    #[default]
    Pixel = 0,
    Line = 1,
    Rect = 2,
    Ellipse = 3,
    FloodFill = 4,
    Clear = 5,
}

#[derive(Clone, Copy, Default, Hash, Eq, PartialEq)]
#[repr(u8)]
pub enum Fill {
    #[default]
    Off = 0,
    On = 1,
}

#[derive(Clone, Copy, Default, Hash, Eq, PartialEq)]
pub struct Shape {
    pub tool: Tool,
    pub fill: Fill,
    pub color: Color,
    pub pt1: Point,
    pub pt2: Point,
}

impl Point {
    pub const ZERO: Self = Point(0);
    pub const ONE: Self = Point(1);

    pub const fn new(x: i16, y: i16) -> Self {
        Point(x + y * SCREEN_WIDTH as i16)
    }

    pub const fn x(self) -> i16 {
        let Point(value) = self;
        value % SCREEN_WIDTH as i16
    }

    pub const fn y(self) -> i16 {
        let Point(value) = self;
        value / SCREEN_WIDTH as i16
    }

    pub const fn block_index(self) -> usize {
        let Point(value) = self;
        value as usize / PIXELS_PER_BLOCK
    }

    pub const fn pixel_index(self) -> usize {
        let Point(value) = self;
        value as usize % PIXELS_PER_BLOCK
    }
}

impl From<i16> for Point {
    fn from(value: i16) -> Self {
        Point(value)
    }
}

impl Into<i16> for Point {
    fn into(self) -> i16 {
        let Point(value) = self;
        value
    }
}

impl Into<(i16, i16)> for Point {
    fn into(self) -> (i16, i16) {
        (self.x(), self.y())
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        let Point(lhs_val) = self;
        let Point(rhs_val) = rhs;
        Point(lhs_val + rhs_val)
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        let Point(lhs_val) = *self;
        let Point(rhs_val) = rhs;
        *self = Point(lhs_val + rhs_val)
    }
}

impl SubAssign<Point> for Point {
    fn sub_assign(&mut self, rhs: Point) {
        let Point(lhs_val) = *self;
        let Point(rhs_val) = rhs;
        *self = Point(lhs_val - rhs_val)
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        let Point(lhs_val) = self;
        let Point(rhs_val) = rhs;
        Point(lhs_val - rhs_val)
    }
}

impl Shape {
    pub fn new(tool: Tool, fill: Fill, color: Color, pt1: Point, pt2: Point) -> Self {
        Shape {
            tool,
            fill,
            color,
            pt1,
            pt2
        }
    }
}
