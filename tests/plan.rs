mod helpers;

use std::io::BufWriter;

use helpers::{
    with_level,
    with_my_position,
};

use aicup2019::{
    examples::{
        EXAMPLE_MY_UNIT_ID,
        example_rng,
        example_world,
    },
    my_strategy::{
        SIMULATOR_DEFAULT_FLAGS,
        Debug,
        Planner,
        Simulator,
        Vec2,
        parse_level,
    },
};

#[test]
fn test_planner() {
    let stdout = std::io::stdout();
    let handle = stdout.lock();
    let mut stream = BufWriter::new(handle);
    let mut debug = aicup2019::Debug(&mut stream);
    let mut rng = example_rng(7348172934612063328);
    let level = parse_level("\
########################################\n\
#.........#....#........#....#.........#\n\
#.........#....#........#....#.........#\n\
#.........#....#........#....#.........#\n\
#....######....#........#....######....#\n\
#.........#..................#.........#\n\
#.........#..................#.........#\n\
#.........#..................#.........#\n\
#.........#..................#.........#\n\
#^^^^.....#^^H^##########^H^^#.....^^^^#\n\
#.........#..H............H..#.........#\n\
#.........#..H............H..#.........#\n\
#.........#..H............H..#.........#\n\
#.........#..H............H..#.........#\n\
#^^^^#^^^^#..H............H..#^^^^#^^^^#\n\
#....#....#..H............H..#....#....#\n\
#....#....#..H............H..#....#....#\n\
#....#....#..H............H..#....#....#\n\
#....#....#..H............H..#....#....#\n\
######....#^^^^##########^^^^#....######\n\
#..............#........#..............#\n\
#..............#........#..............#\n\
#..............#........#..............#\n\
#.......T......#........#......T.......#\n\
######^^^^#....#........#....#^^^^######\n\
#.........#..................#.........#\n\
#.........#..................#.........#\n\
#.........#..................#.........#\n\
#........X#.T..............T.#X........#\n\
########################################
");
    let source = Vec2::new(30.5, 1.0);
    let destination = Vec2::new(9.450000001000008, 1.000000001000013);
    let world = with_my_position(with_level(example_world(), level), source);
    let unit = world.get_unit(EXAMPLE_MY_UNIT_ID);
    let mut config = world.config().clone();
    config.plan_max_state_depth = 65;
    config.plan_max_iterations = 73097;
    let mut simulator = Simulator::new(&world, unit.id, SIMULATOR_DEFAULT_FLAGS);
    let planner = Planner::new(destination, &config, simulator.clone(), world.max_distance(), world.max_score());
    let plan = planner.make(world.current_tick(), &mut rng, &mut Debug::new(&mut debug));
    let time_interval = world.config().plan_time_interval_factor / world.properties().ticks_per_second as f64;
    for transition in plan.transitions.iter() {
        simulator.set_unit_action(unit.id, transition.get_action(world.properties()));
        simulator.tick(time_interval, world.config().plan_microticks_per_tick, &mut rng, &mut Some(&mut Debug::new(&mut debug)));
        println!("[{}] {:?} {:?}", simulator.current_tick(), simulator.unit().position(), transition.get_action(world.properties()));
    }
    assert_eq!(simulator.unit().position(), destination);
}
