use model::{
    Bullet,
    BulletParams,
    ExplosionParams,
    Unit,
    Weapon,
    WeaponType,
};

use crate::my_strategy::{
    SIMULATOR_ADD_MINES,
    SIMULATOR_ADD_UNITS,
    Debug as Dbg,
    Positionable,
    Rect,
    Rectangular,
    Sector,
    Simulator,
    Vec2,
    World,
    normalize_angle,
};

pub fn get_hit_probability_by_spread(source: Vec2, target: &Rect, spread: f64, bullet_size: f64) -> f64 {
    get_hit_probability_by_spread_with_destination(source, target.center(), target, spread, bullet_size)
}

pub fn get_hit_probability_by_spread_with_destination(source: Vec2, destination: Vec2, target: &Rect, spread: f64, bullet_size: f64) -> f64 {
    Sector::from_direction_and_spread(destination - source, spread + bullet_size / source.distance(target.center()))
        .get_intersection_fraction(Sector::from_source_and_rect(source, target))
}

pub fn is_allowed_to_shoot(unit_id: i32, source: Vec2, spread: f64, opponent: &Unit,
        weapon: &Weapon, world: &World, number_of_directions: usize, debug: &mut Option<&mut Dbg>) -> bool {
    get_shoot_score(unit_id, source, spread, opponent, weapon, world, number_of_directions, debug) > 0
}

pub fn is_allowed_to_shoot_with_direction(unit_id: i32, source: Vec2, direction: Vec2, spread: f64, opponent: &Unit,
        weapon: &Weapon, world: &World, number_of_directions: usize, debug: &mut Option<&mut Dbg>) -> bool {
    get_shoot_score_with_direction(unit_id, source, direction, spread, opponent, weapon, world, number_of_directions, debug) > 0
}

pub fn get_shoot_score(unit_id: i32, source: Vec2, spread: f64, opponent: &Unit,
        weapon: &Weapon, world: &World, number_of_directions: usize, debug: &mut Option<&mut Dbg>) -> i32 {
    let direction = (opponent.center() - source).normalized();
    get_shoot_score_with_direction(unit_id, source, direction, spread, opponent, weapon, world, number_of_directions, debug)
}

pub fn get_shoot_score_with_direction(unit_id: i32, source: Vec2, direction: Vec2, spread: f64, opponent: &Unit,
        weapon: &Weapon, world: &World, number_of_directions: usize, debug: &mut Option<&mut Dbg>) -> i32 {
    let hit_probability_by_spread = get_hit_probability_by_spread(source, &opponent.rect(), spread, weapon.params.bullet.size);

    if hit_probability_by_spread < world.config().min_hit_probability_by_spread_to_shoot {
        return 0;
    }

    let shoot_result = simulate_shoot(unit_id, source, direction, opponent.id, opponent.position(), spread, &weapon.typ,
        &weapon.params.bullet, &weapon.params.explosion, world, number_of_directions, debug);
    let max_damage = weapon.params.bullet.damage + weapon.params.explosion.as_ref().map(|v| v.damage).unwrap_or(0);

    #[cfg(all(feature = "enable_debug", feature = "enable_debug_hit"))]
    {
        if let Some(v) = debug {
            v.log(format!("[{}] shoot_result={:?} max_damage={} opponent={} {:?}",
                unit_id, shoot_result, max_damage, opponent.id, opponent.position()));
        }
    }

    shoot_result.player_score - shoot_result.opponent_score - shoot_result.unit_damage - shoot_result.teammates_damage - max_damage + 1
}

#[derive(Debug, PartialEq, Eq)]
pub struct ShootResult {
    pub player_score: i32,
    pub opponent_score: i32,
    pub teammates_damage: i32,
    pub unit_damage: i32,
}

pub fn simulate_shoot(unit_id: i32, source: Vec2, direction: Vec2, target_unit_id: i32, target_unit_position: Vec2,
        spread: f64, weapon_type: &WeaponType, bullet: &BulletParams, explosion: &Option<ExplosionParams>,
        world: &World, number_of_directions: usize, debug: &mut Option<&mut Dbg>) -> ShootResult {
    let mut initial_simulator = Simulator::new(world, unit_id, SIMULATOR_ADD_UNITS | SIMULATOR_ADD_MINES);
    let player_id = world.get_unit(unit_id).player_id;
    let to_target = direction * world.max_distance();
    let time_interval = world.tick_time_interval();
    let initial_player_score = initial_simulator.player().score;
    let initial_opponent_score = initial_simulator.opponent().score;

    initial_simulator.set_unit_position(unit_id, source - Vec2::only_y(world.properties().unit_size.y / 2.0));
    initial_simulator.set_unit_position(target_unit_id, target_unit_position);

    let mut player_score = 0;
    let mut opponent_score = 0;
    let mut teammates_damage = 0;
    let mut unit_damage = 0;

    for i in 0 .. number_of_directions {
        let angle = ((2 * i) as f64 / (number_of_directions - 1) as f64 - 1.0) * spread;
        let destination = source + to_target.rotated(normalize_angle(angle));
        let current_direction = (destination - source).normalized();
        let mut simulator = initial_simulator.clone();

        simulator.add_bullet(Bullet {
            weapon_type: weapon_type.clone(),
            unit_id: unit_id,
            player_id,
            position: source.as_model(),
            velocity: (current_direction.normalized() * bullet.speed).as_model(),
            damage: bullet.damage,
            size: bullet.size,
            explosion_params: explosion.clone(),
        });

        while !simulator.bullets().is_empty() {
            simulator.tick(time_interval, 1, &mut None, debug);
        }

        player_score += simulator.player().score - initial_player_score;
        opponent_score += simulator.opponent().score - initial_opponent_score;
        teammates_damage += simulator.units().iter()
            .filter(|unit| unit.base().player_id == player_id && unit.base().id != unit_id)
            .map(|unit| world.get_unit(unit.base().id).health - unit.base().health)
            .sum::<i32>();
        unit_damage += initial_simulator.unit().base().health - simulator.unit().base().health;
    }

    ShootResult { player_score, opponent_score, teammates_damage, unit_damage }
}
