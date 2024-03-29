mod helpers;

use model::{
    Item,
    MineState,
    WeaponType,
};

#[cfg(feature = "simulator_pickup_weapon")]
use model::{
    Weapon,
};

use helpers::{
    make_unit_ext,
    with_bullet,
    with_loot_box,
    with_mine,
    with_my_position,
    with_unit_position,
};

#[cfg(feature = "simulator_pickup_weapon")]
use helpers::{
    WeaponWrapper,
    with_my_unit_with_weapon,
};

use aicup2019::{
    examples::{
        EXAMPLE_MY_PLAYER_ID,
        EXAMPLE_MY_UNIT_ID,
        EXAMPLE_OPPONENT_PLAYER_ID,
        EXAMPLE_OPPONENT_UNIT_ID,
        example_properties,
        example_rng,
        example_world,
    },
    my_strategy::{
        SIMULATOR_DEFAULT_FLAGS,
        BulletExt,
        Simulator,
        Vec2,
    },
};

#[test]
fn test_simulator_single_tick() {
    let world = example_world();
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(37.5, 1.0)
    );
}

#[test]
fn test_simulator_unit_move_horizontal_for_one_tick() {
    let world = example_world();
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.unit_mut().action_mut().velocity = world.properties().unit_max_horizontal_speed;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(37.666666666666515, 1.0)
    );
}

#[test]
fn test_simulator_unit_jump_from_wall_for_one_tick() {
    let world = example_world();
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.unit_mut().action_mut().jump = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(37.5, 1.1650000000000038)
    );
}

#[test]
fn test_simulator_unit_jump_from_platform_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(7.5, 8.0));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.unit_mut().action_mut().jump = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(7.5, 8.165000000000026)
    );
}

#[test]
fn test_simulator_unit_jump_from_ladder_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(5.5, 5.0));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.unit_mut().action_mut().jump = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(5.5, 5.165000000000026)
    );
}

#[test]
fn test_simulator_unit_jump_from_jump_pad_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(13.5, 1.0));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(13.5, 1.333333333333341)
    );
}

#[test]
fn test_simulator_unit_jump_on_ladder_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(5.5, 3.0));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.unit_mut().action_mut().jump = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(5.5, 3.1666666666666483)
    );
}

#[test]
fn test_simulator_unit_jump_in_air_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(15.5, 8.0));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.unit_mut().action_mut().jump = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(15.5, 7.833333333333307)
    );
}

#[test]
fn test_simulator_unit_jump_down_by_ladder_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(22.5, 2.0));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.unit_mut().action_mut().jump_down = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(22.5, 1.8333333333333295)
    );
}

#[test]
fn test_simulator_unit_fall_down_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(37.5, 2.0));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.unit_mut().action_mut().jump_down = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(37.5, 1.8333333333333295)
    );
}

#[test]
fn test_simulator_unit_stand_on_platform_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(15.5, 5.0));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(15.5, 5.0)
    );
}

#[test]
fn test_simulator_unit_jump_down_through_platform_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(15.5, 5.0));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.unit_mut().action_mut().jump_down = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(15.5, 4.833333333333307)
    );
}

#[test]
fn test_simulator_unit_jump_from_wall_until_land() {
    let world = example_world();
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.unit_mut().action_mut().jump = true;
    for _ in 0..33 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(37.5, 6.498333333333527)
    );
    simulator.unit_mut().action_mut().jump = false;
    for _ in 33..66 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(37.5, 1.0000000000000004)
    );
}

#[test]
fn test_simulator_unit_cancel_jump() {
    let world = example_world();
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.unit_mut().action_mut().jump = true;
    for _ in 0..2 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(37.5, 1.3316666666666743)
    );
    simulator.unit_mut().action_mut().jump = false;
    for _ in 2..4 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(37.5, 1.0)
    );
}

#[test]
fn test_simulator_unit_run_into_wall() {
    let world = with_my_position(example_world(), Vec2::new(18.549999999, 1.000000001));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.unit_mut().action_mut().velocity = world.properties().unit_max_horizontal_speed;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(18.549999999, 1.000000001)
    );
}

#[test]
fn test_simulator_unit_run_into_unit() {
    let world = with_my_position(example_world(), Vec2::new(3.5, 1.0));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.unit_mut().action_mut().velocity = -world.properties().unit_max_horizontal_speed;
    for _ in 0 .. 10 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(3.400000000000011, 1.0)
    );
}

#[test]
fn test_simulator_unit_fall_onto_unit() {
    let world = with_my_position(example_world(), Vec2::new(2.5, 3.5));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    for _ in 0 .. 10 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(2.5, 2.800000000000077)
    );
}

#[test]
fn test_simulator_bullet_hit_unit() {
    let world = with_bullet(example_world(), WeaponType::AssaultRifle, Vec2::new(30.0, 2.0), Vec2::new(1.0, 0.0), EXAMPLE_OPPONENT_UNIT_ID);
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    assert_eq!(simulator.opponent().score, 0);
    assert_eq!(simulator.player().score, 0);
    for _ in 0 .. 10 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator.unit().health(), 95);
    assert_eq!(simulator.bullets().len(), 0, "{:?}", simulator.bullets());
    assert_eq!(simulator.opponent().score, 5);
    assert_eq!(simulator.player().score, 0);
}

#[test]
fn test_simulator_bullet_does_not_hit_its_shooter() {
    let world = with_bullet(example_world(), WeaponType::AssaultRifle, Vec2::new(30.0, 2.0), Vec2::new(1.0, 0.0), EXAMPLE_MY_UNIT_ID);
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    for _ in 0 .. 10 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator.unit().health(), 100);
    assert_eq!(simulator.bullets().len(), 1, "{:?}", simulator.bullets());
}

#[test]
fn test_simulator_bullet_explode_unit() {
    let world = with_bullet(example_world(), WeaponType::RocketLauncher, Vec2::new(30.0, 2.0), Vec2::new(1.0, 0.0), EXAMPLE_OPPONENT_UNIT_ID);
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    assert_eq!(simulator.opponent().score, 0);
    assert_eq!(simulator.player().score, 0);
    for _ in 0 .. 25 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator.unit().health(), 20);
    assert_eq!(simulator.bullets().len(), 0, "{:?}", simulator.bullets());
    assert_eq!(simulator.opponent().score, 80);
    assert_eq!(simulator.player().score, 0);
}

#[test]
fn test_simulator_bullet_hit_wall() {
    let world = with_bullet(example_world(), WeaponType::AssaultRifle, Vec2::new(15.832623548153254, 5.93438708445076), Vec2::new(1.0, 0.0), EXAMPLE_OPPONENT_UNIT_ID);
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    for _ in 0 .. 30 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator.bullets().len(), 0, "{:?}", simulator.bullets());
}

#[test]
fn test_simulator_bullet_explode_on_hit_wall() {
    let world = with_bullet(example_world(), WeaponType::RocketLauncher, Vec2::new(36.0, 5.0), Vec2::new(0.0, -1.0), EXAMPLE_MY_UNIT_ID);
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    assert_eq!(simulator.opponent().score, 0);
    assert_eq!(simulator.player().score, 0);
    for _ in 0 .. 15 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator.unit().health(), 50);
    assert_eq!(simulator.bullets().len(), 0, "{:?}", simulator.bullets());
    assert_eq!(simulator.opponent().score, 0);
    assert_eq!(simulator.player().score, 0);
}

#[cfg(feature = "simulator_pickup_weapon")]
#[test]
fn test_simulator_unit_pickup_weapon() {
    let world = with_loot_box(example_world(), Item::Weapon {weapon_type: WeaponType::RocketLauncher}, Vec2::new(36.5, 1.0));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.unit_mut().action_mut().velocity = -world.properties().unit_max_horizontal_speed;
    assert!(simulator.unit().weapon().is_none());
    let before = simulator.loot_boxes().len();
    for _ in 0 .. 5 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator.loot_boxes().len(), before - 1);
    assert_eq!(
        simulator.unit().weapon().as_ref().map(|v| WeaponWrapper(v)),
        Some(WeaponWrapper(&Weapon {
            typ: WeaponType::RocketLauncher,
            params: world.properties().weapon_params[&WeaponType::RocketLauncher].clone(),
            magazine: 1,
            was_shooting: false,
            spread: 0.5,
            fire_timer: None,
            last_angle: None,
            last_fire_tick: None,
        }))
    );
}

#[test]
fn test_simulator_unit_pickup_health_pack() {
    let world = with_loot_box(example_world(), Item::HealthPack {health: 40}, Vec2::new(36.5, 1.0));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.unit_mut().action_mut().velocity = -world.properties().unit_max_horizontal_speed;
    simulator.unit_mut().damage(20);
    let before = simulator.loot_boxes().len();
    assert_eq!(simulator.unit().health(), 80);
    for _ in 0 .. 5 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator.loot_boxes().len(), before - 1);
    assert_eq!(simulator.unit().health(), 100);
}

#[cfg(feature = "simulator_pickup_mine")]
#[test]
fn test_simulator_unit_pickup_mine() {
    let world = with_loot_box(example_world(), Item::Mine {}, Vec2::new(36.5, 1.0));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.unit_mut().action_mut().velocity = -world.properties().unit_max_horizontal_speed;
    simulator.unit_mut().damage(20);
    let before = simulator.loot_boxes().len();
    assert_eq!(simulator.unit().mines(), 0);
    for _ in 0 .. 5 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator.loot_boxes().len(), before - 1);
    assert_eq!(simulator.unit().mines(), 1);
}

#[test]
fn test_simulator_single_tick_unit_on_edge() {
    let world = with_my_position(example_world(), Vec2::new(38.55, 24.55555555555557));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(38.55, 24.388888888889056)
    );
}

#[test]
fn test_simulator_single_tick_unit_left_border_on_right_edge() {
    let world = with_my_position(example_world(), Vec2::new(38.49444444444444, 27.16500000099991));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.unit_mut().action_mut().velocity = world.properties().unit_max_horizontal_speed;
    simulator.unit_mut().action_mut().jump = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(38.54944444444439, 26.998333334333395)
    );
}

#[test]
fn test_simulator_unit_stay_on_ladder_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(5.25, 3.0));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(5.25, 3.0)
    );
}

#[test]
fn test_simulator_unit_fall_through_ladder_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(4.9, 3.0));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(4.9, 2.8333333333333517)
    );
}

#[test]
fn test_simulator_unit_fall_through_platform_for_one_tick() {
    let world = with_my_position(example_world(), Vec2::new(7.5, 7.5));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(7.5, 7.333333333333307)
    );
}

#[test]
fn test_simulator_mine_change_state() {
    let world = with_mine(example_world(), Vec2::new(25.716666665660146, 9.000000000999998), EXAMPLE_MY_PLAYER_ID);
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    for _ in 0 .. 70 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator.mines().len(), 1, "{:?}", simulator.mines());
    assert_eq!(simulator.mines()[0].base().state, MineState::Idle,
        "{:?}", simulator.mines()[0].base());
}

#[test]
fn test_simulator_mine_explosion() {
    let world = with_mine(example_world(), Vec2::new(37.5, 1.0), EXAMPLE_OPPONENT_PLAYER_ID);
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    assert_eq!(simulator.unit().health(), 100);
    assert_eq!(simulator.opponent().score, 0);
    assert_eq!(simulator.player().score, 0);
    for _ in 0 .. 100 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator.mines().len(), 0, "{:?}", simulator.mines());
    assert_eq!(simulator.unit().health(), 50);
    assert_eq!(simulator.opponent().score, 50);
    assert_eq!(simulator.player().score, 0);
}

#[test]
fn test_simulator_unit_land_on_platform() {
    let world = with_my_position(example_world(), Vec2::new(15.5, 5.3));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    for _ in 0 .. 3 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(15.5, 5.00166666666662)
    );
}

#[test]
fn test_simulator_unit_land_on_platform_and_jump() {
    let world = with_my_position(example_world(), Vec2::new(15.5, 5.3));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.unit_mut().action_mut().jump = true;
    for _ in 0 .. 2 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(15.5, 5.034999999999958)
    );
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(15.5, 5.201666666666651)
    );
}

#[test]
fn test_simulator_unit_land_on_platform_and_jump_down_for_one_tick_and_walk() {
    let world = with_my_position(example_world(), Vec2::new(15.5, 5.3));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    for _ in 0 .. 3 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(15.5, 5.00166666666662)
    );
    simulator.unit_mut().action_mut().jump_down = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(15.5, 4.834999999999927)
    );
    simulator.unit_mut().action_mut().velocity = world.properties().unit_max_horizontal_speed;
    simulator.unit_mut().action_mut().jump_down = false;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(15.666666666666693, 4.668333333333234)
    );
}

#[test]
fn test_simulator_bullet_explode_and_kill_unit() {
    let world = with_bullet(
        with_bullet(
            example_world(),
            WeaponType::RocketLauncher, Vec2::new(30.0, 2.0), Vec2::new(1.0, 0.0), EXAMPLE_OPPONENT_UNIT_ID
        ),
        WeaponType::RocketLauncher, Vec2::new(30.0, 2.0), Vec2::new(1.0, 0.0), EXAMPLE_OPPONENT_UNIT_ID
    );
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    assert_eq!(simulator.opponent().score, 0);
    assert_eq!(simulator.player().score, 0);
    for _ in 0 .. 25 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator.unit().health(), 0);
    assert_eq!(simulator.bullets().len(), 0, "{:?}", simulator.bullets());
    assert_eq!(simulator.opponent().score, 1100);
    assert_eq!(simulator.player().score, 0);
}

#[test]
fn test_simulator_my_bullet_explode_and_kill_unit() {
    let world = with_bullet(
        with_bullet(
            example_world(),
            WeaponType::RocketLauncher, Vec2::new(36.0, 5.0), Vec2::new(0.0, -1.0), EXAMPLE_MY_UNIT_ID
        ),
        WeaponType::RocketLauncher, Vec2::new(36.0, 5.0), Vec2::new(0.0, -1.0), EXAMPLE_MY_UNIT_ID
    );
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    assert_eq!(simulator.opponent().score, 0);
    assert_eq!(simulator.player().score, 0);
    for _ in 0 .. 15 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator.unit().health(), 0);
    assert_eq!(simulator.bullets().len(), 0, "{:?}", simulator.bullets());
    assert_eq!(simulator.opponent().score, 1000);
    assert_eq!(simulator.player().score, 0);
}

#[test]
fn test_simulator_unit_set_jump_false_cancel_jump() {
    let world = example_world();
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert!(simulator.unit().base().jump_state.can_jump);
    simulator.unit_mut().action_mut().jump = true;
    for _ in 0 .. 2 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert!(simulator.unit().base().jump_state.can_jump);
    simulator.unit_mut().action_mut().jump = false;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert!(!simulator.unit().base().jump_state.can_jump);
}

#[test]
fn test_simulator_unit_jump_from_ladder_and_land_on_ladder() {
    let world = with_my_position(example_world(), Vec2::new(9.5, 8.0));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.unit_mut().action_mut().jump = true;
    for _ in 0 .. 5 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(9.5, 8.831666666666797)
    );
    simulator.unit_mut().action_mut().jump = false;
    for _ in 0 .. 6 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(
        simulator.unit().position(),
        Vec2::new(9.5, 8.0)
    );
}

#[cfg(all(feature = "simulator_weapon", feature = "simulator_shoot"))]
#[test]
fn test_simulator_shoot() {
    let world = with_my_unit_with_weapon(example_world(), WeaponType::Pistol);
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    simulator.unit_mut().action_mut().shoot = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(simulator.bullets().len(), 1);
    assert_eq!(Vec2::from_model(&simulator.bullets()[0].base().position), Vec2::new(36.67500000000005, 1.900000001));
    assert_eq!(Vec2::from_model(&simulator.bullets()[0].base().velocity), Vec2::new(-50.0, 0.0));
}

#[test]
fn test_simulator_bullet_hit_mine() {
    let world = with_mine(
        with_bullet(example_world(), WeaponType::AssaultRifle, Vec2::new(20.0, 9.25), Vec2::new(1.0, 0.0), EXAMPLE_MY_UNIT_ID),
        Vec2::new(25.716666665660146, 9.000000000999998), EXAMPLE_MY_PLAYER_ID
    );
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    assert_eq!(simulator.mines().len(), 1);
    assert_eq!(simulator.mines()[0].base().state, MineState::Preparing,
        "{:?}", simulator.mines()[0].base());
    assert_eq!(simulator.bullets().len(), 1);
    for _ in 0 .. 10 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator.mines().len(), 0);
    assert_eq!(simulator.bullets().len(), 0);
}

#[test]
fn test_simulator_bullet_explosion_chain() {
    let world = with_mine(
        with_mine(
            with_bullet(example_world(), WeaponType::RocketLauncher, Vec2::new(23.0, 12.0), Vec2::new(0.0, -1.0), EXAMPLE_MY_UNIT_ID),
            Vec2::new(25.716666665660146, 9.000000000999998), EXAMPLE_MY_PLAYER_ID
        ),
        Vec2::new(28.716666665660146, 9.000000000999998), EXAMPLE_MY_PLAYER_ID
    );
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    assert_eq!(simulator.mines().len(), 2);
    assert_eq!(simulator.bullets().len(), 1);
    for _ in 0 .. 10 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator.mines().len(), 0);
    assert_eq!(simulator.bullets().len(), 0);
}

#[test]
fn test_simulator_unit_land_on_unit() {
    let world = with_my_position(example_world(), Vec2::new(2.5, 3.5));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    for _ in 0 .. 10 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator.unit().position(), Vec2::new(2.5, 2.800000000000077));
    assert_eq!(simulator.unit().base().jump_state.can_jump, true);
}

#[test]
fn test_simulator_unit_jump_from_unit_below() {
    let world = with_my_position(example_world(), Vec2::new(2.5, 2.800000000000077));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    assert_eq!(simulator.unit().base().jump_state.can_jump, false);
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(simulator.unit().position(), Vec2::new(2.5, 2.800000000000077));
    assert_eq!(simulator.unit().base().jump_state.can_jump, true);
    simulator.unit_mut().action_mut().jump = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(simulator.unit().position(), Vec2::new(2.5, 2.9666666666667254));
}

#[test]
fn test_simulator_unit_cancel_jump_on_hit_unit_above() {
    let world = with_unit_position(example_world(), EXAMPLE_OPPONENT_UNIT_ID, Vec2::new(37.5, 5.0));
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    assert_eq!(simulator.unit().base().jump_state.can_jump, false);
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(simulator.unit().position(), Vec2::new(37.5, 1.0));
    assert_eq!(simulator.unit().base().jump_state.can_jump, true);
    simulator.unit_mut().action_mut().jump = true;
    for _ in 0 .. 6 {
        simulator.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator.unit().position(), Vec2::new(37.5, 2.000000000000023));
    assert_eq!(simulator.unit().base().jump_state.can_jump, true);
    simulator.unit_mut().action_mut().jump = true;
    simulator.tick(
        world.tick_time_interval(),
        world.properties().updates_per_tick as usize,
        &mut Some(&mut rng),
        &mut None,
    );
    assert_eq!(simulator.unit().position(), Vec2::new(37.5, 1.86500000000002));
    assert_eq!(simulator.unit().base().jump_state.can_jump, false);
}

#[test]
fn test_simulator_bullet_on_corner_hit_unit() {
    let world = with_my_position(
        with_bullet(
            example_world(),
            WeaponType::AssaultRifle, Vec2::new(36.3, 3.05), Vec2::new(1.0, 1.0), EXAMPLE_OPPONENT_UNIT_ID
        ),
        Vec2::new(37.5, 2.0)
    );
    let mut rng = example_rng(7348172934612063328);
    let mut simulator1 = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    for _ in 0 .. 3 {
        simulator1.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator1.unit().health(), 95);
    let mut simulator2 = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    for _ in 0 .. 3 {
        simulator2.tick(
            world.tick_time_interval(),
            world.config().plan_microticks_per_tick,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator2.unit().health(), 95);
}

#[test]
fn test_simulator_bullet_on_corner_miss_unit() {
    let world = with_my_position(
        with_bullet(
            example_world(),
            WeaponType::AssaultRifle, Vec2::new(36.3, 3.07), Vec2::new(1.0, 1.0), EXAMPLE_OPPONENT_UNIT_ID
        ),
        Vec2::new(37.5, 2.0)
    );
    let mut rng = example_rng(7348172934612063328);
    let mut simulator1 = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    for _ in 0 .. 4 {
        simulator1.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator1.unit().health(), 100);
    let mut simulator2 = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    for _ in 0 .. 4 {
        simulator2.tick(
            world.tick_time_interval(),
            1,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator2.unit().health(), 100);
}

#[test]
fn test_simulator_bullet_on_corner_hit_tile() {
    let world = with_bullet(
        with_my_position(example_world(), Vec2::new(31.5, 5.0)),
        WeaponType::RocketLauncher, Vec2::new(31.5, 5.8),  Vec2::new(1.0, -1.0), EXAMPLE_MY_UNIT_ID
    );
    let mut rng = example_rng(7348172934612063328);
    let mut simulator1 = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    for _ in 0 .. 3 {
        simulator1.tick(
            world.tick_time_interval(),
            world.properties().updates_per_tick as usize,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator1.unit().base().health, 50);
    assert_eq!(simulator1.bullets().len(), 0);
    let mut simulator2 = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    for _ in 0 .. 4 {
        simulator2.tick(
            world.tick_time_interval(),
            1,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator2.unit().base().health, 50);
    assert_eq!(simulator2.bullets().len(), 0);
}

#[test]
fn test_simulator_bullet_explode_on_rect_border() {
    let world = with_bullet(
        with_my_position(example_world(), Vec2::new(6.59, 1.0)),
        WeaponType::RocketLauncher, Vec2::new(4.0, 1.8),  Vec2::new(-1.0, 0.0), EXAMPLE_MY_UNIT_ID
    );
    let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID, SIMULATOR_DEFAULT_FLAGS);
    let mut rng = example_rng(7348172934612063328);
    assert_eq!(simulator.unit().health(), 100);
    for _ in 0 .. 3 {
        simulator.tick(
            world.tick_time_interval(),
            1,
            &mut Some(&mut rng),
            &mut None,
        );
    }
    assert_eq!(simulator.unit().health(), 50);
    assert_eq!(simulator.player().score, 80);
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

#[test]
fn test_bullet_ext_moving_rect_by_zero() {
    let source = Vec2::new(31.5, 5.8);
    let world = with_bullet(example_world(), WeaponType::RocketLauncher, source, Vec2::new(1.0, 0.0), EXAMPLE_MY_UNIT_ID);
    let bullet_ext = BulletExt::new(world.bullets()[0].clone(), 0);
    let moving_rect = bullet_ext.moving_rect(0.0);

    assert_eq!((moving_rect.min(), moving_rect.max()), (Vec2::new(31.3, 5.6), Vec2::new(31.7, 6.0)));
}

#[test]
fn test_bullet_ext_moving_rect_by_x() {
    let source = Vec2::new(31.5, 5.8);
    let world = with_bullet(example_world(), WeaponType::RocketLauncher, source, Vec2::new(1.0, 0.0), EXAMPLE_MY_UNIT_ID);
    let bullet_ext = BulletExt::new(world.bullets()[0].clone(), 0);
    let moving_rect = bullet_ext.moving_rect(1.0);

    assert_eq!((moving_rect.min(), moving_rect.max()), (Vec2::new(31.3, 5.6), Vec2::new(51.7, 6.0)));
}

#[test]
fn test_bullet_ext_moving_rect_by_y() {
    let source = Vec2::new(31.5, 5.8);
    let world = with_bullet(example_world(), WeaponType::RocketLauncher, source, Vec2::new(0.0, 1.0), EXAMPLE_MY_UNIT_ID);
    let bullet_ext = BulletExt::new(world.bullets()[0].clone(), 0);
    let moving_rect = bullet_ext.moving_rect(1.0);

    assert_eq!((moving_rect.min(), moving_rect.max()), (Vec2::new(31.3, 5.600000000000001), Vec2::new(31.7, 26.0)));
}

#[test]
fn test_bullet_ext_moving_rect_by_x_and_y() {
    let source = Vec2::new(31.5, 5.8);
    let world = with_bullet(example_world(), WeaponType::RocketLauncher, source, Vec2::new(1.0, 1.0), EXAMPLE_MY_UNIT_ID);
    let bullet_ext = BulletExt::new(world.bullets()[0].clone(), 0);
    let moving_rect = bullet_ext.moving_rect(1.0);

    assert_eq!((moving_rect.min(), moving_rect.max()), (Vec2::new(31.3, 5.6), Vec2::new(45.842135623730954, 20.14213562373095)));
}
