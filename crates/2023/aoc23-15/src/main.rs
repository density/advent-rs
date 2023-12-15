use itertools::Itertools;

use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

struct Lens<'a> {
    label: &'a str,
    focal_length: u8,
}

#[derive(Default)]
struct LensBox<'a> {
    lenses: Vec<Lens<'a>>,
}

impl<'a> LensBox<'a> {
    fn insert_lens(&mut self, label: &'a str, focal_length: u8) {
        let lens = Lens {
            label,
            focal_length,
        };

        if let Some(existing) = self.lenses.iter().position(|lens| lens.label == label) {
            self.lenses[existing] = lens;
        } else {
            self.lenses.push(lens);
        }
    }

    fn remove_lens(&mut self, label: &'a str) {
        if let Some(existing) = self.lenses.iter().position(|lens| lens.label == label) {
            self.lenses.remove(existing);
        }
    }
}

fn hash(s: &str) -> u8 {
    s.as_bytes()
        .iter()
        .fold(0_u64, |h, &c| (h + u64::from(c)) * 17 % 256)
        .try_into()
        .unwrap()
}

fn part1() -> u64 {
    INPUT.split(',').map(|s| u64::from(hash(s))).sum()
}

fn part2() -> usize {
    let mut lens_boxes: Vec<LensBox> = Vec::with_capacity(256);
    for _ in 0..256 {
        lens_boxes.push(LensBox::default());
    }

    for instruction in INPUT.split(',') {
        let (label, focal_length) = instruction
            .split(|c| c == '=' || c == '-')
            .collect_tuple()
            .unwrap();

        let box_id = usize::from(hash(label));

        if focal_length.is_empty() {
            lens_boxes[box_id].remove_lens(label);
        } else {
            lens_boxes[box_id].insert_lens(label, focal_length.parse().unwrap());
        }
    }

    lens_boxes
        .into_iter()
        .enumerate()
        .map(|(box_num, lens_box)| {
            lens_box
                .lenses
                .iter()
                .enumerate()
                .map(|(lens_num, lens)| {
                    (box_num + 1) * (lens_num + 1) * usize::from(lens.focal_length)
                })
                .sum::<usize>()
        })
        .sum()
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
        assert_eq!(part1(), 511_257);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 239_484);
    }
}
