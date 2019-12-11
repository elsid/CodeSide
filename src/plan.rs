use model::{
    Properties,
    UnitAction,
    Vec2F64,
};

#[cfg(feature = "enable_debug")]
use model::{
    ColorF32,
    CustomData,
};

use crate::Debug;

use crate::my_strategy::{
    Clamp1,
    Config,
    IdGenerator,
    Identifiable,
    Rng,
    Search,
    Simulator,
    UnitActionWrapper,
    UnitExt,
    Vec2,
    Visitor,
    WeightedIndex,
    XorShiftRng,
    as_score,
};

pub struct Plan {
    pub transitions: Vec<Transition>,
    pub score: i32,
    pub simulator: Simulator,
}

#[derive(Clone)]
pub struct Planner<'c> {
    target: Vec2,
    config: &'c Config,
    simulator: Simulator,
}

impl<'c> Planner<'c> {
    pub fn new(target: Vec2, config: &'c Config, simulator: Simulator) -> Self {
        Self { target, config, simulator }
    }

    pub fn make(&self, rng: &mut XorShiftRng, debug: &mut Debug) -> Plan {
        let mut visitor = VisitorImpl::new(rng, debug);

        let initial_state = visitor.make_initial_state(self.clone());

        let (transitions, final_state, _iterations) = Search {
            max_iterations: self.config.max_plan_iterations,
        }.perform(initial_state, &mut visitor);

        let planner = final_state.map(|v| v.planner).unwrap_or(self.clone());

        Plan {
            transitions,
            score: planner.get_score(),
            simulator: planner.simulator,
        }
    }

    pub fn get_score(&self) -> i32 {
        let distance = self.simulator.me().position().distance(self.target);

        let teammates_health = self.simulator.units().iter()
            .filter(|v| v.is_teammate())
            .map(|v| v.health())
            .sum::<i32>();

        let opponnents_health = self.simulator.units().iter()
            .filter(|v| !v.is_teammate())
            .map(|v| v.health())
            .sum::<i32>();

        let health_diff = (teammates_health - opponnents_health) as f64
            / (self.simulator.units().len() as i32 * self.simulator.properties().unit_max_health) as f64;

        as_score(
            distance * self.config.distance_score_weight
            + health_diff * self.config.health_diff_score_weight
        )
    }

    pub fn properties(&self) -> &Properties {
        self.simulator.properties()
    }

    pub fn target(&self) -> Vec2 {
        self.target
    }

    pub fn simulator(&self) -> &Simulator {
        &self.simulator
    }
}

pub struct VisitorImpl<'r, 'd> {
    rng: &'r mut XorShiftRng,
    debug: &'r mut Debug<'d>,
    state_id_generator: IdGenerator,
}

impl<'r, 'd> VisitorImpl<'r, 'd> {
    pub fn new(rng: &'r mut XorShiftRng, debug: &'r mut Debug<'d>) -> Self {
        VisitorImpl {
            rng,
            debug,
            state_id_generator: IdGenerator::new(),
        }
    }

    pub fn make_initial_state<'c>(&mut self, planner: Planner<'c>) -> State<'c> {
        State::initial(self.state_id_generator.next(), planner)
    }
}

impl<'r, 'c, 'd> Visitor<State<'c>, Transition> for VisitorImpl<'r, 'd> {
    fn is_final(&self, state: &State) -> bool {
        true
    }

    fn get_transitions(&mut self, state: &State) -> Vec<Transition> {
        let mut result = Vec::new();

        match state.allowed_transitions.current() {
            TransitionKind::None => {
                result.push(Transition::left(state.properties()));
                result.push(Transition::right(state.properties()));
                result.push(Transition::jump_left(state.properties()));
                result.push(Transition::jump_right(state.properties()));
                result.push(Transition::jump());
                result.push(Transition::jump_down());
            },
            TransitionKind::Left => {
                result.push(Transition::left(state.properties()));
                result.push(Transition::jump_left(state.properties()));
                result.push(Transition::jump());
                result.push(Transition::jump_down());
            },
            TransitionKind::Right => {
                result.push(Transition::right(state.properties()));
                result.push(Transition::jump_right(state.properties()));
                result.push(Transition::jump());
                result.push(Transition::jump_down());
            },
            TransitionKind::Jump => {
                result.push(Transition::left(state.properties()));
                result.push(Transition::right(state.properties()));
                result.push(Transition::jump_left(state.properties()));
                result.push(Transition::jump_right(state.properties()));
                result.push(Transition::jump());
            },
            TransitionKind::JumpRight => {
                result.push(Transition::right(state.properties()));
                result.push(Transition::jump_right(state.properties()));
                result.push(Transition::jump());
            },
            TransitionKind::JumpLeft => {
                result.push(Transition::left(state.properties()));
                result.push(Transition::jump_left(state.properties()));
                result.push(Transition::jump());
            },
            TransitionKind::JumpDown => {
                result.push(Transition::left(state.properties()));
                result.push(Transition::right(state.properties()));
                result.push(Transition::jump_down());
            },
        }

        self.rng.shuffle(&mut result[..]);

        result
    }

    fn apply(&mut self, iteration: usize, state: &State<'c>, transition: &Transition) -> State<'c> {
        let mut next = state.clone();
        let time_interval = 1.0 / state.properties().ticks_per_second as f64;
        next.id = self.state_id_generator.next();
        next.depth += 1;
        *next.planner.simulator.me_mut().action_mut() = transition.action.clone();
        let min = state.planner.config.min_ticks_per_transition;
        let max = state.planner.config.max_ticks_per_transition;
        for _ in 0..next.depth.clamp1(min, max) {
            next.planner.simulator.tick(time_interval, state.planner.config.microticks_per_tick, self.rng);
        }
        next.allowed_transitions.next(transition.kind, &mut self.rng);

        #[cfg(feature = "enable_debug")]
        self.debug.draw(CustomData::Line {
            p1: state.me().position().as_model_f32(),
            p2: next.me().position().as_model_f32(),
            width: 0.1,
            color: ColorF32 { r: 0.25, g: 0.25, b: 0.75, a: 0.25 },
        });

        next
    }

    fn get_transition_cost(&mut self, source_state: &State, destination_state: &State, transition: &Transition) -> i32 {
        source_state.get_score() - destination_state.get_score()
    }

    fn get_score(&self, state: &State) -> i32 {
        state.get_score()
    }
}

#[derive(Clone)]
pub struct State<'c> {
    id: i32,
    score: i32,
    planner: Planner<'c>,
    transition: TransitionKind,
    depth: usize,
    allowed_transitions: TransitionsAutotomaton,
}

impl<'c> State<'c> {
    pub fn initial(id: i32, planner: Planner<'c>) -> Self {
        Self {
            id,
            score: 0,
            planner,
            transition: TransitionKind::None,
            depth: 0,
            allowed_transitions: TransitionsAutotomaton::new(),
        }
    }

    pub fn planner(&self) -> &Planner<'c> {
        &self.planner
    }

    pub fn properties(&self) -> &Properties {
        self.planner.properties()
    }

    pub fn get_score(&self) -> i32 {
        self.planner.get_score()
    }

    pub fn target(&self) -> Vec2 {
        self.planner.target()
    }

    pub fn me(&self) -> &UnitExt {
        self.planner.simulator().me()
    }
}

impl<'c> std::fmt::Debug for State<'c> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl<'c> Identifiable for State<'c> {
    fn id(&self) -> i32 {
        self.id
    }
}

#[derive(Clone, Debug)]
pub struct Transition {
    pub kind: TransitionKind,
    pub action: UnitAction,
}

impl Transition {
    pub fn left(properties: &Properties) -> Self {
        Self {
            kind: TransitionKind::Left,
            action: UnitAction {
                velocity: -properties.unit_max_horizontal_speed,
                jump: false,
                jump_down: false,
                aim: Vec2F64 {
                    x: 0.0,
                    y: 0.0
                },
                shoot: false,
                reload: false,
                swap_weapon: false,
                plant_mine: false,
            }
        }
    }

    pub fn right(properties: &Properties) -> Self {
        Self {
            kind: TransitionKind::Right,
            action: UnitAction {
                velocity: properties.unit_max_horizontal_speed,
                jump: false,
                jump_down: false,
                aim: Vec2F64 {
                    x: 0.0,
                    y: 0.0
                },
                shoot: false,
                reload: false,
                swap_weapon: false,
                plant_mine: false,
            }
        }
    }

    pub fn jump_left(properties: &Properties) -> Self {
        Self {
            kind: TransitionKind::JumpLeft,
            action: UnitAction {
                velocity: -properties.unit_max_horizontal_speed,
                jump: true,
                jump_down: false,
                aim: Vec2F64 {
                    x: 0.0,
                    y: 0.0
                },
                shoot: false,
                reload: false,
                swap_weapon: false,
                plant_mine: false,
            }
        }
    }

    pub fn jump_right(properties: &Properties) -> Self {
        Self {
            kind: TransitionKind::JumpRight,
            action: UnitAction {
                velocity: properties.unit_max_horizontal_speed,
                jump: true,
                jump_down: false,
                aim: Vec2F64 {
                    x: 0.0,
                    y: 0.0
                },
                shoot: false,
                reload: false,
                swap_weapon: false,
                plant_mine: false,
            }
        }
    }

    pub fn jump() -> Self {
        Self {
            kind: TransitionKind::Jump,
            action: UnitAction {
                velocity: 0.0,
                jump: true,
                jump_down: false,
                aim: Vec2F64 {
                    x: 0.0,
                    y: 0.0
                },
                shoot: false,
                reload: false,
                swap_weapon: false,
                plant_mine: false,
            }
        }
    }

    pub fn jump_down() -> Self {
        Self {
            kind: TransitionKind::JumpDown,
            action: UnitAction {
                velocity: 0.0,
                jump: false,
                jump_down: true,
                aim: Vec2F64 {
                    x: 0.0,
                    y: 0.0
                },
                shoot: false,
                reload: false,
                swap_weapon: false,
                plant_mine: false,
            }
        }
    }
}

impl PartialEq for Transition {
    fn eq(&self, other: &Self) -> bool {
        UnitActionWrapper(&self.action).eq(&UnitActionWrapper(&other.action))
    }
}

impl Eq for Transition {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum TransitionKind {
    None = 0,
    Left = 1,
    Right = 2,
    Jump = 3,
    JumpLeft = 4,
    JumpRight = 5,
    JumpDown = 6,
}

impl std::convert::TryFrom<usize> for TransitionKind {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => TransitionKind::Left,
            2 => TransitionKind::Right,
            3 => TransitionKind::Jump,
            4 => TransitionKind::JumpLeft,
            5 => TransitionKind::JumpRight,
            6 => TransitionKind::JumpDown,
            _ => TransitionKind::None,
        })
    }
}

#[derive(Clone, Debug)]
struct TransitionsAutotomaton {
    state: TransitionKind,
    arcs: Vec<WeightedIndex>,
}

impl TransitionsAutotomaton {
    pub fn new() -> Self {
        Self {
            state: TransitionKind::None,
            arcs: vec![
                WeightedIndex::new(vec![1, 0, 0, 0, 0, 0, 0]), // None
                WeightedIndex::new(vec![0, 3, 0, 1, 1, 0, 1]), // Left
                WeightedIndex::new(vec![0, 0, 3, 1, 0, 1, 1]), // Right
                WeightedIndex::new(vec![0, 1, 1, 3, 1, 1, 0]), // Jump
                WeightedIndex::new(vec![0, 1, 0, 1, 3, 0, 0]), // JumpLeft
                WeightedIndex::new(vec![0, 0, 1, 1, 0, 3, 0]), // JumpRight
                WeightedIndex::new(vec![0, 1, 1, 0, 0, 0, 3]), // JumpDown
            ],
        }
    }

    pub fn next(&mut self, transition: TransitionKind, rng: &mut XorShiftRng) {
        use std::convert::TryFrom;
        self.state = TransitionKind::try_from(self.arcs[transition as usize].sample(rng)).unwrap();
    }

    pub fn current(&self) -> TransitionKind {
        self.state
    }
}
