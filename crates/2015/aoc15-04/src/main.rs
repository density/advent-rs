use std::time::Instant;

use md5::Digest;

const INPUT: &str = include_str!("../input.txt");

fn inputs(prefix: &str) -> impl Iterator<Item = Digest> + '_ {
    let prefix_bytes = prefix.as_bytes();

    (1..).map(move |n| {
        let to_hash = [prefix_bytes, n.to_string().as_bytes()].concat();

        md5::compute(to_hash)
    })
}

fn part1() -> usize {
    inputs(INPUT)
        .position(|s| {
            let n = u32::from_ne_bytes(s[0..4].try_into().unwrap()).to_be();

            // bits should be 00 00 0x xx
            n >> 12 == 0
        })
        .unwrap()
        + 1
}

fn part2() -> usize {
    inputs(INPUT)
        .position(|s| {
            let n = u32::from_ne_bytes(s[0..4].try_into().unwrap()).to_be();
            // bits should be 00 00 00 xy where x is not 0
            n >> 8 == 0 && n & 0xf0 != 0
        })
        .unwrap()
        + 1
}

fn main() {
    let start = Instant::now();
    println!("part 1: {}", part1());
    println!("part 1 took {}ms", start.elapsed().as_millis());

    let start = Instant::now();
    println!("part 2: {}", part2());
    println!("part 2 took {}ms", start.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 282_749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 9_962_624);
    }
}
