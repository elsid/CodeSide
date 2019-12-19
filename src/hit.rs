use model::{
    Level,
    Mine,
    Tile,
    Unit,
};
use crate::my_strategy::{
    Location,
    Rect,
    Rectangular,
    Vec2,
    WalkGrid,
    World,
    as_score,
    get_tile,
    get_tile_by_vec2,
};

#[derive(Debug)]
pub struct HitProbabilities {
    pub wall: f64,
    pub opponent_units: f64,
    pub teammate_units: f64,
    pub opponent_mines: f64,
    pub teammate_mines: f64,
    pub target: f64,
    pub min_distance: Option<f64>,
}

pub fn get_hit_probabilities(my_id: i32, source: Vec2, target: &Target, spread: f64, bullet_size: f64, world: &World) -> HitProbabilities {
    let direction = (target.rect.center() - source).normalized();
    let to_target = direction * world.max_distance();
    let left = direction.left() * bullet_size;
    let right = direction.right() * bullet_size;
    let number_of_directions = world.config().hit_number_of_directions;

    let mut hit_wall = 0;
    let mut hit_opponent_units = 0;
    let mut hit_teammate_units = 0;
    let mut hit_opponent_mines = 0;
    let mut hit_teammate_mines = 0;
    let mut hit_target = 0;
    let mut min_distance = None;

    for i in 0 .. number_of_directions {
        let angle = ((2 * i) as f64 / (number_of_directions - 1) as f64 - 1.0) * spread;
        let destination = source + to_target.rotated(angle);
        let hits = [
            get_nearest_hit(my_id, source, destination, target, world),
            get_nearest_hit(my_id, source + left, destination + left, target, world),
            get_nearest_hit(my_id, source + right, destination + right, target, world),
        ];
        for hit in hits.into_iter().filter_map(|v| *v).min_by_key(|v| as_score(v.distance)) {
            match hit.object {
                Object::Wall => hit_wall += 1,
                Object::OpponentUnit => hit_opponent_units += 1,
                Object::TeammateUnit => hit_teammate_units += 1,
                Object::OpponentMine => hit_opponent_mines += 1,
                Object::TeammateMine => hit_teammate_mines += 1,
            }
            hit_target += hit.is_target as i32;
            if min_distance.is_none() || min_distance.unwrap() > hit.distance {
                min_distance = Some(hit.distance);
            }
        }
    }

    let number_of_rays = number_of_directions as f64;

    HitProbabilities {
        wall: hit_wall as f64 / number_of_rays,
        opponent_units: hit_opponent_units as f64 / number_of_rays,
        teammate_units: hit_teammate_units as f64 / number_of_rays,
        opponent_mines: hit_opponent_mines as f64 / number_of_rays,
        teammate_mines: hit_teammate_mines as f64 / number_of_rays,
        target: hit_target as f64 / number_of_rays,
        min_distance,
    }
}

pub enum TargetType {
    Unit { id: i32 },
    Mine { position: Vec2 },
}

pub struct Target {
    rect: Rect,
    typ: TargetType,
}

impl Target {
    pub fn from_unit(unit: &Unit) -> Self {
        Self { rect: unit.rect(), typ: TargetType::Unit { id: unit.id } }
    }

    pub fn from_mine(mine: &Mine) -> Self {
        Self { rect: mine.rect(), typ: TargetType::Mine { position: Vec2::from_model(&mine.position) } }
    }
}

#[derive(Clone, Copy)]
pub struct Hit {
    pub distance: f64,
    pub object: Object,
    pub is_target: bool,
}

#[derive(Clone, Copy)]
pub enum Object {
    Wall,
    OpponentUnit,
    TeammateUnit,
    OpponentMine,
    TeammateMine,
}

pub fn get_nearest_hit(my_id: i32, source: Vec2, destination: Vec2, target: &Target, world: &World) -> Option<Hit> {
    let target_unit_id = if let &TargetType::Unit { id } = &target.typ { id } else { -1 };
    let target_mine_position = if let &TargetType::Mine { position } = &target.typ { position } else { Vec2::zero() };
    let unit_hit = get_distance_to_nearest_hit_unit_by_line(my_id, source, destination, world)
        .map(|unit| Hit {
            distance: unit.distance,
            object: if unit.is_teammate { Object::TeammateUnit } else { Object::OpponentUnit },
            is_target: target_unit_id == unit.id,
        });
    let mine_hit = get_distance_to_nearest_hit_mine_by_line(source, destination, world)
        .map(|mine| Hit {
            distance: mine.distance,
            object: if mine.is_teammate { Object::TeammateMine } else { Object::OpponentMine },
            is_target: target_mine_position == mine.position,
        });
    let wall_hit = get_distance_to_nearest_hit_wall_by_line(source, destination, world.level())
        .map(|distance| Hit { distance, object: Object::Wall, is_target: false });
    [unit_hit, mine_hit, wall_hit].into_iter()
        .filter_map(|v| *v)
        .min_by_key(|v| as_score(v.distance))
}

pub fn get_hit_probability_by_spread(shooter: Vec2, target: &Rect, spread: f64, bullet_size: f64) -> f64 {
    target.get_max_cross_section_from(shooter, spread + bullet_size / shooter.distance(target.center()))
}

pub fn get_hit_probability_over_obstacles(shooter: &Rect, target: &Rect, level: &Level) -> f64 {
    let begin = shooter.center();
    let end = target.center();
    if begin.x() as i32 == end.x() as i32 && begin.y() as i32 == end.y() as i32 {
        return (get_tile_by_vec2(level, begin) != Tile::Wall) as i32 as f64;
    }
    if begin.x() as i32 == end.x() as i32 {
        return will_hit_by_vertical(begin, end, level) as i32 as f64;
    }
    if begin.y() as i32 == end.y() as i32 {
        return will_hit_by_horizontal(begin, end, level) as i32 as f64;
    }
    let lower = target.center() - Vec2::new(0.0, target.half().y() / 2.0);
    let upper = target.center() + Vec2::new(0.0, target.half().y() / 2.0);
    (
        will_hit_by_line(begin, end, level) as i32
        + will_hit_by_line(begin, lower, level) as i32
        + will_hit_by_line(begin, upper, level) as i32
    ) as f64 / 3.0
}

pub fn will_hit_by_vertical(begin: Vec2, end: Vec2, level: &Level) -> bool {
    let x = begin.x() as isize;
    let mut y = begin.y() as isize;
    let end_y = end.y() as isize;
    let direction = (end_y - y).signum();
    while y != end_y {
        if get_tile(level, Location::new(x as usize, y as usize)) == Tile::Wall {
            return false;
        }
        y += direction;
    }
    get_tile(level, Location::new(x as usize, y as usize)) != Tile::Wall
}

pub fn will_hit_by_horizontal(begin: Vec2, end: Vec2, level: &Level) -> bool {
    let y = begin.y() as i32;
    let mut x = begin.x() as i32;
    let end_x = end.x() as i32;
    let direction = (end_x - x).signum();
    while x != end_x {
        if get_tile(level, Location::new(x as usize, y as usize)) == Tile::Wall {
            return false;
        }
        x += direction;
    }
    get_tile(level, Location::new(x as usize, y as usize)) != Tile::Wall
}

pub fn will_hit_by_line(begin: Vec2, end: Vec2, level: &Level) -> bool {
    for position in WalkGrid::new(begin, end) {
        if get_tile_by_vec2(level, position) == Tile::Wall {
            return false;
        }
    }
    true
}

pub fn get_distance_to_nearest_hit_wall_by_line(begin: Vec2, end: Vec2, level: &Level) -> Option<f64> {
    for position in WalkGrid::new(begin, end) {
        if get_tile_by_vec2(level, position) == Tile::Wall {
            return Some(begin.distance(position));
        }
    }
    None
}

pub struct UnitHit {
    id: i32,
    distance: f64,
    is_teammate: bool,
}

pub fn get_distance_to_nearest_hit_unit_by_line(my_id: i32, source: Vec2, target: Vec2, world: &World) -> Option<UnitHit> {
    world.units().iter()
        .filter(|unit| unit.id != my_id)
        .filter_map(|unit| {
            unit.rect().get_intersection_with_line(source, target)
                .map(|v| (unit.id, v, world.is_teammate(unit)))
        })
        .min_by_key(|&(_, distance, _)| as_score(distance))
        .map(|(id, distance, is_teammate)| UnitHit { id, distance, is_teammate })
}

pub struct MineHit {
    position: Vec2,
    distance: f64,
    is_teammate: bool,
}

pub fn get_distance_to_nearest_hit_mine_by_line(source: Vec2, target: Vec2, world: &World) -> Option<MineHit> {
    world.mines().iter()
        .filter_map(|mine| {
            mine.rect().get_intersection_with_line(source, target)
                .map(|v| (Vec2::from_model(&mine.position), v, world.is_teammate_mine(mine)))
        })
        .min_by_key(|&(_, distance, _)| as_score(distance))
        .map(|(position, distance, is_teammate)| MineHit { position, distance, is_teammate })
}
