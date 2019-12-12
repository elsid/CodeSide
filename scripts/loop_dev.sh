#!/bin/bash -ex

PORT=${1}
VERSION=$(date +%Y-%m-%d_%H-%M-%S)-$(git rev-parse --short HEAD)
BIN=bin/${VERSION}

if ! [[ "${PORT}" ]]; then
    PORT=31002
fi

cargo build --release --features=dump_level
cp target/release/aicup2019 ${BIN}

{
    number=0
    while true; do
        date
        ${BIN} 127.0.0.1 ${PORT} 2>&1
        sleep 0.2
        number=$(( number + 1 ))
    done
} | tee run.${PORT}.log
