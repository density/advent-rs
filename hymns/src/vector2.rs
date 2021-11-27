use std::mem;
use std::ops::{Add, AddAssign, Mul};

use num_traits::{PrimInt, Signed};

#[derive(Copy, Clone)]
pub enum Rotation {
    Right90,
    OneEighty,
    Left90,
}

#[derive(Debug, Eq, PartialEq, Hash, Default, Copy, Clone)]
pub struct Vector2<T: PrimInt + AddAssign> {
    pub x: T,
    pub y: T,
}

impl<T: PrimInt + AddAssign> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: PrimInt + Signed + AddAssign> Vector2<T> {
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

impl<T: PrimInt + AddAssign> AddAssign<Vector2<T>> for Vector2<T> {
    fn add_assign(&mut self, rhs: Vector2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: PrimInt + AddAssign> Mul<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: PrimInt + AddAssign> Add<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, rhs: Vector2<T>) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
