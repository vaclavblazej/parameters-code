#!/usr/bin/env sh

cd "$(dirname "$0")"/../../code/ || exit \
    && mkdir -p /tmp/hops \
    && cargo build --all-targets --release --target-dir /tmp/hops \
    && (/tmp/hops/release/par "$@") \
    && cmd server
