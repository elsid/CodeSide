use model::{
    Properties,
    Tile,
    Unit,
};

#[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_path"))]
use model::{
    ColorF32,
    CustomData,
};

use crate::my_strategy::{
    Debug as Dbg,
    IdGenerator,
    Identifiable,
    ImplicitProperties,
    Level,
    Location,
    Positionable,
    Vec2i,
    Visitor,
    WalkGrid,
    World,
    as_score,
    search,
};

#[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_path"))]
use crate::my_strategy::{
    Rectangular,
};

const TRANSITIONS: &[(Vec2i, f64)] = &[
    (Vec2i::new(-1, -1), std::f64::consts::SQRT_2),
    (Vec2i::new(-1, 0), 1.0),
    (Vec2i::new(-1, 1), std::f64::consts::SQRT_2),
    (Vec2i::new(0, -1), 1.0),
    (Vec2i::new(0, 1), 1.0),
    (Vec2i::new(1, -1), std::f64::consts::SQRT_2),
    (Vec2i::new(1, 0), 1.0),
    (Vec2i::new(1, 1), std::f64::consts::SQRT_2),
];

#[inline(never)]
pub fn get_optimal_path(unit: &Unit, locations_score: &Vec<i32>, world: &World, debug: &mut Dbg) -> (i32, Vec<Location>) {
    let mut visitor = VisitorImpl {
        state_id_generator: IdGenerator::new(),
        transition_id_generator: IdGenerator::new(),
        applied: std::iter::repeat(false).take(world.level().size() * TRANSITIONS.len()).collect(),
        level: world.level(),
        properties: world.properties(),
        locations_score,
    };

    let source = world.level().get_tile_index(unit.location());
    let initial_state = visitor.make_initial_state(locations_score[source], source);

    let (transitions, final_state, _) = search(world.current_tick(), initial_state, &mut visitor);

    let mut tiles_path = Vec::with_capacity(transitions.len());
    let mut location = world.level().get_tile_location(source);

    for transition in transitions.iter() {
        let next_location = location + TRANSITIONS[transition.index].0;
        if next_location != unit.location() + Vec2i::only_y(1) {
            tiles_path.push(next_location);
        }
        location = next_location;
    }

    let result = shortcut_tiles_path(world.level().get_tile_location(source), tiles_path, world.level());

    #[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_path"))]
    render_tiles_path(unit, &result, debug);

    #[cfg(all(feature = "enable_debug", feature = "enable_debug_log", feature = "enable_debug_optimal_path"))]
    debug.log(format!("[{}] tiles_path: {:?}", unit.id, result));

    (final_state.map(|v| v.score).unwrap_or(0), result)
}

struct VisitorImpl<'le, 'p, 'ls> {
    state_id_generator: IdGenerator,
    transition_id_generator: IdGenerator,
    applied: Vec<bool>,
    level: &'le Level,
    properties: &'p Properties,
    locations_score: &'ls Vec<i32>,
}

impl<'le, 'p, 'ls> VisitorImpl<'le, 'p, 'ls> {
    fn make_initial_state(&mut self, score: i32, tile_index: usize) -> State {
        State {
            id: self.state_id_generator.next(),
            score,
            tile_index,
        }
    }
}

impl<'le, 'p, 'ls> Visitor<State, Transition> for VisitorImpl<'le, 'p, 'ls> {
    fn is_final(&self, _state: &State) -> bool {
        true
    }

    fn get_transitions(&mut self, _iteration: usize, state: &State) -> Vec<Transition> {
        let mut result = Vec::with_capacity(TRANSITIONS.len());

        for index in 0 .. TRANSITIONS.len() {
            let tile_location = self.level.get_tile_location(state.tile_index);
            let neighbor_location = tile_location + TRANSITIONS[index].0;
            if neighbor_location.x() >= self.level.size_x() || neighbor_location.y() >= self.level.size_y() {
                continue;
            }
            let neighbor_index = self.level.get_tile_index(neighbor_location);
            if self.applied[state.tile_index * TRANSITIONS.len() + index] {
                continue;
            }
            let score = self.locations_score[neighbor_index];
            if score == std::i32::MIN
                    || !is_tile_reachable_from(tile_location, neighbor_location, &self.level, &self.properties) {
                continue;
            }
            result.push(Transition { id: self.transition_id_generator.next(), index });
        }

        result
    }

    fn apply(&mut self, _iteration: usize, state: &State, transition: &Transition) -> State {
        let tile_location = self.level.get_tile_location(state.tile_index);
        let neighbor_location = tile_location + TRANSITIONS[transition.index].0;
        let neighbor_index = self.level.get_tile_index(neighbor_location);

        self.applied[state.tile_index * TRANSITIONS.len() + transition.index] = true;

        State {
            id: self.state_id_generator.next(),
            score: self.locations_score[neighbor_index],
            tile_index: neighbor_index,
        }
    }

    fn get_transition_cost(&mut self, source_state: &State, destination_state: &State, transition: &Transition) -> i32 {
        source_state.score - destination_state.score + as_score(TRANSITIONS[transition.index].1)
    }

    fn get_score(&self, state: &State) -> i32 {
        state.score
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
struct AppliedState {
    tile_index: usize,
    transition_index: usize,
}

#[derive(Debug, Clone)]
struct State {
    id: i32,
    score: i32,
    tile_index: usize,
}

impl Identifiable for State {
    fn id(&self) -> i32 {
        self.id
    }
}

#[derive(Debug, Clone)]
struct Transition {
    id: i32,
    index: usize,
}

impl Identifiable for Transition {
    fn id(&self) -> i32 {
        self.id
    }
}

pub fn shortcut_tiles_path(source: Location, tiles_path: Vec<Location>, level: &Level) -> Vec<Location> {
    if tiles_path.len() <= 1 {
        return tiles_path;
    }

    let mut result = Vec::with_capacity(tiles_path.len());
    let mut next = 0;
    let mut current = source;
    let last = tiles_path.len() - 1;

    while next < last {
        let mut tile = last;
        while tile > next && !is_valid_shortcut(current, tiles_path[tile], level) {
            tile -= 1;
        }
        result.push(tiles_path[tile]);
        current = tiles_path[tile];
        if tile == next {
            next += 1;
        } else {
            next = tile + 1;
        }
    }

    result
}

fn is_valid_shortcut(begin: Location, end: Location, level: &Level) -> bool {
    if begin.x() == end.x() {
        let mut y = begin.y() as isize;
        let shift: isize = if y < end.y() as isize { 1 } else { -1 };
        while y != end.y() as isize {
            let tile = level.get_tile(Location::new(begin.x(), y as usize));
            if tile == Tile::Wall || tile == Tile::JumpPad {
                return false;
            }
            y += shift;
        }
        true
    } else if begin.y() == end.y() {
        if level.get_tile(begin) == Tile::JumpPad {
            return false;
        }
        let mut x = begin.x() as isize;
        let shift: isize = if x < end.x() as isize { 1 } else { -1 };
        while x != end.x() as isize {
            let tile = level.get_tile(Location::new(x as usize, begin.y()));
            if tile == Tile::Wall || tile == Tile::JumpPad {
                return false;
            }
            x += shift;
        }
        true
    } else {
        for position in WalkGrid::new(begin.center(), end.center()) {
            let tile = level.get_tile(position.as_location());
            if tile == Tile::Wall || tile == Tile::JumpPad {
                return false;
            }
        }
        true
    }
}

fn is_tile_reachable_from(source: Location, destination: Location, level: &Level, properties: &Properties) -> bool {
    if level.get_tile(destination + Vec2i::new(0, 1)) == Tile::Wall {
        return false;
    }
    match level.get_tile(destination) {
        Tile::Wall => false,
        Tile::Ladder | Tile::Platform | Tile::JumpPad => true,
        Tile::Empty => {
            match level.get_tile(source) {
                Tile::Wall => false,
                Tile::Ladder | Tile::Platform => true,
                Tile::JumpPad => source.y() < destination.y(),
                Tile::Empty => source.y() > destination.y()
                    || source.y() == destination.y()
                        && (is_walkable(level.get_tile(source + Vec2i::new(0, -1))) || is_walkable(level.get_tile(destination + Vec2i::new(0, -1))))
                    || source.y() < destination.y()
                        && (source.x() as isize - destination.x() as isize).abs() <= 1
                        && (1 .. source.y() as isize + 1)
                            .find(|&dy| can_jump_up_from(level.get_tile(source + Vec2i::new(0, -dy)), dy as f64, properties)).is_some(),
            }
        },
    }
}

fn can_jump_up_from(tile: Tile, height: f64, properties: &Properties) -> bool {
    match tile {
        Tile::Wall => properties.max_unit_jump_height() >= height,
        Tile::Ladder => properties.max_unit_jump_height() >= height,
        Tile::Platform => properties.max_unit_jump_height() >= height,
        Tile::JumpPad => properties.max_jump_pad_height() >= height,
        Tile::Empty => false,
    }
}

fn is_walkable(tile: Tile) -> bool {
    tile != Tile::Empty
}

#[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_plan"))]
fn render_tiles_path(unit: &Unit, tiles_path: &Vec<Location>, debug: &mut Dbg) {
    if tiles_path.is_empty() {
        return;
    }

    debug.draw(CustomData::Line {
        p1: unit.center().as_debug(),
        p2: tiles_path[0].center().as_debug(),
        width: 0.1,
        color: ColorF32 { a: 0.66, r: 0.66, g: 0.66, b: 0.0 },
    });

    for tile in 0 .. tiles_path.len() - 1 {
        debug.draw(CustomData::Line {
            p1: tiles_path[tile].center().as_debug(),
            p2: tiles_path[tile + 1].center().as_debug(),
            width: 0.1,
            color: ColorF32 { a: 0.66, r: 0.66, g: 0.66, b: 0.0 },
        });
    }
}
