#!/usr/bin/env sh

cd "$(dirname "$0")"/../../code/ || exit \
    && cargo build --all-targets --release --target-dir /tmp/hops && /tmp/hops/release/par "$@" \
    && cd .. || exit \
    && cmd server
