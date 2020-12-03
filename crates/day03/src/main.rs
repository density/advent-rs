const INPUT: &str = include_str!("../input.txt");

fn count_trees(grid: &[Vec<char>], dr: usize, dc: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    (0..rows)
        .step_by(dr)
        .zip((0usize..).step_by(dc))
        .filter(|&(r, c)| grid[r][c % cols] == '#')
        .count()
}

fn part1() -> usize {
    let grid: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();

    count_trees(&grid, 1, 3)
}

fn part2() -> usize {
    let grid: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();

    let slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    slopes
        .iter()
        .map(|&(dr, dc)| count_trees(&grid, dr, dc))
        .product()
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
        assert_eq!(part2(), 9709761600);
    }
}
