use model::{
    Tile,
};

use crate::my_strategy::{
    Level,
    Location,
    Positionable,
    World,
};

struct Unit {
    id: i32,
    health: i32,
    location: Location,
    dead: bool,
}

const NEIGHBORHOOD_SIZE_X: usize = 5;
const NEIGHBORHOOD_SIZE_Y: usize = 5;
const NEIGHBORHOOD_SIZE: usize = NEIGHBORHOOD_SIZE_X * NEIGHBORHOOD_SIZE_Y;
const NEIGHBORHOOD_SHIFT_X: isize = -2;
const NEIGHBORHOOD_SHIFT_Y: isize = -1;

#[derive(Debug, PartialEq, Eq, Hash, Clone, RustcEncodable, RustcDecodable)]
pub struct Neighborhood {
    tiles: [u8; NEIGHBORHOOD_SIZE],
}

pub struct NeighborhoodScoreCounter {
    units: Vec<Unit>,
    locations: Vec<i32>,
    old_neighborhoods: std::collections::HashMap<Neighborhood, i32>,
    #[cfg(feature = "collect_neighborhood_score")]
    new_neighborhoods: std::collections::HashMap<Neighborhood, i32>,
    min_neighborhood_score: i32,
    max_neighborhood_score: i32,
}

#[cfg_attr(feature = "collect_neighborhood_score", derive(RustcEncodable, RustcDecodable))]
pub struct NeighborhoodScore {
    pub neighborhood: [u8; NEIGHBORHOOD_SIZE],
    pub score: i32,
}

impl NeighborhoodScoreCounter {
    pub fn new(world: &World) -> Self {
        Self {
            units: world.units().iter().map(|v| Unit {
                id: v.id,
                health: v.health,
                location: v.location(),
                dead: false,
            }).collect(),
            locations: std::iter::repeat(0).take(world.level().size()).collect(),
            old_neighborhoods: std::collections::HashMap::new(),
            #[cfg(feature = "collect_neighborhood_score")]
            new_neighborhoods: std::collections::HashMap::new(),
            min_neighborhood_score: 0,
            max_neighborhood_score: 0,
        }
    }

    pub fn get_location_score(&self, index: usize) -> i32 {
        self.locations[index]
    }

    pub fn get_neighborhood_score(&self, neighborhood: &Neighborhood) -> i32 {
        *self.old_neighborhoods.get(neighborhood).unwrap_or(&0)
    }

    pub fn min_neighborhood_score(&self) -> i32 {
        self.min_neighborhood_score
    }

    pub fn max_neighborhood_score(&self) -> i32 {
        self.max_neighborhood_score
    }

    #[cfg(feature = "collect_neighborhood_score")]
    pub fn get_neighborhoods(&self) -> Vec<NeighborhoodScore> {
        self.new_neighborhoods.iter()
            .map(|(k, v)| NeighborhoodScore { neighborhood: k.tiles.clone(), score: *v})
            .collect()
    }

    pub fn fill(&mut self, values: &[NeighborhoodScore]) {
        for value in values.iter() {
            self.min_neighborhood_score = self.min_neighborhood_score.min(value.score);
            self.max_neighborhood_score = self.max_neighborhood_score.max(value.score);
            self.old_neighborhoods.insert(Neighborhood { tiles: value.neighborhood.clone() }, value.score);
        }
    }

    pub fn update(&mut self, world: &World) {
        for unit in world.units().iter() {
            let unit_index = self.units.iter().position(|v| v.id == unit.id).unwrap();
            self.units[unit_index].location = unit.location();
            let damage = self.units[unit_index].health - unit.health;
            if damage > 0 {
                self.units[unit_index].health = unit.health;
                #[cfg(feature = "collect_neighborhood_score")]
                {
                    let entry = self.new_neighborhoods.entry(world.get_neighborhood(unit.location()).clone()).or_insert(0);
                    *entry -= damage;
                }
                self.locations[world.level().get_tile_index(unit.location())] -= damage;
            }
        }

        for unit in 0 .. self.units.len() {
            if !self.units[unit].dead && world.units().iter().find(|v| v.id == self.units[unit].id).is_none() {
                self.units[unit].dead = true;
                let score = self.units[unit].health + world.properties().kill_score;
                #[cfg(feature = "collect_neighborhood_score")]
                {
                    let entry = self.new_neighborhoods.entry(world.get_neighborhood(self.units[unit].location).clone()).or_insert(0);
                    *entry -= score;
                }
                self.locations[world.level().get_tile_index(self.units[unit].location)] -= score;
            }
        }
    }
}

pub fn get_neighborhood(location: Location, level: &Level) -> Neighborhood {
    let mut tiles: [u8; NEIGHBORHOOD_SIZE] = [NONE; NEIGHBORHOOD_SIZE];

    for x in 0 .. NEIGHBORHOOD_SIZE_X {
        for y in 0 .. NEIGHBORHOOD_SIZE_Y {
            let location_x = location.x() as isize + x as isize + NEIGHBORHOOD_SHIFT_X;
            let location_y = location.y() as isize + y as isize + NEIGHBORHOOD_SHIFT_Y;
            if 0 <= location_x && location_x < level.size_x() as isize
                    && 0 <= location_y && location_y < level.size_y() as isize {
                tiles[x * NEIGHBORHOOD_SIZE_Y + y] = get_tile_code(level.get_tile(Location::new(location_x as usize, location_y as usize)));
            }
        }
    }

    Neighborhood { tiles }
}

const NONE: u8 = 0;
const EMPTY: u8 = 1;
const WALL: u8 = 2;
const PLATFORM: u8 = 3;
const LADDER: u8 = 4;
const JUMP_PAD: u8 = 5;

fn get_tile_code(tile: Tile) -> u8 {
    match tile {
        Tile::Empty => EMPTY,
        Tile::Wall => WALL,
        Tile::Platform => PLATFORM,
        Tile::Ladder => LADDER,
        Tile::JumpPad => JUMP_PAD,
    }
}
