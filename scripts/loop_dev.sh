#!/bin/bash -ex

PORT=${1}
CONFIG=${2}
VERSION=$(date +%Y-%m-%d_%H-%M-%S)-$(git rev-parse --short HEAD)
BIN=bin/${VERSION}

if ! [[ "${PORT}" ]]; then
    PORT=31002
fi

if [[ "${CONFIG}" ]] && [[ "${CONFIG}" != 'etc/_' ]]; then
    cargo build --release --features=read_config
else
    cargo build --release
fi

cp target/release/aicup2019 ${BIN}

{
    number=0
    try=0
    while [[ ${try} -lt 10 ]]; do
        echo "date $(date) ${number}"
        env CONFIG=${CONFIG} ${BIN} 127.0.0.1 ${PORT} && {
            try=1
        } || {
            try=$(( try + 1 ))
        }
        sleep $( echo "print 0.1 * ${try}" | perl )
        number=$(( number + 1 ))
    done
} 2>&1 | tee results/logs/run.${PORT}.log
