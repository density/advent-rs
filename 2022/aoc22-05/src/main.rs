use regex::Regex;

use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

fn generate_stacks(input: &str) -> Vec<Vec<char>> {
    let mut line_iter = input.lines().rev();

    let stack_count = line_iter.next().unwrap().split_ascii_whitespace().count();

    let mut stacks = vec![vec![]; stack_count];

    for line in line_iter {
        let char_iter = line.chars().skip(1).step_by(4);

        for (stack, crate_char) in stacks.iter_mut().zip(char_iter) {
            if crate_char.is_ascii_uppercase() {
                stack.push(crate_char);
            }
        }
    }

    stacks
}

fn read_instructions(input: &str) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    input.lines().map(move |line| {
        let caps = re.captures(line).unwrap();

        let count: usize = caps[1].parse().unwrap();
        let from: usize = caps[2].parse().unwrap();
        let to: usize = caps[3].parse().unwrap();

        (count, from - 1, to - 1)
    })
}

fn move_crates(keep_order: bool) -> String {
    let mut sections = INPUT.split("\n\n");

    let mut stacks: Vec<Vec<char>> = generate_stacks(sections.next().unwrap());

    for (count, from, to) in read_instructions(sections.next().unwrap()) {
        let take_idx = stacks[from].len() - count;

        let mut take = if keep_order {
            stacks[from].split_off(take_idx)
        } else {
            stacks[from].drain(take_idx..).rev().collect()
        };

        stacks[to].append(&mut take);
    }

    stacks.iter().map(|s| s.last().unwrap()).collect()
}

fn part1() -> String {
    move_crates(false)
}

fn part2() -> String {
    move_crates(true)
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
        assert_eq!(part1(), "VPCDMSLWJ");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "TPWCGNCCG");
    }
}
