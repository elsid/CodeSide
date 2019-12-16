use std::collections::BTreeMap;
use model::{
    Bullet,
    Game,
    Item,
    Level,
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
    Location,
    Positionable,
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
    me: Unit,
    game: Game,
    size: Vec2,
    items_by_tile: BTreeMap<Location, Item>,
    paths: Vec<BTreeMap<(Location, Location), TilePathInfo>>,
    backtracks: Vec<Vec<usize>>,
    teammates: Vec<i32>,
    me_index: usize,
    changed_locations: bool,
    max_distance: f64,
    number_of_teammates: usize,
}

impl World {
    pub fn new(config: Config, me: Unit, game: Game) -> Self {
        let teammates: Vec<i32> = game.units.iter()
            .filter(|v| v.player_id == me.player_id)
            .map(|v| v.id)
            .collect();
        let size = Vec2::new(get_level_size_x(&game.level) as f64, get_level_size_y(&game.level) as f64);
        Self {
            size,
            items_by_tile: game.loot_boxes.iter()
                .map(|v| (v.location(), v.item.clone()))
                .collect(),
            paths: (0 .. teammates.len()).map(|_| BTreeMap::new()).collect(),
            backtracks: (0 .. teammates.len()).map(|_| Vec::new()).collect(),
            number_of_teammates: teammates.len(),
            teammates,
            config,
            me,
            game,
            me_index: 0,
            changed_locations: true,
            max_distance: size.norm(),
        }
    }

    pub fn update(&mut self, game: &Game) {
        let old_units_locations = get_units_locations(&self.game.units);
        self.game = game.clone();
        self.items_by_tile = game.loot_boxes.iter()
            .map(|v| (v.location(), v.item.clone()))
            .collect();
        let new_units_locations = get_units_locations(&self.game.units);
        self.changed_locations = self.paths.iter().find(|v| v.is_empty()).is_some() || old_units_locations != new_units_locations;
        self.number_of_teammates = game.units.iter().filter(|v| self.is_teammate(v)).count();
    }

    pub fn update_me(&mut self, me: &Unit) {
        self.me = me.clone();
        self.me_index = self.teammates.iter().position(|&v| v == self.me.id).unwrap();
        if self.changed_locations {
            let source = me.location();
            let (infos, backtrack) = get_tile_path_infos(source, self);
            for (destination, info) in infos.into_iter() {
                self.paths[self.me_index].insert((source, destination), info);
            }
            self.backtracks[self.me_index] = backtrack;
        }
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

    pub fn mines(&self) -> &Vec<Mine> {
        &self.game.mines
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

    pub fn max_distance(&self) -> f64 {
        self.max_distance
    }

    pub fn number_of_teammates(&self) -> usize {
        self.number_of_teammates
    }

    pub fn me_index(&self) -> usize {
        self.me_index
    }

    pub fn tick_time_interval(&self) -> f64 {
        1.0 / self.game.properties.ticks_per_second as f64
    }

    pub fn tile(&self, location: Location) -> Tile {
        get_tile(&self.game.level, location)
    }

    pub fn tile_by_position(&self, position: Vec2) -> Tile {
        get_tile_by_vec2(&self.game.level, position)
    }

    pub fn get_unit(&self, id: i32) -> &Unit {
        self.game.units.iter()
            .find(|v| v.id == id)
            .unwrap()
    }

    pub fn tile_item(&self, location: Location) -> Option<&Item> {
        self.items_by_tile.get(&location)
    }

    pub fn path_info(&self, source: Location, destination: Location) -> Option<&TilePathInfo> {
        self.paths[self.me_index].get(&(source, destination))
    }

    pub fn has_opponent_unit(&self, location: Location) -> bool {
        self.game.units.iter()
            .filter(|v| v.player_id != self.me.player_id)
            .find(|v| {
                let unit_location = v.location();
                unit_location == location || unit_location + Vec2i::new(0, 1) == location
            })
            .is_some()
    }

    pub fn has_mine(&self, location: Location) -> bool {
        self.game.mines.iter()
            .filter(|v| v.player_id != self.me.player_id)
            .find(|v| v.location() == location)
            .is_some()
    }

    pub fn paths(&self) -> &BTreeMap<(Location, Location), TilePathInfo> {
        &self.paths[self.me_index]
    }

    pub fn backtrack(&self) -> &Vec<usize> {
        &self.backtracks[self.me_index]
    }

    pub fn find_shortcut_tiles_path(&self, source: Location, destination: Location) -> Vec<Location> {
        let tiles_path = self.find_reversed_tiles_path(source, destination);

        let mut result = Vec::new();
        let mut end = tiles_path.len();
        let mut current = source;

        while end > 0 {
            let mut tile = 0;
            while tile < end && !will_hit_by_line(current.center(), tiles_path[tile].center(), &self.game.level) {
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

    pub fn find_reversed_tiles_path(&self, source: Location, destination: Location) -> Vec<Location> {
        let mut result = Vec::new();
        let mut index = get_tile_index(&self.game.level, destination);

        loop {
            let prev = self.backtracks[self.me_index][index];
            if prev == index {
                return Vec::new()
            }
            result.push(get_tile_location(&self.game.level, index));
            if prev == get_tile_index(&self.game.level, source) {
                break;
            }
            index = prev;
        }

        result
    }

    pub fn is_teammate(&self, unit: &Unit) -> bool {
        self.me.id != unit.id && self.me.player_id == unit.player_id
    }

    pub fn is_opponent(&self, unit: &Unit) -> bool {
        self.me.player_id != unit.player_id
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

pub fn get_tile_path_infos(from: Location, world: &World) -> (Vec<(Location, TilePathInfo)>, Vec<usize>) {
    use std::collections::{BTreeSet, BinaryHeap};

    let size_x = get_level_size_x(world.level());
    let size_y = get_level_size_y(world.level());

    let mut distances: Vec<f64> = std::iter::repeat(std::f64::MAX).take(size_x * size_y).collect();
    distances[get_tile_index(world.level(), from)] = 0.0;

    let mut has_opponent_unit: Vec<bool> = std::iter::repeat(false).take(size_x * size_y).collect();

    let mut has_mine: Vec<bool> = std::iter::repeat(false).take(size_x * size_y).collect();

    let mut backtrack: Vec<usize> = (0 .. size_x * size_y).collect();

    let mut ordered: BinaryHeap<(i32, Location)> = BinaryHeap::new();
    ordered.push((0, from));

    let mut destinations: BTreeSet<Location> = BTreeSet::new();
    destinations.insert(from);

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
                || !is_tile_reachable_from(node_location, neighbor_location, world.level(), world.properties()) {
                continue;
            }
            let node_index = get_tile_index(world.level(), node_location);
            let neighbor_index = get_tile_index(world.level(), neighbor_location);
            let new_distance = distances[node_index] + distance;
            if new_distance < distances[neighbor_index] {
                distances[neighbor_index] = new_distance;
                has_opponent_unit[neighbor_index] = has_opponent_unit[node_index] || world.has_opponent_unit(neighbor_location);
                has_mine[neighbor_index] = has_mine[node_index] || world.has_mine(neighbor_location);
                backtrack[neighbor_index] = node_index;
                if destinations.insert(neighbor_location) {
                    ordered.push((as_score(distance), neighbor_location));
                }
            }
        }
    }

    let mut result = Vec::new();

    for x in 0 .. size_x {
        for y in 0 .. size_y {
            let location = Location::new(x, y);
            let index = get_tile_index(world.level(), location);
            let distance = distances[index];
            if distance != std::f64::MAX {
                result.push((location, TilePathInfo {
                    distance,
                    has_opponent_unit: has_opponent_unit[index],
                    has_mine: has_mine[index],
                }));
            }
        }
    }

    (result, backtrack)
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
