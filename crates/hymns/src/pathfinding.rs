use std::cmp::Reverse;
use std::collections::HashMap;
use std::hash::Hash;

use priority_queue::PriorityQueue;

fn reconstruct_path<State, GetCost>(
    get_cost: GetCost,
    came_from: &HashMap<State, State>,
    end: &State,
) -> (Vec<State>, usize)
where
    State: Hash + Eq + Clone,
    GetCost: Fn(&State, &State) -> usize,
{
    let mut result = vec![end.clone()];
    let mut cost = 0;

    let mut current = end;

    while let Some(next) = came_from.get(current) {
        cost += get_cost(next, current);
        current = next;
        result.push(current.clone());
    }

    result.reverse();
    (result, cost)
}

// https://en.wikipedia.org/wiki/A*_search_algorithm
pub fn a_star<State, NextStateFn, HeuristicFn, CostFn>(
    start: &State,
    goal: &State,
    get_cost: CostFn,
    get_next_states: NextStateFn,
    heuristic_fn: HeuristicFn,
) -> Option<(Vec<State>, usize)>
where
    State: Hash + Eq + Clone,
    NextStateFn: Fn(&State) -> Vec<State>,
    HeuristicFn: Fn(&State, &State) -> usize,
    CostFn: Fn(&State, &State) -> usize,
{
    let mut open_set = PriorityQueue::new();
    open_set.push(start.clone(), Reverse(0));

    let mut came_from = HashMap::new();

    let mut g_score = HashMap::new();
    g_score.insert(start.clone(), 0_usize);

    while let Some((current, _)) = open_set.pop() {
        if current == *goal {
            return Some(reconstruct_path(get_cost, &came_from, &current));
        }

        for neigh in get_next_states(&current) {
            let tentative_g_score = g_score[&current] + get_cost(&current, &neigh);

            if tentative_g_score < *g_score.get(&neigh).unwrap_or(&usize::MAX) {
                came_from.insert(neigh.clone(), current.clone());
                g_score.insert(neigh.clone(), tentative_g_score);

                let f_score = tentative_g_score + heuristic_fn(&neigh, goal);
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
    fn test_basic_generic() {
        let grid = vec![vec![10, 1, 1], vec![9, 50, 9], vec![1, 1, 3]];
        let grid: Grid<u8> = Grid::new(grid);

        let (path, cost) = a_star(
            &GPoint::default(),
            &p2!(grid.cols() - 1, grid.rows() - 1),
            |_, dst| usize::from(grid[*dst]),
            |state| grid.all_neighbors(state, false),
            GPoint::manhattan_dist,
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
