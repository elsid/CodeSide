use model::UnitAction;
use my_strategy::Debug;
use my_strategy::examples::example_world;
use my_strategy::my_strategy::my_strategy_impl::MyStrategyImpl;
use my_strategy::my_strategy::unit_action::UnitActionWrapper;
use std::io::BufWriter;

#[test]
fn test_first_action() {
    let stdout = std::io::stdout();
    let handle = stdout.lock();
    let mut stream = BufWriter::new(handle);
    let world = example_world();
    let mut my_strategy = MyStrategyImpl::new(world.config().clone(), world.me().clone(), world.game().clone());
    let result = my_strategy.get_action(world.me(), world.game(), &mut Debug(&mut stream));
    assert_eq!(UnitActionWrapper(&result), UnitActionWrapper(&UnitAction {
        velocity: -27.0,
        jump: true,
        jump_down: false,
        aim: model::Vec2F64 {
            x: -35.0,
            y: 0.0
        },
        shoot: true,
        swap_weapon: false,
        plant_mine: false,
    }));
}
