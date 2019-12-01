use model::{
    Bullet,
    Game,
    Level,
    Player,
    Properties,
    Tile,
    Unit,
};
use crate::my_strategy::config::Config;
use crate::my_strategy::level::get_tile_by_vec2;
use crate::my_strategy::vec2::Vec2;

#[derive(Debug, Clone)]
pub struct World {
    config: Config,
    me: Unit,
    game: Game,
    size: Vec2,
}

impl World {
    pub fn new(config: Config, me: Unit, game: Game) -> Self {
        Self {
            size: Vec2::new(
                game.level.tiles.len() as f64,
                game.level.tiles.iter().max_by_key(|v| v.len()).unwrap().len() as f64
            ),
            config,
            me,
            game,
        }
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

    pub fn properties(&self) -> &Properties {
        &self.game.properties
    }

    pub fn units(&self) -> &Vec<Unit> {
        &self.game.units
    }

    pub fn bullets(&self) -> &Vec<Bullet> {
        &self.game.bullets
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.game.players
    }

    pub fn level(&self) -> &Level {
        &self.game.level
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn tick_time_interval(&self) -> f64 {
        1.0 / self.game.properties.ticks_per_second as f64
    }

    pub fn tile_by_position(&self, position: Vec2) -> Tile {
        get_tile_by_vec2(&self.game.level, position)
    }
}
