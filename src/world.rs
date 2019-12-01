use model::{Game, Unit};
use crate::my_strategy::config::Config;

#[derive(Debug, Clone)]
pub struct World {
    config: Config,
    me: Unit,
    game: Game,
}

impl World {
    pub fn new(config: Config, me: Unit, game: Game) -> Self {
        World { config, me, game }
    }

    pub fn update(&mut self, me: &Unit, game: &Game) {
        self.me = me.clone();
        self.game = game.clone();
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn me(&self) -> &Unit {
        &self.me
    }

    pub fn game(&self) -> &Game {
        &self.game
    }
}
