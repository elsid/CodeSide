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
        Positionable,
        Rect,
        Rectangular,
        ShootResult,
        Vec2,
        get_hit_probability_by_spread,
        simulate_shoot,
    },
};

use helpers::{
    with_my_position,
    with_my_unit_with_weapon,
    with_unit_health,
    with_unit_position,
};

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
fn test_simulate_shoot_with_assault_rifle_far_from_healthy_opponent() {
    let world = with_my_unit_with_weapon(example_world(), WeaponType::Pistol);
    let my_unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let opponent_unit = world.get_unit(EXAMPLE_OPPONENT_UNIT_ID);
    let direction = (opponent_unit.center() - my_unit.center()).normalized();
    let weapon = my_unit.weapon.as_ref().unwrap();
    let number_of_directions = 11;

    let shoot_result = simulate_shoot(EXAMPLE_MY_UNIT_ID, my_unit.center(), direction,
        opponent_unit.id, opponent_unit.position(), weapon.params.min_spread, &weapon.typ,
        &weapon.params.bullet, &weapon.params.explosion, &world, number_of_directions, &mut None);

    assert_eq!(shoot_result, ShootResult { player_score: 20, opponent_score: 0, teammates_damage: 0, unit_damage: 0 });
}

#[test]
fn test_simulate_shoot_with_rocket_assault_rifle_far_from_healthy_opponent() {
    let world = with_my_unit_with_weapon(example_world(), WeaponType::AssaultRifle);
    let my_unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let opponent_unit = world.get_unit(EXAMPLE_OPPONENT_UNIT_ID);
    let direction = (opponent_unit.center() - my_unit.center()).normalized();
    let weapon = my_unit.weapon.as_ref().unwrap();
    let number_of_directions = 11;

    let shoot_result = simulate_shoot(EXAMPLE_MY_UNIT_ID, my_unit.center(), direction,
        opponent_unit.id, opponent_unit.position(), weapon.params.min_spread, &weapon.typ,
        &weapon.params.bullet, &weapon.params.explosion, &world, number_of_directions, &mut None);

    assert_eq!(shoot_result, ShootResult { player_score: 5, opponent_score: 0, teammates_damage: 0, unit_damage: 0 });
}

#[test]
fn test_simulate_shoot_with_rocket_launcher_far_from_healthy_opponent() {
    let world = with_my_unit_with_weapon(example_world(), WeaponType::RocketLauncher);
    let my_unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let opponent_unit = world.get_unit(EXAMPLE_OPPONENT_UNIT_ID);
    let direction = (opponent_unit.center() - my_unit.center()).normalized();
    let weapon = my_unit.weapon.as_ref().unwrap();
    let number_of_directions = 11;

    let shoot_result = simulate_shoot(EXAMPLE_MY_UNIT_ID, my_unit.center(), direction,
        opponent_unit.id, opponent_unit.position(), weapon.params.min_spread, &weapon.typ,
        &weapon.params.bullet, &weapon.params.explosion, &world, number_of_directions, &mut None);

    assert_eq!(shoot_result, ShootResult { player_score: 180, opponent_score: 0, teammates_damage: 0, unit_damage: 0 });
}

#[test]
fn test_simulate_shoot_with_rocket_launcher_not_far_from_healthy_opponent() {
    let world = with_my_position(
        with_my_unit_with_weapon(example_world(), WeaponType::RocketLauncher),
        Vec2::new(7.5, 1.0)
    );
    let my_unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let opponent_unit = world.get_unit(EXAMPLE_OPPONENT_UNIT_ID);
    let direction = (opponent_unit.center() - my_unit.center()).normalized();
    let weapon = my_unit.weapon.as_ref().unwrap();
    let number_of_directions = 11;

    let shoot_result = simulate_shoot(EXAMPLE_MY_UNIT_ID, my_unit.center(), direction,
        opponent_unit.id, opponent_unit.position(), weapon.params.min_spread, &weapon.typ,
        &weapon.params.bullet, &weapon.params.explosion, &world, number_of_directions, &mut None);

    assert_eq!(shoot_result, ShootResult { player_score: 880, opponent_score: 0, teammates_damage: 0, unit_damage: 0 });
}

#[test]
fn test_simulate_shoot_with_rocket_launcher_not_far_from_damaged_opponent() {
    let world = with_unit_health(
        with_my_position(
            with_my_unit_with_weapon(example_world(), WeaponType::RocketLauncher),
            Vec2::new(7.5, 1.0)
        ),
        EXAMPLE_OPPONENT_UNIT_ID, 50
    );
    let my_unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let opponent_unit = world.get_unit(EXAMPLE_OPPONENT_UNIT_ID);
    let direction = (opponent_unit.center() - my_unit.center()).normalized();
    let weapon = my_unit.weapon.as_ref().unwrap();
    let number_of_directions = 11;

    let shoot_result = simulate_shoot(EXAMPLE_MY_UNIT_ID, my_unit.center(), direction,
        opponent_unit.id, opponent_unit.position(), weapon.params.min_spread, &weapon.typ,
        &weapon.params.bullet, &weapon.params.explosion, &world, number_of_directions, &mut None);

    assert_eq!(shoot_result, ShootResult { player_score: 11550, opponent_score: 0, teammates_damage: 0, unit_damage: 0 });
}

#[test]
fn test_simulate_shoot_my_damaged_units_with_rocket_launcher_nearby_damaged_opponent() {
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
    let weapon = my_unit.weapon.as_ref().unwrap();
    let number_of_directions = 11;

    let shoot_result = simulate_shoot(EXAMPLE_MY_UNIT_ID, my_unit.center(), direction,
        opponent_unit.id, opponent_unit.position(), weapon.params.min_spread, &weapon.typ,
        &weapon.params.bullet, &weapon.params.explosion, &world, number_of_directions, &mut None);

    assert_eq!(shoot_result, ShootResult { player_score: 11550, opponent_score: 11000, teammates_damage: 0, unit_damage: 550 });
}

#[test]
fn test_simulate_shoot_2x2_with_rocket_launcher() {
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
    let weapon = my_unit.weapon.as_ref().unwrap();
    let number_of_directions = 11;

    let shoot_result = simulate_shoot(EXAMPLE_MY_UNIT_ID, my_unit.center(), direction,
        opponent_unit.id, opponent_unit.position(), weapon.params.min_spread, &weapon.typ,
        &weapon.params.bullet, &weapon.params.explosion, &world, number_of_directions, &mut None);

    assert_eq!(shoot_result, ShootResult { player_score: 12100, opponent_score: 22000, teammates_damage: 880, unit_damage: 550 });
}
