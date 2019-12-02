extern crate my_strategy;

use criterion::{Criterion, criterion_group, criterion_main};
use my_strategy::examples::{
    example_rng,
    example_world,
};
use my_strategy::my_strategy::simulator::Simulator;

fn simulator_tick(c: &mut Criterion) {
    c.bench_function("simulator_tick", |b| {
        let world = example_world();
        let mut simulator = Simulator::new(&world, world.me().id);
        let time_interval = world.tick_time_interval();
        let micro_ticks_per_tick = world.properties().updates_per_tick as usize;
        let mut rng = example_rng(7348172934612063328);
        b.iter(move || {
            simulator.tick(time_interval, micro_ticks_per_tick, &mut rng);
        })
    });
}

fn simulator_tick_with_half_micro_ticks(c: &mut Criterion) {
    c.bench_function("simulator_tick_with_half_micro_ticks", |b| {
        let world = example_world();
        let mut simulator = Simulator::new(&world, world.me().id);
        let time_interval = world.tick_time_interval();
        let micro_ticks_per_tick = world.properties().updates_per_tick as usize / 2;
        let mut rng = example_rng(7348172934612063328);
        b.iter(move || {
            simulator.tick(time_interval, micro_ticks_per_tick, &mut rng);
        })
    });
}

criterion_group!(benches, simulator_tick, simulator_tick_with_half_micro_ticks);
criterion_main!(benches);
