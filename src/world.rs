use std::collections::BTreeMap;

use model::{
    Bullet,
    Game,
    Item,
    LootBox,
    Mine,
    Player,
    Properties,
    Tile,
    Unit,
};

use crate::my_strategy::{
    Config,
    Level,
    Location,
    Positionable,
    Rect,
    Rectangular,
    Vec2,
    Vec2i,
    make_location_rect,
};

#[derive(Debug, Clone)]
pub struct World {
    config: Config,
    player_id: i32,
    current_tick: i32,
    properties: Properties,
    level: Level,
    players: Vec<Player>,
    units: Vec<Unit>,
    bullets: Vec<Bullet>,
    mines: Vec<Mine>,
    loot_boxes: Vec<LootBox>,
    size: Vec2,
    items_by_tile: BTreeMap<Location, Item>,
    unit_index: Vec<i32>,
    max_distance: f64,
    number_of_teammates: usize,
    number_of_opponents: usize,
    max_score: i32,
    is_complex_level: bool,
    my_player_index: usize,
    opponent_player_index: usize,
    rect: Rect,
    reachablility: Vec<bool>,
}

impl World {
    pub fn new(config: Config, player_id: i32, game: Game) -> Self {
        let unit_index: Vec<i32> = game.units.iter()
            .map(|v| v.id)
            .collect();
        let level = Level::from_model(&game.level);
        let level_size_x = level.size_x();
        let level_size_y = level.size_y();
        let size = Vec2::new(level_size_x as f64, level_size_y as f64);
        Self {
            player_id,
            size,
            items_by_tile: game.loot_boxes.iter()
                .map(|v| (v.location(), v.item.clone()))
                .collect(),
            unit_index,
            number_of_teammates: game.units.iter().filter(|v| v.player_id == player_id).count().max(1) - 1,
            number_of_opponents: game.units.iter().filter(|v| v.player_id != player_id).count(),
            config,
            current_tick: game.current_tick,
            properties: game.properties.clone(),
            is_complex_level: is_complex_level(&level),
            reachablility: make_reachability_matrix(level.get_tile_index(game.units[0].location()), &level),
            level,
            players: game.players.clone(),
            units: game.units.clone(),
            bullets: game.bullets.clone(),
            mines: game.mines.clone(),
            loot_boxes: game.loot_boxes.clone(),
            max_distance: size.norm(),
            max_score: (
                game.properties.team_size
                * (game.properties.kill_score + game.properties.unit_max_health)
                + game.loot_boxes.iter().filter(|v| is_health_pack(v)).count() as i32 * game.properties.health_pack_health
            ),
            my_player_index: game.players.iter().position(|v| v.id == player_id).unwrap(),
            opponent_player_index: game.players.iter().position(|v| v.id != player_id).unwrap(),
            rect: Rect::new(size / 2.0, size / 2.0),
        }
    }

    pub fn update(&mut self, game: &Game) {
        self.current_tick = game.current_tick;
        self.players = game.players.clone();
        self.units = game.units.clone();
        self.bullets = game.bullets.clone();
        self.mines = game.mines.clone();
        self.loot_boxes = game.loot_boxes.clone();
        self.items_by_tile = game.loot_boxes.iter()
            .map(|v| (v.location(), v.item.clone()))
            .collect();

        self.number_of_teammates = game.units.iter().filter(|v| self.is_teammate_unit(v)).count() - 1;
        self.number_of_opponents = game.units.iter().filter(|v| self.is_opponent_unit(v)).count();
        self.my_player_index = self.players().iter().position(|v| v.id == self.player_id).unwrap();
        self.opponent_player_index = (self.my_player_index + 1) % 2;

        if self.unit_index.len() > self.units.len() {
            self.unit_index.retain(|&id| game.units.iter().find(|v| v.id == id).is_some());
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn player_id(&self) -> i32 {
        self.player_id
    }

    pub fn current_tick(&self) -> i32 {
        self.current_tick
    }

    pub fn game(&self) -> Game {
        Game {
            current_tick: self.current_tick,
            properties: self.properties.clone(),
            level: self.level.as_model(),
            players: self.players.clone(),
            units: self.units.clone(),
            bullets: self.bullets.clone(),
            mines: self.mines.clone(),
            loot_boxes: self.loot_boxes.clone(),
        }
    }

    pub fn properties(&self) -> &Properties {
        &self.properties
    }

    pub fn units(&self) -> &Vec<Unit> {
        &self.units
    }

    pub fn bullets(&self) -> &Vec<Bullet> {
        &self.bullets
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn mines(&self) -> &Vec<Mine> {
        &self.mines
    }

    pub fn loot_boxes(&self) -> &Vec<LootBox> {
        &self.loot_boxes
    }

    pub fn level(&self) -> &Level {
        &self.level
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn max_distance(&self) -> f64 {
        self.max_distance
    }

    pub fn number_of_teammates(&self) -> usize {
        self.number_of_teammates
    }

    pub fn number_of_opponents(&self) -> usize {
        self.number_of_opponents
    }

    pub fn max_score(&self) -> i32 {
        self.max_score
    }

    pub fn is_complex_level(&self) -> bool {
        self.is_complex_level
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    pub fn my_player(&self) -> &Player {
        &self.players[self.my_player_index]
    }

    pub fn opponent_player(&self) -> &Player {
        &self.players[self.opponent_player_index]
    }

    pub fn tick_time_interval(&self) -> f64 {
        1.0 / self.properties.ticks_per_second as f64
    }

    pub fn get_tile(&self, location: Location) -> Tile {
        self.level.get_tile(location)
    }

    pub fn get_unit(&self, id: i32) -> &Unit {
        self.units.iter()
            .find(|v| v.id == id)
            .unwrap()
    }

    pub fn get_unit_index(&self, id: i32) -> usize {
        self.unit_index.iter()
            .position(|&v| v == id)
            .unwrap()
    }

    pub fn tile_item(&self, location: Location) -> Option<&Item> {
        self.items_by_tile.get(&location)
    }

    pub fn has_opponent_unit(&self, location: Location) -> bool {
        let location_rect = make_location_rect(location);
        self.units.iter()
            .filter(|v| self.is_opponent_unit(v))
            .find(|v| v.rect().has_collision(&location_rect))
            .is_some()
    }

    pub fn has_teammate_unit(&self, unit_id: i32, location: Location) -> bool {
        let location_rect = make_location_rect(location);
        self.units.iter()
            .filter(|v| v.id != unit_id && self.is_teammate_unit(v))
            .find(|v| v.rect().has_collision(&location_rect))
            .is_some()
    }

    pub fn has_mine(&self, location: Location) -> bool {
        let location_rect = make_location_rect(location);
        let mine_half = Vec2::new(self.properties.mine_trigger_radius, self.properties.mine_trigger_radius);
        self.mines.iter()
            .find(|v| Rect::new(v.position(), mine_half).has_collision(&location_rect))
            .is_some()
    }

    pub fn has_bullet(&self, unit_id: i32, location: Location) -> bool {
        let location_rect = make_location_rect(location);
        let above_location_rect = make_location_rect(location + Vec2i::only_y(1));
        self.bullets.iter()
            .filter(|v| v.unit_id != unit_id)
            .find(|v| {
                let half = if let Some(explosion_params) = v.explosion_params.as_ref() {
                    explosion_params.radius
                } else {
                    v.size / 2.0
                };
                let rect = Rect::new(v.position(), Vec2::new(half, half));
                rect.has_collision(&location_rect) || rect.has_collision(&above_location_rect)
            })
            .is_some()
    }

    pub fn is_teammate_unit(&self, unit: &Unit) -> bool {
        unit.player_id == self.player_id
    }

    pub fn is_opponent_unit(&self, unit: &Unit) -> bool {
        unit.player_id != self.player_id
    }

    pub fn is_teammate_mine(&self, mine: &Mine) -> bool {
        mine.player_id == self.player_id
    }

    pub fn get_player(&self) -> &Player {
        self.players.iter()
            .find(|v| v.id == self.player_id)
            .unwrap()
    }

    pub fn get_opponent(&self) -> &Player {
        self.players.iter()
            .find(|v| v.id != self.player_id)
            .unwrap()
    }

    pub fn is_reachable(&self, index: usize) -> bool {
        self.reachablility[index]
    }
}

fn get_units_locations(units: &Vec<Unit>) -> Vec<(i32, Location)> {
    let mut result: Vec<(i32, Location)> = units.iter()
        .map(|v| (v.id, v.location()))
        .collect();
    result.sort();
    result
}

fn get_distance_factor(source: Tile, destination: Tile) -> f64 {
    if source == Tile::JumpPad || destination == Tile::JumpPad {
        2.0
    } else {
        1.0
    }
}

fn is_health_pack(loot_box: &LootBox) -> bool {
    if let &Item::HealthPack { .. } = &loot_box.item {
        true
    } else {
        false
    }
}

fn is_complex_level(level: &Level) -> bool {
    let wall = (0 .. level.size()).find(|v| level.get_tile_by_index(*v) == Tile::Wall);

    if wall.is_none() {
        return false;
    }

    let mut used = std::iter::repeat(false).take(level.size()).collect::<Vec<_>>();
    let mut stack = vec![wall.unwrap()];

    used[wall.unwrap()] = true;

    while let Some(index) = stack.pop() {
        let location = level.get_tile_location(index);

        if location.x() > 0 {
            let neighbor_index = level.get_tile_index(location + Vec2i::only_x(-1));
            if level.get_tile_by_index(neighbor_index) == Tile::Wall && !used[neighbor_index] {
                used[neighbor_index] = true;
                stack.push(neighbor_index);
            }
        }

        if location.x() + 1 < level.size_x() {
            let neighbor_index = level.get_tile_index(location + Vec2i::only_x(1));
            if level.get_tile_by_index(neighbor_index) == Tile::Wall && !used[neighbor_index] {
                used[neighbor_index] = true;
                stack.push(neighbor_index);
            }
        }

        if location.y() > 0 {
            let neighbor_index = level.get_tile_index(location + Vec2i::only_y(-1));
            if level.get_tile_by_index(neighbor_index) == Tile::Wall && !used[neighbor_index] {
                used[neighbor_index] = true;
                stack.push(neighbor_index);
            }
        }

        if location.y() + 1 < level.size_y() {
            let neighbor_index = level.get_tile_index(location + Vec2i::only_y(1));
            if level.get_tile_by_index(neighbor_index) == Tile::Wall && !used[neighbor_index] {
                used[neighbor_index] = true;
                stack.push(neighbor_index);
            }
        }
    }

    (0 .. level.size()).find(|v| !used[*v] && level.get_tile_by_index(*v) == Tile::Wall).is_some()
}

fn make_reachability_matrix(origin: usize, level: &Level) -> Vec<bool> {
    let mut reached = std::iter::repeat(false).take(level.size()).collect::<Vec<_>>();
    let mut stack = vec![origin];

    reached[origin] = true;

    while let Some(index) = stack.pop() {
        let location = level.get_tile_location(index);

        if location.x() > 0 {
            let neighbor_index = level.get_tile_index(location + Vec2i::only_x(-1));
            if level.get_tile_by_index(neighbor_index) != Tile::Wall && !reached[neighbor_index] {
                reached[neighbor_index] = true;
                stack.push(neighbor_index);
            }
        }

        if location.x() + 1 < level.size_x() {
            let neighbor_index = level.get_tile_index(location + Vec2i::only_x(1));
            if level.get_tile_by_index(neighbor_index) != Tile::Wall && !reached[neighbor_index] {
                reached[neighbor_index] = true;
                stack.push(neighbor_index);
            }
        }

        if location.y() > 0 {
            let neighbor_index = level.get_tile_index(location + Vec2i::only_y(-1));
            if level.get_tile_by_index(neighbor_index) != Tile::Wall && !reached[neighbor_index] {
                reached[neighbor_index] = true;
                stack.push(neighbor_index);
            }
        }

        if location.y() + 1 < level.size_y() {
            let neighbor_index = level.get_tile_index(location + Vec2i::only_y(1));
            if level.get_tile_by_index(neighbor_index) != Tile::Wall && !reached[neighbor_index] {
                reached[neighbor_index] = true;
                stack.push(neighbor_index);
            }
        }
    }

    reached
}
