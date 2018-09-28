#!/bin/sh

#set -ex
#cd "$(dirname $0)"

#cargo +nightly run \
#  --bin wasm-bindgen -- \
#  target/wasm32-unknown-unknown/debug/wurst.wasm --out-dir .
# Ensure upto date binary:
# cargo install -f wasm-bindgen-cli --git https://github.com/rustwasm/wasm-bindgen.git
# Enable formatted code output:
# export WEBIDL_RUSTFMT_BINDINGS = 1;
cargo +nightly build --target wasm32-unknown-unknown --verbose --release &&
wasm-bindgen ../../target/wasm32-unknown-unknown/release/form_example.wasm --out-dir . &&
npm install &&
npm run serve
