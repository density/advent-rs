use hashbrown::{HashMap, HashSet};

use hymns::grid::{GPoint, Grid};
use hymns::p2;
use hymns::runner::timed_run;
use hymns::vector2::Direction::{Down, Left, Right, Up};
use hymns::vector2::{Direction, Point2};

use crate::Terrain::{Forest, Path, Slope};

const INPUT: &str = include_str!("../input.txt");

type Graph = HashMap<GPoint, Vec<(GPoint, usize)>>;

#[derive(Eq, PartialEq)]
enum Terrain {
    Path,
    Slope(Direction),
    Forest,
}

impl From<char> for Terrain {
    fn from(value: char) -> Self {
        match value {
            '.' => Path,
            '#' => Forest,
            '^' => Slope(Up),
            '>' => Slope(Right),
            'v' => Slope(Down),
            '<' => Slope(Left),
            _ => unreachable!(),
        }
    }
}

fn dfs(
    grid: &Grid<Terrain>,
    seen: &mut HashSet<GPoint>,
    goal: &GPoint,
    point: &GPoint,
    dist: usize,
    max_dist: &mut usize,
) {
    if point == goal {
        *max_dist = (*max_dist).max(dist);
        return;
    }

    if seen.contains(point) {
        return;
    }

    seen.insert(*point);

    match grid[point] {
        Path => {
            for neigh in grid.all_neighbors(point, false) {
                dfs(grid, seen, goal, &neigh, dist + 1, max_dist);
            }
        }
        Slope(dir) => {
            if let Some(neigh) = grid.get_neighbor(point, dir) {
                dfs(grid, seen, goal, &neigh, dist + 1, max_dist);
            }
        }
        Forest => {}
    };

    seen.remove(point);
}

fn dfs_graph(
    graph: &Graph,
    seen: &mut HashSet<GPoint>,
    goal: &GPoint,
    point: &GPoint,
    dist: usize,
    max_dist: &mut usize,
) {
    if point == goal {
        *max_dist = (*max_dist).max(dist);
        return;
    }

    if seen.contains(point) {
        return;
    }

    seen.insert(*point);

    if let Some(neighbors) = graph.get(point) {
        for (neigh, neigh_dist) in neighbors {
            dfs_graph(graph, seen, goal, neigh, dist + neigh_dist, max_dist);
        }
    }

    seen.remove(point);
}

fn dfs_contracted(
    grid: &Grid<Terrain>,
    goal: &GPoint,
    prev: &GPoint,
    current: &GPoint,
    graph: &mut Graph,
    distance: usize,
    prev_intersection: &GPoint,
) {
    if graph
        .get(current)
        .is_some_and(|neighbors| neighbors.iter().any(|(p, _)| p == prev_intersection))
    {
        return;
    }

    let mut neighbors = grid.all_neighbors(current, false);
    neighbors.retain(|p| grid[p] != Forest && p != prev);

    if neighbors.len() == 1 {
        dfs_contracted(
            grid,
            goal,
            current,
            &neighbors[0],
            graph,
            distance + 1,
            prev_intersection,
        );
    } else if current == goal || neighbors.len() > 1 {
        graph
            .entry(*current)
            .or_default()
            .push((*prev_intersection, distance));

        graph
            .entry(*prev_intersection)
            .or_default()
            .push((*current, distance));

        for neigh in neighbors {
            dfs_contracted(grid, goal, current, &neigh, graph, 1, current);
        }
    }
}

fn build_contracted_graph() -> (GPoint, GPoint, Graph) {
    let grid: Grid<Terrain> = INPUT.parse().unwrap();

    let start = p2!(grid.row(0).into_iter().position(|t| *t == Path).unwrap(), 0);
    let goal = p2!(
        grid.row(grid.rows() - 1)
            .into_iter()
            .position(|t| *t == Path)
            .unwrap(),
        grid.rows() - 1
    );

    let mut graph = HashMap::default();

    dfs_contracted(&grid, &goal, &start, &start, &mut graph, 0, &start);

    (start, goal, graph)
}

fn part1() -> usize {
    let grid: Grid<Terrain> = INPUT.parse().unwrap();

    let start = p2!(grid.row(0).into_iter().position(|t| *t == Path).unwrap(), 0);
    let goal = p2!(
        grid.row(grid.rows() - 1)
            .into_iter()
            .position(|t| *t == Path)
            .unwrap(),
        grid.rows() - 1
    );

    let mut max_distance = 0;

    dfs(
        &grid,
        &mut HashSet::default(),
        &goal,
        &start,
        0,
        &mut max_distance,
    );

    max_distance
}

fn part2() -> usize {
    let (start, goal, graph) = build_contracted_graph();

    let mut max_dist = 0;

    dfs_graph(
        &graph,
        &mut HashSet::default(),
        &goal,
        &start,
        0,
        &mut max_dist,
    );

    max_dist
}

fn main() {
    timed_run(1, part1);
    timed_run(2, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 2182);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 6670);
    }
}
