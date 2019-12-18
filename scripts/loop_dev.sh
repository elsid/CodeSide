#!/bin/bash -e

PORT=${1}
VERSION=$(date +%Y-%m-%d_%H-%M-%S)-$(git rev-parse --short HEAD)
BIN=bin/${VERSION}

if ! [[ "${PORT}" ]]; then
    PORT=31002
fi

cargo build --release
cp target/release/aicup2019 ${BIN}

{
    number=0
    try=0
    while [[ ${try} -lt 10 ]]; do
        echo "date $(date) ${number}"
        ${BIN} 127.0.0.1 ${PORT} && {
            try=0
        } || {
            try=$(( try + 1 ))
        }
        sleep 0.2
        number=$(( number + 1 ))
    done
} 2>&1 | tee results/logs/run.${PORT}.log
