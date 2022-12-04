use num_traits::PrimInt;

use crate::p2;
use crate::vector2::Point2;

pub struct Grid<T: PrimInt> {
    elems: Vec<Vec<T>>,
}

impl<T: PrimInt> Grid<T> {
    pub fn new(elems: Vec<Vec<T>>) -> Self {
        Self { elems }
    }

    pub fn rows(&self) -> usize {
        self.elems.len()
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
        (0..self.rows()).flat_map(|x| (0..self.cols()).map(move |y| p2!(x, y)))
    }

    pub fn iter_points_values(&self) -> impl Iterator<Item = (Point2<usize>, T)> + '_ {
        self.iter_points().map(|p| (p, self.get_value(&p)))
    }

    pub fn get_value(&self, p: &Point2<usize>) -> T {
        self.elems[p.x][p.y]
    }

    pub fn get_value_mut(&mut self, p: &Point2<usize>) -> &mut T {
        self.elems.get_mut(p.x).unwrap().get_mut(p.y).unwrap()
    }

    pub fn set_value(&mut self, p: &Point2<usize>, val: T) {
        self.elems[p.x][p.y] = val;
    }

    pub fn neighbor_coords(&self, p: &Point2<usize>, include_diagonal: bool) -> Vec<Point2<usize>> {
        let mut neighbors = vec![];

        let mut offsets: Vec<(i8, i8)> = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
        if include_diagonal {
            offsets.extend([(-1, -1), (1, 1), (-1, 1), (1, -1)].into_iter());
        }

        for (dx, dy) in offsets {
            let is_invalid = (dx == -1 && p.x == 0)
                || (dy == -1 && p.y == 0)
                || (dx == 1 && p.x == self.rows() - 1)
                || (dy == 1 && p.y == self.cols() - 1);

            if !is_invalid {
                let new_x = if dx == -1 { p.x - 1 } else { p.x + dx as usize };

                let new_y = if dy == -1 { p.y - 1 } else { p.y + dy as usize };

                neighbors.push(p2!(new_x, new_y));
            }
        }

        neighbors
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

        assert_eq!(g.get_value(&p2!(0, 0)), 0);
        assert_eq!(g.get_value(&p2!(2, 2)), 12);

        let mut g = g;
        g.set_value(&p2!(1, 1), 999);
        assert_eq!(g.get_value(&p2!(1, 1)), 999);
    }

    #[test]
    fn test_neighbors() {
        let data = vec![
            vec![0, 1, 2, 3, 4],
            vec![5, 6, 7, 8, 9],
            vec![10, 11, 12, 13, 14],
        ];

        let g = Grid::new(data);

        // corners without diagonals
        assert_eq!(
            g.neighbor_coords(&p2!(0, 0), false),
            vec![p2!(1, 0), p2!(0, 1)]
        );
        assert_eq!(
            g.neighbor_coords(&p2!(0, 4), false),
            vec![p2!(0, 3), p2!(1, 4)]
        );
        assert_eq!(
            g.neighbor_coords(&p2!(2, 0), false),
            vec![p2!(1, 0), p2!(2, 1)]
        );
        assert_eq!(
            g.neighbor_coords(&p2!(2, 4), false),
            vec![p2!(1, 4), p2!(2, 3)]
        );

        // corners with diagonals
        assert_eq!(
            g.neighbor_coords(&p2!(0, 0), true),
            vec![p2!(1, 0), p2!(0, 1), p2!(1, 1)]
        );
        assert_eq!(
            g.neighbor_coords(&p2!(0, 4), true),
            vec![p2!(0, 3), p2!(1, 4), p2!(1, 3)]
        );
        assert_eq!(
            g.neighbor_coords(&p2!(2, 0), true),
            vec![p2!(1, 0), p2!(2, 1), p2!(1, 1)]
        );
        assert_eq!(
            g.neighbor_coords(&p2!(2, 4), true),
            vec![p2!(1, 4), p2!(2, 3), p2!(1, 3)]
        );

        // non-corner without diagonals
        assert_eq!(
            g.neighbor_coords(&p2!(1, 1), false),
            vec![p2!(0, 1), p2!(1, 0), p2!(2, 1), p2!(1, 2)]
        );

        // non-corner with diagonals
        assert_eq!(
            g.neighbor_coords(&p2!(1, 1), true),
            vec![
                p2!(0, 1),
                p2!(1, 0),
                p2!(2, 1),
                p2!(1, 2),
                p2!(0, 0),
                p2!(2, 2),
                p2!(0, 2),
                p2!(2, 0)
            ]
        );
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
                p2!(0, 1),
                p2!(0, 2),
                p2!(1, 0),
                p2!(1, 1),
                p2!(1, 2),
                p2!(2, 0),
                p2!(2, 1),
                p2!(2, 2),
            ]
        );
        assert_eq!(
            g.iter_points_values().collect::<Vec<_>>(),
            vec![
                (p2!(0, 0), 0),
                (p2!(0, 1), 1),
                (p2!(0, 2), 2),
                (p2!(1, 0), 3),
                (p2!(1, 1), 4),
                (p2!(1, 2), 5),
                (p2!(2, 0), 6),
                (p2!(2, 1), 7),
                (p2!(2, 2), 8)
            ]
        );

        assert_eq!(
            g.iter_values().cloned().collect::<Vec<i32>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8]
        );
    }
}
