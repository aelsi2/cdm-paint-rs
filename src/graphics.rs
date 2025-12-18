use core::ops::{Add, AddAssign, Sub, SubAssign};

pub const PIXELS_PER_BLOCK: usize = 16;
pub const SCREEN_WIDTH: usize = 32;
pub const SCREEN_HEIGHT: usize = 32;
pub const FRAMEBUF_SIZE: usize = SCREEN_HEIGHT * SCREEN_WIDTH / PIXELS_PER_BLOCK;

pub type Block = u16;
pub type FrameBuf = [Block; FRAMEBUF_SIZE];

#[derive(Clone, Copy, Default, Hash, Eq, PartialEq)]
#[repr(transparent)]
pub struct Point(pub(crate) i16);

#[repr(u8)]
pub enum Color {
    Black = 0,
    White = 1,
}

#[repr(u8)]
pub enum Tool {
    Pixel = 0,
    Line = 1,
    Rect = 2,
    Ellipse = 3,
    FloodFill = 4,
    Clear = 5,
}

impl Point {
    pub const fn zero() -> Self {
        Point(0)
    }

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
