#!/bin/bash

export RUSTFLAGS='-Zincremental-verify-ich'

cp tiles.rs src/tiles.rs
cargo clean -p mantle-diver
cargo build

./apply.py

OUT=$(cargo build 2>&1)

cp tiles.rs src/tiles.rs

if echo $OUT | grep "internal compiler error";
then
    echo "The ICE reproduces"
    exit 0
else
    echo "No reproduction"
    exit 1
fi