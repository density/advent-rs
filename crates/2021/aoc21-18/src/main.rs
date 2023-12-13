use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Peekable;
use std::ops::AddAssign;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use hymns::runner::timed_run;

const INPUT: &str = include_str!("../input.txt");

type SharedNode = Rc<RefCell<Node>>;

#[derive(Debug)]
enum NodeData {
    Leaf(u8),
    Children(SharedNode, SharedNode),
}

#[derive(Debug)]
struct Node {
    parent: Option<Weak<RefCell<Node>>>,
    contents: NodeData,
}

impl Node {
    fn new(contents: NodeData) -> SharedNode {
        Rc::new(RefCell::new(Self {
            parent: None,
            contents,
        }))
    }

    fn magnitude(&self) -> u64 {
        match &self.contents {
            NodeData::Leaf(n) => u64::from(*n),
            NodeData::Children(left, right) => {
                3 * left.borrow().magnitude() + 2 * right.borrow().magnitude()
            }
        }
    }

    fn increment_value(&mut self, add: u8) {
        match &self.contents {
            NodeData::Leaf(n) => self.contents = NodeData::Leaf(n + add),
            NodeData::Children(_, _) => unreachable!(),
        }
    }

    fn value(&self) -> Option<u8> {
        match &self.contents {
            NodeData::Leaf(n) => Some(*n),
            NodeData::Children(_, _) => None,
        }
    }

    fn left(&self) -> Option<SharedNode> {
        match &self.contents {
            NodeData::Leaf(_) => None,
            NodeData::Children(left, _) => Some(Rc::clone(left)),
        }
    }

    fn right(&self) -> Option<SharedNode> {
        match &self.contents {
            NodeData::Leaf(_) => None,
            NodeData::Children(_, right) => Some(Rc::clone(right)),
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.contents {
            NodeData::Leaf(n) => write!(f, "{n}"),
            NodeData::Children(left, right) => {
                write!(f, "[{},{}]", left.borrow(), right.borrow())
            }
        }
    }
}

struct Tree {
    root: SharedNode,
}

impl Tree {
    fn find_node_to_expode(&self) -> Option<SharedNode> {
        let mut queue = VecDeque::new();
        queue.push_back((Rc::clone(&self.root), 0));

        while let Some((node, depth)) = queue.pop_front() {
            if let NodeData::Children(left, right) = &node.borrow().contents {
                if depth == 4
                    && matches!(left.borrow().contents, NodeData::Leaf(_))
                    && matches!(right.borrow().contents, NodeData::Leaf(_))
                {
                    return Some(Rc::clone(&node));
                }

                queue.push_back((Rc::clone(left), depth + 1));
                queue.push_back((Rc::clone(right), depth + 1));
            }
        }

        None
    }

    fn find_node_to_split(&self) -> Option<SharedNode> {
        fn find_node(tree: &SharedNode) -> Option<SharedNode> {
            match &tree.borrow().contents {
                NodeData::Leaf(n) if *n >= 10 => Some(Rc::clone(tree)),
                NodeData::Leaf(_) => None,
                NodeData::Children(left, right) => find_node(left).or_else(|| find_node(right)),
            }
        }

        find_node(&self.root)
    }

    fn find_branch_point(start_node: &SharedNode, want_to_go_left: bool) -> Option<SharedNode> {
        let mut parent = start_node.borrow().parent.clone();
        let mut previous = Rc::clone(start_node);

        while let Some(current_node) = &parent {
            let possible_target = if want_to_go_left {
                current_node.upgrade().unwrap().borrow().left()
            } else {
                current_node.upgrade().unwrap().borrow().right()
            };

            if let Some(target) = possible_target {
                if !Rc::ptr_eq(&target, &previous) {
                    return Some(Rc::clone(&target));
                }
            }

            previous = Rc::clone(&current_node.upgrade().unwrap());
            parent = previous.borrow().parent.clone();
        }

        None
    }

    fn combine(n1: &SharedNode, n2: &SharedNode) -> SharedNode {
        let new_root = Node::new(NodeData::Children(Rc::clone(n1), Rc::clone(n2)));

        n1.borrow_mut().parent.replace(Rc::downgrade(&new_root));
        n2.borrow_mut().parent.replace(Rc::downgrade(&new_root));

        new_root
    }

    fn find_successor(node: &SharedNode, left: bool) -> Option<SharedNode> {
        if let Some(branch_point) = Tree::find_branch_point(node, left) {
            let mut possible_value_node = branch_point;

            loop {
                if possible_value_node.borrow().value().is_some() {
                    return Some(possible_value_node);
                }

                let next = if left {
                    Rc::clone(&possible_value_node.borrow().right().unwrap())
                } else {
                    Rc::clone(&possible_value_node.borrow().left().unwrap())
                };
                possible_value_node = next;
            }
        }
        None
    }

    fn explode_node(node: &SharedNode) {
        if let Some(left) = Tree::find_successor(node, true) {
            let left_value = node.borrow().left().unwrap().borrow().value().unwrap();
            left.borrow_mut().increment_value(left_value);
        }

        if let Some(right) = Tree::find_successor(node, false) {
            let right_value = node.borrow().right().unwrap().borrow().value().unwrap();
            right.borrow_mut().increment_value(right_value);
        }

        node.borrow_mut().contents = NodeData::Leaf(0);
    }

    fn split_node(node: &SharedNode) {
        let value = node.borrow().value().unwrap();

        let new_left_value = value / 2;

        let left_node = Node::new(NodeData::Leaf(new_left_value));
        left_node.borrow_mut().parent = Some(Rc::downgrade(node));

        let new_right_value = new_left_value + value % 2;
        let right_node = Node::new(NodeData::Leaf(new_right_value));
        right_node.borrow_mut().parent = Some(Rc::downgrade(node));

        node.borrow_mut().contents = NodeData::Children(left_node, right_node);
    }

    fn build<I>(input: &mut Peekable<I>) -> SharedNode
    where
        I: Iterator<Item = u8>,
    {
        match input.next().unwrap() {
            c if c.is_ascii_digit() => Node::new(NodeData::Leaf(c - b'0')),
            b'[' => {
                let left = Self::build(input);

                while let Some(&c) = input.peek() {
                    if c == b',' || c == b']' {
                        input.next();
                    } else {
                        break;
                    }
                }

                let right = Self::build(input);

                let new_parent = Node::new(NodeData::Children(Rc::clone(&left), Rc::clone(&right)));

                left.borrow_mut().parent = Some(Rc::downgrade(&new_parent));
                right.borrow_mut().parent = Some(Rc::downgrade(&new_parent));

                new_parent
            }
            _ => unreachable!(),
        }
    }

    fn magnitude(&self) -> u64 {
        self.root.borrow().magnitude()
    }
}

impl FromStr for Tree {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes_iter = s.as_bytes().iter().copied().peekable();
        Ok(Self {
            root: Self::build(&mut bytes_iter),
        })
    }
}

impl AddAssign for Tree {
    fn add_assign(&mut self, rhs: Self) {
        self.root = Self::combine(&self.root, &rhs.root);

        loop {
            if let Some(node) = self.find_node_to_expode() {
                Tree::explode_node(&node);
                continue;
            }

            if let Some(node) = self.find_node_to_split() {
                Tree::split_node(&node);
                continue;
            }

            break;
        }
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root.borrow())
    }
}

fn part1() -> u64 {
    INPUT
        .lines()
        .map(|line| line.parse::<Tree>().unwrap())
        .reduce(|mut acc, tree| {
            acc += tree;
            acc
        })
        .unwrap()
        .magnitude()
}

fn part2() -> u64 {
    let lines: Vec<_> = INPUT.lines().collect();

    let mut max_magnitude = 0;

    for i in 0..lines.len() {
        for j in (i + 1)..lines.len() {
            max_magnitude = max_magnitude.max({
                let mut t1 = lines[i].parse::<Tree>().unwrap();
                let t2 = lines[j].parse::<Tree>().unwrap();
                t1 += t2;
                t1.magnitude()
            });
            max_magnitude = max_magnitude.max({
                let mut t1 = lines[j].parse::<Tree>().unwrap();
                let t2 = lines[i].parse::<Tree>().unwrap();
                t1 += t2;
                t1.magnitude()
            });
        }
    }

    max_magnitude
}

fn main() {
    // NB: This is not the best way to solve this problem in rust because trees are annoying to implement.
    timed_run(1, part1);
    timed_run(2, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 4124);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 4673);
    }

    #[test]
    fn test_build() {
        let test_strs = [
            "[1,2]",
            "[[1,2],3]",
            "[9,[8,7]]",
            "[[1,9],[8,5]]",
            "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]",
            "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]",
            "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]",
        ];

        for s in test_strs {
            let tree: Tree = s.parse().unwrap();
            assert_eq!(tree.to_string(), s);
        }
    }

    #[test]
    fn test_explode() {
        let tests = [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ];

        for (before, after) in tests {
            let tree: Tree = before.parse().unwrap();
            let to_explode = tree.find_node_to_expode();
            Tree::explode_node(&to_explode.unwrap());
            assert_eq!(tree.to_string(), after);
        }
    }

    #[test]
    fn test_addition_steps() {
        let t1: Tree = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse().unwrap();
        let t2: Tree = "[1,1]".parse().unwrap();

        let tree = Tree {
            root: Tree::combine(&t1.root, &t2.root),
        };

        let to_explode = tree.find_node_to_expode().unwrap();
        Tree::explode_node(&to_explode);
        assert_eq!(tree.to_string(), "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");

        let to_explode = tree.find_node_to_expode().unwrap();
        Tree::explode_node(&to_explode);
        assert_eq!(tree.to_string(), "[[[[0,7],4],[15,[0,13]]],[1,1]]");

        assert!(tree.find_node_to_expode().is_none());

        let to_split = tree.find_node_to_split().unwrap();
        Tree::split_node(&to_split);
        assert_eq!(tree.to_string(), "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");

        let to_split = tree.find_node_to_split().unwrap();
        Tree::split_node(&to_split);
        assert_eq!(tree.to_string(), "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");

        assert!(tree.find_node_to_split().is_none());

        let to_explode = tree.find_node_to_expode().unwrap();
        Tree::explode_node(&to_explode);
        assert_eq!(tree.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

        assert!(tree.find_node_to_expode().is_none());
        assert!(tree.find_node_to_split().is_none());
    }

    #[test]
    fn test_addition_full() {
        let mut t1: Tree = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse().unwrap();
        let t2: Tree = "[1,1]".parse().unwrap();

        t1 += t2;
        assert_eq!(t1.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_magnitude() {
        let tests = [
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ];

        for (tree, magnitude) in tests {
            let tree: Tree = tree.parse().unwrap();
            assert_eq!(tree.magnitude(), magnitude);
        }
    }

    #[test]
    fn test_example_addition() {
        let input = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#;

        let result = input
            .lines()
            .map(|line| line.parse::<Tree>().unwrap())
            .reduce(|mut acc, next| {
                acc += next;
                acc
            })
            .unwrap();
        assert_eq!(result.magnitude(), 4140);
    }
}
