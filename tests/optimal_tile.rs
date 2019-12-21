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
    with_opponent_unit_with_weapon,
};
use aicup2019::{
    Debug,
    examples::{
        EXAMPLE_MY_PLAYER_ID,
        EXAMPLE_MY_UNIT_ID,
        EXAMPLE_OPPONENT_UNIT_ID,
        example_world,
    },
    my_strategy::{
        Location,
        Positionable,
        Vec2,
        get_optimal_tile,
        get_tile_score,
    },
};

#[test]
fn test_get_optimal_tile() {
    use std::io::BufWriter;

    let stdout = std::io::stdout();
    let handle = stdout.lock();
    let mut stream = BufWriter::new(handle);
    let world = updated_world(example_world());
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);

    assert_eq!(
        get_optimal_tile(&unit, &world, &Vec::new(), &mut Debug(&mut stream)),
        Some((2.5833333333333335, Location::new(29, 1)))
    );
}

#[test]
fn test_get_tile_score_random_tile() {
    let world = updated_world(example_world());
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let unit_index = world.get_unit_index(unit.id);
    let location = Location::new(10, 5);
    let path_info = world.path_info(unit_index, unit.location(), location);
    assert!(path_info.is_some());
    assert_eq!(
        get_tile_score(location, &unit, &world, path_info.unwrap()),
        -0.03833333333333333
    );
}

#[test]
fn test_get_tile_score_for_tile_with_bullet() {
    let world = updated_world(with_bullet(example_world(), WeaponType::AssaultRifle, Vec2::new(15.832623548153254, 5.93438708445076), Vec2::new(1.0, 0.0), EXAMPLE_OPPONENT_UNIT_ID));
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let unit_index = world.get_unit_index(unit.id);
    let location = world.bullets().iter()
        .find(|v| v.unit_id != EXAMPLE_MY_UNIT_ID)
        .unwrap().location();
    let path_info = world.path_info(unit_index, unit.location(), location);
    assert!(path_info.is_some());
    assert_eq!(
        get_tile_score(location, &unit, &world, path_info.unwrap()),
        -0.5133333333333333
    );
}

#[test]
fn test_get_tile_score_for_tile_with_opponent() {
    let world = updated_world(example_world());
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let unit_index = world.get_unit_index(unit.id);
    let location = world.get_unit(EXAMPLE_OPPONENT_UNIT_ID).location();
    let path_info = world.path_info(unit_index, unit.location(), location);
    assert!(path_info.is_some());
    assert_eq!(
        get_tile_score(location, &unit, &world, path_info.unwrap()),
        -1.0742377344785319
    );
}

#[test]
fn test_get_tile_score_for_tile_with_weapon() {
    let world = updated_world(example_world());
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let unit_index = world.get_unit_index(unit.id);
    let location = world.loot_boxes().iter()
        .find(|v| if let Item::Weapon { .. } = v.item { true } else { false })
        .unwrap().location();
    let path_info = world.path_info(unit_index, unit.location(), location);
    assert!(path_info.is_some());
    assert_eq!(
        get_tile_score(location, &unit, &world, path_info.unwrap()),
        0.053333333333333455
    );
}

#[test]
fn test_get_tile_score_my_unit_with_weapon_for_tile_with_weapon() {
    let world = updated_world(with_my_unit_with_weapon(example_world(), WeaponType::AssaultRifle));
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let unit_index = world.get_unit_index(unit.id);
    let location = world.loot_boxes().iter()
        .find(|v| if let Item::Weapon { .. } = v.item { true } else { false })
        .unwrap().location();
    let path_info = world.path_info(unit_index, unit.location(), location);
    assert!(path_info.is_some());
    assert_eq!(
        get_tile_score(location, &unit, &world, path_info.unwrap()),
        0.9333333333333333
    );
}

#[test]
fn test_get_tile_score_for_tile_with_health_pack() {
    let world = updated_world(example_world());
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let unit_index = world.get_unit_index(unit.id);
    let location = world.loot_boxes().iter()
        .find(|v| if let Item::HealthPack { .. } = v.item { true } else { false })
        .unwrap().location();
    let path_info = world.path_info(unit_index, unit.location(), location);
    assert!(path_info.is_some());
    assert_eq!(
        get_tile_score(location, &unit, &world, path_info.unwrap()),
        1.4166666666666667
    );
}

#[test]
fn test_get_tile_score_for_tile_with_loot_box_mine() {
    let world = updated_world(example_world());
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let unit_index = world.get_unit_index(unit.id);
    let location = world.loot_boxes().iter()
        .find(|v| if let Item::Mine { } = v.item { true } else { false })
        .unwrap().location();
    let path_info = world.path_info(unit_index, unit.location(), location);
    assert!(path_info.is_some());
    assert_eq!(
        get_tile_score(location, &unit, &world, path_info.unwrap()),
        0.06833333333333336
    );
}

#[test]
fn test_get_tile_score_for_tile_with_mine() {
    let world = updated_world(with_mine(example_world(), Vec2::new(25.716666665660146, 9.000000000999998), EXAMPLE_MY_PLAYER_ID));
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let unit_index = world.get_unit_index(unit.id);
    let location = world.mines()[0].location();
    let path_info = world.path_info(unit_index, unit.location(), location);
    assert!(path_info.is_some());
    assert_eq!(
        get_tile_score(location, &unit, &world, path_info.unwrap()),
        -5.970000000000001
    );
}

#[test]
fn test_get_tile_score_for_tile_with_mine_on_the_way() {
    let world = updated_world(with_mine(example_world(), Vec2::new(25.716666665660146, 9.000000000999998), EXAMPLE_MY_PLAYER_ID));
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let unit_index = world.get_unit_index(unit.id);
    let location = Location::new(24, 9);
    let path_info = world.path_info(unit_index, unit.location(), location);
    assert!(path_info.is_some());
    assert_eq!(
        get_tile_score(location, &unit, &world, path_info.unwrap()),
        -5.975
    );
}

#[test]
fn test_get_tile_score_my_unit_without_weapon_nearby_opponent_without_weapon() {
    let world = updated_world(example_world());
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let unit_index = world.get_unit_index(unit.id);
    let location = Location::new(5, 1);
    let path_info = world.path_info(unit_index, unit.location(), location);
    assert!(path_info.is_some());
    assert_eq!(
        get_tile_score(location, &unit, &world, path_info.unwrap()),
        -0.04807727471516629
    );
}

#[test]
fn test_get_tile_score_my_unit_with_weapon_nearby_opponent_without_weapon() {
    let world = updated_world(with_my_unit_with_weapon(example_world(), WeaponType::AssaultRifle));
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let unit_index = world.get_unit_index(unit.id);
    let location = Location::new(5, 1);
    let path_info = world.path_info(unit_index, unit.location(), location);
    assert!(path_info.is_some());
    assert_eq!(
        get_tile_score(location, &unit, &world, path_info.unwrap()),
        1.951922725284834
    );
}

#[test]
fn test_get_tile_score_my_unit_without_weapon_nearby_opponent_with_weapon() {
    let world = updated_world(with_opponent_unit_with_weapon(example_world(), WeaponType::AssaultRifle));
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let unit_index = world.get_unit_index(unit.id);
    let location = Location::new(5, 1);
    let path_info = world.path_info(unit_index, unit.location(), location);
    assert!(path_info.is_some());
    assert_eq!(
        get_tile_score(location, &unit, &world, path_info.unwrap()),
        -1.048077274715166
    );
}

#[test]
fn test_get_tile_score_my_unit_with_weapon_nearby_opponent_with_weapon() {
    let world = updated_world(with_opponent_unit_with_weapon(with_my_unit_with_weapon(example_world(), WeaponType::AssaultRifle), WeaponType::AssaultRifle));
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let unit_index = world.get_unit_index(unit.id);
    let location = Location::new(5, 1);
    let path_info = world.path_info(unit_index, unit.location(), location);
    assert!(path_info.is_some());
    assert_eq!(
        get_tile_score(location, &unit, &world, path_info.unwrap()),
        0.9519227252848337
    );
}
