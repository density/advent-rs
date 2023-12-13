use hymns::grid::Grid;
use hymns::runner::timed_run;
use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn find_row<const MAX_DIFF: usize>(lines: &Grid<u8>) -> Option<usize> {
    (1..lines.rows()).find(|&m| {
        let above = (0..m).rev();
        let below = m..lines.rows();

        let mismatches = above
            .zip(below)
            .map(|(l, r)| {
                lines
                    .row(l)
                    .into_iter()
                    .zip(lines.row(r))
                    .filter(|(x, y)| x != y)
                    .count()
            })
            .sum::<usize>();

        mismatches == MAX_DIFF
    })
}

fn find_col<const MAX_DIFF: usize>(lines: &Grid<u8>) -> Option<usize> {
    (1..lines.cols()).find(|&m| {
        let left = (0..m).rev();
        let right = m..lines.cols();

        let mismatches = left
            .zip(right)
            .map(|(l, r)| {
                lines
                    .col(l)
                    .into_iter()
                    .zip(lines.col(r))
                    .filter(|(x, y)| x != y)
                    .count()
            })
            .sum::<usize>();

        mismatches == MAX_DIFF
    })
}

fn solve<const MAX_DIFF: usize>() -> usize {
    let grids = INPUT
        .split("\n\n")
        .map(|pat| {
            Grid::new(
                pat.lines()
                    .map(|line| line.as_bytes().iter().copied().collect_vec())
                    .collect_vec(),
            )
        })
        .collect_vec();

    grids
        .iter()
        .map(|grid| {
            find_col::<MAX_DIFF>(grid).unwrap_or_else(|| find_row::<MAX_DIFF>(grid).unwrap() * 100)
        })
        .sum()
}

fn part1() -> usize {
    solve::<0>()
}

fn part2() -> usize {
    solve::<1>()
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
        assert_eq!(part1(), 43614);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 36771);
    }
}
