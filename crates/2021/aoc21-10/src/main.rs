use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

fn part1() -> u64 {
    let mut score = 0;

    for line in INPUT.lines() {
        let mut stack = vec![];

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                closing => match (stack.pop().unwrap(), closing) {
                    ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => (),
                    (_, ')') => score += 3,
                    (_, ']') => score += 57,
                    (_, '}') => score += 1197,
                    (_, '>') => score += 25137,
                    _ => unreachable!(),
                },
            }
        }
    }

    score
}

fn build_stack(line: &str) -> Option<Vec<char>> {
    let mut stack = vec![];

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            closing => match (stack.last().unwrap(), closing) {
                ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => {
                    stack.pop();
                }
                _ => return None,
            },
        }
    }

    Some(stack)
}

fn part2() -> u64 {
    // TODO: use running median instead of sorting
    let mut scores = vec![];

    for stack in INPUT.lines().filter_map(build_stack) {
        let score = stack.into_iter().rev().fold(0, |mut score, c| {
            score *= 5;

            match c {
                '(' => score += 1,
                '[' => score += 2,
                '{' => score += 3,
                '<' => score += 4,
                _ => unreachable!(),
            }

            score
        });

        scores.push(score);
    }

    scores.sort_unstable();

    scores[scores.len() / 2]
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
        assert_eq!(part1(), 392_367);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2_192_104_158);
    }
}
