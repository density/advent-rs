use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");

struct Circuit {
    wire_values: HashMap<String, u16>,
    wire_to_instruction: HashMap<String, String>,
}

impl Circuit {
    fn new(cmd_map: HashMap<String, String>) -> Self {
        Self {
            wire_values: HashMap::new(),
            wire_to_instruction: cmd_map,
        }
    }

    fn reset(&mut self) {
        self.wire_values.clear();
    }

    fn get_value(&mut self, wire: &str) -> u16 {
        if let Ok(n) = wire.parse() {
            return n;
        }

        if let Some(n) = self.wire_values.get(wire) {
            return *n;
        }

        let instruction = self.wire_to_instruction[wire].clone();
        let split: Vec<_> = instruction.split_whitespace().collect();

        let value = if split[0] == "NOT" {
            !self.get_value(split[1])
        } else if split.len() == 1 {
            self.get_value(split[0])
        } else if split[1] == "AND" {
            self.get_value(split[0]) & self.get_value(split[2])
        } else if split[1] == "OR" {
            self.get_value(split[0]) | self.get_value(split[2])
        } else if split[1] == "LSHIFT" {
            self.get_value(split[0]) << split[2].parse::<u16>().unwrap()
        } else if split[1] == "RSHIFT" {
            self.get_value(split[0]) >> split[2].parse::<u16>().unwrap()
        } else {
            unreachable!()
        };

        self.wire_values.insert(wire.to_owned(), value);
        value
    }
}

fn build_circuit() -> Circuit {
    let mut wire_to_instruction = HashMap::new();

    for line in INPUT.lines() {
        let mut split = line.split(" -> ");

        let instruction = split.next().unwrap();
        let wire = split.next().unwrap();

        wire_to_instruction.insert(wire.to_owned(), instruction.to_owned());
    }

    Circuit::new(wire_to_instruction)
}

fn part1() -> u16 {
    let mut circuit = build_circuit();
    circuit.get_value("a")
}

fn part2() -> u16 {
    let mut circuit = build_circuit();
    let a = circuit.get_value("a");
    circuit
        .wire_to_instruction
        .insert("b".to_string(), a.to_string());
    circuit.reset();
    circuit.get_value("a")
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
        assert_eq!(part1(), 16076);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 2797);
    }
}
