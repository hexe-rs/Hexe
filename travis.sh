#!/usr/bin/env bash

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

for d in hexe hexe_core; do
    cd "$DIR/$d"
    cargo test
    cargo test --no-default-features
done
