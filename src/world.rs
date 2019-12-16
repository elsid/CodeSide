use std::collections::BTreeMap;
use model::{
    Bullet,
    Game,
    Item,
    JumpState,
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
    Location,
    Positionable,
    Supercover,
    Vec2,
    Vec2i,
    as_score,
    get_level_size_x,
    get_level_size_y,
    get_tile,
    get_tile_by_vec2,
    get_tile_index,
    get_tile_location,
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
    jump_states: Vec<Vec<JumpState>>,
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
            jump_states: (0 .. teammates.len()).map(|_| Vec::new()).collect(),
            number_of_teammates: teammates.len() - 1,
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
            let (infos, backtrack, jump_states) = get_tile_path_infos(source, self);
            for (destination, info) in infos.into_iter() {
                self.paths[self.me_index].insert((source, destination), info);
            }
            self.backtracks[self.me_index] = backtrack;
            self.jump_states[self.me_index] = jump_states;
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

    pub fn jump_states(&self) -> &Vec<JumpState> {
        &self.jump_states[self.me_index]
    }

    pub fn find_shortcut_tiles_path(&self, source: Location, destination: Location) -> Vec<Location> {
        let tiles_path = self.find_reversed_tiles_path(source, destination);

        if tiles_path.len() <= 1 {
            return tiles_path;
        }

        let mut result = Vec::new();
        let mut end = tiles_path.len();
        let mut current = source;

        while end > 1 {
            let skip = tiles_path.len() - end;
            let mut tile = tiles_path.iter().rev()
                .skip(skip)
                .position(|&v| get_tile(&self.game.level, v) == Tile::JumpPad)
                .map(|v| tiles_path.len() - v - 1 - skip)
                .unwrap_or(0);
            while tile < end && !is_valid_shortcut(current, tiles_path[tile], self.jump_states()[get_tile_index(&self.game.level, current)].clone(), &self.game.level, &self.game.properties) {
                tile += 1;
            }
            if tile == tiles_path.len() {
                tile -= 1;
            } else if tile == end {
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

    pub fn is_me(&self, unit: &Unit) -> bool {
        self.me.id == unit.id
    }

    pub fn current_tick(&self) -> i32 {
        self.game.current_tick
    }

    pub fn is_teammate_mine(&self, mine: &Mine) -> bool {
        mine.player_id == self.me.player_id
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

pub fn get_tile_path_infos(source: Location, world: &World) -> (Vec<(Location, TilePathInfo)>, Vec<usize>, Vec<JumpState>) {
    use std::collections::{BTreeSet, BinaryHeap};

    let size_x = get_level_size_x(world.level());
    let size_y = get_level_size_y(world.level());

    let mut times: Vec<f64> = std::iter::repeat(std::f64::MAX).take(size_x * size_y).collect();
    times[get_tile_index(world.level(), source)] = 0.0;

    let mut jump_states: Vec<JumpState> = std::iter::repeat(JumpState { can_jump: false, speed: 0.0, max_time: 0.0, can_cancel: false }).take(size_x * size_y).collect();
    jump_states[get_tile_index(world.level(), source)] = world.me().jump_state.clone();

    let mut has_opponent_unit: Vec<bool> = std::iter::repeat(false).take(size_x * size_y).collect();

    let mut has_mine: Vec<bool> = std::iter::repeat(false).take(size_x * size_y).collect();

    let mut backtrack: Vec<usize> = (0 .. size_x * size_y).collect();

    let mut ordered: BinaryHeap<(i32, Location)> = BinaryHeap::new();
    ordered.push((0, source));

    let mut destinations: BTreeSet<Location> = BTreeSet::new();
    destinations.insert(source);

    const EDGES: &[Vec2i] = &[
        Vec2i::new(-1, -1),
        Vec2i::new(-1, 0),
        Vec2i::new(-1, 1),
        Vec2i::new(0, -1),
        Vec2i::new(0, 1),
        Vec2i::new(1, -1),
        Vec2i::new(1, 0),
        Vec2i::new(1, 1),
    ];

    while let Some((_, node_location)) = ordered.pop() {
        destinations.remove(&node_location);
        for &shift in EDGES.iter() {
            let neighbor_location = node_location + shift;
            if neighbor_location.x() >= size_x || neighbor_location.y() >= size_y || get_tile(world.level(), neighbor_location) == Tile::Wall {
                continue;
            }
            let node_index = get_tile_index(world.level(), node_location);
            let jump_state = &jump_states[node_index];
            if let Some((time, neighbor_jump_state)) = get_time_and_jump_state(node_location, neighbor_location, jump_state, world.level(), world.properties()) {
                let neighbor_index = get_tile_index(world.level(), neighbor_location);
                let new_time = times[node_index] + time;
                let new_jump_state = change_jump_state(jump_state, &neighbor_jump_state, time);
                if new_time < times[neighbor_index] {
                    times[neighbor_index] = new_time;
                    jump_states[neighbor_index] = new_jump_state;
                    has_opponent_unit[neighbor_index] = has_opponent_unit[node_index] || world.has_opponent_unit(neighbor_location);
                    has_mine[neighbor_index] = has_mine[node_index] || world.has_mine(neighbor_location);
                    backtrack[neighbor_index] = node_index;
                    if destinations.insert(neighbor_location) {
                        ordered.push((as_score(time), neighbor_location));
                    }
                }
            }
        }
    }

    let mut result = Vec::new();

    for x in 0 .. size_x {
        for y in 0 .. size_y {
            let location = Location::new(x, y);
            let index = get_tile_index(world.level(), location);
            let distance = times[index];
            if distance != std::f64::MAX {
                result.push((location, TilePathInfo {
                    distance,
                    has_opponent_unit: has_opponent_unit[index],
                    has_mine: has_mine[index],
                }));
            }
        }
    }

    (result, backtrack, jump_states)
}

pub fn is_valid_shortcut(begin: Location, end: Location, mut jump_state: JumpState, level: &Level, properties: &Properties) -> bool {
    for position in WalkGrid::new(begin.center(), end.center()) {
        if get_tile_by_vec2(level, position) == Tile::Wall {
            return false;
        }
    }
    let mut prev = begin;
    for location in Supercover::new(begin, end) {
        if location != prev {
            if let Some((time, next_jump_state)) = get_time_and_jump_state(prev, location, &jump_state, level, properties) {
                jump_state = change_jump_state(&jump_state, &next_jump_state, time);
            } else {
                return false;
            }
        }
        prev = location;
    }
    true
}

pub fn get_time_and_jump_state(source: Location, destination: Location, jump_state: &JumpState, level: &Level, properties: &Properties) -> Option<(f64, JumpState)> {
    if let Some(next_jump_state) = get_tile_jump_state(destination, level, properties) {
        if source.x() != destination.x() && source.y() == destination.y() && can_move_by_horizontal(source, destination, jump_state, level) {
            Some((1.0 / properties.unit_max_horizontal_speed, next_jump_state))
        } else if (!jump_state.can_jump || jump_state.can_cancel) && source.x() == destination.x() && source.y() > destination.y() {
            Some((1.0 / (properties.unit_fall_speed), next_jump_state))
        } else if jump_state.can_jump && source.x() == destination.x() && source.y() < destination.y() {
            Some((1.0 / jump_state.speed, next_jump_state))
        } else if (!jump_state.can_jump || jump_state.can_cancel) && source.x() != destination.x() && source.y() > destination.y() {
            Some((std::f64::consts::SQRT_2 / Vec2::new(properties.unit_max_horizontal_speed, properties.unit_fall_speed * 0.98).norm(), next_jump_state))
        } else if jump_state.can_jump && source.x() != destination.x() && source.y() < destination.y() && get_tile(level, Location::new(source.x(), destination.y())) != Tile::Wall {
            Some((std::f64::consts::SQRT_2 / Vec2::new(properties.unit_max_horizontal_speed, jump_state.speed * 0.99).norm(), next_jump_state))
        } else {
            None
        }
    } else {
        None
    }
}

pub fn can_move_by_horizontal(source: Location, destination: Location, jump_state: &JumpState, level: &Level) -> bool {
    let source_walkable = is_walkable(get_tile(level, source + Vec2i::new(0, -1)));
    let destination_walkable = is_walkable(get_tile(level, destination + Vec2i::new(0, -1)));
    let source_tile = get_tile(level, source);
    let destination_tile = get_tile(level, destination);
        source_walkable
        || (source_tile == Tile::Ladder && destination_walkable)
        || (source_tile == Tile::Ladder && destination_tile == Tile::Ladder)
        || (jump_state.max_time > 0.0 && (destination_walkable || destination_tile == Tile::Ladder || destination_tile == Tile::JumpPad))
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

pub fn get_tile_jump_state(location: Location, level: &Level, properties: &Properties) -> Option<JumpState> {
    match get_tile(level, location) {
        Tile::Wall => None,
        Tile::Ladder => Some(JumpState {
            can_jump: true,
            speed: properties.unit_jump_speed,
            max_time: properties.unit_jump_time,
            can_cancel: true,
        }),
        Tile::JumpPad => Some(JumpState {
            can_jump: true,
            speed: properties.jump_pad_jump_speed,
            max_time: properties.jump_pad_jump_time,
            can_cancel: false,
        }),
        Tile::Platform | Tile::Empty => if is_walkable(get_tile(level, location + Vec2i::new(0, -1))) {
            Some(JumpState {
                can_jump: true,
                speed: properties.unit_jump_speed,
                max_time: properties.unit_jump_time,
                can_cancel: true,
            })
        } else {
            Some(JumpState {
                can_jump: false,
                speed: 0.0,
                max_time: 0.0,
                can_cancel: false,
            })
        },
    }
}

pub fn change_jump_state(source: &JumpState, destination: &JumpState, time: f64) -> JumpState {
    if source.can_jump {
        let max_time = source.max_time - time;
        if destination.can_jump {
            if destination.can_cancel {
                let can_cancel = source.can_cancel && destination.can_cancel;
                let save_jump = !can_cancel && max_time > 0.0;
                JumpState {
                    can_jump: true,
                    speed: if save_jump { source.speed } else { destination.speed },
                    max_time: if save_jump { max_time } else { destination.max_time },
                    can_cancel: !save_jump,
                }
            } else {
                JumpState {
                    can_jump: true,
                    speed: destination.speed,
                    max_time: destination.max_time,
                    can_cancel: false,
                }
            }
        } else {
            if max_time > 0.0 {
                JumpState {
                    can_jump: true,
                    speed: source.speed,
                    max_time: max_time,
                    can_cancel: source.can_cancel,
                }
            } else {
                JumpState {
                    can_jump: false,
                    speed: 0.0,
                    max_time: 0.0,
                    can_cancel: false,
                }
            }
        }
    } else {
        if destination.can_jump {
            JumpState {
                can_jump: true,
                speed: destination.speed,
                max_time: destination.max_time,
                can_cancel: destination.can_cancel,
            }
        } else {
            JumpState {
                can_jump: false,
                speed: 0.0,
                max_time: 0.0,
                can_cancel: false,
            }
        }
    }
}

fn get_units_locations(units: &Vec<Unit>) -> Vec<(i32, Location, bool, f64, f64, bool)> {
    let mut result: Vec<(i32, Location, bool, f64, f64, bool)> = units.iter()
        .map(|v| (v.id, v.location(), v.jump_state.can_jump, v.jump_state.speed, v.jump_state.max_time, v.jump_state.can_cancel))
        .collect();
    result.sort_by_key(|&(id, _, _, _, _, _)| id);
    result
}
