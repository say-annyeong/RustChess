use std::collections::{HashMap, HashSet};
use std::path::Iter;
use crate::position::RelativePosition;

type MovementType = String;
type StateType = String;

/// ## IndependentMove
/// IndependentMove
struct IndependentMove<const D: usize> {
    movement_type: HashSet<MovementType>,
    offest: RelativePosition<D>
}

/// ## DependentMove
/// DependentMove
struct DependentMove<const D: usize> {
    movement_type: IndependentMove<D>,
    start_offest: RelativePosition<D>,
    state: State,
    max_times: usize,
    times: usize
}

impl<const D: usize> Iterator for DependentMove<D> {
    type Item = IndependentMove<D>;

    fn next(&mut self) -> Option<IndependentMove<D>> {
        if self.times >= self.max_times {
            return None;
        }
        self.times += 1;
        self.state.state_define(&self.start_offest, &self.movement_type)
    }
}

pub struct State {
    states: HashMap<StateType, String>,
    states_code: HashMap<StateType, String>,
}

impl State {
    pub fn state_define<const D: usize>(&self, start_move: &RelativePosition<D>, delta_move: &IndependentMove<D>) -> Option<IndependentMove<D>> {
        None
    }
}
