#!/usr/bin/env sh
set -e

(cd calculator && cargo component build --release)
(cd operators/add && cargo component build --release)
(cd api && cargo component build --release)

mkdir -p "dist/local:calculator"
cp "target/wasm32-wasi/release/calculator.wasm" "dist"
cp "target/wasm32-wasi/release/add.wasm" "dist/local:calculator/"

RUST_LOG=none wasm-tools compose -c api-config.yml -o dist/api.wasm target/wasm32-wasi/release/api.wasm
