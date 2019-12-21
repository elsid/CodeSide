use model::UnitAction;
use aicup2019::{
    Debug,
    examples::{
        EXAMPLE_MY_UNIT_ID,
        example_world
    },
    my_strategy::{
        MyStrategyImpl,
        UnitActionWrapper,
    }
};
use std::io::BufWriter;

#[test]
fn test_first_action() {
    let stdout = std::io::stdout();
    let handle = stdout.lock();
    let mut stream = BufWriter::new(handle);
    let world = example_world();
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let mut my_strategy = MyStrategyImpl::new(world.config().clone(), unit.clone(), world.game().clone());
    let result = my_strategy.get_action(&unit, &world.game(), &mut Debug(&mut stream));
    assert_eq!(UnitActionWrapper(&result), UnitActionWrapper(&UnitAction {
        velocity: -10.0,
        jump: false,
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
