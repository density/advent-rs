use std::ops::{Index, IndexMut};
use crate::p2;
use crate::vector2::Point2;

pub struct Grid<T> {
    elems: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(elems: Vec<Vec<T>>) -> Self {
        Self { elems }
    }

    pub fn rows(&self) -> usize {
        self.elems.len()
    }

    pub fn contains(&self, point: Point2<usize>) -> bool {
        point.y < self.rows() && point.x < self.cols()
    }

    pub fn cols(&self) -> usize {
        self.elems.first().map_or(0, |row| row.len())
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &Vec<T>> {
        self.elems.iter()
    }

    pub fn iter_rows_mut(&mut self) -> impl Iterator<Item = &mut Vec<T>> {
        self.elems.iter_mut()
    }

    pub fn iter_values(&self) -> impl Iterator<Item = &T> {
        self.elems.iter().flatten()
    }

    pub fn iter_values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.elems.iter_mut().flatten()
    }

    pub fn iter_points(&self) -> impl Iterator<Item = Point2<usize>> + '_ {
        (0..self.rows()).flat_map(|y| (0..self.cols()).map(move |x| p2!(x, y)))
    }

    pub fn iter_points_values(&self) -> impl Iterator<Item = (Point2<usize>, &T)> + '_ {
        self.iter_points().map(|p| (p, self.get_value(&p)))
    }

    pub fn get_value(&self, p: &Point2<usize>) -> Option<&T> {
        self.elems.get(p.y).and_then(|row| row.get(p.x))
    }

    pub fn get_value_mut(&mut self, p: &Point2<usize>) -> Option<&mut T> {
        self.elems.get_mut(p.y).and_then(|row| row.get_mut(p.x))
    }

    pub fn set_value(&mut self, p: &Point2<usize>, val: T) {
        self.elems[p.y][p.x] = val;
    }

    pub fn neighbor_coords(&self, p: &Point2<usize>, extended: bool) -> Vec<Point2<usize>> {
        let mut neighbors = p.unsigned_neighbors(extended, false);
        neighbors.retain(|p| p.y < self.rows() && p.x < self.cols());
        neighbors
    }
}

impl<T> Index<Point2<usize>> for Grid<T> {
    type Output = T;

    fn index(&self, p: Point2<usize>) -> &Self::Output {
        &self.elems[p.y][p.x]
    }
}

impl<T> IndexMut<Point2<usize>> for Grid<T> {
    fn index_mut(&mut self, p: Point2<usize>) -> &mut Self::Output {
        &mut self.elems[p.y][p.x]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let data = vec![
            vec![0, 1, 2, 3, 4],
            vec![5, 6, 7, 8, 9],
            vec![10, 11, 12, 13, 14],
        ];

        let g = Grid::new(data);

        assert_eq!(g.rows(), 3);
        assert_eq!(g.cols(), 5);

        assert_eq!(g.get_value(&p2!(0, 0)), &0);
        assert_eq!(g.get_value(&p2!(2, 2)), &12);

        let mut g = g;
        g.set_value(&p2!(1, 1), 999);
        assert_eq!(g.get_value(&p2!(1, 1)), &999);
    }

    #[test]
    fn test_iterators() {
        let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
        let g = Grid::new(data.clone());

        assert_eq!(g.iter_rows().cloned().collect::<Vec<Vec<i32>>>(), data);
        for (n1, n2) in g.iter_values().zip(data.iter().flatten()) {
            assert_eq!(n1, n2);
        }

        assert_eq!(
            g.iter_points().collect::<Vec<_>>(),
            vec![
                p2!(0, 0),
                p2!(1, 0),
                p2!(2, 0),
                p2!(0, 1),
                p2!(1, 1),
                p2!(2, 1),
                p2!(0, 2),
                p2!(1, 2),
                p2!(2, 2),
            ]
        );
        assert_eq!(
            g.iter_points_values().collect::<Vec<_>>(),
            vec![
                (p2!(0, 0), &0),
                (p2!(1, 0), &1),
                (p2!(2, 0), &2),
                (p2!(0, 1), &3),
                (p2!(1, 1), &4),
                (p2!(2, 1), &5),
                (p2!(0, 2), &6),
                (p2!(1, 2), &7),
                (p2!(2, 2), &8),
            ]
        );

        assert_eq!(
            g.iter_values().cloned().collect::<Vec<i32>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8]
        );
    }

    #[test]
    fn test_neighbors() {
        let g = Grid::new(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]);

        assert_eq!(
            g.neighbor_coords(&p2!(0, 0), false),
            vec![p2!(1, 0), p2!(0, 1)]
        );
        assert_eq!(
            g.neighbor_coords(&p2!(0, 0), true),
            vec![p2!(1, 0), p2!(0, 1), p2!(1, 1)]
        );
        assert_eq!(
            g.neighbor_coords(&p2!(1, 1), false),
            vec![p2!(0, 1), p2!(2, 1), p2!(1, 0), p2!(1, 2)]
        );
        assert_eq!(
            g.neighbor_coords(&p2!(1, 1), true),
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
        );
    }
}
