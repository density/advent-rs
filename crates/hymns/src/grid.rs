use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use itertools::Itertools;

use crate::p2;
use crate::vector2::{Direction, Point2};

pub type GPoint = Point2<usize>;

#[derive(Eq, PartialEq, Clone)]
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
    pub fn contains(&self, point: GPoint) -> bool {
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

    pub fn iter_points(&self) -> impl Iterator<Item = GPoint> + '_ {
        (0..self.rows()).flat_map(|y| (0..self.cols()).map(move |x| p2!(x, y)))
    }

    pub fn iter_points_values(&self) -> impl Iterator<Item = (GPoint, &T)> + '_ {
        self.iter_points().map(|p| (p, &self[p]))
    }

    pub fn into_iter_points_values(self) -> impl Iterator<Item = (GPoint, T)> {
        let cols = self.cols();

        self.into_iter_values().enumerate().map(move |(i, val)| {
            let y = i / cols;
            let x = i % cols;
            (p2!(x, y), val)
        })
    }

    #[must_use]
    pub fn get_value(&self, p: &GPoint) -> Option<&T> {
        self.elems.get(p.y).and_then(|row| row.get(p.x))
    }

    pub fn get_value_mut(&mut self, p: &GPoint) -> Option<&mut T> {
        self.elems.get_mut(p.y).and_then(|row| row.get_mut(p.x))
    }

    pub fn set_value(&mut self, p: &GPoint, val: T) {
        self.elems[p.y][p.x] = val;
    }

    #[must_use]
    pub fn all_neighbors(&self, p: &GPoint, extended: bool) -> Vec<GPoint> {
        let mut neighbors = p.all_neighbors(extended, false);
        neighbors.retain(|p| p.y < self.rows() && p.x < self.cols());
        neighbors
    }

    pub fn iter_all_neighbors(
        &self,
        point2: &GPoint,
        extended: bool,
    ) -> impl Iterator<Item = (GPoint, &T)> + '_ {
        self.all_neighbors(point2, extended)
            .into_iter()
            .map(move |p| (p, &self[p]))
    }

    #[must_use]
    pub fn get_neighbors(&self, p: &GPoint, directions: &[Direction]) -> Vec<GPoint> {
        let mut neighbors = Vec::with_capacity(directions.len());

        neighbors.extend(directions.iter().filter_map(|shift| {
            p.shifted(*shift)
                .filter(|p| p.y < self.rows() && p.x < self.cols())
        }));

        neighbors
    }

    #[must_use]
    pub fn get_neighbor(&self, p: &GPoint, direction: Direction) -> Option<GPoint> {
        self.get_neighbors(p, &[direction]).first().copied()
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

impl<T> FromStr for Grid<T>
where
    T: From<char>,
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            s.lines()
                .map(|line| line.chars().map(|c| T::from(c)).collect_vec())
                .collect_vec(),
        ))
    }
}

impl<T> TryFrom<&str> for Grid<T>
where
    T: TryFrom<char>,
{
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.lines()
            .map(|line| line.chars().map(T::try_from).try_collect().map_err(|_| ()))
            .try_collect()
            .map(Self::new)
    }
}

impl<T> Hash for Grid<T>
where
    T: Hash + Eq,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        for row in self.iter_rows() {
            Hash::hash_slice(row, state);
        }
    }
}

impl<T, U> Index<U> for Grid<T>
where
    U: Borrow<GPoint>,
{
    type Output = T;

    fn index(&self, p: U) -> &Self::Output {
        let p = p.borrow();
        &self.elems[p.y][p.x]
    }
}

impl<T, U> IndexMut<U> for Grid<T>
where
    U: Borrow<GPoint>,
{
    fn index_mut(&mut self, p: U) -> &mut Self::Output {
        let p = p.borrow();
        &mut self.elems[p.y][p.x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector2::Direction::{Down, DownRight, Up};

    type GridRow = Vec<i32>;
    type GridVec = Vec<GridRow>;
    type Points = Vec<GPoint>;

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
        assert_eq!(g[&p2!(0, 0)], 0);
        assert_eq!(g[p2!(2, 2)], 12);
        assert_eq!(g[&p2!(2, 2)], 12);

        assert!(g.iter_points().all(|p| g.get_value(&p) == Some(&g[p])));
        assert_eq!(g.get_value(&p2!(0, 3)), None);

        let mut g = g;
        g.set_value(&p2!(1, 1), 999);
        assert_eq!(g.get_value(&p2!(1, 1)), Some(&999));

        g[p2!(1, 1)] = 998;
        assert_eq!(g.get_value(&p2!(1, 1)), Some(&998));
        g[&p2!(1, 1)] = 997;
        assert_eq!(g[p2!(1, 1)], 997);

        assert_eq!(g.row(1), vec![&5, &997, &7, &8, &9]);
        assert_eq!(g.col(1), vec![&1, &997, &11]);
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
            g.all_neighbors(&p2!(0, 0), false),
            vec![p2!(0, 1), p2!(1, 0)]
        );
        assert_eq!(
            g.all_neighbors(&p2!(0, 0), true),
            vec![p2!(1, 0), p2!(1, 1), p2!(0, 1)]
        );
        assert_eq!(
            g.all_neighbors(&p2!(1, 1), false),
            vec![p2!(1, 0), p2!(1, 2), p2!(0, 1), p2!(2, 1)]
        );
        assert_eq!(
            g.all_neighbors(&p2!(1, 1), true),
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

        assert_eq!(
            g.get_neighbors(&p2!(0, 0), &[Up, DownRight, Down]),
            vec![p2!(1, 1), p2!(0, 1)]
        );
        assert_eq!(g.get_neighbor(&p2!(0, 0), Down), Some(p2!(0, 1)));
        assert_eq!(g.get_neighbor(&p2!(0, 0), Up), None);

        assert_eq!(
            g.iter_all_neighbors(&p2!(1, 1), false).collect::<Vec<_>>(),
            vec![
                (p2!(1, 0), &1),
                (p2!(1, 2), &7),
                (p2!(0, 1), &3),
                (p2!(2, 1), &5),
            ]
        );
        assert_eq!(
            g.iter_all_neighbors(&p2!(1, 1), true).collect::<Vec<_>>(),
            vec![
                (p2!(1, 0), &1),
                (p2!(2, 0), &2),
                (p2!(2, 1), &5),
                (p2!(2, 2), &8),
                (p2!(1, 2), &7),
                (p2!(0, 2), &6),
                (p2!(0, 1), &3),
                (p2!(0, 0), &0),
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
        #[derive(Eq, PartialEq, Debug)]
        enum Direction {
            North,
            South,
        }

        impl From<char> for Direction {
            fn from(c: char) -> Self {
                match c {
                    'N' => Direction::North,
                    'S' => Direction::South,
                    _ => unreachable!(),
                }
            }
        }

        let grid: Grid<Direction> = "NS\nSN\n".parse().unwrap();
        assert_eq!(
            grid.elems,
            vec![
                vec![Direction::North, Direction::South],
                vec![Direction::South, Direction::North]
            ]
        );

        let grid: Grid<char> = "01\n23\n".parse().unwrap();
        assert_eq!(grid.elems, vec![vec!['0', '1'], vec!['2', '3']]);

        let grid: Grid<u8> = "01\n23\n".try_into().unwrap();
        assert_eq!(grid.elems, vec![vec![b'0', b'1'], vec![b'2', b'3']]);
    }
}
