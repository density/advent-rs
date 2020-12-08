use regex::Regex;

use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

type Graph = HashMap<String, Vec<(u64, String)>>;

fn make_graph() -> Graph {
    let bag_re = Regex::new(r"(?P<count>\d*) ?(?P<color>[\w\s]+?) bags?(?:, | contain )?").unwrap();

    INPUT
        .lines()
        .filter_map(|line| {
            let mut cap_iter = bag_re.captures_iter(line).peekable();

            let src_color = cap_iter.next().unwrap().name("color").unwrap().as_str();

            if cap_iter.peek().unwrap().name("color").unwrap().as_str() == "no other" {
                return None;
            }

            let dst_colors = cap_iter
                .filter_map(|cap| {
                        let color = cap.name("color").unwrap().as_str();
                        let count: u64 = cap.name("count").unwrap().as_str().parse().unwrap();
                        Some((count, color.to_owned()))
                })
                .collect();

            Some((src_color.into(), dst_colors))
        })
        .collect()
}

fn part1() -> usize {
    let graph = make_graph();

    graph
        .keys()
        .filter(|&src_bag| can_reach_gold(&graph, src_bag))
        .count()
        - 1
}

fn can_reach_gold(graph: &Graph, start_color: &str) -> bool {
    if start_color == "shiny gold" {
        return true;
    }

    match graph.get(start_color) {
        Some(other_colors) => other_colors
            .iter()
            .any(|(_count, color)| color == "shiny gold" || can_reach_gold(&graph, color)),
        None => false,
    }
}

fn bags_required(graph: &Graph, memo: &mut HashMap<String, u64>, start_color: &str) -> u64 {
    if let Some(n) = memo.get(start_color) {
        return *n;
    }

    let result = match graph.get(start_color) {
        Some(other_colors) => other_colors
            .iter()
            .map(|(count, color)| *count * bags_required(graph, memo, color))
            .sum(),
        None => 0,
    } + 1;

    memo.insert(start_color.into(), result);
    result
}

fn part2() -> u64 {
    let graph: HashMap<String, Vec<(u64, String)>> = make_graph();
    let mut memo: HashMap<String, u64> = HashMap::new();

    bags_required(&graph, &mut memo, "shiny gold") - 1
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 131);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 11261);
    }
}
