use hymns::grid::Grid;
use hymns::more_itertools::MoreItertools;
use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

fn part1() -> String {
    let grid: Grid<u8> = Grid::try_from(INPUT).unwrap();

    grid.iter_cols()
        .map(|col| {
            char::from(
                col.into_iter()
                    .copied()
                    .collect_counter()
                    .into_iter()
                    .max_by_key(|(_, count)| *count)
                    .unwrap()
                    .0,
            )
        })
        .collect_string()
}

fn part2() -> String {
    let grid: Grid<u8> = Grid::try_from(INPUT).unwrap();

    grid.iter_cols()
        .map(|col| {
            char::from(
                col.into_iter()
                    .copied()
                    .collect_counter()
                    .into_iter()
                    .min_by_key(|(_, count)| *count)
                    .unwrap()
                    .0,
            )
        })
        .collect_string()
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
        assert_eq!(part1(), "qzedlxso");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "ucmifjae");
    }
}
