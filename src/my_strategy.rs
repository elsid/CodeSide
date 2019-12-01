#[path = "config.rs"]
pub mod config;

#[path = "world.rs"]
pub mod world;

#[cfg(feature = "dump_examples")]
#[path = "my_strategy_dump_examples.rs"]
pub mod my_strategy_dump_examples;

#[cfg(not(feature = "dump_examples"))]
#[path = "my_strategy_impl.rs"]
pub mod my_strategy_impl;

#[cfg(feature = "dump_examples")]
use self::my_strategy_dump_examples::MyStrategyImpl;

#[cfg(not(feature = "dump_examples"))]
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
            #[cfg(not(feature = "dump_examples"))]
            {
                self.strategy_impl = Some(MyStrategyImpl::new(config, me.clone(), game.clone()));
            }
            #[cfg(feature = "dump_examples")]
            {
                self.strategy_impl = Some(MyStrategyImpl::new());
            }
        }
        self.strategy_impl.as_mut().unwrap().get_action(me, game, debug)
    }
}
