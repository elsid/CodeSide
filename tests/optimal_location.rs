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
        get_location_score,
        get_optimal_location,
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
        Some((1223333, Location::new(29, 1)))
    );
}

#[test]
fn test_get_location_score_optimal_location() {
    let world = updated_world(example_world());
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let location = Location::new(29, 1);
    let unit_index = world.get_unit_index(unit.id);
    let path_info = world.get_path_info(unit_index, location);
    assert!(path_info.is_some());
    assert_eq!(
        get_location_score(location, &unit, &world, &path_info.unwrap()),
        1223333
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
        963019
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
        865486
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
        757833
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
        1059999
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
        1065231
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
        967342
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
        977833
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
        670789
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
        670368
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
        959333
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
        1026000
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
        926000
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
        992667
    );
}
