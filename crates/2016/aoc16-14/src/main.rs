use std::collections::VecDeque;
use std::io::Write;

use hymns::formatting::to_hex_array;
use itertools::Itertools;
use md5::Context;
use rayon::prelude::*;
use smallvec::{smallvec_inline, SmallVec};

use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");
const MD5_BATCH_SIZE: usize = 100;

type Checksum = SmallVec<[u8; 32]>;

fn calc_checksum(input: &[u8]) -> Checksum {
    let mut output = smallvec_inline![0; 32];
    to_hex_array(input, &mut output);
    output
}

struct Miner {
    context: Context,
    counter: usize,
    checksums: VecDeque<(usize, Checksum)>,
    extra_rounds: u64,
}

impl Miner {
    fn new(salt: &str, extra_rounds: u64) -> Self {
        let mut context = Context::new();
        context.write_all(salt.as_bytes()).unwrap();

        Self {
            context,
            counter: 0,
            checksums: VecDeque::with_capacity(1000 + MD5_BATCH_SIZE),
            extra_rounds,
        }
    }

    fn generate_md5_batch(&mut self) {
        self.checksums.par_extend(
            (self.counter..self.counter + MD5_BATCH_SIZE)
                .into_par_iter()
                .map(|counter| {
                    let mut context = self.context.clone();
                    context.write_all(counter.to_string().as_bytes()).unwrap();

                    let mut checksum = calc_checksum(context.compute().as_slice());

                    for _ in 0..self.extra_rounds {
                        checksum = calc_checksum(md5::compute(&checksum).as_slice());
                    }

                    (counter, checksum)
                }),
        );

        self.counter += MD5_BATCH_SIZE;
    }
}

impl Iterator for Miner {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.checksums.is_empty() {
                self.generate_md5_batch();
            }

            let (count, current) = self.checksums.pop_front().unwrap();

            let Some(target) = current
                .windows(3)
                .find(|w| w.iter().all_equal())
                .map(|w| w[0])
            else {
                continue;
            };

            for attempt in 0..1000 {
                if attempt == self.checksums.len() {
                    self.generate_md5_batch();
                };

                let (_, next) = &self.checksums[attempt];
                if next.windows(5).any(|w| w.iter().all(|c| c == &target)) {
                    return Some(count);
                }
            }
        }
    }
}

fn part1() -> usize {
    Miner::new(INPUT, 0).nth(63).unwrap()
}

fn part2() -> usize {
    Miner::new(INPUT, 2016).nth(63).unwrap()
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
        assert_eq!(part1(), 25427);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 22045);
    }
}
