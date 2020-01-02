use model::{
    ExplosionParams,
    Tile,
    Unit,
};

use crate::my_strategy::{
    Level,
    Location,
    Rect,
    Rectangular,
    Sector,
    Vec2,
    WalkGrid,
    World,
    as_score,
    make_location_rect,
    normalize_angle,
};

#[derive(Debug)]
pub struct HitProbabilities {
    pub wall: usize,
    pub opponent_units: usize,
    pub teammate_units: usize,
    pub opponent_mines: usize,
    pub teammate_mines: usize,
    pub target: usize,
    pub total: usize,
    pub min_distance: Option<f64>,
    pub opponent_units_damage: i32,
    pub teammate_units_damage: i32,
    pub target_damage: i32,
    pub opponent_units_kills: usize,
    pub teammate_units_kills: usize,
    pub target_kills: usize,
}

#[derive(Debug)]
pub struct Target {
    id: i32,
    rect: Rect,
}

impl Target {
    pub fn new(id: i32, rect: Rect) -> Self {
        Self { id, rect }
    }

    pub fn from_unit(unit: &Unit) -> Self {
        Self { id: unit.id, rect: unit.rect() }
    }
}

#[inline(never)]
pub fn get_hit_probabilities(unit_id: i32, source: Vec2, direction: Vec2, target: &Target,
        spread: f64, bullet_size: f64, bullet_damage: i32, explosion: &Option<ExplosionParams>,
        world: &World, number_of_directions: usize) -> HitProbabilities {
    let to_target = direction * world.max_distance();
    let left = direction.left() * bullet_size;
    let right = direction.right() * bullet_size;

    let mut hit_wall = 0;
    let mut hit_opponent_units = 0;
    let mut hit_teammate_units = 0;
    let mut hit_opponent_mines = 0;
    let mut hit_teammate_mines = 0;
    let mut hit_target = 0;
    let mut min_distance = None;
    let mut opponent_units_damage = 0;
    let mut teammate_units_damage = 0;
    let mut target_damage = 0;
    let mut opponent_units_kills = 0;
    let mut teammate_units_kills = 0;
    let mut target_kills = 0;
    let mut units_damage = world.units().iter().map(|v| (v.id, 0)).collect::<Vec<_>>();

    for i in 0 .. number_of_directions {
        let angle = normalize_angle(((2 * i) as f64 / (number_of_directions - 1) as f64 - 1.0) * spread);
        let far_destination = source + to_target.rotated(angle);
        let destination = source + (far_destination - source)
            * world.rect().get_intersection_with_line(source, far_destination).unwrap();
        let (src, dst) = if i == 0 {
            (source + right, destination + right)
        } else if i == number_of_directions - 1 {
            (source + left, destination + left)
        } else {
            (source, destination)
        };
        if let Some(hit) = get_nearest_hit(unit_id, src, dst, target, world) {
            hit_opponent_units += !hit.is_target as usize & !hit.is_teammate as usize & (hit.object_type == ObjectType::Unit) as usize;
            hit_teammate_units += !hit.is_target as usize & hit.is_teammate as usize & (hit.object_type == ObjectType::Unit) as usize;
            hit_opponent_mines += !hit.is_target as usize & !hit.is_teammate as usize & (hit.object_type == ObjectType::Mine) as usize;
            hit_teammate_mines += !hit.is_target as usize & hit.is_teammate as usize & (hit.object_type == ObjectType::Mine) as usize;
            hit_wall += (hit.object_type == ObjectType::Wall) as usize;
            hit_target += hit.is_target as usize;
            if min_distance.is_none() || min_distance.unwrap() > hit.distance {
                min_distance = Some(hit.distance);
            }
            opponent_units_damage += bullet_damage * (!hit.is_target as usize & !hit.is_teammate as usize & (hit.object_type == ObjectType::Unit) as usize) as i32;
            teammate_units_damage += bullet_damage * (!hit.is_target as usize & hit.is_teammate as usize & (hit.object_type == ObjectType::Unit) as usize) as i32;
            target_damage += bullet_damage * hit.is_target as i32;
            if let Some(unit_id) = hit.unit_id {
                units_damage.iter_mut().find(|(id, _)| *id == unit_id).unwrap().1 += bullet_damage;
            }
            if let Some(explosion) = explosion {
                let mut explosions = vec![(explosion, source + Vec2::only_x(hit.distance).rotated(angle))];
                while let Some((explosion, center)) = explosions.pop() {
                    let explosion_rect = Rect::new(center, Vec2::new(explosion.radius, explosion.radius));
                    for unit in world.units().iter() {
                        if explosion_rect.has_collision(&unit.rect()) {
                            if unit.id == target.id {
                                target_damage += explosion.damage;
                            } else if world.is_teammate_unit(unit) {
                                teammate_units_damage += explosion.damage;
                            } else {
                                opponent_units_damage += explosion.damage;
                            }
                            units_damage.iter_mut().find(|(id, _)| *id == unit_id).unwrap().1 += explosion.damage;
                        }
                    }
                    for mine in world.mines().iter() {
                        if explosion_rect.has_collision(&mine.rect()) {
                            explosions.push((&mine.explosion_params, mine.center()));
                        }
                    }
                }
            }
            for (damaged_unit_id, damage) in units_damage.iter_mut() {
                let unit = world.get_unit(*damaged_unit_id);
                if *damage >= unit.health {
                    opponent_units_kills += world.is_opponent_unit(unit) as usize;
                    teammate_units_kills += world.is_teammate_unit(unit) as usize;
                    target_kills += (*damaged_unit_id == unit_id) as usize;
                }
                *damage = 0;
            }
        }
    }

    HitProbabilities {
        wall: hit_wall,
        opponent_units: hit_opponent_units,
        teammate_units: hit_teammate_units,
        opponent_mines: hit_opponent_mines,
        teammate_mines: hit_teammate_mines,
        target: hit_target,
        total: number_of_directions,
        min_distance,
        opponent_units_damage,
        teammate_units_damage,
        target_damage,
        opponent_units_kills,
        teammate_units_kills,
        target_kills,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub distance: f64,
    pub object_type: ObjectType,
    pub is_target: bool,
    pub is_teammate: bool,
    pub unit_id: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum ObjectType {
    Mine,
    Unit,
    Wall,
}

#[inline(never)]
pub fn get_nearest_hit(unit_id: i32, source: Vec2, mut destination: Vec2, target: &Target, world: &World) -> Option<Hit> {
    let to_destination = destination - source;
    let destination_distance = to_destination.norm();
    let mut max_distance = destination_distance;
    let direction = to_destination / max_distance;

    let mut hit = if let Some(factor) = target.rect.get_intersection_with_line(source, destination) {
        max_distance = factor * destination_distance;
        Some(Hit {
            distance: max_distance,
            object_type: ObjectType::Unit,
            is_target: true,
            is_teammate: false,
            unit_id: Some(unit_id),
        })
    } else {
        None
    };

    if let Some(unit_hit) = get_distance_to_nearest_hit_unit_by_line(unit_id, target.id, source, destination, world) {
        if max_distance > unit_hit.distance {
            max_distance = unit_hit.distance;
            destination = source + direction * unit_hit.distance;
            hit = Some(Hit {
                distance: max_distance,
                object_type: ObjectType::Unit,
                is_target: target.id == unit_hit.id,
                is_teammate: unit_hit.is_teammate,
                unit_id: Some(unit_hit.id),
            });
        }
    }

    if let Some(mine_hit) = get_distance_to_nearest_hit_mine_by_line(source, destination, world) {
        if max_distance > mine_hit.distance {
            max_distance = mine_hit.distance;
            destination = source + direction * mine_hit.distance;
            hit = Some(Hit {
                distance: mine_hit.distance,
                object_type: ObjectType::Mine,
                is_target: false,
                is_teammate: mine_hit.is_teammate,
                unit_id: None,
            });
        }
    }

    if let Some(distance) = get_distance_to_nearest_hit_wall(source, destination, world.level()) {
        if max_distance > distance {
            hit = Some(Hit {
                distance: distance,
                object_type: ObjectType::Wall,
                is_target: false,
                is_teammate: false,
                unit_id: None,
            });
        }
    }

    hit
}

pub fn get_hit_probability_by_spread(source: Vec2, target: &Rect, spread: f64, bullet_size: f64) -> f64 {
    get_hit_probability_by_spread_with_destination(source, target.center(), target, spread, bullet_size)
}

pub fn get_hit_probability_by_spread_with_destination(source: Vec2, destination: Vec2, target: &Rect, spread: f64, bullet_size: f64) -> f64 {
    Sector::from_direction_and_spread(destination - source, spread + bullet_size / source.distance(target.center()))
        .get_intersection_fraction(Sector::from_source_and_rect(source, target))
}

fn get_distance_to_nearest_hit_wall(begin: Vec2, end: Vec2, level: &Level) -> Option<f64> {
    if begin.x() as i32 == end.x() as i32 {
        get_distance_to_nearest_hit_wall_by_vertical(begin, end, level)
    } else if begin.y() as i32 == end.y() as i32 {
        get_distance_to_nearest_hit_wall_by_horizontal(begin, end, level)
    } else {
        get_distance_to_nearest_hit_wall_by_line(begin, end, level)
    }
}

pub fn get_distance_to_nearest_hit_wall_by_vertical(begin: Vec2, end: Vec2, level: &Level) -> Option<f64> {
    let x = begin.x() as isize;
    let mut y = begin.y() as isize;
    let end_y = end.y() as isize;
    let direction = (end_y - y).signum();
    while y != end_y {
        if level.get_tile(Location::new(x as usize, y as usize)) == Tile::Wall {
            if (y as f64) < begin.y() {
                return Some(begin.y() - (y + 1) as f64);
            } else {
                return Some(y as f64 - begin.y());
            }
        }
        y += direction;
    }
    if level.get_tile(Location::new(x as usize, y as usize)) == Tile::Wall {
        if (y as f64) < begin.y() {
            return Some(begin.y() - (y + 1) as f64);
        } else {
            return Some(y as f64 - begin.y());
        }
    } else {
        None
    }
}

pub fn get_distance_to_nearest_hit_wall_by_horizontal(begin: Vec2, end: Vec2, level: &Level) -> Option<f64> {
    let y = begin.y() as i32;
    let mut x = begin.x() as i32;
    let end_x = end.x() as i32;
    let direction = (end_x - x).signum();
    while x != end_x {
        if level.get_tile(Location::new(x as usize, y as usize)) == Tile::Wall {
            if (x as f64) < begin.x() {
                return Some(begin.x() - (x + 1) as f64);
            } else {
                return Some(x as f64 - begin.x());
            }
        }
        x += direction;
    }
    if level.get_tile(Location::new(x as usize, y as usize)) == Tile::Wall {
        if (x as f64) < begin.x() {
            return Some(begin.x() - (x + 1) as f64);
        } else {
            return Some(x as f64 - begin.x());
        }
    } else {
        None
    }
}

pub fn wall_or_jump_pad_on_the_way(begin: Vec2, end: Vec2, level: &Level) -> bool {
    for position in WalkGrid::new(begin, end) {
        let tile = level.get_tile(position.as_location());
        if tile == Tile::Wall || tile == Tile::JumpPad {
            return true;
        }
    }
    false
}

pub fn get_distance_to_nearest_hit_wall_by_line(begin: Vec2, end: Vec2, level: &Level) -> Option<f64> {
    for position in WalkGrid::new(begin, end) {
        if level.get_tile(position.as_location()) == Tile::Wall {
            let rect = make_location_rect(position.as_location());
            let factor = rect.get_intersection_with_line(begin, end);
            assert!(factor.is_some(), "{:?} {:?} {:?}", rect, begin, end);
            return Some(factor.unwrap() * begin.distance(end));
        }
    }
    None
}

#[derive(Debug)]
pub struct UnitHit {
    id: i32,
    distance: f64,
    is_teammate: bool,
}

fn get_distance_to_nearest_hit_unit_by_line(unit_id: i32, target_id: i32, source: Vec2, target: Vec2, world: &World) -> Option<UnitHit> {
    world.units().iter()
        .filter(|unit| unit.id != unit_id && unit.id != target_id)
        .filter_map(|unit| {
            unit.rect().get_intersection_with_line(source, target)
                .map(|v| (unit.id, v, world.is_teammate_unit(unit)))
        })
        .min_by_key(|&(_, factor, _)| as_score(factor))
        .map(|(id, factor, is_teammate)| UnitHit { id, distance: factor * target.distance(source), is_teammate })
}

#[derive(Debug)]
pub struct MineHit {
    distance: f64,
    is_teammate: bool,
}

fn get_distance_to_nearest_hit_mine_by_line(source: Vec2, target: Vec2, world: &World) -> Option<MineHit> {
    world.mines().iter()
        .filter_map(|mine| {
            mine.rect().get_intersection_with_line(source, target)
                .map(|v| (v, world.is_teammate_mine(mine)))
        })
        .min_by_key(|&(factor, _)| as_score(factor))
        .map(|(factor, is_teammate)| MineHit { distance: factor * target.distance(source), is_teammate })
}
