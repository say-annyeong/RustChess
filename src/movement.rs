use std::collections::HashSet;
use std::path::Iter;
use crate::position::RelativePosition;

type MovementType = String;
type State = String;

/// ## IndependentMove
/// IndependentMove
struct IndependentMove<const D: usize> {
    movement_type: HashSet<MovementType>,
    offest: RelativePosition<D>
}

/// ## StateDependentMove
/// StateDependentMove
struct StateDependentMove<const D: usize> {
    movement_type: IndependentMove<D>,
    start_offest: RelativePosition<D>,
    state: State,
    max_times: usize,
    times: usize
}

impl<const D: usize> Iterator for StateDependentMove<D> {
    type Item = IndependentMove<D>;

    fn next(&mut self) -> Option<IndependentMove<D>> {
        if self.times >= self.max_times {
            return None;
        }
        self.times += 1;
        state_define(&self.start_offest, &self.movement_type, &self.state)
    }
}

fn state_define<const D: usize>(start_move: &RelativePosition<D>, delta_move: &IndependentMove<D>, state: &State) -> Option<IndependentMove<D>> {
    None
}
