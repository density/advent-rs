use std::mem;
use std::ops::{Add, AddAssign, Mul};

use num_traits::{PrimInt, Signed};

#[macro_export]
macro_rules! p2 {
    ($x:expr, $y:expr) => {
        Point2::new($x, $y)
    };
}

#[derive(Copy, Clone)]
pub enum Rotation {
    Right90,
    OneEighty,
    Left90,
}

#[derive(Debug, Eq, PartialEq, Hash, Default, Copy, Clone)]
pub struct Point2<T: PrimInt + AddAssign> {
    pub x: T,
    pub y: T,
}

impl<T: PrimInt + AddAssign> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: PrimInt + Signed + AddAssign> Point2<T> {
    pub fn rotate(&mut self, degrees: Rotation) {
        match degrees {
            Rotation::Right90 => {
                mem::swap(&mut self.x, &mut self.y);
                self.y = -self.y
            }
            Rotation::OneEighty => {
                self.x = -self.x;
                self.y = -self.y;
            }
            Rotation::Left90 => {
                mem::swap(&mut self.x, &mut self.y);
                self.x = -self.x;
            }
        }
    }
}

impl<T: PrimInt + AddAssign> AddAssign<Point2<T>> for Point2<T> {
    fn add_assign(&mut self, rhs: Point2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: PrimInt + AddAssign> Mul<T> for Point2<T> {
    type Output = Point2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Point2::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: PrimInt + AddAssign> Add<Point2<T>> for Point2<T> {
    type Output = Point2<T>;

    fn add(self, rhs: Point2<T>) -> Self::Output {
        Point2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        assert_eq!(p2!(10, 20), Point2::new(10, 20));
    }
}
