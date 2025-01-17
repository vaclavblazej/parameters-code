#!/usr/bin/env sh

cd "$(dirname "$0")"/../../code/ || exit \
    && cargo build --release --target-dir /tmp && /tmp/release/par "$@" \
    && cd .. || exit \
    && cmd server
