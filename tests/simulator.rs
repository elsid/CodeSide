use model::{
    JumpState,
    Properties,
    Unit,
    UnitAction,
    Vec2F64,
};
use my_strategy::examples::{
    example_properties,
    example_rng,
    example_world,
};
use my_strategy::my_strategy::{
    Simulator,
    UnitExt,
    Vec2,
    World,
    collide_units_by_x,
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
        Vec2::new(37.5, 1.0)
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
        Vec2::new(37.666666666666515, 1.0)
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
        Vec2::new(37.5, 1.1650000000000036)
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
        Vec2::new(7.5, 8.165000000000026)
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
        Vec2::new(5.5, 5.165000000000026)
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
        Vec2::new(5.5, 3.1633333333333153)
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
        Vec2::new(15.5, 5.0)
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
        Vec2::new(37.5, 6.498333333333526)
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
        Vec2::new(37.5, 0.9999999999999998)
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
        Vec2::new(37.5, 1.331666666666674)
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
        Vec2::new(37.5, 1.0)
    );
}

#[test]
fn test_simulator_unit_run_into_wall() {
    let world = with_my_position(example_world(), Vec2::new(18.549999999, 1.000000001));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.me_mut().action_mut().velocity = 10.0;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut rng,
    );
    assert_eq!(
        simulator.me().position(),
        Vec2::new(18.55, 1.0)
    );
}

#[test]
fn test_simulator_unit_run_into_unit() {
    let world = with_my_position(example_world(), Vec2::new(4.5, 1.0));
    let mut simulator = Simulator::new(&world, world.me().id);
    let mut rng = example_rng(7348172934612063328);
    simulator.me_mut().action_mut().velocity = -10.0;
    for _ in 0 .. 10 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut rng,
        );
    }
    assert_eq!(
        simulator.me().position(),
        Vec2::new(3.4, 1.0)
    );
}

#[test]
fn test_simulator_unit_fall_onto_unit() {
    let world = with_my_position(example_world(), Vec2::new(2.5, 4.0));
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
        Vec2::new(2.5, 2.8)
    );
}

#[test]
fn test_collide_units_by_x_without_penetration() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(9.5, 10.0), &properties);
    let mut b = make_unit_ext(Vec2::new(10.5, 10.0), &properties);
    collide_units_by_x(&mut a, &mut b);
    assert_eq!(a.position(), Vec2::new(9.5, 10.0));
    assert_eq!(b.position(), Vec2::new(10.5, 10.0));
}

#[test]
fn test_collide_units_by_x_not_moved() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(9.8, 10.0), &properties);
    let mut b = make_unit_ext(Vec2::new(10.2, 10.0), &properties);
    collide_units_by_x(&mut a, &mut b);
    assert_eq!(a.position(), Vec2::new(9.55, 10.0));
    assert_eq!(b.position(), Vec2::new(10.45, 10.0));
}

#[test]
fn test_collide_units_by_x_moved_on_each_other_equally() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(9.5, 10.0), &properties);
    let mut b = make_unit_ext(Vec2::new(10.5, 10.0), &properties);
    a.move_by_x(0.3);
    b.move_by_x(-0.3);
    collide_units_by_x(&mut a, &mut b);
    assert_eq!(a.position(), Vec2::new(9.55, 10.0));
    assert_eq!(b.position(), Vec2::new(10.45, 10.0));
}

#[test]
fn test_collide_units_by_x_moved_on_each_other_unequally() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(9.5, 10.0), &properties);
    let mut b = make_unit_ext(Vec2::new(10.5, 10.0), &properties);
    a.move_by_x(0.6);
    b.move_by_x(-0.2);
    collide_units_by_x(&mut a, &mut b);
    assert_eq!(a.position(), Vec2::new(9.575000000000001, 10.0));
    assert_eq!(b.position(), Vec2::new(10.475, 10.0));
}

#[test]
fn test_collide_units_by_x_moved_one_onto_other_positive() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(9.5, 10.0), &properties);
    let mut b = make_unit_ext(Vec2::new(10.5, 10.0), &properties);
    a.move_by_x(0.5);
    collide_units_by_x(&mut a, &mut b);
    assert_eq!(a.position(), Vec2::new(9.6, 10.0));
    assert_eq!(b.position(), Vec2::new(10.5, 10.0));
}

#[test]
fn test_collide_units_by_x_moved_one_onto_other_negative() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(9.5, 10.0), &properties);
    let mut b = make_unit_ext(Vec2::new(10.5, 10.0), &properties);
    b.move_by_x(-0.5);
    collide_units_by_x(&mut a, &mut b);
    assert_eq!(a.position(), Vec2::new(9.5, 10.0));
    assert_eq!(b.position(), Vec2::new(10.4, 10.0));
}

#[test]
fn test_collide_units_by_x_moved_in_same_positive_direction_unequally() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(9.5, 10.0), &properties);
    let mut b = make_unit_ext(Vec2::new(10.5, 10.0), &properties);
    a.move_by_x(0.5);
    b.move_by_x(0.25);
    collide_units_by_x(&mut a, &mut b);
    assert_eq!(a.position(), Vec2::new(9.9, 10.0));
    assert_eq!(b.position(), Vec2::new(10.7, 10.0));
}

#[test]
fn test_collide_units_by_x_moved_in_same_negative_direction_unequally() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(9.5, 10.0), &properties);
    let mut b = make_unit_ext(Vec2::new(10.5, 10.0), &properties);
    a.move_by_x(-0.25);
    b.move_by_x(-0.5);
    collide_units_by_x(&mut a, &mut b);
    assert_eq!(a.position(), Vec2::new(9.3, 10.0));
    assert_eq!(b.position(), Vec2::new(10.1, 10.0));
}

#[test]
fn test_collide_units_by_x_in_reverse_order_moved_in_same_positive_direction_unequally() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(9.5, 10.0), &properties);
    let mut b = make_unit_ext(Vec2::new(10.5, 10.0), &properties);
    a.move_by_x(0.5);
    b.move_by_x(0.25);
    collide_units_by_x(&mut b, &mut a);
    assert_eq!(a.position(), Vec2::new(9.9, 10.0));
    assert_eq!(b.position(), Vec2::new(10.7, 10.0));
}

#[test]
fn test_collide_units_by_x_in_reverse_order_moved_in_same_negative_direction_unequally() {
    let properties = example_properties();
    let mut a = make_unit_ext(Vec2::new(9.5, 10.0), &properties);
    let mut b = make_unit_ext(Vec2::new(10.5, 10.0), &properties);
    a.move_by_x(-0.25);
    b.move_by_x(-0.5);
    collide_units_by_x(&mut b, &mut a);
    assert_eq!(a.position(), Vec2::new(9.3, 10.0));
    assert_eq!(b.position(), Vec2::new(10.1, 10.0));
}

fn with_my_position(world: World, position: Vec2) -> World {
    let mut game = world.game().clone();
    let me_index = game.units.iter().position(|v| v.id == world.me().id).unwrap();
    game.units[me_index].position = position.as_model();
    World::new(world.config().clone(), game.units[me_index].clone(), game)
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
