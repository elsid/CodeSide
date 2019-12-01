use crate::my_strategy::config::Config;

#[derive(Debug, Clone)]
pub struct World {
    config: Config,
    me: model::Unit,
    game: model::Game,
}

impl World {
    pub fn new(config: Config, me: model::Unit, game: model::Game) -> Self {
        World { config, me, game }
    }

    pub fn update(&mut self, me: &model::Unit, game: &model::Game) {
        self.me = me.clone();
        self.game = game.clone();
    }

    pub fn me(&self) -> &model::Unit {
        &self.me
    }

    pub fn game(&self) -> &model::Game {
        &self.game
    }
}
