use std::collections::BinaryHeap;

pub struct State<State: Clone + std::fmt::Debug> {
    pub id: i32,
    pub cost: i32,
    pub score: i32,
    pub state: State,
    pub transition: Option<usize>,
}

pub trait Visitor<State, Transition> {
    fn is_final(&self, state: &State) -> bool;
    fn get_transitions(&mut self, iteration: usize, state: &State) -> Vec<Transition>;
    fn apply(&mut self, iteration: usize, state: &State, transition: &Transition) -> State;
    fn get_score(&self, state: &State) -> i32;
}

pub trait Identifiable {
    fn id(&self) -> i32;
}

pub fn search<S, T, V>(current_tick: i32, initial: S, visitor: &mut V) -> (Vec<T>, Option<S>, usize)
    where S: Clone + std::fmt::Debug + Identifiable,
            T: Clone + std::fmt::Debug,
            V: Visitor<S, T> {

    let mut iteration: usize = 0;
    let mut transitions = Vec::new();
    let mut frontier = BinaryHeap::new();

    let initial_state = State {
        id: initial.id(),
        cost: 0,
        score: visitor.get_score(&initial),
        state: initial,
        transition: None,
    };

    frontier.push(initial_state);

    let mut optimal_final_state: Option<State<S>> = None;

    while let Some(state) = frontier.pop() {
        log!(current_tick, "[{}] pop cost={}", state.id, state.cost);
        if (optimal_final_state.is_none() || optimal_final_state.as_ref().unwrap().score < state.score)
            && visitor.is_final(&state.state) {
            optimal_final_state = Some(state.clone());
            log!(current_tick, "optimal_final_state={:?}", state.state);
        }
        iteration += 1;
        for transition in visitor.get_transitions(iteration, &state.state) {
            let next_state = visitor.apply(iteration, &state.state, &transition);
            let next_state_score = visitor.get_score(&next_state);
            let next_search_state = State {
                id: next_state.id(),
                cost: state.cost + state.score - next_state_score,
                score: next_state_score,
                state: next_state,
                transition: Some(transitions.len()),
            };
            log!(current_tick, "[{}] push cost={}", next_search_state.id, next_search_state.cost);
            frontier.push(next_search_state);
            transitions.push((state.transition, transition));
        }
    }

    (
        reconstruct_sequence(&transitions, &optimal_final_state),
        optimal_final_state.map(|v| v.state),
        iteration
    )
}

impl<S> Clone for State<S>
    where S: Clone + std::fmt::Debug + Identifiable {

    fn clone(&self) -> Self {
        State {
            id: self.id,
            cost: self.cost,
            score: self.score,
            state: self.state.clone(),
            transition: self.transition.clone(),
        }
    }
}

impl<S> std::fmt::Debug for State<S>
    where S: Clone + std::fmt::Debug + Identifiable {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} {}", self.state, self.id)
    }
}

impl<S> std::cmp::PartialEq for State<S>
    where S: Clone + std::fmt::Debug + Identifiable {

    fn eq(&self, other: &Self) -> bool {
        (self.cost, self.id).eq(&(other.cost, other.id))
    }
}

impl<S> std::cmp::Eq for State<S>
    where S: Clone + std::fmt::Debug + Identifiable {}

impl<S> std::cmp::PartialOrd for State<S>
    where S: Clone + std::fmt::Debug + Identifiable {

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.cost, self.id).partial_cmp(&(other.cost, other.id)).map(|v| v.reverse())
    }
}

impl<S> std::cmp::Ord for State<S>
    where S: Clone + std::fmt::Debug + Identifiable {

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.cost, self.id).cmp(&(other.cost, other.id)).reverse()
    }
}

fn reconstruct_sequence<S, T>(transitions: &Vec<(Option<usize>, T)>, final_state: &Option<State<S>>) -> Vec<T>
    where S: Clone + std::fmt::Debug,
          T: Clone {

    if final_state.is_none() || final_state.as_ref().unwrap().transition.is_none() {
        return Vec::new();
    }
    let mut current = final_state.as_ref().unwrap().transition.unwrap();
    let mut result = Vec::new();
    loop {
        let (prev, ref transition) = transitions[current];
        result.push(transition.clone());
        if let Some(prev_value) = prev {
            current = prev_value;
        } else {
            break;
        }
    }
    result.reverse();
    result
}
