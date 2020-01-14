mod helpers;

use model::{
    Item,
    WeaponType,
};
use helpers::{
    updated_world,
    with_bullet,
    with_mine,
    with_my_unit_with_weapon,
    with_opponent_unit_with_weapon_type,
    with_unit_weapon_fire_timer,
    with_unit_health,
    with_unit_with_mines,
    with_unit_with_weapon,
};
use aicup2019::{
    examples::{
        EXAMPLE_MY_PLAYER_ID,
        EXAMPLE_MY_UNIT_ID,
        EXAMPLE_OPPONENT_UNIT_ID,
        example_world,
    },
    my_strategy::{
        Debug,
        Location,
        Positionable,
        Vec2,
        as_score,
        get_optimal_location,
        get_location_score,
    },
};

#[test]
fn test_get_optimal_location() {
    use std::io::BufWriter;

    let stdout = std::io::stdout();
    let handle = stdout.lock();
    let mut stream = BufWriter::new(handle);
    let world = updated_world(example_world());
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let mut debug = aicup2019::Debug(&mut stream);

    assert_eq!(
        get_optimal_location(&unit, &Vec::new(), &world, &mut Debug::new(&mut debug)),
        Some((7.497636005210866, Location::new(29, 1)))
    );
}

#[test]
fn test_get_location_score_random_tile() {
    let world = updated_world(example_world());
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = Location::new(10, 5);
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        0.473114662103054
    );
}

#[test]
fn test_get_location_score_for_tile_with_bullet() {
    let world = updated_world(with_bullet(example_world(), WeaponType::AssaultRifle, Vec2::new(15.832623548153254, 5.93438708445076), Vec2::new(1.0, 0.0), EXAMPLE_OPPONENT_UNIT_ID));
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = world.bullets().iter()
        .find(|v| v.unit_id != EXAMPLE_MY_UNIT_ID)
        .unwrap().location();
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        -0.8896563017807871
    );
}

#[test]
fn test_get_location_score_for_tile_with_opponent() {
    let world = updated_world(example_world());
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = world.get_unit(EXAMPLE_OPPONENT_UNIT_ID).location();
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        -3.273057980995273
    );
}

#[test]
fn test_get_location_score_for_tile_with_weapon() {
    let world = updated_world(example_world());
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = world.loot_boxes().iter()
        .find(|v| if let Item::Weapon { .. } = v.item { true } else { false })
        .unwrap().location();
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        5.4659022055846895
    );
}

#[test]
fn test_get_location_score_my_unit_with_weapon_for_tile_with_weapon() {
    let world = updated_world(with_my_unit_with_weapon(example_world(), WeaponType::AssaultRifle));
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = world.loot_boxes().iter()
        .find(|v| if let Item::Weapon { .. } = v.item { true } else { false })
        .unwrap().location();
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        1.1355219370756926
    );
}

#[test]
fn test_get_location_score_for_tile_with_health_pack() {
    let world = updated_world(example_world());
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = world.loot_boxes().iter()
        .find(|v| if let Item::HealthPack { .. } = v.item { true } else { false })
        .unwrap().location();
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        0.3389864431933246
    );
}

#[test]
fn test_get_location_score_for_tile_with_nearest_health_pack() {
    let world = updated_world(example_world());
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = world.loot_boxes().iter()
        .filter(|v| if let Item::HealthPack { .. } = v.item { true } else { false })
        .min_by_key(|v| as_score(v.position().distance(unit.position())))
        .unwrap().location();
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        0.5127859483135319
    );
}

#[test]
fn test_get_location_score_for_tile_with_health_pack_for_damaged_unit() {
    let world = updated_world(with_unit_health(example_world(), EXAMPLE_MY_UNIT_ID, 50));
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = world.loot_boxes().iter()
        .find(|v| if let Item::HealthPack { .. } = v.item { true } else { false })
        .unwrap().location();
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        4.961528850375023
    );
}

#[test]
fn test_get_location_score_for_tile_with_nearest_health_pack_for_damaged_unit() {
    let world = updated_world(with_unit_health(example_world(), EXAMPLE_MY_UNIT_ID, 50));
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = world.loot_boxes().iter()
        .filter(|v| if let Item::HealthPack { .. } = v.item { true } else { false })
        .min_by_key(|v| as_score(v.position().distance(unit.position())))
        .unwrap().location();
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        8.452697406071444
    );
}

#[test]
fn test_get_location_score_for_tile_with_loot_box_mine() {
    let world = updated_world(example_world());
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = world.loot_boxes().iter()
        .find(|v| if let Item::Mine { } = v.item { true } else { false })
        .unwrap().location();
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        0.5759340781394907
    );
}

#[test]
fn test_get_location_score_for_tile_with_nearest_loot_box_mine() {
    let world = updated_world(example_world());
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = world.loot_boxes().iter()
        .filter(|v| if let Item::Mine { } = v.item { true } else { false })
        .min_by_key(|v| as_score(v.position().distance(unit.position())))
        .unwrap().location();
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        0.6012909624425664
    );
}

#[test]
fn test_get_location_score_for_tile_with_mine() {
    let world = updated_world(with_mine(example_world(), Vec2::new(25.716666665660146, 9.000000000999998), EXAMPLE_MY_PLAYER_ID));
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = world.mines()[0].location();
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        -12.233788073139744
    );
}

#[test]
fn test_get_location_score_for_tile_with_mine_on_the_way() {
    let world = updated_world(with_mine(example_world(), Vec2::new(25.716666665660146, 9.000000000999998), EXAMPLE_MY_PLAYER_ID));
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = Location::new(24, 9);
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        -12.215045519877354
    );
}

#[test]
fn test_get_location_score_my_unit_without_weapon_nearby_opponent_without_weapon() {
    let world = updated_world(example_world());
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = Location::new(5, 1);
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        -0.953861081129515
    );
}

#[test]
fn test_get_location_score_my_unit_with_weapon_nearby_opponent_without_weapon() {
    let world = updated_world(with_my_unit_with_weapon(example_world(), WeaponType::AssaultRifle));
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = Location::new(5, 1);
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        0.692590631914239
    );
}

#[test]
fn test_get_location_score_my_unit_without_weapon_nearby_opponent_with_weapon() {
    let world = updated_world(with_opponent_unit_with_weapon_type(example_world(), WeaponType::AssaultRifle));
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = Location::new(5, 1);
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        -1.777086937651392
    );
}

#[test]
fn test_get_location_score_my_unit_with_weapon_nearby_opponent_with_weapon() {
    let world = updated_world(with_opponent_unit_with_weapon_type(with_my_unit_with_weapon(example_world(), WeaponType::AssaultRifle), WeaponType::AssaultRifle));
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = Location::new(5, 1);
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        -0.13063522460763788
    );
}

#[test]
fn test_get_location_score_my_unit_nearby_opponent_with_mines_without_weapon() {
    let world = updated_world(
        with_unit_with_mines(example_world(), EXAMPLE_OPPONENT_UNIT_ID, 2)
    );
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = Location::new(5, 1);
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        -0.953861081129515
    );
}

#[test]
fn test_get_location_score_my_unit_nearby_opponent_with_mines_not_ready_to_shoot() {
    let world = updated_world(
        with_unit_weapon_fire_timer(
            with_unit_with_weapon(
                with_unit_with_mines(example_world(), EXAMPLE_OPPONENT_UNIT_ID, 2),
                EXAMPLE_OPPONENT_UNIT_ID, WeaponType::AssaultRifle
            ),
            EXAMPLE_OPPONENT_UNIT_ID, Some(1.0)
        )
    );
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = Location::new(5, 1);
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        -0.953861081129515
    );
}

#[test]
fn test_get_location_score_my_unit_nearby_opponent_with_mines_ready_to_shoot() {
    let world = updated_world(
        with_unit_with_weapon(
            with_unit_with_mines(example_world(), EXAMPLE_OPPONENT_UNIT_ID, 2),
            EXAMPLE_OPPONENT_UNIT_ID, WeaponType::AssaultRifle
        )
    );
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = Location::new(5, 1);
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        -6.716442076782653
    );
}

#[test]
fn test_get_location_score_my_unit_with_weapon_nearby_opponent_with_mines() {
    let world = updated_world(
        with_unit_with_weapon(
            with_unit_with_mines(example_world(), EXAMPLE_OPPONENT_UNIT_ID, 2),
            EXAMPLE_OPPONENT_UNIT_ID, WeaponType::AssaultRifle
        )
    );
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = Location::new(5, 1);
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        -6.716442076782653
    );
}

#[test]
fn test_get_location_score_my_unit_outside_mine_explosion_range_for_opponent_with_mines() {
    let world = updated_world(
        with_unit_with_weapon(
            with_unit_with_mines(example_world(), EXAMPLE_OPPONENT_UNIT_ID, 2),
            EXAMPLE_OPPONENT_UNIT_ID, WeaponType::AssaultRifle
        )
    );
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = Location::new(8, 1);
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        0.14911844276866554
    );
}
