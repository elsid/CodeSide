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
    TextAlignment,
    Vec2F32,
};

use crate::Debug;

use crate::my_strategy::{
    Config,
    Location,
    Planner,
    Positionable,
    Rectangular,
    SeedableRng,
    Simulator,
    Vec2,
    World,
    XorShiftRng,
    get_optimal_tile,
    get_weapon_score,
    should_shoot,
};

#[cfg(feature = "enable_debug")]
use crate::my_strategy::{
    Object,
    Target,
    WalkGrid,
    as_score,
    get_nearest_hit,
    get_tile_location,
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
    optimal_tiles: Vec<Option<(f64, Location)>>,
    calls_per_tick: usize,
    last_tick: i32,
    slow_down: bool,
}

impl MyStrategyImpl {
    pub fn new(config: Config, me: Unit, game: Game) -> Self {
        #[cfg(feature = "dump_level")]
        println!("{}", dump_level(&game.level));
        let world = World::new(config.clone(), me, game);
        Self {
            config,
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
            optimal_tiles: std::iter::repeat(None).take(world.properties().team_size as usize).collect(),
            world,
            calls_per_tick: 0,
            last_tick: -1,
            slow_down: false,
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
        if self.last_tick != game.current_tick {
            self.last_tick = game.current_tick;
            self.world.update(game);
        }
        self.world.update_me(me);
        #[cfg(feature = "enable_debug")]
        {
            debug.draw(CustomData::PlacedText {
                text: format!("{}", me.id),
                pos: (me.position() + Vec2::only_y(me.size.y)).as_model_f32(),
                alignment: TextAlignment::Center,
                size: 40.0,
                color: ColorF32 { a: 1.0, r: 1.0, g: 1.0, b: 1.0 },
            });
        }
        #[cfg(feature = "enable_debug")]
        for unit in self.world.units().iter() {
            if let Some(weapon) = unit.weapon.as_ref() {
                if let Some(last_angle) = weapon.last_angle {
                    let direction = Vec2::i().rotated(normalize_angle(last_angle));
                    let lower_spread = Vec2::i().rotated(normalize_angle(last_angle - weapon.spread));
                    let upper_spread = Vec2::i().rotated(normalize_angle(last_angle + weapon.spread));
                    debug.draw(CustomData::Line {
                        p1: unit.rect().center().as_model_f32(),
                        p2: (unit.rect().center() + direction * self.world.max_distance()).as_model_f32(),
                        width: 0.1,
                        color: ColorF32 { a: 0.5, r: 0.66, g: 0.0, b: 0.0 },
                    });
                    debug.draw(CustomData::Line {
                        p1: unit.rect().center().as_model_f32(),
                        p2: (unit.rect().center() + lower_spread * self.world.max_distance()).as_model_f32(),
                        width: 0.1,
                        color: ColorF32 { a: 0.5, r: 0.66, g: 0.0, b: 0.0 },
                    });
                    debug.draw(CustomData::Line {
                        p1: unit.rect().center().as_model_f32(),
                        p2: (unit.rect().center() + upper_spread * self.world.max_distance()).as_model_f32(),
                        width: 0.1,
                        color: ColorF32 { a: 0.5, r: 0.66, g: 0.0, b: 0.0 },
                    });
                }
            }
        }
        #[cfg(feature = "enable_debug")]
        {
            let backtrack = self.world.backtrack();
            for i in 0 .. backtrack.len() {
                if backtrack[i] == i {
                    continue;
                }
                let dst = get_tile_location(self.world.level(), i).center();
                let src = get_tile_location(self.world.level(), backtrack[i]).center();
                debug.draw(CustomData::Line {
                    p1: get_tile_location(self.world.level(), i).center().as_model_f32(),
                    p2: get_tile_location(self.world.level(), backtrack[i]).center().as_model_f32(),
                    width: 0.05,
                    color: ColorF32 { a: 0.66, r: 0.66, g: 0.66, b: 0.33 },
                });
            }
        }
        let nearest_opponent = self.world.units().iter()
            .filter(|unit| self.world.is_opponent(unit))
            .filter(|unit| {
                if let Some(weapon) = self.world.me().weapon.as_ref() {
                    should_shoot(&self.world.me().rect(), &unit, weapon, &self.world, true)
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
        let optimal_tile_target = if !self.slow_down {
            if let Some((score, location)) = get_optimal_tile(&self.world, &self.optimal_tiles, debug) {
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
                    color: ColorF32 { a: 0.66, r: 0.0, g: 0.66, b: 0.0 },
                });
                self.optimal_tiles[self.world.me_index()] = Some((score, location));
                Some(Vec2::new(location.x() as f64 + 0.5, location.y() as f64))
            } else {
                None
            }
        } else {
            if let Some((_, location)) = self.optimal_tiles[self.world.me_index()] {
                Some(Vec2::new(location.x() as f64 + 0.5, location.y() as f64))
            } else {
                None
            }
        };
        let global_target = if let Some(v) = optimal_tile_target {
            v
        } else {
            if let (&None, Some(weapon)) = (&me.weapon, nearest_weapon) {
                weapon.position()
            } else if let (true, Some(health_pack)) = (me.health < self.world.properties().unit_max_health, nearest_health_pack) {
                health_pack.position()
            } else {
                me.position()
            }
        };
        let (shoot, aim) = if let Some(opponent) = nearest_opponent {
            #[cfg(feature = "enable_debug")]
            {
                let mut s = Vec::new();
                for position in WalkGrid::new(me.rect().center(), opponent.rect().center()) {
                    s.push(position);
                    debug.draw(CustomData::Rect {
                        pos: position.as_location().as_model_f32(),
                        size: Vec2F32 { x: 1.0, y: 1.0 },
                        color: ColorF32 { a: 0.5, r: 0.66, g: 0.0, b: 0.66 },
                    });
                }
                if let Some(weapon) = me.weapon.as_ref() {
                    let source = me.rect().center();
                    let direction = (opponent.rect().center() - source).normalized();
                    let to_target = direction * self.world.max_distance();
                    let left = direction.left() * weapon.params.bullet.size;
                    let right = direction.right() * weapon.params.bullet.size;
                    let number_of_directions = self.world.config().hit_number_of_directions;
                    let target = Target::from_unit(opponent);

                    for i in 0 .. number_of_directions {
                        let angle = ((2 * i) as f64 / (number_of_directions - 1) as f64 - 1.0) * weapon.spread;
                        let destination = source + to_target.rotated(angle);
                        let rays = [
                            (source, destination),
                            (source + left, destination + left),
                            (source + right, destination + right),
                        ];
                        for (src, dst, hit) in rays.into_iter().filter_map(|&(src, dst)| get_nearest_hit(me.id, src, dst, &target, &self.world).map(|v| (src, dst, v))).min_by_key(|(_, _, v)| as_score(v.distance)) {
                            let color = match hit.object {
                                Object::Wall => ColorF32 { a: 0.33, r: 0.66, g: 0.66, b: 0.66 },
                                Object::OpponentUnit => ColorF32 { a: 0.33, r: 0.0, g: 0.66, b: 0.33 },
                                Object::TeammateUnit => ColorF32 { a: 0.33, r: 0.66, g: 0.33, b: 0.0 },
                                Object::OpponentMine => ColorF32 { a: 0.33, r: 0.5, g: 0.33, b: 0.0 },
                                Object::TeammateMine => ColorF32 { a: 0.33, r: 0.33, g: 0.5, b: 0.0 },
                            };
                            #[cfg(feature = "enable_debug")]
                            debug.draw(CustomData::Line {
                                p1: source.as_model_f32(),
                                p2: (source + (destination - source).normalized() * hit.distance).as_model_f32(),
                                width: 0.075,
                                color,
                            });
                        }
                    }
                }
            }
            (
                true,
                Vec2F64 {
                    x: opponent.position.x - me.position.x,
                    y: opponent.position.y - me.position.y,
                }
            )
        } else {
            (false, model::Vec2F64 { x: 0.0, y: 0.0 })
        };
        let tiles_path = self.world.find_shortcut_tiles_path(self.world.me().location(), global_target.as_location());
        let local_target = if !tiles_path.is_empty() {
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
            tiles_path[0].bottom()
        } else {
            global_target
        };

        #[cfg(feature = "enable_debug")]
        debug.draw(CustomData::Log { text: format!("global_target: {:?} local_target: {:?}", global_target, local_target) });

        let simulator = Simulator::new(&self.world, me.id);
        let plan = Planner::new(local_target, &self.config, self.world.paths(), simulator)
            .make(game.current_tick, &mut self.rng, debug);
        if !plan.transitions.is_empty() {
            #[cfg(feature = "enable_debug")]
            debug.draw(CustomData::Log {
                text: format!("plan_score={}, transitions: {:?}", plan.score, plan.transitions.iter().map(|v| (v.kind, v.id)).collect::<Vec<_>>())
            });
            let mut action = plan.transitions[0].action.clone();
            action.shoot = shoot;
            action.aim = aim;
            action.swap_weapon = self.should_swap_weapon(shoot);
            #[cfg(feature = "enable_debug")]
            debug.draw(CustomData::Log { text: format!("action: {:?}", action) });
            return action;
        }
        let mut jump = local_target.y() > me.position.y;
        if local_target.x() > me.position.x
            && self.world.game().level.tiles[(me.position.x + 1.0) as usize][(me.position.y) as usize]
                == Tile::Wall
        {
            jump = true
        }
        if local_target.x() < me.position.x
            && self.world.game().level.tiles[(me.position.x - 1.0) as usize][(me.position.y) as usize]
                == Tile::Wall
        {
            jump = true
        }
        UnitAction {
            velocity: local_target.x() - me.position.x,
            jump,
            jump_down: local_target.y() < me.position.y,
            shoot,
            aim,
            reload: false,
            swap_weapon: self.should_swap_weapon(shoot),
            plant_mine: false,
        }
    }

    fn on_start(&mut self) {
        if self.calls_per_tick == 0 {
            self.tick_start_time = Instant::now();
        }
        self.calls_per_tick += 1;
    }

    fn on_finish(&mut self) {
        if self.calls_per_tick < self.world.number_of_teammates() + 1 {
            return;
        }

        let finish = Instant::now();
        let cpu_time_spent = finish - self.tick_start_time;
        self.max_cpu_time_spent = self.max_cpu_time_spent.max(cpu_time_spent);
        self.cpu_time_spent += cpu_time_spent;
        self.time_spent = finish - self.start_time;
        let cpu_time_budget_spent = time_bugdet_spent(self.world.game().current_tick, &self.cpu_time_spent);
        let time_budget_spent = time_bugdet_spent(self.world.game().current_tick, &self.time_spent);
        self.max_cpu_time_budget_spent = self.max_cpu_time_budget_spent.max(cpu_time_budget_spent);
        self.max_time_budget_spent = self.max_time_budget_spent.max(time_budget_spent);
        self.calls_per_tick = 0;

        if cpu_time_budget_spent > 90.0 {
            self.slow_down = true;
            #[cfg(not(feature = "disable_output"))]
            {
                eprintln!(
                    "{} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
                    self.world.game().current_tick, self.time_spent, self.cpu_time_spent, self.max_cpu_time_spent,
                    cpu_time_budget_spent, time_budget_spent, self.max_cpu_time_budget_spent, self.max_time_budget_spent
                );
            }
        } else {
            self.slow_down = false;
        }
    }

    fn should_swap_weapon(&self, should_shoot: bool) -> bool {
        if let Some(weapon) = self.world.me().weapon.as_ref() {
            if should_shoot && weapon.magazine > 0 {
                return false;
            }
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
