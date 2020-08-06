#!/bin/zsh
cargo build
cargo build --release
time ./target/debug/ars
time ./target/release/ars
