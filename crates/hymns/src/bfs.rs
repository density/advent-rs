use std::collections::HashSet;
use std::hash::Hash;
use std::mem::swap;

pub struct BFSState<State, NextStateFn>
where
    State: Hash + Eq + Clone,
    NextStateFn: Fn(&State) -> Vec<State>,
{
    pub seen: HashSet<State>,
    frontier: Vec<State>,
    next_frontier: Vec<State>,
    next_state_fn: NextStateFn,
}

impl<State, NextStateFn> BFSState<State, NextStateFn>
where
    State: Hash + Eq + Clone,
    NextStateFn: Fn(&State) -> Vec<State>,
{
    fn new(start: State, next_state_fn: NextStateFn) -> Self {
        Self {
            seen: HashSet::new(),
            frontier: vec![start],
            next_frontier: vec![],
            next_state_fn,
        }
    }
}

impl<State, NextStateFn> Iterator for BFSState<State, NextStateFn>
where
    State: Hash + Eq + Clone,
    NextStateFn: Fn(&State) -> Vec<State>,
{
    type Item = Vec<State>;

    fn next(&mut self) -> Option<Self::Item> {
        debug_assert!(self.next_frontier.is_empty());

        for state in self.frontier.drain(..) {
            let neighbors = (self.next_state_fn)(&state);

            self.seen.insert(state);

            self.next_frontier
                .extend(neighbors.into_iter().filter(|n| !self.seen.contains(n)));
        }

        if self.next_frontier.is_empty() {
            return None;
        }

        swap(&mut self.next_frontier, &mut self.frontier);

        Some(self.frontier.clone())
    }
}

pub fn bfs<State, NextStateFn>(
    start: State,
    get_neighbors: NextStateFn,
) -> BFSState<State, NextStateFn>
where
    State: Hash + Eq + Clone,
    NextStateFn: Fn(&State) -> Vec<State>,
{
    BFSState::new(start, get_neighbors)
}
