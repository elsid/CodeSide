use std::collections::BTreeMap;

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
    Location,
    Search,
    Simulator,
    TilePathInfo,
    UnitActionWrapper,
    UnitExt,
    Vec2,
    Visitor,
    XorShiftRng,
    as_score,
};

pub struct Plan {
    pub transitions: Vec<Transition>,
    pub score: i32,
    pub simulator: Simulator,
}

#[derive(Clone)]
pub struct Planner<'c, 'p> {
    target: Vec2,
    config: &'c Config,
    paths: &'p BTreeMap<(Location, Location), TilePathInfo>,
    simulator: Simulator,
}

impl<'c, 'p> Planner<'c, 'p> {
    pub fn new(target: Vec2, config: &'c Config, paths: &'p BTreeMap<(Location, Location), TilePathInfo>, simulator: Simulator) -> Self {
        Self { target, config, paths, simulator }
    }

    pub fn make(&self, current_tick: i32, rng: &mut XorShiftRng, debug: &mut Debug) -> Plan {
        let mut visitor = VisitorImpl::new(current_tick, rng, debug);

        let initial_state = visitor.make_initial_state(self.clone());

        log!(current_tick, "state={:?}", initial_state);

        let (transitions, final_state, _iterations) = Search {
            max_iterations: self.config.max_plan_iterations,
        }.perform(current_tick, initial_state, &mut visitor);

        let planner = final_state.map(|v| v.planner).unwrap_or(self.clone());

        for transition in transitions.iter() {
            log!(current_tick, "transition_id={} kind={:?}", transition.id, transition.kind);
        }

        Plan {
            transitions,
            score: planner.get_score(),
            simulator: planner.simulator,
        }
    }

    pub fn get_score(&self) -> i32 {
        as_score(self.get_score_components().iter().sum())
    }

    pub fn get_score_components(&self) -> [f64; 3] {
        let max_distance = self.simulator.world_size().norm();

        let distance_score = max_distance - self.simulator.me().position().distance(self.target) / max_distance;

        let teammates_health = self.simulator.units().iter()
            .filter(|v| v.is_teammate())
            .map(|v| v.health())
            .sum::<i32>();

        let opponnents_health = self.simulator.units().iter()
            .filter(|v| !v.is_teammate())
            .map(|v| v.health())
            .sum::<i32>();

        let health_diff_score = (teammates_health - opponnents_health) as f64
            / (self.simulator.units().len() as i32 * self.simulator.properties().unit_max_health) as f64;

        let my_score = self.simulator.my_player().score;

        let opponents_score = self.simulator.players().iter()
            .filter(|v| v.id != self.simulator.my_player().id)
            .map(|v| v.score)
            .sum::<i32>();

        let game_score_diff_score = (my_score - opponents_score) as f64 / ((self.simulator.players().len() as i32 * self.simulator.properties().kill_score * self.simulator.properties().team_size) as f64 * 1.5);

        [
            distance_score * self.config.plan_distance_score_weight,
            health_diff_score * self.config.plan_health_diff_score_weight,
            game_score_diff_score * self.config.plan_game_score_diff_score_weight,
        ]
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
    current_tick: i32,
    rng: &'r mut XorShiftRng,
    debug: &'r mut Debug<'d>,
    state_id_generator: IdGenerator,
    transition_id_generator: IdGenerator,
}

impl<'r, 'd> VisitorImpl<'r, 'd> {
    pub fn new(current_tick: i32, rng: &'r mut XorShiftRng, debug: &'r mut Debug<'d>) -> Self {
        VisitorImpl {
            current_tick,
            rng,
            debug,
            state_id_generator: IdGenerator::new(),
            transition_id_generator: IdGenerator::new(),
        }
    }

    pub fn make_initial_state<'c, 'p>(&mut self, planner: Planner<'c, 'p>) -> State<'c, 'p> {
        State::initial(self.state_id_generator.next(), planner)
    }
}

impl<'r, 'c, 'd, 'p> Visitor<State<'c, 'p>, Transition> for VisitorImpl<'r, 'd> {
    fn is_final(&self, state: &State) -> bool {
        true
    }

    fn get_transitions(&mut self, state: &State) -> Vec<Transition> {
        let mut result = Vec::new();

        result.push(Transition::left(self.transition_id_generator.next(), state.properties()));
        result.push(Transition::right(self.transition_id_generator.next(), state.properties()));
        result.push(Transition::jump_left(self.transition_id_generator.next(), state.properties()));
        result.push(Transition::jump_right(self.transition_id_generator.next(), state.properties()));
        result.push(Transition::jump(self.transition_id_generator.next()));
        result.push(Transition::jump_down(self.transition_id_generator.next()));
        result.push(Transition::idle(self.transition_id_generator.next()));

        result
    }

    fn apply(&mut self, iteration: usize, state: &State<'c, 'p>, transition: &Transition) -> State<'c, 'p> {
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

        #[cfg(feature = "enable_debug")]
        self.debug.draw(CustomData::Line {
            p1: state.me().position().as_model_f32(),
            p2: next.me().position().as_model_f32(),
            width: 0.1,
            color: ColorF32 { r: 0.25, g: 0.25, b: 0.75, a: 0.25 },
        });

        log!(self.current_tick, "transition_id={} kind={:?}", transition.id, transition.kind);
        log!(self.current_tick, "state={:?}", next);

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
pub struct State<'c, 'p> {
    id: i32,
    score: i32,
    planner: Planner<'c, 'p>,
    depth: usize,
}

impl<'c, 'p> State<'c, 'p> {
    pub fn initial(id: i32, planner: Planner<'c, 'p>) -> Self {
        Self {
            id,
            score: 0,
            planner,
            depth: 0,
        }
    }

    pub fn planner(&self) -> &Planner<'c, 'p> {
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

impl<'c, 'p> std::fmt::Debug for State<'c, 'p> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "id={} position={:?} score={:?}", self.id, self.planner.simulator.me().position(), self.planner.get_score_components())
    }
}

impl<'c, 'p> Identifiable for State<'c, 'p> {
    fn id(&self) -> i32 {
        self.id
    }
}

#[derive(Clone, Debug)]
pub struct Transition {
    pub id: i32,
    pub kind: TransitionKind,
    pub action: UnitAction,
}

impl Transition {
    pub fn left(id: i32, properties: &Properties) -> Self {
        Self {
            id,
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

    pub fn right(id: i32, properties: &Properties) -> Self {
        Self {
            id,
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

    pub fn jump_left(id: i32, properties: &Properties) -> Self {
        Self {
            id,
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

    pub fn jump_right(id: i32, properties: &Properties) -> Self {
        Self {
            id,
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

    pub fn jump(id: i32) -> Self {
        Self {
            id,
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

    pub fn jump_down(id: i32) -> Self {
        Self {
            id,
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

    pub fn idle(id: i32) -> Self {
        Self {
            id,
            kind: TransitionKind::Idle,
            action: UnitAction {
                velocity: 0.0,
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
    Left = 1,
    Right = 2,
    Jump = 3,
    JumpLeft = 4,
    JumpRight = 5,
    JumpDown = 6,
    Idle = 7,
}
