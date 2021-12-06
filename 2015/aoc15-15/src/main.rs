use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn new(line: &str) -> Self {
        let mut components: Vec<_> = line.split_ascii_whitespace().collect();

        Self {
            name: components[0].strip_suffix(':').unwrap().to_string(),
            capacity: components[2].strip_suffix(',').unwrap().parse().unwrap(),
            durability: components[4].strip_suffix(',').unwrap().parse().unwrap(),
            flavor: components[6].strip_suffix(',').unwrap().parse().unwrap(),
            texture: components[8].strip_suffix(',').unwrap().parse().unwrap(),
            calories: components[10].parse().unwrap(),
        }
    }
}

fn part1() -> u64 {
    let ingredients: Vec<_> = INPUT.lines().map(Ingredient::new).collect();

    dbg!(ingredients);

    todo!()
}

fn part2() -> u64 {
    todo!()
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
        assert_eq!(part1(), todo!());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), todo!());
    }
}
