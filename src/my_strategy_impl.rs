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
    get_hit_probability,
};

#[cfg(feature = "dump_level")]
use crate::my_strategy::dump_level;

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
        #[cfg(feature = "dump_level")]
        println!("{}", dump_level(&game.level));
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
        let nearest_opponent = self.world.game()
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
        let nearest_health_pack = self.world.game()
            .loot_boxes
            .iter()
            .filter(|loot| {
                if let model::Item::HealthPack { .. } = loot.item {
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
        } else if let (true, Some(health_pack)) = (me.health < self.world.properties().unit_max_health, nearest_health_pack) {
            target_pos = health_pack.position.clone();
        } else if let Some(opponent) = nearest_opponent {
            target_pos = opponent.position.clone();
        }
        #[cfg(feature = "enable_debug")]
        debug.draw(model::CustomData::Log {
            text: format!("Target pos: {:?}", target_pos),
        });
        let aim: Option<model::Vec2F64> = if let Some(opponent) = nearest_opponent {
            if get_hit_probability(&me, opponent, self.world.level()) > 0.0 {
                Some(model::Vec2F64 {
                    x: opponent.position.x - me.position.x,
                    y: opponent.position.y - me.position.y,
                })
            } else {
                None
            }
        } else {
            None
        };
        let simulator = Simulator::new(&self.world, me.id);
        let plan = Planner::new(Vec2::from_model(&target_pos), &self.config, simulator).make(&mut self.rng, debug);
        if !plan.transitions.is_empty() {
            #[cfg(feature = "enable_debug")]
            debug.draw(model::CustomData::Log {
                text: format!("has_plan: score={}", plan.score),
            });
            let mut action = plan.transitions[0].action.clone();
            action.shoot = aim.is_some();
            action.aim = aim.unwrap_or(model::Vec2F64 { x: 0.0, y: 0.0 });
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
            shoot: aim.is_some(),
            aim: aim.unwrap_or(model::Vec2F64 { x: 0.0, y: 0.0 }),
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
