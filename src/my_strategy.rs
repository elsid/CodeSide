#[path = "config.rs"]
pub mod config;

#[path = "world.rs"]
pub mod world;

#[path = "my_strategy_impl.rs"]
pub mod my_strategy_impl;

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
            self.strategy_impl = Some(MyStrategyImpl::new(config, me.clone(), game.clone()));
        }
        self.strategy_impl.as_mut().unwrap().get_action(me, game, debug)
    }
}
