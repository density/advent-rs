use std::collections::HashMap;
use std::path::PathBuf;

use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

fn calculate_sizes() -> HashMap<PathBuf, usize> {
    let mut dir_sizes: HashMap<PathBuf, usize> = HashMap::new();
    let mut cur_path = PathBuf::new();

    for line in INPUT.lines() {
        let mut word_iter = line.split_ascii_whitespace();

        match word_iter.next().unwrap() {
            "$" => {
                if word_iter.next().unwrap() == "cd" {
                    match word_iter.next().unwrap() {
                        "/" => cur_path.push("/"),
                        ".." => {
                            cur_path.pop();
                        }
                        other => cur_path.push(other),
                    }
                }
            }
            word if word != "dir" => {
                let size: usize = word.parse().unwrap();

                for path in cur_path.ancestors() {
                    *dir_sizes.entry(path.into()).or_insert(0) += size;
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
    let need = 30_000_000 - (70_000_000 - dir_sizes[&PathBuf::from("/")]);
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
        assert_eq!(part1(), 1_642_503);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 6_999_588);
    }
}
