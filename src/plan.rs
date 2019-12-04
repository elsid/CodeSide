use model::{
    ColorF32,
    CustomData,
    Properties,
    UnitAction,
    Vec2F64,
};
use crate::Debug;
use crate::my_strategy::{
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
    XorShiftRng,
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
        -(self.simulator.me().position().distance(self.target) * 1000.0) as i32
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

        match state.transition {
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
        *next.planner.simulator.me_mut().action_mut() = transition.action.clone();
        next.planner.simulator.tick(time_interval, 3, self.rng);

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
        1
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
}

impl<'c> State<'c> {
    pub fn initial(id: i32, planner: Planner<'c>) -> Self {
        Self { id, score: 0, planner, transition: TransitionKind::None }
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
pub enum TransitionKind {
    None,
    Left,
    Right,
    Jump,
    JumpLeft,
    JumpRight,
    JumpDown,
}
