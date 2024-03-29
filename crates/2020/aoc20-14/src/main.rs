use regex::Regex;
use std::collections::HashMap;
use std::convert::TryInto;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

#[derive(Default)]
struct Mask {
    str: Box<[u8]>,
    and: u64,
    or: u64,
}

impl Mask {
    fn apply(&self, n: u64) -> u64 {
        n & self.and | self.or
    }
}

impl From<&[u8]> for Mask {
    fn from(bytes: &[u8]) -> Self {
        let mut and = 0;
        let mut or = 0;

        for c in bytes {
            and <<= 1;
            or <<= 1;

            match c {
                b'X' => and |= 1,
                b'0' => (),
                b'1' => or |= 1,
                _ => unreachable!(),
            }
        }

        Self {
            str: bytes.into(),
            and,
            or,
        }
    }
}

enum Command {
    SetMask(&'static [u8]),
    Write(usize, u64),
}

fn read_commands() -> impl Iterator<Item = Command> {
    let mask_re = Regex::new(r"mask = (.+)").unwrap();
    let mem_re = Regex::new(r"mem\[(?P<addr>\d+)\] = (?P<val>\d+)").unwrap();

    INPUT.lines().map(move |line| {
        if let Some(cap) = mask_re.captures(line) {
            let mask_str = cap.get(1).unwrap().as_str();

            Command::SetMask(mask_str.as_bytes())
        } else if let Some(cap) = mem_re.captures(line) {
            let addr: usize = cap.name("addr").unwrap().as_str().parse().unwrap();
            let val: u64 = cap.name("val").unwrap().as_str().parse().unwrap();

            Command::Write(addr, val)
        } else {
            unreachable!()
        }
    })
}

fn part1() -> u64 {
    let mut mem: HashMap<usize, u64> = HashMap::new();

    let mut mask = Mask::default();

    for command in read_commands() {
        match command {
            Command::SetMask(new_mask) => mask = new_mask.into(),
            Command::Write(addr, val) => {
                mem.insert(addr, mask.apply(val));
            }
        }
    }

    mem.values().sum()
}

fn addresses_from_mask(orig_mask: &[u8]) -> impl Iterator<Item = usize> {
    // Get the positions of each of the floating bits
    let floating_positions: Vec<_> = orig_mask
        .iter()
        .rev()
        .enumerate()
        .filter_map(|(i, c)| if *c == b'X' { Some(i) } else { None })
        .collect();

    let mask_max = 1 << floating_positions.len();

    // Apply the address mask to 0 so all Xs are eliminated and replaced by 0s
    let base_address: usize = Mask::from(orig_mask).apply(0).try_into().unwrap();

    // Generate numbers that take on all the possible values for the number of floating bits we have
    // and apply them toe the base address in the floating positions.
    (0..mask_max).map(move |mut i| {
        let mut address = base_address;

        for masked_pos in &floating_positions {
            if i & 1 == 1 {
                address |= 1 << masked_pos;
            } else {
                address &= !(1 << masked_pos);
            }

            i >>= 1;
        }

        address
    })
}

fn generate_mask_from_address(address: usize, mask: &Mask) -> Vec<u8> {
    let mut mask = mask.str.to_vec();
    let mut cur_address = address;

    for mask_offset in (0..mask.len()).rev() {
        if cur_address == 0 {
            break;
        }

        match mask[mask_offset] {
            b'X' | b'1' => (),
            b'0' => mask[mask_offset] = if (cur_address & 1) == 1 { b'1' } else { b'0' },
            _ => unreachable!(),
        }

        cur_address >>= 1;
    }

    mask
}

fn part2() -> u64 {
    let mut mem: HashMap<usize, u64> = HashMap::new();

    let mut cur_mask = Mask::default();

    for command in read_commands() {
        match command {
            Command::SetMask(new_mask) => cur_mask = new_mask.into(),
            Command::Write(addr, val) => {
                let initial_mask = generate_mask_from_address(addr, &cur_mask);

                mem.extend(addresses_from_mask(&initial_mask).map(|new_addr| (new_addr, val)));
            }
        }
    }

    mem.values().sum()
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
        assert_eq!(part1(), 11_884_151_942_312);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2_625_449_018_811);
    }
}
