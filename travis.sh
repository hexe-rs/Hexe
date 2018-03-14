#!/usr/bin/env bash

set -e

if [[ -n "$CLIPPY" ]]; then
    if ! cargo install clippy --debug --force; then
        echo "COULD NOT COMPILE CLIPPY, IGNORING CLIPPY TESTS"
        exit
    fi

    DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    for crate in hexe_core hexe; do
        cd "$DIR/$crate"
        cargo clippy -- -Dclippy
    done
else
    if [[ -z "$TARGET" ]]; then
        TARGET_ARGS=""
    else
        rustup target add "$TARGET"
        TARGET_ARGS="--target $TARGET"
    fi

    cargo test $TARGET_ARGS -p hexe $FEATURES

    cd hexe_core
    cargo test $TARGET_ARGS $FEATURES
    cargo test $TARGET_ARGS $FEATURES --no-default-features
fi
