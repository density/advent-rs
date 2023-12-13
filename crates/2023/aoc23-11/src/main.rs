use hymns::grid::Grid;
use hymns::p2;
use hymns::runner::timed_run;
use hymns::vector2::Point2;
use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn solve<const SCALE: usize>() -> usize {
    let grid: Grid<char> = INPUT.parse().unwrap();

    let zero_rows = grid
        .iter_rows()
        .positions(|row| row.iter().all(|c| *c == '.'))
        .collect_vec();

    let zero_cols = grid
        .iter_cols()
        .positions(|col| col.iter().all(|c| **c == '.'))
        .collect_vec();

    let total: usize = grid
        .iter_points_values()
        .filter_map(|(p, v)| if *v == '#' { Some(p) } else { None })
        .combinations(2)
        .map(|galaxies| {
            let g1 = galaxies[0];
            let g2 = galaxies[1];

            let new_g1 = p2!(
                g1.x + zero_cols.iter().filter(|c| **c < g1.x).count() * (SCALE - 1),
                g1.y + zero_rows.iter().filter(|r| **r < g1.y).count() * (SCALE - 1)
            );

            let new_g2 = p2!(
                g2.x + zero_cols.iter().filter(|c| **c < g2.x).count() * (SCALE - 1),
                g2.y + zero_rows.iter().filter(|r| **r < g2.y).count() * (SCALE - 1)
            );

            new_g1.manhattan_dist(&new_g2)
        })
        .sum();

    total
}

fn part1() -> usize {
    solve::<2>()
}

fn part2() -> usize {
    solve::<1_000_000>()
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
        assert_eq!(part1(), 10_165_598);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 678_728_808_158);
    }
}
