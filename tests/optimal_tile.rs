use model::Item;
use aicup2019::{
    Debug,
    examples::example_world,
    my_strategy::{
        Location,
        Positionable,
        World,
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

    assert_eq!(get_optimal_tile(&world, &mut Debug(&mut stream)), Some(Location::new(29, 1)));
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
fn test_get_tile_score_for_tile_with_health_pack() {
    let world = updated_world(example_world());
    let location = world.loot_boxes().iter()
        .find(|v| if let Item::HealthPack { .. } = v.item { true } else { false })
        .unwrap().location();
    assert_eq!(
        get_tile_score(&world, location, world.path_info(world.me().location(), location).unwrap()),
        0.9166666666666666
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
        -0.47
    );
}

fn updated_world(mut world: World) -> World {
    let game = world.game().clone();
    let me = world.me().clone();
    world.update(&me, &game);
    world
}
