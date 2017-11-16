#!/usr/bin/env bash

set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [[ -z "$TARGET" ]]; then
    TARGET_ARGS=""
else
    rustup target add "$TARGET"
    TARGET_ARGS="--target $TARGET"
fi

for d in hexe_core hexe; do
    cd "$DIR/$d"
    cargo test $TARGET_ARGS $FEATURES
    cargo test $TARGET_ARGS $FEATURES --no-default-features
done
