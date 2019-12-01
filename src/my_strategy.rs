#[path = "random.rs"]
pub mod random;

#[path = "unit_action.rs"]
pub mod unit_action;

#[path = "vec2_f64.rs"]
pub mod vec2_f64;

#[path = "level.rs"]
pub mod level;

#[path = "common.rs"]
pub mod common;

#[path = "vec2.rs"]
pub mod vec2;

#[path = "rect.rs"]
pub mod rect;

#[path = "config.rs"]
pub mod config;

#[path = "world.rs"]
pub mod world;

#[path = "simulator.rs"]
pub mod simulator;

#[cfg(feature = "dump_examples")]
#[path = "my_strategy_dump_examples.rs"]
pub mod my_strategy_dump_examples;

#[cfg(feature = "dump_opponent")]
#[path = "my_strategy_dump_opponent.rs"]
pub mod my_strategy_dump_opponent;

#[cfg(all(not(feature = "dump_examples"), not(feature = "dump_opponent")))]
#[path = "my_strategy_impl.rs"]
pub mod my_strategy_impl;

#[cfg(feature = "dump_examples")]
use self::my_strategy_dump_examples::MyStrategyImpl;

#[cfg(feature = "dump_opponent")]
use self::my_strategy_dump_opponent::MyStrategyImpl;

#[cfg(all(not(feature = "dump_examples"), not(feature = "dump_opponent")))]
use self::my_strategy_impl::MyStrategyImpl;

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
        use self::config::Config;

        if self.strategy_impl.is_none() {
            let config = Config::new();
            #[cfg(all(not(feature = "dump_examples"), not(feature = "dump_opponent")))]
            {
                self.strategy_impl = Some(MyStrategyImpl::new(config, me.clone(), game.clone()));
            }
            #[cfg(feature = "dump_examples")]
            {
                self.strategy_impl = Some(MyStrategyImpl::new());
            }
            #[cfg(feature = "dump_opponent")]
            {
                self.strategy_impl = Some(MyStrategyImpl::new());
            }
        }
        self.strategy_impl.as_mut().unwrap().get_action(me, game, debug)
    }
}
