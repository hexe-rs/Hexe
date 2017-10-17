#!/usr/bin/env bash

DIR=$(realpath $(dirname "${BASH_SOURCE[0]}"))

for d in hexe hexe_core; do
    cd "$DIR/$d"
    cargo test
done
