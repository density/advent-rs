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

impl<T: PrimInt + AddAssign> Point2<T> {
    pub fn manhattan_dist(&self, other: &Point2<T>) -> T {
        self.x.max(other.x) - self.x.min(other.x) + self.y.max(other.y) - self.y.min(other.y)
    }

    pub fn neighbors(&self, extended: bool, include_self: bool) -> Vec<Point2<T>> {
        let mut neighbors = Vec::with_capacity(if extended { 8 } else { 4 });

        neighbors.append(&mut vec![
            Point2::new(self.x - T::one(), self.y),
            Point2::new(self.x + T::one(), self.y),
            Point2::new(self.x, self.y - T::one()),
            Point2::new(self.x, self.y + T::one()),
        ]);

        if extended {
            neighbors.append(&mut vec![
                Point2::new(self.x - T::one(), self.y - T::one()),
                Point2::new(self.x - T::one(), self.y + T::one()),
                Point2::new(self.x + T::one(), self.y - T::one()),
                Point2::new(self.x + T::one(), self.y + T::one()),
            ]);
        }

        if include_self {
            neighbors.push(*self);
        }

        neighbors
    }

    pub fn unsigned_neighbors(&self, extended: bool, include_self: bool) -> Vec<Point2<T>> {
        let x_minus_one = self.x.checked_sub(&T::one());
        let y_minus_one = self.y.checked_sub(&T::one());
        let x_plus_one = Some(self.x + T::one());
        let y_plus_one = Some(self.y + T::one());

        let mut neighbors = vec![
            (x_minus_one, Some(self.y)),
            ((x_plus_one), Some(self.y)),
            (Some(self.x), y_minus_one),
            (Some(self.x), y_plus_one),
        ];

        if extended {
            neighbors.extend(vec![
                (x_minus_one, y_minus_one),
                (x_minus_one, y_plus_one),
                (x_plus_one, y_minus_one),
                (x_plus_one, y_plus_one),
            ])
        }

        if include_self {
            neighbors.push((Some(self.x), Some(self.y)));
        }

        neighbors
            .into_iter()
            .filter_map(|(x, y)| match (x, y) {
                (Some(x), Some(y)) => Some(p2!(x, y)),
                _ => None,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        assert_eq!(p2!(10, 20), Point2::new(10, 20));
    }

    #[test]
    fn test_manhattan() {
        let p1: Point2<i64> = Point2::default();

        assert_eq!(p1.manhattan_dist(&Point2::default()), 0);
        assert_eq!(p2!(3, 5).manhattan_dist(&Point2::default()), 8);
        assert_eq!(p2!(-3, -5).manhattan_dist(&Point2::default()), 8);
    }

    #[test]
    fn test_neighbors() {
        let p = Point2::default();

        assert_eq!(
            p.neighbors(false, false),
            vec![p2!(-1, 0), p2!(1, 0), p2!(0, -1), p2!(0, 1),]
        );

        let p = p2!(1, 1);

        assert_eq!(
            p.neighbors(false, false),
            vec![p2!(0, 1), p2!(2, 1), p2!(1, 0), p2!(1, 2)]
        );

        let p = p2!(1, 1);

        assert_eq!(
            p.neighbors(false, true),
            vec![p2!(0, 1), p2!(2, 1), p2!(1, 0), p2!(1, 2), p2!(1, 1)]
        );
    }

    #[test]
    fn test_unsigned_neighbors() {
        let p: Point2<usize> = Point2::default();

        assert_eq!(
            p.unsigned_neighbors(false, false),
            vec![p2!(1, 0), p2!(0, 1),]
        );

        let p = p2!(1, 1);

        assert_eq!(
            p.unsigned_neighbors(false, false),
            p.neighbors(false, false)
        );
    }

    #[test]
    fn test_extended_neighbors() {
        let p = Point2::default();

        assert_eq!(
            p.neighbors(true, false),
            vec![
                p2!(-1, 0),
                p2!(1, 0),
                p2!(0, -1),
                p2!(0, 1),
                p2!(-1, -1),
                p2!(-1, 1),
                p2!(1, -1),
                p2!(1, 1),
            ]
        );

        let p = p2!(1, 1);

        assert_eq!(
            p.neighbors(true, false),
            vec![
                p2!(0, 1),
                p2!(2, 1),
                p2!(1, 0),
                p2!(1, 2),
                p2!(0, 0),
                p2!(0, 2),
                p2!(2, 0),
                p2!(2, 2),
            ]
        )
    }

    #[test]
    fn test_unsigned_extended_neighbors() {
        let p: Point2<usize> = Point2::default();

        assert_eq!(
            p.unsigned_neighbors(true, false),
            vec![p2!(1, 0), p2!(0, 1), p2!(1, 1),]
        );

        let p: Point2<usize> = p2!(1, 1);

        assert_eq!(p.unsigned_neighbors(true, false), p.neighbors(true, false));

        let p: Point2<usize> = p2!(1, 1);

        assert_eq!(p.unsigned_neighbors(true, true), p.neighbors(true, true));
    }
}
