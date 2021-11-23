use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn inputs(prefix: &str) -> impl Iterator<Item = String> + '_ {
    let prefix_bytes = prefix.as_bytes();

    (1..).map(move |n| {
        let to_hash = [prefix_bytes, n.to_string().as_bytes()].concat();

        let digest = md5::compute(to_hash);
        format!("{:x}", digest)
    })
}

fn part1() -> usize {
    inputs(INPUT)
        .position(|s| s.chars().take(5).all(|c| c == '0'))
        .unwrap()
        + 1
}

fn part2() -> usize {
    inputs(INPUT)
        .position(|s| s.chars().take_while(|&c| c == '0').count() == 6)
        .unwrap()
        + 1
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
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 282749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 9962624);
    }
}
