#!/usr/bin/env sh

cd "$(dirname "$0")"/../../code/ || exit \
    && cargo build --target-dir /tmp && /tmp/debug/par \
    && cd .. || exit \
    && cmd server
