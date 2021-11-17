use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn read_instructions() -> Vec<(&'static str, i64)> {
    INPUT
        .lines()
        .map(|line| {
            let mut components = line.split_whitespace();
            (
                components.next().unwrap(),
                components.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect()
}

fn part1() -> i64 {
    let instructions = read_instructions();

    get_result(&instructions).err().unwrap()
}

fn get_result(instructions: &[(&str, i64)]) -> Result<i64, i64> {
    let mut seen: HashSet<usize> = HashSet::new();
    let mut acc = 0;
    let mut ip = 0;

    while ip != instructions.len() {
        seen.insert(ip);

        match instructions[ip] {
            ("jmp", offset) => {
                let new_offset = i64::try_from(ip).unwrap() + offset;
                ip = new_offset.try_into().unwrap()
            }
            ("acc", num) => {
                acc += num;
                ip += 1
            }
            _ => ip += 1,
        }

        if seen.contains(&ip) {
            return Err(acc);
        }
    }

    Ok(acc)
}

fn part2() -> i64 {
    let mut instructions = read_instructions();

    for i in 0..instructions.len() {
        let opcode = instructions[i].0;

        let old = opcode;

        let new = match old {
            "jmp" => "nop",
            "nop" => "jmp",
            _ => continue,
        };

        instructions[i].0 = new;
        if let Ok(acc) = get_result(&instructions) {
            return acc;
        }
        instructions[i].0 = old;
    }

    unreachable!()
}

fn main() {
    let start = Instant::now();
    println!("part 1: {}", part1());
    println!("part 1 took {}ms", (Instant::now() - start).as_millis());
    let start = Instant::now();
    println!("part 2: {}", part2());
    println!("part 2 took {}ms", (Instant::now() - start).as_millis());
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1671);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 892);
    }
}
