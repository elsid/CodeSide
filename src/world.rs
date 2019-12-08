use std::collections::BTreeMap;
use model::{
    Bullet,
    Game,
    Item,
    Level,
    LootBox,
    Player,
    Properties,
    Tile,
    Unit,
};
use crate::my_strategy::{
    Config,
    Vec2,
    get_tile,
    get_tile_by_vec2,
};

#[derive(Debug, Clone)]
pub struct World {
    config: Config,
    me: Unit,
    game: Game,
    size: Vec2,
    items_by_tile: BTreeMap<(usize, usize), Item>,
}

impl World {
    pub fn new(config: Config, me: Unit, game: Game) -> Self {
        Self {
            size: Vec2::new(
                game.level.tiles.len() as f64,
                game.level.tiles.iter().max_by_key(|v| v.len()).unwrap().len() as f64
            ),
            items_by_tile: game.loot_boxes.iter()
                .map(|v| ((v.position.x as usize, v.position.y as usize), v.item.clone()))
                .collect(),
            config,
            me,
            game,
        }
    }

    pub fn update(&mut self, me: &Unit, game: &Game) {
        self.me = me.clone();
        self.game = game.clone();
        self.items_by_tile = game.loot_boxes.iter()
            .map(|v| ((v.position.x as usize, v.position.y as usize), v.item.clone()))
            .collect();
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

    pub fn loot_boxes(&self) -> &Vec<LootBox> {
        &self.game.loot_boxes
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

    pub fn tile(&self, x: usize, y: usize) -> Tile {
        get_tile(&self.game.level, x, y)
    }

    pub fn tile_by_position(&self, position: Vec2) -> Tile {
        get_tile_by_vec2(&self.game.level, position)
    }

    pub fn get_unit(&self, id: i32) -> &Unit {
        self.game.units.iter()
            .find(|v| v.id == id)
            .unwrap()
    }

    pub fn tile_item(&self, x: usize, y: usize) -> Option<&Item> {
        self.items_by_tile.get(&(x, y))
    }
}
