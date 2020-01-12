mod helpers;

use std::io::BufWriter;

use model::{
    UnitAction,
    WeaponType,
};

use aicup2019::{
    examples::{
        EXAMPLE_MY_UNIT_ID,
        example_world
    },
    my_strategy::{
        Debug,
        MyStrategyImpl,
        UnitActionWrapper,
        Vec2,
    }
};

use helpers::{
    with_my_position,
    with_my_unit_with_weapon,
};

#[test]
fn test_my_strategy_impl_get_action_initial() {
    let stdout = std::io::stdout();
    let handle = stdout.lock();
    let mut stream = BufWriter::new(handle);
    let world = example_world();
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let mut my_strategy = MyStrategyImpl::new(world.config().clone(), unit.clone(), world.game().clone());
    let mut debug = aicup2019::Debug(&mut stream);
    let result = my_strategy.get_action(&unit, &world.game(), &mut Debug::new(&mut debug));
    assert_eq!(UnitActionWrapper(&result), UnitActionWrapper(&UnitAction {
        velocity: -10.0,
        jump: true,
        jump_down: false,
        aim: model::Vec2F64 {
            x: 0.0,
            y: 0.0
        },
        shoot: false,
        reload: false,
        swap_weapon: false,
        plant_mine: false,
    }));
}

#[test]
fn test_my_strategy_impl_get_action_with_assault_rifle() {
    let stdout = std::io::stdout();
    let handle = stdout.lock();
    let mut stream = BufWriter::new(handle);
    let world = with_my_unit_with_weapon(example_world(), WeaponType::AssaultRifle);
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let mut my_strategy = MyStrategyImpl::new(world.config().clone(), unit.clone(), world.game().clone());
    let mut debug = aicup2019::Debug(&mut stream);
    let result = my_strategy.get_action(&unit, &world.game(), &mut Debug::new(&mut debug));
    assert_eq!(UnitActionWrapper(&result), UnitActionWrapper(&UnitAction {
        velocity: -10.0,
        jump: true,
        jump_down: false,
        aim: model::Vec2F64 {
            x: -35.0,
            y: 0.0
        },
        shoot: false,
        reload: false,
        swap_weapon: false,
        plant_mine: false,
    }));
}

#[test]
fn test_my_strategy_impl_get_action_with_assault_rifle_nearby_opponent() {
    let stdout = std::io::stdout();
    let handle = stdout.lock();
    let mut stream = BufWriter::new(handle);
    let world = with_my_position(with_my_unit_with_weapon(example_world(), WeaponType::AssaultRifle), Vec2::new(4.5, 1.0));
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let mut my_strategy = MyStrategyImpl::new(world.config().clone(), unit.clone(), world.game().clone());
    let mut debug = aicup2019::Debug(&mut stream);
    let result = my_strategy.get_action(&unit, &world.game(), &mut Debug::new(&mut debug));
    assert_eq!(UnitActionWrapper(&result), UnitActionWrapper(&UnitAction {
        velocity: 0.0,
        jump: false,
        jump_down: true,
        aim: model::Vec2F64 {
            x: -2.0,
            y: 0.0
        },
        shoot: true,
        reload: false,
        swap_weapon: false,
        plant_mine: false,
    }));
}
