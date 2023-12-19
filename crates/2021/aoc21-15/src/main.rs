use hymns::grid::{GPoint, Grid};
use hymns::p2;
use hymns::pathfinding::a_star;
use hymns::runner::timed_run;
use hymns::vector2::Point2;

const INPUT: &str = include_str!("../input.txt");

fn get_least_total_risk_path(grid: &Grid<u8>) -> usize {
    let goal = p2!(grid.cols() - 1, grid.rows() - 1);

    let (_, cost) = a_star(
        &[GPoint::origin()],
        |p| *p == goal,
        |_, p2| usize::from(grid[p2]),
        |p| grid.all_neighbors(p, false),
        |p| p.manhattan_dist(&goal),
    )
    .unwrap();

    cost
}

fn build_grid(expanded: bool) -> Grid<u8> {
    let data: Vec<Vec<u8>> = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    if expanded {
        let num_rows = data[0].len();
        let num_cols = data.len();

        let mut expanded = vec![vec![0; data[0].len() * 5]; data.len() * 5];

        for (x, row) in expanded.iter_mut().enumerate() {
            for (y, elem) in row.iter_mut().enumerate() {
                let add = (x / num_rows) + (y / num_cols);

                let new_val = data[x % num_rows][y % num_cols] + u8::try_from(add).unwrap();
                *elem = if new_val > 9 { new_val % 9 } else { new_val };
            }
        }

        Grid::new(expanded)
    } else {
        Grid::new(data)
    }
}

fn part1() -> usize {
    let grid = build_grid(false);
    get_least_total_risk_path(&grid)
}

fn part2() -> usize {
    let grid = build_grid(true);
    get_least_total_risk_path(&grid)
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
        assert_eq!(part1(), 602);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2935);
    }
}
