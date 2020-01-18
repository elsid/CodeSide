use model::{
    Game,
    Unit,
    UnitAction,
};

use crate::my_strategy::{
    Config,
    Debug,
    Plan,
    Positionable,
    Role,
    SeedableRng,
    Target,
    Vec2,
    World,
    XorShiftRng,
    default_action,
    get_miner_action,
    get_optimal_destination,
    get_optimal_plan,
    get_optimal_target,
    get_pusher_destination,
    get_role,
    get_shooter_action,
    update_role,
};

pub struct MyStrategyImpl {
    world: World,
    rng: XorShiftRng,
    last_tick: i32,
    roles: Vec<(i32, Role)>,
    optimal_destinations: Vec<(i32, Vec2)>,
    optimal_targets: Vec<(i32, Option<Target>)>,
    optimal_plans: Vec<(i32, Plan)>,
    optimal_actions: Vec<(i32, UnitAction)>,
}

impl MyStrategyImpl {
    pub fn new(mut config: Config, current_unit: Unit, game: Game) -> Self {
        let world = World::new(config, current_unit.player_id, game);

        #[cfg(feature = "dump_level")]
        println!("{}", crate::my_strategy::dump_level(world.level()));

        Self {
            rng: XorShiftRng::from_seed([
                3918248293,
                2127433321,
                1841971383,
                1904458926,
            ]),
            last_tick: -1,
            roles: world.units().iter().map(|v| (v.id, Role::Shooter)).collect(),
            optimal_destinations: world.units().iter().map(|v| (v.id, v.position())).collect(),
            optimal_targets: world.units().iter().map(|v| (v.id, None)).collect(),
            optimal_plans: world.units().iter().map(|v| (v.id, Plan::default())).collect(),
            optimal_actions: world.units().iter().map(|v| (v.id, default_action())).collect(),
            world,
        }
    }

    pub fn get_action(&mut self, current_unit: &Unit, game: &Game, debug: &mut Debug) -> UnitAction {
        if self.last_tick != game.current_tick {
            self.last_tick = game.current_tick;

            if self.roles.len() > game.units.len() {
                self.roles.retain(|&(id, _)| game.units.iter().find(|v| v.id == id).is_some());
                self.optimal_destinations.retain(|&(id, _)| game.units.iter().find(|v| v.id == id).is_some());
                self.optimal_targets.retain(|&(id, _)| game.units.iter().find(|v| v.id == id).is_some());
                self.optimal_plans.retain(|&(id, _)| game.units.iter().find(|v| v.id == id).is_some());
                self.optimal_actions.retain(|&(id, _)| game.units.iter().find(|v| v.id == id).is_some());
            }

            self.world.update(game);

            self.assign_roles(debug);

            self.set_destinatons();
            self.set_targets(debug);
            self.set_plans(debug);
            self.set_actions(debug);

            self.update_roles();
        }

        let action = self.optimal_actions.iter().find(|(id, _)| *id == current_unit.id).unwrap().1.clone();

        #[cfg(all(feature = "enable_debug", feature = "enable_debug_log"))]
        debug.log(format!("[{}] action: {:?}", current_unit.id, action));

        #[cfg(not(feature = "spectator"))]
        return action;

        #[cfg(feature = "spectator")]
        return default_action();
    }

    fn assign_roles(&mut self, debug: &mut Debug) {
        for i in 0 .. self.roles.len() {
            let unit_id = self.roles[i].0;
            let unit = self.world.get_unit(unit_id);
            if self.world.is_teammate_unit(unit) {
                self.roles[i].1 = get_role(unit, &self.roles[i].1, &Role::Pusher, &self.world, debug);
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
                self.roles[i].1 = update_role(unit, &self.roles[i].1, &self.world);
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
                        self.optimal_destinations[i].1 = get_optimal_destination(unit, &None, &self.world);
                    },
                    Role::Miner { .. } => {
                        self.optimal_destinations[i].1 = unit.position();
                    },
                    Role::Pusher => {
                        self.optimal_destinations[i].1 = get_pusher_destination(unit, &self.world);
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
                    Role::Shooter | Role::Pusher => {
                        self.optimal_targets[i].1 = get_optimal_target(unit, &self.world, debug);
                    },
                    Role::Miner { .. } => {
                        self.optimal_targets[i].1 = None;
                    },
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
                    Role::Shooter | Role::Pusher => {
                        let destination = self.optimal_destinations[i].1;
                        self.optimal_plans[i].1 = get_optimal_plan(unit, destination, &self.world, &mut self.rng, debug);
                    },
                    Role::Miner { .. } => {
                        self.optimal_plans[i].1 = Plan::default();
                    },
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
                    Role::Shooter | Role::Pusher => {
                        let plan = &self.optimal_plans[i].1;
                        let target = &self.optimal_targets[i].1;
                        self.optimal_actions[i].1 = get_shooter_action(unit, plan, target, &self.world, debug);
                    },
                    Role::Miner { plant_mines, planted_mines } => {
                        let mines_left = if *plant_mines > *planted_mines {
                            *plant_mines - *planted_mines
                        } else {
                            0
                        };
                        self.optimal_actions[i].1 = get_miner_action(unit, mines_left);
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
            "result {}",
            self.world.current_tick()
        );
    }
}
