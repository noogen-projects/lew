#!/bin/sh

example=${1}
mode=${2:+"--release"}
out_dir=${2:-debug}

cargo build --example $example --target wasm32-unknown-unknown $mode
wasm-bindgen --target web --no-typescript --out-dir examples/static/target --out-name $example target/wasm32-unknown-unknown/${out_dir}/examples/${example}.wasm