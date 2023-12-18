use std::ops::{AddAssign, Div};

use num_traits::{PrimInt, ToPrimitive};

use crate::vector2::Point2;

pub fn area_of_polygon<T>(vertices: &[Point2<T>]) -> usize
where
    T: PrimInt + AddAssign + ToPrimitive + Div,
{
    let mut area = T::zero();

    for i in 0..vertices.len() {
        let p = vertices[i];
        let q = vertices[(i + 1) % vertices.len()];

        area += (p.x * q.y) - (q.x * p.y);
    }

    area = area / T::from(2).unwrap();

    area.to_usize().unwrap()
}
