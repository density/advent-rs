use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

fn read_input() -> (Vec<u32>, impl Iterator<Item = u32>) {
    let bit_length = INPUT.lines().next().unwrap().len();

    let nums: Vec<_> = INPUT
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();

    let masks = (0..bit_length)
        .into_iter()
        .rev()
        .map(|i| (1 << i).try_into().unwrap());

    (nums, masks)
}

fn most_common(nums: &[u32], mask: u32) -> u32 {
    if nums.iter().filter(|&&n| n & mask == 0).count() > nums.len() / 2 {
        0
    } else {
        1
    }
}

fn part1() -> u32 {
    let (nums, masks) = read_input();

    let mut gamma = 0;
    let mut episilon = 0;

    for mask in masks {
        if most_common(&nums, mask) == 1 {
            gamma |= mask;
        } else {
            episilon |= mask;
        }
    }

    gamma * episilon
}

fn part2() -> u32 {
    let (nums, masks) = read_input();
    let masks: Vec<_> = masks.collect();

    let oxygen = find_co2_or_oxygen(&nums, &masks, u32::eq);
    let co2 = find_co2_or_oxygen(&nums, &masks, u32::ne);

    oxygen * co2
}

fn find_co2_or_oxygen(nums: &[u32], masks: &[u32], cmp: fn(&u32, &u32) -> bool) -> u32 {
    let mut co2_or_oxygen_nums = nums.to_vec();

    for &mask in masks {
        let common = most_common(&co2_or_oxygen_nums, mask);

        for i in (0..co2_or_oxygen_nums.len()).rev() {
            if cmp(
                &((co2_or_oxygen_nums[i] & mask) >> mask.trailing_zeros()),
                &common,
            ) {
                co2_or_oxygen_nums.swap_remove(i);
            }

            if co2_or_oxygen_nums.len() == 1 {
                return co2_or_oxygen_nums[0];
            }
        }
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
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 3969000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 4267809);
    }
}
