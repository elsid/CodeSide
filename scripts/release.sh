#!/bin/bash -ex

VERSION=$(date +%Y-%m-%d_%H-%M-%S)-$(git rev-parse --short HEAD)
SRC=${PWD}
DIR=${SRC}/release/${VERSION}

mkdir -p release
mkdir ${DIR}
mkdir ${DIR}/src

cp src/bullet.rs ${DIR}/src
cp src/common.rs ${DIR}/src
cp src/config.rs ${DIR}/src
cp src/hit.rs ${DIR}/src
cp src/level.rs ${DIR}/src
cp src/location.rs ${DIR}/src
cp src/loot_box.rs ${DIR}/src
cp src/mine.rs ${DIR}/src
cp src/my_strategy.rs ${DIR}/src
cp src/my_strategy_impl.rs ${DIR}/src
cp src/optimal_tile.rs ${DIR}/src
cp src/plan.rs ${DIR}/src
cp src/positionable.rs ${DIR}/src
cp src/properties.rs ${DIR}/src
cp src/random.rs ${DIR}/src
cp src/rect.rs ${DIR}/src
cp src/rectangular.rs ${DIR}/src
cp src/search.rs ${DIR}/src
cp src/simulator.rs ${DIR}/src
cp src/unit.rs ${DIR}/src
cp src/unit_action.rs ${DIR}/src
cp src/vec2.rs ${DIR}/src
cp src/vec2_f64.rs ${DIR}/src
cp src/vec2i.rs ${DIR}/src
cp src/walk_grid.rs ${DIR}/src
cp src/world.rs ${DIR}/src

cp ${SRC}/raic/clients/rust/Cargo.toml ${DIR}/
cp -r ${SRC}/raic/clients/rust/src/main.rs ${DIR}/src/
cp -r ${SRC}/raic/clients/rust/model ${DIR}/

cd ${DIR}/

zip ${SRC}/release/${VERSION}.zip -r Cargo.toml src model

cp ${SRC}/src/lib.rs src/
cp ${SRC}/src/examples.rs src/

cp -r ${SRC}/tests ${DIR}/

cargo build --release
cargo test --release

ls -al ${SRC}/release/${VERSION}.zip
