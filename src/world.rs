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
    ImplicitProperties,
    Level,
    Location,
    Positionable,
    Rectangular,
    Vec2,
    Vec2i,
    as_score,
    get_level_size_x,
    get_level_size_y,
    get_tile,
    get_tile_by_vec2,
    get_tile_index,
    get_tile_location,
    will_hit_by_line,
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
    paths: Vec<(i32, BTreeMap<(Location, Location), TilePathInfo>)>,
    backtracks: Vec<(i32, Vec<usize>)>,
    unit_index: Vec<i32>,
    max_distance: f64,
    number_of_teammates: usize,
}

impl World {
    pub fn new(config: Config, player_id: i32, game: Game) -> Self {
        let unit_index: Vec<i32> = game.units.iter()
            .map(|v| v.id)
            .collect();
        let level = Level::from_model(&game.level);
        let level_size_x = get_level_size_x(&level);
        let level_size_y = get_level_size_y(&level);
        let size = Vec2::new(level_size_x as f64, level_size_y as f64);
        Self {
            player_id,
            size,
            items_by_tile: game.loot_boxes.iter()
                .map(|v| (v.location(), v.item.clone()))
                .collect(),
            paths: game.units.iter().map(|v| (v.id, BTreeMap::new())).collect(),
            backtracks: game.units.iter().map(|v| (v.id, std::iter::repeat(0).take(level_size_x * level_size_y).collect::<Vec<_>>())).collect(),
            unit_index,
            number_of_teammates: game.units.iter().filter(|v| v.player_id == player_id).count().max(1) - 1,
            config,
            current_tick: game.current_tick,
            properties: game.properties.clone(),
            level,
            players: game.players.clone(),
            units: game.units.clone(),
            bullets: game.bullets.clone(),
            mines: game.mines.clone(),
            loot_boxes: game.loot_boxes.clone(),
            max_distance: size.norm(),
        }
    }

    pub fn update(&mut self, game: &Game) {
        let old_units_locations = get_units_locations(&self.units);
        self.current_tick = game.current_tick;
        self.players = game.players.clone();
        self.units = game.units.clone();
        self.bullets = game.bullets.clone();
        self.mines = game.mines.clone();
        self.loot_boxes = game.loot_boxes.clone();
        self.items_by_tile = game.loot_boxes.iter()
            .map(|v| (v.location(), v.item.clone()))
            .collect();
        let new_units_locations = get_units_locations(&self.units);
        self.number_of_teammates = game.units.iter().filter(|v| self.is_teammate_unit(v)).count();

        if self.unit_index.len() > self.units.len() {
            self.unit_index.retain(|&id| game.units.iter().find(|v| v.id == id).is_some());
            self.paths.retain(|&(id, _)| game.units.iter().find(|v| v.id == id).is_some());
            self.backtracks.retain(|&(id, _)| game.units.iter().find(|v| v.id == id).is_some());
        }

        if self.paths.iter().find(|v| v.1.is_empty()).is_some() || old_units_locations != new_units_locations {
            for i in 0 .. self.paths.len() {
                let unit = self.get_unit(self.paths[i].0);
                if self.is_teammate_unit(unit) {
                    let location = unit.location();
                    self.update_tile_path_infos(i, self.paths[i].0, location);
                }
            }
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

    pub fn tick_time_interval(&self) -> f64 {
        1.0 / self.properties.ticks_per_second as f64
    }

    pub fn tile(&self, location: Location) -> Tile {
        get_tile(&self.level, location)
    }

    pub fn tile_by_position(&self, position: Vec2) -> Tile {
        get_tile_by_vec2(&self.level, position)
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

    pub fn get_path_info(&self, unit_index: usize, source: Location, destination: Location) -> Option<&TilePathInfo> {
        self.paths[unit_index].1.get(&(source, destination))
    }

    pub fn has_opponent_unit(&self, location: Location) -> bool {
        self.units.iter()
            .filter(|v| self.is_opponent_unit(v))
            .find(|v| v.center().distance(location.center()) <= v.size.y)
            .is_some()
    }

    pub fn has_mine(&self, location: Location) -> bool {
        self.mines.iter()
            .find(|v| v.location().center().distance(location.center()) <= 2.0 * self.properties.mine_trigger_radius)
            .is_some()
    }

    pub fn get_backtrack(&self, unit_id: i32) -> &Vec<usize> {
        &self.backtracks.iter()
            .find(|(id, _)| *id == unit_id)
            .map(|(_, v)| v)
            .unwrap()
    }

    pub fn find_shortcut_tiles_path(&self, unit_id: i32, source: Location, destination: Location) -> Vec<Location> {
        let tiles_path = self.find_reversed_tiles_path(unit_id, source, destination);

        let mut result = Vec::new();
        let mut end = tiles_path.len();
        let mut current = source;

        while end > 0 {
            let mut tile = 0;
            while tile < end && !will_hit_by_line(current.center(), tiles_path[tile].center(), &self.level) {
                tile += 1;
            }
            if tile == tiles_path.len() {
                break;
            }
            if tile == end {
                result.push(destination);
                break;
            }
            current = tiles_path[tile];
            end = tile;
            result.push(tiles_path[end]);
        }

        result
    }

    pub fn find_reversed_tiles_path(&self, unit_id: i32, source: Location, destination: Location) -> Vec<Location> {
        let mut result = Vec::new();
        let mut index = get_tile_index(&self.level, destination);

        let backtrack = self.get_backtrack(unit_id);

        loop {
            let prev = backtrack[index];
            if prev == index {
                return Vec::new()
            }
            result.push(get_tile_location(&self.level, index));
            if prev == get_tile_index(&self.level, source) {
                break;
            }
            index = prev;
        }

        result
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

    fn update_tile_path_infos(&mut self, index: usize, unit_id: i32, source: Location) {
        use std::collections::{BTreeSet, BinaryHeap};

        let size_x = get_level_size_x(&self.level);
        let size_y = get_level_size_y(&self.level);

        let mut distances: Vec<f64> = std::iter::repeat(std::f64::MAX).take(size_x * size_y).collect();
        distances[get_tile_index(&self.level, source)] = 0.0;

        let mut has_opponent_unit: Vec<bool> = std::iter::repeat(false).take(size_x * size_y).collect();

        let mut has_mine: Vec<bool> = std::iter::repeat(false).take(size_x * size_y).collect();

        for i in 0 .. self.backtracks[index].1.len() {
            self.backtracks[index].1[i] = i;
        }

        let mut ordered: BinaryHeap<(i32, Location)> = BinaryHeap::new();
        ordered.push((0, source));

        let mut destinations: BTreeSet<Location> = BTreeSet::new();
        destinations.insert(source);

        const EDGES: &[(Vec2i, f64)] = &[
            (Vec2i::new(-1, -1), std::f64::consts::SQRT_2),
            (Vec2i::new(-1, 0), 1.0),
            // (Vec2i::new(-1, 1), std::f64::consts::SQRT_2),
            (Vec2i::new(0, -1), 1.0),
            (Vec2i::new(0, 1), 1.0),
            (Vec2i::new(1, -1), std::f64::consts::SQRT_2),
            (Vec2i::new(1, 0), 1.0),
            // (Vec2i::new(1, 1), std::f64::consts::SQRT_2),
        ];

        while let Some((_, node_location)) = ordered.pop() {
            destinations.remove(&node_location);
            for &(shift, distance) in EDGES.iter() {
                let neighbor_location = node_location + shift;
                if neighbor_location.x() >= size_x || neighbor_location.y() >= size_y
                    || !is_tile_reachable_from(node_location, neighbor_location, &self.level, self.properties()) {
                    continue;
                }
                let node_index = get_tile_index(&self.level, node_location);
                let neighbor_index = get_tile_index(&self.level, neighbor_location);
                let new_distance = distances[node_index] + distance;
                if new_distance < distances[neighbor_index] {
                    distances[neighbor_index] = new_distance;
                    has_opponent_unit[neighbor_index] = has_opponent_unit[node_index] || self.has_opponent_unit(neighbor_location);
                    has_mine[neighbor_index] = has_mine[node_index] || self.has_mine(neighbor_location);
                    self.backtracks[index].1[neighbor_index] = node_index;
                    if destinations.insert(neighbor_location) {
                        ordered.push((as_score(distance), neighbor_location));
                    }
                }
            }
        }

        for x in 0 .. size_x {
            for y in 0 .. size_y {
                let destination = Location::new(x, y);
                let tile_index = get_tile_index(&self.level, destination);
                let distance = distances[tile_index];
                if distance != std::f64::MAX {
                    self.paths[index].1.insert((source, destination), TilePathInfo {
                        distance,
                        has_opponent_unit: has_opponent_unit[tile_index],
                        has_mine: has_mine[tile_index],
                    });
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct TilePathInfo {
    has_opponent_unit: bool,
    has_mine: bool,
    distance: f64,
}

impl TilePathInfo {
    #[inline(always)]
    pub fn has_opponent_unit(&self) -> bool {
        self.has_opponent_unit
    }

    #[inline(always)]
    pub fn has_mine(&self) -> bool {
        self.has_mine
    }

    #[inline(always)]
    pub fn distance(&self) -> f64 {
        self.distance
    }
}

pub fn is_tile_reachable_from(source: Location, destination: Location, level: &Level, properties: &Properties) -> bool {
    match get_tile(level, destination) {
        Tile::Wall => false,
        Tile::Ladder => true,
        Tile::Platform => true,
        Tile::JumpPad => true,
        Tile::Empty => {
            match get_tile(level, source) {
                Tile::Wall => false,
                Tile::Ladder => true,
                Tile::Platform => true,
                Tile::JumpPad => true,
                Tile::Empty => source.y() > destination.y()
                    || (source.y() > 0
                        && (
                            is_walkable(get_tile(level, source + Vec2i::new(0, -1)))
                            || is_walkable(get_tile(level, destination + Vec2i::new(0, -1)))
                            || (2 .. source.y() as isize + 1).find(|&dy| {
                                can_jump_up_from(get_tile(level, source + Vec2i::new(0, -dy)), dy as f64 + 0.5, properties)
                            }).is_some()
                            || (1 .. destination.x() as isize).find(|&dx| {
                                can_fly_from(get_tile(level, destination + Vec2i::new(-dx, 0)), dx as f64 + 0.5, properties)
                            }).is_some()
                            || (destination.x() + 1 .. get_level_size_x(level) - 1).find(|&x| {
                                can_fly_from(get_tile(level, Location::new(x, destination.y())), (x - destination.x()) as f64 + 0.5, properties)
                            }).is_some()
                        )
                    ),
            }
        },
    }
}

pub fn can_jump_up_from(tile: Tile, height: f64, properties: &Properties) -> bool {
    match tile {
        Tile::Wall => properties.max_unit_jump_height() >= height,
        Tile::Ladder => properties.max_unit_jump_height() >= height,
        Tile::Platform => properties.max_unit_jump_height() >= height,
        Tile::JumpPad => properties.max_jump_pad_height() >= height,
        Tile::Empty => false,
    }
}

pub fn can_fly_from(tile: Tile, length: f64, properties: &Properties) -> bool {
    match tile {
        Tile::Wall => properties.max_unit_jump_length() >= length,
        Tile::Ladder => properties.max_unit_jump_length() >= length,
        Tile::Platform => properties.max_unit_jump_length() >= length,
        Tile::JumpPad => properties.max_jump_pad_length() >= length,
        Tile::Empty => false,
    }
}

pub fn is_walkable(tile: Tile) -> bool {
    match tile {
        Tile::Wall => true,
        Tile::Ladder => true,
        Tile::Platform => true,
        Tile::JumpPad => true,
        Tile::Empty => false,
    }
}

fn get_units_locations(units: &Vec<Unit>) -> Vec<(i32, Location)> {
    let mut result: Vec<(i32, Location)> = units.iter()
        .map(|v| (v.id, v.location()))
        .collect();
    result.sort();
    result
}
