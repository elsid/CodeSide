#[path = "random.rs"]
pub mod random;

#[allow(unused_imports)]
pub use random::*;

#[path = "unit_action.rs"]
pub mod unit_action;

#[allow(unused_imports)]
pub use unit_action::*;

#[path = "vec2_f64.rs"]
pub mod vec2_f64;

#[allow(unused_imports)]
pub use vec2_f64::*;

#[path = "level.rs"]
pub mod level;

#[allow(unused_imports)]
pub use level::*;

#[path = "weapon_params.rs"]
pub mod weapon_params;

#[allow(unused_imports)]
pub use weapon_params::*;

#[path = "weapon.rs"]
pub mod weapon;

#[allow(unused_imports)]
pub use weapon::*;

#[path = "common.rs"]
pub mod common;

#[allow(unused_imports)]
pub use common::*;

#[path = "vec2.rs"]
pub mod vec2;

#[allow(unused_imports)]
pub use vec2::*;

#[path = "rect.rs"]
pub mod rect;

#[allow(unused_imports)]
pub use rect::*;

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

#[cfg(feature = "dump_examples")]
#[path = "my_strategy_dump_examples.rs"]
pub mod my_strategy_dump_examples;

#[cfg(feature = "dump_opponent")]
#[path = "my_strategy_dump_opponent.rs"]
pub mod my_strategy_dump_opponent;

#[cfg(feature = "go_to_target")]
#[path = "my_strategy_go_to_target.rs"]
pub mod my_strategy_go_to_target;

#[cfg(all(not(feature = "dump_examples"),
          not(feature = "dump_opponent"),
          not(feature = "go_to_target")))]
#[path = "my_strategy_impl.rs"]
pub mod my_strategy_impl;

#[cfg(feature = "dump_examples")]
pub use self::my_strategy_dump_examples::MyStrategyImpl;

#[cfg(feature = "dump_opponent")]
pub use self::my_strategy_dump_opponent::MyStrategyImpl;

#[cfg(feature = "go_to_target")]
pub use self::my_strategy_go_to_target::MyStrategyImpl;

#[cfg(all(not(feature = "dump_examples"),
          not(feature = "dump_opponent"),
          not(feature = "go_to_target")))]
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
            let config = Config::new();
            #[cfg(any(all(not(feature = "dump_examples"), not(feature = "dump_opponent")),
                      feature = "go_to_target"))]
            {
                self.strategy_impl = Some(MyStrategyImpl::new(config, me.clone(), game.clone()));
            }
            #[cfg(any(feature = "dump_examples", feature = "dump_opponent"))]
            {
                self.strategy_impl = Some(MyStrategyImpl::new());
            }
        }
        self.strategy_impl.as_mut().unwrap().get_action(me, game, debug)
    }
}
