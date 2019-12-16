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

#[cfg(feature = "enable_debug")]
#[path = "debug.rs"]
pub mod debug;

#[cfg(feature = "enable_debug")]
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

#[path = "walk_grid.rs"]
pub mod walk_grid;

#[allow(unused_imports)]
pub use walk_grid::*;

#[path = "supercover.rs"]
pub mod supercover;

#[allow(unused_imports)]
pub use supercover::*;

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

#[path = "optimal_tile.rs"]
pub mod optimal_tile;

#[allow(unused_imports)]
pub use optimal_tile::*;

#[cfg(feature = "dump_examples")]
#[path = "my_strategy_dump_examples.rs"]
pub mod my_strategy_dump_examples;

#[cfg(feature = "dump_opponent")]
#[path = "my_strategy_dump_opponent.rs"]
pub mod my_strategy_dump_opponent;

#[cfg(feature = "dump_properties_json")]
#[path = "my_strategy_dump_properties_json.rs"]
pub mod my_strategy_dump_properties_json;

#[cfg(all(not(feature = "dump_examples"),
          not(feature = "dump_opponent"),
          not(feature = "dump_properties_json")))]
#[path = "my_strategy_impl.rs"]
pub mod my_strategy_impl;

#[cfg(feature = "dump_examples")]
pub use self::my_strategy_dump_examples::MyStrategyImpl;

#[cfg(feature = "dump_opponent")]
pub use self::my_strategy_dump_opponent::MyStrategyImpl;

#[cfg(feature = "dump_properties_json")]
pub use self::my_strategy_dump_properties_json::MyStrategyImpl;

#[cfg(all(not(feature = "dump_examples"),
          not(feature = "dump_opponent"),
          not(feature = "dump_properties_json")))]
pub use self::my_strategy_impl::MyStrategyImpl;

pub struct MyStrategy {
    strategy_impl: Option<MyStrategyImpl>,
}

impl MyStrategy {
    pub fn new() -> Self {
        Self {strategy_impl: None}
    }

    pub fn get_action(
        &mut self,
        me: &model::Unit,
        game: &model::Game,
        debug: &mut crate::Debug,
    ) -> model::UnitAction {
        if self.strategy_impl.is_none() {
            let config = get_config();
            #[cfg(any(all(not(feature = "dump_examples"), not(feature = "dump_opponent"), not(feature = "dump_properties_json"))))]
            {
                self.strategy_impl = Some(MyStrategyImpl::new(config, me.clone(), game.clone()));
            }
            #[cfg(any(feature = "dump_examples", feature = "dump_opponent", feature = "dump_properties_json"))]
            {
                self.strategy_impl = Some(MyStrategyImpl::new());
            }
        }
        self.strategy_impl.as_mut().unwrap().get_action(me, game, debug)
    }
}

#[cfg(not(feature = "read_config"))]
fn get_config() -> Config {
    Config::new()
}

#[cfg(feature = "read_config")]
fn get_config() -> Config {
    serde_json::from_str(
        std::fs::read_to_string(
            std::env::var("CONFIG").expect("CONFIG env is not found")
        ).expect("Can't read config file").as_str()
    ).expect("Can't parse config file")
}
