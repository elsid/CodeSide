use model::{
    Game,
    Unit,
    UnitAction,
    Vec2F64,
};
use crate::Debug;
use crate::my_strategy::config::Config;
use crate::my_strategy::plan::Planner;
use crate::my_strategy::random::{XorShiftRng, SeedableRng};
use crate::my_strategy::simulator::Simulator;
use crate::my_strategy::vec2::Vec2;
use crate::my_strategy::world::World;

pub struct MyStrategyImpl {
    config: Config,
    world: World,
    rng: XorShiftRng,
}

impl MyStrategyImpl {
    pub fn new(config: Config, me: model::Unit, game: model::Game) -> Self {
        Self {
            config: config.clone(),
            world: World::new(config, me, game),
            rng: XorShiftRng::from_seed([
                3918248293,
                2127433321,
                1841971383,
                1904458926,
            ]),
        }
    }

    pub fn get_action(&mut self, me: &Unit, game: &Game, debug: &mut Debug) -> UnitAction {
        self.world.update(me, game);
        let target = Vec2::from_model(
            &self.world.units().iter()
                .find(|v| v.player_id != me.player_id).unwrap().position
        );
        let simulator = Simulator::new(&self.world, me.id);
        let plan = Planner::new(target, &self.config, simulator).make(&mut self.rng);
        if !plan.transitions.is_empty() {
            return plan.transitions[0].action.clone();
        }
        UnitAction {
            velocity: 0.0,
            jump: false,
            jump_down: false,
            aim: Vec2F64 {
                x: 0.0,
                y: 0.0
            },
            shoot: false,
            swap_weapon: false,
            plant_mine: false,
        }
    }
}
