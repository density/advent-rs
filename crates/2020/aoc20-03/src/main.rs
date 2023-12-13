const INPUT: &str = include_str!("../input.txt");

fn count_trees(slopes: &'static [(usize, usize)]) -> impl Iterator<Item = usize> {
    let grid: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    slopes.iter().map(move |&(dr, dc)| {
        (0..rows)
            .step_by(dr)
            .zip((0..cols).cycle().step_by(dc))
            .filter(|&(r, c)| grid[r][c] == '#')
            .count()
    })
}

fn part1() -> usize {
    count_trees(&[(1, 3)]).next().unwrap()
}

fn part2() -> usize {
    count_trees(&[(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]).product()
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 278);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 9_709_761_600);
    }
}
