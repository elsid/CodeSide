extern crate aicup2019;

use criterion::{Criterion, criterion_group, criterion_main};

use aicup2019::{
    examples::{
        EXAMPLE_MY_UNIT_ID,
        example_rng,
        example_world,
    },
    my_strategy::{
        Simulator,
    },
};

fn simulator_tick(c: &mut Criterion) {
    c.bench_function("simulator_tick", |b| {
        let world = example_world();
        let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID);
        let time_interval = world.tick_time_interval();
        let micro_ticks_per_tick = world.properties().updates_per_tick as usize;
        let mut rng = example_rng(7348172934612063328);
        b.iter(move || {
            simulator.tick(time_interval, micro_ticks_per_tick, &mut rng, &mut None);
        })
    });
}

fn simulator_tick_with_half_micro_ticks(c: &mut Criterion) {
    c.bench_function("simulator_tick_with_half_micro_ticks", |b| {
        let world = example_world();
        let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID);
        let time_interval = world.tick_time_interval();
        let micro_ticks_per_tick = world.properties().updates_per_tick as usize / 2;
        let mut rng = example_rng(7348172934612063328);
        b.iter(move || {
            simulator.tick(time_interval, micro_ticks_per_tick, &mut rng, &mut None);
        })
    });
}

fn simulator_tick_with_single_micro_tick(c: &mut Criterion) {
    c.bench_function("simulator_tick_with_single_micro_tick", |b| {
        let world = example_world();
        let mut simulator = Simulator::new(&world, EXAMPLE_MY_UNIT_ID);
        let time_interval = world.tick_time_interval();
        let micro_ticks_per_tick = 1;
        let mut rng = example_rng(7348172934612063328);
        b.iter(move || {
            simulator.tick(time_interval, micro_ticks_per_tick, &mut rng, &mut None);
        })
    });
}

criterion_group!(benches, simulator_tick, simulator_tick_with_half_micro_ticks, simulator_tick_with_single_micro_tick);
criterion_main!(benches);
