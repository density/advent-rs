use num_traits::{PrimInt, Signed};

use std::mem;
use std::ops::{Add, AddAssign, Mul};

#[derive(Copy, Clone)]
pub enum Rotation {
    Right90,
    OneEighty,
    Left90,
}

#[derive(Debug, Eq, PartialEq, Hash, Default, Copy, Clone)]
pub struct Vec2D<T: PrimInt + Signed + AddAssign> {
    pub x: T,
    pub y: T,
}

impl<T: PrimInt + Signed + AddAssign> Vec2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

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

    pub fn move_by(&mut self, delta: Vec2D<T>) {
        *self += delta;
    }
}

impl<T: PrimInt + Signed + AddAssign> AddAssign<Vec2D<T>> for Vec2D<T> {
    fn add_assign(&mut self, rhs: Vec2D<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: PrimInt + Signed + AddAssign> Mul<T> for Vec2D<T> {
    type Output = Vec2D<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2D::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: PrimInt + Signed + AddAssign> Add<Vec2D<T>> for Vec2D<T> {
    type Output = Vec2D<T>;

    fn add(self, rhs: Vec2D<T>) -> Self::Output {
        Vec2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}
