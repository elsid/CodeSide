#[path = "random.rs"]
pub mod random;

#[allow(unused_imports)]
pub use random::*;

#[path = "vec2_f64.rs"]
pub mod vec2_f64;

#[allow(unused_imports)]
pub use vec2_f64::*;

#[path = "unit_action.rs"]
pub mod unit_action;

#[allow(unused_imports)]
pub use unit_action::*;

#[path = "level.rs"]
pub mod level;

#[allow(unused_imports)]
pub use level::*;

#[path = "common.rs"]
#[macro_use]
pub mod common;

#[allow(unused_imports)]
pub use common::*;

#[path = "optimization.rs"]
pub mod optimization;

#[allow(unused_imports)]
pub use optimization::*;

#[path = "debug.rs"]
pub mod debug;

#[allow(unused_imports)]
pub use debug::*;

#[path = "location.rs"]
pub mod location;

#[allow(unused_imports)]
pub use location::*;

#[path = "vec2i.rs"]
pub mod vec2i;

#[allow(unused_imports)]
pub use vec2i::*;

#[path = "vec2.rs"]
pub mod vec2;

#[allow(unused_imports)]
pub use vec2::*;

#[path = "sector.rs"]
pub mod sector;

#[allow(unused_imports)]
pub use sector::*;

#[path = "walk_grid.rs"]
pub mod walk_grid;

#[allow(unused_imports)]
pub use walk_grid::*;

#[path = "positionable.rs"]
pub mod positionable;

#[allow(unused_imports)]
pub use positionable::*;

#[path = "rect.rs"]
pub mod rect;

#[allow(unused_imports)]
pub use rect::*;

#[path = "rectangular.rs"]
pub mod rectangular;

#[allow(unused_imports)]
pub use rectangular::*;

#[path = "unit.rs"]
pub mod unit;

#[allow(unused_imports)]
pub use unit::*;

#[path = "loot_box.rs"]
pub mod loot_box;

#[allow(unused_imports)]
pub use loot_box::*;

#[path = "bullet.rs"]
pub mod bullet;

#[allow(unused_imports)]
pub use bullet::*;

#[path = "mine.rs"]
pub mod mine;

#[allow(unused_imports)]
pub use mine::*;

#[path = "properties.rs"]
pub mod properties;

#[allow(unused_imports)]
pub use properties::*;

#[path = "hit.rs"]
pub mod hit;

#[allow(unused_imports)]
pub use hit::*;

#[path = "config.rs"]
pub mod config;

#[allow(unused_imports)]
pub use config::*;

#[path = "world.rs"]
pub mod world;

#[allow(unused_imports)]
pub use world::*;

#[path = "simulator.rs"]
pub mod simulator;

#[allow(unused_imports)]
pub use simulator::*;

#[path = "search.rs"]
pub mod search;

#[allow(unused_imports)]
pub use search::*;

#[path = "plan.rs"]
pub mod plan;

#[allow(unused_imports)]
pub use plan::*;

#[path = "role.rs"]
pub mod role;

#[allow(unused_imports)]
pub use role::*;

#[path = "optimal_location.rs"]
pub mod optimal_location;

#[allow(unused_imports)]
pub use optimal_location::*;

#[path = "optimal_destination.rs"]
pub mod optimal_destination;

#[allow(unused_imports)]
pub use optimal_destination::*;

#[path = "optimal_target.rs"]
pub mod optimal_target;

#[allow(unused_imports)]
pub use optimal_target::*;

#[path = "optimal_plan.rs"]
pub mod optimal_plan;

#[allow(unused_imports)]
pub use optimal_plan::*;

#[path = "optimal_action.rs"]
pub mod optimal_action;

#[allow(unused_imports)]
pub use optimal_action::*;

#[cfg(feature = "dump_examples")]
#[path = "my_strategy_dump_examples.rs"]
pub mod my_strategy_dump_examples;

#[cfg(feature = "dump_opponent")]
#[path = "my_strategy_dump_opponent.rs"]
pub mod my_strategy_dump_opponent;

#[cfg(feature = "dump_properties_json")]
#[path = "my_strategy_dump_properties_json.rs"]
pub mod my_strategy_dump_properties_json;

#[cfg(feature = "jump")]
#[path = "my_strategy_jump.rs"]
pub mod my_strategy_jump;

#[cfg(feature = "check_simulator")]
#[path = "my_strategy_check_simulator.rs"]
pub mod my_strategy_check_simulator;

#[cfg(feature = "test")]
#[path = "my_strategy_test.rs"]
pub mod my_strategy_test;

#[cfg(feature = "pusher")]
#[path = "my_strategy_pusher.rs"]
pub mod my_strategy_pusher;

#[cfg(feature = "bullets_spread")]
#[path = "my_strategy_bullets_spread.rs"]
pub mod my_strategy_bullets_spread;

#[cfg(all(
    not(feature = "dump_examples"),
    not(feature = "dump_opponent"),
    not(feature = "dump_properties_json"),
    not(feature = "jump"),
    not(feature = "test"),
    not(feature = "pusher"),
    not(feature = "bullets_spread"),
))]
#[path = "my_strategy_impl.rs"]
pub mod my_strategy_impl;

#[cfg(feature = "dump_examples")]
pub use self::my_strategy_dump_examples::MyStrategyImpl;

#[cfg(feature = "dump_opponent")]
pub use self::my_strategy_dump_opponent::MyStrategyImpl;

#[cfg(feature = "dump_properties_json")]
pub use self::my_strategy_dump_properties_json::MyStrategyImpl;

#[cfg(feature = "jump")]
pub use self::my_strategy_jump::MyStrategyImpl;

#[cfg(feature = "check_simulator")]
pub use self::my_strategy_check_simulator::MyStrategyImpl;

#[cfg(feature = "test")]
pub use self::my_strategy_test::MyStrategyImpl;

#[cfg(feature = "pusher")]
pub use self::my_strategy_pusher::MyStrategyImpl;

#[cfg(feature = "bullets_spread")]
pub use self::my_strategy_bullets_spread::MyStrategyImpl;

#[cfg(all(
    not(feature = "dump_examples"),
    not(feature = "dump_opponent"),
    not(feature = "dump_properties_json"),
    not(feature = "jump"),
    not(feature = "check_simulator"),
    not(feature = "test"),
    not(feature = "pusher"),
    not(feature = "bullets_spread"),
))]
pub use self::my_strategy_impl::MyStrategyImpl;

pub struct MyStrategy {
    strategy_impl: Option<MyStrategyImpl>,
    last_tick: i32,
    debug_next_y: f32,
    #[cfg(feature = "max_tick")]
    max_tick: i32,
}

impl MyStrategy {
    pub fn new() -> Self {
        Self {
            strategy_impl: None,
            last_tick: -1,
            debug_next_y: 0.0,
            #[cfg(feature = "max_tick")]
            max_tick: std::env::var("MAX_TICK").map(|v| v.parse::<i32>().unwrap_or(std::i32::MAX)).unwrap_or(std::i32::MAX),
        }
    }

    pub fn get_action(
        &mut self,
        unit: &model::Unit,
        game: &model::Game,
        debug: &mut crate::Debug,
    ) -> model::UnitAction {
        #[cfg(feature = "max_tick")]
        {
            if game.current_tick > self.max_tick {
                std::process::exit(0);
            }
        }
        if self.last_tick != game.current_tick {
            self.last_tick = game.current_tick;
            self.debug_next_y = 31.0;
        }
        if self.strategy_impl.is_none() {
            let config = get_config().adjusted(game.properties.team_size);
            #[cfg(feature = "dump_config")]
            println!("{}", rustc_serialize::json::encode(&config).expect("Can't serialize config"));
            #[cfg(any(
                all(
                    not(feature = "dump_examples"),
                    not(feature = "dump_opponent"),
                    not(feature = "dump_properties_json"),
                    not(feature = "jump"),
                ),
                feature = "check_simulator",
                feature = "pusher",
                feature = "test",
            ))]
            {
                self.strategy_impl = Some(MyStrategyImpl::new(config, unit.clone(), game.clone()));
            }
            #[cfg(any(
                feature = "dump_examples",
                feature = "dump_opponent",
                feature = "dump_properties_json",
                feature = "jump",
            ))]
            {
                self.strategy_impl = Some(MyStrategyImpl::new());
            }
        }
        let mut d = Debug::with_next_y(self.debug_next_y, debug);
        let action = self.strategy_impl.as_mut().unwrap().get_action(unit, game, &mut d);
        self.debug_next_y = d.next_y();
        action
    }
}

#[cfg(not(feature = "read_config"))]
fn get_config() -> Config {
    Config::new()
}

#[cfg(feature = "read_config")]
fn get_config() -> Config {
    rustc_serialize::json::decode(
        std::fs::read_to_string(
            std::env::var("CONFIG").expect("CONFIG env is not found")
        ).expect("Can't read config file").as_str()
    ).expect("Can't parse config file")
}
