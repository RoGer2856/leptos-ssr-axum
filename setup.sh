#!/bin/sh

rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install cargo-leptos

# create new ssr project
# cargo leptos new --git https://github.com/leptos-rs/start-axum
