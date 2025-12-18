use core::ops::{Add, AddAssign, Sub, SubAssign};

const SCREEN_WIDTH: i16 = 32;
const SCREEN_HEIGHT: i16 = 32;

#[derive(Clone, Copy, Default, Hash, Eq, PartialEq)]
#[repr(transparent)]
pub struct Point(i16);

impl Point {
    pub const fn zero() -> Self {
        Point(0)
    }

    pub const fn new(x: i16, y: i16) -> Self {
        Point(x + y * SCREEN_WIDTH)
    }

    pub const fn x(self) -> i16 {
        let Point(value) = self;
        value % SCREEN_WIDTH
    }
    
    pub const fn y(self) -> i16 {
        let Point(value) = self;
        value / SCREEN_WIDTH
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

