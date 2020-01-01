use model::{
    Game,
    Unit,
    UnitAction,
};

use crate::my_strategy::{
    Config,
    Debug,
    Plan,
    Planner,
    Positionable,
    Rectangular,
    SeedableRng,
    Simulator,
    Target,
    Vec2,
    World,
    XorShiftRng,
    as_score,
    default_action,
    get_nearest_hit,
    is_health_pack_item,
    is_weapon_item,
};

pub struct MyStrategyImpl {
    world: World,
    rng: XorShiftRng,
    last_tick: i32,
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
            world,
            last_tick: -1,
        }
    }

    pub fn get_action(&mut self, current_unit: &Unit, game: &Game, debug: &mut Debug) -> UnitAction {
        if self.last_tick != game.current_tick {
            self.last_tick = game.current_tick;
            self.world.update(game);
        }
        let destination = self.get_destination(current_unit);
        let plan = self.get_plan(current_unit.id, destination, debug);
        let nearest_opponent_unit_center = self.world.units().iter()
            .filter_map(|v| {
                if self.world.is_opponent_unit(v) {
                    if let Some(hit) = get_nearest_hit(current_unit.id, current_unit.center(), v.center(), &Target::from_unit(v), &self.world) {
                        if hit.is_target {
                            Some(v.center())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .min_by_key(|center| as_score(center.distance(current_unit.center())));
        let mut action = if let Some(transition) = plan.transitions.first() {
            transition.get_action(&game.properties)
        } else {
            default_action()
        };
        if let Some(v) = nearest_opponent_unit_center {
            action.aim = (v - current_unit.center()).as_model();
            action.shoot = true;
        }

        action
    }

    fn get_destination(&self, current_unit: &Unit) -> Vec2 {
        if current_unit.weapon.is_none() {
            let weapon_position = self.world.loot_boxes().iter()
                .filter(|v| is_weapon_item(&v.item))
                .min_by_key(|v| as_score(v.position().distance(current_unit.position())))
                .map(|v| v.position());
            if let Some(v) = weapon_position {
                return v;
            }
        }

        if current_unit.health < self.world.properties().unit_max_health / 2 {
            let health_pack_position = self.world.loot_boxes().iter()
                .filter(|v| is_health_pack_item(&v.item))
                .min_by_key(|v| as_score(v.position().distance(current_unit.position())))
                .map(|v| v.position());
            if let Some(v) = health_pack_position {
                return v;
            }
        }

        let opponent_unit_position = self.world.units().iter()
            .filter(|v| self.world.is_opponent_unit(v))
            .min_by_key(|v| as_score(v.position().distance(current_unit.position())))
            .map(|v| v.position());
        if let Some(v) = opponent_unit_position {
            return v;
        }

        current_unit.position()
    }

    fn get_plan(&mut self, current_unit_id: i32, destination: Vec2, debug: &mut Debug) -> Plan {
        let simulator = Simulator::new(&self.world, current_unit_id);
        let planner = Planner::new(destination, self.world.config(), simulator, self.world.max_distance(), self.world.max_score());
        planner.make(self.world.current_tick(), &mut self.rng, debug)
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
