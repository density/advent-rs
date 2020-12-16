use std::convert::TryInto;
use std::time::Instant;

const INPUT: [usize; 6] = [8, 11, 0, 19, 1, 2];

fn play_game(max_turns: usize) -> u64 {
    let mut spoken: Vec<_> = vec![0_usize; max_turns];
    let mut last_spoken = 0;

    for (turn, initial) in INPUT.iter().enumerate() {
        spoken[*initial] = turn + 1;
        last_spoken = *initial;
    }

    for turn in INPUT.len()..max_turns {
        let next_to_speak = match spoken[last_spoken] {
            0 => 0,
            last_time_spoken => turn - last_time_spoken,
        };
        spoken[last_spoken] = turn;
        last_spoken = next_to_speak;
    }

    last_spoken.try_into().unwrap()
}

fn part1() -> u64 {
    play_game(2020)
}

fn part2() -> u64 {
    play_game(30_000_000)
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
        assert_eq!(part1(), 447);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 11_721_679);
    }
}
