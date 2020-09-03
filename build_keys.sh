#!/bin/zsh
cargo build --all
cp target/debug/libstd*.so keys/