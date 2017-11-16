#!/usr/bin/env bash

set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if $TEST_32_BIT; then
    echo "$TEST_32_BIT"
    curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain="stable-i686-unknown-linux-gnu" -y
fi

for d in hexe_core hexe; do
    cd "$DIR/$d"
    cargo test $FEATURES
    cargo test $FEATURES --no-default-features
done
