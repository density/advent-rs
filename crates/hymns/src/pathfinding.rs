use hashbrown::HashMap;
use std::cmp::Reverse;
use std::collections::VecDeque;
use std::hash::Hash;

use priority_queue::PriorityQueue;

fn reconstruct_path<State, GetCost>(
    get_cost: GetCost,
    mut came_from: HashMap<State, State>,
    end: State,
) -> (Vec<State>, usize)
where
    State: Hash + Eq + Clone,
    GetCost: Fn(&State, &State) -> usize,
{
    let mut result = VecDeque::new();
    let mut cost = 0;

    let mut current = end;

    while let Some(next) = came_from.remove(&current) {
        cost += get_cost(&next, &current);
        result.push_front(current);
        current = next;
    }
    result.push_front(current);

    (result.into(), cost)
}

// https://en.wikipedia.org/wiki/A*_search_algorithm
pub fn a_star<State, NextStateFn, HeuristicFn, CostFn, GoalFn>(
    starts: &[State],
    is_goal: GoalFn,
    get_cost: CostFn,
    get_next_states: NextStateFn,
    heuristic_fn: HeuristicFn,
) -> Option<(Vec<State>, usize)>
where
    State: Hash + Eq + Clone,
    GoalFn: Fn(&State) -> bool,
    NextStateFn: Fn(&State) -> Vec<State>,
    HeuristicFn: Fn(&State) -> usize,
    CostFn: Fn(&State, &State) -> usize,
{
    let mut open_set = PriorityQueue::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();

    for start in starts {
        open_set.push(start.clone(), Reverse(0));
        g_score.insert(start.clone(), 0_usize);
    }

    while let Some((current, _)) = open_set.pop() {
        if is_goal(&current) {
            return Some(reconstruct_path(get_cost, came_from, current));
        }

        for neigh in get_next_states(&current) {
            let tentative_g_score = g_score[&current] + get_cost(&current, &neigh);

            if tentative_g_score < *g_score.get(&neigh).unwrap_or(&usize::MAX) {
                came_from.insert(neigh.clone(), current.clone());
                g_score.insert(neigh.clone(), tentative_g_score);

                let f_score = tentative_g_score + heuristic_fn(&neigh);
                if open_set.change_priority(&neigh, Reverse(f_score)).is_none() {
                    open_set.push(neigh, Reverse(f_score));
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::grid::{GPoint, Grid};
    use crate::p2;
    use crate::pathfinding::a_star;
    use crate::vector2::Point2;

    #[test]
    fn test_basic() {
        let grid = vec![vec![10, 1, 1], vec![9, 50, 9], vec![1, 1, 3]];
        let grid: Grid<u8> = Grid::new(grid);

        let goal = p2!(grid.cols() - 1, grid.rows() - 1);

        let (path, cost) = a_star(
            &[GPoint::default()],
            |p| *p == goal,
            |_, dst| usize::from(grid[dst]),
            |state| grid.all_neighbors(state, false),
            |p| p.manhattan_dist(&goal),
        )
        .unwrap();

        assert_eq!(cost, 14);

        let valid_paths = [
            vec![p2!(0, 0), p2!(1, 0), p2!(2, 0), p2!(2, 1), p2!(2, 2)],
            vec![p2!(0, 0), p2!(0, 1), p2!(0, 2), p2!(1, 2), p2!(2, 2)],
        ];

        assert!(valid_paths.contains(&path));
    }
}
