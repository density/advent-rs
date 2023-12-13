use hymns::math::chinese_remainder;
use std::convert::TryInto;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn read_buses() -> (i64, Vec<(i64, i64)>) {
    let mut line_iter = INPUT.lines();

    let earliest = line_iter.next().unwrap().parse().unwrap();

    let buses = line_iter
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(offset, bus)| {
            if bus == "x" {
                None
            } else {
                Some((offset.try_into().unwrap(), bus.parse().unwrap()))
            }
        })
        .collect();

    (earliest, buses)
}

fn part1() -> i64 {
    let (earliest, buses) = read_buses();

    (earliest..)
        .find_map(|time| {
            buses.iter().find_map(|(_, bus)| {
                if time % bus == 0 {
                    Some((time - earliest) * bus)
                } else {
                    None
                }
            })
        })
        .unwrap()
}

fn part2() -> i64 {
    let (_, buses) = read_buses();

    let bus_times: Vec<_> = buses.iter().map(|(_offset, bus_time)| *bus_time).collect();
    let remainders: Vec<_> = buses
        .iter()
        .map(|(offset, bus_time)| *bus_time - *offset)
        .collect();

    chinese_remainder(&remainders, &bus_times).unwrap()
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
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 2935);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 836024966345345);
    }
}
