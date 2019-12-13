use std::time::{
    Instant,
    Duration,
};

use model::{
    Game,
    Item,
    Tile,
    Unit,
    UnitAction,
    Vec2F64,
};

#[cfg(feature = "enable_debug")]
use model::{
    ColorF32,
    CustomData,
    Vec2F32,
};

use crate::Debug;

use crate::my_strategy::{
    Config,
    Planner,
    Positionable,
    Rectangular,
    SeedableRng,
    Simulator,
    Vec2,
    World,
    XorShiftRng,
    get_hit_probability_by_spread,
    get_hit_probability_over_obstacles,
    get_optimal_tile,
    get_weapon_score,
};

#[cfg(feature = "enable_debug")]
use crate::my_strategy::{
    normalize_angle,
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
    max_time_budget_spent: f64,
    max_cpu_time_budget_spent: f64,
}

impl MyStrategyImpl {
    pub fn new(config: Config, me: Unit, game: Game) -> Self {
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
            max_time_budget_spent: 0.0,
            max_cpu_time_budget_spent: 0.0,
        }
    }

    pub fn get_action(&mut self, me: &Unit, game: &Game, debug: &mut Debug) -> UnitAction {
        self.on_start();
        let result = self.get_action_measured(me, game, debug);
        self.on_finish();
        result
    }

    pub fn get_action_measured(&mut self, me: &Unit, game: &Game, debug: &mut Debug) -> UnitAction {
        fn distance_sqr(a: &Vec2F64, b: &Vec2F64) -> f64 {
            (a.x - b.x).powi(2) + (a.y - b.y).powi(2)
        }
        self.world.update(me, game);
        #[cfg(feature = "enable_debug")]
        for unit in self.world.units().iter() {
            if let Some(weapon) = unit.weapon.as_ref() {
                if let Some(last_angle) = weapon.last_angle {
                    let direction = Vec2::i().rotated(normalize_angle(last_angle));
                    let lower_spread = Vec2::i().rotated(normalize_angle(last_angle - weapon.spread));
                    let upper_spread = Vec2::i().rotated(normalize_angle(last_angle + weapon.spread));
                    debug.draw(CustomData::Line {
                        p1: unit.rect().center().as_model_f32(),
                        p2: (unit.rect().center() + direction * 100.0).as_model_f32(),
                        width: 0.1,
                        color: ColorF32 { a: 0.66, r: 0.66, g: 0.0, b: 0.0 },
                    });
                    debug.draw(CustomData::Line {
                        p1: unit.rect().center().as_model_f32(),
                        p2: (unit.rect().center() + lower_spread * 100.0).as_model_f32(),
                        width: 0.1,
                        color: ColorF32 { a: 0.5, r: 0.66, g: 0.0, b: 0.0 },
                    });
                    debug.draw(CustomData::Line {
                        p1: unit.rect().center().as_model_f32(),
                        p2: (unit.rect().center() + upper_spread * 100.0).as_model_f32(),
                        width: 0.1,
                        color: ColorF32 { a: 0.66, r: 0.66, g: 0.0, b: 0.0 },
                    });
                }
            }
        }
        let nearest_opponent = self.world.units().iter()
            .filter(|other| other.player_id != me.player_id)
            .min_by(|a, b| {
                std::cmp::PartialOrd::partial_cmp(
                    &distance_sqr(&a.position, &me.position),
                    &distance_sqr(&b.position, &me.position),
                )
                .unwrap()
            });
        let nearest_weapon = self.world.loot_boxes().iter()
            .filter(|loot| {
                if let Item::Weapon { .. } = loot.item {
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
        let nearest_health_pack = self.world.loot_boxes().iter()
            .filter(|loot| {
                if let Item::HealthPack { .. } = loot.item {
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
        let mut target = Vec2::from_model(&me.position);
        if let Some(location) = get_optimal_tile(&self.world, debug) {
            #[cfg(feature = "enable_debug")]
            debug.draw(CustomData::Rect {
                pos: (location.center() - Vec2::new(0.25, 0.25)).as_model_f32(),
                size: Vec2F32 { x: 0.5, y: 0.5 },
                color: ColorF32 { a: 0.66, r: 0.0, g: 0.0, b: 0.0 },
            });
            #[cfg(feature = "enable_debug")]
            debug.draw(CustomData::Line {
                p1: self.world.me().rect().center().as_model_f32(),
                p2: Vec2F32 { x: location.x() as f32 + 0.5, y: location.y() as f32 + 0.5 },
                width: 0.1,
                color: ColorF32 { a: 0.66, r: 0.0, g: 0.0, b: 0.0 },
            });
            target = Vec2::new(location.x() as f64 + 0.5, location.y() as f64);
        } else if let (&None, Some(weapon)) = (&me.weapon, nearest_weapon) {
            target = weapon.position();
        } else if let (true, Some(health_pack)) = (me.health < self.world.properties().unit_max_health, nearest_health_pack) {
            target = health_pack.position();
        } else if let Some(opponent) = nearest_opponent {
            target = opponent.position();
        }
        #[cfg(feature = "enable_debug")]
        debug.draw(CustomData::Log {
            text: format!("target: {:?}", target),
        });
        let (shoot, aim) = if let (Some(opponent), Some(weapon)) = (nearest_opponent, self.world.me().weapon.as_ref()) {
            let hit_probability_by_spread = get_hit_probability_by_spread(self.world.me().rect().center(), &opponent.rect(), weapon.spread);
            let hit_probability_over_obstacles = get_hit_probability_over_obstacles(&me.rect(), &opponent.rect(), self.world.level());
            #[cfg(feature = "enable_debug")]
            debug.draw(CustomData::Log { text: format!("hit_probability: by_spread={}, over_obstacles={}", hit_probability_by_spread, hit_probability_over_obstacles) });
            (
                hit_probability_by_spread >= self.world.config().min_hit_probability_by_spread_to_shoot
                && hit_probability_over_obstacles >= self.world.config().min_hit_probability_over_obstacles_to_shoot,
                Vec2F64 {
                    x: opponent.position.x - me.position.x,
                    y: opponent.position.y - me.position.y,
                }
            )
        } else {
            (false, model::Vec2F64 { x: 0.0, y: 0.0 })
        };
        let tiles_path = self.world.find_shortcut_tiles_path(self.world.me().location(), target.as_location());
        if !tiles_path.is_empty() {
            target = tiles_path[0].bottom();
            #[cfg(feature = "enable_debug")]
            {
                debug.draw(CustomData::Line {
                    p1: self.world.me().rect().center().as_model_f32(),
                    p2: tiles_path[0].center().as_model_f32(),
                    width: 0.1,
                    color: ColorF32 { a: 0.66, r: 0.66, g: 0.66, b: 0.0 },
                });
                for tile in 0 .. tiles_path.len() - 1 {
                    debug.draw(CustomData::Line {
                        p1: tiles_path[tile].center().as_model_f32(),
                        p2: tiles_path[tile + 1].center().as_model_f32(),
                        width: 0.1,
                        color: ColorF32 { a: 0.66, r: 0.66, g: 0.66, b: 0.0 },
                    });
                }
            }
        }
        let simulator = Simulator::new(&self.world, me.id);
        let plan = Planner::new(target, &self.config, self.world.paths(), simulator)
            .make(&mut self.rng, debug);
        if !plan.transitions.is_empty() {
            #[cfg(feature = "enable_debug")]
            debug.draw(CustomData::Log {
                text: format!("plan_score={}, transitions: {:?}", plan.score, plan.transitions.iter().map(|v| (v.kind, v.id)).collect::<Vec<_>>())
            });
            let mut action = plan.transitions[0].action.clone();
            action.shoot = shoot;
            action.aim = aim;
            action.swap_weapon = !shoot && self.should_swap_weapon();
            return action;
        }
        let mut jump = target.y() > me.position.y;
        if target.x() > me.position.x
            && self.world.game().level.tiles[(me.position.x + 1.0) as usize][(me.position.y) as usize]
                == Tile::Wall
        {
            jump = true
        }
        if target.x() < me.position.x
            && self.world.game().level.tiles[(me.position.x - 1.0) as usize][(me.position.y) as usize]
                == Tile::Wall
        {
            jump = true
        }
        UnitAction {
            velocity: target.x() - me.position.x,
            jump,
            jump_down: target.y() < me.position.y,
            shoot,
            aim,
            reload: false,
            swap_weapon: !shoot && self.should_swap_weapon(),
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
        self.max_cpu_time_budget_spent = self.max_cpu_time_budget_spent.max(time_bugdet_spent(self.world.game().current_tick, &self.cpu_time_spent));
        self.max_time_budget_spent = self.max_time_budget_spent.max(time_bugdet_spent(self.world.game().current_tick, &self.time_spent));

        #[cfg(not(feature = "disable_output"))]
        {
            if self.max_cpu_time_budget_spent > 90.0 || self.max_time_budget_spent > 90.0 {
                eprintln!(
                    "{} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
                    self.world.game().current_tick, self.time_spent, self.cpu_time_spent, self.max_cpu_time_spent,
                    time_bugdet_spent(self.world.game().current_tick, &self.cpu_time_spent),
                    time_bugdet_spent(self.world.game().current_tick, &self.time_spent),
                    self.max_time_budget_spent,
                    self.max_cpu_time_budget_spent
                );
            }
        }
    }

    fn should_swap_weapon(&self) -> bool {
        if let Some(weapon) = self.world.me().weapon.as_ref() {
            match self.world.tile_item(self.world.me().location()) {
                Some(&Item::Weapon { ref weapon_type }) => {
                    get_weapon_score(&weapon.typ) < get_weapon_score(weapon_type)
                }
                _ => false,
            }
        } else {
            false
        }
    }
}

impl Drop for MyStrategyImpl {
    fn drop(&mut self) {
        #[cfg(not(feature = "disable_output"))]
        eprintln!(
            "{} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
            self.world.game().current_tick, self.time_spent, self.cpu_time_spent, self.max_cpu_time_spent,
            time_bugdet_spent(self.world.game().current_tick, &self.cpu_time_spent),
            time_bugdet_spent(self.world.game().current_tick, &self.time_spent),
            self.max_time_budget_spent,
            self.max_cpu_time_budget_spent
        );
    }
}

fn time_bugdet_spent(current_tick: i32, time_spent: &Duration) -> f64 {
    time_spent.as_secs_f64() / ((current_tick * 20 + 20000) as f64 / 1000.0) * 100.0
}
