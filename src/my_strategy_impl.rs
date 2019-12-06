use std::time::{Instant, Duration};
use crate::Debug;
use crate::my_strategy::{
    Config,
    Planner,
    SeedableRng,
    Simulator,
    Vec2,
    World,
    XorShiftRng,
};

pub struct MyStrategyImpl {
    config: Config,
    world: World,
    rng: XorShiftRng,
    start_time: Instant,
    tick_start_time: Instant,
    time_spent: Duration,
    cpu_time_spent: Duration,
    max_cpu_time_spent: Duration,
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
            start_time: Instant::now(),
            tick_start_time: Instant::now(),
            time_spent: Duration::default(),
            cpu_time_spent: Duration::default(),
            max_cpu_time_spent: Duration::default(),
        }
    }

    pub fn get_action(&mut self, me: &model::Unit, game: &model::Game, debug: &mut Debug) -> model::UnitAction {
        self.on_start();
        let result = self.get_action_measured(me, game, debug);
        self.on_finish();
        result
    }

    pub fn get_action_measured(&mut self, me: &model::Unit, game: &model::Game, debug: &mut Debug) -> model::UnitAction {
        fn distance_sqr(a: &model::Vec2F64, b: &model::Vec2F64) -> f64 {
            (a.x - b.x).powi(2) + (a.y - b.y).powi(2)
        }
        self.world.update(me, game);
        let nearest_enemy = self.world.game()
            .units
            .iter()
            .filter(|other| other.player_id != me.player_id)
            .min_by(|a, b| {
                std::cmp::PartialOrd::partial_cmp(
                    &distance_sqr(&a.position, &me.position),
                    &distance_sqr(&b.position, &me.position),
                )
                .unwrap()
            });
        let nearest_weapon = self.world.game()
            .loot_boxes
            .iter()
            .filter(|loot| {
                if let model::Item::Weapon { .. } = loot.item {
                    true
                } else {
                    false
                }
            })
            .min_by(|a, b| {
                std::cmp::PartialOrd::partial_cmp(
                    &distance_sqr(&a.position, &me.position),
                    &distance_sqr(&b.position, &me.position),
                )
                .unwrap()
            });
        let mut target_pos = me.position.clone();
        if let (&None, Some(weapon)) = (&me.weapon, nearest_weapon) {
            target_pos = weapon.position.clone();
        } else if let Some(enemy) = nearest_enemy {
            target_pos = enemy.position.clone();
        }
        debug.draw(model::CustomData::Log {
            text: format!("Target pos: {:?}", target_pos),
        });
        let mut aim = model::Vec2F64 { x: 0.0, y: 0.0 };
        if let Some(enemy) = nearest_enemy {
            aim = model::Vec2F64 {
                x: enemy.position.x - me.position.x,
                y: enemy.position.y - me.position.y,
            };
        }
        let simulator = Simulator::new(&self.world, me.id);
        let plan = Planner::new(Vec2::from_model(&target_pos), &self.config, simulator).make(&mut self.rng, debug);
        if !plan.transitions.is_empty() {
            debug.draw(model::CustomData::Log {
                text: format!("has_plan: score={}", plan.score),
            });
            let mut action = plan.transitions[0].action.clone();
            action.aim = aim;
            action.shoot = true;
            return action;
        }
        let mut jump = target_pos.y > me.position.y;
        if target_pos.x > me.position.x
            && self.world.game().level.tiles[(me.position.x + 1.0) as usize][(me.position.y) as usize]
                == model::Tile::Wall
        {
            jump = true
        }
        if target_pos.x < me.position.x
            && self.world.game().level.tiles[(me.position.x - 1.0) as usize][(me.position.y) as usize]
                == model::Tile::Wall
        {
            jump = true
        }
        model::UnitAction {
            velocity: target_pos.x - me.position.x,
            jump,
            jump_down: target_pos.y < me.position.y,
            aim,
            shoot: true,
            reload: false,
            swap_weapon: false,
            plant_mine: false,
        }
    }

    fn on_start(&mut self) {
        self.tick_start_time = Instant::now();
    }

    fn on_finish(&mut self) {
        let finish = Instant::now();
        let cpu_time_spent = finish - self.tick_start_time;
        self.max_cpu_time_spent = self.max_cpu_time_spent.max(cpu_time_spent);
        self.cpu_time_spent += cpu_time_spent;
        self.time_spent = finish - self.start_time;
    }
}

impl Drop for MyStrategyImpl {
    fn drop(&mut self) {
        #[cfg(not(feature = "disable_output"))]
        eprintln!("{} {:?} {:?} {:?}", self.world.game().current_tick, self.time_spent, self.cpu_time_spent, self.max_cpu_time_spent);
    }
}
