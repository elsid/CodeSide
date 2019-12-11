#!/bin/bash -ex

cargo build --release --features=dump_level,enable_debug
target/release/aicup2019
