use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

fn part1() -> usize {
    let grid: Vec<Vec<u8>> = INPUT
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut total_visible = grid.len() * 4 - 4;

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            let height = grid[row][col];

            let is_visible =
                // left
                (0..row).all(|r| grid[r][col] < height)
                    // right
                    || (row + 1..rows).all(|r| grid[r][col] < height)
                    // up
                    || (0..col).all(|c| grid[row][c] < height)
                    // down
                    || (col + 1..cols).all(|c| grid[row][c] < height);

            if is_visible {
                total_visible += 1;
            }
        }
    }

    total_visible
}

fn part2() -> usize {
    let grid: Vec<Vec<u8>> = INPUT
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut most_scenic = 0;

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            let height = grid[row][col];

            let mut total = 1;

            // up
            total *= (0..row)
                .rev()
                .position(|r| grid[r][col] >= height)
                .map_or(row, |n| n + 1);
            // down
            total *= (row + 1..rows)
                .position(|r| grid[r][col] >= height)
                .map_or(rows - row - 1, |n| n + 1);
            // left
            total *= (0..col)
                .rev()
                .position(|c| grid[row][c] >= height)
                .map_or(col, |n| n + 1);
            // right
            total *= (col + 1..cols)
                .position(|c| grid[row][c] >= height)
                .map_or(cols - col - 1, |n| n + 1);

            most_scenic = most_scenic.max(total);
        }
    }

    most_scenic
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
        assert_eq!(part1(), 1684);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 486540);
    }
}
