use std::ops::{Add, AddAssign};

use num_traits::PrimInt;

#[macro_export]
macro_rules! p3 {
    ($x:expr, $y:expr, $z:expr) => {
        Point3::new($x, $y, $z)
    };
}

#[derive(Debug, Eq, PartialEq, Hash, Default, Copy, Clone)]
pub struct Point3<T: PrimInt + AddAssign> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: PrimInt + AddAssign> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: PrimInt + AddAssign> Add<Point3<T>> for Point3<T> {
    type Output = Point3<T>;

    fn add(self, rhs: Point3<T>) -> Self::Output {
        p3!(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: PrimInt + AddAssign> Point3<T> {
    pub fn neighbors(&self, extended: bool) -> Vec<Point3<T>> {
        let mut result = Vec::with_capacity(if extended { 26 } else { 6 });

        let pos_one = T::one();
        let neg_one = T::zero() - T::one();

        if extended {
            let vals = [neg_one, T::zero(), pos_one];

            for dx in vals {
                for dy in vals {
                    for dz in vals {
                        if [dx, dy, dz] != [T::zero(); 3] {
                            result.push(*self + p3!(dx, dy, dz));
                        }
                    }
                }
            }
        } else {
            for offset in 0..3 {
                let mut deltas = [T::zero(), T::zero(), T::zero()];

                deltas[offset] = pos_one;
                result.push(*self + p3!(deltas[0], deltas[1], deltas[2]));

                deltas[offset] = neg_one;
                result.push(*self + p3!(deltas[0], deltas[1], deltas[2]));
            }
        }

        result
    }
}

impl<T: PrimInt + AddAssign> From<(T, T, T)> for Point3<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        p3!(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        assert_eq!(p3!(10, 20, 30), Point3::new(10, 20, 30));
    }

    #[test]
    fn test_neighbors() {
        let p: Point3<isize> = Point3::default();

        assert_eq!(
            p.neighbors(false),
            vec![
                p3!(1, 0, 0),
                p3!(-1, 0, 0),
                p3!(0, 1, 0),
                p3!(0, -1, 0),
                p3!(0, 0, 1),
                p3!(0, 0, -1)
            ]
        );

        let p = p3!(1, 1, 1);

        assert_eq!(
            p.neighbors(false),
            vec![
                p3!(2, 1, 1),
                p3!(0, 1, 1),
                p3!(1, 2, 1),
                p3!(1, 0, 1),
                p3!(1, 1, 2),
                p3!(1, 1, 0)
            ]
        );
    }

    #[test]
    fn test_extended_neighbors() {
        let p: Point3<isize> = Point3::default();

        assert_eq!(
            p.neighbors(true),
            vec![
                p3!(-1, -1, -1),
                p3!(-1, -1, 0),
                p3!(-1, -1, 1),
                p3!(-1, 0, -1),
                p3!(-1, 0, 0),
                p3!(-1, 0, 1),
                p3!(-1, 1, -1),
                p3!(-1, 1, 0),
                p3!(-1, 1, 1),
                p3!(0, -1, -1),
                p3!(0, -1, 0),
                p3!(0, -1, 1),
                p3!(0, 0, -1),
                p3!(0, 0, 1),
                p3!(0, 1, -1),
                p3!(0, 1, 0),
                p3!(0, 1, 1),
                p3!(1, -1, -1),
                p3!(1, -1, 0),
                p3!(1, -1, 1),
                p3!(1, 0, -1),
                p3!(1, 0, 0),
                p3!(1, 0, 1),
                p3!(1, 1, -1),
                p3!(1, 1, 0),
                p3!(1, 1, 1),
            ]
        );

        let p = p3!(1, 1, 1);

        assert_eq!(
            p.neighbors(true),
            vec![
                p3!(0, 0, 0),
                p3!(0, 0, 1),
                p3!(0, 0, 2),
                p3!(0, 1, 0),
                p3!(0, 1, 1),
                p3!(0, 1, 2),
                p3!(0, 2, 0),
                p3!(0, 2, 1),
                p3!(0, 2, 2),
                p3!(1, 0, 0),
                p3!(1, 0, 1),
                p3!(1, 0, 2),
                p3!(1, 1, 0),
                p3!(1, 1, 2),
                p3!(1, 2, 0),
                p3!(1, 2, 1),
                p3!(1, 2, 2),
                p3!(2, 0, 0),
                p3!(2, 0, 1),
                p3!(2, 0, 2),
                p3!(2, 1, 0),
                p3!(2, 1, 1),
                p3!(2, 1, 2),
                p3!(2, 2, 0),
                p3!(2, 2, 1),
                p3!(2, 2, 2),
            ]
        );
    }
}
