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
        get_optimal_location,
        get_location_score,
        get_location_score_components,
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
        Some((3.028865463678731, Location::new(29, 1)))
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
        0.3964842027797048
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
        -1.0841315288514304
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
        -0.6378392442935059
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
        1.1541766408990042
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
        1.3872443569922313
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
        0.37731656828292665
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
        0.5143823327086419
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
        -5.54753707347546
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
        -5.551413927149232
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
        0.37379131672781285
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
        1.0404579833944796
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
        0.040457983394479535
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
        0.7071246500611461
    );
}
