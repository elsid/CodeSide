use std::time::{
    Instant,
    Duration,
};

use model::{
    Game,
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

use crate::my_strategy::{
    Config,
    Debug,
    Location,
    Positionable,
    SeedableRng,
    Vec2,
    World,
    XorShiftRng,
    get_optimal_action,
    get_optimal_location,
    get_optimal_destination,
};

#[cfg(feature = "enable_debug")]
use crate::my_strategy::{
    Level,
    Rectangular,
    get_tile_location,
};

#[cfg(feature = "dump_level")]
use crate::my_strategy::dump_level;

pub struct MyStrategyImpl {
    world: World,
    rng: XorShiftRng,
    start_time: Instant,
    tick_start_time: Instant,
    time_spent: Duration,
    cpu_time_spent: Duration,
    max_cpu_time_spent: Duration,
    max_time_budget_spent: f64,
    max_cpu_time_budget_spent: f64,
    optimal_locations: Vec<(i32, Option<Location>)>,
    optimal_destinations: Vec<(i32, Vec2)>,
    optimal_actions: Vec<(i32, UnitAction)>,
    calls_per_tick: usize,
    last_tick: i32,
    slow_down: bool,
}

impl MyStrategyImpl {
    pub fn new(config: Config, current_unit: Unit, game: Game) -> Self {
        let world = World::new(config, current_unit.player_id, game);
        #[cfg(feature = "dump_level")]
        println!("{}", dump_level(world.level()));
        let default_action = UnitAction {
            velocity: 0.0,
            jump: false,
            jump_down: false,
            aim: Vec2F64 { x: 0.0, y: 0.0 },
            shoot: false,
            reload: false,
            swap_weapon: false,
            plant_mine: false,
        };
        Self {
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
            optimal_locations: world.units().iter().map(|v| (v.id, None)).collect(),
            optimal_destinations: world.units().iter().map(|v| (v.id, v.position())).collect(),
            optimal_actions: world.units().iter().map(|v| (v.id, default_action.clone())).collect(),
            world,
            calls_per_tick: 0,
            last_tick: -1,
            slow_down: false,
        }
    }

    pub fn get_action(&mut self, unit: &Unit, game: &Game, debug: &mut Debug) -> UnitAction {
        self.on_start();
        let result = self.get_action_measured(unit, game, debug);
        self.on_finish();
        result
    }

    #[inline(never)]
    pub fn get_action_measured(&mut self, current_unit: &Unit, game: &Game, debug: &mut Debug) -> UnitAction {
        if self.last_tick != game.current_tick {
            self.last_tick = game.current_tick;

            if self.optimal_locations.len() > game.units.len() {
                self.optimal_locations.retain(|&(id, _)| game.units.iter().find(|v| v.id == id).is_some());
                self.optimal_destinations.retain(|&(id, _)| game.units.iter().find(|v| v.id == id).is_some());
                self.optimal_actions.retain(|&(id, _)| game.units.iter().find(|v| v.id == id).is_some());
            }

            self.world.update(game);

            if !self.slow_down {
                for i in 0 .. self.optimal_locations.len() {
                    let unit_id = self.optimal_locations[i].0;
                    let unit = self.world.get_unit(unit_id);
                    if self.world.is_teammate_unit(unit) {
                        self.optimal_locations[i] = (unit_id, get_optimal_location(unit, &self.optimal_locations, &self.world, debug).map(|v| v.1));
                    }
                }
            }

            for i in 0 .. self.optimal_destinations.len() {
                let unit_id = self.optimal_destinations[i].0;
                let unit = self.world.get_unit(unit_id);
                if self.world.is_teammate_unit(unit) {
                    self.optimal_destinations[i] = (unit_id, get_optimal_destination(current_unit, &self.optimal_locations[i].1, &self.world));
                }
            }

            for i in 0 .. self.optimal_actions.len() {
                let unit_id = self.optimal_actions[i].0;
                let unit = self.world.get_unit(unit_id);
                if self.world.is_teammate_unit(unit) {
                    self.optimal_actions[i] = (unit_id, get_optimal_action(unit, self.optimal_destinations[i].1, &self.world, &mut self.rng, debug));
                }
            }

            #[cfg(feature = "enable_debug")]
            for unit in self.world.units().iter() {
                render_unit(unit, debug);
                if unit.id == current_unit.id {
                    render_backtrack(self.world.get_backtrack(unit.id), self.world.level(), debug);
                }
            }

            #[cfg(feature = "enable_debug")]
            for &(id, v) in self.optimal_locations.iter() {
                if let Some(location) = v {
                    render_optimal_location(location, self.world.get_unit(id), debug);
                }
            }
        }

        let action = self.optimal_actions.iter().find(|(id, _)| *id == current_unit.id).unwrap().1.clone();

        #[cfg(feature = "enable_debug")]
        debug.log(format!("[{}] action: {:?}", current_unit.id, action));

        action
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
        let cpu_time_budget_spent = time_bugdet_spent(self.world.current_tick(), &self.cpu_time_spent);
        let time_budget_spent = time_bugdet_spent(self.world.current_tick(), &self.time_spent);
        self.max_cpu_time_budget_spent = self.max_cpu_time_budget_spent.max(cpu_time_budget_spent);
        self.max_time_budget_spent = self.max_time_budget_spent.max(time_budget_spent);
        self.calls_per_tick = 0;

        if cpu_time_budget_spent > 90.0 {
            self.slow_down = true;
            #[cfg(not(feature = "disable_output"))]
            {
                eprintln!(
                    "{} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
                    self.world.current_tick(), self.time_spent, self.cpu_time_spent, self.max_cpu_time_spent,
                    cpu_time_budget_spent, time_budget_spent, self.max_cpu_time_budget_spent, self.max_time_budget_spent
                );
            }
        } else {
            self.slow_down = false;
        }
    }
}

impl Drop for MyStrategyImpl {
    fn drop(&mut self) {
        #[cfg(not(feature = "disable_output"))]
        eprintln!(
            "{} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
            self.world.current_tick(), self.time_spent, self.cpu_time_spent, self.max_cpu_time_spent,
            time_bugdet_spent(self.world.current_tick(), &self.cpu_time_spent),
            time_bugdet_spent(self.world.current_tick(), &self.time_spent),
            self.max_time_budget_spent,
            self.max_cpu_time_budget_spent
        );
    }
}

fn time_bugdet_spent(current_tick: i32, time_spent: &Duration) -> f64 {
    time_spent.as_secs_f64() / ((current_tick * 20 + 20000) as f64 / 1000.0) * 100.0
}

#[cfg(feature = "enable_debug")]
fn render_unit(unit: &Unit, debug: &mut Debug) {
    let weapon = unit.weapon.as_ref().map(|v| v.fire_timer.map(|v| format!("{}", v)).unwrap_or(String::new())).unwrap_or(String::new());
    debug.draw(CustomData::PlacedText {
        text: format!("{} {}", unit.id, weapon),
        pos: (unit.position() + Vec2::only_y(unit.size.y)).as_model_f32(),
        alignment: TextAlignment::Center,
        size: 40.0,
        color: ColorF32 { a: 1.0, r: 1.0, g: 1.0, b: 1.0 },
    });
}

#[cfg(feature = "enable_debug")]
fn render_optimal_location(location: Location, unit: &Unit, debug: &mut Debug) {
    debug.draw(CustomData::Rect {
        pos: (location.center() - Vec2::new(0.25, 0.25)).as_model_f32(),
        size: Vec2F32 { x: 0.5, y: 0.5 },
        color: ColorF32 { a: 0.66, r: 0.0, g: 0.0, b: 0.0 },
    });
    debug.draw(CustomData::Line {
        p1: unit.rect().center().as_model_f32(),
        p2: Vec2F32 { x: location.x() as f32 + 0.5, y: location.y() as f32 + 0.5 },
        width: 0.1,
        color: ColorF32 { a: 0.66, r: 0.0, g: 0.66, b: 0.0 },
    });
}

#[cfg(feature = "enable_debug")]
fn render_backtrack(backtrack: &Vec<usize>, level: &Level, debug: &mut Debug) {
    for i in 0 .. backtrack.len() {
        if backtrack[i] == i {
            continue;
        }
        let dst = get_tile_location(level, i).center();
        let src = get_tile_location(level, backtrack[i]).center();
        debug.draw(CustomData::Line {
            p1: get_tile_location(level, i).center().as_model_f32(),
            p2: get_tile_location(level, backtrack[i]).center().as_model_f32(),
            width: 0.05,
            color: ColorF32 { a: 0.66, r: 0.66, g: 0.66, b: 0.33 },
        });
    }
}
