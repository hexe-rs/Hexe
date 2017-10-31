#!/usr/bin/env bash

set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

for d in hexe_core hexe; do
    cd "$DIR/$d"
    cargo test $FEATURES
    cargo test $FEATURES --no-default-features
done
