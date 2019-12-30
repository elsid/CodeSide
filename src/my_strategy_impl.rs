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
    Plan,
    Positionable,
    Role,
    SeedableRng,
    Vec2,
    World,
    XorShiftRng,
    get_miner_action,
    get_shooter_action,
    get_optimal_destination,
    get_optimal_location,
    get_optimal_plan,
    get_optimal_target,
    get_role,
};

#[cfg(feature = "enable_debug")]
use crate::my_strategy::{
    Level,
    Rectangular,
};

#[cfg(feature = "dump_level")]
use crate::my_strategy::dump_level;

pub struct MyStrategyImpl {
    world: World,
    rng: XorShiftRng,
    roles: Vec<(i32, Role)>,
    optimal_locations: Vec<(i32, Option<Location>)>,
    optimal_destinations: Vec<(i32, Vec2)>,
    optimal_targets: Vec<(i32, Option<Vec2>)>,
    optimal_plans: Vec<(i32, Plan)>,
    optimal_actions: Vec<(i32, UnitAction)>,
    last_tick: i32,
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
            roles: world.units().iter().map(|v| (v.id, Role::Shooter)).collect(),
            optimal_locations: world.units().iter().map(|v| (v.id, None)).collect(),
            optimal_destinations: world.units().iter().map(|v| (v.id, v.position())).collect(),
            optimal_targets: world.units().iter().map(|v| (v.id, None)).collect(),
            optimal_plans: world.units().iter().map(|v| (v.id, Plan::default())).collect(),
            optimal_actions: world.units().iter().map(|v| (v.id, default_action.clone())).collect(),
            world,
            last_tick: -1,
        }
    }

    #[inline(never)]
    pub fn get_action(&mut self, current_unit: &Unit, game: &Game, debug: &mut Debug) -> UnitAction {
        if self.last_tick != game.current_tick {
            self.last_tick = game.current_tick;

            if self.optimal_locations.len() > game.units.len() {
                self.roles.retain(|&(id, _)| game.units.iter().find(|v| v.id == id).is_some());
                self.optimal_locations.retain(|&(id, _)| game.units.iter().find(|v| v.id == id).is_some());
                self.optimal_destinations.retain(|&(id, _)| game.units.iter().find(|v| v.id == id).is_some());
                self.optimal_targets.retain(|&(id, _)| game.units.iter().find(|v| v.id == id).is_some());
                self.optimal_plans.retain(|&(id, _)| game.units.iter().find(|v| v.id == id).is_some());
                self.optimal_actions.retain(|&(id, _)| game.units.iter().find(|v| v.id == id).is_some());
            }

            self.world.update(game);

            self.assign_roles(debug);

            self.set_locations(debug);
            self.set_destinatons();
            self.set_targets(debug);
            self.set_plans(debug);
            self.set_actions(debug);

            self.update_roles();

            #[cfg(all(feature = "enable_debug", feature = "enable_debug_backtrack"))]
            for unit in self.world.units().iter() {
                let role = &self.roles.iter().find(|(id, _)| *id == unit.id).unwrap().1;
                render_unit(unit, role, debug);
                if unit.id == current_unit.id {
                    render_backtrack(self.world.get_backtrack(unit.id), self.world.level(), debug);
                }
            }

            #[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_location"))]
            for &(id, v) in self.optimal_locations.iter() {
                if let Some(location) = v {
                    render_optimal_location(location, self.world.get_unit(id), debug);
                }
            }
        }

        #[cfg(all(feature = "enable_debug", feature = "enable_debug_log"))]
        {
            if let Some(weapon) = current_unit.weapon.as_ref() {
                debug.log(format!("[{}] weapon: last_angle={:?}", current_unit.id, weapon.last_angle));
            }
        }

        let action = self.optimal_actions.iter().find(|(id, _)| *id == current_unit.id).unwrap().1.clone();

        #[cfg(all(feature = "enable_debug", feature = "enable_debug_log"))]
        debug.log(format!("[{}][{}] action: {:?}", current_unit.id, game.current_tick, action));

        action
    }

    fn assign_roles(&mut self, debug: &mut Debug) {
        for i in 0 .. self.roles.len() {
            let unit_id = self.roles[i].0;
            let unit = self.world.get_unit(unit_id);
            if self.world.is_teammate_unit(unit) {
                self.roles[i] = (unit_id, get_role(unit, &self.roles[i].1, &self.world));
                #[cfg(all(feature = "enable_debug", feature = "enable_debug_log"))]
                debug.log(format!("[{}] role: {:?}", unit_id, self.roles[i].1));
            }
        }
    }

    fn update_roles(&mut self) {
        for i in 0 .. self.roles.len() {
            let unit_id = self.roles[i].0;
            let unit = self.world.get_unit(unit_id);
            if self.world.is_teammate_unit(unit) {
                self.roles[i].1 = match &self.roles[i].1 {
                    Role::Miner { plant_mines } => Role::Miner { plant_mines: *plant_mines - self.optimal_actions[i].1.plant_mine as usize },
                    v => v.clone(),
                }
            }
        }
    }

    fn set_locations(&mut self, debug: &mut Debug) {
        for i in 0 .. self.optimal_locations.len() {
            let unit_id = self.optimal_locations[i].0;
            let unit = self.world.get_unit(unit_id);
            if self.world.is_teammate_unit(unit) {
                match &self.roles[i].1 {
                    Role::Shooter => {
                        self.optimal_locations[i].1 = get_optimal_location(unit, &self.optimal_locations, &self.world, debug).map(|v| v.1);
                    },
                    Role::Miner { .. } => {
                        self.optimal_locations[i].1 = None;
                    },
                }
            }
        }
    }

    fn set_destinatons(&mut self) {
        for i in 0 .. self.optimal_destinations.len() {
            let unit_id = self.optimal_destinations[i].0;
            let unit = self.world.get_unit(unit_id);
            if self.world.is_teammate_unit(unit) {
                match &self.roles[i].1 {
                    Role::Shooter => {
                        self.optimal_destinations[i].1 = get_optimal_destination(unit, &self.optimal_locations[i].1, &self.world);
                    },
                    Role::Miner { .. } => {
                        self.optimal_destinations[i].1 = unit.position();
                    },
                }
            }
        }
    }

    fn set_targets(&mut self, debug: &mut Debug) {
        for i in 0 .. self.optimal_targets.len() {
            let unit_id = self.optimal_targets[i].0;
            let unit = self.world.get_unit(unit_id);
            if self.world.is_teammate_unit(unit) {
                match &self.roles[i].1 {
                    Role::Shooter => {
                        self.optimal_targets[i].1 = get_optimal_target(unit, &self.world, debug);
                    },
                    Role::Miner { .. } => {
                        self.optimal_targets[i].1 = None;
                    }
                }
            }
        }
    }

    fn set_plans(&mut self, debug: &mut Debug) {
        for i in 0 .. self.optimal_plans.len() {
            let unit_id = self.optimal_plans[i].0;
            let unit = self.world.get_unit(unit_id);
            if self.world.is_teammate_unit(unit) {
                match &self.roles[i].1 {
                    Role::Shooter => {
                        let destination = self.optimal_destinations[i].1;
                        self.optimal_plans[i].1 = get_optimal_plan(unit, destination, &self.world, &mut self.rng, debug);
                    },
                    Role::Miner { .. } => {
                        self.optimal_plans[i].1 = Plan::default();
                    }
                }
            }
        }
    }

    fn set_actions(&mut self, debug: &mut Debug) {
        for i in 0 .. self.optimal_actions.len() {
            let unit_id = self.optimal_actions[i].0;
            let unit = self.world.get_unit(unit_id);
            if self.world.is_teammate_unit(unit) {
                match &self.roles[i].1 {
                    Role::Shooter => {
                        let plan = &self.optimal_plans[i].1;
                        let target = self.optimal_targets[i].1;
                        self.optimal_actions[i].1 = get_shooter_action(unit, plan, target, &self.world, debug);
                    },
                    Role::Miner { plant_mines } => {
                        self.optimal_actions[i].1 = get_miner_action(unit, *plant_mines);
                    }
                }
            }
        }
    }
}

impl Drop for MyStrategyImpl {
    fn drop(&mut self) {
        #[cfg(not(feature = "disable_output"))]
        eprintln!(
            "{}",
            self.world.current_tick()
        );
    }
}

#[cfg(all(feature = "enable_debug", feature = "enable_debug_unit"))]
fn render_unit(unit: &Unit, role: &Role, debug: &mut Debug) {
    let weapon = unit.weapon.as_ref().map(|v| format!("{:?}\n{:?}", v.fire_timer, v.last_angle)).unwrap_or(String::new());
    debug.draw(CustomData::PlacedText {
        text: format!("{}\n{:?}\n{}", unit.id, role, weapon),
        pos: (unit.position() + Vec2::only_y(unit.size.y)).as_debug(),
        alignment: TextAlignment::Center,
        size: 40.0,
        color: ColorF32 { a: 1.0, r: 1.0, g: 1.0, b: 1.0 },
    });
}

#[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_location"))]
fn render_optimal_location(location: Location, unit: &Unit, debug: &mut Debug) {
    debug.draw(CustomData::Rect {
        pos: (location.center() - Vec2::new(0.25, 0.25)).as_debug(),
        size: Vec2F32 { x: 0.5, y: 0.5 },
        color: ColorF32 { a: 0.66, r: 0.0, g: 0.0, b: 0.0 },
    });
    debug.draw(CustomData::Line {
        p1: unit.rect().center().as_debug(),
        p2: Vec2F32 { x: location.x() as f32 + 0.5, y: location.y() as f32 + 0.5 },
        width: 0.1,
        color: ColorF32 { a: 0.66, r: 0.0, g: 0.66, b: 0.0 },
    });
}

#[cfg(all(feature = "enable_debug", feature = "enable_debug_backtrack"))]
fn render_backtrack(backtrack: &Vec<usize>, level: &Level, debug: &mut Debug) {
    for i in 0 .. backtrack.len() {
        if backtrack[i] == i {
            continue;
        }
        debug.draw(CustomData::Line {
            p1: level.get_tile_location(i).center().as_debug(),
            p2: level.get_tile_location(backtrack[i]).center().as_debug(),
            width: 0.05,
            color: ColorF32 { a: 0.66, r: 0.66, g: 0.66, b: 0.33 },
        });
    }
}
