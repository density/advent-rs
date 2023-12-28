use std::fmt::{Display, Formatter};
use std::mem;
use std::ops::{Add, AddAssign, Mul};

use num_traits::{PrimInt, Signed};
use Direction::{Down, DownLeft, DownRight, Left, Right, Up, UpLeft, UpRight};

pub const FOUR_NEIGHBORS: [Direction; 4] = [Up, Down, Left, Right];

pub const EIGHT_NEIGHBORS: [Direction; 8] =
    [Up, UpRight, Right, DownRight, Down, DownLeft, Left, UpLeft];

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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    #[must_use]
    pub fn inverted(self) -> Self {
        match self {
            Up => Down,
            UpRight => DownLeft,
            Right => Left,
            DownRight => UpLeft,
            Down => Up,
            DownLeft => UpRight,
            Left => Right,
            UpLeft => DownRight,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Default, Copy, Clone)]
pub struct Point2<T: PrimInt + AddAssign> {
    pub x: T,
    pub y: T,
}

impl<T: PrimInt + AddAssign + Display> Display for Point2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.y, self.x)
    }
}

impl<T: PrimInt + AddAssign> Point2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub fn origin() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    pub fn shifted(&self, shift: Direction) -> Option<Point2<T>> {
        match shift {
            Up => self
                .y
                .checked_sub(&T::one())
                .map(|y| Point2 { x: self.x, y }),
            UpRight => self.y.checked_sub(&T::one()).map(|y| Point2 {
                x: self.x.add(T::one()),
                y,
            }),
            Right => Some(Point2 {
                x: self.x.add(T::one()),
                y: self.y,
            }),
            DownRight => Some(Point2 {
                x: self.x.add(T::one()),
                y: self.y.add(T::one()),
            }),
            Down => Some(Point2 {
                x: self.x,
                y: self.y.add(T::one()),
            }),
            DownLeft => self.x.checked_sub(&T::one()).map(|x| Point2 {
                x,
                y: self.y.add(T::one()),
            }),
            Left => self
                .x
                .checked_sub(&T::one())
                .map(|x| Point2 { x, y: self.y }),
            UpLeft => match self.x.checked_sub(&T::one()) {
                Some(x) => self.y.checked_sub(&T::one()).map(|y| Point2 { x, y }),
                None => None,
            },
        }
    }
}

impl<T: PrimInt + Signed + AddAssign> Point2<T> {
    pub fn rotate(&mut self, degrees: Rotation) {
        match degrees {
            Rotation::Right90 => {
                mem::swap(&mut self.x, &mut self.y);
                self.y = -self.y;
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

    pub fn all_neighbors(&self, extended: bool, include_self: bool) -> Vec<Point2<T>> {
        // capacity is 8, 9, 4 or 5 depending on arguments
        let mut capacity = if extended { 8 } else { 4 };
        if include_self {
            capacity += 1;
        }

        let mut neighbors = Vec::with_capacity(capacity);

        if extended {
            neighbors.append(&mut self.get_neighbors(&EIGHT_NEIGHBORS));
        } else {
            neighbors.append(&mut self.get_neighbors(&FOUR_NEIGHBORS));
        };

        if include_self {
            neighbors.push(*self);
        }

        neighbors
    }

    pub fn get_neighbors(&self, directions: &[Direction]) -> Vec<Point2<T>> {
        let mut neighbors = Vec::with_capacity(directions.len());
        neighbors.extend(directions.iter().filter_map(|shift| self.shifted(*shift)));
        neighbors
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
            p.all_neighbors(false, false),
            vec![p2!(0, -1), p2!(0, 1), p2!(-1, 0), p2!(1, 0),]
        );

        let p = p2!(1, 1);

        assert_eq!(
            p.all_neighbors(false, false),
            vec![p2!(1, 0), p2!(1, 2), p2!(0, 1), p2!(2, 1),]
        );

        let p = p2!(1, 1);

        assert_eq!(
            p.all_neighbors(false, true),
            vec![p2!(1, 0), p2!(1, 2), p2!(0, 1), p2!(2, 1), p2!(1, 1)]
        );

        let p = p2!(1, 1);
        assert_eq!(p.get_neighbors(&[Up, Down]), vec![p2!(1, 0), p2!(1, 2)]);
    }

    #[test]
    fn test_extended_neighbors() {
        let p = Point2::default();

        assert_eq!(
            p.all_neighbors(true, false),
            vec![
                p2!(0, -1),
                p2!(1, -1),
                p2!(1, 0),
                p2!(1, 1),
                p2!(0, 1),
                p2!(-1, 1),
                p2!(-1, 0),
                p2!(-1, -1),
            ]
        );

        let p = p2!(1, 1);

        assert_eq!(
            p.all_neighbors(true, false),
            vec![
                p2!(1, 0),
                p2!(2, 0),
                p2!(2, 1),
                p2!(2, 2),
                p2!(1, 2),
                p2!(0, 2),
                p2!(0, 1),
                p2!(0, 0),
            ]
        );
    }
}
