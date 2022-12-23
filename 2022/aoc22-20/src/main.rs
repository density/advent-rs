use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Copy, Clone)]
struct Node {
    shift_amount: isize,
    next: usize,
    prev: usize,
}

impl Node {
    fn new(shift_amount: isize, next: usize, prev: usize) -> Self {
        Self {
            shift_amount,
            next,
            prev,
        }
    }
}

fn build_nodes(key: isize) -> Vec<Node> {
    let nums: Vec<isize> = INPUT.lines().map(|line| line.parse().unwrap()).collect();

    nums.iter()
        .enumerate()
        .map(|(i, &v)| {
            Node::new(
                v * key,
                (i + 1) % nums.len(),
                i.checked_sub(1).unwrap_or(nums.len() - 1),
            )
        })
        .collect()
}

fn mix(nodes: &mut [Node]) {
    for i in 0..nodes.len() {
        shift_node(nodes, i);
    }
}

fn shift_node(nodes: &mut [Node], src: usize) {
    let node_to_move = nodes[src];

    let count = isize::try_from(nodes.len()).unwrap() - 1;

    let shift = node_to_move.shift_amount % count;
    // let shift = node_to_move.shift_amount.rem(isize::try_from(nodes.len()).unwrap());

    if shift == 0 {
        return;
    }

    // determine if going left or right is shorter
    let walk_left = if shift < 0 { shift } else { shift - count } - 1;

    let walk_right = if shift > 0 { shift } else { shift + count };

    let shift = if walk_left.abs() < walk_right.abs() {
        walk_left
    } else {
        walk_right
    };

    // take the node out
    nodes[node_to_move.prev].next = node_to_move.next;
    nodes[node_to_move.next].prev = node_to_move.prev;

    let mut prev = src;

    for _ in 0..shift.abs() {
        if shift > 0 {
            prev = nodes[prev].next;
        } else {
            prev = nodes[prev].prev;
        }
    }

    let next = nodes[prev].next;

    nodes[prev].next = src;
    nodes[next].prev = src;

    nodes[src].next = next;
    nodes[src].prev = prev;
}

fn get_coordinate_sum(nodes: &[Node]) -> isize {
    let mut cur = nodes.iter().position(|n| n.shift_amount == 0).unwrap();

    (0..)
        .filter_map(|n| {
            let result = if n > 0 && n % 1000 == 0 {
                Some(nodes[cur].shift_amount)
            } else {
                None
            };

            cur = nodes[cur].next;
            result
        })
        .take(3)
        .sum()
}

fn part1() -> isize {
    let mut nodes = build_nodes(1);

    mix(&mut nodes);

    get_coordinate_sum(&nodes)
}

fn part2() -> isize {
    let mut nodes = build_nodes(811589153);

    for _ in 0..10 {
        mix(&mut nodes);
    }

    get_coordinate_sum(&nodes)
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
        assert_eq!(part1(), 6712);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1595584274798);
    }
}
