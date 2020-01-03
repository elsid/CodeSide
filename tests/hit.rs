mod helpers;

use model::{
    Tile,
    WeaponType,
};

use aicup2019::{
    examples::{
        EXAMPLE_MY_UNIT_ID,
        EXAMPLE_MY_UNIT_ID_1,
        EXAMPLE_OPPONENT_UNIT_ID,
        EXAMPLE_OPPONENT_UNIT_ID_1,
        example_world,
        example_world_with_team_size,
    },
    my_strategy::{
        HitDamage,
        HitTarget,
        Level,
        Rect,
        Rectangular,
        Vec2,
        get_distance_to_nearest_hit_wall_by_horizontal,
        get_distance_to_nearest_hit_wall_by_line,
        get_distance_to_nearest_hit_wall_by_vertical,
        get_hit_damage,
        get_hit_probability_by_spread,
        get_hit_probability_by_spread_with_destination,
    },
};

use helpers::{
    with_my_position,
    with_my_unit_with_weapon,
    with_unit_health,
    with_unit_position,
};

#[test]
fn test_get_distance_to_nearest_hit_wall_by_vertical_with_only_empty_tiles() {
    let level = Level::from_model(&model::Level {
        tiles: vec![
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
        ]
    });
    assert_eq!(
        get_distance_to_nearest_hit_wall_by_vertical(Vec2::new(0.5, 0.5), Vec2::new(0.5, 2.5), &level),
        None
    );
}

#[test]
fn test_get_distance_to_nearest_hit_wall_by_horizontal_with_only_empty_tiles() {
    let level = Level::from_model(&model::Level {
        tiles: vec![
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
        ]
    });
    assert_eq!(
        get_distance_to_nearest_hit_wall_by_horizontal(Vec2::new(0.5, 0.5), Vec2::new(2.5, 0.5), &level),
        None
    );
}

#[test]
fn test_get_distance_to_nearest_hit_wall_by_line_with_only_empty_tiles() {
    let level = Level::from_model(&model::Level {
        tiles: vec![
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
        ]
    });
    assert_eq!(
        get_distance_to_nearest_hit_wall_by_line(Vec2::new(0.5, 0.5), Vec2::new(2.5, 1.5), &level),
        None
    );
}

#[test]
fn test_get_distance_to_nearest_hit_wall_by_line_through_wall() {
    let level = Level::from_model(&model::Level {
        tiles: vec![
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Wall, Tile::Wall, Tile::Wall],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
        ]
    });
    assert_eq!(
        get_distance_to_nearest_hit_wall_by_line(Vec2::new(0.2312, 0.6423), Vec2::new(2.653, 1.234), &level),
        Some(0.79141357808599)
    );
}

#[test]
fn test_get_hit_probability_by_spread() {
    assert_eq!(
        get_hit_probability_by_spread(Vec2::new(0.5, 0.3), &Rect::new(Vec2::new(10.0, 0.5), Vec2::new(0.4, 0.9)), 0.3, 0.4),
        0.2880293914297168
    );
    assert_eq!(
        get_hit_probability_by_spread(Vec2::new(0.5, 0.3), &Rect::new(Vec2::new(10.0, 0.5), Vec2::new(0.4, 0.9)), 0.05, 0.4),
        1.0
    );
    assert_eq!(
        get_hit_probability_by_spread(Vec2::new(19.5, 0.7), &Rect::new(Vec2::new(10.0, 0.5), Vec2::new(0.4, 0.9)), 0.3, 0.4),
        0.2880293914297171
    );
}

#[test]
fn test_get_hit_probability_by_spread_with_destination() {
    assert_eq!(
        get_hit_probability_by_spread_with_destination(Vec2::new(0.5, 0.3), Vec2::new(10.0, 0.5), &Rect::new(Vec2::new(10.0, 0.5), Vec2::new(0.4, 0.9)), 0.3, 0.4),
        0.2880293914297168
    );
    assert_eq!(
        get_hit_probability_by_spread_with_destination(Vec2::new(0.5, 0.3), Vec2::new(10.0, 0.5), &Rect::new(Vec2::new(10.0, 0.5), Vec2::new(0.4, 0.9)), 0.05, 0.4),
        1.0
    );
    assert_eq!(
        get_hit_probability_by_spread_with_destination(Vec2::new(19.5, 0.7), Vec2::new(10.0, 0.5), &Rect::new(Vec2::new(10.0, 0.5), Vec2::new(0.4, 0.9)), 0.3, 0.4),
        0.2880293914297171
    );
    assert_eq!(
        get_hit_probability_by_spread_with_destination(Vec2::new(0.5, 0.3), Vec2::new(10.0, 4.0), &Rect::new(Vec2::new(10.0, 0.5), Vec2::new(0.4, 0.9)), 0.3, 0.4),
        0.13299230152296315
    );
}

#[test]
fn test_get_hit_damage_with_assault_rifle_far_from_healthy_opponent() {
    let world = with_my_unit_with_weapon(example_world(), WeaponType::Pistol);
    let my_unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let opponent_unit = world.get_unit(EXAMPLE_OPPONENT_UNIT_ID);
    let direction = (opponent_unit.center() - my_unit.center()).normalized();
    let target = HitTarget::from_unit(&opponent_unit);
    let weapon = my_unit.weapon.as_ref().unwrap();
    let number_of_directions = 11;

    let hit_damage = get_hit_damage(EXAMPLE_MY_UNIT_ID, my_unit.center(), direction, &target, weapon.params.min_spread,
        &weapon.params.bullet, &weapon.params.explosion, &world, number_of_directions);

    assert_eq!(hit_damage, HitDamage {
        opponent_units_damage_from_opponent: 0,
        opponent_units_damage_from_teammate: 0,
        teammate_units_damage_from_opponent: 0,
        teammate_units_damage_from_teammate: 0,
        target_damage_from_opponent: 0,
        target_damage_from_teammate: 40,
        shooter_damage_from_opponent: 0,
        shooter_damage_from_teammate: 0,
        opponent_units_kills: 0,
        teammate_units_kills: 0,
        target_kills: 0,
        shooter_kills: 0,
    });
}

#[test]
fn test_get_hit_damage_with_rocket_assault_rifle_far_from_healthy_opponent() {
    let world = with_my_unit_with_weapon(example_world(), WeaponType::AssaultRifle);
    let my_unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let opponent_unit = world.get_unit(EXAMPLE_OPPONENT_UNIT_ID);
    let direction = (opponent_unit.center() - my_unit.center()).normalized();
    let target = HitTarget::from_unit(&opponent_unit);
    let weapon = my_unit.weapon.as_ref().unwrap();
    let number_of_directions = 11;

    let hit_damage = get_hit_damage(EXAMPLE_MY_UNIT_ID, my_unit.center(), direction, &target, weapon.params.min_spread,
        &weapon.params.bullet, &weapon.params.explosion, &world, number_of_directions);

    assert_eq!(hit_damage, HitDamage {
        opponent_units_damage_from_opponent: 0,
        opponent_units_damage_from_teammate: 0,
        teammate_units_damage_from_opponent: 0,
        teammate_units_damage_from_teammate: 0,
        target_damage_from_opponent: 0,
        target_damage_from_teammate: 5,
        shooter_damage_from_opponent: 0,
        shooter_damage_from_teammate: 0,
        opponent_units_kills: 0,
        teammate_units_kills: 0,
        target_kills: 0,
        shooter_kills: 0,
    });
}

#[test]
fn test_get_hit_damage_with_rocket_launcher_far_from_healthy_opponent() {
    let world = with_my_unit_with_weapon(example_world(), WeaponType::RocketLauncher);
    let my_unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let opponent_unit = world.get_unit(EXAMPLE_OPPONENT_UNIT_ID);
    let direction = (opponent_unit.center() - my_unit.center()).normalized();
    let target = HitTarget::from_unit(&opponent_unit);
    let weapon = my_unit.weapon.as_ref().unwrap();
    let number_of_directions = 11;

    let hit_damage = get_hit_damage(EXAMPLE_MY_UNIT_ID, my_unit.center(), direction, &target, weapon.params.min_spread,
        &weapon.params.bullet, &weapon.params.explosion, &world, number_of_directions);

    assert_eq!(hit_damage, HitDamage {
        opponent_units_damage_from_opponent: 0,
        opponent_units_damage_from_teammate: 0,
        teammate_units_damage_from_opponent: 0,
        teammate_units_damage_from_teammate: 0,
        target_damage_from_opponent: 0,
        target_damage_from_teammate: 30,
        shooter_damage_from_opponent: 0,
        shooter_damage_from_teammate: 0,
        opponent_units_kills: 0,
        teammate_units_kills: 0,
        target_kills: 0,
        shooter_kills: 0,
    });
}

#[test]
fn test_get_hit_damage_with_rocket_launcher_nearby_healthy_opponent() {
    let world = with_my_position(
        with_my_unit_with_weapon(example_world(), WeaponType::RocketLauncher),
        Vec2::new(6.5, 1.0)
    );
    let my_unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let opponent_unit = world.get_unit(EXAMPLE_OPPONENT_UNIT_ID);
    let direction = (opponent_unit.center() - my_unit.center()).normalized();
    let target = HitTarget::from_unit(&opponent_unit);
    let weapon = my_unit.weapon.as_ref().unwrap();
    let number_of_directions = 11;

    let hit_damage = get_hit_damage(EXAMPLE_MY_UNIT_ID, my_unit.center(), direction, &target, weapon.params.min_spread,
        &weapon.params.bullet, &weapon.params.explosion, &world, number_of_directions);

    assert_eq!(hit_damage, HitDamage {
        opponent_units_damage_from_opponent: 0,
        opponent_units_damage_from_teammate: 0,
        teammate_units_damage_from_opponent: 0,
        teammate_units_damage_from_teammate: 0,
        target_damage_from_opponent: 0,
        target_damage_from_teammate: 880,
        shooter_damage_from_opponent: 0,
        shooter_damage_from_teammate: 0,
        opponent_units_kills: 0,
        teammate_units_kills: 0,
        target_kills: 0,
        shooter_kills: 0,
    });
}

#[test]
fn test_get_hit_damage_with_rocket_launcher_not_far_from_damaged_opponent() {
    let world = with_unit_health(
        with_my_position(
            with_my_unit_with_weapon(example_world(), WeaponType::RocketLauncher),
            Vec2::new(6.5, 1.0)
        ),
        EXAMPLE_OPPONENT_UNIT_ID, 50
    );
    let my_unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let opponent_unit = world.get_unit(EXAMPLE_OPPONENT_UNIT_ID);
    let direction = (opponent_unit.center() - my_unit.center()).normalized();
    let target = HitTarget::from_unit(&opponent_unit);
    let weapon = my_unit.weapon.as_ref().unwrap();
    let number_of_directions = 11;

    let hit_damage = get_hit_damage(EXAMPLE_MY_UNIT_ID, my_unit.center(), direction, &target, weapon.params.min_spread,
        &weapon.params.bullet, &weapon.params.explosion, &world, number_of_directions);

    assert_eq!(hit_damage, HitDamage {
        opponent_units_damage_from_opponent: 0,
        opponent_units_damage_from_teammate: 0,
        teammate_units_damage_from_opponent: 0,
        teammate_units_damage_from_teammate: 0,
        target_damage_from_opponent: 0,
        target_damage_from_teammate: 550,
        shooter_damage_from_opponent: 0,
        shooter_damage_from_teammate: 0,
        opponent_units_kills: 0,
        teammate_units_kills: 0,
        target_kills: 11,
        shooter_kills: 0,
    });
}

#[test]
fn test_get_hit_damage_my_damaged_units_with_rocket_launcher_nearby_damaged_opponent() {
    let world = with_unit_health(
        with_unit_health(
            with_my_position(
                with_my_unit_with_weapon(example_world(), WeaponType::RocketLauncher),
                Vec2::new(4.5, 1.0)
            ),
            EXAMPLE_OPPONENT_UNIT_ID, 50
        ),
        EXAMPLE_MY_UNIT_ID, 50
    );
    let my_unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let opponent_unit = world.get_unit(EXAMPLE_OPPONENT_UNIT_ID);
    let direction = (opponent_unit.center() - my_unit.center()).normalized();
    let target = HitTarget::from_unit(&opponent_unit);
    let weapon = my_unit.weapon.as_ref().unwrap();
    let number_of_directions = 11;

    let hit_damage = get_hit_damage(EXAMPLE_MY_UNIT_ID, my_unit.center(), direction, &target, weapon.params.min_spread,
        &weapon.params.bullet, &weapon.params.explosion, &world, number_of_directions);

    assert_eq!(hit_damage, HitDamage {
        opponent_units_damage_from_opponent: 0,
        opponent_units_damage_from_teammate: 0,
        teammate_units_damage_from_opponent: 0,
        teammate_units_damage_from_teammate: 0,
        target_damage_from_opponent: 0,
        target_damage_from_teammate: 550,
        shooter_damage_from_opponent: 0,
        shooter_damage_from_teammate: 550,
        opponent_units_kills: 0,
        teammate_units_kills: 0,
        target_kills: 11,
        shooter_kills: 11,
    });
}

#[test]
fn test_get_hit_damage_2x2_with_rocket_launcher() {
    let world = with_unit_health(
        with_unit_health(
            with_unit_health(
                with_unit_health(
                    with_unit_position(
                        with_unit_position(
                            with_my_unit_with_weapon(example_world_with_team_size(2), WeaponType::RocketLauncher),
                            EXAMPLE_MY_UNIT_ID,
                            Vec2::new(4.5, 1.0)
                        ),
                        EXAMPLE_MY_UNIT_ID_1,
                        Vec2::new(3.5, 1.9)
                    ),
                    EXAMPLE_OPPONENT_UNIT_ID, 50
                ),
                EXAMPLE_MY_UNIT_ID, 50
            ),
            EXAMPLE_MY_UNIT_ID_1, 80
        ),
        EXAMPLE_OPPONENT_UNIT_ID_1, 80
    );
    let my_unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let opponent_unit = world.get_unit(EXAMPLE_OPPONENT_UNIT_ID);
    let direction = (opponent_unit.center() - my_unit.center()).normalized();
    let target = HitTarget::from_unit(&opponent_unit);
    let weapon = my_unit.weapon.as_ref().unwrap();
    let number_of_directions = 11;

    let hit_damage = get_hit_damage(EXAMPLE_MY_UNIT_ID, my_unit.center(), direction, &target, weapon.params.min_spread,
        &weapon.params.bullet, &weapon.params.explosion, &world, number_of_directions);

    assert_eq!(hit_damage, HitDamage {
        opponent_units_damage_from_opponent: 0,
        opponent_units_damage_from_teammate: 700,
        teammate_units_damage_from_opponent: 0,
        teammate_units_damage_from_teammate: 730,
        target_damage_from_opponent: 0,
        target_damage_from_teammate: 550,
        shooter_damage_from_opponent: 0,
        shooter_damage_from_teammate: 550,
        opponent_units_kills: 5,
        teammate_units_kills: 6,
        target_kills: 11,
        shooter_kills: 11,
    });
}
