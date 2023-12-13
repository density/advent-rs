extern crate core;

use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

fn part1() -> i64 {
    let mut register: i64 = 1;
    let mut cycle_count: usize = 0;
    let mut result = 0;

    for instruction in INPUT
        .lines()
        .map(|l| l.split_ascii_whitespace().collect::<Vec<_>>())
    {
        let (execution_time, add) = match instruction[0] {
            "noop" => (1, 0),
            "addx" => (2, instruction[1].parse().unwrap()),
            _ => unreachable!(),
        };

        for _ in 0..execution_time {
            cycle_count += 1;
            if cycle_count == 20 || cycle_count.checked_sub(20).map_or(false, |n| n % 40 == 0) {
                result += i64::try_from(cycle_count).unwrap() * register;
            }
        }

        register += add;
    }

    result
}

fn part2() -> String {
    let mut screen = vec![vec!["."; 40]; 6];
    let mut register: i64 = 1;
    let mut cycle_count: usize = 0;

    for instruction in INPUT
        .lines()
        .map(|l| l.split_ascii_whitespace().collect::<Vec<_>>())
    {
        let (execution_time, add) = match instruction[0] {
            "noop" => (1, 0),
            "addx" => (2, instruction[1].parse().unwrap()),
            _ => unreachable!(),
        };

        for _ in 0..execution_time {
            let row = cycle_count / 40;
            let col = cycle_count % 40;

            if (register - 1..=register + 1).contains(&col.try_into().unwrap()) {
                screen[row][col] = "#";
            }

            cycle_count += 1;
        }

        register += add;
    }

    screen
        .into_iter()
        .map(|row| {
            let mut row_str = row.into_iter().collect::<String>();
            row_str.push('\n');
            row_str
        })
        .collect()
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
        assert_eq!(part1(), 13060);
    }

    #[test]
    fn test_part2() {
        let expected = "####...##.#..#.###..#..#.#....###..####.
#.......#.#..#.#..#.#..#.#....#..#....#.
###.....#.#..#.###..#..#.#....#..#...#..
#.......#.#..#.#..#.#..#.#....###...#...
#....#..#.#..#.#..#.#..#.#....#.#..#....
#.....##...##..###...##..####.#..#.####.\n";
        assert_eq!(part2(), expected);
    }
}
