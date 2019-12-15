mod helpers;

use model::{
    Item,
    WeaponType,
};
use helpers::{
    me_with_weapon,
    opponent_with_weapon,
    updated_world,
};
use aicup2019::{
    Debug,
    examples::example_world,
    my_strategy::{
        Location,
        Positionable,
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

    assert_eq!(get_optimal_tile(&world, &Vec::new(), &mut Debug(&mut stream)), Some((3.0633333333333335, Location::new(29, 1))));
}

#[test]
fn test_get_tile_score_random_tile() {
    let world = updated_world(example_world());
    {
        let location = Location::new(10, 5);
        assert_eq!(
            get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
            -0.03833333333333333
        );
    }
}

#[test]
fn test_get_tile_score_optimal_tile() {
    let world = updated_world(example_world());
    let location = Location::new(10, 1);
    assert_eq!(
        get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
        3.0015145946466255
    );
}

#[test]
fn test_get_tile_score_for_tile_with_bullet() {
    let world = updated_world(example_world());
    let location = world.bullets().iter()
        .find(|v| v.unit_id != world.me().id)
        .unwrap().location();
    assert_eq!(
        get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
        -0.5133333333333333
    );
}

#[test]
fn test_get_tile_score_for_tile_with_opponent() {
    let world = updated_world(example_world());
    let location = world.units().iter()
        .find(|v| v.player_id != world.me().player_id)
        .unwrap().location();
    assert_eq!(
        get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
        -1.0742377344785319
    );
}

#[test]
fn test_get_tile_score_for_tile_with_weapon() {
    let world = updated_world(example_world());
    let location = world.loot_boxes().iter()
        .find(|v| if let Item::Weapon { .. } = v.item { true } else { false })
        .unwrap().location();
    assert_eq!(
        get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
        2.933333333333333
    );
}

#[test]
fn test_get_tile_score_me_with_weapon_for_tile_with_weapon() {
    let world = updated_world(me_with_weapon(example_world(), WeaponType::AssaultRifle));
    let location = world.loot_boxes().iter()
        .find(|v| if let Item::Weapon { .. } = v.item { true } else { false })
        .unwrap().location();
    assert_eq!(
        get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
        0.9333333333333333
    );
}

#[test]
fn test_get_tile_score_for_tile_with_health_pack() {
    let world = updated_world(example_world());
    let location = world.loot_boxes().iter()
        .find(|v| if let Item::HealthPack { .. } = v.item { true } else { false })
        .unwrap().location();
    assert_eq!(
        get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
        1.4166666666666667
    );
}

#[test]
fn test_get_tile_score_for_tile_with_loot_box_mine() {
    let world = updated_world(example_world());
    let location = world.loot_boxes().iter()
        .find(|v| if let Item::Mine { } = v.item { true } else { false })
        .unwrap().location();
    assert_eq!(
        get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
        0.06833333333333336
    );
}

#[test]
fn test_get_tile_score_for_tile_with_mine() {
    let world = updated_world(example_world());
    let location = world.mines()[0].location();
    assert_eq!(
        get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
        -1.47
    );
}

#[test]
fn test_get_tile_score_for_tile_with_mine_on_the_way() {
    let world = updated_world(example_world());
    let location = Location::new(24, 9);
    assert_eq!(
        get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
        -0.975
    );
}

#[test]
fn test_get_tile_score_me_without_weapon_nearby_opponent_without_weapon() {
    let world = updated_world(example_world());
    {
        let location = Location::new(5, 1);
        assert_eq!(
            get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
            -0.04807727471516629
        );
    }
}

#[test]
fn test_get_tile_score_me_with_weapon_nearby_opponent_without_weapon() {
    let world = updated_world(me_with_weapon(example_world(), WeaponType::AssaultRifle));
    {
        let location = Location::new(5, 1);
        assert_eq!(
            get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
            1.951922725284834
        );
    }
}

#[test]
fn test_get_tile_score_me_without_weapon_nearby_opponent_with_weapon() {
    let world = updated_world(opponent_with_weapon(example_world(), WeaponType::AssaultRifle));
    {
        let location = Location::new(5, 1);
        assert_eq!(
            get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
            0.08833333333333335
        );
    }
}

#[test]
fn test_get_tile_score_me_with_weapon_nearby_opponent_with_weapon() {
    let world = updated_world(opponent_with_weapon(me_with_weapon(example_world(), WeaponType::AssaultRifle), WeaponType::AssaultRifle));
    {
        let location = Location::new(5, 1);
        assert_eq!(
            get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
            0.08833333333333335
        );
    }
}
