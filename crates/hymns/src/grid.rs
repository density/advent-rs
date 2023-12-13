use crate::p2;
use crate::vector2::Point2;
use itertools::Itertools;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Clone)]
pub struct Grid<T> {
    elems: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    #[must_use]
    pub fn new(elems: Vec<Vec<T>>) -> Self {
        debug_assert!(elems.iter().skip(1).all(|row| row.len() == elems[0].len()));
        Self { elems }
    }

    #[must_use]
    pub fn rows(&self) -> usize {
        self.elems.len()
    }

    #[must_use]
    pub fn contains(&self, point: Point2<usize>) -> bool {
        point.y < self.rows() && point.x < self.cols()
    }

    #[must_use]
    pub fn cols(&self) -> usize {
        self.elems.first().map_or(0, Vec::len)
    }

    #[must_use]
    pub fn row(&self, row: usize) -> Vec<&T> {
        self.elems[row].iter().collect_vec()
    }

    #[must_use]
    pub fn col(&self, col: usize) -> Vec<&T> {
        self.elems.iter().map(|row| &row[col]).collect_vec()
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
        self.elems.iter().map(Vec::as_slice)
    }

    pub fn iter_rows_mut(&mut self) -> impl Iterator<Item = &mut Vec<T>> {
        self.elems.iter_mut()
    }

    pub fn into_iter_rows(self) -> impl Iterator<Item = Vec<T>> {
        self.elems.into_iter()
    }

    pub fn iter_values(&self) -> impl Iterator<Item = &T> {
        self.elems.iter().flatten()
    }

    pub fn into_iter_values(self) -> impl Iterator<Item = T> {
        self.elems.into_iter().flatten()
    }

    pub fn iter_values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.elems.iter_mut().flatten()
    }

    pub fn iter_cols(&self) -> impl Iterator<Item = Vec<&T>> {
        (0..self.cols()).map(move |c| {
            (0..self.rows())
                .map(move |r| &self.elems[r][c])
                .collect_vec()
        })
    }

    pub fn iter_points(&self) -> impl Iterator<Item = Point2<usize>> + '_ {
        (0..self.rows()).flat_map(|y| (0..self.cols()).map(move |x| p2!(x, y)))
    }

    pub fn iter_points_values(&self) -> impl Iterator<Item = (Point2<usize>, &T)> + '_ {
        self.iter_points().map(|p| (p, &self[p]))
    }

    pub fn into_iter_points_values(self) -> impl Iterator<Item = (Point2<usize>, T)> {
        let cols = self.cols();

        self.into_iter_values().enumerate().map(move |(i, val)| {
            let y = i / cols;
            let x = i % cols;
            (p2!(x, y), val)
        })
    }

    #[must_use]
    pub fn get_value(&self, p: &Point2<usize>) -> Option<&T> {
        self.elems.get(p.y).and_then(|row| row.get(p.x))
    }

    pub fn get_value_mut(&mut self, p: &Point2<usize>) -> Option<&mut T> {
        self.elems.get_mut(p.y).and_then(|row| row.get_mut(p.x))
    }

    pub fn set_value(&mut self, p: &Point2<usize>, val: T) {
        self.elems[p.y][p.x] = val;
    }

    #[must_use]
    pub fn neighbor_coords(&self, p: &Point2<usize>, extended: bool) -> Vec<Point2<usize>> {
        let mut neighbors = p.unsigned_neighbors(extended, false);
        neighbors.retain(|p| p.y < self.rows() && p.x < self.cols());
        neighbors
    }

    pub fn iter_neighbors(
        &self,
        point2: &Point2<usize>,
        extended: bool,
    ) -> impl Iterator<Item = (Point2<usize>, &T)> + '_ {
        self.neighbor_coords(point2, extended)
            .into_iter()
            .map(move |p| (p, &self[p]))
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.iter_rows() {
            for col in row {
                write!(f, "{col}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T> Debug for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.iter_rows() {
            for col in row {
                write!(f, "{col:?}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl FromStr for Grid<char> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            s.lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec(),
        ))
    }
}

impl FromStr for Grid<u8> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            s.lines()
                .map(|line| line.as_bytes().iter().copied().collect_vec())
                .collect_vec(),
        ))
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
    type GridRow = Vec<i32>;
    type GridVec = Vec<GridRow>;
    type Points = Vec<Point2<usize>>;

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

        assert!(g.iter_points().all(|p| g.contains(p)));
        assert!(!g.contains(p2!(0, 3)));
        assert!(!g.contains(p2!(5, 0)));

        assert_eq!(g[p2!(0, 0)], 0);
        assert_eq!(g[p2!(2, 2)], 12);

        assert!(g.iter_points().all(|p| g.get_value(&p) == Some(&g[p])));
        assert_eq!(g.get_value(&p2!(0, 3)), None);

        let mut g = g;
        g.set_value(&p2!(1, 1), 999);
        assert_eq!(g.get_value(&p2!(1, 1)), Some(&999));

        assert_eq!(g.row(1), vec![&5, &999, &7, &8, &9]);
        assert_eq!(g.col(1), vec![&1, &999, &11]);
    }

    #[test]
    fn test_iterators() {
        let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
        let data_flattened = data.iter().flatten().copied().collect::<GridRow>();
        let g = Grid::new(data.clone());

        assert_eq!(
            g.iter_rows().map(<[i32]>::to_vec).collect::<GridVec>(),
            data
        );
        assert_eq!(g.clone().into_iter_rows().collect::<GridVec>(), data);

        assert_eq!(
            g.iter_values().copied().collect::<GridRow>(),
            data_flattened
        );
        assert_eq!(
            g.clone().into_iter_values().collect::<GridRow>(),
            data_flattened
        );

        assert_eq!(
            g.iter_points().collect::<Points>(),
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
            g.clone().into_iter_points_values().collect::<Vec<_>>(),
            vec![
                (p2!(0, 0), 0),
                (p2!(1, 0), 1),
                (p2!(2, 0), 2),
                (p2!(0, 1), 3),
                (p2!(1, 1), 4),
                (p2!(2, 1), 5),
                (p2!(0, 2), 6),
                (p2!(1, 2), 7),
                (p2!(2, 2), 8),
            ]
        );

        assert_eq!(
            g.iter_values().copied().collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8]
        );

        assert_eq!(
            g.iter_cols().collect::<Vec<_>>(),
            vec![vec![&0, &3, &6], vec![&1, &4, &7], vec![&2, &5, &8],]
        );
    }

    #[test]
    fn test_iter_mut() {
        let data = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
        let g = Grid::new(data.clone());

        let mut g_clone = g.clone();
        for cell in g_clone.iter_values_mut() {
            *cell += 1;
        }
        assert_eq!(
            g_clone.iter_values().copied().collect::<Vec<_>>(),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        );

        let mut g_clone = g.clone();
        for row in g_clone.iter_rows_mut() {
            for cell in row {
                *cell += 1;
            }
        }
        assert_eq!(
            g_clone.iter_values().copied().collect::<Vec<_>>(),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
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

        assert_eq!(
            g.iter_neighbors(&p2!(1, 1), false).collect::<Vec<_>>(),
            vec![
                (p2!(0, 1), &3),
                (p2!(2, 1), &5),
                (p2!(1, 0), &1),
                (p2!(1, 2), &7),
            ]
        );
        assert_eq!(
            g.iter_neighbors(&p2!(1, 1), true).collect::<Vec<_>>(),
            vec![
                (p2!(0, 1), &3),
                (p2!(2, 1), &5),
                (p2!(1, 0), &1),
                (p2!(1, 2), &7),
                (p2!(0, 0), &0),
                (p2!(0, 2), &6),
                (p2!(2, 0), &2),
                (p2!(2, 2), &8),
            ]
        );
    }

    #[test]
    fn test_display_debug() {
        let g = Grid::new(vec![vec!["0", "1"], vec!["2", "3"]]);

        assert_eq!(format!("{g}"), "01\n23\n");

        assert_eq!(
            format!("{g:?}"),
            concat!(r#""0""1""#, "\n", r#""2""3""#, "\n")
        );
    }

    #[test]
    fn test_from_str() {
        let grid: Grid<char> = "01\n23\n".parse().unwrap();
        assert_eq!(grid.elems, vec![vec!['0', '1'], vec!['2', '3']]);

        let grid: Grid<u8> = "01\n23\n".parse().unwrap();
        assert_eq!(grid.elems, vec![vec![b'0', b'1'], vec![b'2', b'3']]);
    }
}
