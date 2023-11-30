use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn get_dimensions() -> impl Iterator<Item = Vec<u64>> {
    INPUT
        .lines()
        .map(|line| line.split('x').map(|n| n.parse().unwrap()).collect())
}

fn paper_length(dims: Vec<u64>) -> u64 {
    let side1 = dims[0] * dims[1];
    let side2 = dims[1] * dims[2];
    let side3 = dims[2] * dims[0];

    2 * side1 + 2 * side2 + 2 * side3 + [side1, side2, side3].iter().min().unwrap()
}

fn part1() -> u64 {
    get_dimensions().map(paper_length).sum()
}

fn ribbon_length(dims: Vec<u64>) -> u64 {
    let perimeters = [
        dims[0] * 2 + dims[1] * 2,
        dims[1] * 2 + dims[2] * 2,
        dims[2] * 2 + dims[0] * 2,
    ];

    perimeters.iter().min().unwrap() + dims.iter().product::<u64>()
}

fn part2() -> u64 {
    get_dimensions().map(ribbon_length).sum()
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
        assert_eq!(part1(), 1606483);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 3842356);
    }
}
