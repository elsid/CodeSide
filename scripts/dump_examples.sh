#!/bin/bash -ex

cargo build --release --features=dump_level,enable_debug,dump_examples
target/release/aicup2019 | tee examples.log.rs
rustfmt examples.log.rs
