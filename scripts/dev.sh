#!/bin/bash -ex

cargo build --release --features=dump_level,enable_debug,enable_debug_backtrack,enable_debug_log,enable_debug_optimal_location,enable_debug_optimal_plan,enable_debug_optimal_target,enable_debug_plan,enable_debug_unit,max_tick,enable_debug_bullet
target/release/aicup2019
