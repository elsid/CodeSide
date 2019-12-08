use model::{
    Bullet,
    Item,
    JumpState,
    LootBox,
    Properties,
    Unit,
    Weapon,
    WeaponType,
};
use my_strategy::examples::{
    example_properties,
    example_rng,
    example_world,
};
use my_strategy::my_strategy::{
    Rectangular,
    Simulator,
    UnitExt,
    Vec2,
    WeaponWrapper,
    World,
};

#[test]
fn test_simulator_single_tick() {
    let world = example_world();
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut rng,
    );
    assert_eq!(
        simulator.me().position(),
        Vec2::new(37.5, 1.000000001)
    );
}

#[test]
fn test_simulator_unit_move_horizontal_for_one_tick() {
    let world = example_world();
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.me_mut().action_mut().velocity = world.properties().unit_max_horizontal_speed;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut rng,
    );
    assert_eq!(
        simulator.me().position(),
        Vec2::new(37.666666666666515, 1.000000001)
    );
}

#[test]
fn test_simulator_unit_jump_from_wall_for_one_tick() {
    let world = example_world();
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.me_mut().action_mut().jump = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut rng,
    );
    assert_eq!(
        simulator.me().position(),
        Vec2::new(37.5, 1.165000001000004)
    );
}

#[test]
fn test_simulator_unit_jump_from_platform_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(7.5, 8.0));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.me_mut().action_mut().jump = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut rng,
    );
    assert_eq!(
        simulator.me().position(),
        Vec2::new(7.5, 8.165000001000026)
    );
}

#[test]
fn test_simulator_unit_jump_from_ladder_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(5.5, 5.0));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.me_mut().action_mut().jump = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut rng,
    );
    assert_eq!(
        simulator.me().position(),
        Vec2::new(5.5, 5.165000001000026)
    );
}

#[test]
fn test_simulator_unit_jump_from_jump_pad_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(13.5, 1.0));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut rng,
    );
    assert_eq!(
        simulator.me().position(),
        Vec2::new(13.5, 1.333333333333341)
    );
}

#[test]
fn test_simulator_unit_jump_on_ladder_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(5.5, 3.0));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.me_mut().action_mut().jump = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut rng,
    );
    assert_eq!(
        simulator.me().position(),
        Vec2::new(5.5, 3.1666666666666483)
    );
}

#[test]
fn test_simulator_unit_jump_in_air_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(15.5, 8.0));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.me_mut().action_mut().jump = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut rng,
    );
    assert_eq!(
        simulator.me().position(),
        Vec2::new(15.5, 7.833333333333307)
    );
}

#[test]
fn test_simulator_unit_jump_down_by_ladder_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(22.5, 2.0));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.me_mut().action_mut().jump_down = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut rng,
    );
    assert_eq!(
        simulator.me().position(),
        Vec2::new(22.5, 1.8333333333333295)
    );
}

#[test]
fn test_simulator_unit_fall_down_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(37.5, 2.0));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.me_mut().action_mut().jump_down = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut rng,
    );
    assert_eq!(
        simulator.me().position(),
        Vec2::new(37.5, 1.8333333333333295)
    );
}

#[test]
fn test_simulator_unit_stand_on_platform_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(15.5, 5.0));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut rng,
    );
    assert_eq!(
        simulator.me().position(),
        Vec2::new(15.5, 5.000000001)
    );
}

#[test]
fn test_simulator_unit_jump_down_through_platform_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(15.5, 5.0));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.me_mut().action_mut().jump_down = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut rng,
    );
    assert_eq!(
        simulator.me().position(),
        Vec2::new(15.5, 4.833333333333307)
    );
}

#[test]
fn test_simulator_unit_jump_from_wall_until_land() {
    let world = example_world();
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.me_mut().action_mut().jump = true;
    for _ in 0..33 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut rng,
        );
    }
    assert_eq!(
        simulator.me().position(),
        Vec2::new(37.5, 6.498333334333527)
    );
    simulator.me_mut().action_mut().jump = false;
    for _ in 33..66 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut rng,
        );
    }
    assert_eq!(
        simulator.me().position(),
        Vec2::new(37.5, 1.000000001)
    );
}

#[test]
fn test_simulator_unit_cancel_jump() {
    let world = example_world();
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.me_mut().action_mut().jump = true;
    for _ in 0..2 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut rng,
        );
    }
    assert_eq!(
        simulator.me().position(),
        Vec2::new(37.5, 1.3316666676666744)
    );
    simulator.me_mut().action_mut().jump = false;
    for _ in 2..4 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut rng,
        );
    }
    assert_eq!(
        simulator.me().position(),
        Vec2::new(37.5, 1.000000001)
    );
}

#[test]
fn test_simulator_unit_run_into_wall() {
    let world = with_my_position(example_world(), Vec2::new(18.549999999, 1.000000001));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.me_mut().action_mut().velocity = world.properties().unit_max_horizontal_speed;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut rng,
    );
    assert_eq!(
        simulator.me().position(),
        Vec2::new(18.549999999, 1.000000001)
    );
}

#[test]
fn test_simulator_unit_run_into_unit() {
    let world = with_my_position(example_world(), Vec2::new(3.5, 1.0));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.me_mut().action_mut().velocity = -world.properties().unit_max_horizontal_speed;
    for _ in 0 .. 10 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut rng,
        );
    }
    assert_eq!(
        simulator.me().position(),
        Vec2::new(3.400000001, 1.000000001)
    );
}

#[test]
fn test_simulator_unit_fall_onto_unit() {
    let world = with_my_position(example_world(), Vec2::new(2.5, 3.5));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    for _ in 0 .. 10 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut rng,
        );
    }
    assert_eq!(
        simulator.me().position(),
        Vec2::new(2.5, 2.800000002)
    );
}

#[test]
fn test_simulator_bullet_hit_unit() {
    let world = with_bullet(example_world(), WeaponType::AssaultRifle, Vec2::new(30.0, 2.0), Vec2::new(1.0, 0.0), 0);
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    for _ in 0 .. 10 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut rng,
        );
    }
    assert_eq!(simulator.me().health(), 95);
    assert_eq!(simulator.bullets().len(), 1);
}

#[test]
fn test_simulator_bullet_does_not_hit_its_shooter() {
    let world = {
        let world = example_world();
        let unit_id = world.me().id;
        with_bullet(world, WeaponType::AssaultRifle, Vec2::new(30.0, 2.0), Vec2::new(1.0, 0.0), unit_id)
    };
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    for _ in 0 .. 10 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut rng,
        );
    }
    assert_eq!(simulator.me().health(), 100);
    assert_eq!(simulator.bullets().len(), 2);
}

#[test]
fn test_simulator_bullet_explode_unit() {
    let world = with_bullet(example_world(), WeaponType::RocketLauncher, Vec2::new(30.0, 2.0), Vec2::new(1.0, 0.0), 0);
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    for _ in 0 .. 25 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut rng,
        );
    }
    assert_eq!(simulator.me().health(), 20);
    assert_eq!(simulator.bullets().len(), 1);
}

#[test]
fn test_simulator_bullet_hit_wall() {
    let world = example_world();
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    assert_eq!(simulator.bullets().len(), 1);
    for _ in 0 .. 30 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut rng,
        );
    }
    assert_eq!(simulator.bullets().len(), 0);
}

#[test]
fn test_simulator_bullet_explode_on_hit_wall() {
    let world = with_bullet(example_world(), WeaponType::RocketLauncher, Vec2::new(36.0, 5.0), Vec2::new(0.0, -1.0), 0);
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    for _ in 0 .. 15 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut rng,
        );
    }
    assert_eq!(simulator.me().health(), 50);
    assert_eq!(simulator.bullets().len(), 1);
}

#[test]
fn test_simulator_unit_pickup_weapon() {
    let world = with_loot_box(example_world(), Item::Weapon {weapon_type: WeaponType::RocketLauncher}, Vec2::new(36.0, 2.0));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.me_mut().action_mut().velocity = -world.properties().unit_max_horizontal_speed;
    assert!(simulator.me().weapon().is_none());
    for _ in 0 .. 5 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut rng,
        );
    }
    assert_eq!(
        simulator.me().weapon().as_ref().map(|v| WeaponWrapper(v)),
        Some(WeaponWrapper(&Weapon {
            typ: WeaponType::RocketLauncher,
            params: world.properties().weapon_params[&WeaponType::RocketLauncher].clone(),
            magazine: 1,
            was_shooting: false,
            spread: 0.0,
            fire_timer: None,
            last_angle: None,
            last_fire_tick: None,
        }))
    );
}

#[test]
fn test_simulator_unit_pickup_health_pack() {
    let world = with_loot_box(example_world(), Item::HealthPack {health: 40}, Vec2::new(36.0, 2.0));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.me_mut().action_mut().velocity = -world.properties().unit_max_horizontal_speed;
    simulator.me_mut().damage(20);
    assert_eq!(simulator.me().health(), 80);
    for _ in 0 .. 5 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut rng,
        );
    }
    assert_eq!(simulator.me().health(), 100);
}

#[test]
fn test_simulator_unit_pickup_mine() {
    let world = with_loot_box(example_world(), Item::Mine {}, Vec2::new(36.0, 2.0));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.me_mut().action_mut().velocity = -world.properties().unit_max_horizontal_speed;
    simulator.me_mut().damage(20);
    assert_eq!(simulator.me().mines(), 0);
    for _ in 0 .. 5 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut rng,
        );
    }
    assert_eq!(simulator.me().mines(), 1);
}

#[test]
fn test_simulator_single_tick_unit_on_edge() {
    let world = with_my_position(example_world(), Vec2::new(38.55, 24.55555555555557));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut rng,
    );
    assert_eq!(
        simulator.me().position(),
        Vec2::new(38.55, 24.388888888889056)
    );
}

#[test]
fn test_simulator_single_tick_unit_left_border_on_right_edge() {
    let world = with_my_position(example_world(), Vec2::new(38.49444444444444, 27.16500000099991));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.me_mut().action_mut().velocity = world.properties().unit_max_horizontal_speed;
    simulator.me_mut().action_mut().jump = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut rng,
    );
    assert_eq!(
        simulator.me().position(),
        Vec2::new(38.549999999, 26.998333334333395)
    );
}

#[test]
fn test_simulator_unit_stay_on_ladder_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(5.25, 3.0));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut rng,
    );
    assert_eq!(
        simulator.me().position(),
        Vec2::new(5.25, 3.0)
    );
}

#[test]
fn test_simulator_unit_fall_through_ladder_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(4.9, 3.0));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut rng,
    );
    assert_eq!(
        simulator.me().position(),
        Vec2::new(4.9, 2.8333333333333517)
    );
}

#[test]
fn test_simulator_unit_fall_through_platform_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(7.5, 7.5));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut rng,
    );
    assert_eq!(
        simulator.me().position(),
        Vec2::new(7.5, 7.333333333333307)
    );
}

#[test]
fn test_collide_with_tile_by_x_without_penetration_by_x() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(9.5, 10.0), &properties);
    a.start_move_by_x(-1.0);
    a.collide_with_tile_by_x(10, 10);
    a.finish_move_by_x();
    assert_eq!(a.position(), Vec2::new(8.5, 10.0));
}

#[test]
fn test_collide_with_tile_by_y_without_penetration_by_y() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(10.0, 9.0), &properties);
    a.start_move_by_y(-1.0);
    a.collide_with_tile_by_y(10, 11);
    a.finish_move_by_y();
    assert_eq!(a.position(), Vec2::new(10.0, 8.0));
}

#[test]
fn test_collide_with_tile_by_x_with_penetration() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(9.5, 10.0), &properties);
    a.start_move_by_x(1.0);
    a.collide_with_tile_by_x(10, 10);
    a.finish_move_by_x();
    assert_eq!(a.position(), Vec2::new(9.549999999, 10.0));
}

#[test]
fn test_collide_with_tile_by_y_with_penetration() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(10.0, 11.5), &properties);
    a.start_move_by_y(-1.0);
    a.collide_with_tile_by_y(10, 10);
    a.finish_move_by_y();
    assert_eq!(a.position(), Vec2::new(10.0, 11.000000001));
}

#[test]
fn test_collide_with_tile_by_x_without_penetration_by_y() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(9.5, 9.0), &properties);
    a.start_move_by_x(1.0);
    a.collide_with_tile_by_x(10, 11);
    a.finish_move_by_x();
    assert_eq!(a.position(), Vec2::new(10.5, 9.0));
}

#[test]
fn test_collide_with_tile_by_y_without_penetration_by_x() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(9.5, 9.0), &properties);
    a.start_move_by_y(1.0);
    a.collide_with_tile_by_y(10, 11);
    a.finish_move_by_y();
    assert_eq!(a.position(), Vec2::new(9.5, 10.0));
}

#[test]
fn test_collide_with_unit_by_x_without_penetration_by_x() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(9.5, 10.0), &properties);
    let b = make_unit_ext(Vec2::new(10.5, 10.0), &properties);
    a.start_move_by_x(-0.5);
    a.collide_with_unit_by_x(&b);
    a.finish_move_by_x();
    assert_eq!(a.position(), Vec2::new(9.0, 10.0));
}

#[test]
fn test_collide_with_unit_by_y_without_penetration_by_y() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(10.0, 9.0), &properties);
    let b = make_unit_ext(Vec2::new(10.0, 11.0), &properties);
    a.start_move_by_y(-0.5);
    a.collide_with_unit_by_y(&b);
    a.finish_move_by_y();
    assert_eq!(a.position(), Vec2::new(10.0, 8.5));
}

#[test]
fn test_collide_with_unit_by_x_with_penetration() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(9.5, 10.0), &properties);
    let b = make_unit_ext(Vec2::new(10.5, 10.0), &properties);
    a.start_move_by_x(0.5);
    a.collide_with_unit_by_x(&b);
    a.finish_move_by_x();
    assert_eq!(a.position(), Vec2::new(9.599999999, 10.0));
}

#[test]
fn test_collide_with_unit_by_y_with_penetration() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(10.0, 9.0), &properties);
    let b = make_unit_ext(Vec2::new(10.0, 11.0), &properties);
    a.start_move_by_y(0.5);
    a.collide_with_unit_by_y(&b);
    a.finish_move_by_y();
    assert_eq!(a.position(), Vec2::new(10.0, 9.199999999000001));
}

#[test]
fn test_collide_with_unit_by_x_without_penetration_by_y() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(9.5, 9.0), &properties);
    let b = make_unit_ext(Vec2::new(10.5, 11.0), &properties);
    a.start_move_by_x(0.5);
    a.collide_with_unit_by_x(&b);
    a.finish_move_by_x();
    assert_eq!(a.position(), Vec2::new(10.0, 9.0));
}

#[test]
fn test_collide_with_unit_by_y_without_penetration_by_x() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(9.5, 9.0), &properties);
    let b = make_unit_ext(Vec2::new(10.5, 11.0), &properties);
    a.start_move_by_y(0.5);
    a.collide_with_unit_by_y(&b);
    a.finish_move_by_y();
    assert_eq!(a.position(), Vec2::new(9.5, 9.5));
}

fn with_my_position(world: World, position: Vec2) -> World {
    let mut game = world.game().clone();
    let me_index = game.units.iter().position(|v| v.id == world.me().id).unwrap();
    game.units[me_index].position = position.as_model();
    World::new(world.config().clone(), game.units[me_index].clone(), game)
}

fn with_bullet(world: World, weapon_type: WeaponType, position: Vec2, direction: Vec2, unit_id: i32) -> World {
    let mut game = world.game().clone();
    let params = &world.properties().weapon_params.get(&weapon_type).unwrap();
    game.bullets.push(Bullet {
        weapon_type: weapon_type,
        unit_id: unit_id,
        player_id: 0,
        position: position.as_model(),
        velocity: (direction.normalized() * params.bullet.speed).as_model(),
        damage: params.bullet.damage,
        size: params.bullet.size,
        explosion_params: params.explosion.clone(),
    });
    World::new(world.config().clone(), world.me().clone(), game)
}

fn with_loot_box(world: World, item: Item, position: Vec2) -> World {
    let mut game = world.game().clone();
    game.loot_boxes.push(LootBox {
        position: position.as_model(),
        size: world.properties().loot_box_size.clone(),
        item: item,
    });
    World::new(world.config().clone(), world.me().clone(), game)
}

fn make_unit_ext(position: Vec2, properties: &Properties) -> UnitExt {
    let base = Unit {
        player_id: 1,
        id: 1,
        health: 100,
        position: position.as_model(),
        size: properties.unit_size.clone(),
        jump_state: JumpState {
            can_jump: false,
            speed: 0.0,
            max_time: 0.0,
            can_cancel: false,
        },
        walked_right: false,
        stand: true,
        on_ground: false,
        on_ladder: false,
        mines: 0,
        weapon: None,
    };
    UnitExt::new(base, false, false)
}
