#!/usr/bin/env bash

set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

CRATES="hexe_core hexe"

if [[ -n "$CLIPPY" ]]; then
    if ! cargo install clippy --debug --force; then
        echo "COULD NOT COMPILE CLIPPY, IGNORING CLIPPY TESTS"
        exit
    fi

    for crate in $CRATES; do
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

    for crate in $CRATES; do
        cd "$DIR/$crate"
        cargo test $TARGET_ARGS $FEATURES
        cargo test $TARGET_ARGS $FEATURES --no-default-features
    done
fi
