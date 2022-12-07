use std::collections::HashMap;

use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

fn calculate_sizes() -> HashMap<String, usize> {
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    let mut dir_stack = vec!["/"];

    for line in INPUT.lines() {
        let mut word_iter = line.split_ascii_whitespace();

        match word_iter.next().unwrap() {
            "$" => {
                if word_iter.next().unwrap() == "cd" {
                    match word_iter.next().unwrap() {
                        "/" => dir_stack.truncate(1),
                        ".." => {
                            if dir_stack.len() > 1 {
                                dir_stack.pop();
                            }
                        }
                        other => dir_stack.push(other),
                    }
                }
            }
            word if word != "dir" => {
                let size: usize = word.parse().unwrap();

                let mut dir_string = String::new();

                for dir in dir_stack.iter() {
                    dir_string.push_str(dir);
                    *dir_sizes.entry(dir_string.clone()).or_insert(0) += size;
                    dir_string.push('/');
                }
            }
            _ => (),
        }
    }

    dir_sizes
}

fn part1() -> usize {
    let dir_sizes = calculate_sizes();
    dir_sizes.into_values().filter(|&s| s <= 100_000).sum()
}

fn part2() -> usize {
    let dir_sizes = calculate_sizes();
    let need = 30_000_000 - (70_000_000 - dir_sizes["/"]);
    dir_sizes
        .into_values()
        .filter(|&s| s >= need)
        .min()
        .unwrap()
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
        assert_eq!(part1(), 1642503);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 6999588);
    }
}
