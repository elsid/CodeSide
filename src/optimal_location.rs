#[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_location"))]
use model::{
    CustomData,
    Vec2F32,
};

use model::{
    Item,
    Tile,
    Unit,
    Weapon,
    WeaponType,
};

#[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_location"))]
use crate::my_strategy::{
    Level,
    color_from_heat,
};

use crate::my_strategy::{
    Debug,
    HitTarget,
    Location,
    Positionable,
    Rect,
    Rectangular,
    Vec2,
    Vec2i,
    World,
    as_score,
    get_hit_probabilities,
    get_hit_probability_by_spread,
    get_hit_probability_by_spread_with_destination,
    is_allowed_to_shoot,
};

#[inline(never)]
pub fn update_locations_score(unit: &Unit, world: &World, optimal_paths: &Vec<(i32, Vec<Location>)>,
        locations_score: &mut Vec<i32>, debug: &mut Debug) {
    for index in 0 .. locations_score.len() {
        if !world.is_reachable(index) {
            locations_score[index] = std::i32::MIN;
            continue;
        }
        let location = world.level().get_tile_location(index);
        if is_busy_by_other(location, unit.id, optimal_paths, world) {
            locations_score[index] = std::i32::MIN;
            continue;
        }
        locations_score[index] = get_location_score(location, unit, world);
    }

    #[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_location"))]
    render_locations_score(locations_score, world.level(), debug);
}

fn is_busy_by_other(location: Location, unit_id: i32, optimal_paths: &Vec<(i32, Vec<Location>)>, world: &World) -> bool {
    for i in 0 .. optimal_paths.len() {
        if optimal_paths[i].0 == unit_id {
            continue;
        }
        if let Some(v) = optimal_paths[i].1.last() {
            if *v == location || *v + Vec2i::only_y(-1) == location {
                return true;
            }
        }
    }
    world.units().iter()
        .filter(|v| v.id != unit_id)
        .find(|v| {
            if !world.is_complex_level() || world.is_opponent_unit(v) {
                v.rect().has_collision(&make_location_rect(location))
            } else if world.is_teammate_unit(v) {
                let radius = world.properties().mine_explosion_params.radius;
                let explosion_rect = Rect::new(location.bottom(), Vec2::new(radius, radius));

                v.rect().has_collision(&explosion_rect)
            } else {
                false
            }
        })
        .is_some()
}

pub fn get_location_score(location: Location, current_unit: &Unit, world: &World) -> i32 {
    as_score(get_location_score_components(location, current_unit, world).iter().sum())
}

pub fn get_location_score_components(location: Location, current_unit: &Unit, world: &World) -> [f64; 16] {
    let current_unit_position = location.bottom();
    let current_unit_center = Vec2::new(location.x() as f64 + 0.5, location.y() as f64 + current_unit.size.y * 0.5);
    let current_unit_rect = Rect::new(current_unit_center, Vec2::from_model(&current_unit.size) / 2.0);
    let location_rect = Rect::new(
        Vec2::new(location.x() as f64 + 0.5, location.y() as f64 + 0.5),
        Vec2::new(0.5, 0.5)
    );
    let distance_to_position_score = 1.0 - current_unit_position.distance(current_unit.position()) / world.max_distance();
    let health_pack_score = match world.tile_item(location) {
        Some(&Item::HealthPack { .. }) => 1.0 - current_unit.health as f64 / world.properties().unit_max_health as f64,
        _ => 0.0,
    };
    let first_weapon_score = if current_unit.weapon.is_none() {
        match world.tile_item(location) {
            Some(&Item::Weapon { .. }) => distance_to_position_score,
            _ => 0.0,
        }
    } else {
        0.0
    };
    let swap_weapon_score = (current_unit.weapon.is_some() && match world.tile_item(location) {
        Some(&Item::Weapon { ref weapon_type }) => {
            get_weapon_score(&current_unit.weapon.as_ref().unwrap().typ) < get_weapon_score(weapon_type)
        },
        _ => false,
    }) as i32 as f64;
    let target = HitTarget::new(current_unit.id, current_unit_rect.clone());
    let hit_by_opponent_score = world.number_of_opponents() as f64 - world.units().iter()
        .filter(|unit| world.is_opponent_unit(unit))
        .map(|unit| {
            if let Some(weapon) = unit.weapon.as_ref() {
                if (weapon.fire_timer.is_none() || weapon.fire_timer.unwrap() < world.config().optimal_location_min_fire_timer)
                        && get_hit_probability_by_spread(unit.center(), &current_unit_rect, get_mean_spread(weapon), weapon.params.bullet.size)
                            >= world.config().min_hit_probability_by_spread_to_shoot {
                    let direction = (current_unit_center - unit.center()).normalized();
                    let hit_probabilities = get_hit_probabilities(unit.id, unit.center(), direction,
                        &target, get_mean_spread(weapon), weapon.params.bullet.size, world,
                        world.config().optimal_location_number_of_directions);
                    (hit_probabilities.target + hit_probabilities.teammate_units) as f64 / hit_probabilities.total as f64
                } else {
                    0.0
                }
            } else {
                0.0
            }
        })
        .sum::<f64>();
    let opponent_obstacle_score = !world.has_opponent_unit(location) as i32 as f64;
    let teammate_obstacle_score = !world.has_teammate_unit(current_unit.id, location) as i32 as f64;
    let mine_obstacle_score = !world.has_mine(location) as i32 as f64;
    let bullet_obstacle_score = !world.has_bullet(current_unit.id, location) as i32 as f64;
    let loot_box_mine_score = match world.tile_item(location) {
        Some(&Item::Mine { }) => 1.0,
        _ => 0.0,
    };
    let nearest_opponent = if let Some(weapon) = current_unit.weapon.as_ref() {
        world.units().iter()
            .filter(|unit| {
                world.is_opponent_unit(unit)
                && may_shoot(current_unit.id, current_unit_center, &unit, weapon, world)
            })
            .min_by_key(|unit| as_score(current_unit_position.distance(unit.position())))
    } else {
        None
    };
    let hit_nearest_opponent_score = if let (Some(weapon), Some(unit)) = (current_unit.weapon.as_ref(), nearest_opponent.as_ref()) {
        if (weapon.fire_timer.is_none() || weapon.fire_timer.unwrap() < world.config().optimal_location_min_fire_timer)
                && (unit.weapon.is_none() || unit.weapon.as_ref().unwrap().fire_timer.is_none() || unit.weapon.as_ref().unwrap().fire_timer.unwrap() >= world.config().optimal_location_min_fire_timer) {
            let direction = (unit.center() - current_unit_center).normalized();
            let hit_probabilities = get_hit_probabilities(current_unit.id, current_unit_center, direction,
                &HitTarget::from_unit(unit), get_mean_spread(weapon), weapon.params.bullet.size, world,
                world.config().optimal_location_number_of_directions);
            (hit_probabilities.target + hit_probabilities.opponent_units) as f64 / hit_probabilities.total as f64
        } else {
            0.0
        }
    } else {
        0.0
    };
    let height_score = location.y() as f64 / world.size().y();
    let over_ground_score = (world.get_tile(location + Vec2i::new(0, -1)) != Tile::Empty) as i32 as f64;
    let number_of_bullets = world.bullets().iter()
        .filter(|v| v.unit_id != current_unit.id)
        .count();
    let bullets_score = number_of_bullets as f64 - if number_of_bullets > 0 {
        world.bullets().iter()
            .filter(|v| v.unit_id != current_unit.id && v.rect().has_collision(&location_rect))
            .count()
    } else {
        0
    } as f64;
    let mines_score = world.mines().len() as f64 - if world.mines().len() > 0 {
        let mine_half = Vec2::new(world.properties().mine_trigger_radius, world.properties().mine_trigger_radius);
        world.mines().iter()
            .filter(|v| Rect::new(v.position(), mine_half).has_collision(&location_rect))
            .count()
    } else {
        0
    } as f64;
    let hit_teammates_score = world.number_of_opponents() as f64 - if let (Some(weapon), Some(opponent)) = (current_unit.weapon.as_ref(), nearest_opponent) {
        if weapon.fire_timer.is_none() || weapon.fire_timer.unwrap() < world.config().optimal_location_min_fire_timer {
            let opponent_rect = opponent.rect();
            world.units().iter()
                .filter(|v| {
                    world.is_opponent_unit(v)
                    && get_hit_probability_by_spread_with_destination(current_unit_center, opponent_rect.center(), &opponent_rect,
                        get_mean_spread(weapon), weapon.params.bullet.size) > 0.0
                })
                .map(|v| {
                    let direction = (opponent.center() - current_unit_center).normalized();
                    get_hit_probabilities(current_unit.id, current_unit_center, direction,
                        &HitTarget::from_unit(v), get_mean_spread(weapon), weapon.params.bullet.size, world,
                        world.config().optimal_location_number_of_directions)
                })
                .map(|v| v.teammate_units as f64 / v.total as f64)
                .sum::<f64>()
        } else {
            0.0
        }
    } else {
        0.0
    };

    [
        distance_to_position_score * world.config().optimal_location_distance_to_position_score_weight,
        health_pack_score * world.config().optimal_location_health_pack_score_weight,
        first_weapon_score * world.config().optimal_location_first_weapon_score_weight,
        swap_weapon_score * world.config().optimal_location_swap_weapon_score_weight,
        hit_by_opponent_score * world.config().optimal_location_hit_by_opponent_score_weight,
        opponent_obstacle_score * world.config().optimal_location_opponent_obstacle_score_weight,
        loot_box_mine_score * world.config().optimal_location_loot_box_mine_score_weight,
        mines_score * world.config().optimal_location_mines_score_weight,
        hit_nearest_opponent_score * world.config().optimal_location_hit_nearest_opponent_score_weight,
        height_score * world.config().optimal_location_height_score_weight,
        over_ground_score * world.config().optimal_location_over_ground_score_weight,
        bullets_score * world.config().optimal_location_bullets_score_weight,
        mine_obstacle_score * world.config().optimal_location_mine_obstacle_score_weight,
        hit_teammates_score * world.config().optimal_location_hit_teammates_score_weight,
        teammate_obstacle_score * world.config().optimal_location_teammate_obstacle_score_weight,
        bullet_obstacle_score * world.config().optimal_location_bullet_obstacle_score_weight,
    ]
}

pub fn get_weapon_score(weapon_type: &WeaponType) -> u32 {
    match weapon_type {
        WeaponType::RocketLauncher => 1,
        WeaponType::AssaultRifle => 2,
        WeaponType::Pistol => 3,
    }
}

fn may_shoot(current_unit_id: i32, current_unit_center: Vec2, opponent: &Unit, weapon: &Weapon, world: &World) -> bool {
    is_allowed_to_shoot(current_unit_id, current_unit_center, get_mean_spread(weapon), &HitTarget::from_unit(&opponent),
        weapon, world, world.config().optimal_location_number_of_directions)
}

pub fn make_location_rect(location: Location) -> Rect {
    Rect::new(location.center(), Vec2::new(0.5, 0.5))
}

fn get_mean_spread(weapon: &Weapon) -> f64 {
    (weapon.params.max_spread + weapon.params.min_spread) / 2.0
}

#[cfg(all(feature = "enable_debug", feature = "enable_debug_optimal_location"))]
fn render_locations_score(locations_score: &Vec<i32>, level: &Level, debug: &mut Debug) {
    let min = locations_score.iter().filter(|v| **v != std::i32::MIN).min().unwrap();
    let max = locations_score.iter().filter(|v| **v != std::i32::MIN).max().unwrap();
    for index in 0 .. locations_score.len() {
        let score = locations_score[index];
        if score == std::i32::MIN {
            continue;
        }
        let location = level.get_tile_location(index);
        debug.draw(CustomData::Rect {
            pos: location.as_debug(),
            size: Vec2F32 { x: 1.0, y: 1.0 },
            color: color_from_heat(0.1, ((score - min) / (max - min)) as f32),
        });
    }
}
