#!/usr/bin/sh

set -x

cargo build --target=wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/browser_snake.wasm js/browser_snake.wasm 
